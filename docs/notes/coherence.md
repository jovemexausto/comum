# Coerencia no Comum (visao geral)

Este eixo registra uma **linha de pesquisa aplicada** sobre como ler
coerencia social em um grafo de Testimonies, sem introduzir regras
normativas no protocolo.

## O que e

- Uma investigacao sobre sinais locais (P/A/PHI) no grafo.
- Um conjunto de hipoteses testadas por simulacao.
- Um instrumento experimental (coherence-sim) com metricas simples.

## O que NAO e

- Nao faz parte das CIPs.
- Nao define validacao, consenso ou regras globais.
- Nao introduz score ou reputacao no protocolo.

## Componentes

- Modelo: `triadic-coherence-comum.md` (P/A/PHI aplicado a Testimony)
- Fenomeno: `coercive-masking.md` (coercao mascarada)
- Hipotese operacional: `phi-expectations.md` (PHI esperado por verbo)
- Simulacao: `impl/simulations/coherence-sim/`
- Resultados: `coherence-findings-001.md`
- Contexto relacionado: `identity-and-sybil.md`

## Metodo (resumo)

- Simulacao multi-cenario (coerente, gaps, repeticoes, diversidade)
- Metricas locais: gaps, repeticoes, diversidade de PHI
- Heuristica local (nao normativa): `coherence_score`

## Resultado atual

- Sinais de coerencia sao observaveis por leitura local do grafo, sem depender
  de validacao central irrevisavel.
- Gaps e repeticoes afetam leitura do grafo de forma distinta.
- Diversidade de contexto (PHI) pode mitigar comportamentos repetitivos.

## Limites

- Sensivel a definicao de PHI por capsula.
- Dependente de heuristicas locais (pesos variam).
- Nao generaliza sem validacao adicional.

## Status

Congelado como pesquisa aplicada (nao normativo).
Evolucao futura deve ocorrer via novas notas e simulacoes,
sem alterar o core do protocolo.
