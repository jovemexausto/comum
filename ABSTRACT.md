# Abstract

Comum Protocol is a local-first, offline-first infrastructure for public memory,
auditable local authority, and federated coordination between worlds that remain
different. It is not built around global truth, mandatory consensus, or a single
measure of value. Its wager is narrower and more disciplined: enough shared form
to coordinate, but not so much shared form that the infrastructure replaces the
communities it claims to serve.

At the protocol layer, Comum is intentionally thin. Its atomic primitive is the
signed, immutable, verifiable `Testimony`. Social state is not stored as global
truth. Balances, receipts, reputation, votes, revocations, and community
boundaries are read locally from a causal graph of testimonies. The technical
core is canonical CBOR, content-addressed identities, cryptographic suites,
Common Transport Envelope (CTE), eventual sync, modular proof of context, and
deterministic capsule execution boundaries.

At the political and epistemic layer, the project has moved away from an
anti-authority framing toward a stricter one: authority does not disappear; it
must become situated, limited, contestable, and revocable. The protocol treats
determinism as its own task, but legitimacy as a task of local institutions and
their procedures of reading, judgment, delegation, and dispute. What the system
can offer is not final truth or universal justice, but a stronger form of public
trace: a way for acts, evidence, support, revocation, and conflict to leave a
legible graph instead of disappearing into opaque administration.

The repository therefore contains more than a runtime. It contains a normative
specification (`spec/cips/`, registries, schemas, test vectors), a reference Rust
implementation, a TypeScript SDK still converging toward stronger parity,
reference capsules, distributed-system experiments, and an increasingly explicit
theoretical corpus in `docs/corpus/`. In its current state, Comum should be read
as a real protocol effort with real discipline and real implementation, but also
as a project still consolidating its structure: runtime parity is incomplete,
mobile execution remains a practical frontier, and the documentation itself is
being reduced from an entropic corpus into a smaller and more mature architecture.

The project is best understood as a foundation, not a finished platform: a
carefully delimited base for sovereign coordination, plural value, reversible
authority, and federative relation without a mandatory center.
