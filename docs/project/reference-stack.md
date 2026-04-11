# Reference Stack

Status: ativo

Objetivo: explicitar a disciplina de artefatos do Comum para humanos e agentes.

O Comum nao deve operar com uma fonte unica de verdade. Ele opera com uma stack
de referencia em que cada camada responde a uma pergunta diferente e limita as
demais.

## Tese central

O corpus, sozinho, nao basta.

A spec, sozinha, nao basta.

Implementacao, sozinha, nao basta.

Claims publicos responsaveis surgem apenas quando sentido, contrato, execucao e
evidencia se encontram de modo coerente.

Formula curta:

> Corpus da sentido, spec da obrigacao, implementation da execucao, evidence da
> permissao, e paper da claim publica.

## Classes de artefato

### 1. Corpus

Casa:

- `docs/corpus/`

Pergunta principal:

- o que isso quer dizer?

Funcao:

- razao de ser do Comum
- ontologia politica
- distincao entre conceitos
- limites da formalizacao
- horizonte institucional

O corpus tem prioridade interpretativa. Ele orienta o significado do projeto,
mas nao substitui a spec e nao autoriza claims publicos sozinho.

### 2. Spec

Casa:

- `spec/`

Pergunta principal:

- o que deve ser obedecido?

Funcao:

- contrato normativo
- requisitos MUST/SHOULD/MAY
- schemas
- registries
- vetores oficiais

A spec tem prioridade normativa. Ela traduz parte do corpus em obrigacoes
interoperaveis. Nem tudo que faz sentido no corpus deve subir para a spec.

### 3. Implementation

Casa:

- `crates/`
- `packages/`
- `capsules/`
- `simulations/`
- `docs/implementation/`

Pergunta principal:

- como isso existe hoje?

Funcao:

- comportamento executavel
- restricoes de runtime
- superficie tecnica real
- interfaces e fluxos implementados

Implementation tem prioridade operacional. Ela mostra o estado real do projeto,
mas nao vira norma automaticamente e nao substitui o corpus.

### 4. Evidence

Casa principal:

- `tests/`
- `simulations/`
- test commands documentados
- pacotes de paper em `docs/project/`

Pergunta principal:

- o que ja foi demonstrado?

Funcao:

- testes automatizados
- conformance
- simulacoes
- benchmarks
- casos reproduziveis

Evidence tem prioridade epistemica para claims externos. Sem evidence, o projeto
tem intuicao ou tese, nao demonstracao suficiente.

### 5. Paper

Casa de preparacao:

- `docs/project/`

Pergunta principal:

- o que podemos afirmar publicamente agora?

Funcao:

- framing externo
- claims
- evidence selecionada
- limits
- estrategia de venue

Paper nao e dump do corpus. E projecao publica limitada por evidence e mediada
pela spec quando houver claim tecnica ou de interoperabilidade.

### 6. Project

Casa:

- `docs/project/`

Pergunta principal:

- o que estamos fazendo agora?

Funcao:

- roadmap
- plano de implementacao
- releases
- paper packets
- disciplina de artefatos

Project coordena tempo, cadencia e prioridade. Nao substitui corpus, spec ou
implementation.

### 7. Notes

Casa:

- `docs/notes/`

Pergunta principal:

- o que ainda esta aberto, excedente ou nao integrado?

Funcao:

- laboratorio curto
- exploracao
- tensoes ainda sem formulacao final
- material em promocao potencial

Notes nao devem competir com corpus, spec ou project.

## Fluxo entre artefatos

Fluxo recomendado:

```text
notes -> corpus -> spec -> implementation
                 -> paper

implementation + tests + simulations -> evidence -> paper

project coordena o tempo e a embalagem dos artefatos
```

Leitura pratica:

- notes alimentam corpus
- corpus orienta spec
- spec constrange implementation
- implementation e testes produzem evidence
- paper projeta ao exterior apenas o que corpus formula e evidence sustenta

## Regras fortes

### 1. Corpus is not enough

Nenhuma claim publica relevante nasce apenas do corpus.

### 2. Spec is not enough

Contrato tecnico nao substitui sentido, nem demonstra por si so que algo funciona
na pratica.

### 3. Code is not norm

Implementacao nao vira verdade so por existir. Quando houver conflito, a spec
normativa vence; quando faltar norma, isso deve ser dito claramente.

### 4. Evidence gates claims

Toda afirmacao externa relevante deve ser classificada como:

- demonstrada
- parcialmente demonstrada
- conjectural

### 5. Paper is bounded projection

Paper nao deve ser maior que a evidencia. Se uma tese ainda depende de simulacao,
deployment, benchmark ou caso real, isso precisa aparecer como limitacao.

## Regras de promocao

### Notes -> Corpus

Promover quando:

- a tese estabilizou
- o texto deixou de ser brainstorming
- ja influencia o vocabulario do projeto

### Corpus -> Spec

Promover quando:

- o conceito precisa virar obrigacao interoperavel
- implementacoes diferentes precisam obedecer o mesmo contrato

### Spec -> Implementation

Promover quando:

- o requisito normativo precisa ganhar execucao real

### Implementation -> Evidence

Promover quando:

- o comportamento esta coberto por teste, conformance, simulacao ou benchmark

### Corpus + Spec + Evidence -> Paper

Promover quando:

- o framing esta claro
- os claims estao delimitados
- a evidence e suficiente para sustentar a afirmacao publica
- os limits estao explicitados

## Regras anti-drift

- corpus nao deve funcionar como spec informal
- spec nao deve carregar teoria longa desnecessaria
- implementation nao deve inventar contrato proprio
- paper nao deve vender o que ainda e apenas intuicao interna
- project nao deve competir com corpus ou spec
- notes nao devem virar segundo corpus

## Uso por agentes

Ao analisar uma tarefa, um agente deve perguntar:

1. o problema principal e de sentido, contrato, execucao, evidencia, claim
   publica, prioridade temporal ou exploracao?
2. qual camada da stack tem prioridade neste caso?
3. o resultado esperado e promover, restringir, demonstrar, ou apenas registrar?

## Criterio de sucesso

A reference stack esta funcionando quando:

- o corpus orienta sem dominar tudo
- a spec contrai sem empobrecer o sentido
- a implementacao executa sem inventar norma
- a evidence limita claims publicos
- papers ficam mais fortes e mais honestos
