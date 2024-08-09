(module $test_rust_df76c987a66b14b20c7ce818414abfb720a4e3e74eb3dc8015560af9bdb3fee4.wasm
  (type (;0;) (func (param i32) (result i32)))
  (func $entrypoint (;0;) (type 0) (param i32) (result i32)
    local.get 0
    i32.const 65535
    i32.xor
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