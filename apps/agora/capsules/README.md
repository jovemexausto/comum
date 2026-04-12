# Capsula Agora

Capsula de governanca minima com semantica de votacao e fechamento local.
Esta referencia cobre proposta, voto e close. Nao implementa, nesta fase,
mandato, contestacao, escalacao ou resolucao de disputa.
Compila para WASM (wasm32-unknown-unknown, no_std).

SSOT local desta capsula:

- `capsule.yaml`: identidade da capsula como package, artefato, verbos locais e payloads

Esse arquivo descreve a semantica propria da capsula Agora. Ele nao deve virar
mini-spec do protocolo nem carregar estado contingente da interface.

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
