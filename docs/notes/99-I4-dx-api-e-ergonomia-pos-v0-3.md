---
note_class: "99"
integration_state: "I4"
status: "arquivo pos-v0.3"
destino: "archive"
rationale: "Captura um backlog localizado de DX apos v0.3 e deve ser mantido apenas como referencia historica."
---

# DX, API e Ergonomia — Pos v0.3 (nota)

Objetivo: capturar o estado atual de DX/API/ergonomia apos os avancos de
mobile node + E2E multi-node, e listar o backlog pos v0.3 sem tirar foco do
release.

## Estado atual (2026-04-07)

- App mobile agora modela node por instancia (Commoner encapsulado em AppNode).
- E2E multi-node automatizado existe com relay WebSocket real.
- `comum-js` expoe entrada React Native (`react-native -> dist/mobile.js`) sem N-API.
- Fluxo Feira ponta a ponta (offer/accept/receipt) esta operacional no app e no harness.

## Impacto em DX

- A barreira de entrada para testar topologia real caiu (nao depende de N-API para E2E de app).
- O foco de DX sai de "fazer rodar" para "garantir paridade e previsibilidade".
- Transporte plugavel virou requisito central de ergonomia (nao detalhe de implementacao).

## Impacto em API

- A API de alto nivel comecou a surgir (`AppNode`), mas ainda esta fora do SDK principal.
- Naming continua inconsistente entre fronteiras (`id_hex`/`payload_cbor` vs camelCase esperado em app).
- Builders de payload ajudam, mas app ainda enxerga detalhes de CBOR cedo demais.

## Ergonomia desejada

- `ComumClient` oficial no SDK: encapsular `Commoner + transport + sync + ingest`.
- Interface unica de transporte (`connect/publish/onMessage/close`) para WS/BLE/NFC.
- API orientada a acoes de capsula (ex.: Feira) sem exigir montagem manual de CBOR no app.
- Convencao de naming unica no boundary TypeScript (camelCase), com adaptacao interna.
- Script unico de bootstrap para contribuicao local (setup/build/test/e2e).

## Insights

- Unidade correta de produto e o node local, nao a tela.
- E2E valido precisa exercitar rede real e convergencia de estado, nao apenas chamada local.
- O maior risco de DX agora e drift semantico entre runtime Rust e runtime JS mobile.
- Ergonomia boa depende de contratos estaveis de API mais do que de wrappers ad-hoc.

## Backlog pos v0.3 (nao normativo)

- Parity suite: mesmos cenarios executando em runtime Rust (N-API) e runtime JS mobile.
- Fault injection no relay (drop/delay/reorder) para validar antifragilidade e sync.
- Extrair `AppNode` para `comum-js` como API publica (`ComumClient`).
- Introduzir semver de API ergonomica com changelog dedicado de DX.
