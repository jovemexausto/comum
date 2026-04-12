---
note_class: "00"
integration_state: "I2"
status: "guia editorial ativo"
destino: "stay-note"
rationale: "Define a disciplina do laboratorio e ja orienta o indice inteiro de notas."
---

# Metodo epistemologico das notas

Objetivo: disciplinar a escrita de notas para que o laboratorio continue vivo
sem substituir o corpus, a spec ou a documentacao tecnica.

## Tese do metodo

Uma nota boa nao tenta resolver tudo. Ela registra uma tensao com forma minima,
destino claro e status epistemico explicito.

## Classes de nota

### 01 — Excedente

Uso:

- quando uma ideia boa apareceu, mas ainda nao encontrou o lugar certo no corpus
  ou na arquitetura

Pergunta-tipica:

- o que surgiu e importa, mas ainda nao foi integrado?

### 02 — Aberto

Uso:

- quando existe um problema forte ainda sem formulacao final
- quando falta simulacao, caso real ou clareza conceitual

Pergunta-tipica:

- o que sabemos que importa, mas ainda nao sabemos formular ou resolver bem?

### 03 — Curiosidade

Uso:

- quando existe intuicao lateral, analogia, referencia ou hipoteses menores

Pergunta-tipica:

- o que vale preservar sem dar centralidade prematura?

### 90 — Dossie

Uso:

- para sessoes longas, dumps extensos, consolidacoes temporarias e material de
  transicao

Regra:

- um dossie nao deve ser confundido com nota curta madura
- ele existe para nao perder contexto, nao para virar forma editorial final

### 99 — Arquivo

Uso:

- para material superado, congelado ou absorvido por outro corpus

## Estrutura recomendada para nota curta

Frontmatter minimo:

```yaml
---
note_class: "01|02|03|90|99"
integration_state: "I0|I1|I2|I3|I4"
status: "frase curta"
destino: "stay-note|corpus|spec|archive|docs-tech|capsule-rfc-sim"
rationale: "por que esta nota existe agora"
---
```

1. Pergunta
2. Tese provisoria
3. Por que importa
4. Tensao ou limite
5. Destino provavel
6. Status

## Regra de tamanho

- nota curta: 300 a 900 palavras
- acima disso, perguntar se o texto virou:
  - capitulo de corpus
  - dossie
  - documento tecnico

## Regra de promocao

Uma nota sobe de nivel quando:

- deixou de ser brainstorm
- ganhou tese clara
- ja influencia decisao real do projeto
- precisa de desenvolvimento mais longo

Destinos possiveis:

- `docs/corpus/`
- `spec/`
- docs tecnicos
- capsula / RFC / simulacao

## Regra de honestidade

- nota nao promete estabilidade que ainda nao existe
- nota nao finge contrato tecnico
- nota nao substitui caso real ou teste quando eles sao necessarios

## Regra de linguagem

- curta
- precisa
- sem jargao ornamental desnecessario
- com tipo de verdade explicito: intuicao, hipotese, sintese, dossie ou arquivo

## Formula disciplinar do Comum para notas

1. observar
2. nomear a tensao
3. formular uma tese provisoria
4. indicar destino
5. definir criterio de promocao ou descarte

## Status

Metodo editorial ativo para disciplinar o laboratorio do Comum.
