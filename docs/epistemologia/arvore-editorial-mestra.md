# Arvore editorial mestra

Objetivo: separar explicitamente duas coisas diferentes:

1. a documentacao tecnica e institucional do projeto no repo
2. o futuro livro/whitepaper teorico-politico do Comum

As duas partes se relacionam, mas nao devem colapsar uma na outra.

## Principio organizador

O projeto passa a ter dois corpus paralelos:

### A. Corpus tecnico do repo

- orientado a implementacao, especificacao, arquitetura, operacao e release
- serve a contribuidores, implementadores e leitores tecnicos
- permanece acoplado ao estado real do projeto

### B. Livro/whitepaper modular

- orientado a teoria, filosofia, economia politica e imaginario institucional
- fala apenas o suficiente de engenharia para sustentar os argumentos
- nao serve como manual de implementacao nem como SSOT tecnico

O erro a evitar e fazer o livro virar documentacao ou a documentacao virar ensaio.

## Arvore mestra proposta

```text
README.md
ABSTRACT.md

docs/
  00-introducao/
  10-protocolo/
  20-arquitetura/
    overview.md
    commoner.md
    wasm-abi.md
    transportes.md
    sdk-mobile-contract.md
    interop.md

  30-laboratorio/
    introducao.md
    notes/
      README.md
      00-metodo-epistemologico.md
      INDEX.md
    bloco-coerencia/
      coherence.md
      triadic-coherence-comum.md
      coercive-masking.md
      phi-expectations.md
      coherence-findings-001.md
    inferencia-social.md
    identidade-e-sybil.md
    notas-historicas-e-brainstorms.md

  40-governanca-do-corpus/
    mapa-epistemologico.md
    cartografia-do-acervo.md
    guia-editorial.md
    plano-de-reestruturacao.md

  50-roadmap-e-releases/
    roadmap.md
    implementation-plan.md
    release-notes/
      v0.1.md
      v0.2.md
      v0.3.md

  livro/
    00-o-que-e-o-comum.md
    01-apresentacao.md
    02-o-problema-da-coordenacao.md
    03-do-centro-ao-comum.md
    04-testemunho-e-legibilidade-publica.md
    05-contexto-prova-e-legitimidade.md
    06-autoridade-emergente-e-revogabilidade.md
    07-par-triade-e-escala-institucional.md
    08-comunidade-fronteira-e-federacao.md
    09-valor-plural-e-interoperabilidade.md
    10-compensacao-sem-equivalencia-unica.md
    11-infraestrutura-e-colonizacao-abstrata.md
    12-instituicoes-programaveis-e-limites.md
    13-conclusoes-e-perguntas-abertas.md

spec/
  cips/
  registries/
  schemas/
  test-vectors/
```

## Papel de cada bloco

### `README.md`

- manifesto publico
- porta de entrada do projeto
- tom politico e productish
- sem excesso de operacionalidade

### `ABSTRACT.md`

- compressao precisa do estado e do escopo real do repo
- ponte entre manifesto, spec, implementacao e corpus conceitual
- deve evitar promessas acima da implementacao atual
- bom ponto de entrada para leitores tecnicos, pesquisadores e revisores

### `docs/00-introducao/`

- textos de acolhimento e orientacao
- explica o projeto e orienta leitura tecnica
- sem assumir papel de livro teorico

### `docs/10-protocolo/`

- guias narrativos do nucleo normativo
- nao substitui as CIPs, mas as torna legiveis
- bom lugar para textos de visao geral tecnica e leitura orientada da spec

### `docs/20-arquitetura/`

- implementacao, contratos, runtime, SDK, interop
- documentos verificaveis, orientados a quem constroi

### `docs/30-laboratorio/`

- hipoteses, pesquisa aplicada, brainstorms, experimentos
- espaco onde ideias podem existir sem virarem compromisso editorial maduro
- notas novas devem ser curtas, classificadas e indexadas

### `docs/40-governanca-do-corpus/`

