# Framework Multi-Agente

## Proposito

O Comum usa um framework multi-agente proprio.

Os agentes do projeto nao existem apenas para executar tarefas. Eles existem para
aplicar formas distintas de pensar o Comum sem aumentar a entropia do repositorio.

Regra central:

> Todo agente ativo deve agir primeiro como orquestrador responsavel do problema
> recebido e, depois, como arquetipo especializado segundo a natureza da tarefa.

## Hierarquia de verdade do repo

- `spec/`: contrato normativo vivo
- `docs/corpus/`: fundamento teorico, politico e filosofico
- `docs/implementation/`: documentacao tecnica informativa
- `docs/project/`: contexto operacional vivo
- `docs/notes/`: laboratorio curto e ativo

Regras:

- implementacao nao vira norma so por existir
- teoria nao vira contrato tecnico so por ser persuasiva
- contexto operacional nao compete com corpus ou spec
- material superseded sai do corpus vivo

## Protocolo de orquestracao

Quando uma tarefa chega, o agente ativo deve responder internamente:

1. Isto e `spec`, `corpus`, `implementation`, `project`, `notes` ou `archive`?
2. Qual arquetipo principal deve liderar?
3. Quais outras lentes precisam ser consultadas?
4. O resultado final deve criar, mover, fundir, remover do corpus vivo ou nao fazer nada?

## Regras anti-entropia

- preferir consolidar a duplicar
- preferir retirar do corpus vivo a manter ambiguidade
- preferir reduzir antes de expandir
- preferir nomear corretamente antes de multiplicar documentos
- nenhum agente deve aumentar a superficie viva do repo sem papel claro

## Regras epistemologicas

- separar fato, hipotese, interpretacao e norma
- quando faltar evidencia, dizer que falta evidencia
- quando algo for exploratorio, nao vestir como consolidado
- quando algo for norma, apontar o artefato normativo vivo

## Reference Stack

Quando houver duvida entre sentido, contrato, execucao e claim publica, usar a
reference stack do projeto (`docs/project/reference-stack.md`).

Regra curta:

- corpus da sentido
- spec da obrigacao
- implementation da execucao
- evidence da permissao
- paper da claim publica

## Build e Plan

O ambiente pode oferecer agentes embutidos como `Build` e `Plan`.

Eles sao infraestrutura de trabalho. Os agentes locais do projeto sao lentes
cognitivas proprias do Comum.

## Criterio de sucesso

O framework multi-agente esta funcionando quando:

- o repo ganha clareza sem perder densidade
- hipoteses ficam mais fortes depois de serem atacadas
- cada arquivo tem papel evidente
- o projeto avanca sem espalhar entropia
