import { spawnSync } from "node:child_process";

export type EncodeResult = {
  cbor_hex: string;
  id: string;
};

export type EncodeOptions = {
  bin?: string;
};

export function encodeTestimony(
  testimonyWithoutId: unknown,
  options: EncodeOptions = {}
): EncodeResult {
  const bin = options.bin || process.env.COMUM_RS_BIN || "comum-cbor";
  const input = JSON.stringify(testimonyWithoutId);
  const res = spawnSync(bin, [], { input, encoding: "utf8" });
  if (res.error) throw res.error;
  if (res.status !== 0) throw new Error(res.stderr || "comum-cbor failed");
  return JSON.parse(res.stdout) as EncodeResult;
}

export function verifyTestimony(
  testimonyWithoutId: unknown,
  expectedId: string,
  options: EncodeOptions = {}
): boolean {
  const out = encodeTestimony(testimonyWithoutId, options);
  return out.id === expectedId;
}
