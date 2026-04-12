import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { sha3_256 } from "@noble/hashes/sha3";
import * as ed25519 from "@noble/ed25519";

import { Commoner, loadNative } from "../index.js";
import {
  accept,
  buildAcceptPayload,
  buildOfferPayload,
  buildReceiptPayload,
  computeOfferId,
  offer,
  receipt,
} from "../feira.js";

function readCapsuleId(): Uint8Array {
  const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../..");
  const wasmPath = resolve(root, "apps/feira/capsules/feira.wasm");
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

async function main() {
  const native = loadNative();
  if (!native?.Commoner) {
    console.error("comum-napi not available; set COMUM_NAPI_PATH or install comum-napi");
    process.exit(1);
  }
  const capsuleId = readCapsuleId();
  const ctx = emptyContext();

  const sellerSk = new Uint8Array(32).fill(1);
  const buyerSk = new Uint8Array(32).fill(2);
  const seller = new Commoner(sellerSk, 1);
  const buyer = new Commoner(buyerSk, 1);

  const sellerPk = await ed25519.getPublicKeyAsync(sellerSk);
  const buyerPk = await ed25519.getPublicKeyAsync(buyerSk);
  seller.registerPk(buyerPk);
  buyer.registerPk(sellerPk);

  const item = "cafe";
  const price = 5;
  const currency = "feira-credito";
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

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
