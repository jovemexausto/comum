# Notas de Trabalho — Fronteiras de Comunidade

## Problema

O grafo e global por natureza, mas a comunidade e uma interpretacao local.
Precisamos explicitar como um Commoner decide "o que pertence" a uma
comunidade sem transformar comunidade em container fisico.

## Hipoteses de trabalho

- Comunidade = filtro de leitura, nao particao de armazenamento.
- Community ID ancora no Genesis (id do Testemunho de Genesis).
- Um Testimony pode ser relevante para multiplas comunidades.

## Questao central

Qual criterio minimo um Commoner usa para dizer: "este Testimony pertence
a comunidade X"?

## Caminhos possiveis

1) **Genesis-first**
   - Pertence se e alcançavel por `refs` a partir do Genesis daquela comunidade.
   - Pro: simples, deterministico.
   - Contra: depende de disciplina social nos `refs`.

2) **Namespace por verbos/payload**
   - Pertence se o payload declara `community_id` (quando aplicavel).
   - Pro: explicito.
   - Contra: nem todo verbo tem esse campo.

3) **Capsula define fronteira**
   - Cada capsula aplica suas regras (ex.: Agora filtra por Genesis local).
   - Pro: respeita autonomia local.
   - Contra: pode fragmentar interoperabilidade.

4) **Filtro composto**
   - Genesis-first + campos explicitos quando existirem.
   - Pro: equilibrio entre rigor e praticidade.

5) **Membership explicito via Testimony**
   - Pertence se existe `comum/member` valido para o author,
     co-assinado por fundadores ou aprovado por Capsula de entrada.
   - Pro: auditavel, revogavel, explicito.
   - Contra: adiciona verb e carga de membership.

## Questao aberta

Como evitar que um adversario injete Testimonies "estranhos" e force
interpretacoes indevidas sem quebrar o principio de grafo global?

## Criterio minimo recomendado (nao normativo para v0.1)

Filtro composto:

- Genesis-first para identidade da comunidade.
- `comum/member` para verificar pertencimento do author.
- Campos explicitos de `community_id` quando presentes.
- Capsula aplica regras adicionais.

Formalizar como CIP quando houver implementacao validada.

## Proximos passos

- Decidir o criterio minimo (provavelmente o filtro composto).
- Formalizar no CIP (se virar regra normativa).
- Adicionar exemplos no manual e na teoria.

## Nota sobre revogacao

Revogacao e sempre local a uma comunidade. Uma revogacao nao remove
o Commoner do grafo, apenas altera a leitura e os fluxos futuros
naquela comunidade. O efeito global e emergente.
