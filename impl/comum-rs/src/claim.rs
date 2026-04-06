use crate::decoder::{CborValue, DecodeError, Decoder};
use crate::{encode_array, encode_bstr, encode_map, encode_tstr, encode_uint};

pub fn build_receive_payload(of: &[u8; 32], timestamp: u64) -> Vec<u8> {
    let mut pairs = Vec::new();
    // keys sorted by length then lexicographic: of(2), timestamp(9)
    pairs.push([encode_tstr("of"), encode_bstr(of)].concat());
    pairs.push([encode_tstr("timestamp"), encode_uint(timestamp)].concat());
    encode_map(pairs)
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
