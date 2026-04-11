import assert from "node:assert";
import {
  buildGenesisPayload,
  validateGenesisPayload,
} from "../index.js";

const founders = ["did:comum:alpha", "did:comum:bravo", "did:comum:charlie"];
const capsules = [new Uint8Array(32).fill(0x11), new Uint8Array(32).fill(0x22)];
const mintPolicy = new Uint8Array(32).fill(0x33);
const payload = buildGenesisPayload("Comum Demo", 2, founders, capsules, 0, mintPolicy);
validateGenesisPayload(payload);

const foundersPair = ["did:comum:alpha", "did:comum:bravo"];
const pairPayload = buildGenesisPayload("Comum Par", 2, foundersPair, capsules, 0, mintPolicy);
validateGenesisPayload(pairPayload);

const badFounders = buildGenesisPayload("Comum Demo", 1, ["did:comum:only"], capsules, 0, mintPolicy);
assert.throws(() => validateGenesisPayload(badFounders));

const badPairThreshold = buildGenesisPayload("Comum Par", 1, foundersPair, capsules, 0, mintPolicy);
assert.throws(() => validateGenesisPayload(badPairThreshold));

const badThreshold = buildGenesisPayload("Comum Demo", 4, founders, capsules, 0, mintPolicy);
assert.throws(() => validateGenesisPayload(badThreshold));

const badCapsules = buildGenesisPayload(
  "Comum Demo",
  2,
  founders,
  [new Uint8Array(31).fill(0x11)],
  0,
  mintPolicy
);
assert.throws(() => validateGenesisPayload(badCapsules));

const badMintPolicy = buildGenesisPayload(
  "Comum Demo",
  2,
  founders,
  capsules,
  0,
  new Uint8Array(31).fill(0x33)
);
assert.throws(() => validateGenesisPayload(badMintPolicy));

console.log("comum-js genesis ok");
