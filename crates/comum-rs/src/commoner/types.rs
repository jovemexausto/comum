#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommonerErrorKind {
    Format,
    Proof,
    State,
}

#[derive(Debug, Clone)]
pub struct CommonerError {
    pub kind: CommonerErrorKind,
    pub message: String,
}

impl CommonerError {
    pub(crate) fn format(message: &str) -> Self {
        Self {
            kind: CommonerErrorKind::Format,
            message: message.to_string(),
        }
    }

    pub(crate) fn proof(message: &str) -> Self {
        Self {
            kind: CommonerErrorKind::Proof,
            message: message.to_string(),
        }
    }

    pub(crate) fn state(message: &str) -> Self {
        Self {
            kind: CommonerErrorKind::State,
            message: message.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProofInput {
    pub version: u64,
    pub signatures: Vec<Vec<u8>>,
    pub zk_proofs: Vec<Vec<u8>>,
    pub nullifiers: Vec<Vec<u8>>,
}

impl Default for ProofInput {
    fn default() -> Self {
        Self {
            version: 1,
            signatures: Vec::new(),
            zk_proofs: Vec::new(),
            nullifiers: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContextInput {
    pub r#type: String,
    pub payload_cbor: Vec<u8>,
    pub proof: ProofInput,
}

#[derive(Debug, Clone)]
pub struct Testimony {
    pub id: [u8; 32],
    pub id_hex: String,
    pub cbor: Vec<u8>,
}
