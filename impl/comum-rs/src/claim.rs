use crate::decoder::{CborValue, DecodeError, Decoder};
use crate::{encode_bstr, encode_map, encode_tstr, encode_uint};

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
