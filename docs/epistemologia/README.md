# Disciplina Agentica do Comum

Objetivo: dar forma curta e explicita ao modo como agentes devem operar dentro
do projeto, sem competir com o corpus teorico, a spec ou a documentacao tecnica.

Esta pasta nao e mais um corpus filosofico paralelo. Ela serve como camada de
disciplina editorial e agentica do repositorio.

## Papel

- dizer como agentes devem ler o repo
- preservar fronteiras entre corpus, spec, implementation, project e notes
- reduzir entropia e duplicacao
- orientar promocao, arquivo e consolidacao de documentos

## Fonte operacional dos agentes

Agentes locais do projeto vivem em:

- `.opencode/agents/`

Arquivos principais:

- `normas.md`
- `corpus.md`
- `research.md`
- `parceiro-epistemico.md`
- `interop.md`
- `capsulas.md`
- `paridade-runtime.md`
- `orquestrador.md`

## Regra de ouro

Agentes nao devem aumentar a superficie viva do repo sem clareza de papel.

Quando houver ambiguidade:

- preferir consolidar
- preferir arquivar a duplicar
- preferir corpus para teoria longa
- preferir spec para contrato normativo
- preferir notes para exploracao curta

## Personalidades nucleares

- `parceiro-epistemico`: estressa hipoteses, testes e criterios de falsificacao
- `normas`: protege alinhamento entre spec e implementacao
- `corpus`: cuida da forma e do lugar da teoria
- `research`: explora a base e separa fato de inferencia
- `interop`: olha vetores, contratos e risco de drift
- `capsulas`: protege a fronteira entre core e semantica local
- `paridade-runtime`: foca no que muda comportamento entre runtimes
- `orquestrador`: decompõe tarefas grandes e escolhe a casa certa no repo

## Status

Camada minima de disciplina agentica do Comum.
