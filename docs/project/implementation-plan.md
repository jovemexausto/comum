# Plano de Implementacao

Legenda: [x] = feito, [~] = em andamento (apenas um por vez)

## Estado atual

- v0.3 entregue como sistema distribuido minimo funcional
- App = node (Commoner encapsulado)
- Sync via transporte (WebSocket no ambiente de teste)
- E2E multi-node com convergencia validada

## Roadmap ativo (v0.4)

Legenda: [x] = feito, [~] = em andamento (apenas um por vez)

- [~] Feira MVP como vertical slice principal
- [ ] Extracao de ComumClient para o SDK
- [ ] Remocao de vazamento de CBOR no nivel de app
- [ ] Paridade de runtime no fluxo Feira (JS mobile vs Rust)
- [ ] Suite de conformance cross-runtime focada no fluxo Feira
- [ ] Fault injection em transporte (delay/drop/reorder)
- [ ] Padronizacao de naming (boundary TS)
- [ ] Unificacao de API de transporte (WS/BLE/NFC)

## Direcao

- Foco em um produto vivo antes de expandir teoria ou paper
- Usar Feira como prova pratica do Comum rodando entre nos reais
- Fechar API de alto nivel e fluxo de app antes de abrir novos vertical slices
- Paridade importa na medida em que sustenta o fluxo Feira completo

## Feira MVP

Definicao curta:

- duas instancias do app conseguem publicar oferta, aceitar oferta e observar o
  receipt sincronizado sem precisar lidar com CBOR cru ou detalhes internos de
  Commoner

Submetas:

- `ComumClient` ou superficie equivalente no SDK
- fluxo app -> SDK -> transporte -> sync -> capsula sem vazamento excessivo de detalhes
- receipt observavel no app remoto apos convergencia
- roteiro manual curto e reprodutivel

Critero de pronto:

- fluxo completo reproduzivel entre dois nos
- comportamento coerente entre JS mobile e runtime canônico no que o fluxo expõe
- demo explicavel sem exigir leitura do repo inteiro

## Trabalho secundario

- paper 1 continua vivo, mas subordinado ao que o produto realmente sustentar
- teoria nova so sobe de prioridade se mudar o vertical slice ou o contrato

## Pesquisa aplicada — Coerencia (nao normativo)

- [x] coherence-sim com metricas (contagem, lacunas, repeticao, diversidade PHI)
- [x] coherence-sim com coherence_score (heuristica local)
- [x] Tabela de PHI esperado por verbo (nota tecnica)
- [x] Loop simulacao -> achado -> implicacao -> decisao (findings-001)
- [x] Nota de identidade e Sybil (posicionamento nao normativo)
