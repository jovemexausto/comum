# Abstract

Comum Protocol is a local-first, offline-first infrastructure for sovereign
community coordination. The repository combines a normative protocol core, a
reference Rust implementation, a hybrid TypeScript SDK, reference capsules, a
mobile app-as-node experiment, and an expanding theoretical corpus about how to
coordinate without mandatory global consensus or platform lock-in, while
keeping local authority auditable, limited, and revocable.

At the protocol layer, Comum is intentionally narrow. Its atomic primitive is
the signed, immutable, verifiable `Testimony`. Higher-level social state such as
balances, reputation, receipts, votes, or community boundaries is not stored as
global truth; it is derived locally from a causal graph of testimonies. The
core stack is: canonical CBOR serialization, SHA3-256 content addressing,
cryptographic suites, Common Transport Envelope (CTE), eventual sync, modular
proof of context without mandatory GPS, and deterministic WASM capsules for
local semantics. The protocol explicitly rejects global ordering, strong global
consensus, mandatory platform identity, and non-revisable central validation.

At the normative layer, the repository is organized around public,
interoperability-focused artifacts. `spec/cips/CIP-0001.md` is the base
specification and defines the data model, canonicalization, crypto suites,
transport envelope, sync, context proofs, capsules, genesis, snapshots, and
governance. `CIP-0002` defines the `Commoner` facade as the high-level node
interface. `CIP-0003` standardizes the reference capsule semantics for Agora,
Feira, and Mutirao. Registries, CDDL schemas, official test vectors, and the
repository-level conformance runner act as the concrete contract for cross-
implementation interoperability. Governance is CIP-driven, public, and rough-
consensus based, with no central committee.

At the implementation layer, the project is strongest today in Rust. The
reference core in `impl/comum-rs/` implements canonical testimony
encoding/validation, SHA3-derived identifiers, Ed25519 signing and verification,
nullifiers, DID derivation, FROST threshold signatures, sync payload builders,
CTE fragmentation and reassembly, snapshots, pruning, and a `Commoner` runtime
that emits, validates, ingests, and synchronizes testimonies. The WASM runtime
boundary exists, including fuel-limited execution, but its host syscalls remain
stubbed and the committed reference capsule semantics still live primarily in
library logic rather than in a fully exercised contract VM path.

The TypeScript SDK in `impl/comum-js/` is hybrid by design. In Node contexts it
reuses the canonical Rust implementation via N-API or the `comum-cbor` CLI.
In React Native it ships a pure-JS runtime to support the app-as-node model and
mobile E2E flows without native bindings. That mobile runtime is explicitly
non-canonical: it favors execution portability and developer experience over
full parity with Rust. This is a documented limitation, not a hidden mismatch.
The current roadmap therefore focuses on runtime parity, cross-runtime
conformance, transport robustness, and API consolidation instead of adding new
surface area.

At the application and semantics layer, the repository includes three reference
capsules: Agora for governance, Feira for local exchange, and Mutirao for
collective work. Their payload builders, stable object identifiers, and tests
are implemented in Rust, with committed WASM artifacts in `impl/capsulas/`.
The mobile reference app in `apps/mobile/` demonstrates the "app = node"
direction: each app instance runs a local node, exposes a DID/status, supports
pluggable transport, and participates in a real multi-node Feira end-to-end flow
over WebSocket. This proves topology and convergence for a minimal distributed
system, but it does not yet imply protocol parity between the mobile runtime and
the Rust reference.

The repository is also more than a protocol implementation. Its documentation
tracks a sustained epistemic and political argument: shared semantics should be
strong enough to coordinate, but not so strong that they absorb local worlds.
This tension appears throughout the corpus as subsidiariedade tecnica,
interoperability without single equivalence, local legitimacy over central
authority, and anti-colonial limits on the protocol itself. The project treats
determinism as a property of the protocol, but legitimacy as a property of local
institutions and capsule-level interpretation. The protocol does not attempt to
eliminate authority; it attempts to make authority emergence, scope,
contestation, and revocation legible through auditable testimony chains.

In its current state, the repository should be read as a serious protocol and
runtime effort with real normative discipline, real reference implementations,
and real distributed-system experiments, but also with visible edges that are
still under construction. v0.3 is delivered as a minimal functional distributed
system. v0.4 is active and focused on parity, conformance, transport fault
handling, and higher-level SDK ergonomics. The repo is therefore best understood
not as a finished platform, but as a rigorously specified and increasingly
implemented foundation for sovereign, plural, auditable community coordination.
