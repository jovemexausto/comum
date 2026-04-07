import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { sha3_256 } from "@noble/hashes/sha3";

import { Commoner } from "../index";
import {
  accept,
  buildAcceptPayload,
  buildOfferPayload,
  buildReceiptPayload,
  computeOfferId,
  offer,
  receipt,
} from "../feira";

function readCapsuleId(): Uint8Array {
  const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../../..");
  const wasmPath = resolve(root, "impl/capsulas/feira/feira.wasm");
  const wasm = readFileSync(wasmPath);
  return sha3_256(new Uint8Array(wasm));
}

function emptyContext() {
  return {
    type: "none",
    payload_cbor: new Uint8Array([0xa0]),
    proof: { version: 1, signatures: [], zk_proofs: [], nullifiers: [] },
  };
}

function main() {
  const capsuleId = readCapsuleId();
  const ctx = emptyContext();

  const seller = new Commoner(new Uint8Array(32).fill(1), 1);
  const buyer = new Commoner(new Uint8Array(32).fill(2), 1);

  const item = "cafe";
  const price = 5;
  const currency = "comum";
  const expires = 1_700_000_200_000;

  const offerParams = buildOfferPayload(item, price, currency, expires, seller.did());
  const offerId = computeOfferId(item, price, currency, expires, seller.did());
  const acceptParams = buildAcceptPayload(offerId, buyer.did());
  const receiptParams = buildReceiptPayload(offerId, 1_700_000_210_000);

  const tOffer = offer(seller, capsuleId, offerParams, ctx);
  buyer.ingest(tOffer.cbor);

  const tAccept = accept(buyer, capsuleId, acceptParams, ctx);
  seller.ingest(tAccept.cbor);

  const tReceipt = receipt(buyer, capsuleId, receiptParams, ctx);
  seller.ingest(tReceipt.cbor);

  console.log("E2E flow complete");
  console.log({ offer: tOffer.id_hex, accept: tAccept.id_hex, receipt: tReceipt.id_hex });
}

main();
