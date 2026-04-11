use std::collections::BTreeMap;

use crate::validate_claim_payload;

#[derive(Debug, Clone)]
pub enum CborValue {
    Unsigned(u64),
    Bytes(Vec<u8>),
    Text(String),
    Array(Vec<CborValue>),
    Map(BTreeMap<u64, CborValue>),
    MapText(BTreeMap<String, CborValue>),
}

#[derive(Debug)]
pub enum DecodeError {
    Eof,
    InvalidType,
    InvalidUtf8,
    NonCanonical,
    InvalidMapKey,
    MissingField,
    InvalidValue,
}

pub(crate) struct Decoder<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Decoder<'a> {
    pub(crate) fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    fn read_u8(&mut self) -> Result<u8, DecodeError> {
        if self.pos >= self.data.len() {
            return Err(DecodeError::Eof);
        }
        let b = self.data[self.pos];
        self.pos += 1;
        Ok(b)
    }

    fn read_n(&mut self, n: usize) -> Result<&'a [u8], DecodeError> {
        if self.pos + n > self.data.len() {
            return Err(DecodeError::Eof);
        }
        let out = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Ok(out)
    }

    fn decode_len(&mut self, ai: u8) -> Result<u64, DecodeError> {
        match ai {
            0..=23 => Ok(ai as u64),
            24 => Ok(self.read_u8()? as u64),
            25 => {
                let b = self.read_n(2)?;
                Ok(u16::from_be_bytes([b[0], b[1]]) as u64)
            }
            26 => {
                let b = self.read_n(4)?;
                Ok(u32::from_be_bytes([b[0], b[1], b[2], b[3]]) as u64)
            }
            27 => {
                let b = self.read_n(8)?;
                Ok(u64::from_be_bytes([
                    b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
                ]))
            }
            _ => Err(DecodeError::InvalidType),
        }
    }

    fn check_canonical_len(ai: u8, len: u64) -> Result<(), DecodeError> {
        let canonical_ai = if len < 24 {
            len as u8
        } else if len < 256 {
            24
        } else if len < 65536 {
            25
        } else if len < 4294967296 {
            26
        } else {
            27
        };
        if ai != canonical_ai {
            return Err(DecodeError::NonCanonical);
        }
        Ok(())
    }

    pub(crate) fn decode(&mut self) -> Result<CborValue, DecodeError> {
        let initial = self.read_u8()?;
        let major = initial >> 5;
        let ai = initial & 0x1f;

        match major {
            0 => {
                let val = self.decode_len(ai)?;
                Self::check_canonical_len(ai, val)?;
                Ok(CborValue::Unsigned(val))
            }
            2 => {
                let len = self.decode_len(ai)?;
                Self::check_canonical_len(ai, len)?;
                let b = self.read_n(len as usize)?.to_vec();
                Ok(CborValue::Bytes(b))
            }
            3 => {
                let len = self.decode_len(ai)?;
                Self::check_canonical_len(ai, len)?;
                let b = self.read_n(len as usize)?.to_vec();
                let s = String::from_utf8(b).map_err(|_| DecodeError::InvalidUtf8)?;
                Ok(CborValue::Text(s))
            }
            4 => {
                let len = self.decode_len(ai)?;
                Self::check_canonical_len(ai, len)?;
                let mut items = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    items.push(self.decode()?);
                }
                Ok(CborValue::Array(items))
            }
            5 => {
                let len = self.decode_len(ai)?;
                Self::check_canonical_len(ai, len)?;
                let mut map_num = BTreeMap::new();
                let mut map_text = BTreeMap::new();
                let mut last_num: Option<u64> = None;
                let mut last_text: Option<Vec<u8>> = None;
                let mut text_keys = false;
                for _ in 0..len {
                    let key_val = self.decode()?;
                    let val = self.decode()?;
                    match key_val {
                        CborValue::Unsigned(v) => {
                            if text_keys {
                                return Err(DecodeError::InvalidMapKey);
                            }
                            if let Some(prev) = last_num {
                                if v <= prev {
                                    return Err(DecodeError::NonCanonical);
                                }
                            }
                            last_num = Some(v);
                            map_num.insert(v, val);
                        }
                        CborValue::Text(s) => {
                            text_keys = true;
                            let s_bytes = s.as_bytes().to_vec();
                            if let Some(prev) = &last_text {
                                let prev_len = prev.len();
                                let s_len = s_bytes.len();
                                let ordered = if s_len != prev_len {
                                    s_len > prev_len
                                } else {
                                    s_bytes > *prev
                                };
                                if !ordered {
                                    return Err(DecodeError::NonCanonical);
                                }
                            }
                            last_text = Some(s_bytes);
                            map_text.insert(s, val);
                        }
                        _ => return Err(DecodeError::InvalidMapKey),
                    }
                }
                if text_keys {
                    Ok(CborValue::MapText(map_text))
                } else {
                    Ok(CborValue::Map(map_num))
                }
            }
            _ => Err(DecodeError::InvalidType),
        }
    }
}

