import bs58 from "bs58";
import { sha3_256 } from "@noble/hashes/sha3";
import { sha256 } from "@noble/hashes/sha2";
import { hkdf } from "@noble/hashes/hkdf";
import { hmac } from "@noble/hashes/hmac";

export const VERB_CAPSULE_INVOKE = "capsule/invoke";

export type CommonerContextInput = {
  type: string;
  payload_cbor: Uint8Array;
  proof: CommonerProofInput;
};

export type CommonerProofInput = {
  version: number;
  signatures: Uint8Array[];
  zk_proofs: Uint8Array[];
  nullifiers: Uint8Array[];
};

export type CommonerEmitResult = {
  id_hex: string;
  cbor: Uint8Array;
};

export class Commoner {
  private readonly author: Uint8Array;
  private readonly didValue: string;
  private readonly suite: number;
  private clockValue = 0;

  constructor(sk: Uint8Array, suite: number) {
    if (sk.length !== 32) throw new Error("invalid sk length");
    this.author = sha3_256(sk).slice(0, 32);
    this.didValue = deriveDid(this.author);
    this.suite = suite;
  }

  did(): string {
    return this.didValue;
  }

  clock(): number {
    return this.clockValue;
  }

  registerPk(pk: Uint8Array): Uint8Array {
    if (pk.length !== 32) throw new Error("invalid pk length");
    return sha3_256(pk).slice(0, 32);
  }

  addSupportedSuite(_suite: number): void {}

  validate(testimonyCbor: Uint8Array): void {
    if (testimonyCbor.length === 0) throw new Error("empty testimony");
  }

  ingest(testimonyCbor: Uint8Array): void {
    this.validate(testimonyCbor);
  }

  emit(verb: string, payloadCbor: Uint8Array, context: CommonerContextInput): CommonerEmitResult {
    const timestamp = Date.now();
    const body = buildTestimonyBody(this.author, timestamp, this.suite, verb, payloadCbor, context);
    const id = sha3_256(body);
    this.clockValue = Math.max(this.clockValue, timestamp);
    return { id_hex: bytesToHex(id), cbor: body };
  }

  buildHello(profile: string): Uint8Array {
    const pairs = [
      concatBytes([encodeTstr("profile"), encodeTstr(profile)]),
      concatBytes([encodeTstr("clock"), encodeUint(this.clockValue)]),
      concatBytes([encodeTstr("did"), encodeTstr(this.didValue)]),
    ];
    return encodeMap(pairs);
  }

  buildRequest(clock: number, limit: number): Uint8Array {
    const pairs = [
      concatBytes([encodeTstr("clock"), encodeUint(clock)]),
      concatBytes([encodeTstr("limit"), encodeUint(limit)]),
    ];
    return encodeMap(pairs);
  }

  applyResponse(_payload: Uint8Array): void {}

  encodeCte(payload: Uint8Array): Uint8Array {
    return payload;
  }

  fragmentCte(cte: Uint8Array, mtu: number, fragId: Uint8Array): CteFragment[] {
    if (fragId.length !== 8) throw new Error("invalid frag_id length");
    if (mtu <= 0) throw new Error("invalid mtu");
    const total = Math.ceil(cte.length / mtu);
    const out: CteFragment[] = [];
    for (let i = 0; i < total; i += 1) {
      out.push({
        frag_id: fragId,
        frag_index: i,
        frag_total: total,
        frag_payload: cte.slice(i * mtu, Math.min((i + 1) * mtu, cte.length)),
      });
    }
    return out;
  }

  reassemble(fragments: CteFragment[]): Uint8Array {
    const sorted = [...fragments].sort((a, b) => a.frag_index - b.frag_index);
    return concatBytes(sorted.map((f) => f.frag_payload));
  }
}

export type CteFragment = {
  frag_id: Uint8Array;
  frag_index: number;
  frag_total: number;
  frag_payload: Uint8Array;
};

export function invokeCapsule(
  commoner: Commoner,
  capsuleId: Uint8Array,
  action: string,
  paramsCbor: Uint8Array,
  context: CommonerContextInput
): CommonerEmitResult {
  const payload = buildInvokePayload(capsuleId, action, paramsCbor);
  return commoner.emit(VERB_CAPSULE_INVOKE, payload, context);
}

