(module $neg_felt.wasm
  (type (;0;) (func (param f64 f64) (result f64)))
  (import "miden:stdlib/intrinsics_felt" "sub" (func $miden_stdlib_sys::intrinsics::felt::extern_sub (;0;) (type 0)))
  (func $entrypoint (;1;) (type 0) (param f64 f64) (result f64)
    local.get 0
    local.get 1
    call $miden_stdlib_sys::intrinsics::felt::extern_sub
  )
  (table (;0;) 1 1 funcref)
  (memory (;0;) 16)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (export "memory" (memory 0))
  (export "entrypoint" (func $entrypoint))
)