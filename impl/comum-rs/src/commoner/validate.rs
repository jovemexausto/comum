use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{validate_testimony_cbor, verify_ed25519};

use super::codec::{compute_id_from_decoded, decode_testimony, DecodedTestimony};
use super::types::CommonerError;
use super::Commoner;

pub(crate) fn validate_testimony(
    commoner: &Commoner,
    testimony_cbor: &[u8],
) -> Result<DecodedTestimony, CommonerError> {
    validate_testimony_cbor(testimony_cbor).map_err(|e| {
        CommonerError::format(&format!("cbor validation error: {:?}", e))
    })?;
    let decoded = decode_testimony(testimony_cbor)?;
    validate_decoded(
        &decoded,
        &commoner.supported_suites,
        &commoner.keybook,
        &commoner.known_ids,
        &commoner.known_nullifiers,
    )?;
    Ok(decoded)
}

fn validate_decoded(
    decoded: &DecodedTestimony,
    supported_suites: &[u64],
    keybook: &HashMap<[u8; 32], [u8; 32]>,
    known_ids: &HashSet<[u8; 32]>,
    known_nullifiers: &HashSet<[u8; 32]>,
) -> Result<(), CommonerError> {
    if !supported_suites.contains(&decoded.suite) {
        return Err(CommonerError::state("unsupported suite"));
    }

    if decoded.proof_version != 1 || decoded.context_proof_version != 1 {
        return Err(CommonerError::proof("invalid proof version"));
    }

    let author = match decoded.author {
        Some(a) => a,
        None => return Err(CommonerError::proof("author missing")),
    };
    let pk = match keybook.get(&author) {
        Some(k) => k,
        None => return Err(CommonerError::proof("unknown author key")),
    };

    let id = match decoded.id {
        Some(id) => id,
        None => return Err(CommonerError::format("missing id")),
    };
    let expected = compute_id_from_decoded(decoded);
    if id != expected {
        return Err(CommonerError::format("id mismatch"));
    }

    if decoded.signatures.is_empty() {
        return Err(CommonerError::proof("missing signatures"));
    }
    if !decoded
        .signatures
        .iter()
        .any(|sig| verify_ed25519(&id, sig, pk))
    {
        return Err(CommonerError::proof("invalid signature"));
    }

    if !context_proof_ok(&decoded.context_type, &decoded.context_signatures) {
        return Err(CommonerError::proof("invalid context proof"));
    }

    for nul in &decoded.nullifiers {
        if known_nullifiers.contains(nul) {
            return Err(CommonerError::state("nullifier already seen"));
        }
    }

    if let Some(prev) = decoded.prev_id {
        if !known_ids.contains(&prev) {
            return Err(CommonerError::state("prev_id not found"));
        }
    }
    for r in &decoded.refs {
        if !known_ids.contains(r) {
            return Err(CommonerError::state("ref not found"));
        }
    }

    let now = now_ms();
    if decoded.timestamp > now + 300_000 {
        return Err(CommonerError::state("timestamp too far in future"));
    }

    Ok(())
}

fn context_proof_ok(context_type: &str, signatures: &[[u8; 64]]) -> bool {
    if context_type == "proximity" || context_type == "none" {
        return true;
    }
    !signatures.is_empty()
}

fn now_ms() -> u64 {
    let dur = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    (dur.as_secs() * 1000) + (dur.subsec_millis() as u64)
}
