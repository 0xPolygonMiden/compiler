(module $test_rust_6c95bc4ac3c970051b695c8b0d81ce4e464588966d9047a4db06a19a2885319c.wasm
  (type (;0;) (func (param i32 i32) (result i32)))
  (func $entrypoint (;0;) (type 0) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.sub
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