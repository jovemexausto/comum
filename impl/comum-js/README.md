# comum-js

SDK TypeScript (ESM) para mobile-first.

Dois modos:

- CLI: usa o binario `comum-cbor` para encode e hash.
- N-API (opcional): bindings nativos via `impl/comum-rs/napi`.

Para usar N-API localmente:

```sh
cargo build -p comum-napi
export COMUM_NAPI_PATH=/caminho/para/comum-napi.node
```
