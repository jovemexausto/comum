# RFC-0003 — Capsula Agora, Feira e Mutirao

## Status

Superseded by CIP-0003

## Resumo

Este documento define tres capsulas referencia:

- Agora: governanca local (propor, votar, aprovar)
- Feira: mercado local (oferta, aceite, entrega, recibo, disputa)
- Mutirao: trabalho coletivo (tarefa, compromisso, check-in, conclusao, recompensa)

Nao altera o protocolo. Define apenas a semantica e os payloads de
`capsule/invoke` e `capsule/result` para essas capsulas.

## Motivacao

Agora, Feira e Mutirao cobrem governanca, trocas e trabalho coletivo
offline-first. Definir formatos claros permite interoperabilidade entre
apps e runtime sem impor UX.

## Escopo

- Semantica de payloads por acao
- Regras minimas de validacao
- Estrutura canonica de invocacao e resultado

## Envelope de Invocacao

`capsule/invoke` payload (CBOR canonical, chaves tstr):

```
{
  "capsule_id": <bytes32>,  // SHA3-256 do WASM
  "action":     <tstr>,     // nome da acao
  "params":     <bstr>      // CBOR do payload de acao
}
```

`capsule/result` payload (CBOR canonical, chaves tstr):

```
{
  "capsule_id": <bytes32>,
  "action":     <tstr>,
  "status":     <uint>,     // 0 = ok, >0 = erro
  "result":     <bstr>      // CBOR do resultado (pode ser vazio)
}
```

## Capsula Agora

### Acoes

1) `propose`

```
{
  "proposal_id": <bytes32>,
  "title":       <tstr>,
  "body":        <tstr>,
  "quorum":      <uint64>,
  "expires":     <uint64>,
  "author":      <DID>
}
```

2) `vote`

```
{
  "proposal_id": <bytes32>,
  "choice":      <uint>,    // 0 = nao, 1 = sim, 2 = abstencao
  "voter":       <DID>
}
```

3) `close`

```
{
  "proposal_id": <bytes32>,
  "timestamp":   <uint64>
}
```

### Regras

- `proposal_id` MUST ser SHA3-256 do CBOR canonical de `propose`.
- `vote` MUST referenciar um `proposal_id` valido.
- `close` SHOULD ser emitido apos `expires`.

## Capsula Feira

### Acoes

1) `offer`

```
{
  "offer_id":   <bytes32>,
  "item":       <tstr>,
  "price":      <uint64>,
  "currency":   <tstr>,
  "expires":    <uint64>,
  "seller":     <DID>
}
```

2) `accept`

```
{
  "offer_id":   <bytes32>,
  "buyer":      <DID>
}
```

3) `deliver`

```
{
  "offer_id":   <bytes32>,
  "timestamp":  <uint64>
}
```

4) `receipt`

```
{
  "offer_id":   <bytes32>,
  "timestamp":  <uint64>
}
```

5) `cancel`

```
{
  "offer_id":   <bytes32>,
  "reason":     <tstr>
}
```

6) `dispute`

```
{
  "offer_id":   <bytes32>,
  "reason":     <tstr>
}
```

### Regras

- `offer_id` MUST ser SHA3-256 do CBOR canonical de `offer`.
- `accept` MUST referenciar um `offer_id` valido.
- `receipt` SHOULD ser emitido pelo comprador.

## Capsula Mutirao

### Acoes

1) `task`

```
{
  "task_id":    <bytes32>,
  "title":      <tstr>,
  "details":    <tstr>,
  "reward":     <uint64>,
  "expires":    <uint64>,
  "creator":    <DID>
}
```

2) `commit`

```
{
  "task_id":    <bytes32>,
  "worker":     <DID>
}
```

3) `checkin`

```
{
  "task_id":    <bytes32>,
  "timestamp":  <uint64>
}
```

4) `complete`

```
{
  "task_id":    <bytes32>,
  "timestamp":  <uint64>
}
```

5) `reward`

```
{
  "task_id":    <bytes32>,
  "amount":     <uint64>,
  "timestamp":  <uint64>
}
```

### Regras

- `task_id` MUST ser SHA3-256 do CBOR canonical de `task`.
- `commit` SHOULD ocorrer antes de `checkin`.
- `reward` SHOULD ser emitido apos `complete`.

## Compatibilidade

Este RFC apenas define payloads. Implementacoes podem adicionar campos
opcionais, desde que preservem CBOR canonical e nao quebrem parsing.

## Consideracoes de Seguranca

- Recomendado exigir prova de contexto para `offer`, `accept` e `reward`.
- Disputas podem exigir regras adicionais via Capsula Agora.
