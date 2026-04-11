# Plano de sincronizacao: autoridade emergente e Genesis 2+

Objetivo: registrar o plano tecnico-editorial para manter consistencia entre
discurso, norma e implementacao apos a mudanca de framing.

## Escopo desta rodada

1. Framing de autoridade emergente auditavel em docs centrais.
2. Regra de Genesis atualizada para founders >= 2.
3. Default normativo para N=2: threshold 2-of-2.
4. Paridade Rust/JS na validacao de Genesis.
5. Testes atualizados com casos de par e invalidacoes correspondentes.

## Arquivos alvo (sincronizacao direta)

- `ABSTRACT.md`
- `README.md`
- `docs/corpus/`
- `spec/cips/CIP-0001.md`
- `impl/comum-rs/src/claim.rs`
- `impl/comum-rs/tests/conformance.rs`
- `impl/comum-js/src/index.ts`
- `impl/comum-js/src/tests/genesis.test.ts`

## Criterios de consistencia

- Nenhum texto central deve confundir "descentralizacao" com "ausencia de
  autoridade".
- CIP-0001 deve distinguir:
  - existencia comunitaria minima (2)
  - robustez institucional (3+ recomendada conforme contexto)
- Regras de validacao devem ser identicas entre runtimes para Genesis.

## Casos minimos de teste para Genesis

Validos:

- N=2, threshold=2
- N=3, threshold=2

Invalidos:

- N=1, qualquer threshold
- N=2, threshold=1
- N>=2, threshold=0
- threshold>N

## Fora de escopo desta rodada

- Reescrever completamente a semantica da capsula Agora.
- Introduzir novos verbos de disputa/accountability no core.
- Reestruturar registries para formato machine-readable nesta mesma entrega.

## Proxima rodada sugerida

1. Casos concretos de disputa em Feira/Agora.
2. Mecanismos capsulares de contestacao e resolucao.
3. Metricas de captura e centralizacao institucional em simulacao.

## Status da nota

Nota de execucao. Nao normativa.
