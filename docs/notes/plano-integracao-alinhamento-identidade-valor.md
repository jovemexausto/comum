# Plano de integracao do alinhamento (teoria, spec, codigo)

Objetivo: transformar a sintese da sessao em trabalho executavel sem inflar o core.

Referencia de entrada: `docs/notes/sessao-alinhamento-identidade-valor-apps.md`.

## 1) Entregas por trilha

### Trilha A — Teoria (corpus)

- Fixar desambiguacao: protocolo, comunidade, capsula, app, identidade, valor.
- Definir separacao entre comunidade nominal e comunidade funcional.
- Definir separacao entre valor vivido, valor legivel e valor compensavel.
- Definir linguagem de limites: federacao de borda nao e soberania superior.

Arquivos alvo:

- `docs/corpus/00-o-que-e-o-comum.md`
- `docs/corpus/08-comunidade-fronteira-e-federacao.md`
- `docs/corpus/09-valor-plural-e-interoperabilidade.md`
- `docs/corpus/10-compensacao-sem-equivalencia-unica.md`
- `docs/corpus/14-conclusoes-e-perguntas-abertas.md`

### Trilha B — Especificacao (CIPs + registries)

- Declarar explicitamente: `Comum` e nome do protocolo, nao unidade monetaria global.
- Delimitar emissao local como competencia comunitaria/capsular.
- Resolver status de `comum/mint` (normativo, capsular ou removido).
- Delimitar fronteira app x capsula x protocolo nos textos normativos.
- Delimitar identidade soberana desacoplada de app (raiz + personas como direcao).

Arquivos alvo:

- `spec/cips/CIP-0001.md`
- `spec/cips/CIP-0002.md`
- `spec/cips/CIP-0003.md`
- `spec/cips/CIP-0004.md`
- `spec/registries/verbs.md`
- `spec/registries/verbs.yaml`

### Trilha C — Implementacao e DX

- Remover defaults que impliquem unidade universal (`currency = "comum"`).
- Marcar fluxos do app como politica local de UX, nao semantica global.
- Introduzir cenarios de teste com mais de uma unidade local de valor.
- Preparar boundary para identidade desacoplada do app (vault/agent externo).

Arquivos alvo iniciais:

- `apps/mobile/App.tsx`
- `apps/mobile/src/node.ts`
- `packages/comum-js/src/client.ts`
- `packages/comum-js/src/client.mobile.ts`
- `packages/comum-js/src/core.mobile.ts`
- `packages/comum-js/src/tests/*.ts`

## 2) Sequencia recomendada

1. Fechar glossario canonico (nota curta em `docs/notes/`).
2. Aplicar ajustes de linguagem em teoria (corpus).
3. Subir apenas o que for obrigacao interoperavel para a spec.
4. Ajustar defaults e exemplos no app/SDK.
5. Fechar testes de nao-totalizacao (multiplas unidades, nao-conversao, regras locais).

## 3) Criterios de pronto por camada

### Teoria

- Nao ha ambiguidade entre protocolo, app, capsula e comunidade.
- Nao ha ambiguidade entre nome do protocolo e nome de unidade de valor.

### Spec

- Invariantes de limite estao explicitos.
- Nenhuma secao implica unidade monetaria universal do protocolo.
- Status de emissao local e de `comum/mint` esta resolvido.

### Codigo

- App e SDK nao induzem ontologia de moeda global por default.
- Fluxos de exemplo funcionam com unidades locais distintas.
- Interface de identidade nao depende estruturalmente de um app especifico.

## 4) Riscos de regressao

- Reintroduzir `Comum` como nome de moeda por conveniencia de demo.
- Deixar politica de app parecer regra de protocolo.
- Tratar capsula como substituta da comunidade viva.
- Transformar vault em requisito obrigatorio de participacao.

## 5) Decisoes ja fechadas nesta sessao

- `Comum` fica reservado ao protocolo/projeto.
- Identidade deve ser desacoplada do app.
- Metadados pessoais devem ser opcionais e minimizados.
- Provas por predicado (ex.: `18+`) sao preferiveis a exposicao de dado bruto.
- Suporte a cofres frios e desejavel, mas nunca obrigatorio.
