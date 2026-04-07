import assert from 'node:assert/strict'
import { fork } from 'node:child_process'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const here = dirname(fileURLToPath(import.meta.url))
const workerPath = resolve(here, 'worker.mjs')
const relayPath = resolve(here, 'relay.mjs')
const wsUrl = 'ws://127.0.0.1:8787'
const room = `room-${Date.now()}`

function sleep(ms) {
  return new Promise((resolveSleep) => setTimeout(resolveSleep, ms))
}

async function waitForCount(node, expected, timeoutMs) {
  const until = Date.now() + timeoutMs
  while (Date.now() < until) {
    const stats = await node.call('stats')
    if (stats.testimonyCount >= expected) return stats
    await sleep(50)
  }
  throw new Error(`timeout waiting for count ${expected}`)
}

function spawnNode() {
  const child = fork(workerPath, { stdio: ['inherit', 'inherit', 'inherit', 'ipc'] })
  let seq = 0

  function call(cmd, payload = {}) {
    return new Promise((resolveCall, rejectCall) => {
      const id = ++seq
      const onMessage = (msg) => {
        if (!msg || msg.id !== id) return
        child.off('message', onMessage)
        if (msg.ok) resolveCall(msg.payload)
        else rejectCall(new Error(msg.payload?.error || 'worker error'))
      }
      child.on('message', onMessage)
      child.send({ id, cmd, payload })
    })
  }

  return {
    call,
    stop: () => child.kill('SIGTERM'),
  }
}

async function main() {
  const relay = fork(relayPath, { stdio: ['inherit', 'inherit', 'inherit', 'ipc'] })
  await sleep(150)

  const seller = spawnNode()
  const buyer = spawnNode()
  const witness = spawnNode()

  try {
    await Promise.all([
      seller.call('init', { seedHex: '11'.repeat(32), wsUrl, room }),
      buyer.call('init', { seedHex: '22'.repeat(32), wsUrl, room }),
      witness.call('init', { seedHex: '33'.repeat(32), wsUrl, room }),
    ])

    const offer = await seller.call('emitOffer', {
      item: 'banana',
      price: 10,
      currency: 'comum',
      expires: Date.now() + 60_000,
    })
    await Promise.all([waitForCount(buyer, 1, 2_000), waitForCount(witness, 1, 2_000)])

    const accept = await buyer.call('emitAccept', { offerIdHex: offer.offerIdHex })
    await Promise.all([waitForCount(seller, 2, 2_000), waitForCount(witness, 2, 2_000)])

    const receipt = await buyer.call('emitReceipt', {
      offerIdHex: offer.offerIdHex,
      timestamp: Date.now(),
    })
    await Promise.all([waitForCount(seller, 3, 2_000), waitForCount(witness, 3, 2_000)])

    const [sellerStats, buyerStats, witnessStats] = await Promise.all([
      seller.call('stats'),
      buyer.call('stats'),
      witness.call('stats'),
    ])

    assert.ok(offer.testimonyIdHex.length === 64)
    assert.ok(accept.testimonyIdHex.length === 64)
    assert.ok(receipt.testimonyIdHex.length === 64)
    assert.equal(sellerStats.testimonyCount, 3)
    assert.equal(buyerStats.testimonyCount, 3)
    assert.equal(witnessStats.testimonyCount, 3)

    console.log('multi-node e2e ok')
    console.log({
      offer: offer.testimonyIdHex,
      accept: accept.testimonyIdHex,
      receipt: receipt.testimonyIdHex,
      seller: sellerStats.did,
      buyer: buyerStats.did,
      witness: witnessStats.did,
    })
  } finally {
    seller.stop()
    buyer.stop()
    witness.stop()
    relay.kill('SIGTERM')
  }
}

main().catch((err) => {
  console.error(err)
  process.exit(1)
})
