import { WebSocketServer } from 'ws'

const port = Number.parseInt(process.env.COMUM_E2E_RELAY_PORT || '8787', 10)
const wss = new WebSocketServer({ port })
const rooms = new Map()

function safeSend(ws, payload) {
  if (ws.readyState === ws.OPEN) ws.send(JSON.stringify(payload))
}

wss.on('connection', (ws) => {
  let room = ''
  let nodeId = ''

  ws.on('message', (raw) => {
    let msg
    try {
      msg = JSON.parse(raw.toString())
    } catch {
      return
    }

    if (msg.type === 'join') {
      room = msg.room
      nodeId = msg.nodeId
      if (!rooms.has(room)) rooms.set(room, new Set())
      rooms.get(room).add(ws)
      safeSend(ws, { type: 'joined', room, nodeId })
      return
    }

    if (msg.type === 'testimony' && room) {
      const peers = rooms.get(room)
      if (!peers) return
      for (const peer of peers) {
        if (peer === ws) continue
        safeSend(peer, {
          type: 'testimony',
          room,
          from: nodeId,
          testimony: msg.testimony,
        })
      }
    }
  })

  ws.on('close', () => {
    if (!room) return
    const peers = rooms.get(room)
    if (!peers) return
    peers.delete(ws)
    if (peers.size === 0) rooms.delete(room)
  })
})

console.log(`relay listening on ws://127.0.0.1:${port}`)
