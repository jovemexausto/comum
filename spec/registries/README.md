# Registros

Este diretorio contem registros normativos do protocolo.

Durante a transicao para parity/codegen, os registros possuem duas formas:

- `*.md`: tabela legivel para humanos e revisao normativa
- `*.yaml`: espelho machine-readable para tooling, codegen e testes de paridade

Enquanto a migracao nao estiver completa, ambas as formas MUST permanecer em
sincronia. A intencao e que tooling futuro derive constantes, docs e checks de
paridade a partir dos arquivos machine-readable.

Regras de alocacao:

- Todo novo ID, verb ou tipo requer um CIP aprovado.
- Mudancas em registros sao aditivas, nunca destrutivas.
- Itens obsoletos devem ser marcados como Obsoleted, nao removidos.

Arquivos:

- verbs.md: verbos do protocolo
- suites.md: suites criptograficas
- cte-types.md: tipos do Envelope Comum de Transporte
- context-types.md: tipos de Prova de Contexto
- transport-profiles.md: perfis de transporte

Espelhos machine-readable iniciais:

- verbs.yaml
- suites.yaml
- cte-types.yaml
- context-types.yaml
- transport-profiles.yaml
