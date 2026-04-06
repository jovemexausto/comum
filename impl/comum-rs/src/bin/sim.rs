use std::collections::{HashMap, HashSet};

use comum_rs::{
    compute_id_hex, encode_testimony_without_id, validate_testimony_cbor, TestimonyWithoutId,
    COMUM_ENCOUNTER, COMUM_TRANSFER,
};
use sha3::{Digest, Sha3_256};

#[derive(Clone)]
struct Record {
    id: String,
    cbor: Vec<u8>,
    verb: String,
    author: String,
    to: Option<String>,
    amount: Option<u64>,
    refs: Vec<String>,
}

struct Node {
    name: String,
    records: Vec<Record>,
}

impl Node {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            records: Vec::new(),
        }
    }

    fn author_hex(&self) -> String {
        let mut hasher = Sha3_256::new();
        hasher.update(self.name.as_bytes());
        let digest = hasher.finalize();
        hex::encode(&digest[..32])
    }

    fn add(&mut self, record: Record) {
        validate_testimony_cbor(&record.cbor).expect("cbor invalido");
        self.records.push(record);
    }

    fn ids(&self) -> HashSet<String> {
        self.records.iter().map(|r| r.id.clone()).collect()
    }

    fn make_testimony(&self, verb: &str, to: Option<String>, amount: Option<u64>) -> Record {
        let t = TestimonyWithoutId {
            version: 3,
            author: Some(self.author_hex()),
            timestamp: 1730784000000,
            suite: 1,
            prev_id: None,
            refs: vec![],
            claim: comum_rs::Claim {
                verb: verb.to_string(),
                payload_cbor_hex: "a0".to_string(),
            },
            context: comum_rs::Context {
                r#type: "proximity".to_string(),
                payload_cbor_hex: "a0".to_string(),
                proof: comum_rs::Proof {
                    version: 1,
                    signatures: vec!["11".repeat(64)],
                    zk_proofs: vec![],
                    nullifiers: vec![],
                },
            },
            proof: comum_rs::Proof {
                version: 1,
                signatures: vec!["22".repeat(64)],
                zk_proofs: vec![],
                nullifiers: vec![],
            },
        };
        let cbor = encode_testimony_without_id(&t);
        let id = compute_id_hex(&cbor);
        Record {
            id,
            cbor,
            verb: verb.to_string(),
            author: self.author_hex(),
            to,
            amount,
            refs: vec![],
        }
    }
}

fn sync(a: &Node, b: &mut Node) -> (usize, usize) {
    let b_ids = b.ids();
    let mut count = 0;
    let mut skipped = 0;
    for r in &a.records {
        if !b_ids.contains(&r.id) {
            b.add(r.clone());
            count += 1;
        } else {
            skipped += 1;
        }
    }
    (count, skipped)
}

fn derive_balances(records: &[Record]) -> HashMap<String, i64> {
    let mut balances: HashMap<String, i64> = HashMap::new();
    for r in records {
        if r.verb == COMUM_TRANSFER {
            if let (Some(to), Some(amount)) = (&r.to, r.amount) {
                *balances.entry(r.author.clone()).or_insert(0) -= amount as i64;
                *balances.entry(to.clone()).or_insert(0) += amount as i64;
            }
        }
    }
    balances
}

fn short(hex: &str) -> String {
    if hex.len() <= 8 {
        return hex.to_string();
    }
    format!("{}...", &hex[..8])
}

fn print_state(label: &str, node: &Node, aliases: &HashMap<String, String>) {
    let balances = derive_balances(&node.records);
    let mut parts = Vec::new();
    for (k, v) in balances {
        let name = aliases.get(&k).cloned().unwrap_or_else(|| short(&k));
        parts.push(format!("{}:{:+}", name, v));
    }
    parts.sort();
    println!("[sim] estado {}: registros={}, saldos=[{}]", label, node.records.len(), parts.join(" "));
}

