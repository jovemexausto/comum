---
note_class: "03"
integration_state: "I1"
status: "nota tecnica viva"
destino: "stay-note"
rationale: "Extrai hipoteses operacionais do modelo triadico para simulacao e leitura local, sem virar norma do protocolo."
---

# Triadic Coherence no Comum (nota tecnica)

Objetivo: extrair do TCC (Triadic Coherence) um conjunto de hipoteses
operacionais para o Comum, sem transformar pesquisa externa em norma.
Tudo abaixo e nao normativo.

## 1) Modelo triadico (P/A/PHI)

Base conceitual do TCC: toda afirmacao social relevante pode ser
descrita como uma triade:

- P (Pessoa): quem emite ou sofre a acao.
- A (Acao): o que foi feito, prometido, observado ou alegado.
- PHI (Contexto): em que condicoes, quando, onde, com quais provas.

No Comum, a triade encaixa naturalmente em Testimony:

- P: author + subject (quando houver).
- A: verbo + payload normativo da capsula.
- PHI: Context + Proof (proximidade, beacon, place, vouch, etc.).

Implicacao: coerencia social pode ser tratada como consistencia
entre P/A/PHI ao longo do grafo, sem relogio global.

## 2) Hipoteses aplicaveis ao Comum

### 2.1 Coerencia triadica como sinal (hipotese)

- Proposta: medir coerencia entre P/A/PHI para ordenar ou filtrar
  Testimonies localmente (heuristica de leitura).
- Exemplo: um `capsule/accept` sem `comum/offer` anterior no mesmo
  contexto tem baixa coerencia local.
- Trade-off: heuristicas podem variar entre capsulas e fragmentar
  interpretacao.
- Destino: nota tecnica + simulacao (nao normativo).

### 2.2 Coerencia como anti-abuso local (hipotese)

- Proposta: usar baixa coerencia triadica como alerta de abuso
  (spam, coercoes, sequencias anormais).
- Exemplo: mesmo author com alta taxa de acoes sem PHI suficiente.
- Trade-off: falso positivo em comunidades com baixa instrumentacao.
- Destino: nota tecnica + vetores de teste.

### 2.3 Meta-adaptacao (hipotese)

- Proposta: capsulas podem ajustar suas regras quando detectam
  incoerencias sistematicas entre P/A/PHI (mudanca de regra local).
- Exemplo: reforcar necessidade de `comum/vouch` apos surto de spam.
- Trade-off: risco de oscilacao se ajustes forem muito reativos.
- Destino: nota tecnica; requer simulacao controlada.

## 3) O que NAO vira normativo agora

- Nenhuma metrica de coerencia vira requisito do protocolo.
- Nenhum score global, ranking ou reputacao e imposto pelo Comum.
- Nenhum algoritmo do TCC entra na CIP sem RFC e validacao.

## 4) Vetores de teste sugeridos (minimos)

- Sequencia coerente: offer -> accept -> settle com PHI suficiente.
- Sequencia incoerente: accept sem offer, ou sem PHI esperado.
- Inversao de contexto: PHI inconsistente entre testemunhos do mesmo P.

## 5) Riscos

- Heuristicas podem virar norma de fato se forem copiadas sem debate.
- Metrica unica pode induzir coercoes ou enviesar comunidades.

## 6) Proximos passos

- Formalizar exemplos triadicos em uma simulacao simples.
- Definir como capsulas registram PHI esperado por verbo.
- Avaliar se vale uma RFC para heuristicas de coerencia local.
