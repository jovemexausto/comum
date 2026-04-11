use crate::decoder::{CborValue, DecodeError, Decoder};
use crate::{
    compute_id_hex, encode_array, encode_bstr, encode_map, encode_tstr, encode_uint,
    encode_testimony_without_id, Claim, Context, Proof, TestimonyWithoutId,
};

use super::types::CommonerError;

pub(crate) struct DecodedTestimony {
    pub id: Option<[u8; 32]>,
    pub author: Option<[u8; 32]>,
    pub timestamp: u64,
    pub suite: u64,
    pub prev_id: Option<[u8; 32]>,
    pub refs: Vec<[u8; 32]>,
    pub claim_verb: String,
    pub claim_payload: Vec<u8>,
    pub context_type: String,
    pub context_payload: Vec<u8>,
    pub context_signatures: Vec<[u8; 64]>,
    pub context_zk_proofs: Vec<Vec<u8>>,
    pub context_nullifiers: Vec<Vec<u8>>,
    pub signatures: Vec<[u8; 64]>,
    pub nullifiers: Vec<[u8; 32]>,
    pub proof_version: u64,
    pub context_proof_version: u64,
    pub zk_proofs: Vec<Vec<u8>>,
}

pub(crate) fn decode_testimony(data: &[u8]) -> Result<DecodedTestimony, CommonerError> {
    let mut dec = Decoder::new(data);
    let value = dec.decode().map_err(map_decode_error)?;
    let map = match value {
        CborValue::Map(m) => m,
        _ => return Err(CommonerError::format("invalid testimony map")),
    };

    let id = match map.get(&1) {
        Some(CborValue::Bytes(b)) => Some(to_32(b, "id")?),
        None => None,
        _ => return Err(CommonerError::format("invalid id")),
    };
    let author = match map.get(&2) {
        Some(CborValue::Bytes(b)) => Some(to_32(b, "author")?),
        None => None,
        _ => return Err(CommonerError::format("invalid author")),
    };
    let timestamp = match map.get(&3) {
        Some(CborValue::Unsigned(v)) => *v,
        _ => return Err(CommonerError::format("invalid timestamp")),
    };
    let suite = match map.get(&4) {
        Some(CborValue::Unsigned(v)) => *v,
        _ => return Err(CommonerError::format("invalid suite")),
    };
    let prev_id = match map.get(&5) {
        Some(CborValue::Bytes(b)) => Some(to_32(b, "prev_id")?),
        None => None,
        _ => return Err(CommonerError::format("invalid prev_id")),
    };
    let refs = match map.get(&6) {
        Some(CborValue::Array(items)) => {
            let mut out = Vec::new();
            for item in items {
                match item {
                    CborValue::Bytes(b) => out.push(to_32(b, "ref")?),
                    _ => return Err(CommonerError::format("invalid ref")),
                }
            }
            out
        }
        _ => return Err(CommonerError::format("invalid refs")),
    };
    let claim_map = match map.get(&7) {
        Some(CborValue::Map(m)) => m,
        _ => return Err(CommonerError::format("invalid claim")),
    };
    let claim_verb = match claim_map.get(&0) {
        Some(CborValue::Text(s)) => s.clone(),
        _ => return Err(CommonerError::format("invalid claim verb")),
    };
    let claim_payload = match claim_map.get(&1) {
        Some(CborValue::Bytes(b)) => b.clone(),
        _ => return Err(CommonerError::format("invalid claim payload")),
    };
    let context_map = match map.get(&8) {
        Some(CborValue::Map(m)) => m,
        _ => return Err(CommonerError::format("invalid context")),
    };
    let context_type = match context_map.get(&0) {
        Some(CborValue::Text(s)) => s.clone(),
        _ => return Err(CommonerError::format("invalid context type")),
    };
    let context_payload = match context_map.get(&1) {
        Some(CborValue::Bytes(b)) => b.clone(),
        _ => return Err(CommonerError::format("invalid context payload")),
    };
    let context_proof_map = match context_map.get(&2) {
        Some(CborValue::Map(m)) => m,
        _ => return Err(CommonerError::format("invalid context proof")),
    };

    let context_proof_version = match context_proof_map.get(&0) {
        Some(CborValue::Unsigned(v)) => *v,
        _ => return Err(CommonerError::format("invalid context proof version")),
    };
    let context_signatures = match context_proof_map.get(&1) {
        Some(CborValue::Array(items)) => decode_sig_array(items, "context signatures")?,
        _ => return Err(CommonerError::format("invalid context signatures")),
    };
    let context_zk_proofs = match context_proof_map.get(&2) {
        Some(CborValue::Array(items)) => decode_bytes_array(items, "context zk_proofs")?,
        _ => return Err(CommonerError::format("invalid context zk_proofs")),
    };
    let context_nullifiers = match context_proof_map.get(&3) {
        Some(CborValue::Array(items)) => decode_bytes_array(items, "context nullifiers")?,
        _ => return Err(CommonerError::format("invalid context nullifiers")),
    };

    let proof_map = match map.get(&9) {
        Some(CborValue::Map(m)) => m,
        _ => return Err(CommonerError::format("invalid proof")),
    };
    let proof_version = match proof_map.get(&0) {
        Some(CborValue::Unsigned(v)) => *v,
        _ => return Err(CommonerError::format("invalid proof version")),
    };
    let signatures = match proof_map.get(&1) {
        Some(CborValue::Array(items)) => decode_sig_array(items, "signatures")?,
        _ => return Err(CommonerError::format("invalid signatures")),
    };
    let zk_proofs = match proof_map.get(&2) {
        Some(CborValue::Array(items)) => decode_bytes_array(items, "zk_proofs")?,
        _ => return Err(CommonerError::format("invalid zk_proofs")),
    };
    let nullifiers = match proof_map.get(&3) {
        Some(CborValue::Array(items)) => decode_nullifiers(items)?,
        _ => return Err(CommonerError::format("invalid nullifiers")),
    };

    Ok(DecodedTestimony {
        id,
        author,
        timestamp,
        suite,
        prev_id,
        refs,
        claim_verb,
        claim_payload,
        context_type,
        context_payload,
        context_signatures,
        context_zk_proofs,
        context_nullifiers,
        signatures,
        nullifiers,
        proof_version,
        context_proof_version,
        zk_proofs,
    })
}

