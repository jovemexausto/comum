default:
  @just --list

conformance:
  node runtime/conformance/run.js

conformance-rs:
  cargo test --manifest-path runtime/rust/comum-rs/Cargo.toml

comum-rs-cli:
  cat spec/test-vectors/vector-0001.json | cargo run --manifest-path runtime/rust/comum-rs/Cargo.toml --bin comum-cbor

comum-rs-cli-file FILE:
  cat {{FILE}} | cargo run --manifest-path runtime/rust/comum-rs/Cargo.toml --bin comum-cbor

interop:
  node runtime/conformance/run.js

test:
  node runtime/conformance/run.js
  cargo test --manifest-path runtime/rust/comum-rs/Cargo.toml
  (cd runtime/js/comum-js && npm install)
  (cd runtime/js/comum-js && npm run build)
  (cd runtime/js/comum-js && npm test)

comum-js-tests:
  cargo build --manifest-path runtime/rust/comum-rs/Cargo.toml
  (cd runtime/js/comum-js && npm install)
  (cd runtime/js/comum-js && npm run build)
  (cd runtime/js/comum-js && npm test)

capsula-agora-build:
  rustup target add wasm32-unknown-unknown
  (cd apps/agora/capsules && RUSTFLAGS="-C link-arg=--export=invoke -C link-arg=--export-memory" cargo build --release --target wasm32-unknown-unknown)
  cp target/wasm32-unknown-unknown/release/capsula_agora.wasm apps/agora/capsules/agora.wasm

capsula-feira-build:
  rustup target add wasm32-unknown-unknown
  (cd apps/feira/capsules && RUSTFLAGS="-C link-arg=--export=invoke -C link-arg=--export-memory" cargo build --release --target wasm32-unknown-unknown)
  cp target/wasm32-unknown-unknown/release/capsula_feira.wasm apps/feira/capsules/feira.wasm

capsula-mutirao-build:
  rustup target add wasm32-unknown-unknown
  (cd cases/mutirao-legacy/capsule && RUSTFLAGS="-C link-arg=--export=invoke -C link-arg=--export-memory" cargo build --release --target wasm32-unknown-unknown)
  cp target/wasm32-unknown-unknown/release/capsula_mutirao.wasm cases/mutirao-legacy/capsule/mutirao.wasm


sim:
  cargo run --manifest-path runtime/rust/comum-rs/Cargo.toml --bin sim

agora-sim:
  cargo run -p agora-sim

feira-sim:
  cargo run -p feira-sim

mutirao-sim:
  cargo run -p mutirao-sim

list:
  @just --list
