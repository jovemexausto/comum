use crate::decoder::{CborValue, DecodeError, Decoder};
use crate::{
    encode_array, encode_bstr, encode_map, encode_tstr, encode_uint,
    COMUM_AUTH_NULLIFIER, COMUM_IDENTITY_COMMITMENT, COMUM_IDENTITY_VOUCH,
};

pub fn build_receive_payload(of: &[u8; 32], timestamp: u64) -> Vec<u8> {
    let mut pairs = Vec::new();
    // keys sorted by length then lexicographic: of(2), timestamp(9)
    pairs.push([encode_tstr("of"), encode_bstr(of)].concat());
    pairs.push([encode_tstr("timestamp"), encode_uint(timestamp)].concat());
    encode_map(pairs)
}

pub fn build_identity_commitment_payload(
    commitment: &[u8; 32],
    epoch: u64,
    proof_system: &str,
    circuit_id: &[u8; 32],
    supersedes: Option<&[u8; 32]>,
) -> Vec<u8> {
    let mut pairs = Vec::new();
    // keys sorted by length then lexicographic:
    // epoch(5), circuit_id(10), commitment(10), supersedes(10), proof_system(12)
    pairs.push([encode_tstr("epoch"), encode_uint(epoch)].concat());
    pairs.push([encode_tstr("circuit_id"), encode_bstr(circuit_id)].concat());
    pairs.push([encode_tstr("commitment"), encode_bstr(commitment)].concat());
    if let Some(value) = supersedes {
        pairs.push([encode_tstr("supersedes"), encode_bstr(value)].concat());
    }
    pairs.push([encode_tstr("proof_system"), encode_tstr(proof_system)].concat());
    encode_map(pairs)
}

pub fn validate_identity_commitment_payload(payload: &[u8]) -> Result<(), DecodeError> {
    let mut dec = Decoder::new(payload);
    let value = dec.decode()?;
    let map = match value {
        CborValue::MapText(m) => m,
        _ => return Err(DecodeError::InvalidType),
    };

    let required = ["commitment", "epoch", "proof_system", "circuit_id"];
    for k in required {
        if !map.contains_key(k) {
            return Err(DecodeError::MissingField);
        }
    }

    match map.get("commitment") {
        Some(CborValue::Bytes(b)) if b.len() == 32 => {}
        _ => return Err(DecodeError::InvalidValue),
    }
    match map.get("epoch") {
        Some(CborValue::Unsigned(_)) => {}
        _ => return Err(DecodeError::InvalidType),
    }
    match map.get("proof_system") {
        Some(CborValue::Text(s)) if s == "groth16" || s == "ultrahonk" => {}
        _ => return Err(DecodeError::InvalidValue),
    }
    match map.get("circuit_id") {
        Some(CborValue::Bytes(b)) if b.len() == 32 => {}
        _ => return Err(DecodeError::InvalidValue),
    }
    if let Some(val) = map.get("supersedes") {
        match val {
            CborValue::Bytes(b) if b.len() == 32 => {}
            _ => return Err(DecodeError::InvalidValue),
        }
    }

    Ok(())
}

pub fn build_auth_nullifier_payload(
    session_nullifier: &[u8; 32],
    session_nonce: &[u8; 32],
    commitment_ref: &[u8; 32],
    resource_id: &[u8; 32],
    proof_system: &str,
    circuit_id: &[u8; 32],
) -> Vec<u8> {
    let pairs = vec![
        [encode_tstr("circuit_id"), encode_bstr(circuit_id)].concat(),
        [encode_tstr("resource_id"), encode_bstr(resource_id)].concat(),
        [encode_tstr("proof_system"), encode_tstr(proof_system)].concat(),
        [encode_tstr("session_nonce"), encode_bstr(session_nonce)].concat(),
        [encode_tstr("commitment_ref"), encode_bstr(commitment_ref)].concat(),
        [
            encode_tstr("session_nullifier"),
            encode_bstr(session_nullifier),
        ]
        .concat(),
    ];
    encode_map(pairs)
}

pub fn validate_auth_nullifier_payload(payload: &[u8]) -> Result<(), DecodeError> {
    let mut dec = Decoder::new(payload);
    let value = dec.decode()?;
    let map = match value {
        CborValue::MapText(m) => m,
        _ => return Err(DecodeError::InvalidType),
    };

    let required = [
        "session_nullifier",
        "session_nonce",
        "commitment_ref",
        "resource_id",
        "proof_system",
        "circuit_id",
    ];
    for k in required {
        if !map.contains_key(k) {
            return Err(DecodeError::MissingField);
        }
    }

    for key in [
        "session_nullifier",
        "session_nonce",
        "commitment_ref",
        "resource_id",
        "circuit_id",
    ] {
        match map.get(key) {
            Some(CborValue::Bytes(b)) if b.len() == 32 => {}
            _ => return Err(DecodeError::InvalidValue),
        }
    }
    match map.get("proof_system") {
        Some(CborValue::Text(s)) if s == "groth16" || s == "ultrahonk" => {}
        _ => return Err(DecodeError::InvalidValue),
    }

    Ok(())
}

pub fn build_identity_vouch_payload(
    commitment_ref: &[u8; 32],
    voucher_depth: u64,
    vouch_nonce: &[u8; 32],
) -> Vec<u8> {
    let pairs = vec![
        [encode_tstr("vouch_nonce"), encode_bstr(vouch_nonce)].concat(),
        [encode_tstr("voucher_depth"), encode_uint(voucher_depth)].concat(),
        [encode_tstr("commitment_ref"), encode_bstr(commitment_ref)].concat(),
    ];
    encode_map(pairs)
}

