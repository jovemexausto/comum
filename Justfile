default:
  @just --list

capsule-list:
  cargo run -p comum-capsule -- list

capsule-check DIR:
  cargo run -p comum-capsule -- check {{DIR}}

capsule-build DIR:
  cargo run -p comum-capsule -- build {{DIR}}

capsule-verify DIR:
  cargo run -p comum-capsule -- verify {{DIR}}

capsule-inspect DIR:
  cargo run -p comum-capsule -- inspect {{DIR}}

capsule-resolve APPDIR:
  cargo run -p comum-capsule -- resolve {{APPDIR}}

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
  cargo run -p comum-capsule -- build apps/agora/capsules

capsula-feira-build:
  cargo run -p comum-capsule -- build apps/feira/capsules

capsula-mutirao-build:
  cargo run -p comum-capsule -- build cases/mutirao-legacy/capsule


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
