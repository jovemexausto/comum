# Teoria do Protocolo

## Fundamento

O Comum Protocol se organiza a partir de um unico atomo: **Testimony**.
Tudo o que o sistema faz e uma composicao de testemunhos assinados e
verificaveis. Nao existe camada magica: apenas evidencia, contexto e
historia.

## Separacao essencial

- **Semantica** (Claim): o que esta sendo afirmado.
- **Justificativa** (Context + Proof): sob que condicoes e com que evidencia.

Esta separacao impede que o protocolo confunda significado com prova.
Uma comunidade pode trocar o criterio de prova sem alterar a semantica.

## Abstracoes nucleares

- **Testimony**: evidencia minima, assinada e reproduzivel.
- **Claim**: descricao formal do ato (verbo + payload).
- **Context**: condicao de emissao (proximidade, beacon, place, vouch).
- **Proof**: conjunto de provas (assinaturas, zk, nullifiers).
- **Graph**: historia compartilhada derivada de referencias.

**Fluxos** sao padroes de leitura sobre o grafo. Nao sao entidades
armazenadas, nem precisam de um tipo proprio. Um fluxo e uma interpretacao
local de sequencias de Testimonies (via `refs` e `prev_id`).

## Commoner como unidade

O Comum usa **Commoner** como unidade central. Commoner e a entidade
que armazena, valida e sincroniza o grafo, e tambem a que emite
Testimonies com suas provas. Nao ha papeis separados: tudo isso e uma
mesma funcao operacional.

## Tempo e causalidade

O protocolo nao impone um relogio global. Timestamps sao locais e
insuficientes para ordenar o mundo inteiro. A causalidade e dada por
referencias (`prev_id`, `refs`) e pela leitura do grafo. A ordem social
emerge do grafo, nao de um tempo universal.

## Silencio e incompletude

A ausencia de um Testimony nao e negacao; e silencio. Uma `offer` sem
`accept`, um `propose` sem `close` ou um `vote` sem quorum nao sao
falhas do protocolo: sao estados sociais. O significado do silencio
e definido pela Capsula (ex.: `expires`).

## Fronteiras de comunidade

O grafo e global por natureza, mas a comunidade e um filtro de leitura.
Pertencer a uma comunidade e uma interpretacao sobre Testimonies (por
exemplo, Genesis e regras locais), nao um container separado de dados.

## O que o protocolo garante

- Determinismo de serializacao (CBOR canonical).
- Identidade por conteudo (hash de payload canonical).
- Convergencia do grafo via sync e referencias.
- Privacidade por padrao (sem GPS obrigatorio).

## O que o protocolo possibilita, mas nao define

- Semantica de verbos e contratos sociais.
- Criterios de legitimidade e excecoes comunitarias.
- Tempo social e regras de expiracao.
- Fronteiras e filtros de comunidade.

## O que o protocolo recusa deliberadamente

- Ordenacao global forçada.
- Consenso universal como requisito.
- Identidade emitida por autoridade externa.
- Localizacao fisica obrigatoria.
- Autoridade central de validacao.

## Criterios de legitimidade

Determinismo e propriedade do protocolo (reprodutibilidade tecnica).
Legitimidade e propriedade das Capsulas (decisao social). Uma comunidade
pode aceitar excecoes ou julgamento coletivo sem violar o protocolo.

## Anti-padroes

- Centralizar prova ou validacao em uma entidade global.
- Misturar semantica com prova e congelar criterios de legitimidade.
- Esconder limites do protocolo atras de UX ou infraestrutura.