function buildInvokePayload(capsuleId: Uint8Array, action: string, params: Uint8Array): Uint8Array {
  const pairs = [
    concatBytes([encodeTstr("action"), encodeTstr(action)]),
    concatBytes([encodeTstr("params"), encodeBstr(params)]),
    concatBytes([encodeTstr("capsule_id"), encodeBstr(capsuleId)]),
  ];
  return encodeMap(pairs);
}

export function offer(
  commoner: Commoner,
  capsuleId: Uint8Array,
  paramsCbor: Uint8Array,
  context: CommonerContextInput
): CommonerEmitResult {
  return invokeCapsule(commoner, capsuleId, "offer", paramsCbor, context);
}

export function accept(
  commoner: Commoner,
  capsuleId: Uint8Array,
  paramsCbor: Uint8Array,
  context: CommonerContextInput
): CommonerEmitResult {
  return invokeCapsule(commoner, capsuleId, "accept", paramsCbor, context);
}

export function receipt(
  commoner: Commoner,
  capsuleId: Uint8Array,
  paramsCbor: Uint8Array,
  context: CommonerContextInput
): CommonerEmitResult {
  return invokeCapsule(commoner, capsuleId, "receipt", paramsCbor, context);
}

export function computeOfferId(
  item: string,
  price: number,
  currency: string,
  expires: number,
  seller: string
): Uint8Array {
  const cbor = encodeOfferCore(item, price, currency, expires, seller);
  return sha3_256(cbor);
}

export function buildOfferPayload(
  item: string,
  price: number,
  currency: string,
  expires: number,
  seller: string
): Uint8Array {
  const offerId = computeOfferId(item, price, currency, expires, seller);
  return encodeOfferWithId(item, price, currency, expires, seller, offerId);
}

export function buildAcceptPayload(offerId: Uint8Array, buyer: string): Uint8Array {
  const pairs = [
    concatBytes([encodeTstr("buyer"), encodeTstr(buyer)]),
    concatBytes([encodeTstr("offer_id"), encodeBstr(offerId)]),
  ];
  return encodeMap(pairs);
}

export function buildReceiptPayload(offerId: Uint8Array, timestamp: number): Uint8Array {
  const pairs = [
    concatBytes([encodeTstr("offer_id"), encodeBstr(offerId)]),
    concatBytes([encodeTstr("timestamp"), encodeUint(timestamp)]),
  ];
  return encodeMap(pairs);
}

function encodeOfferCore(
  item: string,
  price: number,
  currency: string,
  expires: number,
  seller: string
): Uint8Array {
  const pairs = [
    concatBytes([encodeTstr("item"), encodeTstr(item)]),
    concatBytes([encodeTstr("price"), encodeUint(price)]),
    concatBytes([encodeTstr("seller"), encodeTstr(seller)]),
    concatBytes([encodeTstr("expires"), encodeUint(expires)]),
    concatBytes([encodeTstr("currency"), encodeTstr(currency)]),
  ];
  return encodeMap(pairs);
}

function encodeOfferWithId(
  item: string,
  price: number,
  currency: string,
  expires: number,
  seller: string,
  offerId: Uint8Array
): Uint8Array {
  const pairs = [
    concatBytes([encodeTstr("item"), encodeTstr(item)]),
    concatBytes([encodeTstr("price"), encodeUint(price)]),
    concatBytes([encodeTstr("seller"), encodeTstr(seller)]),
    concatBytes([encodeTstr("expires"), encodeUint(expires)]),
    concatBytes([encodeTstr("currency"), encodeTstr(currency)]),
    concatBytes([encodeTstr("offer_id"), encodeBstr(offerId)]),
  ];
  return encodeMap(pairs);
}

function buildTestimonyBody(
  author: Uint8Array,
  timestamp: number,
  suite: number,
  verb: string,
  payloadCbor: Uint8Array,
  context: CommonerContextInput
): Uint8Array {
  const pairs = [
    concatBytes([encodeTstr("version"), encodeUint(3)]),
    concatBytes([encodeTstr("author"), encodeBstr(author)]),
    concatBytes([encodeTstr("timestamp"), encodeUint(timestamp)]),
    concatBytes([encodeTstr("suite"), encodeUint(suite)]),
    concatBytes([encodeTstr("verb"), encodeTstr(verb)]),
    concatBytes([encodeTstr("payload_cbor"), encodeBstr(payloadCbor)]),
    concatBytes([encodeTstr("context_type"), encodeTstr(context.type)]),
    concatBytes([encodeTstr("context_payload"), encodeBstr(context.payload_cbor)]),
  ];
  return encodeMap(pairs);
}

