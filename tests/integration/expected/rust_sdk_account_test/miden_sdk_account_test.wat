(module $miden_sdk_account_test.wasm
  (type (;0;) (func (result f64)))
  (type (;1;) (func (param i64) (result f64)))
  (type (;2;) (func (param f64 f64) (result f64)))
  (type (;3;) (func (param f64 f64 f64 f64 i32)))
  (type (;4;) (func (param i32 i32)))
  (type (;5;) (func (param i32 i64 i64 i64 i64)))
  (import "miden:tx_kernel/account" "get_id<0x0000000000000000000000000000000000000000000000000000000000000000>" (func $miden_sdk_tx_kernel::extern_account_get_id (;0;) (type 0)))
  (import "miden:types/felt" "from_u64_unchecked" (func $miden_sdk_types::extern_felt_from_u64_unchecked (;1;) (type 1)))
  (import "miden:types/felt" "add" (func $miden_sdk_types::extern_felt_add (;2;) (type 2)))
  (import "miden:tx_kernel/account" "add_asset<0x0000000000000000000000000000000000000000000000000000000000000000>" (func $miden_sdk_tx_kernel::extern_account_add_asset (;3;) (type 3)))
  (func $get_wallet_magic_number (;4;) (type 0) (result f64)
    (local f64)
    call $miden_sdk_tx_kernel::extern_account_get_id
    local.set 0
    i64.const 42
    call $miden_sdk_types::extern_felt_from_u64_unchecked
    local.get 0
    call $miden_sdk_types::extern_felt_add
  )
  (func $test_add_asset (;5;) (type 0) (result f64)
    (local i32 f64)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    local.get 0
    i64.const 1
    i64.const 2
    i64.const 3
    i64.const 4
    call $miden_sdk_types::Word::from_u64_unchecked
    local.get 0
    i32.const 32
    i32.add
    local.get 0
    call $miden_sdk_tx_kernel::add_assets
    local.get 0
    f64.load offset=32
    local.set 1
    local.get 0
    i32.const 64
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $miden_sdk_tx_kernel::add_assets (;6;) (type 4) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 64
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
    call $miden_sdk_tx_kernel::extern_account_add_asset
    local.get 2
    i32.const 32
    i32.add
    local.get 2
    i64.load
    local.get 2
    i32.const 8
    i32.add
    i64.load
    local.get 2
    i32.const 16
    i32.add
    i64.load
    local.get 2
    i32.const 24
    i32.add
    i64.load
    call $miden_sdk_types::Word::from_u64_unchecked
    local.get 0
    i32.const 24
    i32.add
    local.get 2
    i32.const 32
    i32.add
    i32.const 24
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 16
    i32.add
    local.get 2
    i32.const 32
    i32.add
    i32.const 16
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 8
    i32.add
    local.get 2
    i32.const 32
    i32.add
    i32.const 8
    i32.add
    i64.load
    i64.store
    local.get 0
    local.get 2
    i64.load offset=32
    i64.store
    local.get 2
    i32.const 64
    i32.add
    global.set $__stack_pointer
  )
  (func $miden_sdk_types::Word::from_u64_unchecked (;7;) (type 5) (param i32 i64 i64 i64 i64)
    (local f64 f64 f64)
    local.get 1
    call $miden_sdk_types::extern_felt_from_u64_unchecked
    local.set 5
    local.get 2
    call $miden_sdk_types::extern_felt_from_u64_unchecked
    local.set 6
    local.get 3
    call $miden_sdk_types::extern_felt_from_u64_unchecked
    local.set 7
    local.get 0
    local.get 4
    call $miden_sdk_types::extern_felt_from_u64_unchecked
    f64.store offset=24
    local.get 0
    local.get 7
    f64.store offset=16
    local.get 0
    local.get 6
    f64.store offset=8
    local.get 0
    local.get 5
    f64.store
  )
  (table (;0;) 1 1 funcref)
  (memory (;0;) 16)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (export "memory" (memory 0))
  (export "get_wallet_magic_number" (func $get_wallet_magic_number))
  (export "test_add_asset" (func $test_add_asset))
)