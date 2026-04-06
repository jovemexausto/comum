use wasmtime::{Engine, Linker, Module, Store};

pub fn run_capsule(wasm_bytes: &[u8]) -> Result<i32, String> {
    let engine = Engine::default();
    let module = Module::from_binary(&engine, wasm_bytes).map_err(|e| e.to_string())?;
    let mut store = Store::new(&engine, ());

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
