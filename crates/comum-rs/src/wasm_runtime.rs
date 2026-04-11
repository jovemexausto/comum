use wasmtime::{Config, Engine, Linker, Module, Store};

use crate::abi::{WASM_FUEL_DEFAULT, WASM_MAX_MEMORY_PAGES};

pub fn run_capsule(wasm_bytes: &[u8]) -> Result<i32, String> {
    let mut config = Config::new();
    config.consume_fuel(true);
    let engine = Engine::new(&config).map_err(|e| e.to_string())?;
    let module = Module::from_binary(&engine, wasm_bytes).map_err(|e| e.to_string())?;
    let mut store = Store::new(&engine, ());
    let _ = WASM_MAX_MEMORY_PAGES;
    store
        .set_fuel(WASM_FUEL_DEFAULT)
        .map_err(|e| e.to_string())?;

    let mut linker = Linker::new(&engine);

    // Stub syscalls
    linker
        .func_wrap("env", "read_graph", |_a: i32, _b: i32| -> i32 { 0 })
        .map_err(|e| e.to_string())?;
    linker
        .func_wrap("env", "verify_proof", |_a: i32, _b: i32, _c: i32, _d: i32| -> i32 { 0 })
        .map_err(|e| e.to_string())?;
    linker
        .func_wrap("env", "emit_testimony", |_a: i32, _b: i32| -> i32 { 0 })
        .map_err(|e| e.to_string())?;

    let instance = linker.instantiate(&mut store, &module).map_err(|e| e.to_string())?;
    let invoke = instance
        .get_typed_func::<(), i32>(&mut store, "invoke")
        .map_err(|e| e.to_string())?;
    let result = invoke.call(&mut store, ()).map_err(|e| e.to_string())?;
    Ok(result)
}

pub fn run_capsule_with_limits(wasm_bytes: &[u8], fuel: u64) -> Result<i32, String> {
    let mut config = Config::new();
    config.consume_fuel(true);
    let engine = Engine::new(&config).map_err(|e| e.to_string())?;
    let module = Module::from_binary(&engine, wasm_bytes).map_err(|e| e.to_string())?;
    let mut store = Store::new(&engine, ());
    let _ = WASM_MAX_MEMORY_PAGES;
    store.set_fuel(fuel).map_err(|e| e.to_string())?;

    let mut linker = Linker::new(&engine);
    linker
        .func_wrap("env", "read_graph", |_a: i32, _b: i32| -> i32 { 0 })
        .map_err(|e| e.to_string())?;
    linker
        .func_wrap("env", "verify_proof", |_a: i32, _b: i32, _c: i32, _d: i32| -> i32 { 0 })
        .map_err(|e| e.to_string())?;
    linker
        .func_wrap("env", "emit_testimony", |_a: i32, _b: i32| -> i32 { 0 })
        .map_err(|e| e.to_string())?;

    let instance = linker.instantiate(&mut store, &module).map_err(|e| e.to_string())?;
    let invoke = instance
        .get_typed_func::<(), i32>(&mut store, "invoke")
        .map_err(|e| e.to_string())?;
    let result = invoke.call(&mut store, ()).map_err(|e| e.to_string())?;
    Ok(result)
}
