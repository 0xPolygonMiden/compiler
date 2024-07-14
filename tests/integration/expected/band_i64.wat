(module $test_rust_e0a0c73e267170746d23527e7bebc6ba880b828e73598793b934a688e2bd8869.wasm
  (type (;0;) (func (param i64 i64) (result i64)))
  (func $entrypoint (;0;) (type 0) (param i64 i64) (result i64)
    local.get 1
    local.get 0
    i64.and
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