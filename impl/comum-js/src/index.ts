import { spawnSync } from "node:child_process";
import { createRequire } from "node:module";
import bs58 from "bs58";
import { sha3_256 } from "@noble/hashes/sha3";
import { sha256 } from "@noble/hashes/sha2";
import { hkdf } from "@noble/hashes/hkdf";
import { hmac } from "@noble/hashes/hmac";

export type EncodeResult = {
  cbor_hex: string;
  id: string;
};

export type EncodeOptions = {
  bin?: string;
};

export const VERB_COMUM_TRANSFER = "comum/transfer";
export const VERB_COMUM_VOUCH = "comum/vouch";
export const VERB_COMUM_ENCOUNTER = "comum/encounter";
export const VERB_COMUM_VOTE = "comum/vote";
export const VERB_COMUM_PROPOSE = "comum/propose";
export const VERB_COMUM_REVOKE = "comum/revoke";
export const VERB_COMUM_KEY_ROTATE = "comum/key_rotate";
export const VERB_COMUM_RECEIVE = "comum/receive";
export const VERB_CAPSULE_INVOKE = "capsule/invoke";
export const VERB_CAPSULE_RESULT = "capsule/result";
export const VERB_GENESIS = "genesis";

type NativeModule = {
  encode_testimony?: (json: string) => string;
  Commoner?: new (sk: Buffer, suite: number) => NativeCommoner;
};

type NativeCommoner = {
  did(): string;
  clock(): number;
  register_pk(pk: Buffer): Buffer;
  add_supported_suite(suite: number): void;
  validate(testimony_cbor: Buffer): void;
  ingest(testimony_cbor: Buffer): void;
  emit(verb: string, payload_cbor: Buffer, context: NativeContextInput): NativeEmitResult;
  build_hello(profile: string): Buffer;
  build_request(clock: number, limit: number): Buffer;
  apply_response(payload: Buffer): void;
  encode_cte(payload: Buffer): Buffer;
  fragment_cte(cte: Buffer, mtu: number, frag_id: Buffer): NativeCteFragment[];
  reassemble(fragments: NativeCteFragment[]): Buffer;
};

type NativeContextInput = {
  type: string;
  payload_cbor: Buffer;
  proof: NativeProofInput;
};

type NativeProofInput = {
  version: number;
  signatures: Buffer[];
  zk_proofs: Buffer[];
  nullifiers: Buffer[];
};

type NativeEmitResult = {
  id_hex: string;
  cbor: Buffer;
};

type NativeCteFragment = {
  frag_id: Buffer;
  frag_index: number;
  frag_total: number;
  frag_payload: Buffer;
};

let nativeCache: NativeModule | null | undefined;

export function loadNative(): NativeModule | null {
  if (nativeCache !== undefined) return nativeCache;
  const require = createRequire(import.meta.url);
  try {
    if (process.env.COMUM_NAPI_PATH) {
      nativeCache = require(process.env.COMUM_NAPI_PATH) as NativeModule;
      return nativeCache;
    }
    nativeCache = require("comum-napi") as NativeModule;
    return nativeCache;
  } catch {
    nativeCache = null;
    return null;
  }
}

export function encodeTestimony(
  testimonyWithoutId: unknown,
  options: EncodeOptions = {}
): EncodeResult {
  const input = JSON.stringify(testimonyWithoutId);
  const native = loadNative();
  if (native?.encode_testimony) {
    const out = native.encode_testimony(input);
    return JSON.parse(out) as EncodeResult;
  }
  const bin = options.bin || process.env.COMUM_RS_BIN || "comum-cbor";
  const res = spawnSync(bin, [], { input, encoding: "utf8" });
  if (res.error) throw res.error;
  if (res.status !== 0) throw new Error(res.stderr || "comum-cbor failed");
  return JSON.parse(res.stdout) as EncodeResult;
}

