import { useEffect, useMemo, useState } from 'react'
import { Text, View } from 'react-native'
import { AppNode } from './src/node'

export default function App() {
  const [status, setStatus] = useState('booting')
  const [did, setDid] = useState('')

  const node = useMemo(() => {
    const seed = new Uint8Array(32).fill(0x11)
    const capsuleId = new Uint8Array(32).fill(0x46)
    return new AppNode(seed, 1, capsuleId)
  }, [])

  useEffect(() => {
    setDid(node.did())
    setStatus('node-ready')
  }, [node])

  return (
    <View style={{ flex: 1, alignItems: 'center', justifyContent: 'center' }}>
      <Text>Comum Mobile Node</Text>
      <Text>{status}</Text>
      <Text>{did}</Text>
    </View>
  )
}
