use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub fn encode_testimony(testimony_without_id_json: String) -> Result<String> {
    let v: serde_json::Value = serde_json::from_str(&testimony_without_id_json)
        .map_err(|e: serde_json::Error| Error::from_reason(e.to_string()))?;
    let t: comum_rs::TestimonyWithoutId = serde_json::from_value(v)
        .map_err(|e: serde_json::Error| Error::from_reason(e.to_string()))?;
    let cbor = comum_rs::encode_testimony_without_id(&t);
    let id = comum_rs::compute_id_hex(&cbor);
    let out = serde_json::json!({
        "cbor_hex": hex::encode(cbor),
        "id": id
    });
    Ok(out.to_string())
}

#[napi(object)]
pub struct ProofInput {
    pub version: u32,
    pub signatures: Vec<Buffer>,
    pub zk_proofs: Vec<Buffer>,
    pub nullifiers: Vec<Buffer>,
}

#[napi(object)]
pub struct ContextInput {
    pub r#type: String,
    pub payload_cbor: Buffer,
    pub proof: ProofInput,
}

#[napi(object)]
pub struct EmitResult {
    pub id_hex: String,
    pub cbor: Buffer,
}

#[napi(object)]
pub struct CteFragment {
    pub frag_id: Buffer,
    pub frag_index: u32,
    pub frag_total: u32,
    pub frag_payload: Buffer,
}

#[napi]
pub struct Commoner {
    inner: comum_rs::Commoner,
}

#[napi]
impl Commoner {
    #[napi(constructor)]
    pub fn new(sk: Buffer, suite: u32) -> Result<Self> {
        if sk.len() != 32 {
            return Err(Error::from_reason("invalid sk length"));
        }
        let mut sk_bytes = [0u8; 32];
        sk_bytes.copy_from_slice(&sk);
        Ok(Self {
            inner: comum_rs::Commoner::new(sk_bytes, suite as u64),
        })
    }

    #[napi]
    pub fn did(&self) -> String {
        self.inner.did()
    }

    #[napi]
    pub fn clock(&self) -> i64 {
        self.inner.clock() as i64
    }

    #[napi]
    pub fn register_pk(&mut self, pk: Buffer) -> Result<Buffer> {
        if pk.len() != 32 {
            return Err(Error::from_reason("invalid pk length"));
        }
        let mut pk_bytes = [0u8; 32];
        pk_bytes.copy_from_slice(&pk);
        let author = self.inner.register_pk(pk_bytes);
        Ok(Buffer::from(author.to_vec()))
    }

    #[napi]
    pub fn add_supported_suite(&mut self, suite: u32) {
        self.inner.add_supported_suite(suite as u64)
    }

    #[napi]
    pub fn validate(&self, testimony_cbor: Buffer) -> Result<()> {
        self.inner
            .validate(&testimony_cbor)
            .map_err(map_commoner_error)
    }

    #[napi]
    pub fn ingest(&mut self, testimony_cbor: Buffer) -> Result<()> {
        self.inner
            .ingest(&testimony_cbor)
            .map_err(map_commoner_error)
    }

    #[napi]
    pub fn emit(&mut self, verb: String, payload_cbor: Buffer, context: ContextInput) -> Result<EmitResult> {
        let ctx = comum_rs::ContextInput {
            r#type: context.r#type,
            payload_cbor: context.payload_cbor.to_vec(),
            proof: comum_rs::ProofInput {
                version: context.proof.version as u64,
                signatures: context
                    .proof
                    .signatures
                    .into_iter()
                    .map(|b| b.to_vec())
                    .collect(),
                zk_proofs: context
                    .proof
                    .zk_proofs
                    .into_iter()
                    .map(|b| b.to_vec())
                    .collect(),
                nullifiers: context
                    .proof
                    .nullifiers
                    .into_iter()
                    .map(|b| b.to_vec())
                    .collect(),
            },
        };
        let out = self
            .inner
            .emit(&verb, &payload_cbor, ctx)
            .map_err(map_commoner_error)?;
        Ok(EmitResult {
            id_hex: out.id_hex,
            cbor: Buffer::from(out.cbor),
        })
    }

    #[napi]
    pub fn build_hello(&self, profile: String) -> Buffer {
        Buffer::from(self.inner.build_hello(&profile))
    }

    #[napi]
    pub fn build_request(&self, clock: i64, limit: u32) -> Result<Buffer> {
        let clock_u64 = to_u64("clock", clock)?;
        Ok(Buffer::from(self.inner.build_request(clock_u64, limit as u64)))
    }

    #[napi]
    pub fn apply_response(&mut self, payload: Buffer) -> Result<()> {
        self.inner.apply_response(&payload).map_err(map_commoner_error)
    }

    #[napi]
    pub fn encode_cte(&self, payload: Buffer) -> Buffer {
        Buffer::from(self.inner.encode_cte(&payload))
    }

    #[napi]
    pub fn fragment_cte(&self, cte: Buffer, mtu: u32, frag_id: Buffer) -> Result<Vec<CteFragment>> {
        if frag_id.len() != 8 {
            return Err(Error::from_reason("invalid frag_id length"));
        }
        let mut frag = [0u8; 8];
        frag.copy_from_slice(&frag_id);
        let frags = self.inner.fragment_cte(&cte, mtu as usize, frag);
        Ok(frags
            .into_iter()
            .map(|f| CteFragment {
                frag_id: Buffer::from(f.frag_id.to_vec()),
                frag_index: f.frag_index as u32,
                frag_total: f.frag_total as u32,
                frag_payload: Buffer::from(f.frag_payload),
            })
            .collect())
    }

    #[napi]
    pub fn reassemble(&self, fragments: Vec<CteFragment>) -> Result<Buffer> {
        let mut frags = Vec::new();
        for f in fragments {
            if f.frag_id.len() != 8 {
                return Err(Error::from_reason("invalid frag_id length"));
            }
            let mut frag_id = [0u8; 8];
            frag_id.copy_from_slice(&f.frag_id);
            frags.push(comum_rs::CteFragment {
                frag_id,
                frag_index: f.frag_index as u64,
                frag_total: f.frag_total as u64,
                frag_payload: f.frag_payload.to_vec(),
            });
        }
        let out = self.inner.reassemble(frags).map_err(map_commoner_error)?;
        Ok(Buffer::from(out))
    }
}

fn map_commoner_error(err: comum_rs::CommonerError) -> Error {
    Error::from_reason(format!("{:?}: {}", err.kind, err.message))
}

fn to_u64(label: &str, value: i64) -> Result<u64> {
    if value < 0 {
        return Err(Error::from_reason(format!("invalid {}", label)));
    }
    Ok(value as u64)
}
