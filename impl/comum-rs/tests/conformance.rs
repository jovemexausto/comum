use std::fs;
use std::path::Path;

use comum_rs::{compute_id_hex, encode_testimony_without_id, validate_testimony_cbor, Vector};

#[test]
fn vectors_match() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let manifest_path = root.join("spec/test-vectors/manifest.json");
    let manifest: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(manifest_path).unwrap()).unwrap();
    let vectors = manifest["vectors"].as_array().unwrap();

    for v in vectors {
        let name = v.as_str().unwrap();
        let path = root.join("spec/test-vectors").join(name);
        let data = fs::read_to_string(path).unwrap();
        let vector: Vector = serde_json::from_str(&data).unwrap();

        let cbor = encode_testimony_without_id(&vector.testimony_without_id);
        let id = compute_id_hex(&cbor);
        assert_eq!(id, vector.expected_id, "id mismatch for {}", vector.name);

        if let Some(hex) = vector.testimony_without_id_cbor_hex {
            assert_eq!(hex, hex::encode(&cbor), "cbor mismatch for {}", vector.name);
        }

        validate_testimony_cbor(&cbor).expect("invalid testimony cbor");
    }
}

#[test]
fn cte_roundtrip_and_fragmentation() {
    use comum_rs::{decode_cte, encode_cte, encode_fragment, fragment_cte, reassemble_fragments, Cte};

    let cte = Cte {
        cte_type: 2,
        version: 1,
        origin_hint: None,
        payload: vec![0xAA; 1200],
    };

    let encoded = encode_cte(&cte);
    let decoded = decode_cte(&encoded).expect("cte decode failed");
    assert_eq!(decoded.cte_type, 2);
    assert_eq!(decoded.version, 1);
    assert_eq!(decoded.origin_hint, None);
    assert_eq!(decoded.payload.len(), 1200);

    let frag_id = [0x01; 8];
    let frags = fragment_cte(&encoded, 500, frag_id);
    assert_eq!(frags.len(), 3);
    let frag_bytes = encode_fragment(&frags[0]);
    assert!(!frag_bytes.is_empty());
    let rebuilt = reassemble_fragments(frags).expect("reassembly failed");
    assert_eq!(rebuilt, encoded);
}

#[test]
fn decoder_rejects_noncanonical_map_key_order() {
    use comum_rs::validate_testimony_cbor;

    // Map with 2 pairs, but keys out of order: 3 then 0
    let data = vec![0xA2, 0x03, 0x01, 0x00, 0x01];
    let res = validate_testimony_cbor(&data);
    assert!(res.is_err());
}

#[test]
fn sync_payloads_build() {
    use comum_rs::{build_hello, build_hello_ack, build_request, build_response, decode_payload_kv};

    let node_id = [0x01; 32];
    let clock = [0x02; 4];
    let hello = build_hello(&node_id, &[1, 2], &clock, "LIGHT");
    assert!(!hello.is_empty());

    let session_id = [0xAA; 8];
    let ack = build_hello_ack(1, &session_id);
    assert!(!ack.is_empty());

    let req = build_request("testimonies", b"epoch:1", 50);
    assert!(!req.is_empty());

    let resp = build_response(&[vec![0xA0]]);
    assert!(!resp.is_empty());

    let _ = decode_payload_kv(&hello).expect("decode hello");
    let _ = decode_payload_kv(&ack).expect("decode ack");
    let _ = decode_payload_kv(&req).expect("decode req");
    let _ = decode_payload_kv(&resp).expect("decode resp");
}

#[test]
fn ed25519_sign_and_verify() {
    use comum_rs::{sign_ed25519, verify_ed25519};

    let sk: [u8; 32] = [0x42; 32];
    let message = b"comum-protocol";
    let sig = sign_ed25519(message, &sk);

    // Derive public key from signing key using dalek
    let pk = ed25519_dalek::SigningKey::from_bytes(&sk).verifying_key().to_bytes();

    assert!(verify_ed25519(message, &sig, &pk));
    assert!(!verify_ed25519(b"tampered", &sig, &pk));
}

#[test]
fn did_derivation_and_key_rotate_payload() {
    use comum_rs::{build_key_rotate_payload, derive_did, validate_key_rotate_payload};

    let pk: [u8; 32] = [0x01; 32];
    let did = derive_did(&pk);
    assert!(did.starts_with("did:comum:"));

    let community: [u8; 32] = [0xAA; 32];
    let old_pk = [0x11u8; 32];
    let new_pk = [0x22u8; 32];
    let payload = build_key_rotate_payload(&community, &old_pk, &new_pk, 123456);
    validate_key_rotate_payload(&payload).expect("valid key_rotate payload");
}
