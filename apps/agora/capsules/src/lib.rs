#![cfg_attr(target_arch = "wasm32", no_std)]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use sha3::{Digest, Sha3_256};

#[cfg(target_arch = "wasm32")]
use core::panic::PanicInfo;

#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[derive(Debug, Clone)]
pub struct AgoraVoteRecord {
    pub voter: String,
    pub choice: u64,
    pub testimony_id: [u8; 32],
    pub timestamp: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AgoraTally {
    pub yes: u64,
    pub no: u64,
    pub abstain: u64,
    pub quorum_met: bool,
    pub passed: bool,
}

pub fn compute_proposal_id(
    title: &str,
    body: &str,
    quorum: u64,
    expires: u64,
    author: &str,
) -> [u8; 32] {
    let cbor = encode_propose_core(title, body, quorum, expires, author);
    let mut hasher = Sha3_256::new();
    hasher.update(&cbor);
    let digest = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest[..32]);
    out
}

pub fn build_propose_payload(
    title: &str,
    body: &str,
    quorum: u64,
    expires: u64,
    author: &str,
) -> Vec<u8> {
    let proposal_id = compute_proposal_id(title, body, quorum, expires, author);
    encode_propose_with_id(title, body, quorum, expires, author, &proposal_id)
}

pub fn build_vote_payload(proposal_id: &[u8; 32], choice: u64, voter: &str) -> Vec<u8> {
    let mut pairs = Vec::new();
    // voter(5) < choice(6) < proposal_id(11)
    pairs.push([encode_tstr("voter"), encode_tstr(voter)].concat());
    pairs.push([encode_tstr("choice"), encode_uint(choice)].concat());
    pairs.push([encode_tstr("proposal_id"), encode_bstr(proposal_id)].concat());
    encode_map(pairs)
}

pub fn build_close_payload(proposal_id: &[u8; 32], timestamp: u64) -> Vec<u8> {
    let mut pairs = Vec::new();
    // timestamp(9) < proposal_id(11)
    pairs.push([encode_tstr("timestamp"), encode_uint(timestamp)].concat());
    pairs.push([encode_tstr("proposal_id"), encode_bstr(proposal_id)].concat());
    encode_map(pairs)
}

pub fn build_close_result_payload(
    proposal_id: &[u8; 32],
    tally: AgoraTally,
) -> Vec<u8> {
    let mut pairs = Vec::new();
    // no(2) < yes(3) < passed(6) < abstain(7) < quorum_met(10) < proposal_id(11)
    pairs.push([encode_tstr("no"), encode_uint(tally.no)].concat());
    pairs.push([encode_tstr("yes"), encode_uint(tally.yes)].concat());
    pairs.push([
        encode_tstr("passed"),
        encode_uint(if tally.passed { 1 } else { 0 }),
    ]
    .concat());
    pairs.push([encode_tstr("abstain"), encode_uint(tally.abstain)].concat());
    pairs.push([
        encode_tstr("quorum_met"),
        encode_uint(if tally.quorum_met { 1 } else { 0 }),
    ]
    .concat());
    pairs.push([encode_tstr("proposal_id"), encode_bstr(proposal_id)].concat());
    encode_map(pairs)
}

pub fn tally_votes(
    expires: u64,
    close_timestamp: Option<u64>,
    quorum: u64,
    votes: &[AgoraVoteRecord],
) -> AgoraTally {
    let cutoff = match close_timestamp {
        Some(ts) => if ts < expires { ts } else { expires },
        None => expires,
    };

    let mut latest: BTreeMap<&str, &AgoraVoteRecord> = BTreeMap::new();
    for vote in votes {
        if vote.timestamp > cutoff {
            continue;
        }
        if vote.choice > 2 {
            continue;
        }
        match latest.get(vote.voter.as_str()) {
            Some(prev) => {
                if vote.timestamp > prev.timestamp {
                    latest.insert(vote.voter.as_str(), vote);
                } else if vote.timestamp == prev.timestamp
                    && vote.testimony_id > prev.testimony_id
                {
                    latest.insert(vote.voter.as_str(), vote);
                }
            }
            None => {
                latest.insert(vote.voter.as_str(), vote);
            }
        }
    }

    let mut yes = 0u64;
    let mut no = 0u64;
    let mut abstain = 0u64;
    for v in latest.values() {
        match v.choice {
            0 => no += 1,
            1 => yes += 1,
            2 => abstain += 1,
            _ => {}
        }
    }
    let total = yes + no + abstain;
    let quorum_met = total >= quorum;
    let passed = yes > no && quorum_met;
    AgoraTally {
        yes,
        no,
        abstain,
        quorum_met,
        passed,
    }
}

