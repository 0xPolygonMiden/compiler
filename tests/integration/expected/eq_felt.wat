(module $eq_felt.wasm
  (type (;0;) (func (param f64 f64) (result i32)))
  (import "miden:prelude/intrinsics_felt" "eq" (func $miden_prelude::intrinsics::felt::extern_eq (;0;) (type 0)))
  (func $entrypoint (;1;) (type 0) (param f64 f64) (result i32)
    local.get 0
    local.get 1
    call $miden_prelude::intrinsics::felt::extern_eq
    i32.const 1
    i32.eq
  )
  (table (;0;) 1 1 funcref)
  (memory (;0;) 16)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (export "memory" (memory 0))
  (export "entrypoint" (func $entrypoint))
)