(module $gt_felt.wasm
  (type (;0;) (func (param f64 f64) (result i32)))
  (import "miden:prelude/intrinsics_felt" "gt" (func $miden_prelude::intrinsics::felt::extern_gt (;0;) (type 0)))
  (func $entrypoint (;1;) (type 0) (param f64 f64) (result i32)
    local.get 0
    local.get 1
    call $miden_prelude::intrinsics::felt::extern_gt
    i32.const 0
    i32.ne
  )
  (table (;0;) 1 1 funcref)
  (memory (;0;) 16)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (export "memory" (memory 0))
  (export "entrypoint" (func $entrypoint))
)