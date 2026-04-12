use std::fs;
use std::path::Path;

use comum_rs::run_capsule;

#[test]
fn run_agora_capsule_wat() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let wasm_path = root.join("apps/agora/capsules/agora.wasm");
    let wasm = fs::read(wasm_path).expect("read WASM");
    let result = run_capsule(&wasm).expect("run capsule");
    assert_eq!(result, 0);
}

#[test]
fn run_capsule_with_limits_test() {
    use comum_rs::run_capsule_with_limits;
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let wasm_path = root.join("apps/agora/capsules/agora.wasm");
    let wasm = fs::read(wasm_path).expect("read WASM");
    let result = run_capsule_with_limits(&wasm, 10_000).expect("run capsule with limits");
    assert_eq!(result, 0);
}