pub fn validate_testimony_cbor(data: &[u8]) -> Result<(), DecodeError> {
    let mut dec = Decoder::new(data);
    let value = dec.decode()?;
    let map = match value {
        CborValue::Map(m) => m,
        _ => return Err(DecodeError::InvalidType),
    };

    // Required keys: 0,3,4,6,7,8,9 (id may be absent for pre-hash form)
    let required = [0u64, 3, 4, 6, 7, 8, 9];
    for k in required {
        if !map.contains_key(&k) {
            return Err(DecodeError::MissingField);
        }
    }

    // Basic type checks
    match map.get(&0) {
        Some(CborValue::Unsigned(v)) => {
            if *v != 3 {
                return Err(DecodeError::InvalidValue);
            }
        }
        _ => return Err(DecodeError::InvalidType),
    }
    match map.get(&3) {
        Some(CborValue::Unsigned(_)) => {}
        _ => return Err(DecodeError::InvalidType),
    }
    match map.get(&4) {
        Some(CborValue::Unsigned(v)) => {
            if *v != 1 && *v != 2 {
                return Err(DecodeError::InvalidValue);
            }
        }
        _ => return Err(DecodeError::InvalidType),
    }
    match map.get(&6) {
        Some(CborValue::Array(items)) => {
            for item in items {
                match item {
                    CborValue::Bytes(b) => {
                        if b.len() != 32 {
                            return Err(DecodeError::InvalidValue);
                        }
                    }
                    _ => return Err(DecodeError::InvalidType),
                }
            }
        }
        _ => return Err(DecodeError::InvalidType),
    }
    let claim_map = match map.get(&7) {
        Some(CborValue::Map(m)) => m,
        _ => return Err(DecodeError::InvalidType),
    };
    let context_map = match map.get(&8) {
        Some(CborValue::Map(m)) => m,
        _ => return Err(DecodeError::InvalidType),
    };
    let proof_map = match map.get(&9) {
        Some(CborValue::Map(m)) => m,
        _ => return Err(DecodeError::InvalidType),
    };

    // Optional id
    if let Some(val) = map.get(&1) {
        match val {
            CborValue::Bytes(b) => {
                if b.len() != 32 {
                    return Err(DecodeError::InvalidValue);
                }
            }
            _ => return Err(DecodeError::InvalidType),
        }
    }

    // Optional author
    if let Some(val) = map.get(&2) {
        match val {
            CborValue::Bytes(b) => {
                if b.len() != 32 {
                    return Err(DecodeError::InvalidValue);
                }
            }
            _ => return Err(DecodeError::InvalidType),
        }
    }

    // Optional prev_id
    if let Some(val) = map.get(&5) {
        match val {
            CborValue::Bytes(b) => {
                if b.len() != 32 {
                    return Err(DecodeError::InvalidValue);
                }
            }
            _ => return Err(DecodeError::InvalidType),
        }
    }

    // Claim validation
    let claim_verb = match claim_map.get(&0) {
        Some(CborValue::Text(s)) => {
            if s.is_empty() {
                return Err(DecodeError::InvalidValue);
            }
            s.as_str()
        }
        _ => return Err(DecodeError::InvalidType),
    };
    let claim_payload = match claim_map.get(&1) {
        Some(CborValue::Bytes(b)) => b.as_slice(),
        _ => return Err(DecodeError::InvalidType),
    };
    validate_claim_payload(claim_verb, claim_payload)?;

    // Context validation
    match context_map.get(&0) {
        Some(CborValue::Text(s)) => {
            let allowed = ["proximity", "beacon", "place", "vouch", "none"];
            if !allowed.contains(&s.as_str()) {
                return Err(DecodeError::InvalidValue);
            }
        }
        _ => return Err(DecodeError::InvalidType),
    }
    match context_map.get(&1) {
        Some(CborValue::Bytes(_)) => {}
        _ => return Err(DecodeError::InvalidType),
    }
    match context_map.get(&2) {
        Some(CborValue::Map(_)) => {}
        _ => return Err(DecodeError::InvalidType),
    }

    // Proof validation
    match proof_map.get(&0) {
        Some(CborValue::Unsigned(_)) => {}
        _ => return Err(DecodeError::InvalidType),
    }
    match proof_map.get(&1) {
        Some(CborValue::Array(items)) => {
            if items.is_empty() {
                return Err(DecodeError::InvalidValue);
            }
            for item in items {
                match item {
                    CborValue::Bytes(b) => {
                        if b.is_empty() {
                            return Err(DecodeError::InvalidValue);
                        }
                    }
                    _ => return Err(DecodeError::InvalidType),
                }
            }
        }
        _ => return Err(DecodeError::InvalidType),
    }
    match proof_map.get(&2) {
        Some(CborValue::Array(_)) => {}
        _ => return Err(DecodeError::InvalidType),
    }
    match proof_map.get(&3) {
        Some(CborValue::Array(_)) => {}
        _ => return Err(DecodeError::InvalidType),
    }

    Ok(())
}
