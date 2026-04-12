// ABI formal para syscalls das Capsulas.
//
// Convencao de memoria:
// - buffers sao passados como (ptr, len) em i32.
// - ptr aponta para a memoria linear exportada como "memory".
// - retorno negativo indica erro.

pub const ERR_OK: i32 = 0;
pub const ERR_INVALID_PTR: i32 = -1;
pub const ERR_INVALID_LEN: i32 = -2;
pub const ERR_NOT_SUPPORTED: i32 = -3;

pub const SYSCALL_READ_GRAPH: &str = "read_graph";
pub const SYSCALL_VERIFY_PROOF: &str = "verify_proof";
pub const SYSCALL_EMIT_TESTIMONY: &str = "emit_testimony";

// Limites normativos (CIP-0001)
pub const WASM_MAX_MEMORY_PAGES: u64 = 16;
pub const WASM_FUEL_DEFAULT: u64 = 10_000;
pub const WASM_EXECUTION_TIMEOUT_MS: u64 = 200;

// Estrutura recomendada para chamadas:
// read_graph(query_ptr, query_len) -> result_ptr
// verify_proof(data_ptr, data_len, proof_ptr, proof_len) -> i32
// emit_testimony(claim_ptr, claim_len) -> testimony_id_ptr
