# Simulacoes Locais

Objetivo: exercitar fluxos sociais e de sync sem hardware real. Cada
cenario mostra um padrao minimo de Testimonies, por que importa e
como interpretar a saida.

## Cenario 1 — Encounter + Transfer + Sync

O que simula: encontro e transferencia em rede offline, seguidos de
convergencia via sync.
Por que importa: valida o caminho minimo de causalidade (refs) e a
propagacao eventual sem relogio global.
O que significa: ao final, o Commoner C deve reconstruir o mesmo grafo
e estado derivado que A e B conhecem localmente.

1. A e B registram um encounter (comum/encounter)
2. A cria uma transferencia (comum/transfer)
3. A sincroniza com C
4. B sincroniza com C

Execucao:

```
just sim
```

Saida esperada:

- C recebe 2 testemunhos (encounter + transfer)
- Estado final mostra contagem e saldos derivados

## Cenario 2 — Agora (governanca)

O que simula: ciclo de decisao local (proposta, voto, fechamento).
Por que importa: mostra como a capsula aplica regras sociais sem impor
ordenacao global, usando apenas referencias.
O que significa: o fechamento reflete quorum e votos observados pelo
Commoner avaliador.

Fluxo minimo: propose -> vote -> close.

```sh
just agora-sim
```

## Cenario 3 — Feira (mercado local)

O que simula: oferta, aceitacao e comprovante de entrega.
Por que importa: valida encadeamento de acoes economicas e o minimo
necessario para resolver uma troca local.
O que significa: a receipt confirma a conclusao da troca no grafo.

Fluxo minimo: offer -> accept -> receipt.

```sh
just feira-sim
```

## Cenario 4 — Mutirao (trabalho coletivo)

O que simula: ciclo de trabalho coletivo com compromissos e recompensa.
Por que importa: exercita dependencias entre tarefas e validacao de
progresso sem dependencia de autoridade central irrevisavel.
O que significa: a sequencia produz um fluxo completo de trabalho local.

Fluxo minimo: task -> commit -> checkin -> complete -> reward.

```sh
just mutirao-sim
```

## Cenario 5 — Coerencia triadica (hipotese)

O que simula: sequencia coerente (offer -> accept -> receipt) e
sequencias incoerentes (accept sem offer, repeticao de accept).
Por que importa: oferece vetores minimos para testar hipoteses de
coerencia local sem tornar heuristicas normativas.
O que significa: as sequencias incoerentes sao visiveis no grafo e
podem ser tratadas por capsulas ou heuristicas locais.

```sh
just coherence-sim
```

## Cenario 6 — Feira E2E (JS)

O que simula: fluxo completo em JS usando Commoner + capsula Feira.
Por que importa: valida a camada de SDK e o caminho ponta-a-ponta.
O que significa: os mesmos Testimonies sao gerados e ingeridos em JS.

```sh
(cd impl/comum-js && npm run e2e:feira)
```

Requer `comum-napi` disponivel (instalado ou via `COMUM_NAPI_PATH`).
