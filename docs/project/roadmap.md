# Roadmap

## v0.1 (entregue)

- CIP-0001 + registries + CDDL
- comum-rs / comum-js alinhados
- runtime WASM + capsula Agora
- vetores de teste + conformance
- interop matrix + release

## v0.2 (entregue)

- Commoner facade (API de no)
- Perfis de transporte NFC/BLE/QR/RNS
- Primeira referencia historica de Agora antes da virada para apps/capsulas do monorepo
- SDKs iniciais (JS/mobile)

## v0.3 (entregue)

- Sistema distribuido minimo funcional (nodes + rede + convergencia)
- App = node (Commoner no mobile)
- Fluxo Feira E2E (offer -> accept -> receipt)
- E2E multi-node via WebSocket (convergencia verificada)
- SDK JS com entrada React Native (sem N-API obrigatoria para E2E de app)

## v0.4

- Feira MVP como vertical slice principal (offer -> accept -> receipt entre dois nos)
- Extracao de ComumClient (API de alto nivel no SDK)
- Reducao de vazamento de CBOR no nivel de app
- Paridade de runtime no fluxo Feira (JS mobile vs Rust canônico)
- Suite de conformance cross-runtime focada no fluxo Feira
- Robustez de transporte (fault injection: drop/delay/reorder)
- Unificacao de API de transporte (WS/BLE/NFC)
- Padronizacao de naming (boundary TS: camelCase)

## Depois do Feira MVP

- Framing e testes iniciais de autoridade emergente auditavel
- ampliar vertical slices para Agora e Mutirao
- aumentar evidencia para paper 1 sem competir com entrega de produto
