(module $test_rust_ea8702a3bb1d585901491e69ce53966b652b3ff0f4bb262e492dde9341f66195.wasm
  (type (;0;) (func (param i64) (result i64)))
  (func $entrypoint (;0;) (type 0) (param i64) (result i64)
    local.get 0
    i64.const -1
    i64.xor
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