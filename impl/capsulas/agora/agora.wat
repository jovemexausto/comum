(module
  (import "env" "read_graph" (func $read_graph (param i32 i32) (result i32)))
  (import "env" "verify_proof" (func $verify_proof (param i32 i32 i32 i32) (result i32)))
  (import "env" "emit_testimony" (func $emit_testimony (param i32 i32) (result i32)))

  (memory (export "memory") 1)

  ;; invoke() -> i32
  (func (export "invoke") (result i32)
    ;; placeholder implementation
    i32.const 0
  )
)