- metadocumentacao
- explica como o corpus se organiza, evolui e se disciplina

### `docs/50-roadmap-e-releases/`

- planejamento, contexto temporal e marcos entregues
- evita contaminar teoria e manifesto com cronograma

### `docs/livro/`

- corpus teorico, filosofico e politico
- modular, autoral e argumentativo
- usa engenharia apenas o suficiente para sustentar o argumento
- nao vira manual, contrato tecnico nem release note
- responde centralmente: o que e o Comum?

### `spec/`

- artefato normativo propriamente dito
- CIPs, registries, CDDL, vetores
- nao deve absorver notas teoricas ou material exploratorio

## Distribuicao sugerida do acervo atual

### Permanecem aproximadamente onde estao

- `README.md`
- `spec/cips/*`
- `spec/registries/*`
- `spec/schemas/*`
- `spec/test-vectors/*`
- `docs/release-notes/*`
- `docs/roadmap.md`
- `docs/implementation-plan.md`

### Devem migrar ou ser espelhados no corpus tecnico

- `docs/teoria-do-protocolo.md` continua como ponte conceitual tecnico-teorica do repo
- `docs/architecture/overview.md` -> `docs/20-arquitetura/overview.md`
- `docs/architecture/wasm-abi.md` -> `docs/20-arquitetura/wasm-abi.md`
- `docs/sdk/mobile-contract.md` -> `docs/20-arquitetura/sdk-mobile-contract.md`
- `docs/interop/README.md` -> `docs/20-arquitetura/interop.md`

### Devem alimentar principalmente o livro

- `docs/notes/interoperabilidade-sem-equivalencia-unica.md`
- `docs/notes/comum-e-hawala.md`
- `docs/notes/community-boundaries.md`
- `docs/notes/identity-and-sybil.md`
- `docs/notes/sintese-jornada-exploratoria.md`

### Devem permanecer em laboratorio exploratorio

- `docs/notes/brainstorm-inferencia-social.md`
- bloco de coerencia
- `docs/notes/dx-api-ergonomics-post-v0.3.md`

### Devem sustentar a governanca do corpus

- `docs/epistemologia/README.md`
- `docs/epistemologia/cartografia-do-acervo.md`
- `docs/epistemologia/plano-de-reestruturacao.md`

## Regras de fronteira editorial

### Uma ideia vai para `spec/` quando:

- exige comportamento interoperavel
- precisa virar contrato publico
- precisa ser testada por vetor ou conformance

### Uma ideia vai para o corpus tecnico quando:

- esclarece implementacao, arquitetura, contratos ou uso do protocolo
- ajuda a operar o projeto com rigor

### Uma ideia vai para o livro quando:

- trata de valor, soberania, pluralidade, federacao ou colonizacao
- e exige elaboracao argumentativa mais longa
- depende mais de teoria do que de contrato tecnico

### Uma ideia fica em `30-laboratorio/` quando:

- ainda esta em exploracao
- depende de experimento
- pode ser abandonada sem dano ao corpus central

### Uma ideia pode aparecer nos dois corpus quando:

- o repo precisa de uma versao curta e operacional
- o livro precisa de uma versao longa e conceitual

Nesse caso, os textos devem ser explicitamente diferentes, nao copias disfarçadas.

## Sequencia editorial recomendada

1. Fixar a arvore mestra
2. Separar guia editorial do repo e esboco do livro
3. Criar as pastas-mestras vazias
4. Migrar os documentos tecnicos maduros
5. Derivar o sumario argumentativo do livro
6. Reindexar tudo com referencias cruzadas

## Criterio de sucesso

O corpus estara bem estruturado quando:

- a spec ficar mais legivel sem ficar mais inchada
- teoria e exploracao deixarem de competir com CIPs
- o livro ganhar casa propria sem virar documentacao
- notas deixarem de ser o unico lugar da elaboracao conceitual

## Status

Estrutura editorial mestra proposta para orientar a reorganizacao do corpus.
