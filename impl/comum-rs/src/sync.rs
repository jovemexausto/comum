use std::collections::BTreeMap;

use crate::decoder::{CborValue, DecodeError, Decoder};
use crate::{encode_array, encode_bstr, encode_map, encode_tstr, encode_uint};

pub fn build_hello(node_id: &[u8; 32], suites: &[u64], clock: &[u8], profile: &str) -> Vec<u8> {
    let mut pairs = Vec::new();
    pairs.push([encode_tstr("clock"), encode_bstr(clock)].concat());
    let suite_items = suites.iter().map(|s| encode_uint(*s)).collect();
    pairs.push([encode_tstr("suites"), encode_array(suite_items)].concat());
    pairs.push([encode_tstr("node_id"), encode_bstr(node_id)].concat());
    pairs.push([encode_tstr("profile"), encode_tstr(profile)].concat());
    encode_map(pairs)
}

pub fn build_hello_ack(suite: u64, session_id: &[u8; 8]) -> Vec<u8> {
    let mut pairs = Vec::new();
    pairs.push([encode_tstr("suite"), encode_uint(suite)].concat());
    pairs.push([encode_tstr("session_id"), encode_bstr(session_id)].concat());
    encode_map(pairs)
}

pub fn build_request(want: &str, since: &[u8], limit: u64) -> Vec<u8> {
    let mut pairs = Vec::new();
    pairs.push([encode_tstr("want"), encode_tstr(want)].concat());
    pairs.push([encode_tstr("limit"), encode_uint(limit)].concat());
    pairs.push([encode_tstr("since"), encode_bstr(since)].concat());
    encode_map(pairs)
}

pub fn build_response(items: &[Vec<u8>]) -> Vec<u8> {
    let mut pairs = Vec::new();
    let encoded_items = items.iter().map(|i| encode_bstr(i)).collect();
    pairs.push([encode_tstr("items"), encode_array(encoded_items)].concat());
    encode_map(pairs)
}

pub fn decode_payload_kv(data: &[u8]) -> Result<BTreeMap<String, CborValue>, DecodeError> {
    let mut dec = Decoder::new(data);
    let value = dec.decode()?;
    match value {
        CborValue::MapText(m) => Ok(m),
        _ => Err(DecodeError::InvalidType),
    }
}
