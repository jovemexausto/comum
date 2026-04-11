use bs58;
use sha3::{Digest, Sha3_256};

use crate::decoder::{CborValue, DecodeError, Decoder};
use crate::{encode_bstr, encode_map, encode_tstr, encode_uint};

pub fn derive_did(pk_bytes: &[u8; 32]) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(pk_bytes);
    let digest = hasher.finalize();
    let short = &digest[..20];
    let b58 = bs58::encode(short).with_check().into_string();
    format!("did:comum:{}", b58)
}

pub fn build_key_rotate_payload(
    community_id: &[u8; 32],
    old_pk: &[u8],
    new_pk: &[u8],
    timestamp: u64,
) -> Vec<u8> {
    // Canonical order for text keys: by length, then lexicographic
    // Keys: new_pk (6), old_pk (6), community (9), timestamp (9)
    let mut pairs = Vec::new();
    pairs.push([encode_tstr("new_pk"), encode_bstr(new_pk)].concat());
    pairs.push([encode_tstr("old_pk"), encode_bstr(old_pk)].concat());
    pairs.push([encode_tstr("community"), encode_bstr(community_id)].concat());
    pairs.push([encode_tstr("timestamp"), encode_uint(timestamp)].concat());
    encode_map(pairs)
}

pub fn validate_key_rotate_payload(payload: &[u8]) -> Result<(), DecodeError> {
    let mut dec = Decoder::new(payload);
    let value = dec.decode()?;
    let map = match value {
        CborValue::MapText(m) => m,
        _ => return Err(DecodeError::InvalidType),
    };

    let required = ["community", "old_pk", "new_pk", "timestamp"];
    for k in required {
        if !map.contains_key(k) {
            return Err(DecodeError::MissingField);
        }
    }

    match map.get("community") {
        Some(CborValue::Bytes(b)) if b.len() == 32 => {}
        _ => return Err(DecodeError::InvalidValue),
    }
    match map.get("old_pk") {
        Some(CborValue::Bytes(b)) if !b.is_empty() => {}
        _ => return Err(DecodeError::InvalidValue),
    }
    match map.get("new_pk") {
        Some(CborValue::Bytes(b)) if !b.is_empty() => {}
        _ => return Err(DecodeError::InvalidValue),
    }
    match map.get("timestamp") {
        Some(CborValue::Unsigned(_)) => {}
        _ => return Err(DecodeError::InvalidType),
    }

    Ok(())
}
