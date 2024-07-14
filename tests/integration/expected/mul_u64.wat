(module $test_rust_873d7388fcc4ee534ec2730026c14ba8e8b6f0fcb2133a11b18c2b4c23407376.wasm
  (type (;0;) (func (param i64 i64) (result i64)))
  (func $entrypoint (;0;) (type 0) (param i64 i64) (result i64)
    local.get 1
    local.get 0
    i64.mul
  )
  (memory (;0;) 16)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (global (;1;) i32 i32.const 1048576)
  (global (;2;) i32 i32.const 1048576)
  (export "memory" (memory 0))
  (export "entrypoint" (func $entrypoint))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
)