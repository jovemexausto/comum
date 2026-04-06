use std::io::{self, Read};

use comum_rs::{compute_id_hex, encode_testimony_without_id, TestimonyWithoutId};
use serde_json::Value;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let v: Value = serde_json::from_str(&input).expect("invalid json");

    let testimony_value = if v.get("testimony_without_id").is_some() {
        v.get("testimony_without_id").unwrap().clone()
    } else {
        v
    };

    let testimony: TestimonyWithoutId =
        serde_json::from_value(testimony_value).expect("invalid testimony");
    let cbor = encode_testimony_without_id(&testimony);
    let id = compute_id_hex(&cbor);

    let out = serde_json::json!({
        "cbor_hex": hex::encode(cbor),
        "id": id
    });
    println!("{}", out);
}
