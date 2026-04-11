# Contrato do SDK Mobile (v0.3)

## Objetivo

Definir a API minima e compatibilidade do SDK mobile para Comum Protocol.
Este documento e a SSOT do contrato do SDK, independente de linguagem.

## Escopo

- API de alto nivel para emitir, validar e sincronizar Testemunhos
- Suporte a snapshots e perfis de no (LIGHT/FULL/ARCHIVE)
- Suporte a capsulas via `capsule/invoke` e `capsule/result`
- Offline-first: funcionamento sem conectividade constante

Fora do escopo (por enquanto):

- Transporte fisico (BLE/NFC/QR/RNS) direto no SDK
- UI/UX, armazenamento criptografado e telemetry
- Execucao de capsulas em runtime WASM no dispositivo

## Compatibilidade

- Deve ser compatível com `spec/cips/CIP-0001.md` e registries
- Deve aceitar CBOR canonical e rejeitar nao-canonical
- Deve usar verbos oficiais (registro em `spec/registries/verbs.md`)
- Deve operar com `comum-js` (paridade funcional)

## Tipos base

```
Did            = string
Verb           = string
Bytes          = Uint8Array
TimestampMs    = number
SuiteId        = number
SnapshotCBOR   = Bytes
TestimonyCBOR  = Bytes
```

## API minima

### Node/Runtime

```
Commoner.new(sk: Bytes, suite: SuiteId): Commoner
Commoner.did(): Did
Commoner.clock(): TimestampMs
Commoner.registerPk(pk: Bytes): Bytes   // author hash
Commoner.addSupportedSuite(suite: SuiteId): void
```

### Emit/Validate/Ingest

```
Commoner.emit(verb: Verb, payloadCbor: Bytes, context: ContextInput): EmitResult
Commoner.validate(testimonyCbor: TestimonyCBOR): void
Commoner.ingest(testimonyCbor: TestimonyCBOR): void
```

### Sync

```
Commoner.buildHello(profile: "LIGHT"|"FULL"|"ARCHIVE"): Bytes
Commoner.buildRequest(clock: TimestampMs, limit: number): Bytes
Commoner.applyResponse(payload: Bytes): void

Commoner.buildSnapshotRequest(since: Bytes, limit: number): Bytes
Commoner.applySnapshotResponse(payload: Bytes): void
```

### CTE

```
Commoner.encodeCte(payload: Bytes): Bytes
Commoner.fragmentCte(cte: Bytes, mtu: number, fragId: Bytes): CteFragment[]
Commoner.reassemble(fragments: CteFragment[]): Bytes
```

### Snapshot/Pruning

```
Commoner.pruneBefore(cutoffMs: TimestampMs): number
Commoner.pruneToSnapshot(snapshotCbor: SnapshotCBOR): number
```

## ContextInput

```
ContextInput = {
  type: "proximity"|"beacon"|"place"|"vouch"|"none"
  payload_cbor: Bytes
  proof: ProofInput
}

ProofInput = {
  version: number
  signatures: Bytes[]
  zk_proofs: Bytes[]
  nullifiers: Bytes[]
}

EmitResult = {
  id_hex: string
  cbor: Bytes
}

CteFragment = {
  frag_id: Bytes
  frag_index: number
  frag_total: number
  frag_payload: Bytes
}
```

## Erros

- Erros devem ser deterministas e classificaveis (format/proof/state)
- Mensagens devem incluir causa minima (ex.: "invalid signature")

## Persistencia

- O SDK nao deve impor storage; deve expor interfaces para plugar DB local
- Testemunhos e snapshots devem ser armazenados de forma independente

## Notas de Implementacao

- Em mobile, sempre assumir ambiente offline-first
- Verificacao de CBOR canonical e obrigatoria
- Verbos devem vir do registry (SSOT)

## Compatibilidade com NAPI-RS

- NAPI-RS e opcional e apenas para ambientes Node
- Em mobile, a API deve ser JS puro ou Rust/Swift/Kotlin nativo
