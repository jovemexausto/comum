# Capsula Governanca

Placeholder minimo em Rust, compila para WASM (wasm32-unknown-unknown, no_std).
Sem semantica de negocio definida nesta etapa.

Build manual:

```
rustup target add wasm32-unknown-unknown
RUSTFLAGS="-C link-arg=--export=invoke -C link-arg=--export-memory" \
  cargo build --release --target wasm32-unknown-unknown
```

Artefato:

`target/wasm32-unknown-unknown/release/capsula_governanca.wasm`
