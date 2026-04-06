mod codec;
mod emit;
mod sync;
mod types;
mod validate;

pub use types::{CommonerError, CommonerErrorKind, ContextInput, ProofInput, Testimony};

use std::collections::{HashMap, HashSet};

use ed25519_dalek::SigningKey;
use sha3::{Digest, Sha3_256};

use crate::{
    build_hello, build_request, encode_uint, fragment_cte, reassemble_fragments, Cte, CteFragment,
};

pub struct Commoner {
    sk: [u8; 32],
    author: [u8; 32],
    did: String,
    suite: u64,
    supported_suites: Vec<u64>,
    node_id: [u8; 32],
    clock: u64,
    known_ids: HashSet<[u8; 32]>,
    known_nullifiers: HashSet<[u8; 32]>,
    last_ids: HashMap<[u8; 32], [u8; 32]>,
    keybook: HashMap<[u8; 32], [u8; 32]>,
    store: Vec<Vec<u8>>,
}

impl Commoner {
    pub fn new(sk: [u8; 32], suite: u64) -> Self {
        let pk = SigningKey::from_bytes(&sk).verifying_key().to_bytes();
        let author = author_from_pk(&pk);
        let did = crate::derive_did(&pk);
        let mut keybook = HashMap::new();
        keybook.insert(author, pk);
        Self {
            sk,
            author,
            did,
            suite,
            supported_suites: vec![suite],
            node_id: author,
            clock: 0,
            known_ids: HashSet::new(),
            known_nullifiers: HashSet::new(),
            last_ids: HashMap::new(),
            keybook,
            store: Vec::new(),
        }
    }

    pub fn did(&self) -> String {
        self.did.clone()
    }

    pub fn clock(&self) -> u64 {
        self.clock
    }

    pub fn register_pk(&mut self, pk: [u8; 32]) -> [u8; 32] {
        let author = author_from_pk(&pk);
        self.keybook.insert(author, pk);
        author
    }

    pub fn add_supported_suite(&mut self, suite: u64) {
        if !self.supported_suites.contains(&suite) {
            self.supported_suites.push(suite);
        }
    }

    pub fn validate(&self, testimony_cbor: &[u8]) -> Result<(), CommonerError> {
        validate::validate_testimony(self, testimony_cbor).map(|_| ())
    }

    pub fn ingest(&mut self, testimony_cbor: &[u8]) -> Result<(), CommonerError> {
        let decoded = validate::validate_testimony(self, testimony_cbor)?;
        let id = decoded.id.unwrap();
        self.known_ids.insert(id);
        for nul in decoded.nullifiers {
            self.known_nullifiers.insert(nul);
        }
        if let Some(author) = decoded.author {
            self.last_ids.insert(author, id);
        }
        if decoded.timestamp > self.clock {
            self.clock = decoded.timestamp;
        }
        self.store.push(testimony_cbor.to_vec());
        Ok(())
    }

    pub fn emit(
        &mut self,
        verb: &str,
        payload_cbor: &[u8],
        context: ContextInput,
    ) -> Result<Testimony, CommonerError> {
        emit::emit_testimony(self, verb, payload_cbor, context)
    }

    pub fn build_hello(&self, profile: &str) -> Vec<u8> {
        let clock = encode_uint(self.clock);
        build_hello(&self.node_id, &self.supported_suites, &clock, profile)
    }

    pub fn build_request(&self, clock: u64, limit: u64) -> Vec<u8> {
        let since = encode_uint(clock);
        build_request("testimonies", &since, limit)
    }

    pub fn apply_response(&mut self, payload: &[u8]) -> Result<(), CommonerError> {
        sync::apply_response(self, payload)
    }

    pub fn encode_cte(&self, payload: &[u8]) -> Vec<u8> {
        let cte = Cte {
            cte_type: 2,
            version: 1,
            origin_hint: None,
            payload: payload.to_vec(),
        };
        crate::encode_cte(&cte)
    }

    pub fn fragment_cte(&self, cte: &[u8], mtu: usize, frag_id: [u8; 8]) -> Vec<CteFragment> {
        fragment_cte(cte, mtu, frag_id)
    }

    pub fn reassemble(&self, fragments: Vec<CteFragment>) -> Result<Vec<u8>, CommonerError> {
        reassemble_fragments(fragments)
            .map_err(|_| CommonerError::format("reassembly failed"))
    }
}

fn author_from_pk(pk: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha3_256::new();
    hasher.update(pk);
    let digest = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest[..32]);
    out
}
