(module
  (type (;0;) (func (param i32) (result i32)))
  (func $fib (;0;) (type 0) (param i32) (result i32)
    loop (result i32) ;; label = @1
      block ;; label = @2
        local.get 0
        i32.const 3
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        return
      end
      local.get 0
      i32.const 3
      i32.div_u
      local.set 0
      br 0 (;@1;)
    end
  )
  (table (;0;) 1 1 funcref)
  (memory (;0;) 16)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (global (;1;) i32 i32.const 1048576)
  (global (;2;) i32 i32.const 1048576)
  (export "memory" (memory 0))
  (export "fib" (func $fib))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
)