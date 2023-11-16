(module
  (type (;0;) (func (param i32) (result i32)))
  (func $fib (;0;) (type 0) (param i32) (result i32)
    local.get 0
    i32.const 1000
    i32.lt_u
  )
  (memory (;0;) 16)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (global (;1;) i32 i32.const 1048576)
  (global (;2;) i32 i32.const 1048576)
  (export "memory" (memory 0))
  (export "fib" (func $fib))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
)