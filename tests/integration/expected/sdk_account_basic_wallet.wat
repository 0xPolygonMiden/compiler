(module
  (type (;0;) (func (param i32 i32 i32) (result i32)))
  (type (;1;) (func (param i32 i32)))
  (type (;2;) (func (param i32)))
  (type (;3;) (func (param i32 i64 i64 i64 i64)))
  (type (;4;) (func (param i32 i32 i32)))
  (type (;5;) (func (param i64) (result i64)))
  (import "env" "memcpy" (func $memcpy (;0;) (type 0)))
  (import "env" "miden::sat::account::add_asset" (func $miden::sat::account::add_asset (;1;) (type 1)))
  (func $rust_begin_unwind (;2;) (type 2) (param i32)
    loop ;; label = @1
      br 0 (;@1;)
    end
  )
  (func $basic_wallet::MyWallet::receive_asset (;3;) (type 1) (param i32 i32)
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
  (func $my_wallet::receive_asset (;4;) (type 3) (param i32 i64 i64 i64 i64)
    (local i32)
    global.get $__stack_pointer
    i32.const 80
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    block ;; label = @1
      i64.const 1
      call $miden::note::Tag::new
      local.get 1
      i64.eq
      br_if 0 (;@1;)
      local.get 5
      i32.const 44
      i32.add
      i64.const 0
      i64.store align=4
      local.get 5
      i32.const 1
      i32.store offset=36
      local.get 5
      i32.const 1048616
      i32.store offset=32
      local.get 5
      local.get 5
      i32.const 76
      i32.add
      i32.store offset=40
      local.get 5
      i32.const 32
      i32.add
      i32.const 1048636
      call $core::panicking::panic_fmt
      unreachable
    end
    local.get 5
    local.get 4
    i64.store offset=24
    local.get 5
    local.get 3
    i64.store offset=16
    local.get 5
    local.get 2
    i64.store offset=8
    local.get 5
    i32.const 32
    i32.add
    local.get 5
    i32.const 8
    i32.add
    i32.const 3
    call $<miden::asset::Asset as miden::serialization::FeltDeserialize>::from_felts
    local.get 5
    local.get 5
    i32.const 32
    i32.add
    call $basic_wallet::MyWallet::receive_asset
    local.get 5
    i32.const 80
    i32.add
    global.set $__stack_pointer
  )
  (func $<miden::asset::Asset as miden::serialization::FeltDeserialize>::from_felts (;5;) (type 4) (param i32 i32 i32)
    i32.const 1048652
    i32.const 19
    i32.const 1048720
    call $core::panicking::panic
    unreachable
  )
  (func $miden::note::Tag::new (;6;) (type 5) (param i64) (result i64)
    local.get 0
  )
  (func $miden::account::add_asset (;7;) (type 1) (param i32 i32)
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
  (func $core::ptr::drop_in_place<core::fmt::Error> (;8;) (type 2) (param i32))
  (func $core::panicking::panic_fmt (;9;) (type 1) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 0
    i32.store offset=24
    local.get 2
    i32.const 1048736
    i32.store offset=16
    local.get 2
    i32.const 1048736
    i32.store offset=12
    local.get 2
    i32.const 1
    i32.store8 offset=28
    local.get 2
    local.get 1
    i32.store offset=20
    local.get 2
    i32.const 12
    i32.add
    call $rust_begin_unwind
    unreachable
  )
  (func $core::panicking::panic (;10;) (type 4) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 12
    i32.add
    i64.const 0
    i64.store align=4
    local.get 3
    i32.const 1
    i32.store offset=4
    local.get 3
    i32.const 1048736
    i32.store offset=8
    local.get 3
    local.get 1
    i32.store offset=28
    local.get 3
    local.get 0
    i32.store offset=24
    local.get 3
    local.get 3
    i32.const 24
    i32.add
    i32.store
    local.get 3
    local.get 2
    call $core::panicking::panic_fmt
    unreachable
  )
  (func $<T as core::any::Any>::type_id (;11;) (type 1) (param i32 i32)
    local.get 0
    i64.const -3751304911407043677
    i64.store offset=8
    local.get 0
    i64.const 118126004786499436
    i64.store
  )
  (table (;0;) 3 3 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (export "memory" (memory 0))
  (export "my_wallet::receive_asset" (func $my_wallet::receive_asset))
  (elem (;0;) (i32.const 1) func $core::ptr::drop_in_place<core::fmt::Error> $<T as core::any::Any>::type_id)
  (data $.rodata (;0;) (i32.const 1048576) "not yet implemented: use advice provider\00\00\10\00(\00\00\00src/lib.rs\00\000\00\10\00\0a\00\00\00_\00\00\00\0d\00\00\00not yet implemented/Users/dzadorozhnyi/src/miden-ir/sdk/src/asset.rs_\00\10\001\00\00\00&\00\00\00\09\00\00\00\01\00\00\00\00\00\00\00\01\00\00\00\02\00\00\00")
)