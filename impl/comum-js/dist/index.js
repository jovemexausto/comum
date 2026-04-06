import { spawnSync } from "node:child_process";
import bs58 from "bs58";
import { sha3_256 } from "@noble/hashes/sha3";
import { sha256 } from "@noble/hashes/sha2";
import { hkdf } from "@noble/hashes/hkdf";
import { hmac } from "@noble/hashes/hmac";
export function encodeTestimony(testimonyWithoutId, options = {}) {
    const bin = options.bin || process.env.COMUM_RS_BIN || "comum-cbor";
    const input = JSON.stringify(testimonyWithoutId);
    const res = spawnSync(bin, [], { input, encoding: "utf8" });
    if (res.error)
        throw res.error;
    if (res.status !== 0)
        throw new Error(res.stderr || "comum-cbor failed");
    return JSON.parse(res.stdout);
}
export function verifyTestimony(testimonyWithoutId, expectedId, options = {}) {
    const out = encodeTestimony(testimonyWithoutId, options);
    return out.id === expectedId;
}
export function deriveDid(pk) {
    if (pk.length !== 32)
        throw new Error("invalid pk length");
    const digest = sha3_256(pk);
    const short = digest.slice(0, 20);
    const checksum = sha256(sha256(short)).slice(0, 4);
    const data = new Uint8Array(short.length + checksum.length);
    data.set(short, 0);
    data.set(checksum, short.length);
    const b58 = bs58.encode(data);
    return `did:comum:${b58}`;
}
export function computeNullifier(sk, testimonyId) {
    if (sk.length !== 32)
        throw new Error("invalid sk length");
    if (testimonyId.length !== 32)
        throw new Error("invalid id length");
    const info = new TextEncoder().encode("comum-nullifier-v1");
    const key = hkdf(sha3_256, sk, new Uint8Array(), info, 32);
    return hmac(sha3_256, key, testimonyId);
}
function concatBytes(chunks) {
    let total = 0;
    for (const chunk of chunks)
        total += chunk.length;
    const out = new Uint8Array(total);
    let offset = 0;
    for (const chunk of chunks) {
        out.set(chunk, offset);
        offset += chunk.length;
    }
    return out;
}
function encodeUnsignedHeader(major, len) {
    if (len < 24)
        return new Uint8Array([major | len]);
    if (len < 256)
        return new Uint8Array([major | 24, len]);
    if (len < 65536)
        return new Uint8Array([major | 25, (len >> 8) & 0xff, len & 0xff]);
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
function encodeUint(n) {
    if (!Number.isSafeInteger(n) || n < 0)
        throw new Error("invalid uint");
    if (n < 24)
        return new Uint8Array([n]);
    if (n < 256)
        return new Uint8Array([0x18, n]);
    if (n < 65536)
        return new Uint8Array([0x19, (n >> 8) & 0xff, n & 0xff]);
    if (n < 4294967296)
        return new Uint8Array([
            0x1a,
            (n >> 24) & 0xff,
            (n >> 16) & 0xff,
            (n >> 8) & 0xff,
            n & 0xff,
        ]);
    const big = BigInt(n);
    if (big > BigInt(Number.MAX_SAFE_INTEGER))
        throw new Error("uint too large");
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
function encodeBstr(data) {
    const header = encodeUnsignedHeader(0x40, data.length);
    return concatBytes([header, data]);
}
function encodeTstr(text) {
    const data = Buffer.from(text, "utf8");
    const header = encodeUnsignedHeader(0x60, data.length);
    return concatBytes([header, data]);
}
function encodeMap(pairs) {
    const header = encodeUnsignedHeader(0xa0, pairs.length);
    return concatBytes([header, ...pairs]);
}
export function buildProximityContextPayload(method, nonce, timestamp) {
    const pairs = [
        concatBytes([encodeTstr("nonce"), encodeBstr(nonce)]),
        concatBytes([encodeTstr("method"), encodeTstr(method)]),
        concatBytes([encodeTstr("timestamp"), encodeUint(timestamp)]),
    ];
    return encodeMap(pairs);
}
export function buildBeaconContextPayload(beaconId, token, timestamp) {
    const pairs = [
        concatBytes([encodeTstr("token"), encodeBstr(token)]),
        concatBytes([encodeTstr("beacon_id"), encodeBstr(beaconId)]),
        concatBytes([encodeTstr("timestamp"), encodeUint(timestamp)]),
    ];
    return encodeMap(pairs);
}
export function buildPlaceContextPayload(placeHash, timestamp) {
    const pairs = [
        concatBytes([encodeTstr("timestamp"), encodeUint(timestamp)]),
        concatBytes([encodeTstr("place_hash"), encodeBstr(placeHash)]),
    ];
    return encodeMap(pairs);
}
export function buildVouchContextPayload(subject, community, timestamp) {
    const pairs = [
        concatBytes([encodeTstr("subject"), encodeTstr(subject)]),
        concatBytes([encodeTstr("community"), encodeBstr(community)]),
        concatBytes([encodeTstr("timestamp"), encodeUint(timestamp)]),
    ];
    return encodeMap(pairs);
}
export function buildReceivePayload(of, timestamp) {
    const pairs = [
        concatBytes([encodeTstr("of"), encodeBstr(of)]),
        concatBytes([encodeTstr("timestamp"), encodeUint(timestamp)]),
    ];
    return encodeMap(pairs);
}
function decodeUnsigned(data, offset) {
    const first = data[offset];
    const major = first >> 5;
    const ai = first & 0x1f;
    let value = 0;
    let size = 1;
    if (ai < 24) {
        value = ai;
    }
    else if (ai === 24) {
        value = data[offset + 1];
        size = 2;
    }
    else if (ai === 25) {
        value = (data[offset + 1] << 8) | data[offset + 2];
        size = 3;
    }
    else if (ai === 26) {
        value =
            (data[offset + 1] << 24) |
                (data[offset + 2] << 16) |
                (data[offset + 3] << 8) |
                data[offset + 4];
        size = 5;
    }
    else if (ai === 27) {
        const big = (BigInt(data[offset + 1]) << 56n) |
            (BigInt(data[offset + 2]) << 48n) |
            (BigInt(data[offset + 3]) << 40n) |
            (BigInt(data[offset + 4]) << 32n) |
            (BigInt(data[offset + 5]) << 24n) |
            (BigInt(data[offset + 6]) << 16n) |
            (BigInt(data[offset + 7]) << 8n) |
            BigInt(data[offset + 8]);
        if (big > BigInt(Number.MAX_SAFE_INTEGER))
            throw new Error("uint too large");
        value = Number(big);
        size = 9;
    }
    else {
        throw new Error("unsupported uint size");
    }
    return { major, value, size };
}
function decodeItem(data, offset) {
    const { major, value, size } = decodeUnsigned(data, offset);
    const start = offset + size;
    if (major === 0) {
        return { value: { type: "uint", value }, size };
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
        const map = {};
        for (let i = 0; i < value; i += 1) {
            const keyItem = decodeItem(data, cursor);
            cursor += keyItem.size;
            if (keyItem.value.type !== "text")
                throw new Error("map key not text");
            const valItem = decodeItem(data, cursor);
            cursor += valItem.size;
            map[keyItem.value.value] = valItem.value;
        }
        return { value: { type: "map", value: map }, size: cursor - offset };
    }
    throw new Error("unsupported cbor type");
}
export function validateContextPayload(ctxType, payload) {
    const { value } = decodeItem(payload, 0);
    if (value.type !== "map")
        throw new Error("invalid payload type");
    const map = value.value;
    if (ctxType === "proximity") {
        if (!map.method || !map.nonce || !map.timestamp)
            throw new Error("missing field");
        if (map.method.type !== "text" || (map.method.value !== "nfc" && map.method.value !== "ble"))
            throw new Error("invalid method");
        if (map.nonce.type !== "bytes" || map.nonce.value.length !== 16)
            throw new Error("invalid nonce");
        if (map.timestamp.type !== "uint")
            throw new Error("invalid timestamp");
        return;
    }
    if (ctxType === "beacon") {
        if (!map.beacon_id || !map.token || !map.timestamp)
            throw new Error("missing field");
        if (map.beacon_id.type !== "bytes" || map.beacon_id.value.length !== 32)
            throw new Error("invalid beacon_id");
        if (map.token.type !== "bytes" || map.token.value.length === 0)
            throw new Error("invalid token");
        if (map.timestamp.type !== "uint")
            throw new Error("invalid timestamp");
        return;
    }
    if (ctxType === "place") {
        if (!map.place_hash || !map.timestamp)
            throw new Error("missing field");
        if (map.place_hash.type !== "bytes" || map.place_hash.value.length !== 32)
            throw new Error("invalid place_hash");
        if (map.timestamp.type !== "uint")
            throw new Error("invalid timestamp");
        return;
    }
    if (ctxType === "vouch") {
        if (!map.subject || !map.community || !map.timestamp)
            throw new Error("missing field");
        if (map.subject.type !== "text" || !map.subject.value.startsWith("did:comum:"))
            throw new Error("invalid subject");
        if (map.community.type !== "bytes" || map.community.value.length !== 32)
            throw new Error("invalid community");
        if (map.timestamp.type !== "uint")
            throw new Error("invalid timestamp");
        return;
    }
    throw new Error("invalid context type");
}
export function validateReceivePayload(payload) {
    const { value } = decodeItem(payload, 0);
    if (value.type !== "map")
        throw new Error("invalid payload type");
    const map = value.value;
    if (!map.of || !map.timestamp)
        throw new Error("missing field");
    if (map.of.type !== "bytes" || map.of.value.length !== 32)
        throw new Error("invalid of");
    if (map.timestamp.type !== "uint")
        throw new Error("invalid timestamp");
}