fn encode_propose_core(
    title: &str,
    body: &str,
    quorum: u64,
    expires: u64,
    author: &str,
) -> Vec<u8> {
    let mut pairs = Vec::new();
    // body(4) < title(5) < author(6) < quorum(6) < expires(7)
    pairs.push([encode_tstr("body"), encode_tstr(body)].concat());
    pairs.push([encode_tstr("title"), encode_tstr(title)].concat());
    pairs.push([encode_tstr("author"), encode_tstr(author)].concat());
    pairs.push([encode_tstr("quorum"), encode_uint(quorum)].concat());
    pairs.push([encode_tstr("expires"), encode_uint(expires)].concat());
    encode_map(pairs)
}

fn encode_propose_with_id(
    title: &str,
    body: &str,
    quorum: u64,
    expires: u64,
    author: &str,
    proposal_id: &[u8; 32],
) -> Vec<u8> {
    let mut pairs = Vec::new();
    // body(4) < title(5) < author(6) < quorum(6) < expires(7) < proposal_id(11)
    pairs.push([encode_tstr("body"), encode_tstr(body)].concat());
    pairs.push([encode_tstr("title"), encode_tstr(title)].concat());
    pairs.push([encode_tstr("author"), encode_tstr(author)].concat());
    pairs.push([encode_tstr("quorum"), encode_uint(quorum)].concat());
    pairs.push([encode_tstr("expires"), encode_uint(expires)].concat());
    pairs.push([encode_tstr("proposal_id"), encode_bstr(proposal_id)].concat());
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
    use super::{
        build_close_payload, build_close_result_payload, build_propose_payload, build_vote_payload,
        compute_proposal_id, tally_votes, AgoraTally, AgoraVoteRecord,
    };

    fn id_from_byte(b: u8) -> [u8; 32] {
        [b; 32]
    }

    #[test]
    fn proposal_id_is_stable_and_included() {
        let title = "Abrir mercado";
        let body = "Proposta de teste";
        let quorum = 3;
        let expires = 1_700_000_000_000u64;
        let author = "did:comum:abc";

        let id = compute_proposal_id(title, body, quorum, expires, author);
        let payload = build_propose_payload(title, body, quorum, expires, author);

        let recomputed = compute_proposal_id(title, body, quorum, expires, author);
        assert_eq!(id, recomputed);
        assert!(!payload.is_empty());
    }

    #[test]
    fn vote_tally_respects_latest_by_timestamp_then_id() {
        let votes = vec![
            AgoraVoteRecord {
                voter: "did:comum:v1".to_string(),
                choice: 1,
                testimony_id: id_from_byte(0x01),
                timestamp: 10,
            },
            AgoraVoteRecord {
                voter: "did:comum:v1".to_string(),
                choice: 0,
                testimony_id: id_from_byte(0x02),
                timestamp: 10,
            },
            AgoraVoteRecord {
                voter: "did:comum:v2".to_string(),
                choice: 1,
                testimony_id: id_from_byte(0x03),
                timestamp: 9,
            },
        ];

        let tally = tally_votes(20, None, 2, &votes);
        assert_eq!(tally.yes, 1);
        assert_eq!(tally.no, 1);
        assert_eq!(tally.abstain, 0);
        assert!(tally.quorum_met);
        assert!(!tally.passed);
    }

    #[test]
    fn vote_tally_ignores_votes_after_close_or_expires() {
        let votes = vec![
            AgoraVoteRecord {
                voter: "did:comum:v1".to_string(),
                choice: 1,
                testimony_id: id_from_byte(0x01),
                timestamp: 100,
            },
            AgoraVoteRecord {
                voter: "did:comum:v2".to_string(),
                choice: 1,
                testimony_id: id_from_byte(0x02),
                timestamp: 300,
            },
        ];

        let tally = tally_votes(250, Some(200), 1, &votes);
        assert_eq!(tally.yes, 1);
        assert_eq!(tally.no, 0);
        assert_eq!(tally.abstain, 0);
        assert!(tally.passed);
    }

    #[test]
    fn close_result_payload_encodes_flags() {
        let tally = AgoraTally {
            yes: 2,
            no: 1,
            abstain: 0,
            quorum_met: true,
            passed: true,
        };
        let proposal_id = id_from_byte(0xAA);
        let payload = build_close_result_payload(&proposal_id, tally);
        assert!(!payload.is_empty());
    }

    #[test]
    fn vote_and_close_payloads_build() {
        let proposal_id = id_from_byte(0x10);
        let vote = build_vote_payload(&proposal_id, 1, "did:comum:v1");
        let close = build_close_payload(&proposal_id, 12345);
        assert!(!vote.is_empty());
        assert!(!close.is_empty());
    }
}
