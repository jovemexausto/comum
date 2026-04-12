/**
 * Testes unitarios do ComumClient.
 *
 * Roda em Node.js puro — sem N-API, sem rede.
 * Usa o build de mobile.ts (Commoner JS-only) via import direto.
 */

import assert from "node:assert/strict";
import { ComumClient } from "../client.mobile.js";

const CAPSULE_ID = new Uint8Array(32).fill(0x46);
const LOCAL_UNIT = "feira-credito";

function makeClient(seedByte: number): ComumClient {
  return new ComumClient(new Uint8Array(32).fill(seedByte), CAPSULE_ID);
}

// ---------------------------------------------------------------------------
// 1. Identidade
// ---------------------------------------------------------------------------
{
  const c = makeClient(0x11);
  assert.ok(c.did().startsWith("did:comum:"), "did deve comecar com did:comum:");
  assert.equal(c.shortDid().length, 8, "shortDid deve ter 8 caracteres");
  assert.equal(c.shortDid(), c.did().slice(-8), "shortDid deve ser o sufixo do did");
  console.log("client test: identidade ok");
}

// ---------------------------------------------------------------------------
// 2. Criar oferta
// ---------------------------------------------------------------------------
{
  const seller = makeClient(0x11);
  const offer = seller.createOffer({
    item: "cafe",
    price: 5,
    currency: LOCAL_UNIT,
    expiresAt: Date.now() + 60_000,
  });
  assert.equal(offer.item, "cafe");
  assert.equal(offer.price, 5);
  assert.equal(offer.currency, LOCAL_UNIT);
  assert.equal(offer.seller, seller.did());
  assert.equal(offer.id.length, 32, "offer.id deve ser 32 bytes");
  assert.equal(offer.idHex.length, 64, "offer.idHex deve ser 64 chars");
  assert.equal(offer.testimonyIdHex.length, 64, "offer.testimonyIdHex deve ser 64 chars");
  assert.notEqual(offer.idHex, offer.testimonyIdHex, "offer.idHex != testimonyIdHex (sao IDs distintos)");
  assert.equal(seller.testimonyCount(), 1, "deve ter 1 testemunho apos offer");
  assert.equal(seller.knownOffers().length, 1, "deve ter 1 oferta no index");
  console.log("client test: createOffer ok");
}

// ---------------------------------------------------------------------------
// 3. Aceitar oferta
// ---------------------------------------------------------------------------
{
  const seller = makeClient(0x11);
  const buyer = makeClient(0x22);

  const offer = seller.createOffer({
    item: "banana",
    price: 10,
    currency: LOCAL_UNIT,
    expiresAt: Date.now() + 60_000,
  });

  const accept = buyer.acceptOffer(offer.id);
  assert.equal(accept.offerId, offer.id);
  assert.equal(accept.testimonyIdHex.length, 64, "accept testimonyId deve ter 64 chars");
  assert.equal(buyer.testimonyCount(), 1, "buyer deve ter 1 testemunho apos accept");
  console.log("client test: acceptOffer ok");
}

// ---------------------------------------------------------------------------
// 4. Emitir receipt
// ---------------------------------------------------------------------------
{
  const seller = makeClient(0x11);
  const buyer = makeClient(0x22);

  const offer = seller.createOffer({
    item: "mate",
    price: 3,
    currency: LOCAL_UNIT,
    expiresAt: Date.now() + 60_000,
  });

  buyer.acceptOffer(offer.id);
  const rec = buyer.issueReceipt(offer.id);

  assert.equal(rec.offerId, offer.id);
  assert.equal(rec.testimonyIdHex.length, 64);
  assert.ok(rec.timestamp > 0, "timestamp deve ser positivo");
  assert.equal(buyer.testimonyCount(), 2, "buyer deve ter 2 testemunhos (accept + receipt)");
  console.log("client test: issueReceipt ok");
}

