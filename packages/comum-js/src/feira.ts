import { sha3_256 } from "@noble/hashes/sha3";
import { Commoner, invokeCapsule, CommonerContextInput } from "./index.js";

// Thin wrapper for feira capsule (non-normative ergonomics)

export function offer(
  commoner: Commoner,
  capsuleId: Uint8Array,
  paramsCbor: Uint8Array,
  ctx: CommonerContextInput
) {
  return invokeCapsule(commoner, capsuleId, "offer", paramsCbor, ctx);
}

export function accept(
  commoner: Commoner,
  capsuleId: Uint8Array,
  paramsCbor: Uint8Array,
  ctx: CommonerContextInput
) {
  return invokeCapsule(commoner, capsuleId, "accept", paramsCbor, ctx);
}

export function receipt(
  commoner: Commoner,
  capsuleId: Uint8Array,
  paramsCbor: Uint8Array,
  ctx: CommonerContextInput
) {
  return invokeCapsule(commoner, capsuleId, "receipt", paramsCbor, ctx);
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
  const pairs: number[][] = [];
  pairs.push([...encodeTstr("buyer"), ...encodeTstr(buyer)]);
  pairs.push([...encodeTstr("offer_id"), ...encodeBstr(offerId)]);
  return new Uint8Array(encodeMap(pairs));
}

export function buildReceiptPayload(offerId: Uint8Array, timestamp: number): Uint8Array {
  const pairs: number[][] = [];
  pairs.push([...encodeTstr("offer_id"), ...encodeBstr(offerId)]);
  pairs.push([...encodeTstr("timestamp"), ...encodeUint(timestamp)]);
  return new Uint8Array(encodeMap(pairs));
}

function encodeOfferCore(
  item: string,
  price: number,
  currency: string,
  expires: number,
  seller: string
): Uint8Array {
  const pairs: number[][] = [];
  pairs.push([...encodeTstr("item"), ...encodeTstr(item)]);
  pairs.push([...encodeTstr("price"), ...encodeUint(price)]);
  pairs.push([...encodeTstr("seller"), ...encodeTstr(seller)]);
  pairs.push([...encodeTstr("expires"), ...encodeUint(expires)]);
  pairs.push([...encodeTstr("currency"), ...encodeTstr(currency)]);
  return new Uint8Array(encodeMap(pairs));
}

function encodeOfferWithId(
  item: string,
  price: number,
  currency: string,
  expires: number,
  seller: string,
  offerId: Uint8Array
): Uint8Array {
  const pairs: number[][] = [];
  pairs.push([...encodeTstr("item"), ...encodeTstr(item)]);
  pairs.push([...encodeTstr("price"), ...encodeUint(price)]);
  pairs.push([...encodeTstr("seller"), ...encodeTstr(seller)]);
  pairs.push([...encodeTstr("expires"), ...encodeUint(expires)]);
  pairs.push([...encodeTstr("currency"), ...encodeTstr(currency)]);
  pairs.push([...encodeTstr("offer_id"), ...encodeBstr(offerId)]);
  return new Uint8Array(encodeMap(pairs));
}

function encodeUint(n: number): number[] {
  if (!Number.isInteger(n) || n < 0) throw new Error("uint required");
  if (n < 24) return [n];
  if (n < 256) return [0x18, n];
  if (n < 65536) return [0x19, (n >> 8) & 0xff, n & 0xff];
  if (n < 4294967296) {
    return [
      0x1a,
      (n >> 24) & 0xff,
      (n >> 16) & 0xff,
      (n >> 8) & 0xff,
      n & 0xff,
    ];
  }
  const bi = BigInt(n);
  return [
    0x1b,
    Number((bi >> 56n) & 0xffn),
    Number((bi >> 48n) & 0xffn),
    Number((bi >> 40n) & 0xffn),
    Number((bi >> 32n) & 0xffn),
    Number((bi >> 24n) & 0xffn),
    Number((bi >> 16n) & 0xffn),
    Number((bi >> 8n) & 0xffn),
    Number(bi & 0xffn),
  ];
}

function encodeBstr(data: Uint8Array): number[] {
  const len = data.length;
  if (len < 24) return [0x40 + len, ...data];
  if (len < 256) return [0x58, len, ...data];
  if (len < 65536) return [0x59, (len >> 8) & 0xff, len & 0xff, ...data];
  throw new Error("bstr too long");
}

function encodeTstr(s: string): number[] {
  const data = new TextEncoder().encode(s);
  const len = data.length;
  if (len < 24) return [0x60 + len, ...data];
  if (len < 256) return [0x78, len, ...data];
  if (len < 65536) return [0x79, (len >> 8) & 0xff, len & 0xff, ...data];
  throw new Error("tstr too long");
}

function encodeMap(pairs: number[][]): number[] {
  const len = pairs.length;
  const out = len < 24 ? [0xa0 + len] : [0xb8, len];
  for (const p of pairs) out.push(...p);
  return out;
}
