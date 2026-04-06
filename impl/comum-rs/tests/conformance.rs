use std::fs;
use std::path::Path;

use comum_rs::{
    compute_id_hex, encode_testimony_without_id, validate_testimony_cbor, Vector,
    COMUM_TRANSFER,
};

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
        let value: serde_json::Value = serde_json::from_str(&data).unwrap();

        if value.get("testimony_without_id").is_some() {
            let vector: Vector = serde_json::from_value(value).unwrap();

            let cbor = encode_testimony_without_id(&vector.testimony_without_id);
            let id = compute_id_hex(&cbor);
            assert_eq!(id, vector.expected_id, "id mismatch for {}", vector.name);

            if let Some(hex) = vector.testimony_without_id_cbor_hex {
                assert_eq!(hex, hex::encode(&cbor), "cbor mismatch for {}", vector.name);
            }

            validate_testimony_cbor(&cbor).expect("invalid testimony cbor");
        } else if value.get("cte").is_some() {
            validate_cte_vector(name, &value);
        } else {
            panic!("unknown vector format: {}", name);
        }
    }
}

fn validate_cte_vector(name: &str, value: &serde_json::Value) {
    use comum_rs::{encode_cte, encode_fragment, fragment_cte, Cte};

    let cte_value = value.get("cte").expect("missing cte");
    let cte_type = cte_value["cte_type"].as_u64().expect("cte_type");
    let version = cte_value["version"].as_u64().expect("version");
    let payload_hex = cte_value["payload_hex"].as_str().expect("payload_hex");
    let payload = hex::decode(payload_hex).expect("payload_hex decode");
    let origin_hint = match cte_value.get("origin_hint_hex") {
        Some(v) => Some(hex::decode(v.as_str().expect("origin_hint_hex")).unwrap()),
        None => None,
    };
    let mtu = cte_value["mtu"].as_u64().expect("mtu") as usize;
    let frag_id_hex = cte_value["frag_id_hex"].as_str().expect("frag_id_hex");
    let frag_id_vec = hex::decode(frag_id_hex).expect("frag_id_hex decode");
    let frag_id: [u8; 8] = frag_id_vec.as_slice().try_into().expect("frag_id len");

    let cte = Cte {
        cte_type,
        version,
        origin_hint,
        payload,
    };
    let cbor = encode_cte(&cte);
    let expected_cbor_hex = value["cte_cbor_hex"].as_str().expect("cte_cbor_hex");
    assert_eq!(hex::encode(&cbor), expected_cbor_hex, "cte_cbor_hex mismatch for {}", name);

    let frags = fragment_cte(&cbor, mtu, frag_id);
    let fragments = value["fragments"].as_array().expect("fragments");
    assert_eq!(frags.len(), fragments.len(), "fragment count mismatch for {}", name);

    for (i, frag_value) in fragments.iter().enumerate() {
        let frag = &frags[i];
        let expected_index = frag_value["frag_index"].as_u64().expect("frag_index");
        let expected_total = frag_value["frag_total"].as_u64().expect("frag_total");
        let expected_payload_hex = frag_value["frag_payload_hex"].as_str().expect("frag_payload_hex");
        let expected_payload = hex::decode(expected_payload_hex).expect("frag_payload_hex decode");
        let expected_cbor_hex = frag_value["cbor_hex"].as_str().expect("cbor_hex");

        assert_eq!(frag.frag_index, expected_index, "frag_index mismatch for {}", name);
        assert_eq!(frag.frag_total, expected_total, "frag_total mismatch for {}", name);
        assert_eq!(frag.frag_payload, expected_payload, "frag_payload mismatch for {}", name);

        let frag_cbor = encode_fragment(frag);
        assert_eq!(hex::encode(&frag_cbor), expected_cbor_hex, "frag_cbor mismatch for {}", name);
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

#[test]
fn abi_constants_present() {
    use comum_rs::{WASM_EXECUTION_TIMEOUT_MS, WASM_FUEL_DEFAULT, WASM_MAX_MEMORY_PAGES};

    assert!(WASM_FUEL_DEFAULT > 0);
    assert!(WASM_MAX_MEMORY_PAGES > 0);
    assert!(WASM_EXECUTION_TIMEOUT_MS > 0);
}

#[test]
fn frost_threshold_signature_roundtrip() {
    use comum_rs::{frost_keygen_with_dealer, frost_sign, frost_verify};

    let (key_packages, pubkey_package) =
        frost_keygen_with_dealer(5, 3).expect("frost keygen");

    let mut signers: Vec<_> = key_packages.keys().cloned().collect();
    signers.sort();
    let signing_ids: Vec<_> = signers.into_iter().take(3).collect();

    let message = b"comum-genesis";
    let signature = frost_sign(message, &signing_ids, &key_packages, &pubkey_package)
        .expect("frost sign");

    assert!(frost_verify(message, &signature, &pubkey_package));
    assert!(!frost_verify(b"tampered", &signature, &pubkey_package));
}

#[test]
fn proximity_context_payload_roundtrip() {
    use comum_rs::{build_proximity_context_payload, validate_context_payload};

    let nonce = [0xAB; 16];
    let payload = build_proximity_context_payload("nfc", &nonce, 123);
    validate_context_payload("proximity", &payload).expect("valid proximity payload");
}

#[test]
fn context_payloads_roundtrip() {
    use comum_rs::{
        build_beacon_context_payload, build_place_context_payload, build_vouch_context_payload,
        derive_did, validate_context_payload,
    };

    let beacon_id = [0x11; 32];
    let token = [0xAA, 0xBB, 0xCC];
    let beacon_payload = build_beacon_context_payload(&beacon_id, &token, 456);
    validate_context_payload("beacon", &beacon_payload).expect("valid beacon payload");

    let place_hash = [0x22; 32];
    let place_payload = build_place_context_payload(&place_hash, 789);
    validate_context_payload("place", &place_payload).expect("valid place payload");

    let pk = [0x33; 32];
    let subject = derive_did(&pk);
    let community = [0x44; 32];
    let vouch_payload = build_vouch_context_payload(&subject, &community, 987);
    validate_context_payload("vouch", &vouch_payload).expect("valid vouch payload");
}

fn encode_uint(n: u64) -> Vec<u8> {
    if n < 24 {
        return vec![n as u8];
    }
    if n < 256 {
        return vec![0x18, n as u8];
    }
    if n < 65536 {
        return vec![0x19, ((n >> 8) & 0xff) as u8, (n & 0xff) as u8];
    }
    vec![
        0x1a,
        ((n >> 24) & 0xff) as u8,
        ((n >> 16) & 0xff) as u8,
        ((n >> 8) & 0xff) as u8,
        (n & 0xff) as u8,
    ]
}

fn encode_bstr(data: &[u8]) -> Vec<u8> {
    let len = data.len();
    if len < 24 {
        let mut out = vec![0x40 + len as u8];
        out.extend_from_slice(data);
        return out;
    }
    if len < 256 {
        let mut out = vec![0x58, len as u8];
        out.extend_from_slice(data);
        return out;
    }
    panic!("bstr too long");
}

fn encode_tstr(s: &str) -> Vec<u8> {
    let data = s.as_bytes();
    let len = data.len();
    if len < 24 {
        let mut out = vec![0x60 + len as u8];
        out.extend_from_slice(data);
        return out;
    }
    if len < 256 {
        let mut out = vec![0x78, len as u8];
        out.extend_from_slice(data);
        return out;
    }
    panic!("tstr too long");
}

fn encode_map(pairs: Vec<Vec<u8>>) -> Vec<u8> {
    let len = pairs.len();
    if len >= 24 {
        panic!("map too large");
    }
    let mut out = vec![0xa0 + len as u8];
    for pair in pairs {
        out.extend_from_slice(&pair);
    }
    out
}

fn encode_array(items: Vec<Vec<u8>>) -> Vec<u8> {
    let len = items.len();
    if len >= 24 {
        panic!("array too large");
    }
    let mut out = vec![0x80 + len as u8];
    for item in items {
        out.extend_from_slice(&item);
    }
    out
}

fn build_minimal_testimony(proof_signatures: Vec<Vec<u8>>, context_type: &str) -> Vec<u8> {
    let claim_map = encode_map(vec![
        [encode_uint(0), encode_tstr(COMUM_TRANSFER)].concat(),
        [encode_uint(1), encode_bstr(&[])].concat(),
    ]);

    let context_proof = encode_map(vec![
        [encode_uint(0), encode_uint(1)].concat(),
        [encode_uint(1), encode_array(vec![])].concat(),
        [encode_uint(2), encode_array(vec![])].concat(),
        [encode_uint(3), encode_array(vec![])].concat(),
    ]);

    let context_map = encode_map(vec![
        [encode_uint(0), encode_tstr(context_type)].concat(),
        [encode_uint(1), encode_bstr(&[])].concat(),
        [encode_uint(2), context_proof].concat(),
    ]);

    let sig_items = proof_signatures
        .into_iter()
        .map(|s| encode_bstr(&s))
        .collect();
    let proof_map = encode_map(vec![
        [encode_uint(0), encode_uint(1)].concat(),
        [encode_uint(1), encode_array(sig_items)].concat(),
        [encode_uint(2), encode_array(vec![])].concat(),
        [encode_uint(3), encode_array(vec![])].concat(),
    ]);

    encode_map(vec![
        [encode_uint(0), encode_uint(3)].concat(),
        [encode_uint(3), encode_uint(1)].concat(),
        [encode_uint(4), encode_uint(1)].concat(),
        [encode_uint(6), encode_array(vec![])].concat(),
        [encode_uint(7), claim_map].concat(),
        [encode_uint(8), context_map].concat(),
        [encode_uint(9), proof_map].concat(),
    ])
}

fn build_testimony_with_maps(
    refs: Vec<Vec<u8>>,
    claim_map: Vec<Vec<u8>>,
    context_map: Vec<Vec<u8>>,
    proof_map: Vec<Vec<u8>>,
) -> Vec<u8> {
    encode_map(vec![
        [encode_uint(0), encode_uint(3)].concat(),
        [encode_uint(3), encode_uint(1)].concat(),
        [encode_uint(4), encode_uint(1)].concat(),
        [encode_uint(6), encode_array(refs)].concat(),
        [encode_uint(7), encode_map(claim_map)].concat(),
        [encode_uint(8), encode_map(context_map)].concat(),
        [encode_uint(9), encode_map(proof_map)].concat(),
    ])
}

#[test]
fn context_payloads_invalid() {
    use comum_rs::validate_context_payload;

    let beacon_id = [0x11u8; 32];
    let beacon_payload = encode_map(vec![
        [encode_tstr("token"), encode_bstr(&[])].concat(),
        [encode_tstr("beacon_id"), encode_bstr(&beacon_id)].concat(),
        [encode_tstr("timestamp"), encode_uint(456)].concat(),
    ]);
    assert!(validate_context_payload("beacon", &beacon_payload).is_err());

    let place_hash = [0x22u8; 31];
    let place_payload = encode_map(vec![
        [encode_tstr("timestamp"), encode_uint(789)].concat(),
        [encode_tstr("place_hash"), encode_bstr(&place_hash)].concat(),
    ]);
    assert!(validate_context_payload("place", &place_payload).is_err());

    let community = [0x44u8; 32];
    let vouch_payload = encode_map(vec![
        [encode_tstr("subject"), encode_tstr("did:wrong:abc")].concat(),
        [encode_tstr("community"), encode_bstr(&community)].concat(),
        [encode_tstr("timestamp"), encode_uint(987)].concat(),
    ]);
    assert!(validate_context_payload("vouch", &vouch_payload).is_err());
}

#[test]
fn receive_payload_roundtrip() {
    use comum_rs::{build_receive_payload, validate_receive_payload};

    let of = [0x11; 32];
    let payload = build_receive_payload(&of, 1234);
    validate_receive_payload(&payload).expect("valid receive payload");
}

#[test]
fn receive_payload_invalid() {
    use comum_rs::validate_receive_payload;

    let of_short = [0x11u8; 31];
    let bad_of = encode_map(vec![
        [encode_tstr("of"), encode_bstr(&of_short)].concat(),
        [encode_tstr("timestamp"), encode_uint(1234)].concat(),
    ]);
    assert!(validate_receive_payload(&bad_of).is_err());

    let of = [0x11u8; 32];
    let bad_timestamp = encode_map(vec![
        [encode_tstr("of"), encode_bstr(&of)].concat(),
        [encode_tstr("timestamp"), encode_bstr(&[0x01, 0x02])].concat(),
    ]);
    assert!(validate_receive_payload(&bad_timestamp).is_err());

    let missing_of = encode_map(vec![[encode_tstr("timestamp"), encode_uint(1234)].concat()]);
    assert!(validate_receive_payload(&missing_of).is_err());
}

#[test]
fn genesis_payload_roundtrip() {
    use comum_rs::{build_genesis_payload, validate_genesis_payload};

    let founders = [
        "did:comum:alpha",
        "did:comum:bravo",
        "did:comum:charlie",
    ];
    let capsules = [[0x11u8; 32], [0x22u8; 32]];
    let mint_policy = [0x33u8; 32];
    let payload = build_genesis_payload(
        "Comum Demo",
        2,
        &founders,
        &capsules,
        0,
        &mint_policy,
    );
    validate_genesis_payload(&payload).expect("valid genesis payload");
}

#[test]
fn genesis_payload_invalid() {
    use comum_rs::validate_genesis_payload;

    let bad_founders = encode_array(vec![
        encode_tstr("did:comum:only"),
        encode_tstr("did:comum:two"),
    ]);
    let payload = encode_map(vec![
        [encode_tstr("name"), encode_tstr("Comum")].concat(),
        [encode_tstr("supply"), encode_uint(0)].concat(),
        [encode_tstr("capsules"), encode_array(vec![])].concat(),
        [encode_tstr("founders"), bad_founders].concat(),
        [encode_tstr("threshold"), encode_uint(2)].concat(),
        [encode_tstr("mint_policy"), encode_bstr(&[0x11; 32])].concat(),
    ]);
    assert!(validate_genesis_payload(&payload).is_err());

    let payload = encode_map(vec![
        [encode_tstr("name"), encode_tstr("Comum")].concat(),
        [encode_tstr("supply"), encode_uint(0)].concat(),
        [encode_tstr("capsules"), encode_array(vec![])].concat(),
        [
            encode_tstr("founders"),
            encode_array(vec![
                encode_tstr("did:comum:alpha"),
                encode_tstr("did:comum:bravo"),
                encode_tstr("did:comum:charlie"),
            ]),
        ]
        .concat(),
        [encode_tstr("threshold"), encode_uint(4)].concat(),
        [encode_tstr("mint_policy"), encode_bstr(&[0x11; 32])].concat(),
    ]);
    assert!(validate_genesis_payload(&payload).is_err());

    let payload = encode_map(vec![
        [encode_tstr("name"), encode_tstr("")].concat(),
        [encode_tstr("supply"), encode_uint(0)].concat(),
        [
            encode_tstr("capsules"),
            encode_array(vec![encode_bstr(&[0x22; 31])]),
        ]
        .concat(),
        [
            encode_tstr("founders"),
            encode_array(vec![
                encode_tstr("did:comum:alpha"),
                encode_tstr("did:comum:bravo"),
                encode_tstr("did:comum:charlie"),
            ]),
        ]
        .concat(),
        [encode_tstr("threshold"), encode_uint(2)].concat(),
        [encode_tstr("mint_policy"), encode_bstr(&[0x11; 31])].concat(),
    ]);
    assert!(validate_genesis_payload(&payload).is_err());
}

#[test]
fn security_rejects_empty_signatures() {
    let data = build_minimal_testimony(vec![], "proximity");
    assert!(validate_testimony_cbor(&data).is_err());
}

#[test]
fn security_rejects_unknown_context_type() {
    let sig = vec![0x11u8; 64];
    let data = build_minimal_testimony(vec![sig], "unknown");
    assert!(validate_testimony_cbor(&data).is_err());
}

#[test]
fn security_rejects_empty_signature_item() {
    let sigs = vec![vec![]];
    let data = build_minimal_testimony(sigs, "proximity");
    assert!(validate_testimony_cbor(&data).is_err());
}

#[test]
fn security_rejects_refs_wrong_length() {
    let claim_map = vec![
        [encode_uint(0), encode_tstr(COMUM_TRANSFER)].concat(),
        [encode_uint(1), encode_bstr(&[])].concat(),
    ];
    let context_proof = vec![
        [encode_uint(0), encode_uint(1)].concat(),
        [encode_uint(1), encode_array(vec![])].concat(),
        [encode_uint(2), encode_array(vec![])].concat(),
        [encode_uint(3), encode_array(vec![])].concat(),
    ];
    let context_map = vec![
        [encode_uint(0), encode_tstr("proximity")].concat(),
        [encode_uint(1), encode_bstr(&[])].concat(),
        [encode_uint(2), encode_map(context_proof)].concat(),
    ];
    let proof_map = vec![
        [encode_uint(0), encode_uint(1)].concat(),
        [encode_uint(1), encode_array(vec![encode_bstr(&[0x11; 64])])].concat(),
        [encode_uint(2), encode_array(vec![])].concat(),
        [encode_uint(3), encode_array(vec![])].concat(),
    ];
    let refs = vec![encode_bstr(&[0x22; 31])];
    let data = build_testimony_with_maps(refs, claim_map, context_map, proof_map);
    assert!(validate_testimony_cbor(&data).is_err());
}

#[test]
fn security_rejects_claim_payload_not_bstr() {
    let claim_map = vec![
        [encode_uint(0), encode_tstr(COMUM_TRANSFER)].concat(),
        [encode_uint(1), encode_uint(1)].concat(),
    ];
    let context_proof = vec![
        [encode_uint(0), encode_uint(1)].concat(),
        [encode_uint(1), encode_array(vec![])].concat(),
        [encode_uint(2), encode_array(vec![])].concat(),
        [encode_uint(3), encode_array(vec![])].concat(),
    ];
    let context_map = vec![
        [encode_uint(0), encode_tstr("proximity")].concat(),
        [encode_uint(1), encode_bstr(&[])].concat(),
        [encode_uint(2), encode_map(context_proof)].concat(),
    ];
    let proof_map = vec![
        [encode_uint(0), encode_uint(1)].concat(),
        [encode_uint(1), encode_array(vec![encode_bstr(&[0x11; 64])])].concat(),
        [encode_uint(2), encode_array(vec![])].concat(),
        [encode_uint(3), encode_array(vec![])].concat(),
    ];
    let data = build_testimony_with_maps(vec![], claim_map, context_map, proof_map);
    assert!(validate_testimony_cbor(&data).is_err());
}

#[test]
fn security_rejects_context_proof_not_map() {
    let claim_map = vec![
        [encode_uint(0), encode_tstr(COMUM_TRANSFER)].concat(),
        [encode_uint(1), encode_bstr(&[])].concat(),
    ];
    let context_map = vec![
        [encode_uint(0), encode_tstr("proximity")].concat(),
        [encode_uint(1), encode_bstr(&[])].concat(),
        [encode_uint(2), encode_bstr(&[0x01])].concat(),
    ];
    let proof_map = vec![
        [encode_uint(0), encode_uint(1)].concat(),
        [encode_uint(1), encode_array(vec![encode_bstr(&[0x11; 64])])].concat(),
        [encode_uint(2), encode_array(vec![])].concat(),
        [encode_uint(3), encode_array(vec![])].concat(),
    ];
    let data = build_testimony_with_maps(vec![], claim_map, context_map, proof_map);
    assert!(validate_testimony_cbor(&data).is_err());
}
