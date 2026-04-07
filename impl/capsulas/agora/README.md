# Capsula Agora

Capsula de governanca com semantica de votacao e testes.
Compila para WASM (wasm32-unknown-unknown, no_std).

Build manual:

```
rustup target add wasm32-unknown-unknown
RUSTFLAGS="-C link-arg=--export=invoke -C link-arg=--export-memory" \
  cargo build --release --target wasm32-unknown-unknown
```

Ou via Justfile:

```
just capsula-agora-build
```

Artefato:

`target/wasm32-unknown-unknown/release/capsula_agora.wasm`
