#![cfg_attr(target_arch = "wasm32", no_std)]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use sha3::{Digest, Sha3_256};

#[cfg(target_arch = "wasm32")]
use core::panic::PanicInfo;

#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn compute_offer_id(
    item: &str,
    price: u64,
    currency: &str,
    expires: u64,
    seller: &str,
) -> [u8; 32] {
    let cbor = encode_offer_core(item, price, currency, expires, seller);
    let mut hasher = Sha3_256::new();
    hasher.update(&cbor);
    let digest = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest[..32]);
    out
}

pub fn build_offer_payload(
    item: &str,
    price: u64,
    currency: &str,
    expires: u64,
    seller: &str,
) -> Vec<u8> {
    let offer_id = compute_offer_id(item, price, currency, expires, seller);
    encode_offer_with_id(item, price, currency, expires, seller, &offer_id)
}

pub fn build_accept_payload(offer_id: &[u8; 32], buyer: &str) -> Vec<u8> {
    let mut pairs = Vec::new();
    // buyer(5) < offer_id(8)
    pairs.push([encode_tstr("buyer"), encode_tstr(buyer)].concat());
    pairs.push([encode_tstr("offer_id"), encode_bstr(offer_id)].concat());
    encode_map(pairs)
}

pub fn build_deliver_payload(offer_id: &[u8; 32], timestamp: u64) -> Vec<u8> {
    let mut pairs = Vec::new();
    // offer_id(8) < timestamp(9)
    pairs.push([encode_tstr("offer_id"), encode_bstr(offer_id)].concat());
    pairs.push([encode_tstr("timestamp"), encode_uint(timestamp)].concat());
    encode_map(pairs)
}

pub fn build_receipt_payload(offer_id: &[u8; 32], timestamp: u64) -> Vec<u8> {
    let mut pairs = Vec::new();
    // offer_id(8) < timestamp(9)
    pairs.push([encode_tstr("offer_id"), encode_bstr(offer_id)].concat());
    pairs.push([encode_tstr("timestamp"), encode_uint(timestamp)].concat());
    encode_map(pairs)
}

pub fn build_cancel_payload(offer_id: &[u8; 32], reason: &str) -> Vec<u8> {
    let mut pairs = Vec::new();
    // reason(6) < offer_id(8)
    pairs.push([encode_tstr("reason"), encode_tstr(reason)].concat());
    pairs.push([encode_tstr("offer_id"), encode_bstr(offer_id)].concat());
    encode_map(pairs)
}

pub fn build_dispute_payload(offer_id: &[u8; 32], reason: &str) -> Vec<u8> {
    let mut pairs = Vec::new();
    // reason(6) < offer_id(8)
    pairs.push([encode_tstr("reason"), encode_tstr(reason)].concat());
    pairs.push([encode_tstr("offer_id"), encode_bstr(offer_id)].concat());
    encode_map(pairs)
}

fn encode_offer_core(
    item: &str,
    price: u64,
    currency: &str,
    expires: u64,
    seller: &str,
) -> Vec<u8> {
    let mut pairs = Vec::new();
    // item(4) < price(5) < seller(6) < expires(7) < currency(8)
    pairs.push([encode_tstr("item"), encode_tstr(item)].concat());
    pairs.push([encode_tstr("price"), encode_uint(price)].concat());
    pairs.push([encode_tstr("seller"), encode_tstr(seller)].concat());
    pairs.push([encode_tstr("expires"), encode_uint(expires)].concat());
    pairs.push([encode_tstr("currency"), encode_tstr(currency)].concat());
    encode_map(pairs)
}

fn encode_offer_with_id(
    item: &str,
    price: u64,
    currency: &str,
    expires: u64,
    seller: &str,
    offer_id: &[u8; 32],
) -> Vec<u8> {
    let mut pairs = Vec::new();
    // item(4) < price(5) < seller(6) < expires(7) < currency(8) < offer_id(8)
    pairs.push([encode_tstr("item"), encode_tstr(item)].concat());
    pairs.push([encode_tstr("price"), encode_uint(price)].concat());
    pairs.push([encode_tstr("seller"), encode_tstr(seller)].concat());
    pairs.push([encode_tstr("expires"), encode_uint(expires)].concat());
    pairs.push([encode_tstr("currency"), encode_tstr(currency)].concat());
    pairs.push([encode_tstr("offer_id"), encode_bstr(offer_id)].concat());
    encode_map(pairs)
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
    if n < 4294967296 {
        return vec![
            0x1a,
            ((n >> 24) & 0xff) as u8,
            ((n >> 16) & 0xff) as u8,
            ((n >> 8) & 0xff) as u8,
            (n & 0xff) as u8,
        ];
    }
    vec![
        0x1b,
        ((n >> 56) & 0xff) as u8,
        ((n >> 48) & 0xff) as u8,
        ((n >> 40) & 0xff) as u8,
        ((n >> 32) & 0xff) as u8,
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
    if len < 65536 {
        let mut out = vec![0x59, ((len >> 8) & 0xff) as u8, (len & 0xff) as u8];
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
    if len < 65536 {
        let mut out = vec![0x79, ((len >> 8) & 0xff) as u8, (len & 0xff) as u8];
        out.extend_from_slice(data);
        return out;
    }
    panic!("tstr too long");
}

fn encode_map(pairs: Vec<Vec<u8>>) -> Vec<u8> {
    let len = pairs.len();
    let mut out = if len < 24 {
        vec![0xa0 + len as u8]
    } else if len < 256 {
        vec![0xb8, len as u8]
    } else if len < 65536 {
        vec![0xb9, ((len >> 8) & 0xff) as u8, (len & 0xff) as u8]
    } else {
        panic!("map too long")
    };
    for pair in pairs {
        out.extend_from_slice(&pair);
    }
    out
}

#[no_mangle]
pub extern "C" fn invoke() -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offer_id_is_stable_and_payload_builds() {
        let id_a = compute_offer_id("arroz", 10, "comum", 123, "did:comum:a");
        let id_b = compute_offer_id("arroz", 10, "comum", 123, "did:comum:a");
        assert_eq!(id_a, id_b);
        let payload = build_offer_payload("arroz", 10, "comum", 123, "did:comum:a");
        assert!(!payload.is_empty());
    }

    #[test]
    fn action_payloads_build() {
        let offer_id = compute_offer_id("cafe", 5, "comum", 999, "did:comum:s");
        assert!(!build_accept_payload(&offer_id, "did:comum:b").is_empty());
        assert!(!build_deliver_payload(&offer_id, 111).is_empty());
        assert!(!build_receipt_payload(&offer_id, 222).is_empty());
        assert!(!build_cancel_payload(&offer_id, "sem estoque").is_empty());
        assert!(!build_dispute_payload(&offer_id, "divergencia").is_empty());
    }
}
