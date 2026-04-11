/**
 * ComumClient — fachada de alto nivel para o fluxo Feira MVP.
 *
 * Objetivo: o codigo de app nao deve conhecer CBOR, verbos, ou detalhes
 * internos de Commoner. Tudo isso fica aqui.
 *
 * Nao normativo: esta camada nao altera o contrato do protocolo.
 * Serve apenas como boundary limpo entre app e SDK.
 */

import {
  Commoner,
  invokeCapsule,
  type CommonerContextInput,
  type CommonerEmitResult,
} from "./index.js";
import {
  buildOfferPayload,
  buildAcceptPayload,
  buildReceiptPayload,
  computeOfferId,
} from "./feira.js";

// ---------------------------------------------------------------------------
// Tipos publicos
// ---------------------------------------------------------------------------

export type OfferParams = {
  item: string;
  price: number;
  currency: string;
  /** Unix timestamp ms */
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

// ---------------------------------------------------------------------------
// Contexto vazio (sem prova de proximidade — suficiente para Feira MVP)
// ---------------------------------------------------------------------------

const EMPTY_CONTEXT: CommonerContextInput = {
  type: "none",
  payload_cbor: new Uint8Array([0xa0]), // CBOR empty map
  proof: { version: 1, signatures: [], zk_proofs: [], nullifiers: [] },
};

// ---------------------------------------------------------------------------
// ComumClient
// ---------------------------------------------------------------------------

export class ComumClient {
  private readonly core: Commoner;
  private readonly capsuleId: Uint8Array;

  /** Testemunhos locais (emitidos + ingeridos). */
  private readonly store: Uint8Array[] = [];

  /** Ofertas vistas localmente, indexadas por idHex. */
  private readonly offerIndex: Map<string, Offer> = new Map();

  private transport: NodeTransport | null = null;

  private receiptListeners: Map<string, Array<(r: ReceiptResult) => void>> =
    new Map();

  constructor(sk: Uint8Array, capsuleId: Uint8Array, suite = 1) {
    this.core = new Commoner(sk, suite);
    this.capsuleId = capsuleId;
  }

  // -------------------------------------------------------------------------
  // Identidade
  // -------------------------------------------------------------------------

  /** DID do no local. */
  did(): string {
    return this.core.did();
  }

  /** Versao curta do DID para display (ultimos 8 chars). */
  shortDid(): string {
    const d = this.did();
    return d.slice(-8);
  }

  // -------------------------------------------------------------------------
  // Transporte
  // -------------------------------------------------------------------------

  async connect(transport: NodeTransport): Promise<void> {
    this.transport = transport;
    await transport.start((cbor) => this._ingest(cbor));
  }

  disconnect(): void {
    this.transport?.close?.();
    this.transport = null;
  }

  // -------------------------------------------------------------------------
  // Fluxo Feira
  // -------------------------------------------------------------------------

  /**
   * Publica uma oferta.
   * Retorna o objeto Offer com id pre-computado.
   */
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

  /**
   * Aceita uma oferta recebida pelo id (Uint8Array de 32 bytes).
   */
  acceptOffer(offerId: Uint8Array): AcceptResult {
    const payload = buildAcceptPayload(offerId, this.did());
    const out = this._emitCapsule("accept", payload);
    return {
      offerId,
      testimonyCbor: out.cbor,
      testimonyIdHex: out.id_hex,
    };
  }

  /**
   * Emite o receipt de uma oferta aceita.
   */
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

  // -------------------------------------------------------------------------
  // Observabilidade
  // -------------------------------------------------------------------------

  /** Ofertas conhecidas localmente (proprias + recebidas via sync). */
  knownOffers(): Offer[] {
    return Array.from(this.offerIndex.values());
  }

  /** Numero de testemunhos no store local. */
  testimonyCount(): number {
    return this.store.length;
  }

  /**
   * Registra um listener que sera chamado quando um receipt chegar
   * para o offerId dado. Util para UI aguardar confirmacao.
   */
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

  // -------------------------------------------------------------------------
  // Sync manual (usado em demo / testes)
  // -------------------------------------------------------------------------

  /** Envia todos os testemunhos locais para outro client (simulacao de sync). */
  syncTo(peer: ComumClient): number {
    for (const t of this.store) peer._ingest(t);
    return this.store.length;
  }

  // -------------------------------------------------------------------------
  // Internos
  // -------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Util
// ---------------------------------------------------------------------------

function bytesToHex(data: Uint8Array): string {
  return Array.from(data)
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");
}
