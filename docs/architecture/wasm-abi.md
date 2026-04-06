# WASM ABI — Capsulas do Comum Protocol

Referencia: CIP-0001, Secao 12.3 e 12.4.

## Convencao de Memoria

- Buffers sao passados como (ptr: i32, len: i32).
- ptr aponta para a memoria linear exportada como "memory".
- Retorno negativo indica erro.
- Retorno positivo ou zero indica sucesso (ponteiro ou ERR_OK).

## Codigos de Erro

  ERR_OK            =  0
  ERR_INVALID_PTR   = -1
  ERR_INVALID_LEN   = -2
  ERR_NOT_SUPPORTED = -3

## Syscalls

### read_graph(query_ptr, query_len) -> i32

query e um mapa CBOR com chaves tstr (todos opcionais):

  {
    "verb":      <tstr>,     -- filtrar por claim.verb
    "author":    <bytes32>,  -- filtrar por author
    "community": <bytes32>,  -- filtrar por community_id
    "since":     <uint64>,   -- timestamp minimo (Unix ms)
    "limit":     <uint>      -- maximo de resultados (default: 100)
  }

Retorno: ponteiro para array CBOR de Testemunhos serializados
         escrito na memoria linear do modulo pelo runtime.
         Retorna ERR_INVALID_PTR se query invalida.

### verify_proof(data_ptr, data_len, proof_ptr, proof_len) -> i32

data e os bytes sobre os quais a prova foi gerada (normalmente testimony_id).
proof e o mapa CBOR de Proof (CIP-0001 Sec. 4.11).

Retorno: ERR_OK se valido, ERR_INVALID_PTR ou ERR_INVALID_LEN se invalido.

### emit_testimony(claim_ptr, claim_len) -> i32

claim e um mapa CBOR de Claim (CIP-0001 Sec. 4.9):

  {
    0: <tstr>,  -- verb
    1: <bstr>   -- payload CBOR canonical
  }

O runtime constroi o Testemunho completo (version, author, timestamp,
suite, id, proof) e o adiciona ao grafo local.

Retorno: ponteiro para o id (bytes32) do Testemunho emitido.
         Retorna ERR_INVALID_PTR se claim invalido.

## Entrypoint

O modulo WASM MUST exportar:

  invoke() -> i32

Retorna ERR_OK (0) em sucesso.

## Limites Normativos

  WASM_MAX_MEMORY_PAGES   = 16       (1 MB)
  WASM_FUEL_DEFAULT       = 10_000   (instrucoes WASM)
  WASM_EXECUTION_TIMEOUT  = 200 ms
  MAX_COMPOSITION_DEPTH   = 8
