/**
 * ComumClient — versao mobile (React Native).
 *
 * Mesma API publica do client.ts, mas usa o Commoner mobile-only
 * (sem N-API, sem node:child_process).
 */

import {
  Commoner,
  invokeCapsule,
  type CommonerContextInput,
  type CommonerEmitResult,
  buildOfferPayload,
  buildAcceptPayload,
  buildReceiptPayload,
  computeOfferId,
} from "./mobile.js";

export type OfferParams = {
  item: string;
  price: number;
  currency: string;
  expiresAt: number;
};

export type Offer = {
  id: Uint8Array;
  idHex: string;
  item: string;
  price: number;
  currency: string;
  expiresAt: number;
  seller: string;
  testimonyCbor: Uint8Array;
};

export type AcceptResult = {
  offerId: Uint8Array;
  testimonyCbor: Uint8Array;
  testimonyIdHex: string;
};

export type ReceiptResult = {
  offerId: Uint8Array;
  testimonyCbor: Uint8Array;
  testimonyIdHex: string;
  timestamp: number;
};

export type NodeTransport = {
  start: (onTestimony: (cbor: Uint8Array) => void) => Promise<void> | void;
  publish: (cbor: Uint8Array) => void;
  close?: () => void;
};

const EMPTY_CONTEXT: CommonerContextInput = {
  type: "none",
  payload_cbor: new Uint8Array([0xa0]),
  proof: { version: 1, signatures: [], zk_proofs: [], nullifiers: [] },
};

export class ComumClient {
  private readonly core: Commoner;
  private readonly capsuleId: Uint8Array;
  private readonly store: Uint8Array[] = [];
  private readonly offerIndex: Map<string, Offer> = new Map();
  private transport: NodeTransport | null = null;
  private receiptListeners: Map<string, Array<(r: ReceiptResult) => void>> =
    new Map();

  constructor(sk: Uint8Array, capsuleId: Uint8Array, suite = 1) {
    this.core = new Commoner(sk, suite);
    this.capsuleId = capsuleId;
  }

  did(): string {
    return this.core.did();
  }

  shortDid(): string {
    return this.did().slice(-8);
  }

  async connect(transport: NodeTransport): Promise<void> {
    this.transport = transport;
    await transport.start((cbor) => this._ingest(cbor));
  }

  disconnect(): void {
    this.transport?.close?.();
    this.transport = null;
  }

  createOffer(params: OfferParams): Offer {
    const seller = this.did();
    const payload = buildOfferPayload(
      params.item,
      params.price,
      params.currency,
      params.expiresAt,
      seller
    );
    const id = computeOfferId(
      params.item,
      params.price,
      params.currency,
      params.expiresAt,
      seller
    );
    const out = this._emitCapsule("offer", payload);
    const offer: Offer = {
      id,
      idHex: bytesToHex(id),
      item: params.item,
      price: params.price,
      currency: params.currency,
      expiresAt: params.expiresAt,
      seller,
      testimonyCbor: out.cbor,
    };
    this.offerIndex.set(offer.idHex, offer);
    return offer;
  }

  acceptOffer(offerId: Uint8Array): AcceptResult {
    const payload = buildAcceptPayload(offerId, this.did());
    const out = this._emitCapsule("accept", payload);
    return {
      offerId,
      testimonyCbor: out.cbor,
      testimonyIdHex: out.id_hex,
    };
  }

  issueReceipt(offerId: Uint8Array): ReceiptResult {
    const timestamp = Date.now();
    const payload = buildReceiptPayload(offerId, timestamp);
    const out = this._emitCapsule("receipt", payload);
    const idHex = bytesToHex(offerId);
    this._notifyReceiptListeners(idHex, {
      offerId,
      testimonyCbor: out.cbor,
      testimonyIdHex: out.id_hex,
      timestamp,
    });
    return {
      offerId,
      testimonyCbor: out.cbor,
      testimonyIdHex: out.id_hex,
      timestamp,
    };
  }

  knownOffers(): Offer[] {
    return Array.from(this.offerIndex.values());
  }

  testimonyCount(): number {
    return this.store.length;
  }

  onReceipt(offerIdHex: string, fn: (r: ReceiptResult) => void): () => void {
    if (!this.receiptListeners.has(offerIdHex)) {
      this.receiptListeners.set(offerIdHex, []);
    }
    this.receiptListeners.get(offerIdHex)!.push(fn);
    return () => {
      const arr = this.receiptListeners.get(offerIdHex) ?? [];
      const idx = arr.indexOf(fn);
      if (idx !== -1) arr.splice(idx, 1);
    };
  }

  syncTo(peer: ComumClient): number {
    for (const t of this.store) peer._ingest(t);
    return this.store.length;
  }

  _ingest(cbor: Uint8Array): void {
    this.core.ingest(cbor);
    this.store.push(cbor);
  }

  private _emitCapsule(
    action: string,
    paramsCbor: Uint8Array
  ): CommonerEmitResult {
    const out = invokeCapsule(
      this.core,
      this.capsuleId,
      action,
      paramsCbor,
      EMPTY_CONTEXT
    );
    this.store.push(out.cbor);
    this.transport?.publish(out.cbor);
    return out;
  }

  private _notifyReceiptListeners(idHex: string, r: ReceiptResult): void {
    for (const fn of this.receiptListeners.get(idHex) ?? []) fn(r);
  }
}

function bytesToHex(data: Uint8Array): string {
  return Array.from(data)
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");
}
