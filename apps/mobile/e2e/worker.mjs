import {
  Commoner,
  offer,
  accept,
  receipt,
  buildOfferPayload,
  buildAcceptPayload,
  buildReceiptPayload,
  computeOfferId,
} from 'comum-js/dist/mobile.js'
import { WebSocket } from 'ws'

const EMPTY_CONTEXT = {
  type: 'none',
  payload_cbor: new Uint8Array([0xa0]),
  proof: { version: 1, signatures: [], zk_proofs: [], nullifiers: [] },
}

const capsuleId = new Uint8Array(32).fill(0x46)
const local = {
  commoner: null,
  testimonies: [],
  ws: null,
  room: '',
  nodeId: '',
}

function hexToBytes(hex) {
  if (hex.length % 2 !== 0) throw new Error('invalid hex')
  const out = new Uint8Array(hex.length / 2)
  for (let i = 0; i < out.length; i += 1) out[i] = Number.parseInt(hex.slice(i * 2, i * 2 + 2), 16)
  return out
}

function bytesToHex(bytes) {
  return Array.from(bytes)
    .map((b) => b.toString(16).padStart(2, '0'))
    .join('')
}

function encodeBase64(bytes) {
  return Buffer.from(bytes).toString('base64')
}

function decodeBase64(data) {
  return new Uint8Array(Buffer.from(data, 'base64'))
}

function reply(id, ok, payload) {
  process.send({ id, ok, payload })
}

function publish(testimony) {
  if (!local.ws || local.ws.readyState !== WebSocket.OPEN) throw new Error('transport not connected')
  local.ws.send(
    JSON.stringify({
      type: 'testimony',
      room: local.room,
      from: local.nodeId,
      testimony: encodeBase64(testimony),
    })
  )
}

function connect(wsUrl, room, nodeId) {
  return new Promise((resolveConnect, rejectConnect) => {
    const ws = new WebSocket(wsUrl)
    const timeout = setTimeout(() => rejectConnect(new Error('ws connect timeout')), 5_000)

    ws.on('open', () => {
      ws.send(JSON.stringify({ type: 'join', room, nodeId }))
    })

    ws.on('message', (raw) => {
      let msg
      try {
        msg = JSON.parse(raw.toString())
      } catch {
        return
      }

      if (msg.type === 'joined') {
        clearTimeout(timeout)
        local.ws = ws
        local.room = room
        local.nodeId = nodeId
        resolveConnect()
        return
      }

      if (msg.type === 'testimony') {
        if (msg.from === local.nodeId) return
        if (!local.commoner) return
        const testimony = decodeBase64(msg.testimony)
        local.commoner.ingest(testimony)
        local.testimonies.push(testimony)
      }
    })

    ws.on('error', (err) => {
      clearTimeout(timeout)
      rejectConnect(err)
    })
  })
}

process.on('message', async (msg) => {
  const { id, cmd, payload } = msg
  try {
    if (cmd === 'init') {
      const seed = hexToBytes(payload.seedHex)
      local.commoner = new Commoner(seed, 1)
      await connect(payload.wsUrl, payload.room, local.commoner.did())
      return reply(id, true, { did: local.commoner.did() })
    }

    if (!local.commoner) throw new Error('worker not initialized')

    if (cmd === 'emitOffer') {
      const { item, price, currency, expires } = payload
      const offerPayload = buildOfferPayload(item, price, currency, expires, local.commoner.did())
      const offerId = computeOfferId(item, price, currency, expires, local.commoner.did())
      const out = offer(local.commoner, capsuleId, offerPayload, EMPTY_CONTEXT)
      local.testimonies.push(out.cbor)
      publish(out.cbor)
      return reply(id, true, { testimonyIdHex: out.id_hex, offerIdHex: bytesToHex(offerId) })
    }

    if (cmd === 'emitAccept') {
      const offerId = hexToBytes(payload.offerIdHex)
      const acceptPayload = buildAcceptPayload(offerId, local.commoner.did())
      const out = accept(local.commoner, capsuleId, acceptPayload, EMPTY_CONTEXT)
      local.testimonies.push(out.cbor)
      publish(out.cbor)
      return reply(id, true, { testimonyIdHex: out.id_hex })
    }

    if (cmd === 'emitReceipt') {
      const offerId = hexToBytes(payload.offerIdHex)
      const receiptPayload = buildReceiptPayload(offerId, payload.timestamp)
      const out = receipt(local.commoner, capsuleId, receiptPayload, EMPTY_CONTEXT)
      local.testimonies.push(out.cbor)
      publish(out.cbor)
      return reply(id, true, { testimonyIdHex: out.id_hex })
    }

    if (cmd === 'stats') {
      return reply(id, true, { did: local.commoner.did(), testimonyCount: local.testimonies.length })
    }

    throw new Error(`unknown command: ${cmd}`)
  } catch (err) {
    reply(id, false, { error: err instanceof Error ? err.message : String(err) })
  }
})
