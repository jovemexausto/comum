use comum_rs::{
    build_response, decode_epoch_snapshot, encode_epoch_snapshot, validate_epoch_snapshot_cbor,
    Commoner, ContextInput, EpochSnapshot, ProofInput, COMUM_TRANSFER,
};

fn root(byte: u8) -> [u8; 32] {
    [byte; 32]
}

#[test]
fn snapshot_roundtrip_and_validate() {
    let snapshot = EpochSnapshot {
        epoch: 1,
        period_start: 1_700_000_000_000,
        period_end: 1_700_086_400_000,
        balances_root: root(0x01),
        reputation_root: root(0x02),
        nullifiers_root: root(0x03),
        capsules_root: root(0x04),
        prev_snapshot: root(0x05),
        signatures: vec![vec![0xAA; 64], vec![0xBB; 64]],
    };

    let cbor = encode_epoch_snapshot(&snapshot);
    validate_epoch_snapshot_cbor(&cbor).expect("snapshot validation failed");
    let decoded = decode_epoch_snapshot(&cbor).expect("snapshot decode failed");
    assert_eq!(decoded, snapshot);
}

#[test]
fn snapshot_rejects_bad_period() {
    let snapshot = EpochSnapshot {
        epoch: 2,
        period_start: 200,
        period_end: 100,
        balances_root: root(0x01),
        reputation_root: root(0x02),
        nullifiers_root: root(0x03),
        capsules_root: root(0x04),
        prev_snapshot: root(0x05),
        signatures: vec![vec![0xAA; 64]],
    };

    let cbor = encode_epoch_snapshot(&snapshot);
    assert!(validate_epoch_snapshot_cbor(&cbor).is_err());
}

#[test]
fn snapshot_response_ingest() {
    let snapshot = EpochSnapshot {
        epoch: 3,
        period_start: 10,
        period_end: 20,
        balances_root: root(0x10),
        reputation_root: root(0x11),
        nullifiers_root: root(0x12),
        capsules_root: root(0x13),
        prev_snapshot: root(0x14),
        signatures: vec![vec![0xCC; 64]],
    };
    let cbor = encode_epoch_snapshot(&snapshot);
    let response = build_response(&[cbor]);

    let mut node = Commoner::new([0x11u8; 32], 1);
    node.apply_snapshot_response(&response).expect("apply snapshot response");
    assert_eq!(node.snapshot_count(), 1);
}

#[test]
fn prune_before_removes_all() {
    let mut node = Commoner::new([0x11u8; 32], 1);
    let ctx = ContextInput {
        r#type: "none".to_string(),
        payload_cbor: vec![0xa0],
        proof: ProofInput::default(),
    };
    node.emit(COMUM_TRANSFER, &[], ctx.clone()).expect("emit");
    node.emit(COMUM_TRANSFER, &[], ctx).expect("emit");
    assert_eq!(node.testimony_count(), 2);

    let removed = node.prune_before(u64::MAX).expect("prune");
    assert_eq!(removed, 2);
    assert_eq!(node.testimony_count(), 0);
}

#[test]
fn prune_to_snapshot_uses_period_start() {
    let mut node = Commoner::new([0x22u8; 32], 1);
    let ctx = ContextInput {
        r#type: "none".to_string(),
        payload_cbor: vec![0xa0],
        proof: ProofInput::default(),
    };
    node.emit(COMUM_TRANSFER, &[], ctx.clone()).expect("emit");
    node.emit(COMUM_TRANSFER, &[], ctx).expect("emit");
    assert_eq!(node.testimony_count(), 2);

    let snapshot = EpochSnapshot {
        epoch: 9,
        period_start: u64::MAX,
        period_end: u64::MAX,
        balances_root: root(0x01),
        reputation_root: root(0x02),
        nullifiers_root: root(0x03),
        capsules_root: root(0x04),
        prev_snapshot: root(0x05),
        signatures: vec![vec![0xAA; 64]],
    };
    let cbor = encode_epoch_snapshot(&snapshot);
    let removed = node.prune_to_snapshot(&cbor).expect("prune snapshot");
    assert_eq!(removed, 2);
    assert_eq!(node.testimony_count(), 0);
}
