#!/usr/bin/env node
/*
 * Conformance runner for CIP-0001 test vectors.
 * Validates CBOR canonical encoding and SHA3-256 id.
 */

const fs = require("fs");
const path = require("path");
const crypto = require("crypto");

function encodeUint(n) {
  if (n < 0) throw new Error("negative int not supported");
  if (n < 24) return Buffer.from([n]);
  if (n < 0x100) return Buffer.from([0x18, n]);
  if (n < 0x10000) return Buffer.from([0x19, (n >> 8) & 0xff, n & 0xff]);
  if (n < 0x100000000) {
    return Buffer.from([
      0x1a,
      (n >>> 24) & 0xff,
      (n >>> 16) & 0xff,
      (n >>> 8) & 0xff,
      n & 0xff,
    ]);
  }
  const hi = Math.floor(n / 0x100000000);
  const lo = n >>> 0;
  return Buffer.from([
    0x1b,
    (hi >>> 24) & 0xff,
    (hi >>> 16) & 0xff,
    (hi >>> 8) & 0xff,
    hi & 0xff,
    (lo >>> 24) & 0xff,
    (lo >>> 16) & 0xff,
    (lo >>> 8) & 0xff,
    lo & 0xff,
  ]);
}

function encodeBytes(buf) {
  const len = buf.length;
  if (len < 24) return Buffer.concat([Buffer.from([0x40 + len]), buf]);
  if (len < 0x100) return Buffer.concat([Buffer.from([0x58, len]), buf]);
  if (len < 0x10000)
    return Buffer.concat([
      Buffer.from([0x59, (len >> 8) & 0xff, len & 0xff]),
      buf,
    ]);
  throw new Error("bstr too long");
}

function encodeText(str) {
  const buf = Buffer.from(str, "utf8");
  const len = buf.length;
  if (len < 24) return Buffer.concat([Buffer.from([0x60 + len]), buf]);
  if (len < 0x100) return Buffer.concat([Buffer.from([0x78, len]), buf]);
  if (len < 0x10000)
    return Buffer.concat([
      Buffer.from([0x79, (len >> 8) & 0xff, len & 0xff]),
      buf,
    ]);
  throw new Error("tstr too long");
}

function encodeArray(items) {
  const len = items.length;
  let head;
  if (len < 24) head = Buffer.from([0x80 + len]);
  else if (len < 0x100) head = Buffer.from([0x98, len]);
  else if (len < 0x10000)
    head = Buffer.from([0x99, (len >> 8) & 0xff, len & 0xff]);
  else throw new Error("array too long");
  return Buffer.concat([head, ...items]);
}

function encodeMap(pairs) {
  const len = pairs.length;
  let head;
  if (len < 24) head = Buffer.from([0xa0 + len]);
  else if (len < 0x100) head = Buffer.from([0xb8, len]);
  else if (len < 0x10000)
    head = Buffer.from([0xb9, (len >> 8) & 0xff, len & 0xff]);
  else throw new Error("map too long");
  return Buffer.concat([head, ...pairs]);
}

function hexToBuf(hex) {
  return Buffer.from(hex, "hex");
}

function encodeProof(proof) {
  const pairs = [];
  pairs.push(Buffer.concat([encodeUint(0), encodeUint(proof.version)]));
  const sigs = proof.signatures.map(hexToBuf).map(encodeBytes);
  pairs.push(Buffer.concat([encodeUint(1), encodeArray(sigs)]));
  const zks = proof.zk_proofs.map(hexToBuf).map(encodeBytes);
  pairs.push(Buffer.concat([encodeUint(2), encodeArray(zks)]));
  const nulls = proof.nullifiers.map(hexToBuf).map(encodeBytes);
  pairs.push(Buffer.concat([encodeUint(3), encodeArray(nulls)]));
  return encodeMap(pairs);
}

function encodeClaim(claim) {
  const pairs = [];
  pairs.push(Buffer.concat([encodeUint(0), encodeText(claim.verb)]));
  pairs.push(
    Buffer.concat([
      encodeUint(1),
      encodeBytes(hexToBuf(claim.payload_cbor_hex)),
    ])
  );
  return encodeMap(pairs);
}

function encodeContext(ctx) {
  const pairs = [];
  pairs.push(Buffer.concat([encodeUint(0), encodeText(ctx.type)]));
  pairs.push(
    Buffer.concat([
      encodeUint(1),
      encodeBytes(hexToBuf(ctx.payload_cbor_hex)),
    ])
  );
  pairs.push(Buffer.concat([encodeUint(2), encodeProof(ctx.proof)]));
  return encodeMap(pairs);
}

function encodeTestimonyWithoutId(t) {
  const pairs = [];
  pairs.push(Buffer.concat([encodeUint(0), encodeUint(t.version)]));
  if (t.author) {
    pairs.push(Buffer.concat([encodeUint(2), encodeBytes(hexToBuf(t.author))]));
  }
  pairs.push(Buffer.concat([encodeUint(3), encodeUint(t.timestamp)]));
  pairs.push(Buffer.concat([encodeUint(4), encodeUint(t.suite)]));
  if (t.prev_id) {
    pairs.push(Buffer.concat([encodeUint(5), encodeBytes(hexToBuf(t.prev_id))]));
  }
  const refs = t.refs.map(hexToBuf).map(encodeBytes);
  pairs.push(Buffer.concat([encodeUint(6), encodeArray(refs)]));
  pairs.push(Buffer.concat([encodeUint(7), encodeClaim(t.claim)]));
  pairs.push(Buffer.concat([encodeUint(8), encodeContext(t.context)]));
  pairs.push(Buffer.concat([encodeUint(9), encodeProof(t.proof)]));

  // Keys already inserted in ascending order.
  return encodeMap(pairs);
}

function sha3Hex(buf) {
  return crypto.createHash("sha3-256").update(buf).digest("hex");
}

function run() {
  const manifestPath = path.join(__dirname, "..", "..", "spec", "test-vectors", "manifest.json");
  const manifest = JSON.parse(fs.readFileSync(manifestPath, "utf8"));
  let failures = 0;

  for (const name of manifest.vectors) {
    const vecPath = path.join(__dirname, "..", "..", "spec", "test-vectors", name);
    const vector = JSON.parse(fs.readFileSync(vecPath, "utf8"));
    const t = vector.testimony_without_id;
    const cbor = encodeTestimonyWithoutId(t);
    const cborHex = cbor.toString("hex");
    if (vector.testimony_without_id_cbor_hex) {
      if (cborHex !== vector.testimony_without_id_cbor_hex) {
        console.error(`[${name}] cbor_hex mismatch`);
        failures += 1;
      }
    }
    const id = sha3Hex(cbor);
    if (id !== vector.expected_id) {
      console.error(`[${name}] expected_id mismatch`);
      failures += 1;
    }
    if (failures === 0) {
      console.log(`[${name}] ok`);
    }
  }

  if (failures > 0) process.exit(1);
}

run();
