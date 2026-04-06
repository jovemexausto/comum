export type EncodeResult = {
    cbor_hex: string;
    id: string;
};
export type EncodeOptions = {
    bin?: string;
};
type NativeModule = {
    encode_testimony?: (json: string) => string;
    Commoner?: new (sk: Buffer, suite: number) => NativeCommoner;
};
type NativeCommoner = {
    did(): string;
    clock(): number;
    register_pk(pk: Buffer): Buffer;
    add_supported_suite(suite: number): void;
    validate(testimony_cbor: Buffer): void;
    ingest(testimony_cbor: Buffer): void;
    emit(verb: string, payload_cbor: Buffer, context: NativeContextInput): NativeEmitResult;
    build_hello(profile: string): Buffer;
    build_request(clock: number, limit: number): Buffer;
    apply_response(payload: Buffer): void;
    encode_cte(payload: Buffer): Buffer;
    fragment_cte(cte: Buffer, mtu: number, frag_id: Buffer): NativeCteFragment[];
    reassemble(fragments: NativeCteFragment[]): Buffer;
};
type NativeContextInput = {
    type: string;
    payload_cbor: Buffer;
    proof: NativeProofInput;
};
type NativeProofInput = {
    version: number;
    signatures: Buffer[];
    zk_proofs: Buffer[];
    nullifiers: Buffer[];
};
type NativeEmitResult = {
    id_hex: string;
    cbor: Buffer;
};
type NativeCteFragment = {
    frag_id: Buffer;
    frag_index: number;
    frag_total: number;
    frag_payload: Buffer;
};
export declare function loadNative(): NativeModule | null;
export declare function encodeTestimony(testimonyWithoutId: unknown, options?: EncodeOptions): EncodeResult;
export declare function verifyTestimony(testimonyWithoutId: unknown, expectedId: string, options?: EncodeOptions): boolean;
export type CommonerContextInput = {
    type: string;
    payload_cbor: Uint8Array;
    proof: CommonerProofInput;
};
export type CommonerProofInput = {
    version: number;
    signatures: Uint8Array[];
    zk_proofs: Uint8Array[];
    nullifiers: Uint8Array[];
};
export type CommonerEmitResult = {
    id_hex: string;
    cbor: Uint8Array;
};
export type CteFragment = {
    frag_id: Uint8Array;
    frag_index: number;
    frag_total: number;
    frag_payload: Uint8Array;
};
export declare class Commoner {
    private native;
    constructor(sk: Uint8Array, suite: number);
    did(): string;
    clock(): number;
    registerPk(pk: Uint8Array): Uint8Array;
    addSupportedSuite(suite: number): void;
    validate(testimonyCbor: Uint8Array): void;
    ingest(testimonyCbor: Uint8Array): void;
    emit(verb: string, payloadCbor: Uint8Array, context: CommonerContextInput): CommonerEmitResult;
    buildHello(profile: string): Uint8Array;
    buildRequest(clock: number, limit: number): Uint8Array;
    applyResponse(payload: Uint8Array): void;
    encodeCte(payload: Uint8Array): Uint8Array;
    fragmentCte(cte: Uint8Array, mtu: number, fragId: Uint8Array): CteFragment[];
    reassemble(fragments: CteFragment[]): Uint8Array;
}
export declare function deriveDid(pk: Uint8Array): string;
export declare function computeNullifier(sk: Uint8Array, testimonyId: Uint8Array): Uint8Array;
export declare function buildProximityContextPayload(method: "nfc" | "ble", nonce: Uint8Array, timestamp: number): Uint8Array;
export declare function buildBeaconContextPayload(beaconId: Uint8Array, token: Uint8Array, timestamp: number): Uint8Array;
export declare function buildPlaceContextPayload(placeHash: Uint8Array, timestamp: number): Uint8Array;
export declare function buildVouchContextPayload(subject: string, community: Uint8Array, timestamp: number): Uint8Array;
export declare function buildReceivePayload(of: Uint8Array, timestamp: number): Uint8Array;
export declare function buildGenesisPayload(name: string, threshold: number, founders: string[], capsules: Uint8Array[], supply: number, mintPolicy: Uint8Array): Uint8Array;
export declare function validateContextPayload(ctxType: string, payload: Uint8Array): void;
export declare function validateReceivePayload(payload: Uint8Array): void;
export declare function validateGenesisPayload(payload: Uint8Array): void;
export {};
