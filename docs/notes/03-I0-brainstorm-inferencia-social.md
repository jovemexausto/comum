---
note_class: "03"
integration_state: "I0"
status: "brainstorm bruto preservado"
destino: "stay-note"
rationale: "Guarda intuicoes laterais sobre inferencia social programavel sem promover centralidade prematura."
---

# Brainstorm — inferencia social programavel (nao normativo)

Objetivo: guardar ideias exploratorias sobre inferencia distribuida, aprendizagem
local-first e experimentos com o sistema atual, sem virar roadmap.

## Intuicao central

- O sistema permite inferencia distribuida eventual: sinais locais + relacoes + contexto.
- O peso de um testimony depende de quem emite e da relacao com o emissor.
- Isso nao substitui verdade tecnica (validacao), apenas orienta decisao local.

## Modelo mental

Camadas:

1. Sinais: testimonies (eventos locais)
2. Estrutura: relacoes (vouch, encounter) e contexto (PHI)
3. Transformacao: capsules interpretam historico e emitem novos testimonies

Loop:

eventos -> relacoes -> interpretacao -> novo evento -> ...

## Diferenca vs ML classico

- Nao ha dataset global nem treino central.
- O aprendizado e experiencial: ajusta-se com o historico vivido.
- O comportamento emerge, nao e imposto.

## Decidir sob incerteza social

- Decisao nao e descobrir a verdade, e escolher acao com base em sinais incompletos.
- Tres fontes principais:
  - experiencia direta
  - relacoes (vouch / encounter)
  - contexto (PHI)

Regra minima:

if experiencia || vouch confiavel || contexto forte -> agir
else -> ignorar

Decisoes sao reversiveis e acumulativas: feedback ajusta thresholds locais.

## Teoria dos jogos (aplicacao adequada)

- Mais proximo de jogos repetidos com informacao incompleta.
- Payoff e vivido (entrega/recibo), nao abstrato.
- Serve para analisar o que emerge, nao para prescrever regras globais.

## Ideias de experimentos (playground)

1) Mercado fantasma
   - Muitos nodes publicam offers/accepts automaticamente.
   - Observa convergencia e densidade de transacoes.

2) Gossip caotico
   - Relay com delay/drop/duplicate.
   - Mede robustez e tempo de convergencia.

3) Ataque Sybil controlado
   - Muitos nodes falsos, poucos reais.
   - Observa como relacoes emergem.

4) Verdade local
   - Testimonies conflitantes.
   - Capsule resolve baseado em contexto/relacao.

5) Aprendizado sem treino
   - Ajusta thresholds conforme receipts positivos/negativos.

## Politicas de confianca (exemplos)

- Conservadora: exige experiencia direta.
- Relacional: aceita se ha vouch de confianca.
- Oportunista: aceita quase tudo e aprende rapido.

## Termos-chave

- inferencia distribuida eventual
- inteligencia social programavel
- consenso local, nao global