function concatBytes(chunks: Uint8Array[]): Uint8Array {
  let total = 0;
  for (const chunk of chunks) total += chunk.length;
  const out = new Uint8Array(total);
  let offset = 0;
  for (const chunk of chunks) {
    out.set(chunk, offset);
    offset += chunk.length;
  }
  return out;
}

function encodeUnsignedHeader(major: number, len: number): Uint8Array {
  if (len < 24) return new Uint8Array([major | len]);
  if (len < 256) return new Uint8Array([major | 24, len]);
  if (len < 65536) return new Uint8Array([major | 25, (len >> 8) & 0xff, len & 0xff]);
  return new Uint8Array([
    major | 26,
    (len >> 24) & 0xff,
    (len >> 16) & 0xff,
    (len >> 8) & 0xff,
    len & 0xff,
  ]);
}

function encodeUint(n: number): Uint8Array {
  if (!Number.isSafeInteger(n) || n < 0) throw new Error("invalid uint");
  if (n < 24) return new Uint8Array([n]);
  if (n < 256) return new Uint8Array([0x18, n]);
  if (n < 65536) return new Uint8Array([0x19, (n >> 8) & 0xff, n & 0xff]);
  if (n < 4294967296)
    return new Uint8Array([0x1a, (n >> 24) & 0xff, (n >> 16) & 0xff, (n >> 8) & 0xff, n & 0xff]);
  const big = BigInt(n);
  return new Uint8Array([
    0x1b,
    Number((big >> 56n) & 0xffn),
    Number((big >> 48n) & 0xffn),
    Number((big >> 40n) & 0xffn),
    Number((big >> 32n) & 0xffn),
    Number((big >> 24n) & 0xffn),
    Number((big >> 16n) & 0xffn),
    Number((big >> 8n) & 0xffn),
    Number(big & 0xffn),
  ]);
}

function encodeBstr(data: Uint8Array): Uint8Array {
  return concatBytes([encodeUnsignedHeader(0x40, data.length), data]);
}

function encodeTstr(text: string): Uint8Array {
  const data = new TextEncoder().encode(text);
  return concatBytes([encodeUnsignedHeader(0x60, data.length), data]);
}

function encodeMap(pairs: Uint8Array[]): Uint8Array {
  return concatBytes([encodeUnsignedHeader(0xa0, pairs.length), ...pairs]);
}

function bytesToHex(data: Uint8Array): string {
  return Array.from(data)
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");
}

export function deriveDid(pk: Uint8Array): string {
  if (pk.length !== 32) throw new Error("invalid pk length");
  const digest = sha3_256(pk);
  const short = digest.slice(0, 20);
  const checksum = sha256(sha256(short)).slice(0, 4);
  const data = new Uint8Array(short.length + checksum.length);
  data.set(short, 0);
  data.set(checksum, short.length);
  return `did:comum:${bs58.encode(data)}`;
}

export function computeNullifier(sk: Uint8Array, testimonyId: Uint8Array): Uint8Array {
  if (sk.length !== 32) throw new Error("invalid sk length");
  if (testimonyId.length !== 32) throw new Error("invalid id length");
  const info = new TextEncoder().encode("comum-nullifier-v1");
  const key = hkdf(sha3_256, sk, new Uint8Array(), info, 32);
  return hmac(sha3_256, key, testimonyId);
}

// Re-export high-level client (mobile build)
export { ComumClient } from "./client.mobile.js";
export type {
  OfferParams,
  Offer,
  AcceptResult,
  ReceiptResult,
  NodeTransport,
} from "./client.mobile.js";

// Re-export WebSocket transport (usa apenas APIs de plataforma, sem node:)
export { makeWsTransport } from "./transport/ws.js";
export type { WsTransportOptions } from "./transport/ws.js";
