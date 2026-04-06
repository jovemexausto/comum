# Plano de Implementacao (CIP-0001)

Legenda: [x] = feito, [~] = em andamento (apenas um por vez)

Regra: toda implementacao nova MUST incluir testes (unitarios e/ou vetores).

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
- [x] Decoder CBOR canonical + validacao estrutural
- [x] Validacao completa do Testimony (refs, prev_id, suites, context, proof)
- [x] CTE encode/decode + fragmentacao
- [x] Sync state machine (HELLO/REQUEST/RESPONSE)
- [x] Suite CLASSIC_1 (assinatura real)
- [x] API did:comum (resolucao local + key_rotate)

## Epic 2 — comum-js (Wrapper)
- [x] API encode/verify (JS)
- [x] Tests com vetores
- [x] Pipeline de build (node + binario comum-cbor)

## Epic 3 — Capsulas (WASM)
- [x] Runtime WASM MVP (sandbox + syscalls)
- [x] Capsula Agora (propose/vote) compilada para .wasm
- [x] Teste de invocacao local
- [x] Runtime WASM completo (limites de memoria/tempo, determinismo, fuel)
- [x] Atualizar CIP-0001 (limites, ABI, ordenacao canonical, fragmentacao)
- [x] ABI formal de syscalls (tipos e convencoes)
- [x] Constantes normativas (fuel, memoria, timeout)
- [x] Limites de reassembly (fragmentos e timeout)
- [x] Ordenacao canonical de chaves textuais (regra explicita)
- [x] Sync payloads: regra de ordenacao canonical
- [x] Versao e dependencias do runtime (wasmtime)
- [x] Capsulas em Rust (wasm32-unknown-unknown, no_std)

## Epic 3B — Simulacoes Locais
- [x] Simulador basico de nos (A, B, C)
- [x] Cenario: encounter + transfer + sync
- [x] Logs e asserts de estado

## Epic 4 — Prova de Contexto
- [x] Proximidade (NFC/BLE)
- [x] Beacon comunitario
- [x] Ancora de lugar
- [x] Vouch comunitario

## Epic 5 — Genesis e Governanca
- [x] Testimony de Genesis (minimo 3)
- [x] FROST (threshold signature)
- [x] Capsula de governanca (vote/propose)

## Epic 6 — Interop e Release
- [ ] Vetores adicionais (author omitido + CTE fragmentado)
- [ ] Matriz interop em docs/interop
- [ ] Tag de release v0.1

## Correcoes Aplicadas (revisao CIP-0001)

- [x] Ordenacao canonical de chaves tstr em sync.rs (clock/suites/node_id/profile)
- [x] Separacao de dominio criptografico em nullifiers: HKDF + HMAC (crypto.rs)
- [x] Dependencias hmac e hkdf adicionadas ao Cargo.toml
- [x] Correcao de namespace: common/key_rotate -> comum/key_rotate (did.rs)
- [x] sync.cddl formal criado em spec/schemas/
- [x] wasm-abi.md atualizado com formato completo do query de read_graph
- [x] CIP-0001 revisado: version como inteiro (nao hex), context proof
      com signatures opcionalmente vazia para "proximity", Ap. C com
      fluxo end-to-end completo NFC transfer
