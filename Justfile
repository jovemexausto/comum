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
  cargo run --manifest-path impl/comum-rs/Cargo.toml --bin wat2wasm -- impl/capsulas/agora/agora.wat impl/capsulas/agora/agora.wasm

list:
  @just --list
