(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (type (;1;) (func (result i32)))
  (func $add (;0;) (type 0) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.add
  )
  (func $__main (;1;) (type 1) (result i32)
    i32.const 1
    i32.const 2
    call $add
  )
  (memory (;0;) 16)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (global (;1;) i32 i32.const 1048576)
  (global (;2;) i32 i32.const 1048576)
  (export "memory" (memory 0))
  (export "add" (func $add))
  (export "__main" (func $__main))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
)