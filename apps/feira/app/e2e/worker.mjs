/**
 * Worker do E2E multi-node.
 *
 * Cada worker e um processo filho que representa um no independente.
 * Usa ComumClient em vez de Commoner diretamente.
 */

import { ComumClient, makeWsTransport } from '../../../../runtime/js/comum-js/dist/mobile.js'
import { WebSocket } from 'ws'

const CAPSULE_ID = new Uint8Array(32).fill(0x46)

/** @type {ComumClient | null} */
let client = null

function hexToBytes(hex) {
  if (hex.length % 2 !== 0) throw new Error('invalid hex')
  const out = new Uint8Array(hex.length / 2)
  for (let i = 0; i < out.length; i += 1)
    out[i] = Number.parseInt(hex.slice(i * 2, i * 2 + 2), 16)
  return out
}

function reply(id, ok, payload) {
  process.send({ id, ok, payload })
}

process.on('message', async (msg) => {
  const { id, cmd, payload } = msg
  try {
    // ------------------------------------------------------------------
    if (cmd === 'init') {
      const seed = hexToBytes(payload.seedHex)
      client = new ComumClient(seed, CAPSULE_ID)

      const transport = makeWsTransport({
        url: payload.wsUrl,
        room: payload.room,
        did: client.did(),
        WebSocket,
      })

      await client.connect(transport)
      return reply(id, true, { did: client.did() })
    }

    // ------------------------------------------------------------------
    if (!client) throw new Error('worker not initialized')

    if (cmd === 'emitOffer') {
      const offer = client.createOffer({
        item: payload.item,
        price: payload.price,
        currency: payload.currency,
        expiresAt: payload.expires,
      })
      return reply(id, true, {
        testimonyIdHex: offer.testimonyIdHex,
        offerIdHex: offer.idHex,
      })
    }

    if (cmd === 'emitAccept') {
      const offerId = hexToBytes(payload.offerIdHex)
      const result = client.acceptOffer(offerId)
      return reply(id, true, { testimonyIdHex: result.testimonyIdHex })
    }

    if (cmd === 'emitReceipt') {
      const offerId = hexToBytes(payload.offerIdHex)
      const result = client.issueReceipt(offerId)
      return reply(id, true, { testimonyIdHex: result.testimonyIdHex })
    }

    if (cmd === 'stats') {
      return reply(id, true, {
        did: client.did(),
        testimonyCount: client.testimonyCount(),
      })
    }

    throw new Error(`unknown command: ${cmd}`)
  } catch (err) {
    reply(id, false, { error: err instanceof Error ? err.message : String(err) })
  }
})
