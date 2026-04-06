use comum_rs::{decode_epoch_snapshot, encode_epoch_snapshot, validate_epoch_snapshot_cbor, EpochSnapshot};

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
