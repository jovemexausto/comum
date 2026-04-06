default:
  @just --list

conformance:
  node tests/conformance/run.js

conformance-rs:
  cargo test --manifest-path impl/comum-rs/Cargo.toml

comum-rs-cli:
  cat spec/test-vectors/vector-0001.json | cargo run --manifest-path impl/comum-rs/Cargo.toml --bin comum-cbor

comum-rs-cli-file FILE:
  cat {{FILE}} | cargo run --manifest-path impl/comum-rs/Cargo.toml --bin comum-cbor

interop:
  node tests/conformance/run.js

test:
  node tests/conformance/run.js
  cargo test --manifest-path impl/comum-rs/Cargo.toml
  (cd impl/comum-js && npm install)
  (cd impl/comum-js && npm run build)
  (cd impl/comum-js && npm test)

comum-js-tests:
  cargo build --manifest-path impl/comum-rs/Cargo.toml
  (cd impl/comum-js && npm install)
  (cd impl/comum-js && npm run build)
  (cd impl/comum-js && npm test)

capsula-agora-build:
  rustup target add wasm32-unknown-unknown
  (cd impl/capsulas/agora && RUSTFLAGS="-C link-arg=--export=invoke -C link-arg=--export-memory" cargo build --release --target wasm32-unknown-unknown)
  cp target/wasm32-unknown-unknown/release/capsula_agora.wasm impl/capsulas/agora/agora.wasm

capsula-governanca-build:
  rustup target add wasm32-unknown-unknown
  (cd impl/capsulas/governanca && RUSTFLAGS="-C link-arg=--export=invoke -C link-arg=--export-memory" cargo build --release --target wasm32-unknown-unknown)
  cp target/wasm32-unknown-unknown/release/capsula_governanca.wasm impl/capsulas/governanca/governanca.wasm

sim:
  cargo run --manifest-path impl/comum-rs/Cargo.toml --bin sim

list:
  @just --list
