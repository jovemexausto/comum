import assert from "node:assert";
import {
  buildBeaconContextPayload,
  buildPlaceContextPayload,
  buildProximityContextPayload,
  buildVouchContextPayload,
  validateContextPayload,
} from "../index.js";

function toHex(data: Uint8Array): string {
  return Buffer.from(data).toString("hex");
}

const nonce = new Uint8Array(16).fill(0xab);
const proximity = buildProximityContextPayload("nfc", nonce, 123);
const proximityHex =
  "a3" +
  "656e6f6e6365" +
  "50" +
  "ab".repeat(16) +
  "666d6574686f64" +
  "636e6663" +
  "6974696d657374616d70" +
  "187b";
assert.equal(toHex(proximity), proximityHex);
validateContextPayload("proximity", proximity);

const beaconId = new Uint8Array(32).fill(0x11);
const token = new Uint8Array([0xaa, 0xbb, 0xcc]);
const beacon = buildBeaconContextPayload(beaconId, token, 456);
const beaconHex =
  "a3" +
  "65746f6b656e" +
  "43aabbcc" +
  "69626561636f6e5f6964" +
  "5820" +
  "11".repeat(32) +
  "6974696d657374616d70" +
  "1901c8";
assert.equal(toHex(beacon), beaconHex);
validateContextPayload("beacon", beacon);

const placeHash = new Uint8Array(32).fill(0x22);
const place = buildPlaceContextPayload(placeHash, 789);
validateContextPayload("place", place);

const subject = "did:comum:abc";
const community = new Uint8Array(32).fill(0x44);
const vouch = buildVouchContextPayload(subject, community, 987);
validateContextPayload("vouch", vouch);

const beaconBadToken = buildBeaconContextPayload(beaconId, new Uint8Array(), 456);
assert.throws(() => validateContextPayload("beacon", beaconBadToken));

const placeBadHash = buildPlaceContextPayload(new Uint8Array(31).fill(0x22), 789);
assert.throws(() => validateContextPayload("place", placeBadHash));

const vouchBadSubject = buildVouchContextPayload("did:wrong:abc", community, 987);
assert.throws(() => validateContextPayload("vouch", vouchBadSubject));

console.log("comum-js context ok");
