import assert from "node:assert";
import {
  buildGenesisPayload,
  validateGenesisPayload,
} from "../index.js";

const founders = ["did:comum:alpha", "did:comum:bravo", "did:comum:charlie"];
const capsules = [new Uint8Array(32).fill(0x11), new Uint8Array(32).fill(0x22)];
const mintPolicy = new Uint8Array(32).fill(0x33);
const payload = buildGenesisPayload("Feira da Se", 2, founders, capsules, 0, mintPolicy);
validateGenesisPayload(payload);

const foundersPair = ["did:comum:alpha", "did:comum:bravo"];
const pairPayload = buildGenesisPayload("Mutirao do Bairro", 2, foundersPair, capsules, 0, mintPolicy);
validateGenesisPayload(pairPayload);

const badFounders = buildGenesisPayload("Feira da Se", 1, ["did:comum:only"], capsules, 0, mintPolicy);
assert.throws(() => validateGenesisPayload(badFounders));

const badPairThreshold = buildGenesisPayload("Mutirao do Bairro", 1, foundersPair, capsules, 0, mintPolicy);
assert.throws(() => validateGenesisPayload(badPairThreshold));

const badThreshold = buildGenesisPayload("Feira da Se", 4, founders, capsules, 0, mintPolicy);
assert.throws(() => validateGenesisPayload(badThreshold));

const badCapsules = buildGenesisPayload(
  "Feira da Se",
  2,
  founders,
  [new Uint8Array(31).fill(0x11)],
  0,
  mintPolicy
);
assert.throws(() => validateGenesisPayload(badCapsules));

const badMintPolicy = buildGenesisPayload(
  "Feira da Se",
  2,
  founders,
  capsules,
  0,
  new Uint8Array(31).fill(0x33)
);
assert.throws(() => validateGenesisPayload(badMintPolicy));

console.log("comum-js genesis ok");
