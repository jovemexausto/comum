# Plano de Implementacao

Legenda: [x] = feito, [~] = em andamento (apenas um por vez)

## Estado atual

- v0.3 entregue como sistema distribuido minimo funcional
- App = node (Commoner encapsulado)
- Sync via transporte (WebSocket no ambiente de teste)
- E2E multi-node com convergencia validada

## Roadmap ativo (v0.4)

Legenda: [x] = feito, [~] = em andamento (apenas um por vez)

- [~] Paridade de runtime (JS mobile vs Rust)
- [ ] Suite de conformance cross-runtime
- [ ] Fault injection em transporte (delay/drop/reorder)
- [ ] Extracao de ComumClient para o SDK
- [ ] Padronizacao de naming (boundary TS)
- [ ] Unificacao de API de transporte (WS/BLE/NFC)
- [ ] Remocao de vazamento de CBOR no nivel de app

## Direcao

- Foco em estabilidade, previsibilidade e interop
- Evitar expansao de features antes de garantir paridade
- Consolidar API de alto nivel antes de novos casos de uso

## Pesquisa aplicada — Coerencia (nao normativo)

- [x] coherence-sim com metricas (contagem, lacunas, repeticao, diversidade PHI)
- [x] coherence-sim com coherence_score (heuristica local)
- [x] Tabela de PHI esperado por verbo (nota tecnica)
- [x] Loop simulacao -> achado -> implicacao -> decisao (findings-001)
- [x] Nota de identidade e Sybil (posicionamento nao normativo)
