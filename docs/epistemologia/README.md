# Disciplina Agentica do Comum

Objetivo: dar forma curta e explicita ao modo como agentes devem operar dentro
do projeto, sem competir com o corpus teorico, a spec ou a documentacao tecnica.

Esta pasta nao e mais um corpus filosofico paralelo. Ela serve como camada de
disciplina editorial e agentica do repositorio.

O texto-base do framework multi-agente agora vive em `docs/epistemologia/framework-agente`.

## Papel

- dizer como agentes devem ler o repo
- preservar fronteiras entre corpus, spec, implementation, project e notes
- reduzir entropia e duplicacao
- orientar promocao, arquivo e consolidacao de documentos

## Fonte operacional dos agentes

Agentes locais do projeto vivem em:

- `.opencode/agents/`

Arquivos principais:

- `orquestrador.md`
- `parceiro-epistemico.md`
- `curador.md`
- `cartografo.md`

## Regra de ouro

Agentes nao devem aumentar a superficie viva do repo sem clareza de papel.

Quando houver ambiguidade:

- preferir consolidar
- preferir retirar do corpus vivo a duplicar
- preferir corpus para teoria longa
- preferir spec para contrato normativo
- preferir notes para exploracao curta

## Personalidades nucleares

- `orquestrador`: decompõe tarefas grandes e escolhe a casa certa no repo
- `parceiro-epistemico`: estressa hipoteses, testes e criterios de falsificacao
- `curador`: cuida da forma e do lugar do corpus vivo
- `cartografo`: explora a base e separa fato de inferencia

## Ordem de operacao recomendada

1. o agente ativo orquestra a tarefa
2. o cartografo mapeia a base quando a superficie ainda nao esta clara
3. o parceiro epistemico tensiona a tese quando a tarefa envolve argumento,
   estrategia ou hipotese
4. o curador decide destino, consolidacao, promocao ou arquivo

## Status

Camada minima de disciplina agentica do Comum.
