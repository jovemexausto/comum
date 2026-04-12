# Notas do Comum

Objetivo: fazer de `docs/notes/` um laboratorio disciplinado, e nao um deposito
indistinto de intuicoes longas.

As notas existem para registrar excedentes, aberturas e curiosidades ativas sem
capturar o papel do corpus, da spec ou da documentacao tecnica.

## Regra simples

- corpus: teoria longa e integrada
- spec: contrato normativo
- docs tecnicos: arquitetura e operacao
- notas: excedente, aberto, curiosidade, dossie ou arquivo

## Arquivos de entrada

- `00-I2-metodo-epistemologico.md`
- `INDEX.md`

## Prefixos recomendados para novas notas

- `01-Ix-...` excedente
- `02-Ix-...` aberto
- `03-Ix-...` curiosidade
- `90-Ix-...` dossie
- `99-Ix-...` arquivo

Onde `Ix` representa o estado de integracao:

- `I0`: bruto
- `I1`: nota viva
- `I2`: pronta para promocao
- `I3`: parcialmente absorvida
- `I4`: absorvida, congelada ou arquivada

Frontmatter minimo esperado:

- `note_class`
- `integration_state`
- `status`
- `destino`
- `rationale`

Tudo o que deixar de ser nota curta viva deve sair do corpus de `docs/notes/`.
