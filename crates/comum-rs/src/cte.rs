use crate::decoder::{CborValue, DecodeError, Decoder};
use crate::{encode_bstr, encode_map, encode_uint};

#[derive(Debug, Clone)]
pub struct Cte {
    pub cte_type: u64,
    pub version: u64,
    pub origin_hint: Option<Vec<u8>>,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct CteFragment {
    pub frag_id: [u8; 8],
    pub frag_index: u64,
    pub frag_total: u64,
    pub frag_payload: Vec<u8>,
}

pub fn encode_cte(cte: &Cte) -> Vec<u8> {
    let mut pairs = Vec::new();
    pairs.push([encode_uint(0), encode_uint(cte.cte_type)].concat());
    pairs.push([encode_uint(1), encode_uint(cte.version)].concat());
    if let Some(hint) = &cte.origin_hint {
        pairs.push([encode_uint(2), encode_bstr(hint)].concat());
    }
    pairs.push([encode_uint(3), encode_bstr(&cte.payload)].concat());
    encode_map(pairs)
}

pub fn decode_cte(data: &[u8]) -> Result<Cte, DecodeError> {
    let mut dec = Decoder::new(data);
    let value = dec.decode()?;
    let map = match value {
        CborValue::Map(m) => m,
        _ => return Err(DecodeError::InvalidType),
    };

    let cte_type = match map.get(&0) {
        Some(CborValue::Unsigned(v)) => *v,
        _ => return Err(DecodeError::InvalidType),
    };
    let version = match map.get(&1) {
        Some(CborValue::Unsigned(v)) => *v,
        _ => return Err(DecodeError::InvalidType),
    };
    let origin_hint = match map.get(&2) {
        Some(CborValue::Bytes(b)) => Some(b.clone()),
        None => None,
        _ => return Err(DecodeError::InvalidType),
    };
    let payload = match map.get(&3) {
        Some(CborValue::Bytes(b)) => b.clone(),
        _ => return Err(DecodeError::InvalidType),
    };

    Ok(Cte {
        cte_type,
        version,
        origin_hint,
        payload,
    })
}

pub fn fragment_cte(cte: &[u8], mtu: usize, frag_id: [u8; 8]) -> Vec<CteFragment> {
    let max_payload = mtu;
    let total = (cte.len() + max_payload - 1) / max_payload;
    let mut out = Vec::new();
    for i in 0..total {
        let start = i * max_payload;
        let end = std::cmp::min(start + max_payload, cte.len());
        out.push(CteFragment {
            frag_id,
            frag_index: i as u64,
            frag_total: total as u64,
            frag_payload: cte[start..end].to_vec(),
        });
    }
    out
}

pub fn encode_fragment(f: &CteFragment) -> Vec<u8> {
    let mut pairs = Vec::new();
    pairs.push([encode_uint(0), encode_bstr(&f.frag_id)].concat());
    pairs.push([encode_uint(1), encode_uint(f.frag_index)].concat());
    pairs.push([encode_uint(2), encode_uint(f.frag_total)].concat());
    pairs.push([encode_uint(3), encode_bstr(&f.frag_payload)].concat());
    encode_map(pairs)
}

pub fn reassemble_fragments(mut frags: Vec<CteFragment>) -> Result<Vec<u8>, DecodeError> {
    if frags.is_empty() {
        return Err(DecodeError::MissingField);
    }
    let total = frags[0].frag_total;
    frags.sort_by_key(|f| f.frag_index);
    if frags.len() as u64 != total {
        return Err(DecodeError::InvalidValue);
    }
    for (i, f) in frags.iter().enumerate() {
        if f.frag_index != i as u64 {
            return Err(DecodeError::InvalidValue);
        }
    }
    let mut out = Vec::new();
    for f in frags {
        out.extend_from_slice(&f.frag_payload);
    }
    Ok(out)
}

// No additional helpers
