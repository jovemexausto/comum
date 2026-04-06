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

list:
  @just --list
