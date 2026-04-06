# Matriz de Interop

## Core do protocolo

| Recurso                                         | Especificacao     | comum-rs | comum-js |
| ----------------------------------------------- | ----------------- | -------- | -------- |
| CBOR canonical (Testimony)                      | CIP-0001          | OK       | OK       |
| Vetores oficiais (0001-0006)                    | spec/test-vectors | OK       | OK       |
| CTE encode/decode + fragmentacao                | CIP-0001          | OK       | N/A      |
| Sync payloads (HELLO/REQUEST/RESPONSE)          | CIP-0001          | OK       | N/A      |
| Context payloads (proximity/beacon/place/vouch) | CIP-0001          | OK       | OK       |
| Genesis payload                                 | CIP-0001          | OK       | OK       |
| FROST threshold signatures                      | CIP-0001          | OK       | N/A      |

## Runtime e capsulas

| Recurso                    | Especificacao | Runtime WASM (comum-rs) | Capsulas |
| -------------------------- | ------------- | ----------------------- | -------- |
| Runtime WASM + limites     | CIP-0001      | OK                      | N/A      |
| Capsula Agora (governanca) | CIP-0001      | OK                      | OK       |
