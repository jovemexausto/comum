export type EncodeResult = {
    cbor_hex: string;
    id: string;
};
export type EncodeOptions = {
    bin?: string;
};
export declare function encodeTestimony(testimonyWithoutId: unknown, options?: EncodeOptions): EncodeResult;
export declare function verifyTestimony(testimonyWithoutId: unknown, expectedId: string, options?: EncodeOptions): boolean;