export function verifyTestimony(
  testimonyWithoutId: unknown,
  expectedId: string,
  options: EncodeOptions = {}
): boolean {
  const out = encodeTestimony(testimonyWithoutId, options);
  return out.id === expectedId;
}

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

export type CteFragment = {
  frag_id: Uint8Array;
  frag_index: number;
  frag_total: number;
  frag_payload: Uint8Array;
};

export class Commoner {
  private native: NativeCommoner;

  constructor(sk: Uint8Array, suite: number) {
    const native = loadNative();
    if (!native?.Commoner) {
      throw new Error("comum-napi not available; set COMUM_NAPI_PATH or install comum-napi");
    }
    this.native = new native.Commoner(Buffer.from(sk), suite);
  }

  did(): string {
    return this.native.did();
  }

  clock(): number {
    return this.native.clock();
  }

  registerPk(pk: Uint8Array): Uint8Array {
    return this.native.register_pk(Buffer.from(pk));
  }

  addSupportedSuite(suite: number): void {
    this.native.add_supported_suite(suite);
  }

  validate(testimonyCbor: Uint8Array): void {
    this.native.validate(Buffer.from(testimonyCbor));
  }

  ingest(testimonyCbor: Uint8Array): void {
    this.native.ingest(Buffer.from(testimonyCbor));
  }

  emit(verb: string, payloadCbor: Uint8Array, context: CommonerContextInput): CommonerEmitResult {
    const out = this.native.emit(verb, Buffer.from(payloadCbor), {
      type: context.type,
      payload_cbor: Buffer.from(context.payload_cbor),
      proof: {
        version: context.proof.version,
        signatures: context.proof.signatures.map((s) => Buffer.from(s)),
        zk_proofs: context.proof.zk_proofs.map((s) => Buffer.from(s)),
        nullifiers: context.proof.nullifiers.map((s) => Buffer.from(s)),
      },
    });
    return { id_hex: out.id_hex, cbor: new Uint8Array(out.cbor) };
  }

  buildHello(profile: string): Uint8Array {
    return new Uint8Array(this.native.build_hello(profile));
  }

  buildRequest(clock: number, limit: number): Uint8Array {
    return new Uint8Array(this.native.build_request(clock, limit));
  }

  applyResponse(payload: Uint8Array): void {
    this.native.apply_response(Buffer.from(payload));
  }

  encodeCte(payload: Uint8Array): Uint8Array {
    return new Uint8Array(this.native.encode_cte(Buffer.from(payload)));
  }

  fragmentCte(cte: Uint8Array, mtu: number, fragId: Uint8Array): CteFragment[] {
    return this.native.fragment_cte(Buffer.from(cte), mtu, Buffer.from(fragId)).map((f) => ({
      frag_id: new Uint8Array(f.frag_id),
      frag_index: f.frag_index,
      frag_total: f.frag_total,
      frag_payload: new Uint8Array(f.frag_payload),
    }));
  }

  reassemble(fragments: CteFragment[]): Uint8Array {
    return new Uint8Array(
      this.native.reassemble(
        fragments.map((f) => ({
          frag_id: Buffer.from(f.frag_id),
          frag_index: f.frag_index,
          frag_total: f.frag_total,
          frag_payload: Buffer.from(f.frag_payload),
        }))
      )
    );
  }
}

export function deriveDid(pk: Uint8Array): string {
  if (pk.length !== 32) throw new Error("invalid pk length");
  const digest = sha3_256(pk);
  const short = digest.slice(0, 20);
  const checksum = sha256(sha256(short)).slice(0, 4);
  const data = new Uint8Array(short.length + checksum.length);
  data.set(short, 0);
  data.set(checksum, short.length);
  const b58 = bs58.encode(data);
  return `did:comum:${b58}`;
}

