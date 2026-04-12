# Arquitetura

Camadas:

1. Transporte (BLE/NFC/QR/RNS)
2. Sincronizacao (gossip + delta)
3. Primitivos (Testimony, DAG, CBOR canonical)
4. Commoner (validacao, sync, snapshots)
5. Capsulas (WASM, semantica local)
6. Apps (mobile-first)

Este documento descreve o fluxo fim-a-fim a partir do Genesis e como o
Commoner aplica regras locais sem consenso global.

Observacao de fronteira: a semantica local de uma capsula nao deve viver aqui
como se fosse contrato global do protocolo. No monorepo, ela deve viver no SSOT
local de cada capsula, enquanto esta camada descreve apenas mecanismo de
execucao, sync e runtime.
