use std::fs;
use std::path::Path;

use comum_rs::run_capsule;

#[test]
fn run_agora_capsule_wat() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let wasm_path = root.join("impl/capsulas/agora/agora.wasm");
    let wasm = fs::read(wasm_path).expect("read WASM");
    let result = run_capsule(&wasm).expect("run capsule");
    assert_eq!(result, 0);
}
