import {
  Commoner,
  offer,
  accept,
  receipt,
  buildOfferPayload,
  buildAcceptPayload,
  buildReceiptPayload,
  computeOfferId,
  type CommonerEmitResult,
} from 'comum-js'

export type OfferInput = {
  item: string
  price: number
  currency: string
  expires: number
}

export type NodeTransport = {
  start: (onTestimony: (testimony: Uint8Array) => void) => Promise<void> | void
  publish: (testimony: Uint8Array) => void
  close?: () => void
}

const EMPTY_CONTEXT = {
  type: 'none',
  payload_cbor: new Uint8Array([0xa0]),
  proof: { version: 1, signatures: [], zk_proofs: [], nullifiers: [] },
}

export class AppNode {
  private readonly core: Commoner
  private readonly capsuleId: Uint8Array
  private readonly testimonies: Uint8Array[] = []
  private transport: NodeTransport | null = null

  constructor(seed: Uint8Array, suite: number, capsuleId: Uint8Array) {
    this.core = new Commoner(seed, suite)
    this.capsuleId = capsuleId
  }

  did(): string {
    return this.core.did()
  }

  async attachTransport(transport: NodeTransport): Promise<void> {
    this.transport = transport
    await transport.start((testimony) => {
      this.ingest(testimony)
    })
  }

  closeTransport(): void {
    this.transport?.close?.()
    this.transport = null
  }

  publishOffer(input: OfferInput): CommonerEmitResult {
    const payload = buildOfferPayload(
      input.item,
      input.price,
      input.currency,
      input.expires,
      this.did()
    )
    const out = offer(this.core, this.capsuleId, payload, EMPTY_CONTEXT)
    this.testimonies.push(out.cbor)
    this.transport?.publish(out.cbor)
    return out
  }

  acceptOffer(offerId: Uint8Array): CommonerEmitResult {
    const payload = buildAcceptPayload(offerId, this.did())
    const out = accept(this.core, this.capsuleId, payload, EMPTY_CONTEXT)
    this.testimonies.push(out.cbor)
    this.transport?.publish(out.cbor)
    return out
  }

  issueReceipt(offerId: Uint8Array, timestamp: number): CommonerEmitResult {
    const payload = buildReceiptPayload(offerId, timestamp)
    const out = receipt(this.core, this.capsuleId, payload, EMPTY_CONTEXT)
    this.testimonies.push(out.cbor)
    this.transport?.publish(out.cbor)
    return out
  }

  computeOfferId(input: OfferInput): Uint8Array {
    return computeOfferId(input.item, input.price, input.currency, input.expires, this.did())
  }

  ingest(testimony: Uint8Array): void {
    this.core.ingest(testimony)
    this.testimonies.push(testimony)
  }

  syncTo(peer: AppNode): number {
    for (const t of this.testimonies) peer.ingest(t)
    return this.testimonies.length
  }

  testimonyCount(): number {
    return this.testimonies.length
  }
}
