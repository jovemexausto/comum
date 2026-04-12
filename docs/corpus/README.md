# Livro do Comum

Objetivo: fazer de `docs/corpus/` o fundamento teorico, politico e filosofico do
projeto, sem confundir esse corpus com especificacao, manual de implementacao ou
changelog.

## Pergunta central

> O que e o Comum, e contra que condicao ele se ergue?

O corpus se organiza em torno de uma contra-tese. Se o tecnofeudalismo e a forma
pela qual a mediacao digital se tornou senhorio — identidade como concessao,
memoria como refem, atencao como renda, saida como punicao — entao o Comum e a
tentativa de construir uma contra-arquitetura federativa: comunidades soberanas,
com suas proprias economias, governancas e criterios de legitimidade,
relacionando-se por um substrato tecnico fino o bastante para coordenar e fraco o
bastante para nao virar o novo centro.

Resposta provisoria:

> O Comum e uma infraestrutura para produzir legibilidade publica,
> autoridade local auditavel e relacao federativa entre mundos que
> permanecem diferentes. E tecnofederalismo como contra-tese ao
> tecnofeudalismo.

## O que o corpus faz

- nomeia a condicao feudal da mediacao digital como o adversario
- formula a razao de ser do Comum como contra-arquitetura
- explicita sua ontologia politica
- elabora os limites da formalizacao
- situa testemunho, legitimidade, autoridade, federacao e valor plural num mesmo
  campo teorico
- usa engenharia apenas o suficiente para sustentar o argumento
- examina por que as respostas anteriores (blockchain, fediverso, plataformas
  civicas) falharam de modos especificos

## Ontologia minima do projeto

Para manter o corpus, a spec e a implementacao alinhados, o projeto assume um
glossario curto de camadas:

- `Comum` nomeia o protocolo/projeto, nao uma moeda universal.
- `Node` e a instancia tecnica local.
- `Commoner` e a fachada tecnica de no.
- `App` e superficie de uso, nao a comunidade.
- `Capsule` e instituicao executavel parcial, nao a totalidade da vida social.
- `Comunidade nominal` e ancorada por `Genesis`; `comunidade funcional` emerge do grafo.
- `Regime local de valor` pertence a comunidades e suas instituicoes.
- `Federacao de borda` trata traducao e compensacao limitadas entre mundos diferentes.
- `Identidade soberana` deve permanecer portavel e desacoplada de um app unico.

## O que o corpus nao faz

- nao substitui `spec/`
- nao funciona como SSOT tecnico
- nao e roadmap
- nao deve repetir README, CIPs ou docs operacionais em prosa longa

## Tese editorial

Quatro eixos recentram o corpus:

1. O Comum nao deve mais ser narrado apenas por negacao do centro. Ele deve
   nomear o adversario: a forma feudal da mediacao digital.
2. Autoridade nao desaparece; ela precisa tornar-se situada, limitada,
   revogavel e auditavel.
3. O par e reconhecido como unidade instituinte minima, enquanto a triade surge
   como forma minima forte de coerencia institucional.
4. As implementacoes de referencia carregam principios de design que sao
   consequencia direta da posicao anti-feudal do protocolo.

## Voz

A escrita segue os principios do agente escritor (`.opencode/agents/escritor.md`):
fusao entre erudição e cultura popular, metafora concreta, humor como forma de
pensamento, a frase que pode ser lida em voz alta sem que ninguem tropece.
Referencia: Ariano Suassuna — o sertao nao e menor que a academia.

## Espinha atual do corpus

### Bloco A — Condicao e contra-tese

0. `00-o-que-e-o-comum.md` — nomeia o adversario, formula a contra-tese
1. `01-apresentacao.md` — abre o corpus, situa o diagnóstico
2. `02-o-problema-da-coordenacao.md`
3. `03-do-centro-ao-comum.md`

### Bloco B — Forma minima do Comum

1. `04-testemunho-e-legibilidade-publica.md`
2. `05-contexto-prova-e-legitimidade.md`
3. `06-autoridade-emergente-e-revogabilidade.md`
4. `07-par-triade-e-escala-institucional.md`

### Bloco C — Comunidade, valor e federacao

1. `08-comunidade-fronteira-e-federacao.md`
2. `09-valor-plural-e-interoperabilidade.md`
3. `10-compensacao-sem-equivalencia-unica.md`

### Bloco D — Limites, horizonte e oficio

1. `11-infraestrutura-e-colonizacao-abstrata.md`
2. `12-instituicoes-programaveis-e-limites.md`
3. `13-design-como-etica-da-forma.md` — a etica do projeto sobre suas proprias ferramentas
4. `14-conclusoes-e-perguntas-abertas.md`

### Funcao de cada bloco

### Bloco A

- nomear a condicao feudal da mediacao digital
- abrir a contra-tese: tecnofederalismo
- mostrar por que as respostas anteriores falharam
- situar o Comum no campo da coordenacao como problema politico e epistemico

### Bloco B

- fixar a ontologia minima do Comum
- explicar testemunho, prova, legitimidade e autoridade auditavel
- introduzir o problema da escala institucional sem inflar o protocolo base

### Bloco C

- tratar comunidade, fronteira e federacao
- elaborar pluralidade de valor e interoperabilidade sem equivalencia unica
- preparar a infraestrutura de compensacao

### Bloco D

- explicitar os limites da programabilidade
- enfrentar o risco de colonizacao abstrata pelo proprio Comum
- preservar as perguntas que o projeto nao deve fechar cedo demais
- enunciar os principios de design como consequencia do campo teorico

## Relacao com o restante do repo

- `spec/`: contrato normativo
- fundamentos conceituais curtos agora devem ser absorvidos por `docs/corpus/`
- `docs/notes/`: excedente, aberto, curiosidade e dossies ainda nao integrados
- `.opencode/agents/escritor.md`: guia a voz de toda escrita do projeto

O corpus recebe ideias promovidas do laboratorio. As notas nao devem continuar
funcionando como lugar principal da teoria longa.

## Regra de promocao para o corpus

Uma nota sobe para capitulo quando:

- ganhou tese reconhecivel
- deixou de ser apenas brainstorming
- ja influencia o vocabulario do projeto
- exige desenvolvimento argumentativo maior do que uma nota curta permite

## Status

Estrutura editorial revisada com a contra-tese ao tecnofeudalismo como eixo
central. Capitulos 00 e 01 reescritos para nomear o adversario e posicionar o
Comum como tecnofederalismo. Principios de design incorporados ao corpus.
