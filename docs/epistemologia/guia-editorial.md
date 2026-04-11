# Guia editorial do corpus tecnico

Objetivo: disciplinar a documentacao tecnica e institucional do projeto no
repo, impedindo mistura recorrente entre especificacao, arquitetura,
implementacao, exploracao e elaboracao teorica de longa duracao.

## Regra principal

O repo nao e o livro.

O corpus tecnico do repo deve servir a:

- implementacao
- interoperabilidade
- verificacao
- operacao do projeto
- contexto de desenvolvimento

Quando uma elaboracao exigir desenvolvimento filosofico, politico ou economico
mais longo, ela deve alimentar `docs/livro/` ou permanecer como nota
exploratoria, sem capturar a documentacao operacional.

## Tipos de documento

### 1. Normativo

Onde:

- `spec/cips/`
- `spec/registries/`
- `spec/schemas/`
- `spec/test-vectors/`

Funcao:

- definir contratos publicos e interoperaveis

Tom:

- seco
- preciso
- verificavel
- sem especulacao

### 2. Arquitetura e implementacao

Onde:

- `docs/architecture/`
- `docs/sdk/`
- `docs/interop/`
- READMEs de implementacao

Funcao:

- explicar como o sistema e construido e operado

Tom:

- tecnico
- concreto
- orientado a decisao e verificacao

### 3. Teoria do protocolo (curta e disciplinada)

Onde:

- `docs/teoria-do-protocolo.md`
- textos conceituais estritamente ligados a leitura da spec

Funcao:

- esclarecer o modelo do sistema sem legislar nada novo

Tom:

- conceitual
- compacto
- sem virar ensaio autonomo

### 4. Exploratorio / laboratorio

Onde:

- `docs/notes/`
- `docs/simulations.md`

Funcao:

- registrar hipoteses, achados, brainstorms e direcoes em aberto
- registrar excedentes, aberturas, curiosidades e dossies sem inflar o centro
  teorico nem a spec

Tom:

- honesto
- provisoriamente incompleto
- explicitamente nao normativo

Taxonomia recomendada para novas notas:

- `01-...`: excedente
- `02-...`: aberto
- `03-...`: curiosidade
- `90-...`: dossie
- `99-...`: arquivo

### 5. Planejamento e contexto temporal

Onde:

- `docs/roadmap.md`
- `docs/implementation-plan.md`
- `docs/release-notes/`
- `docs/context.md`

Funcao:

- registrar o estado do projeto no tempo

Tom:

- objetivo
- contextual
- sem teoria excessiva

## O que NAO deve acontecer

- CIPs virarem ensaios
- README virar manual interno
- notas exploratorias virarem pseudo-spec
- documento tecnico carregar sozinho o debate politico inteiro
- livro virar copia em prosa da documentacao do repo

## Quando uma ideia vai para cada lugar

### Vai para `spec/` quando:

- precisa virar contrato interoperavel
- exige vetor, schema ou registry
- precisa ser obedecida por multiplas implementacoes

### Vai para docs tecnicos quando:

- ajuda implementadores a construir, integrar ou verificar o sistema
- explica arquitetura ou contrato de runtime/SDK

### Vai para `docs/notes/` quando:

- ainda esta em elaboracao
- depende de pesquisa ou experimento
- pode ser abandonada sem dano a coerencia do projeto

### Vai para `docs/livro/` quando:

- exige desenvolvimento teorico longo
- trata de valor, soberania, pluralidade, federacao ou colonizacao
- nao precisa virar contrato tecnico para ser importante

## Marcadores de status recomendados

Todo documento nao normativo deveria deixar claro um destes status:

- ativo
- exploratorio
- maduro
- legado
- precisa atualizar

## Regra de promocao editorial

Uma nota pode subir de nivel quando:

- deixou de ser apenas brainstorm
- ganhou pergunta clara, vocabulario proprio e tese reconhecivel
- ja influencia decisoes do projeto de forma consistente

Neste caso, ela pode:

- virar capitulo do livro
- virar secao de teoria do protocolo
- virar documento tecnico mais claro

Antes de uma nota subir, perguntar:

- a tese ja esta clara?
- o leitor ideal ja apareceu?
- o texto ainda cabe numa nota curta ou virou capitulo/dossie?

## Regra de duplicacao aceitavel

Uma mesma ideia pode aparecer em mais de um corpus, desde que cumpra papeis
diferentes.

Exemplo:

- no repo tecnico: forma curta, operacional, suficiente
- no livro: forma longa, argumentativa, teorica

Nao vale manter duas copias equivalentes do mesmo texto.

## Regra de linguagem

- README: manifesto e productish
- spec: normativa e seca
- arquitetura: tecnica e verificavel
- notas: abertas e explicitamente provisórias
- notas: curtas, classificadas e com destino epistemologico claro
- livro: teorico-politico, com engenharia apenas como sustentacao

## Criterio de qualidade editorial

Um documento esta bem colocado quando:

- seu leitor ideal e evidente
- seu tom corresponde ao seu papel
- seu tipo de verdade e claro
- ele nao invade a funcao de outro tipo de documento

## Status

Guia editorial inicial do corpus tecnico do repo.
