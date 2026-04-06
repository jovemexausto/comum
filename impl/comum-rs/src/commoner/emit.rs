use std::time::{SystemTime, UNIX_EPOCH};

use crate::{compute_id_hex, encode_testimony_without_id, sign_ed25519, TestimonyWithoutId};

use super::codec::encode_testimony_with_id;
use super::types::{CommonerError, ContextInput, Testimony};
use super::Commoner;

pub(crate) fn emit_testimony(
    commoner: &mut Commoner,
    verb: &str,
    payload_cbor: &[u8],
    context: ContextInput,
) -> Result<Testimony, CommonerError> {
    if verb.is_empty() {
        return Err(CommonerError::format("empty verb"));
    }

    let prev_id = commoner.last_ids.get(&commoner.author).map(hex::encode);
    let t = TestimonyWithoutId {
        version: 3,
        author: Some(hex::encode(commoner.author)),
        timestamp: now_ms(),
        suite: commoner.suite,
        prev_id,
        refs: Vec::new(),
        claim: crate::Claim {
            verb: verb.to_string(),
            payload_cbor_hex: hex::encode(payload_cbor),
        },
        context: crate::Context {
            r#type: context.r#type,
            payload_cbor_hex: hex::encode(context.payload_cbor),
            proof: crate::Proof {
                version: context.proof.version,
                signatures: context
                    .proof
                    .signatures
                    .iter()
                    .map(hex::encode)
                    .collect(),
                zk_proofs: context
                    .proof
                    .zk_proofs
                    .iter()
                    .map(hex::encode)
                    .collect(),
                nullifiers: context
                    .proof
                    .nullifiers
                    .iter()
                    .map(hex::encode)
                    .collect(),
            },
        },
        proof: crate::Proof {
            version: 1,
            signatures: Vec::new(),
            zk_proofs: Vec::new(),
            nullifiers: Vec::new(),
        },
    };

    let without_id_cbor = encode_testimony_without_id(&t);
    let id_hex = compute_id_hex(&without_id_cbor);
    let id_vec = hex::decode(&id_hex).expect("id hex");
    let id: [u8; 32] = id_vec.as_slice().try_into().expect("id length");

    let sig = sign_ed25519(&id, &commoner.sk);
    let nullifier = crate::compute_nullifier(&commoner.sk, &id);
    let proof = crate::Proof {
        version: 1,
        signatures: vec![hex::encode(sig)],
        zk_proofs: Vec::new(),
        nullifiers: vec![hex::encode(nullifier)],
    };

    let cbor = encode_testimony_with_id(&t, &id, &proof);

    commoner.last_ids.insert(commoner.author, id);
    commoner.clock = t.timestamp;
    commoner.known_ids.insert(id);
    commoner.known_nullifiers.insert(nullifier);
    commoner.store.push(cbor.clone());

    Ok(Testimony { id, id_hex, cbor })
}

fn now_ms() -> u64 {
    let dur = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    (dur.as_secs() * 1000) + (dur.subsec_millis() as u64)
}
