(module $8c8a3ab08866a8cdf13f461d225bc40cc694eab7818133d8e81630fc58c00003.wasm
  (type (;0;) (func (param i32 i32) (result i32)))
  (func $entrypoint (;0;) (type 0) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.or
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