pub fn validate_identity_vouch_payload(payload: &[u8]) -> Result<(), DecodeError> {
    let mut dec = Decoder::new(payload);
    let value = dec.decode()?;
    let map = match value {
        CborValue::MapText(m) => m,
        _ => return Err(DecodeError::InvalidType),
    };

    let required = ["commitment_ref", "voucher_depth", "vouch_nonce"];
    for k in required {
        if !map.contains_key(k) {
            return Err(DecodeError::MissingField);
        }
    }

    match map.get("commitment_ref") {
        Some(CborValue::Bytes(b)) if b.len() == 32 => {}
        _ => return Err(DecodeError::InvalidValue),
    }
    match map.get("voucher_depth") {
        Some(CborValue::Unsigned(_)) => {}
        _ => return Err(DecodeError::InvalidType),
    }
    match map.get("vouch_nonce") {
        Some(CborValue::Bytes(b)) if b.len() == 32 => {}
        _ => return Err(DecodeError::InvalidValue),
    }

    Ok(())
}

pub fn validate_claim_payload(verb: &str, payload: &[u8]) -> Result<(), DecodeError> {
    match verb {
        COMUM_IDENTITY_COMMITMENT => validate_identity_commitment_payload(payload),
        COMUM_AUTH_NULLIFIER => validate_auth_nullifier_payload(payload),
        COMUM_IDENTITY_VOUCH => validate_identity_vouch_payload(payload),
        _ => Ok(()),
    }
}

pub fn validate_receive_payload(payload: &[u8]) -> Result<(), DecodeError> {
    let mut dec = Decoder::new(payload);
    let value = dec.decode()?;
    let map = match value {
        CborValue::MapText(m) => m,
        _ => return Err(DecodeError::InvalidType),
    };

    let required = ["of", "timestamp"];
    for k in required {
        if !map.contains_key(k) {
            return Err(DecodeError::MissingField);
        }
    }

    match map.get("of") {
        Some(CborValue::Bytes(b)) if b.len() == 32 => {}
        _ => return Err(DecodeError::InvalidValue),
    }
    match map.get("timestamp") {
        Some(CborValue::Unsigned(_)) => {}
        _ => return Err(DecodeError::InvalidType),
    }

    Ok(())
}

pub fn build_genesis_payload(
    name: &str,
    threshold: u64,
    founders: &[&str],
    capsules: &[[u8; 32]],
    supply: u64,
    mint_policy: &[u8; 32],
) -> Vec<u8> {
    let mut pairs = Vec::new();
    // keys sorted by length then lexicographic:
    // name(4), supply(6), capsules(8), founders(8), threshold(9), mint_policy(11)
    pairs.push([encode_tstr("name"), encode_tstr(name)].concat());
    pairs.push([encode_tstr("supply"), encode_uint(supply)].concat());

    let capsule_items = capsules.iter().map(|c| encode_bstr(c)).collect();
    pairs.push([encode_tstr("capsules"), encode_array(capsule_items)].concat());

    let founder_items = founders.iter().map(|d| encode_tstr(d)).collect();
    pairs.push([encode_tstr("founders"), encode_array(founder_items)].concat());

    pairs.push([encode_tstr("threshold"), encode_uint(threshold)].concat());
    pairs.push([encode_tstr("mint_policy"), encode_bstr(mint_policy)].concat());
    encode_map(pairs)
}

pub fn validate_genesis_payload(payload: &[u8]) -> Result<(), DecodeError> {
    let mut dec = Decoder::new(payload);
    let value = dec.decode()?;
    let map = match value {
        CborValue::MapText(m) => m,
        _ => return Err(DecodeError::InvalidType),
    };

    let required = ["name", "threshold", "founders", "capsules", "supply", "mint_policy"];
    for k in required {
        if !map.contains_key(k) {
            return Err(DecodeError::MissingField);
        }
    }

    match map.get("name") {
        Some(CborValue::Text(s)) if !s.is_empty() => {}
        _ => return Err(DecodeError::InvalidValue),
    }

    let founders_len = match map.get("founders") {
        Some(CborValue::Array(items)) => {
            if items.len() < 3 {
                return Err(DecodeError::InvalidValue);
            }
            for item in items {
                match item {
                    CborValue::Text(s) => {
                        if !s.starts_with("did:comum:") || s.len() <= "did:comum:".len() {
                            return Err(DecodeError::InvalidValue);
                        }
                    }
                    _ => return Err(DecodeError::InvalidType),
                }
            }
            items.len() as u64
        }
        _ => return Err(DecodeError::InvalidType),
    };

    match map.get("threshold") {
        Some(CborValue::Unsigned(v)) => {
            if *v == 0 || *v > founders_len {
                return Err(DecodeError::InvalidValue);
            }
        }
        _ => return Err(DecodeError::InvalidType),
    }

    match map.get("capsules") {
        Some(CborValue::Array(items)) => {
            for item in items {
                match item {
                    CborValue::Bytes(b) if b.len() == 32 => {}
                    _ => return Err(DecodeError::InvalidValue),
                }
            }
        }
        _ => return Err(DecodeError::InvalidType),
    }

    match map.get("supply") {
        Some(CborValue::Unsigned(_)) => {}
        _ => return Err(DecodeError::InvalidType),
    }

    match map.get("mint_policy") {
        Some(CborValue::Bytes(b)) if b.len() == 32 => {}
        _ => return Err(DecodeError::InvalidValue),
    }

    Ok(())
}