fn print_table(nodes: &[(&str, &Node)], aliases: &HashMap<String, String>) {
    println!("[sim] resumo final");
    println!("[sim] no  registros  saldos");
    for (name, node) in nodes {
        let balances = derive_balances(&node.records);
        let mut parts = Vec::new();
        for (k, v) in balances {
            let n = aliases.get(&k).cloned().unwrap_or_else(|| short(&k));
            parts.push(format!("{}:{:+}", n, v));
        }
        parts.sort();
        println!("[sim] {:<3} {:<9} {}", name, node.records.len(), parts.join(" "));
    }
}

fn main() {
    let mut a = Node::new("A");
    let mut b = Node::new("B");
    let mut c = Node::new("C");

    let a_did = a.author_hex();
    let b_did = b.author_hex();
    let c_did = c.author_hex();
    let mut aliases = HashMap::new();
    aliases.insert(a_did.clone(), "A".to_string());
    aliases.insert(b_did.clone(), "B".to_string());
    aliases.insert(c_did.clone(), "C".to_string());

    println!("[sim] ids: A={} B={} C={}", short(&a_did), short(&b_did), short(&c_did));

    println!("[sim] criando encounter A<->B");
    let encounter = a.make_testimony(COMUM_ENCOUNTER, None, None);
    println!(
        "[sim] encounter id={} author=A refs=0",
        short(&encounter.id)
    );
    // Link detectado: sync automatico A <-> B
    a.add(encounter.clone());
    b.add(encounter.clone());
    let (_n, _skipped) = sync(&a, &mut b);
    let (_n, _skipped) = sync(&b, &mut a);
    print_state("A", &a, &aliases);
    print_state("B", &b, &aliases);

    println!("[sim] A cria transferencia para B (5)");
    let mut transfer = a.make_testimony(COMUM_TRANSFER, Some(b_did.clone()), Some(5));
    transfer.refs = vec![encounter.id.clone()];
    println!(
        "[sim] transfer id={} de=A para=B valor=5 refs=[{}]",
        short(&transfer.id),
        short(&encounter.id)
    );
    a.add(transfer);
    // Link ativo: sync automatico A <-> B
    let (_n, _skipped) = sync(&a, &mut b);
    let (_n, _skipped) = sync(&b, &mut a);
    print_state("A", &a, &aliases);
    print_state("B", &b, &aliases);

    println!("[sim] sync A -> C");
    let (n, skipped) = sync(&a, &mut c);
    println!("[sim] C recebeu {} novos ({} repetidos)", n, skipped);
    print_state("C", &c, &aliases);

    println!("[sim] sync B -> C");
    let (n, skipped) = sync(&b, &mut c);
    println!("[sim] C recebeu {} novos ({} repetidos)", n, skipped);
    print_state("C", &c, &aliases);

    print_table(&[("A", &a), ("B", &b), ("C", &c)], &aliases);
}

#[cfg(test)]
mod tests {
    use super::{derive_balances, Record};

    #[test]
    fn balances_transfer() {
        let r = Record {
            id: "id".to_string(),
            cbor: vec![],
            verb: COMUM_TRANSFER.to_string(),
            author: "A".to_string(),
            to: Some("B".to_string()),
            amount: Some(10),
            refs: vec![],
        };
        let balances = derive_balances(&[r]);
        assert_eq!(balances.get("A"), Some(&-10));
        assert_eq!(balances.get("B"), Some(&10));
    }

    #[test]
    fn auto_sync_updates_receiver_balance() {
        use super::{sync, Node};

        let mut a = Node::new("A");
        let mut b = Node::new("B");
        let b_did = b.author_hex();

        let transfer = a.make_testimony(COMUM_TRANSFER, Some(b_did), Some(5));
        a.add(transfer);

        let (_n, _skipped) = sync(&a, &mut b);
        let balances = derive_balances(&b.records);
        assert_eq!(balances.get(&b.author_hex()), Some(&5));
    }
}
