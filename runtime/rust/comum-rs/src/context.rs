use crate::decoder::{CborValue, DecodeError, Decoder};
use crate::{encode_bstr, encode_map, encode_tstr, encode_uint};

pub fn build_proximity_context_payload(method: &str, nonce: &[u8; 16], timestamp: u64) -> Vec<u8> {
    let mut pairs = Vec::new();
    // keys sorted by length then lexicographic: nonce (5), method (6), timestamp (9)
    pairs.push([encode_tstr("nonce"), encode_bstr(nonce)].concat());
    pairs.push([encode_tstr("method"), encode_tstr(method)].concat());
    pairs.push([encode_tstr("timestamp"), encode_uint(timestamp)].concat());
    encode_map(pairs)
}

pub fn build_beacon_context_payload(
    beacon_id: &[u8; 32],
    token: &[u8],
    timestamp: u64,
) -> Vec<u8> {
    let mut pairs = Vec::new();
    // keys sorted by length then lexicographic: token (5), beacon_id (9), timestamp (9)
    pairs.push([encode_tstr("token"), encode_bstr(token)].concat());
    pairs.push([encode_tstr("beacon_id"), encode_bstr(beacon_id)].concat());
    pairs.push([encode_tstr("timestamp"), encode_uint(timestamp)].concat());
    encode_map(pairs)
}

pub fn build_place_context_payload(place_hash: &[u8; 32], timestamp: u64) -> Vec<u8> {
    let mut pairs = Vec::new();
    // keys sorted by length then lexicographic: timestamp (9), place_hash (10)
    pairs.push([encode_tstr("timestamp"), encode_uint(timestamp)].concat());
    pairs.push([encode_tstr("place_hash"), encode_bstr(place_hash)].concat());
    encode_map(pairs)
}

pub fn build_vouch_context_payload(
    subject_did: &str,
    community: &[u8; 32],
    timestamp: u64,
) -> Vec<u8> {
    let mut pairs = Vec::new();
    // keys sorted by length then lexicographic: subject (7), community (9), timestamp (9)
    pairs.push([encode_tstr("subject"), encode_tstr(subject_did)].concat());
    pairs.push([encode_tstr("community"), encode_bstr(community)].concat());
    pairs.push([encode_tstr("timestamp"), encode_uint(timestamp)].concat());
    encode_map(pairs)
}

pub fn validate_context_payload(ctx_type: &str, payload: &[u8]) -> Result<(), DecodeError> {
    let mut dec = Decoder::new(payload);
    let value = dec.decode()?;
    let map = match value {
        CborValue::MapText(m) => m,
        _ => return Err(DecodeError::InvalidType),
    };

    match ctx_type {
        "proximity" => {
            let required = ["method", "nonce", "timestamp"];
            for k in required {
                if !map.contains_key(k) {
                    return Err(DecodeError::MissingField);
                }
            }
            match map.get("method") {
                Some(CborValue::Text(s)) => {
                    if s != "nfc" && s != "ble" {
                        return Err(DecodeError::InvalidValue);
                    }
                }
                _ => return Err(DecodeError::InvalidType),
            }
            match map.get("nonce") {
                Some(CborValue::Bytes(b)) if b.len() == 16 => {}
                _ => return Err(DecodeError::InvalidValue),
            }
            match map.get("timestamp") {
                Some(CborValue::Unsigned(_)) => {}
                _ => return Err(DecodeError::InvalidType),
            }
        }
        "beacon" => {
            let required = ["beacon_id", "token", "timestamp"];
            for k in required {
                if !map.contains_key(k) {
                    return Err(DecodeError::MissingField);
                }
            }
            match map.get("beacon_id") {
                Some(CborValue::Bytes(b)) if b.len() == 32 => {}
                _ => return Err(DecodeError::InvalidValue),
            }
            match map.get("token") {
                Some(CborValue::Bytes(b)) if !b.is_empty() => {}
                _ => return Err(DecodeError::InvalidValue),
            }
            match map.get("timestamp") {
                Some(CborValue::Unsigned(_)) => {}
                _ => return Err(DecodeError::InvalidType),
            }
        }
        "place" => {
            let required = ["place_hash", "timestamp"];
            for k in required {
                if !map.contains_key(k) {
                    return Err(DecodeError::MissingField);
                }
            }
            match map.get("place_hash") {
                Some(CborValue::Bytes(b)) if b.len() == 32 => {}
                _ => return Err(DecodeError::InvalidValue),
            }
            match map.get("timestamp") {
                Some(CborValue::Unsigned(_)) => {}
                _ => return Err(DecodeError::InvalidType),
            }
        }
        "vouch" => {
            let required = ["subject", "community", "timestamp"];
            for k in required {
                if !map.contains_key(k) {
                    return Err(DecodeError::MissingField);
                }
            }
            match map.get("subject") {
                Some(CborValue::Text(s)) => {
                    if !s.starts_with("did:comum:") || s.len() <= "did:comum:".len() {
                        return Err(DecodeError::InvalidValue);
                    }
                }
                _ => return Err(DecodeError::InvalidType),
            }
            match map.get("community") {
                Some(CborValue::Bytes(b)) if b.len() == 32 => {}
                _ => return Err(DecodeError::InvalidValue),
            }
            match map.get("timestamp") {
                Some(CborValue::Unsigned(_)) => {}
                _ => return Err(DecodeError::InvalidType),
            }
        }
        _ => return Err(DecodeError::InvalidValue),
    }

    Ok(())
}
