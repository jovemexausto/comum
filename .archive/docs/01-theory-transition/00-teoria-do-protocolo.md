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

`Commoner` e tanto o sujeito social que emite Testimonies quanto a
unidade tecnica que os valida e sincroniza. Essa unidade intencionalmente
nao distingue papeis internos: armazenar, validar, sincronizar e emitir
sao funcoes de um mesmo objeto. A CIP-0002 formaliza essa interface.

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

Comunidades nominais (Genesis) nao limitam interacoes. Qualquer Commoner
pode se relacionar com qualquer outro; essas relacoes geram comunidades
funcionais que emergem do grafo. As Capsulas atribuem significado local
a esses padroes sem bloquear a liberdade de relacao.

## Revogacao

Revogacao no Comum e local e relacional. Nao existe banimento universal,
porque isso exigiria autoridade global. Um Commoner pode ser revogado por
uma comunidade especifica, e o efeito global emerge da soma de revogacoes
locais. O grafo mantém todos os Testemunhos; revogacao altera a leitura,
nao o passado.

## Autoridade emergente auditavel

O Comum nao parte da ideia de nao-autoridade. Autoridade social e inevitavel.
O objetivo do protocolo e reduzir opacidade: tornar delegacao, escopo,
contestado e revogacao legiveis no grafo. Autoridade valida no Comum e sempre
contextual, limitada e revisavel por procedimentos locais.

A fundacao comunitaria minima pode nascer em um par (2). A triade (3) nao e o
inicio ontologico do comum, mas costuma ser o primeiro ganho forte de
triangulacao institucional: reduz impasse, amplia mediacao e melhora
accountability. Coletivos maiores podem elevar robustez conforme risco e
complexidade do contexto.

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

Notas tecnicas (nao normativas) podem registrar hipoteses de pesquisa
sobre leitura do grafo, como coerencia triadica e coercao mascarada.

## O que o protocolo recusa deliberadamente

- Ordenacao global forçada.
- Consenso universal como requisito.
- Identidade emitida por autoridade externa.
- Localizacao fisica obrigatoria.
- Autoridade central irrevisavel de validacao.

## Criterios de legitimidade

Determinismo e propriedade do protocolo (reprodutibilidade tecnica).
Legitimidade e propriedade das Capsulas (decisao social). Uma comunidade
pode aceitar excecoes ou julgamento coletivo sem violar o protocolo.
O protocolo nao substitui instituicoes locais; ele oferece rastro auditavel
para que instituicoes locais possam ser contestadas e transformadas.

## Anti-padroes

- Centralizar prova ou validacao em uma entidade global.
- Misturar semantica com prova e congelar criterios de legitimidade.
- Esconder limites do protocolo atras de UX ou infraestrutura.
