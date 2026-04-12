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

pub fn compute_task_id(
    title: &str,
    details: &str,
    reward: u64,
    expires: u64,
    creator: &str,
) -> [u8; 32] {
    let cbor = encode_task_core(title, details, reward, expires, creator);
    let mut hasher = Sha3_256::new();
    hasher.update(&cbor);
    let digest = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest[..32]);
    out
}

pub fn build_task_payload(
    title: &str,
    details: &str,
    reward: u64,
    expires: u64,
    creator: &str,
) -> Vec<u8> {
    let task_id = compute_task_id(title, details, reward, expires, creator);
    encode_task_with_id(title, details, reward, expires, creator, &task_id)
}

pub fn build_commit_payload(task_id: &[u8; 32], worker: &str) -> Vec<u8> {
    let mut pairs = Vec::new();
    // worker(6) < task_id(7)
    pairs.push([encode_tstr("worker"), encode_tstr(worker)].concat());
    pairs.push([encode_tstr("task_id"), encode_bstr(task_id)].concat());
    encode_map(pairs)
}

pub fn build_checkin_payload(task_id: &[u8; 32], timestamp: u64) -> Vec<u8> {
    let mut pairs = Vec::new();
    // task_id(7) < timestamp(9)
    pairs.push([encode_tstr("task_id"), encode_bstr(task_id)].concat());
    pairs.push([encode_tstr("timestamp"), encode_uint(timestamp)].concat());
    encode_map(pairs)
}

pub fn build_complete_payload(task_id: &[u8; 32], timestamp: u64) -> Vec<u8> {
    let mut pairs = Vec::new();
    // task_id(7) < timestamp(9)
    pairs.push([encode_tstr("task_id"), encode_bstr(task_id)].concat());
    pairs.push([encode_tstr("timestamp"), encode_uint(timestamp)].concat());
    encode_map(pairs)
}

pub fn build_reward_payload(task_id: &[u8; 32], amount: u64, timestamp: u64) -> Vec<u8> {
    let mut pairs = Vec::new();
    // amount(6) < task_id(7) < timestamp(9)
    pairs.push([encode_tstr("amount"), encode_uint(amount)].concat());
    pairs.push([encode_tstr("task_id"), encode_bstr(task_id)].concat());
    pairs.push([encode_tstr("timestamp"), encode_uint(timestamp)].concat());
    encode_map(pairs)
}

fn encode_task_core(
    title: &str,
    details: &str,
    reward: u64,
    expires: u64,
    creator: &str,
) -> Vec<u8> {
    let mut pairs = Vec::new();
    // title(5) < reward(6) < creator(7) < details(7) < expires(7)
    pairs.push([encode_tstr("title"), encode_tstr(title)].concat());
    pairs.push([encode_tstr("reward"), encode_uint(reward)].concat());
    pairs.push([encode_tstr("creator"), encode_tstr(creator)].concat());
    pairs.push([encode_tstr("details"), encode_tstr(details)].concat());
    pairs.push([encode_tstr("expires"), encode_uint(expires)].concat());
    encode_map(pairs)
}

fn encode_task_with_id(
    title: &str,
    details: &str,
    reward: u64,
    expires: u64,
    creator: &str,
    task_id: &[u8; 32],
) -> Vec<u8> {
    let mut pairs = Vec::new();
    // title(5) < reward(6) < creator(7) < details(7) < expires(7) < task_id(7)
    pairs.push([encode_tstr("title"), encode_tstr(title)].concat());
    pairs.push([encode_tstr("reward"), encode_uint(reward)].concat());
    pairs.push([encode_tstr("creator"), encode_tstr(creator)].concat());
    pairs.push([encode_tstr("details"), encode_tstr(details)].concat());
    pairs.push([encode_tstr("expires"), encode_uint(expires)].concat());
    pairs.push([encode_tstr("task_id"), encode_bstr(task_id)].concat());
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
    fn task_id_is_stable_and_payload_builds() {
        let id_a = compute_task_id("reparo", "telhado", 5, 1000, "did:comum:c");
        let id_b = compute_task_id("reparo", "telhado", 5, 1000, "did:comum:c");
        assert_eq!(id_a, id_b);
        let payload = build_task_payload("reparo", "telhado", 5, 1000, "did:comum:c");
        assert!(!payload.is_empty());
    }

    #[test]
    fn action_payloads_build() {
        let task_id = compute_task_id("mutirao", "limpeza", 3, 10, "did:comum:m");
        assert!(!build_commit_payload(&task_id, "did:comum:w").is_empty());
        assert!(!build_checkin_payload(&task_id, 10).is_empty());
        assert!(!build_complete_payload(&task_id, 20).is_empty());
        assert!(!build_reward_payload(&task_id, 2, 30).is_empty());
    }
}
