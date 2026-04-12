# Runtime

Superficie executavel e de tooling do monorepo.

- `rust/`: runtime e bindings nativos em Rust
- `js/`: SDKs e bindings JS/TS
- `tooling/`: utilitarios de build, empacotamento e suporte
- `conformance/`: runner e fixtures de conformidade do protocolo

Hoje o primeiro bloco real de tooling capsular vive em:

- `tooling/comum-capsule/`

O runtime implementa o contrato da `spec/`, mas nao o substitui.
