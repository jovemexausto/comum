# Coherence Findings 001

## Hipotese

Sequencias sem antecedente esperado (gaps) e repeticoes sao sinais de
baixa coerencia.

## Cenario

coherence-sim com:
- fluxo coerente (offer -> accept -> receipt)
- accept sem offer
- repeticao de accept

## Sinais observados

- Resultados por cenario:

- A_coherent: score=3 (gaps=0, repeats=0, phi=1)
- B_gaps: score=0 (gaps=1, repeats=0, phi=1)
- C_repeats_low_div: score=0 (gaps=0, repeats=3, phi=1)
- D_repeats_high_div: score=1 (gaps=0, repeats=3, phi=2)

## Implicacao

- Gaps penalizam fortemente a coerencia (queda de 3 pontos).
- Repeticao sem diversidade reduz coerencia, mas menos que gaps.
- Aumento de diversidade de PHI mitiga parte do impacto de repeticoes.

## Decisao

- Manter como hipotese (nao normativo)
- Expandir cenarios com mais variacao de PHI e multiplos gaps
- Avaliar outras funcoes de score (sensibilidade a pesos)
