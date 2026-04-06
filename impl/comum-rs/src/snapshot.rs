use crate::decoder::{CborValue, DecodeError, Decoder};
use crate::{encode_array, encode_bstr, encode_map, encode_tstr, encode_uint};
use sha3::{Digest, Sha3_256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpochSnapshot {
    pub epoch: u32,
    pub period_start: u64,
    pub period_end: u64,
    pub balances_root: [u8; 32],
    pub reputation_root: [u8; 32],
    pub nullifiers_root: [u8; 32],
    pub capsules_root: [u8; 32],
    pub prev_snapshot: [u8; 32],
    pub signatures: Vec<Vec<u8>>,
}

pub fn encode_epoch_snapshot(snapshot: &EpochSnapshot) -> Vec<u8> {
    let mut pairs = Vec::new();
    // epoch(5) < period_end(10) < signatures(10) < period_start(12)
    // balances_root(13) < capsules_root(13) < prev_snapshot(13)
    // nullifiers_root(15) < reputation_root(15)
    pairs.push([encode_tstr("epoch"), encode_uint(snapshot.epoch as u64)].concat());
    pairs.push([
        encode_tstr("period_end"),
        encode_uint(snapshot.period_end),
    ]
    .concat());
    let sigs = snapshot
        .signatures
        .iter()
        .map(|s| encode_bstr(s))
        .collect();
    pairs.push([encode_tstr("signatures"), encode_array(sigs)].concat());
    pairs.push([
        encode_tstr("period_start"),
        encode_uint(snapshot.period_start),
    ]
    .concat());
    pairs.push([
        encode_tstr("balances_root"),
        encode_bstr(&snapshot.balances_root),
    ]
    .concat());
    pairs.push([
        encode_tstr("capsules_root"),
        encode_bstr(&snapshot.capsules_root),
    ]
    .concat());
    pairs.push([
        encode_tstr("prev_snapshot"),
        encode_bstr(&snapshot.prev_snapshot),
    ]
    .concat());
    pairs.push([
        encode_tstr("nullifiers_root"),
        encode_bstr(&snapshot.nullifiers_root),
    ]
    .concat());
    pairs.push([
        encode_tstr("reputation_root"),
        encode_bstr(&snapshot.reputation_root),
    ]
    .concat());
    encode_map(pairs)
}

pub fn decode_epoch_snapshot(data: &[u8]) -> Result<EpochSnapshot, DecodeError> {
    let mut dec = Decoder::new(data);
    let value = dec.decode()?;
    let map = match value {
        CborValue::MapText(m) => m,
        _ => return Err(DecodeError::InvalidType),
    };

    let epoch = match map.get("epoch") {
        Some(CborValue::Unsigned(v)) => *v as u32,
        _ => return Err(DecodeError::InvalidType),
    };
    let period_start = match map.get("period_start") {
        Some(CborValue::Unsigned(v)) => *v,
        _ => return Err(DecodeError::InvalidType),
    };
    let period_end = match map.get("period_end") {
        Some(CborValue::Unsigned(v)) => *v,
        _ => return Err(DecodeError::InvalidType),
    };

    let balances_root = read_root(map.get("balances_root"))?;
    let reputation_root = read_root(map.get("reputation_root"))?;
    let nullifiers_root = read_root(map.get("nullifiers_root"))?;
    let capsules_root = read_root(map.get("capsules_root"))?;
    let prev_snapshot = read_root(map.get("prev_snapshot"))?;

    let signatures = match map.get("signatures") {
        Some(CborValue::Array(items)) => {
            let mut out = Vec::new();
            for item in items {
                match item {
                    CborValue::Bytes(b) => out.push(b.clone()),
                    _ => return Err(DecodeError::InvalidType),
                }
            }
            out
        }
        _ => return Err(DecodeError::InvalidType),
    };

    Ok(EpochSnapshot {
        epoch,
        period_start,
        period_end,
        balances_root,
        reputation_root,
        nullifiers_root,
        capsules_root,
        prev_snapshot,
        signatures,
    })
}

pub fn validate_epoch_snapshot_cbor(data: &[u8]) -> Result<(), DecodeError> {
    let snapshot = decode_epoch_snapshot(data)?;
    if snapshot.period_end < snapshot.period_start {
        return Err(DecodeError::InvalidValue);
    }
    Ok(())
}

pub fn compute_snapshot_id(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    let digest = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest[..32]);
    out
}

fn read_root(value: Option<&CborValue>) -> Result<[u8; 32], DecodeError> {
    match value {
        Some(CborValue::Bytes(b)) => {
            if b.len() != 32 {
                return Err(DecodeError::InvalidValue);
            }
            let mut out = [0u8; 32];
            out.copy_from_slice(b);
            Ok(out)
        }
        _ => Err(DecodeError::InvalidType),
    }
}
