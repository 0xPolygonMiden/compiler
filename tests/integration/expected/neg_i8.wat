(module $test_rust_c284c5e58a865bc45e93bf2024b2c204ef39187b90a2cf9cc04c023ea2f4fe6d.wasm
  (type (;0;) (func (param i32) (result i32)))
  (func $entrypoint (;0;) (type 0) (param i32) (result i32)
    i32.const 0
    local.get 0
    i32.sub
    i32.extend8_s
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