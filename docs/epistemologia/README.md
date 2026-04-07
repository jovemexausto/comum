# Mapa Epistemologico do Comum

Objetivo: delimitar o campo conceitual do projeto sem confundir especificacao,
implementacao e exploracao. Este documento funciona como esboco de pagina
unica para um futuro livro/whitepaper modular.

## O que o Comum e

O Comum nao e apenas um protocolo de dados, nem apenas um app, nem apenas uma
teoria politica. E uma tentativa de construir coordenacao verificavel entre
comunidades soberanas sem depender de autoridade central, consenso global ou
plataforma unica.

Seu nucleo tecnico e simples: Testimonies assinados, grafo causal,
serializacao deterministica, sync eventual e semantica local em Capsulas.

Seu nucleo politico e mais exigente: interoperar sem absorver, coordenar sem
colonizar, formalizar sem esmagar o contexto.

## O problema central

O projeto gira em torno de uma tensao permanente:

> Como criar abstracao suficiente para coordenar sem criar abstracao demais a
> ponto de substituir os mundos locais que se pretende servir?

Essa tensao aparece em varios pares:

- soberania vs interoperabilidade
- equivalencia vs pluralidade
- infraestrutura vs poder
- semantica comum vs captura ontologica
- federacao vs centro

## O campo conceitual

Hoje o Comum ja toca, ao menos, os seguintes dominios:

- teoria do valor
- soberania
- mediacao
- forma monetaria
- coordenacao entre comuns
- centro vs federacao
- equivalencia vs pluralidade
- infraestrutura vs poder
- marxismo
- anarquismo
- federalismo
- teoria institucional
- antropologia economica
- teoria monetaria
- sistemas complexos

Isso nao significa que o projeto precise resolver todos esses campos. Significa
que ele inevitavelmente toma posicao neles, mesmo quando fala apenas em
arquitetura.

## Cinco eixos epistemicos

### 1. Nucleo protocolar

Pergunta:

> Qual e o minimo comum necessario para que haja coordenacao verificavel?

Este eixo cobre:

- Testimony, Claim, Context, Proof
- DAG, causalidade, sync eventual
- CBOR canonical, hashing, suites, CTE
- Commoner como unidade tecnica e social

Este e o `thin waist` do sistema: fino o bastante para nao colonizar o local,
forte o bastante para sustentar interoperabilidade.

### 2. Semantica e instituicoes programaveis

Pergunta:

> Como regras sociais locais ganham forma executavel sem virar plataforma?

Este eixo cobre:

- verbos e rotulos semanticos minimos
- Capsulas como interpretacao executavel
- fluxos como leitura local do grafo
- governanca, mercado, mutirao, confianca

Aqui o Comum deixa de ser so protocolo e vira base para instituicoes
programaveis e forkaveis.

### 3. Coordenacao sob incerteza social

Pergunta:

> Como decidir sem verdade global, com sinais incompletos e relacoes assimetricas?

Este eixo cobre:

- inferencia distribuida eventual
- politicas de confianca
- vouch, encounter, receipts e contexto como pesos implicitos
- teoria dos jogos como lente para comportamento emergente
- inteligencia social programavel como hipotese exploratoria

Aqui a questao nao e descobrir a verdade absoluta, mas agir com seguranca
suficiente sem centralizar julgamento.

### 4. Economia politica da interoperabilidade

Pergunta:

> Como permitir interoperabilidade sem reduzir a pluralidade a uma equivalencia unica?

Este eixo cobre:

- economias locais soberanas
- remessa e compensacao intercomunitaria
- hawala como inspiracao e limite
- petrodolar como advertencia politica
- risco de moeda imperial disfarcada de infraestrutura

Aqui a fronteira mais sensivel e entre uma infraestrutura federativa de borda e
uma soberania monetaria abstrata que recoloniza o local.

### 5. Limites anti-coloniais do proprio protocolo

Pergunta:

> Como impedir que o proprio Comum vire um colonizador abstrato?

Este eixo cobre:

- o direito ao opaco
- reversibilidade politica
- pluralidade semantica real
- distincao entre abstracao necessaria e abstracao totalizante
- o criterio: ampliar autonomia local ou obrigar o local a caber no protocolo?

Este eixo precisa permanecer vivo como freio interno do projeto.

## Tipos de documento

Para o corpus nao colapsar, vale separar com rigor:

### 1. Especificacao normativa

- CIPs
- registries
- CDDL
- vetores de teste

Aqui entram apenas invariantes, formatos e contratos publicos.

### 2. Arquitetura e implementacao

- overview de camadas
- ABI/WASM
- SDK/mobile contract
- docs de interop e release

Aqui entram escolhas de execucao, limites e integracao.

### 3. Teoria do protocolo

- conceitos-base do sistema
- separacoes essenciais
- criterios de legitimidade e recusa

Aqui entram os fundamentos conceituais que iluminam a spec, sem se confundir
com ela.

### 4. Economia politica e filosofia do Comum

- valor, pluralidade, soberania, federacao
- compensacao intercomunitaria
- anti-colonizacao do protocolo

Aqui entram as tensoes mais largas que orientam o projeto, mesmo sem se tornar
norma tecnica imediata.

### 5. Laboratorio exploratorio

- notas de brainstorm
- hipoteses de pesquisa
- playgrounds conceituais
- achados ainda nao consolidados

Aqui ficam ideias vivas sem necessidade de fechamento prematuro.

## Regra de ouro

O Comum nao deve tentar resolver tudo no core.

Ele deve:

- formalizar apenas o necessario para coordenar
- deixar espaco para pluralidade local de valor e instituicao
- permitir federacao sem absorcao
- recusar a tentacao de virar ontologia universal do social

## Frase-guia

Se houver uma frase para orientar a futura reestruturacao do corpus, talvez seja:

> O Comum precisa de semantica compartilhada suficiente para coordenar, mas nao
> de tanta semantica a ponto de substituir a vida local que pretende servir.

## Status

Esboco epistemologico inicial para orientar futura reestruturacao da
documentacao em livro/whitepaper modular.
