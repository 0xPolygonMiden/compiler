(module $test_rust_aaaa272f39ec3f234fd119869db2d6791ce4b1e8d3f284fe6c212db33f480554.wasm
  (type (;0;) (func (param i32 i32) (result i32)))
  (func $entrypoint (;0;) (type 0) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.le_u
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