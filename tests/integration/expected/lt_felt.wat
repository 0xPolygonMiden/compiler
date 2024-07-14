(module $lt_felt.wasm
  (type (;0;) (func (param f32 f32) (result i32)))
  (import "miden:stdlib/intrinsics_felt" "lt" (func $miden_stdlib_sys::intrinsics::felt::extern_lt (;0;) (type 0)))
  (func $entrypoint (;1;) (type 0) (param f32 f32) (result i32)
    local.get 1
    local.get 0
    call $miden_stdlib_sys::intrinsics::felt::extern_lt
    i32.const 0
    i32.ne
  )
  (table (;0;) 1 1 funcref)
  (memory (;0;) 16)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (export "memory" (memory 0))
  (export "entrypoint" (func $entrypoint))
)