# Capsule SSOT

Objetivo: fixar onde vive a fonte unica de verdade semantica de uma capsula no
monorepo.

## Regra

Cada capsula mantida no monorepo deve ter um SSOT local em seu proprio diretorio.

Forma atual recomendada:

- `apps/<app>/capsules/capsule.yaml`

Esse arquivo descreve a semantica local da capsula como package, nao o protocolo
inteiro nem o mecanismo contingente do runtime.

## O que o SSOT local deve declarar

- nome da capsula
- versao
- descricao
- artefato principal (`crate`, `wasm`, `runtime_abi`, `entrypoint`)
- verbos locais da capsula
- payload de cada verbo
- verbos de resultado, quando houver

## O que o SSOT local nao deve declarar

- backlog de UI
- estado atual da interface
- caminhos de telas ou componentes
- detalhes contingentes de transporte usados pelo runtime atual
- semantica do protocolo fora do escopo da capsula

## Distincao importante

- `capsule.yaml` responde: "quem e esta capsula e qual e sua semantica propria?"
- o runtime responde: "como essa semantica e transportada/executada hoje?"

Essas duas perguntas nao devem ser misturadas.

## Transporte atual

Hoje o runtime do Comum ainda materializa execucao capsular por envelopes como
`capsule/invoke` e `capsule/result`.

Isso nao faz desses envelopes o SSOT semantico da capsula. Eles descrevem o modo
como o runtime atual carrega a invocacao, nao os verbos locais que a capsula
apresenta ao mundo do app e da comunidade.

## Exemplo de leitura correta

- `feira/offer` e verbo local da capsula Feira
- `agora/vote` e verbo local da capsula Agora
- `capsule/invoke` e detalhe do runtime atual para carregar a chamada

## Consequencia pratica

Se o runtime mudar no futuro, o `capsule.yaml` idealmente permanece estavel.
Se o `capsule.yaml` mudar, e porque a semantica local da capsula mudou.

## Composicao por app

Cada app slice pode declarar suas dependencias capsulares em:

- `apps/<app>/capsules.yaml`

Esse arquivo nao redefine a capsula. Apenas declara quais packages capsulares o
slice quer usar, com versao desejada e, opcionalmente, `path` local.

O resultado material dessa resolucao pode ser persistido em:

- `apps/<app>/capsules.lock`
