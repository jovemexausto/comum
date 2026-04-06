use crate::decoder::{CborValue, Decoder};

use super::types::CommonerError;
use super::Commoner;

pub(crate) fn apply_response(commoner: &mut Commoner, payload: &[u8]) -> Result<(), CommonerError> {
    let mut dec = Decoder::new(payload);
    let value = dec
        .decode()
        .map_err(|e| CommonerError::format(&format!("decode error: {:?}", e)))?;
    let map = match value {
        CborValue::MapText(m) => m,
        _ => return Err(CommonerError::format("invalid response payload")),
    };
    let items = match map.get("items") {
        Some(CborValue::Array(items)) => items,
        _ => return Err(CommonerError::format("missing items")),
    };
    for item in items {
        match item {
            CborValue::Bytes(b) => {
                commoner.ingest(b)?;
            }
            _ => return Err(CommonerError::format("invalid item type")),
        }
    }
    Ok(())
}
