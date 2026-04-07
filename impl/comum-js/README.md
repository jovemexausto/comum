# comum-js

SDK TypeScript (ESM) para mobile-first.

Dois modos:

- CLI: usa o binario `comum-cbor` para encode e hash.
- N-API (opcional): bindings nativos via `impl/comum-rs/napi`.

No React Native, o pacote usa a entrada `react-native` (`dist/mobile.js`) com runtime JS para app-node e E2E multi-node sem N-API.

## v0.3 context

- Designed for app-as-node model
- Supports networked E2E via pluggable transport
- JS runtime is for DX and mobile execution (non-canonical)

Para usar N-API localmente:

```sh
cargo build -p comum-napi
export COMUM_NAPI_PATH=/caminho/para/comum-napi.node
```