export function computeNullifier(sk: Uint8Array, testimonyId: Uint8Array): Uint8Array {
  if (sk.length !== 32) throw new Error("invalid sk length");
  if (testimonyId.length !== 32) throw new Error("invalid id length");
  const info = new TextEncoder().encode("comum-nullifier-v1");
  const key = hkdf(sha3_256, sk, new Uint8Array(), info, 32);
  return hmac(sha3_256, key, testimonyId);
}

type CborMap = Record<string, CborValue>;
type CborValue =
  | { type: "uint"; value: number }
  | { type: "bytes"; value: Uint8Array }
  | { type: "text"; value: string }
  | { type: "array"; value: CborValue[] }
  | { type: "map"; value: CborMap };

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
  if (len < 4294967296)
    return new Uint8Array([
      major | 26,
      (len >> 24) & 0xff,
      (len >> 16) & 0xff,
      (len >> 8) & 0xff,
      len & 0xff,
    ]);
  throw new Error("length too large");
}

function encodeUint(n: number): Uint8Array {
  if (!Number.isSafeInteger(n) || n < 0) throw new Error("invalid uint");
  if (n < 24) return new Uint8Array([n]);
  if (n < 256) return new Uint8Array([0x18, n]);
  if (n < 65536) return new Uint8Array([0x19, (n >> 8) & 0xff, n & 0xff]);
  if (n < 4294967296)
    return new Uint8Array([
      0x1a,
      (n >> 24) & 0xff,
      (n >> 16) & 0xff,
      (n >> 8) & 0xff,
      n & 0xff,
    ]);
  const big = BigInt(n);
  if (big > BigInt(Number.MAX_SAFE_INTEGER)) throw new Error("uint too large");
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
  const header = encodeUnsignedHeader(0x40, data.length);
  return concatBytes([header, data]);
}

function encodeTstr(text: string): Uint8Array {
  const data = Buffer.from(text, "utf8");
  const header = encodeUnsignedHeader(0x60, data.length);
  return concatBytes([header, data]);
}

function encodeMap(pairs: Uint8Array[]): Uint8Array {
  const header = encodeUnsignedHeader(0xa0, pairs.length);
  return concatBytes([header, ...pairs]);
}

function encodeArray(items: Uint8Array[]): Uint8Array {
  const header = encodeUnsignedHeader(0x80, items.length);
  return concatBytes([header, ...items]);
}

export function buildProximityContextPayload(
  method: "nfc" | "ble",
  nonce: Uint8Array,
  timestamp: number
): Uint8Array {
  const pairs = [
    concatBytes([encodeTstr("nonce"), encodeBstr(nonce)]),
    concatBytes([encodeTstr("method"), encodeTstr(method)]),
    concatBytes([encodeTstr("timestamp"), encodeUint(timestamp)]),
  ];
  return encodeMap(pairs);
}

export function buildBeaconContextPayload(
  beaconId: Uint8Array,
  token: Uint8Array,
  timestamp: number
): Uint8Array {
  const pairs = [
    concatBytes([encodeTstr("token"), encodeBstr(token)]),
    concatBytes([encodeTstr("beacon_id"), encodeBstr(beaconId)]),
    concatBytes([encodeTstr("timestamp"), encodeUint(timestamp)]),
  ];
  return encodeMap(pairs);
}

export function buildPlaceContextPayload(
  placeHash: Uint8Array,
  timestamp: number
): Uint8Array {
  const pairs = [
    concatBytes([encodeTstr("timestamp"), encodeUint(timestamp)]),
    concatBytes([encodeTstr("place_hash"), encodeBstr(placeHash)]),
  ];
  return encodeMap(pairs);
}

export function buildVouchContextPayload(
  subject: string,
  community: Uint8Array,
  timestamp: number
): Uint8Array {
  const pairs = [
    concatBytes([encodeTstr("subject"), encodeTstr(subject)]),
    concatBytes([encodeTstr("community"), encodeBstr(community)]),
    concatBytes([encodeTstr("timestamp"), encodeUint(timestamp)]),
  ];
  return encodeMap(pairs);
}

