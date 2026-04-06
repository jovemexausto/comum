use serde::Deserialize;
use sha3::{Digest, Sha3_256};

mod decoder;
pub use decoder::validate_testimony_cbor;

mod cte;
pub use cte::{decode_cte, encode_cte, encode_fragment, fragment_cte, reassemble_fragments, Cte, CteFragment};

mod sync;
pub use sync::{build_hello, build_hello_ack, build_request, build_response, decode_payload_kv};

mod crypto;
pub use crypto::{sign_ed25519, verify_ed25519};

mod did;
pub use did::{build_key_rotate_payload, derive_did, validate_key_rotate_payload};

mod wasm_runtime;
pub use wasm_runtime::{run_capsule, run_capsule_with_limits};

mod abi;
pub use abi::*;

#[derive(Debug, Deserialize)]
pub struct Proof {
    pub version: u64,
    pub signatures: Vec<String>,
    pub zk_proofs: Vec<String>,
    pub nullifiers: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Claim {
    pub verb: String,
    pub payload_cbor_hex: String,
}

#[derive(Debug, Deserialize)]
pub struct Context {
    #[serde(rename = "type")]
    pub r#type: String,
    pub payload_cbor_hex: String,
    pub proof: Proof,
}

#[derive(Debug, Deserialize)]
pub struct TestimonyWithoutId {
    pub version: u64,
    pub author: Option<String>,
    pub timestamp: u64,
    pub suite: u64,
    pub prev_id: Option<String>,
    pub refs: Vec<String>,
    pub claim: Claim,
    pub context: Context,
    pub proof: Proof,
}

#[derive(Debug, Deserialize)]
pub struct Vector {
    pub name: String,
    pub testimony_without_id: TestimonyWithoutId,
    pub expected_id: String,
    pub testimony_without_id_cbor_hex: Option<String>,
}

pub(crate) fn encode_uint(n: u64) -> Vec<u8> {
    if n < 24 {
        return vec![n as u8];
    }
    if n < 256 {
        return vec![0x18, n as u8];
    }
    if n < 65536 {
        return vec![0x19, ((n >> 8) & 0xff) as u8, (n & 0xff) as u8];
    }
    if n < 4294967296 {
        return vec![
            0x1a,
            ((n >> 24) & 0xff) as u8,
            ((n >> 16) & 0xff) as u8,
            ((n >> 8) & 0xff) as u8,
            (n & 0xff) as u8,
        ];
    }
    vec![
        0x1b,
        ((n >> 56) & 0xff) as u8,
        ((n >> 48) & 0xff) as u8,
        ((n >> 40) & 0xff) as u8,
        ((n >> 32) & 0xff) as u8,
        ((n >> 24) & 0xff) as u8,
        ((n >> 16) & 0xff) as u8,
        ((n >> 8) & 0xff) as u8,
        (n & 0xff) as u8,
    ]
}

pub(crate) fn encode_bstr(data: &[u8]) -> Vec<u8> {
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

pub(crate) fn encode_tstr(s: &str) -> Vec<u8> {
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

pub(crate) fn encode_array(items: Vec<Vec<u8>>) -> Vec<u8> {
    let len = items.len();
    let mut out = if len < 24 {
        vec![0x80 + len as u8]
    } else if len < 256 {
        vec![0x98, len as u8]
    } else if len < 65536 {
        vec![0x99, ((len >> 8) & 0xff) as u8, (len & 0xff) as u8]
    } else {
        panic!("array too long")
    };
    for item in items {
        out.extend_from_slice(&item);
    }
    out
}

pub(crate) fn encode_map(pairs: Vec<Vec<u8>>) -> Vec<u8> {
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

pub(crate) fn decode_hex(s: &str) -> Vec<u8> {
    hex::decode(s).expect("invalid hex")
}

fn encode_proof(proof: &Proof) -> Vec<u8> {
    let mut pairs = Vec::new();
    pairs.push([encode_uint(0), encode_uint(proof.version)].concat());
    let sigs = proof
        .signatures
        .iter()
        .map(|s| encode_bstr(&decode_hex(s)))
        .collect();
    pairs.push([encode_uint(1), encode_array(sigs)].concat());
    let zks = proof
        .zk_proofs
        .iter()
        .map(|s| encode_bstr(&decode_hex(s)))
        .collect();
    pairs.push([encode_uint(2), encode_array(zks)].concat());
    let nulls = proof
        .nullifiers
        .iter()
        .map(|s| encode_bstr(&decode_hex(s)))
        .collect();
    pairs.push([encode_uint(3), encode_array(nulls)].concat());
    encode_map(pairs)
}

fn encode_claim(claim: &Claim) -> Vec<u8> {
    let mut pairs = Vec::new();
    pairs.push([encode_uint(0), encode_tstr(&claim.verb)].concat());
    pairs.push([
        encode_uint(1),
        encode_bstr(&decode_hex(&claim.payload_cbor_hex)),
    ]
    .concat());
    encode_map(pairs)
}

fn encode_context(context: &Context) -> Vec<u8> {
    let mut pairs = Vec::new();
    pairs.push([encode_uint(0), encode_tstr(&context.r#type)].concat());
    pairs.push([
        encode_uint(1),
        encode_bstr(&decode_hex(&context.payload_cbor_hex)),
    ]
    .concat());
    pairs.push([encode_uint(2), encode_proof(&context.proof)].concat());
    encode_map(pairs)
}

pub fn encode_testimony_without_id(t: &TestimonyWithoutId) -> Vec<u8> {
    let mut pairs = Vec::new();
    pairs.push([encode_uint(0), encode_uint(t.version)].concat());
    if let Some(author) = &t.author {
        pairs.push([encode_uint(2), encode_bstr(&decode_hex(author))].concat());
    }
    pairs.push([encode_uint(3), encode_uint(t.timestamp)].concat());
    pairs.push([encode_uint(4), encode_uint(t.suite)].concat());
    if let Some(prev) = &t.prev_id {
        pairs.push([encode_uint(5), encode_bstr(&decode_hex(prev))].concat());
    }
    let refs = t
        .refs
        .iter()
        .map(|r| encode_bstr(&decode_hex(r)))
        .collect();
    pairs.push([encode_uint(6), encode_array(refs)].concat());
    pairs.push([encode_uint(7), encode_claim(&t.claim)].concat());
    pairs.push([encode_uint(8), encode_context(&t.context)].concat());
    pairs.push([encode_uint(9), encode_proof(&t.proof)].concat());
    encode_map(pairs)
}

pub fn compute_id_hex(cbor: &[u8]) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(cbor);
    let out = hasher.finalize();
    hex::encode(out)
}
