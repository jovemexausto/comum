use std::fs;
use std::path::Path;

use capsula_feira::{
    build_accept_payload, build_dispute_payload, build_offer_payload, build_receipt_payload,
    compute_offer_id,
};
use comum_rs::{Commoner, ContextInput, ProofInput, CAPSULE_INVOKE};
use ed25519_dalek::SigningKey;
use sha3::{Digest, Sha3_256};

fn main() {
    let capsule_id = read_capsule_id("feira");

    let offer_id = compute_offer_id("cafe", 5, "comum", 1_700_000_200_000, "did:comum:s");
    let offer_params = build_offer_payload("cafe", 5, "comum", 1_700_000_200_000, "did:comum:s");
    let offer_invoke = build_invoke_payload(&capsule_id, "offer", &offer_params);

    let accept_params = build_accept_payload(&offer_id, "did:comum:b");
    let accept_invoke = build_invoke_payload(&capsule_id, "accept", &accept_params);

    let receipt_params = build_receipt_payload(&offer_id, 1_700_000_210_000);
    let receipt_invoke = build_invoke_payload(&capsule_id, "receipt", &receipt_params);

    let dispute_params = build_dispute_payload(&offer_id, "item divergente");
    let dispute_invoke = build_invoke_payload(&capsule_id, "dispute", &dispute_params);

    let mut seller = Commoner::new([0x11u8; 32], 1);
    let mut buyer = Commoner::new([0x22u8; 32], 1);
    let mut arbiter = Commoner::new([0x33u8; 32], 1);

    let pk_s = public_key_from_sk([0x11u8; 32]);
    let pk_b = public_key_from_sk([0x22u8; 32]);
    arbiter.register_pk(pk_s);
    arbiter.register_pk(pk_b);

    let ctx = default_context();
    let t_offer = seller.emit(CAPSULE_INVOKE, &offer_invoke, ctx.clone()).expect("offer");
    arbiter.ingest(&t_offer.cbor).expect("ingest offer");

    let t_accept = buyer.emit(CAPSULE_INVOKE, &accept_invoke, ctx.clone()).expect("accept");
    arbiter.ingest(&t_accept.cbor).expect("ingest accept");

    let t_receipt = buyer.emit(CAPSULE_INVOKE, &receipt_invoke, ctx.clone()).expect("receipt");
    arbiter.ingest(&t_receipt.cbor).expect("ingest receipt");

    let t_dispute = buyer.emit(CAPSULE_INVOKE, &dispute_invoke, ctx).expect("dispute");
    arbiter.ingest(&t_dispute.cbor).expect("ingest dispute");

    println!("[feira-sim] offer_id={}", to_hex(&offer_id));
    println!("[feira-sim] offer_id_testimony={}", t_offer.id_hex);
    println!("[feira-sim] accept_id={}", t_accept.id_hex);
    println!("[feira-sim] receipt_id={}", t_receipt.id_hex);
    println!("[feira-sim] dispute_id={}", t_dispute.id_hex);
}

fn default_context() -> ContextInput {
    ContextInput {
        r#type: "none".to_string(),
        payload_cbor: vec![0xa0],
        proof: ProofInput::default(),
    }
}

fn build_invoke_payload(capsule_id: &[u8; 32], action: &str, params: &[u8]) -> Vec<u8> {
    let mut pairs = Vec::new();
    // action(6) < params(6) < capsule_id(10)
    pairs.push([encode_tstr("action"), encode_tstr(action)].concat());
    pairs.push([encode_tstr("params"), encode_bstr(params)].concat());
    pairs.push([encode_tstr("capsule_id"), encode_bstr(capsule_id)].concat());
    encode_map(pairs)
}

fn read_capsule_id(name: &str) -> [u8; 32] {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let wasm_path = root.join(format!("capsules/{}/{}.wasm", name, name));
    let wasm = fs::read(wasm_path).expect("read capsule wasm");
    let mut hasher = Sha3_256::new();
    hasher.update(&wasm);
    let digest = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest[..32]);
    out
}

fn public_key_from_sk(sk: [u8; 32]) -> [u8; 32] {
    SigningKey::from_bytes(&sk).verifying_key().to_bytes()
}

fn encode_bstr(data: &[u8]) -> Vec<u8> {
    let len = data.len();
    if len < 24 {
        let mut out = vec![0x40 + len as u8];
        out.extend_from_slice(data);
        return out;
    }
    if len < 256 {
        let mut out = vec![0x58, len as u8];
        out.extend_from_slice(data);
        return out;
    }
    if len < 65536 {
        let mut out = vec![0x59, ((len >> 8) & 0xff) as u8, (len & 0xff) as u8];
        out.extend_from_slice(data);
        return out;
    }
    panic!("bstr too long");
}

fn encode_tstr(s: &str) -> Vec<u8> {
    let data = s.as_bytes();
    let len = data.len();
    if len < 24 {
        let mut out = vec![0x60 + len as u8];
        out.extend_from_slice(data);
        return out;
    }
    if len < 256 {
        let mut out = vec![0x78, len as u8];
        out.extend_from_slice(data);
        return out;
    }
    if len < 65536 {
        let mut out = vec![0x79, ((len >> 8) & 0xff) as u8, (len & 0xff) as u8];
        out.extend_from_slice(data);
        return out;
    }
    panic!("tstr too long");
}

fn encode_map(pairs: Vec<Vec<u8>>) -> Vec<u8> {
    let len = pairs.len();
    let mut out = if len < 24 {
        vec![0xa0 + len as u8]
    } else if len < 256 {
        vec![0xb8, len as u8]
    } else if len < 65536 {
        vec![0xb9, ((len >> 8) & 0xff) as u8, (len & 0xff) as u8]
    } else {
        panic!("map too long")
    };
    for pair in pairs {
        out.extend_from_slice(&pair);
    }
    out
}

fn to_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}