export function buildReceivePayload(of: Uint8Array, timestamp: number): Uint8Array {
  const pairs = [
    concatBytes([encodeTstr("of"), encodeBstr(of)]),
    concatBytes([encodeTstr("timestamp"), encodeUint(timestamp)]),
  ];
  return encodeMap(pairs);
}

export function buildGenesisPayload(
  name: string,
  threshold: number,
  founders: string[],
  capsules: Uint8Array[],
  supply: number,
  mintPolicy: Uint8Array
): Uint8Array {
  const capsuleItems = capsules.map((c) => encodeBstr(c));
  const founderItems = founders.map((d) => encodeTstr(d));
  const pairs = [
    concatBytes([encodeTstr("name"), encodeTstr(name)]),
    concatBytes([encodeTstr("supply"), encodeUint(supply)]),
    concatBytes([encodeTstr("capsules"), encodeArray(capsuleItems)]),
    concatBytes([encodeTstr("founders"), encodeArray(founderItems)]),
    concatBytes([encodeTstr("threshold"), encodeUint(threshold)]),
    concatBytes([encodeTstr("mint_policy"), encodeBstr(mintPolicy)]),
  ];
  return encodeMap(pairs);
}

function decodeUnsigned(data: Uint8Array, offset: number) {
  const first = data[offset];
  const major = first >> 5;
  const ai = first & 0x1f;
  let value = 0;
  let size = 1;
  if (ai < 24) {
    value = ai;
  } else if (ai === 24) {
    value = data[offset + 1];
    size = 2;
  } else if (ai === 25) {
    value = (data[offset + 1] << 8) | data[offset + 2];
    size = 3;
  } else if (ai === 26) {
    value =
      (data[offset + 1] << 24) |
      (data[offset + 2] << 16) |
      (data[offset + 3] << 8) |
      data[offset + 4];
    size = 5;
  } else if (ai === 27) {
    const big =
      (BigInt(data[offset + 1]) << 56n) |
      (BigInt(data[offset + 2]) << 48n) |
      (BigInt(data[offset + 3]) << 40n) |
      (BigInt(data[offset + 4]) << 32n) |
      (BigInt(data[offset + 5]) << 24n) |
      (BigInt(data[offset + 6]) << 16n) |
      (BigInt(data[offset + 7]) << 8n) |
      BigInt(data[offset + 8]);
    if (big > BigInt(Number.MAX_SAFE_INTEGER)) throw new Error("uint too large");
    value = Number(big);
    size = 9;
  } else {
    throw new Error("unsupported uint size");
  }
  return { major, value, size };
}

function decodeItem(data: Uint8Array, offset: number): { value: CborValue; size: number } {
  const { major, value, size } = decodeUnsigned(data, offset);
  const start = offset + size;
  if (major === 0) {
    return { value: { type: "uint", value }, size };
  }
  if (major === 4) {
    let cursor = start;
    const items: CborValue[] = [];
    for (let i = 0; i < value; i += 1) {
      const item = decodeItem(data, cursor);
      cursor += item.size;
      items.push(item.value);
    }
    return { value: { type: "array", value: items }, size: cursor - offset };
  }
  if (major === 2) {
    const bytes = data.slice(start, start + value);
    return { value: { type: "bytes", value: bytes }, size: size + value };
  }
  if (major === 3) {
    const bytes = data.slice(start, start + value);
    const text = Buffer.from(bytes).toString("utf8");
    return { value: { type: "text", value: text }, size: size + value };
  }
  if (major === 5) {
    let cursor = start;
    const map: CborMap = {};
    for (let i = 0; i < value; i += 1) {
      const keyItem = decodeItem(data, cursor);
      cursor += keyItem.size;
      if (keyItem.value.type !== "text") throw new Error("map key not text");
      const valItem = decodeItem(data, cursor);
      cursor += valItem.size;
      map[keyItem.value.value] = valItem.value;
    }
    return { value: { type: "map", value: map }, size: cursor - offset };
  }
  throw new Error("unsupported cbor type");
}

