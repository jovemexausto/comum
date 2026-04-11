use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("uso: wat2wasm <input.wat> <output.wasm>");
        std::process::exit(2);
    }

    let input = &args[1];
    let output = &args[2];
    let wat = fs::read_to_string(input).expect("ler WAT");
    let wasm = wat::parse_str(&wat).expect("parse WAT");
    fs::write(output, wasm).expect("escrever WASM");
}
