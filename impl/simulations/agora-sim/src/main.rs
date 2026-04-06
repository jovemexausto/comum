use std::fs;
use std::path::Path;

use capsula_agora::{
    build_close_payload, build_propose_payload, build_vote_payload, tally_votes, AgoraVoteRecord,
};
use comum_rs::{Commoner, ContextInput, ProofInput};
use ed25519_dalek::SigningKey;
use sha3::{Digest, Sha3_256};

fn main() {
    let proposal = ProposalInput {
        title: "Atualizar regras",
        body: "Proposta de teste para simulacao",
        quorum: 3,
        expires: 1_700_000_100_000u64,
        author: "did:comum:sim",
    };

    let capsule_id = read_capsule_id();
    let propose_params = build_propose_payload(
        proposal.title,
        proposal.body,
        proposal.quorum,
        proposal.expires,
        proposal.author,
    );
    let propose_payload = build_invoke_payload(&capsule_id, "propose", &propose_params);

    let mut proposer = Commoner::new([0x11u8; 32], 1);
    let mut voter_b = Commoner::new([0x22u8; 32], 1);
    let mut voter_c = Commoner::new([0x33u8; 32], 1);
    let mut judge = Commoner::new([0x44u8; 32], 1);

    let pk_a = public_key_from_sk([0x11u8; 32]);
    let pk_b = public_key_from_sk([0x22u8; 32]);
    let pk_c = public_key_from_sk([0x33u8; 32]);
    judge.register_pk(pk_a);
    judge.register_pk(pk_b);
    judge.register_pk(pk_c);

    let ctx = default_context();
    let t_propose = proposer
        .emit("capsule/invoke", &propose_payload, ctx.clone())
        .expect("emit propose");
    judge.ingest(&t_propose.cbor).expect("ingest propose");

    let proposal_id = compute_proposal_id_from_params(&propose_params);
    let vote_a = build_vote_payload(&proposal_id, 1, "did:comum:a");
    let vote_b = build_vote_payload(&proposal_id, 0, "did:comum:b");
    let vote_c = build_vote_payload(&proposal_id, 1, "did:comum:c");

    let invoke_vote_a = build_invoke_payload(&capsule_id, "vote", &vote_a);
    let invoke_vote_b = build_invoke_payload(&capsule_id, "vote", &vote_b);
    let invoke_vote_c = build_invoke_payload(&capsule_id, "vote", &vote_c);

    let t_vote_a = proposer
        .emit("capsule/invoke", &invoke_vote_a, ctx.clone())
        .expect("emit vote a");
    let t_vote_b = voter_b
        .emit("capsule/invoke", &invoke_vote_b, ctx.clone())
        .expect("emit vote b");
    let t_vote_c = voter_c
        .emit("capsule/invoke", &invoke_vote_c, ctx.clone())
        .expect("emit vote c");

    judge.ingest(&t_vote_a.cbor).expect("ingest vote a");
    judge.ingest(&t_vote_b.cbor).expect("ingest vote b");
    judge.ingest(&t_vote_c.cbor).expect("ingest vote c");

    let close_payload = build_close_payload(&proposal_id, 1_700_000_100_100u64);
    let invoke_close = build_invoke_payload(&capsule_id, "close", &close_payload);
    let t_close = proposer
        .emit("capsule/invoke", &invoke_close, ctx)
        .expect("emit close");
    judge.ingest(&t_close.cbor).expect("ingest close");

    let votes = vec![
        AgoraVoteRecord {
            voter: "did:comum:a".to_string(),
            choice: 1,
            testimony_id: t_vote_a.id,
            timestamp: 100,
        },
        AgoraVoteRecord {
            voter: "did:comum:b".to_string(),
            choice: 0,
            testimony_id: t_vote_b.id,
            timestamp: 110,
        },
        AgoraVoteRecord {
            voter: "did:comum:c".to_string(),
            choice: 1,
            testimony_id: t_vote_c.id,
            timestamp: 120,
        },
    ];

    let tally = tally_votes(proposal.expires, Some(1_700_000_100_100u64), proposal.quorum, &votes);

    println!("[agora-sim] proposal_id={}", to_hex(&proposal_id));
    println!("[agora-sim] propose_id={}", t_propose.id_hex);
    println!("[agora-sim] close_id={}", t_close.id_hex);
    println!(
        "[agora-sim] yes={} no={} abstain={} quorum_met={} passed={}",
        tally.yes, tally.no, tally.abstain, tally.quorum_met, tally.passed
    );
}

#[derive(Clone)]
struct ProposalInput {
    title: &'static str,
    body: &'static str,
    quorum: u64,
    expires: u64,
    author: &'static str,
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

fn compute_proposal_id_from_params(params: &[u8]) -> [u8; 32] {
    let mut hasher = Sha3_256::new();
    hasher.update(params);
    let digest = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest[..32]);
    out
}

fn read_capsule_id() -> [u8; 32] {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let wasm_path = root.join("impl/capsulas/agora/agora.wasm");
    let wasm = fs::read(wasm_path).expect("read agora.wasm");
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