export function validateContextPayload(ctxType: string, payload: Uint8Array): void {
  const { value } = decodeItem(payload, 0);
  if (value.type !== "map") throw new Error("invalid payload type");
  const map = value.value;
  if (ctxType === "proximity") {
    if (!map.method || !map.nonce || !map.timestamp) throw new Error("missing field");
    if (map.method.type !== "text" || (map.method.value !== "nfc" && map.method.value !== "ble"))
      throw new Error("invalid method");
    if (map.nonce.type !== "bytes" || map.nonce.value.length !== 16)
      throw new Error("invalid nonce");
    if (map.timestamp.type !== "uint") throw new Error("invalid timestamp");
    return;
  }
  if (ctxType === "beacon") {
    if (!map.beacon_id || !map.token || !map.timestamp) throw new Error("missing field");
    if (map.beacon_id.type !== "bytes" || map.beacon_id.value.length !== 32)
      throw new Error("invalid beacon_id");
    if (map.token.type !== "bytes" || map.token.value.length === 0)
      throw new Error("invalid token");
    if (map.timestamp.type !== "uint") throw new Error("invalid timestamp");
    return;
  }
  if (ctxType === "place") {
    if (!map.place_hash || !map.timestamp) throw new Error("missing field");
    if (map.place_hash.type !== "bytes" || map.place_hash.value.length !== 32)
      throw new Error("invalid place_hash");
    if (map.timestamp.type !== "uint") throw new Error("invalid timestamp");
    return;
  }
  if (ctxType === "vouch") {
    if (!map.subject || !map.community || !map.timestamp) throw new Error("missing field");
    if (map.subject.type !== "text" || !map.subject.value.startsWith("did:comum:"))
      throw new Error("invalid subject");
    if (map.community.type !== "bytes" || map.community.value.length !== 32)
      throw new Error("invalid community");
    if (map.timestamp.type !== "uint") throw new Error("invalid timestamp");
    return;
  }
  throw new Error("invalid context type");
}

export function validateReceivePayload(payload: Uint8Array): void {
  const { value } = decodeItem(payload, 0);
  if (value.type !== "map") throw new Error("invalid payload type");
  const map = value.value;
  if (!map.of || !map.timestamp) throw new Error("missing field");
  if (map.of.type !== "bytes" || map.of.value.length !== 32)
    throw new Error("invalid of");
  if (map.timestamp.type !== "uint") throw new Error("invalid timestamp");
}

export function validateGenesisPayload(payload: Uint8Array): void {
  const { value } = decodeItem(payload, 0);
  if (value.type !== "map") throw new Error("invalid payload type");
  const map = value.value;
  if (!map.name || !map.threshold || !map.founders || !map.capsules || !map.supply || !map.mint_policy)
    throw new Error("missing field");

  if (map.name.type !== "text" || map.name.value.length === 0)
    throw new Error("invalid name");

  if (map.founders.type !== "array") throw new Error("invalid founders");
  if (map.founders.value.length < 3) throw new Error("invalid founders");
  for (const item of map.founders.value) {
    if (item.type !== "text" || !item.value.startsWith("did:comum:"))
      throw new Error("invalid founder");
  }

  if (map.threshold.type !== "uint") throw new Error("invalid threshold");
  if (map.threshold.value === 0 || map.threshold.value > map.founders.value.length)
    throw new Error("invalid threshold");

  if (map.capsules.type !== "array") throw new Error("invalid capsules");
  for (const item of map.capsules.value) {
    if (item.type !== "bytes" || item.value.length !== 32)
      throw new Error("invalid capsule");
  }

  if (map.supply.type !== "uint") throw new Error("invalid supply");
  if (map.mint_policy.type !== "bytes" || map.mint_policy.value.length !== 32)
    throw new Error("invalid mint_policy");
}
