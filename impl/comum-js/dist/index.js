import { spawnSync } from "node:child_process";
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
