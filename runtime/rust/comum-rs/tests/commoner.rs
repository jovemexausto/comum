use comum_rs::{
    Commoner, CommonerErrorKind, ContextInput, ProofInput, COMUM_IDENTITY_COMMITMENT,
    COMUM_ATTEST, COMUM_TRANSFER,
};

fn empty_map() -> Vec<u8> {
    vec![0xa0]
}

#[test]
fn commoner_emit_and_ingest_roundtrip() {
    let sk_a = [0x11u8; 32];
    let mut a = Commoner::new(sk_a, 1);

    let ctx = ContextInput {
        r#type: "proximity".to_string(),
        payload_cbor: empty_map(),
        proof: ProofInput::default(),
    };

    let testimony = a
        .emit(COMUM_TRANSFER, &empty_map(), ctx)
        .expect("emit");

    let sk_b = [0x22u8; 32];
    let mut b = Commoner::new(sk_b, 1);
    b.register_pk(a_pk_from_sk(sk_a));

    b.validate(&testimony.cbor).expect("validate");
    b.ingest(&testimony.cbor).expect("ingest");
}

#[test]
fn commoner_rejects_empty_context_proof_for_vouch() {
    let sk = [0x33u8; 32];
    let mut c = Commoner::new(sk, 1);

    let ctx = ContextInput {
        r#type: "vouch".to_string(),
        payload_cbor: empty_map(),
        proof: ProofInput::default(),
    };

    let testimony = c
        .emit(COMUM_ATTEST, &empty_map(), ctx)
        .expect("emit");

    let err = c.validate(&testimony.cbor).expect_err("should fail");
    assert_eq!(err.kind, CommonerErrorKind::Proof);
}

#[test]
fn commoner_requires_known_author_key() {
    let sk_a = [0x44u8; 32];
    let mut a = Commoner::new(sk_a, 1);
    let ctx = ContextInput {
        r#type: "proximity".to_string(),
        payload_cbor: empty_map(),
        proof: ProofInput::default(),
    };
    let testimony = a
        .emit(COMUM_TRANSFER, &empty_map(), ctx)
        .expect("emit");

    let sk_b = [0x55u8; 32];
    let b = Commoner::new(sk_b, 1);
    let err = b.validate(&testimony.cbor).expect_err("should fail");
    assert_eq!(err.kind, CommonerErrorKind::Proof);
}

#[test]
fn commoner_rejects_invalid_identity_commitment_payload_on_emit() {
    let sk = [0x66u8; 32];
    let mut c = Commoner::new(sk, 1);
    let ctx = ContextInput {
        r#type: "proximity".to_string(),
        payload_cbor: empty_map(),
        proof: ProofInput::default(),
    };

    let err = c
        .emit(COMUM_IDENTITY_COMMITMENT, &empty_map(), ctx)
        .expect_err("should fail");
    assert_eq!(err.kind, CommonerErrorKind::Format);
}

fn a_pk_from_sk(sk: [u8; 32]) -> [u8; 32] {
    ed25519_dalek::SigningKey::from_bytes(&sk)
        .verifying_key()
        .to_bytes()
}
