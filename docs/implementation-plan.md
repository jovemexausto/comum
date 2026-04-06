# Plano de Implementacao (CIP-0001)

Legenda: [x] = feito, [~] = em andamento (apenas um por vez)

## Epic 0 — Fundacao do Repositorio e Conformidade
- [x] Estrutura de repo (spec/impl/tests/docs)
- [x] CIP-0001 em spec/cips
- [x] Registries e schemas CDDL
- [x] Vetores reais (0001–0003) com expected_id
- [x] Runner de conformidade (Node)
- [x] Runner de conformidade (Rust)
- [x] Interop basico registrado
- [x] Ajustar Justfile (comum-rs-cli com input/arquivo)

## Epic 1 — comum-rs (Core)
- [~] Decoder CBOR canonical + validacao estrutural
- [ ] Validacao completa do Testimony (refs, prev_id, suites, context, proof)
- [ ] CTE encode/decode + fragmentacao
- [ ] Sync state machine (HELLO/REQUEST/RESPONSE)
- [ ] Suite CLASSIC_1 (assinatura real)
- [ ] API did:comum (resolucao local + key_rotate)

## Epic 2 — comum-js (Wrapper)
- [ ] API encode/verify (JS)
- [ ] Tests com vetores
- [ ] Pipeline de build (node + binario comum-cbor)

## Epic 3 — Capsulas (WASM)
- [ ] Runtime WASM MVP (sandbox + syscalls)
- [ ] Capsula Agora (propose/vote) compilada para .wasm
- [ ] Teste de invocacao local

## Epic 4 — Prova de Contexto
- [ ] Proximidade (NFC/BLE)
- [ ] Beacon comunitario
- [ ] Ancora de lugar
- [ ] Vouch geografico

## Epic 5 — Genesis e Governanca
- [ ] Testimony de Genesis (minimo 3)
- [ ] FROST (threshold signature)
- [ ] Capsula de governanca (vote/propose)

## Epic 6 — Interop e Release
- [ ] Vetores adicionais (author omitido + CTE fragmentado)
- [ ] Matriz interop em docs/interop
- [ ] Tag de release v0.3