pub(crate) fn compute_id_from_decoded(decoded: &DecodedTestimony) -> [u8; 32] {
    let t = TestimonyWithoutId {
        version: 3,
        author: decoded.author.map(hex::encode),
        timestamp: decoded.timestamp,
        suite: decoded.suite,
        prev_id: decoded.prev_id.map(hex::encode),
        refs: decoded.refs.iter().map(hex::encode).collect(),
        claim: Claim {
            verb: decoded.claim_verb.clone(),
            payload_cbor_hex: hex::encode(&decoded.claim_payload),
        },
        context: Context {
            r#type: decoded.context_type.clone(),
            payload_cbor_hex: hex::encode(&decoded.context_payload),
            proof: Proof {
                version: decoded.context_proof_version,
                signatures: decoded
                    .context_signatures
                    .iter()
                    .map(|s| hex::encode(s))
                    .collect(),
                zk_proofs: decoded
                    .context_zk_proofs
                    .iter()
                    .map(hex::encode)
                    .collect(),
                nullifiers: decoded
                    .context_nullifiers
                    .iter()
                    .map(hex::encode)
                    .collect(),
            },
        },
        proof: Proof {
            version: decoded.proof_version,
            signatures: Vec::new(),
            zk_proofs: decoded.zk_proofs.iter().map(hex::encode).collect(),
            nullifiers: Vec::new(),
        },
    };
    let cbor = encode_testimony_without_id(&t);
    let mut out = [0u8; 32];
    out.copy_from_slice(&hex::decode(compute_id_hex(&cbor)).expect("id hex"));
    out
}

