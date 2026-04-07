# Identidade e Sybil no Comum (nota tecnica)

Objetivo: explicitar a posicao do Comum sobre criacao de identidades
(Commoners) e ataques Sybil, sem introduzir regras normativas.

## Principio

- Identidades sao livres: qualquer agente pode criar um Commoner.
- O protocolo nao tenta impor escassez de identidade.

## Consequencia

- Ataques Sybil sao possiveis no nivel de identidade.
- O protocolo nao tenta preveni-los globalmente.

## Mecanismo real de filtragem

O Comum desloca a confianca de identidade para relacao:

- `comum/vouch` (endosso)
- `comum/encounter` (presenca/relacao)
- PHI (contexto verificavel)
- Historico coerente no grafo

## Intuicao

Identidades sao baratas.
Relacoes sao caras.

Ataques Sybil criam muitas identidades, mas nao conseguem facilmente
produzir relacoes confiaveis em escala.

## Implicacao

- Sybils tendem a ser ignorados por filtros locais.
- Capsulas definem criterios de relevancia (nao o protocolo).
- Legitimidade e emergente, nao imposta.

## Limites

- Comunidades pequenas sao mais vulneraveis.
- Bootstrapping inicial exige cuidado.
- Capsulas mal definidas podem aceitar ruido.

## Relacao com coerencia

Sybils tendem a apresentar:

- baixa coerencia (lacunas)
- repeticao de padroes
- baixa diversidade de PHI

Esses sinais podem ser usados por heuristicas locais (nao normativas).

## Status

Posicao documentada. Nao normativo.
