(module $rust_sdk_basic_wallet.wasm
  (type (;0;) (func (param f64 f64 f64 f64 i32)))
  (type (;1;) (func (param f64 f64 f64 f64 f64 f64 f64 f64 f64 f64) (result f64)))
  (type (;2;) (func (param i32)))
  (type (;3;) (func (param i32 f64 f64 i32)))
  (type (;4;) (func (param i32 i32)))
  (type (;5;) (func (param i32 f64 f64 i32) (result f64)))
  (import "miden::account" "add_asset<0x0000000000000000000000000000000000000000000000000000000000000000>" (func $miden_sdk_tx_kernel::externs::extern_account_add_asset (;0;) (type 0)))
  (import "miden::account" "remove_asset<0x0000000000000000000000000000000000000000000000000000000000000000>" (func $miden_sdk_tx_kernel::externs::extern_account_remove_asset (;1;) (type 0)))
  (import "miden::tx" "create_note<0x0000000000000000000000000000000000000000000000000000000000000000>" (func $miden_sdk_tx_kernel::externs::extern_tx_create_note (;2;) (type 1)))
  (func $receive_asset (;3;) (type 2) (param i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 1
    local.get 0
    call $miden_sdk_tx_kernel::add_asset
    local.get 1
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $send_asset (;4;) (type 3) (param i32 f64 f64 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    local.get 4
    local.get 0
    call $miden_sdk_tx_kernel::remove_asset
    local.get 4
    local.get 1
    local.get 2
    local.get 3
    call $miden_sdk_tx_kernel::create_note
    drop
    local.get 4
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $miden_sdk_tx_kernel::add_asset (;5;) (type 4) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 1
    f64.load
    local.get 1
    f64.load offset=8
    local.get 1
    f64.load offset=16
    local.get 1
    f64.load offset=24
    local.get 2
    call $miden_sdk_tx_kernel::externs::extern_account_add_asset
    local.get 0
    i32.const 24
    i32.add
    local.get 2
    i32.const 24
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 16
    i32.add
    local.get 2
    i32.const 16
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 8
    i32.add
    local.get 2
    i32.const 8
    i32.add
    i64.load
    i64.store
    local.get 0
    local.get 2
    i64.load
    i64.store
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $miden_sdk_tx_kernel::remove_asset (;6;) (type 4) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 1
    f64.load
    local.get 1
    f64.load offset=8
    local.get 1
    f64.load offset=16
    local.get 1
    f64.load offset=24
    local.get 2
    call $miden_sdk_tx_kernel::externs::extern_account_remove_asset
    local.get 0
    i32.const 24
    i32.add
    local.get 2
    i32.const 24
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 16
    i32.add
    local.get 2
    i32.const 16
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 8
    i32.add
    local.get 2
    i32.const 8
    i32.add
    i64.load
    i64.store
    local.get 0
    local.get 2
    i64.load
    i64.store
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $miden_sdk_tx_kernel::create_note (;7;) (type 5) (param i32 f64 f64 i32) (result f64)
    local.get 0
    f64.load
    local.get 0
    f64.load offset=8
    local.get 0
    f64.load offset=16
    local.get 0
    f64.load offset=24
    local.get 1
    local.get 2
    local.get 3
    f64.load
    local.get 3
    f64.load offset=8
    local.get 3
    f64.load offset=16
    local.get 3
    f64.load offset=24
    call $miden_sdk_tx_kernel::externs::extern_tx_create_note
  )
  (table (;0;) 1 1 funcref)
  (memory (;0;) 16)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (export "memory" (memory 0))
  (export "receive_asset" (func $receive_asset))
  (export "send_asset" (func $send_asset))
)