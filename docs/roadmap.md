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
- Semantica formal da Capsula Agora (CIP-0003)
- SDKs iniciais (JS/mobile)

## v0.3 (entregue)

- Sistema distribuido minimo funcional (nodes + rede + convergencia)
- App = node (Commoner no mobile)
- Fluxo Feira E2E (offer -> accept -> receipt)
- E2E multi-node via WebSocket (convergencia verificada)
- SDK JS com entrada React Native (sem N-API obrigatoria para E2E de app)

## v0.4

- Paridade de runtime (JS mobile vs Rust canônico)
- Suite de conformance cross-runtime
- Robustez de transporte (fault injection: drop/delay/reorder)
- Unificacao de API de transporte (WS/BLE/NFC)
- Extracao de ComumClient (API de alto nivel no SDK)
- Padronizacao de naming (boundary TS: camelCase)
- Reducao de vazamento de CBOR no nivel de app
- Framing e testes iniciais de autoridade emergente auditavel
