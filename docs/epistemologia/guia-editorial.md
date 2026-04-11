# Guia Editorial e Agentico

## Regra principal

Cada documento do repo deve ter um unico papel dominante.

## Areas vivas

- `README.md`: entrada publica
- `ABSTRACT.md`: teoria condensada + estado real
- `docs/corpus/`: fundamento teorico, politico e filosofico
- `docs/implementation/`: documentacao tecnica informativa
- `docs/project/`: contexto operacional vivo
- `docs/notes/`: laboratorio curto
- `spec/`: contrato normativo

## Regras para agentes

### 1. Nao duplicar papeis

- nao criar segundo texto para "o que e o Comum" fora de `README`, `ABSTRACT` ou `docs/corpus/`
- nao criar teoria longa em `docs/notes/`
- nao deixar contexto operacional competir com corpus ou spec

### 2. Preferir consolidacao a proliferacao

- se um texto novo repete uma funcao existente, fundir em vez de criar outro
- se um texto perdeu funcao clara, mover para `.archive/`

### 3. Respeitar fronteiras

- teoria longa -> `docs/corpus/`
- norma -> `spec/`
- tecnica -> `docs/implementation/`
- operacao -> `docs/project/`
- laboratorio -> `docs/notes/`

### 4. Python sempre via `uv`

Qualquer script Python eventual do projeto deve ser executado via `uv`.

## Regra de promocao

Uma nota sobe de nivel quando:

- ganhou tese clara
- tem leitor ideal evidente
- ja influencia decisao real do projeto
- deixou de ser apenas material de exploracao

## Regra de arquivo

Material superseded, dissolvido, excessivo ou processual nao permanece no corpus
vivo. Deve ir para `.archive/`.

## Status

Guia curto de disciplina editorial e agentica do Comum.