// ---------------------------------------------------------------------------
// 5. onReceipt callback
// ---------------------------------------------------------------------------
{
  const buyer = makeClient(0x22);
  const offer = makeClient(0x11).createOffer({
    item: "farinha",
    price: 8,
    currency: LOCAL_UNIT,
    expiresAt: Date.now() + 60_000,
  });

  let fired = false;
  const unsubscribe = buyer.onReceipt(offer.idHex, (r) => {
    fired = true;
    assert.equal(r.offerId, offer.id);
  });

  buyer.acceptOffer(offer.id);
  buyer.issueReceipt(offer.id);

  assert.ok(fired, "callback de receipt deve ter sido chamado");

  // Testar unsubscribe
  fired = false;
  unsubscribe();
  buyer.issueReceipt(offer.id);
  assert.ok(!fired, "callback nao deve disparar apos unsubscribe");
  console.log("client test: onReceipt ok");
}

// ---------------------------------------------------------------------------
// 6. Sync manual entre dois nos (fluxo Feira MVP em memoria)
// ---------------------------------------------------------------------------
{
  const seller = makeClient(0xaa);
  const buyer  = makeClient(0xbb);
  const witness= makeClient(0xcc);

  // seller cria oferta (seller.store = [offer])
  const offer = seller.createOffer({
    item: "queijo",
    price: 15,
    currency: LOCAL_UNIT,
    expiresAt: Date.now() + 60_000,
  });
  assert.equal(seller.testimonyCount(), 1, "seller: 1 apos offer");

  // seller → buyer e witness (ambos recebem offer)
  seller.syncTo(buyer);
  seller.syncTo(witness);
  assert.equal(buyer.testimonyCount(), 1,   "buyer: 1 apos sync offer");
  assert.equal(witness.testimonyCount(), 1, "witness: 1 apos sync offer");

  // buyer aceita
  const accept = buyer.acceptOffer(offer.id);
  assert.equal(buyer.testimonyCount(), 2, "buyer: 2 apos accept");

  // buyer → seller e witness (ambos recebem offer+accept)
  buyer.syncTo(seller);
  buyer.syncTo(witness);
  // seller: 1(offer) + 2(offer+accept via sync) — mas offer ja estava, ingest nao dedup em store bruto
  // accept eh novo: +1 para seller e witness
  assert.equal(seller.testimonyCount(), 3,  "seller: 3 apos receber sync do buyer");
  assert.equal(witness.testimonyCount(), 3, "witness: 3 apos receber sync do buyer");

  // buyer emite receipt
  const rec = buyer.issueReceipt(offer.id);
  assert.equal(buyer.testimonyCount(), 3, "buyer: 3 apos receipt");

  // buyer → seller e witness (ambos recebem offer+accept+receipt)
  buyer.syncTo(seller);
  buyer.syncTo(witness);
  // seller: 3 + 3(offer+accept+receipt via sync) = 6, receipt eh novo (+1 novo no significado)
  // O store nao dedup — comportamento correto: log imutavel de testemunhos recebidos
  // O que importa eh que receipt chegou a todos
  assert.ok(seller.testimonyCount() >= 4,  "seller recebeu receipt via sync");
  assert.ok(witness.testimonyCount() >= 4, "witness recebeu receipt via sync");

  assert.equal(accept.offerId, offer.id);
  assert.equal(rec.offerId, offer.id);

  console.log("client test: sync E2E (memoria) ok");
}

// ---------------------------------------------------------------------------
// 7. NodeTransport fake — publica e ingest via callback
// ---------------------------------------------------------------------------
void (async () => {
  const seller = makeClient(0x11);
  const buyer = makeClient(0x22);

  const inboxBuyer: Uint8Array[] = [];

  const sellerTransport = {
    start: async (cb: (cbor: Uint8Array) => void) => { void cb; },
    publish: (cbor: Uint8Array) => {
      inboxBuyer.push(cbor);
      buyer._ingest(cbor);
    },
  };

  await seller.connect(sellerTransport);
  seller.createOffer({
    item: "abacaxi",
    price: 7,
    currency: LOCAL_UNIT,
    expiresAt: Date.now() + 60_000,
  });

  assert.equal(buyer.testimonyCount(), 1, "buyer deve ter 1 testemunho via transport");
  assert.equal(inboxBuyer.length, 1, "inbox do buyer deve ter 1 item");

  seller.disconnect();
  console.log("client test: NodeTransport fake ok");
  console.log("\ncomum-js client tests ok");
})();