pub(crate) fn encode_testimony_with_id(
    t: &TestimonyWithoutId,
    id: &[u8; 32],
    proof: &Proof,
) -> Vec<u8> {
    let mut pairs = Vec::new();
    pairs.push([encode_uint(0), encode_uint(t.version)].concat());
    pairs.push([encode_uint(1), encode_bstr(id)].concat());
    if let Some(author) = &t.author {
        pairs.push([encode_uint(2), encode_bstr(&hex::decode(author).unwrap())].concat());
    }
    pairs.push([encode_uint(3), encode_uint(t.timestamp)].concat());
    pairs.push([encode_uint(4), encode_uint(t.suite)].concat());
    if let Some(prev) = &t.prev_id {
        pairs.push([encode_uint(5), encode_bstr(&hex::decode(prev).unwrap())].concat());
    }
    let refs = t
        .refs
        .iter()
        .map(|r| encode_bstr(&hex::decode(r).unwrap()))
        .collect();
    pairs.push([encode_uint(6), encode_array(refs)].concat());
    pairs.push([encode_uint(7), encode_claim(&t.claim)].concat());
    pairs.push([encode_uint(8), encode_context(&t.context)].concat());
    pairs.push([encode_uint(9), encode_proof(proof)].concat());
    encode_map(pairs)
}

fn encode_claim(claim: &Claim) -> Vec<u8> {
    let mut pairs = Vec::new();
    pairs.push([encode_uint(0), encode_tstr(&claim.verb)].concat());
    pairs.push([
        encode_uint(1),
        encode_bstr(&hex::decode(&claim.payload_cbor_hex).unwrap()),
    ]
    .concat());
    encode_map(pairs)
}

fn encode_context(context: &Context) -> Vec<u8> {
    let mut pairs = Vec::new();
    pairs.push([encode_uint(0), encode_tstr(&context.r#type)].concat());
    pairs.push([
        encode_uint(1),
        encode_bstr(&hex::decode(&context.payload_cbor_hex).unwrap()),
    ]
    .concat());
    pairs.push([encode_uint(2), encode_proof(&context.proof)].concat());
    encode_map(pairs)
}

fn encode_proof(proof: &Proof) -> Vec<u8> {
    let mut pairs = Vec::new();
    pairs.push([encode_uint(0), encode_uint(proof.version)].concat());
    let sigs = proof
        .signatures
        .iter()
        .map(|s| encode_bstr(&hex::decode(s).unwrap()))
        .collect();
    pairs.push([encode_uint(1), encode_array(sigs)].concat());
    let zks = proof
        .zk_proofs
        .iter()
        .map(|s| encode_bstr(&hex::decode(s).unwrap()))
        .collect();
    pairs.push([encode_uint(2), encode_array(zks)].concat());
    let nulls = proof
        .nullifiers
        .iter()
        .map(|s| encode_bstr(&hex::decode(s).unwrap()))
        .collect();
    pairs.push([encode_uint(3), encode_array(nulls)].concat());
    encode_map(pairs)
}

fn decode_sig_array(items: &[CborValue], label: &str) -> Result<Vec<[u8; 64]>, CommonerError> {
    let mut out = Vec::new();
    for item in items {
        match item {
            CborValue::Bytes(b) => {
                if b.len() != 64 {
                    return Err(CommonerError::proof(&format!("{} length", label)));
                }
                let mut sig = [0u8; 64];
                sig.copy_from_slice(b);
                out.push(sig);
            }
            _ => return Err(CommonerError::format("invalid signature type")),
        }
    }
    Ok(out)
}

fn decode_bytes_array(items: &[CborValue], label: &str) -> Result<Vec<Vec<u8>>, CommonerError> {
    let mut out = Vec::new();
    for item in items {
        match item {
            CborValue::Bytes(b) => out.push(b.clone()),
            _ => return Err(CommonerError::format(&format!("invalid {} item", label))),
        }
    }
    Ok(out)
}

fn decode_nullifiers(items: &[CborValue]) -> Result<Vec<[u8; 32]>, CommonerError> {
    let mut out = Vec::new();
    for item in items {
        match item {
            CborValue::Bytes(b) => out.push(to_32(b, "nullifier")?),
            _ => return Err(CommonerError::format("invalid nullifier")),
        }
    }
    Ok(out)
}

fn to_32(bytes: &[u8], label: &str) -> Result<[u8; 32], CommonerError> {
    if bytes.len() != 32 {
        return Err(CommonerError::format(&format!("{} length", label)));
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(bytes);
    Ok(out)
}

fn map_decode_error(err: DecodeError) -> CommonerError {
    CommonerError::format(&format!("decode error: {:?}", err))
}
