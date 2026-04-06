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

all:
  node tests/conformance/run.js
  cargo test --manifest-path impl/comum-rs/Cargo.toml

comum-js-tests:
  cargo build --manifest-path impl/comum-rs/Cargo.toml
  (cd impl/comum-js && npm install)
  (cd impl/comum-js && npm run build)
  (cd impl/comum-js && npm test)

list:
  @just --list
