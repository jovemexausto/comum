use std::fs;
use std::path::Path;

use capsula_mutirao::{
    build_checkin_payload, build_commit_payload, build_complete_payload, build_reward_payload,
    build_task_payload, compute_task_id,
};
use comum_rs::{Commoner, ContextInput, ProofInput, CAPSULE_INVOKE};
use ed25519_dalek::SigningKey;
use sha3::{Digest, Sha3_256};

fn main() {
    let capsule_id = read_capsule_id("mutirao");

    let task_id = compute_task_id("limpeza", "praça", 3, 1_700_000_300_000, "did:comum:c");
    let task_params = build_task_payload("limpeza", "praça", 3, 1_700_000_300_000, "did:comum:c");
    let task_invoke = build_invoke_payload(&capsule_id, "task", &task_params);

    let commit_params = build_commit_payload(&task_id, "did:comum:w");
    let commit_invoke = build_invoke_payload(&capsule_id, "commit", &commit_params);

    let checkin_params = build_checkin_payload(&task_id, 1_700_000_300_100);
    let checkin_invoke = build_invoke_payload(&capsule_id, "checkin", &checkin_params);

    let complete_params = build_complete_payload(&task_id, 1_700_000_300_200);
    let complete_invoke = build_invoke_payload(&capsule_id, "complete", &complete_params);

    let reward_params = build_reward_payload(&task_id, 2, 1_700_000_300_300);
    let reward_invoke = build_invoke_payload(&capsule_id, "reward", &reward_params);

    let mut creator = Commoner::new([0x11u8; 32], 1);
    let mut worker = Commoner::new([0x22u8; 32], 1);
    let mut verifier = Commoner::new([0x33u8; 32], 1);

    let pk_c = public_key_from_sk([0x11u8; 32]);
    let pk_w = public_key_from_sk([0x22u8; 32]);
    verifier.register_pk(pk_c);
    verifier.register_pk(pk_w);

    let ctx = default_context();
    let t_task = creator.emit(CAPSULE_INVOKE, &task_invoke, ctx.clone()).expect("task");
    verifier.ingest(&t_task.cbor).expect("ingest task");

    let t_commit = worker.emit(CAPSULE_INVOKE, &commit_invoke, ctx.clone()).expect("commit");
    verifier.ingest(&t_commit.cbor).expect("ingest commit");

    let t_checkin = worker.emit(CAPSULE_INVOKE, &checkin_invoke, ctx.clone()).expect("checkin");
    verifier.ingest(&t_checkin.cbor).expect("ingest checkin");

    let t_complete = creator.emit(CAPSULE_INVOKE, &complete_invoke, ctx.clone()).expect("complete");
    verifier.ingest(&t_complete.cbor).expect("ingest complete");

    let t_reward = creator.emit(CAPSULE_INVOKE, &reward_invoke, ctx).expect("reward");
    verifier.ingest(&t_reward.cbor).expect("ingest reward");

    println!("[mutirao-sim] task_id={}", to_hex(&task_id));
    println!("[mutirao-sim] task_id_testimony={}", t_task.id_hex);
    println!("[mutirao-sim] commit_id={}", t_commit.id_hex);
    println!("[mutirao-sim] checkin_id={}", t_checkin.id_hex);
    println!("[mutirao-sim] complete_id={}", t_complete.id_hex);
    println!("[mutirao-sim] reward_id={}", t_reward.id_hex);
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
