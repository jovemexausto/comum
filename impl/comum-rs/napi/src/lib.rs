use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub fn encode_testimony(testimony_without_id_json: String) -> Result<String> {
    let v: serde_json::Value = serde_json::from_str(&testimony_without_id_json)
        .map_err(|e| Error::from_reason(e.to_string()))?;
    let t: comum_rs::TestimonyWithoutId = serde_json::from_value(v)
        .map_err(|e| Error::from_reason(e.to_string()))?;
    let cbor = comum_rs::encode_testimony_without_id(&t);
    let id = comum_rs::compute_id_hex(&cbor);
    let out = serde_json::json!({
        "cbor_hex": hex::encode(cbor),
        "id": id
    });
    Ok(out.to_string())
}
