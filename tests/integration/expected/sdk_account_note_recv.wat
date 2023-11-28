(module
  (type (;0;) (func))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (param i32 i32 i32) (result i32)))
  (type (;3;) (func (param i32 i32)))
  (type (;4;) (func (result i32)))
  (import "env" "__wasm_call_dtors" (func $__wasm_call_dtors (;0;) (type 0)))
  (import "env" "__wasi_proc_exit" (func $__wasi_proc_exit (;1;) (type 1)))
  (import "env" "memcpy" (func $memcpy (;2;) (type 2)))
  (import "env" "miden::sat::account::add_asset" (func $miden::sat::account::add_asset (;3;) (type 3)))
  (import "env" "miden::sat::note::get_assets" (func $miden::sat::note::get_assets (;4;) (type 1)))
  (func $__wasm_call_ctors (;5;) (type 0))
  (func $_start (;6;) (type 0)
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
  (func $__main_void (;7;) (type 4) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    local.get 0
    i32.const 8
    i32.add
    call $miden::sat::note::get_assets
    i32.const 1048576
    local.get 0
    i32.const 8
    i32.add
    call $basic_wallet::MyWallet::receive_asset
    local.get 0
    i32.const 48
    i32.add
    global.set $__stack_pointer
    i32.const 0
  )
  (func $basic_wallet::MyWallet::receive_asset (;8;) (type 3) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 8
    i32.add
    local.get 1
    call $miden::account::add_asset
    local.get 2
    i32.const 48
    i32.add
    global.set $__stack_pointer
  )
  (func $miden::account::add_asset (;9;) (type 3) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 8
    i32.add
    local.get 1
    i32.const 40
    call $memcpy
    drop
    local.get 0
    local.get 2
    i32.const 8
    i32.add
    call $miden::sat::account::add_asset
    local.get 2
    i32.const 48
    i32.add
    global.set $__stack_pointer
  )
  (func $miden::sat::note::get_assets (;10;) (type 1) (param i32)
    local.get 0
    call $miden::sat::note::get_assets
  )
  (table (;0;) 1 1 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (export "memory" (memory 0))
  (export "_start" (func $_start))
  (export "__main_void" (func $__main_void))
  (data $.rodata (;0;) (i32.const 1048576) "")
)