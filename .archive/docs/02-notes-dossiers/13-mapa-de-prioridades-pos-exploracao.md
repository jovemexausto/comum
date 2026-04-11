# Mapa de prioridades pos exploracao (nota)

Objetivo: registrar o retorno ao foco apos uma fase intensa de exploracao
conceitual, preservando as ideias sem dispersar a execucao.

## Prioridade 1 — Blindar o sistema

Motivo: sem consistencia estrutural, qualquer nova feature amplia risco.

Inclui:

- paridade de runtime (JS mobile vs Rust)
- suite de conformance cross-runtime
- padronizacao de naming no boundary TS

Pergunta-guia:

> O mesmo cenario produz o mesmo comportamento nos dois runtimes?

## Prioridade 2 — Consolidar a abstracao certa

Motivo: a unidade real do sistema ja apareceu: o node/cliente, nao apenas o
wrapper de baixo nivel.

Inclui:

- extrair `AppNode` para o SDK como `ComumClient`
- formalizar interface unica de transporte
- separar melhor protocolo / runtime / app

Pergunta-guia:

> O que e core, o que e interpretacao, e o que e politica local?

## Prioridade 3 — Testar o mundo real

Motivo: o sistema ja roda; agora precisa resistir a redes imperfeitas.

Inclui:

- fault injection no relay
- cenarios com delay / drop / reorder
- metricas de convergencia e divergencia

Pergunta-guia:

> Isso continua coordenando quando a rede deixa de ser limpa?

## Em segundo plano (por enquanto)

Estas linhas permanecem valiosas, mas nao devem capturar a execucao imediata:

- inferencia distribuida eventual
- inteligencia social programavel
- teoria dos jogos
- hawala
- camada de compensacao intercomunitaria
- risco de colonizacao abstrata pelo proprio Comum
- sociedades de codigo aberto

Devem permanecer como campo conceitual ativo, nao como eixo principal de
implementacao agora.

## Regra pratica

- Implementar agora o que reduz risco estrutural
- Documentar agora o que amplia rigor conceitual
- Nao transformar hipotese filosofica em feature antes da hora

## Sintese

Primeiro garantir que o sistema e coerente consigo mesmo.
Depois explorar tudo o que ele pode vir a ser.

## Status

Nota de foco e disciplina. Nao normativa.
