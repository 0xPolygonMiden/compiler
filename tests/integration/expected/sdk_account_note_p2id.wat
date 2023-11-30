(module
  (type (;0;) (func))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (result i32)))
  (import "env" "__wasm_call_dtors" (func $__wasm_call_dtors (;0;) (type 0)))
  (import "env" "__wasi_proc_exit" (func $__wasi_proc_exit (;1;) (type 1)))
  (func $__wasm_call_ctors (;2;) (type 0))
  (func $_start (;3;) (type 0)
    (local i32)
    block ;; label = @1
      block ;; label = @2
        i32.const 0
        i32.load offset=1048576
        br_if 0 (;@2;)
        i32.const 0
        i32.const 1
        i32.store offset=1048576
        call $__wasm_call_ctors
        call $__main_void
        local.set 0
        call $__wasm_call_dtors
        local.get 0
        br_if 1 (;@1;)
        return
      end
      unreachable
      unreachable
    end
    local.get 0
    call $__wasi_proc_exit
    unreachable
  )
  (func $__main_void (;4;) (type 2) (result i32)
    i32.const 0
  )
  (table (;0;) 1 1 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (export "memory" (memory 0))
  (export "_start" (func $_start))
  (export "__main_void" (func $__main_void))
)