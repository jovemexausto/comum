export type EncodeResult = {
    cbor_hex: string;
    id: string;
};
export type EncodeOptions = {
    bin?: string;
};
export declare function encodeTestimony(testimonyWithoutId: unknown, options?: EncodeOptions): EncodeResult;
export declare function verifyTestimony(testimonyWithoutId: unknown, expectedId: string, options?: EncodeOptions): boolean;
export declare function buildProximityContextPayload(method: "nfc" | "ble", nonce: Uint8Array, timestamp: number): Uint8Array;
export declare function buildBeaconContextPayload(beaconId: Uint8Array, token: Uint8Array, timestamp: number): Uint8Array;
export declare function buildPlaceContextPayload(placeHash: Uint8Array, timestamp: number): Uint8Array;
export declare function buildVouchContextPayload(subject: string, community: Uint8Array, timestamp: number): Uint8Array;
export declare function validateContextPayload(ctxType: string, payload: Uint8Array): void;
