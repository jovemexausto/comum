# Capsula Feira

Capsula local de referencia para o fluxo da Feira.

SSOT local desta capsula:

- `capsule.yaml`: identidade da capsula como package, artefato, verbos locais e payloads

Esse arquivo descreve a semantica propria da capsula Feira. Ele nao deve virar
mini-spec do protocolo nem carregar estado contingente da interface.

Build manual:

```sh
rustup target add wasm32-unknown-unknown
RUSTFLAGS="-C link-arg=--export=invoke -C link-arg=--export-memory" \
  cargo build --release --target wasm32-unknown-unknown
```

Ou via Justfile:

```sh
just capsula-feira-build
```

Artefato esperado:

`target/wasm32-unknown-unknown/release/capsula_feira.wasm`
