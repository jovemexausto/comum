use std::fs;
use std::path::Path;

use capsula_feira::{build_accept_payload, build_offer_payload, build_receipt_payload, compute_offer_id};
use comum_rs::{Commoner, ContextInput, ProofInput, CAPSULE_INVOKE};
use ed25519_dalek::SigningKey;
use sha3::{Digest, Sha3_256};

fn main() {
    let capsule_id = read_capsule_id("feira");

    let mut seller = Commoner::new([0x11u8; 32], 1);
    let mut buyer = Commoner::new([0x22u8; 32], 1);
    let mut observer = Commoner::new([0x33u8; 32], 1);

    let pk_s = public_key_from_sk([0x11u8; 32]);
    let pk_b = public_key_from_sk([0x22u8; 32]);
    observer.register_pk(pk_s);
    observer.register_pk(pk_b);

    let ctx = default_context();

    let offer_id = compute_offer_id("cafe", 5, "comum", 1_700_000_200_000, "did:comum:s");
    let offer_params = build_offer_payload("cafe", 5, "comum", 1_700_000_200_000, "did:comum:s");
    let accept_params = build_accept_payload(&offer_id, "did:comum:b");
    let receipt_params = build_receipt_payload(&offer_id, 1_700_000_210_000);

    let offer_invoke = build_invoke_payload(&capsule_id, "offer", &offer_params);
    let accept_invoke = build_invoke_payload(&capsule_id, "accept", &accept_params);
    let receipt_invoke = build_invoke_payload(&capsule_id, "receipt", &receipt_params);

    let t_offer = seller.emit(CAPSULE_INVOKE, &offer_invoke, ctx.clone()).expect("offer");
    observer.ingest(&t_offer.cbor).expect("ingest offer");

    let t_accept = buyer.emit(CAPSULE_INVOKE, &accept_invoke, ctx.clone()).expect("accept");
    observer.ingest(&t_accept.cbor).expect("ingest accept");

    let t_receipt = buyer.emit(CAPSULE_INVOKE, &receipt_invoke, ctx.clone()).expect("receipt");
    observer.ingest(&t_receipt.cbor).expect("ingest receipt");

    let missing_offer_id = compute_offer_id("cafe", 5, "comum", 1_700_000_300_000, "did:comum:s");
    let missing_accept_params = build_accept_payload(&missing_offer_id, "did:comum:b");
    let missing_accept_invoke = build_invoke_payload(&capsule_id, "accept", &missing_accept_params);
    let t_missing_accept = buyer
        .emit(CAPSULE_INVOKE, &missing_accept_invoke, ctx.clone())
        .expect("accept missing");
    observer.ingest(&t_missing_accept.cbor).expect("ingest accept missing");

    let mut repeated_ids = Vec::new();
    for _ in 0..3 {
        let t_repeat = buyer.emit(CAPSULE_INVOKE, &accept_invoke, ctx.clone()).expect("accept repeat");
        observer.ingest(&t_repeat.cbor).expect("ingest accept repeat");
        repeated_ids.push(t_repeat.id_hex);
    }

    println!("[coherence-sim] coherent_offer_id={}", to_hex(&offer_id));
    println!("[coherence-sim] coherent_offer={}", t_offer.id_hex);
    println!("[coherence-sim] coherent_accept={}", t_accept.id_hex);
    println!("[coherence-sim] coherent_receipt={}", t_receipt.id_hex);
    println!("[coherence-sim] incoherent_accept={}", t_missing_accept.id_hex);
    println!("[coherence-sim] repeated_accepts={}", repeated_ids.len());
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
    pairs.push([encode_tstr("action"), encode_tstr(action)].concat());
    pairs.push([encode_tstr("params"), encode_bstr(params)].concat());
    pairs.push([encode_tstr("capsule_id"), encode_bstr(capsule_id)].concat());
    encode_map(pairs)
}

fn read_capsule_id(name: &str) -> [u8; 32] {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let wasm_path = root.join(format!("impl/capsulas/{}/{}.wasm", name, name));
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
