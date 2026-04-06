import fs from "node:fs";
import path from "node:path";
import { encodeTestimony } from "../index.js";
function loadJson(p) {
    return JSON.parse(fs.readFileSync(p, "utf8"));
}
const repoRoot = path.resolve(import.meta.dirname, "..", "..", "..", "..");
const manifestPath = path.join(repoRoot, "spec", "test-vectors", "manifest.json");
const manifest = loadJson(manifestPath);
let failures = 0;
for (const name of manifest.vectors) {
    const vecPath = path.join(repoRoot, "spec", "test-vectors", name);
    const vector = loadJson(vecPath);
    const out = encodeTestimony(vector.testimony_without_id, {
        bin: path.join(repoRoot, "impl", "comum-rs", "target", "debug", "comum-cbor"),
    });
    if (out.id !== vector.expected_id) {
        console.error(`[${name}] expected_id mismatch`);
        failures += 1;
    }
    if (vector.testimony_without_id_cbor_hex) {
        if (out.cbor_hex !== vector.testimony_without_id_cbor_hex) {
            console.error(`[${name}] cbor_hex mismatch`);
            failures += 1;
        }
    }
}
if (failures > 0)
    process.exit(1);
console.log("comum-js vectors ok");
