# ABI de Capsulas (WASM)

Este documento define a ABI minima entre o runtime e as Capsulas.

## Memoria

- Capsulas exportam `memory` (WASM linear memory).
- Buffers sao passados como (ptr, len) em i32.

## Syscalls

### read_graph

Assinatura:

```
read_graph(query_ptr: i32, query_len: i32) -> i32
```

Retorna ponteiro para resultado serializado (CBOR).

### verify_proof

Assinatura:

```
verify_proof(data_ptr: i32, data_len: i32, proof_ptr: i32, proof_len: i32) -> i32
```

Retorna ERR_OK em sucesso.

### emit_testimony

Assinatura:

```
emit_testimony(claim_ptr: i32, claim_len: i32) -> i32
```

Retorna ponteiro para id do Testemunho.

## Codigos de erro

- ERR_OK = 0
- ERR_INVALID_PTR = -1
- ERR_INVALID_LEN = -2
- ERR_NOT_SUPPORTED = -3
