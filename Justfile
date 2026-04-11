default:
  @just --list

conformance:
  node tests/conformance/run.js

conformance-rs:
  cargo test --manifest-path crates/comum-rs/Cargo.toml

comum-rs-cli:
  cat spec/test-vectors/vector-0001.json | cargo run --manifest-path crates/comum-rs/Cargo.toml --bin comum-cbor

comum-rs-cli-file FILE:
  cat {{FILE}} | cargo run --manifest-path crates/comum-rs/Cargo.toml --bin comum-cbor

interop:
  node tests/conformance/run.js

test:
  node tests/conformance/run.js
  cargo test --manifest-path crates/comum-rs/Cargo.toml
  (cd packages/comum-js && npm install)
  (cd packages/comum-js && npm run build)
  (cd packages/comum-js && npm test)

comum-js-tests:
  cargo build --manifest-path crates/comum-rs/Cargo.toml
  (cd packages/comum-js && npm install)
  (cd packages/comum-js && npm run build)
  (cd packages/comum-js && npm test)

capsula-agora-build:
  rustup target add wasm32-unknown-unknown
  (cd capsules/agora && RUSTFLAGS="-C link-arg=--export=invoke -C link-arg=--export-memory" cargo build --release --target wasm32-unknown-unknown)
  cp target/wasm32-unknown-unknown/release/capsula_agora.wasm capsules/agora/agora.wasm

capsula-feira-build:
  rustup target add wasm32-unknown-unknown
  (cd capsules/feira && RUSTFLAGS="-C link-arg=--export=invoke -C link-arg=--export-memory" cargo build --release --target wasm32-unknown-unknown)
  cp target/wasm32-unknown-unknown/release/capsula_feira.wasm capsules/feira/feira.wasm

capsula-mutirao-build:
  rustup target add wasm32-unknown-unknown
  (cd capsules/mutirao && RUSTFLAGS="-C link-arg=--export=invoke -C link-arg=--export-memory" cargo build --release --target wasm32-unknown-unknown)
  cp target/wasm32-unknown-unknown/release/capsula_mutirao.wasm capsules/mutirao/mutirao.wasm


sim:
  cargo run --manifest-path crates/comum-rs/Cargo.toml --bin sim

agora-sim:
  cargo run -p agora-sim

feira-sim:
  cargo run -p feira-sim

mutirao-sim:
  cargo run -p mutirao-sim

list:
  @just --list
