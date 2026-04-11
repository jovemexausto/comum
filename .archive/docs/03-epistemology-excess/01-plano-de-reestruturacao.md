# Plano de reestruturacao do corpus

Objetivo: transformar o acervo atual de README, teoria, notas, arquitetura e
spec em um corpus modular, legivel e deliberado, sem perder riqueza nem
achatamento conceitual.

## Objetivo final

Chegar a um conjunto de documentos Markdown organizado como um livro/whitepaper
modular, em que cada parte tenha:

- escopo claro
- tipo de normatividade claro
- publico principal claro
- relacoes explicitas com as outras partes

## Fase 1 — Cartografia do acervo

Meta:

- listar todos os documentos relevantes
- classificar por tipo, escopo e maturidade

Saida esperada:

- tabela simples contendo:
  - arquivo
  - categoria (normativo / arquitetura / teoria / economia politica / exploratorio)
  - publico principal
  - status (ativo / legado / precisa mover / precisa fundir)

## Fase 2 — Delimitar a arquitetura editorial

Meta:

- fixar as secoes-mestras do corpus

Estrutura inicial sugerida:

1. Manifesto e visao
2. Teoria do protocolo
3. Nucleo protocolar normativo
4. Arquitetura e implementacoes
5. Economia politica e federacao
6. Laboratorio exploratorio
7. Release notes e roadmap

Saida esperada:

- um sumario mestre com a arvore final desejada
- uma arvore editorial explicita (`arvore-editorial-mestra.md`)

## Fase 3 — Definir regras editoriais

Meta:

- impedir mistura continua entre nota, teoria, spec e opiniao

Regras a explicitar:

- o que pode entrar em CIP
- o que deve ficar em teoria
- o que deve virar nota exploratoria
- como marcar status (`normativo`, `informativo`, `exploratorio`, `legado`)
- como fazer referencias cruzadas

Saida esperada:

- guia editorial curto para o corpus

## Fase 4 — Reorganizar o material existente

Meta:

- mover, renomear, fundir e indexar documentos existentes

Trabalho tipico:

- promover notas maduras para teoria ou economia politica
- manter brainstorming em laboratorio exploratorio
- separar claramente docs de arquitetura dos docs conceituais
- arquivar material legado sem apagar contexto historico

Saida esperada:

- nova arvore de docs coerente

## Fase 5 — Escrever capitulos-ponte

Meta:

- preencher vazios entre os materiais existentes

Provaveis lacunas:

- mapa epistemologico
- economia politica do Comum
- limites anti-coloniais do protocolo
- compensacao intercomunitaria e pluralidade de valor
- o que o Comum ja e, para alem do protocolo tecnico

Saida esperada:

- 3 a 6 documentos novos costurando o corpus

## Fase 6 — Criar indexacao viva

Meta:

- evitar voltar ao estado de dispersao

Acao:

- indices por tema
- indices por tipo de documento
- referencias cruzadas explicitas entre texto tecnico e conceitual

Saida esperada:

- cada documento aponta para seu contexto e seus limites

## Fase 7 — Revisao de linguagem e tom

Meta:

- alinhar a voz do corpus

Separacoes desejadas:

- README: manifesto e productish
- teoria: conceitual e precisa
- arquitetura: mecanica e verificavel
- notas: abertas e honestas
- CIP: secas, normativas, sem especulacao

Saida esperada:

- corpus com tom consistente por camada

## Ordem recomendada de execucao

1. Cartografia do acervo
2. Arquitetura editorial
3. Guia editorial
4. Reorganizacao fisica dos arquivos
5. Escrita dos capitulos-ponte
6. Indexacao viva
7. Revisao final de linguagem

## Regra de disciplina

Nao comprimir tudo indefinidamente em notas cada vez maiores.

Em vez disso:

- sintetizar quando houver clareza
- separar quando houver excesso de acoplamento
- promover quando uma nota amadurecer
- manter exploratorio o que ainda e exploratorio

## Proximo passo imediato

Executar a Fase 1: cartografia do acervo atual, usando este mapa epistemologico
como criterio inicial de classificacao.

## Status

Plano inicial de reestruturacao editorial do corpus.
