use std::fs;
use std::path::Path;

use comum_rs::{compute_id_hex, encode_testimony_without_id, Vector};

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
            assert_eq!(hex, hex::encode(cbor), "cbor mismatch for {}", vector.name);
        }
    }
}
