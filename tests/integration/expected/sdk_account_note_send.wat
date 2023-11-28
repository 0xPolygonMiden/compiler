(module
  (type (;0;) (func (result i32)))
  (type (;1;) (func))
  (type (;2;) (func (param i32)))
  (type (;3;) (func (param i32 i32 i32) (result i32)))
  (type (;4;) (func (param i32 i32)))
  (type (;5;) (func (param i32 i64 i32)))
  (type (;6;) (func (param i32 i32 i64 i32)))
  (type (;7;) (func (param i32 i64 i64)))
  (type (;8;) (func (param i32 i64 i64 i64 i64)))
  (type (;9;) (func (param i64) (result i64)))
  (import "env" "__main_void" (func $__main_void (;0;) (type 0)))
  (import "env" "__wasm_call_dtors" (func $__wasm_call_dtors (;1;) (type 1)))
  (import "env" "__wasi_proc_exit" (func $__wasi_proc_exit (;2;) (type 2)))
  (import "env" "memcpy" (func $memcpy (;3;) (type 3)))
  (import "env" "miden::sat::account::remove_asset" (func $miden::sat::account::remove_asset (;4;) (type 4)))
  (import "env" "miden::eoa::basic::auth_tx_rpo_falcon512" (func $miden::eoa::basic::auth_tx_rpo_falcon512 (;5;) (type 1)))
  (import "env" "miden::sat::tx::create_note" (func $miden::sat::tx::create_note (;6;) (type 5)))
  (func $__wasm_call_ctors (;7;) (type 1))
  (func $_start (;8;) (type 1)
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
  (func $__original_main (;9;) (type 0) (result i32)
    (local i32 i64 i64)
    global.get $__stack_pointer
    i32.const 96
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    local.get 0
    i32.const 24
    i32.add
    i64.const 1
    i64.const 2
    i64.const 3
    i64.const 4
    call $miden::felt::Word::from_felts
    local.get 0
    i32.const 64
    i32.add
    local.get 0
    i32.const 24
    i32.add
    call $<miden::note::Recipient as core::convert::From<miden::felt::Word>>::from
    i64.const 4
    call $miden::note::Tag::new
    local.set 1
    local.get 0
    i32.const 8
    i32.add
    i64.const 1234
    call $miden::note::Tag::new
    i64.const 100
    call $miden::asset::FungibleAsset::new
    local.get 0
    i64.load offset=8
    local.set 2
    local.get 0
    local.get 0
    i64.load offset=16
    i64.store offset=40
    local.get 0
    local.get 2
    i64.store offset=32
    local.get 0
    i32.const 0
    i32.store offset=24
    i32.const 1048576
    local.get 0
    i32.const 24
    i32.add
    local.get 1
    local.get 0
    i32.const 64
    i32.add
    call $basic_wallet::MyWallet::send_asset
    call $miden::eoa::basic::auth_tx_rpo_falcon512
    local.get 0
    i32.const 96
    i32.add
    global.set $__stack_pointer
    i32.const 0
  )
  (func $basic_wallet::MyWallet::send_asset (;10;) (type 6) (param i32 i32 i64 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    local.get 4
    i32.const 8
    i32.add
    local.get 1
    call $miden::account::remove_asset
    local.get 4
    i32.const 8
    i32.add
    local.get 2
    local.get 3
    call $miden::sat::tx::create_note
    local.get 4
    i32.const 48
    i32.add
    global.set $__stack_pointer
  )
  (func $miden::asset::FungibleAsset::new (;11;) (type 7) (param i32 i64 i64)
    local.get 0
    local.get 2
    i64.store offset=8
    local.get 0
    local.get 1
    i64.store
  )
  (func $miden::felt::Word::from_felts (;12;) (type 8) (param i32 i64 i64 i64 i64)
    local.get 0
    local.get 4
    i64.store offset=24
    local.get 0
    local.get 3
    i64.store offset=16
    local.get 0
    local.get 2
    i64.store offset=8
    local.get 0
    local.get 1
    i64.store
  )
  (func $<miden::note::Recipient as core::convert::From<miden::felt::Word>>::from (;13;) (type 4) (param i32 i32)
    local.get 0
    local.get 1
    i64.load
    i64.store
    local.get 0
    i32.const 24
    i32.add
    local.get 1
    i32.const 24
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load
    i64.store
  )
  (func $miden::note::Tag::new (;14;) (type 9) (param i64) (result i64)
    local.get 0
  )
  (func $miden::account::remove_asset (;15;) (type 4) (param i32 i32)
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
    call $miden::sat::account::remove_asset
    local.get 2
    i32.const 48
    i32.add
    global.set $__stack_pointer
  )
  (func $miden::eoa::basic::auth_tx_rpo_falcon512 (;16;) (type 1)
    call $miden::eoa::basic::auth_tx_rpo_falcon512
  )
  (func $miden::sat::tx::create_note (;17;) (type 5) (param i32 i64 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 80
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 40
    i32.add
    local.get 0
    i32.const 40
    call $memcpy
    drop
    local.get 3
    i32.const 8
    i32.add
    i32.const 24
    i32.add
    local.get 2
    i32.const 24
    i32.add
    i64.load
    i64.store
    local.get 3
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 2
    i32.const 16
    i32.add
    i64.load
    i64.store
    local.get 3
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 2
    i32.const 8
    i32.add
    i64.load
    i64.store
    local.get 3
    local.get 2
    i64.load
    i64.store offset=8
    local.get 3
    i32.const 40
    i32.add
    local.get 1
    local.get 3
    i32.const 8
    i32.add
    call $miden::sat::tx::create_note
    local.get 3
    i32.const 80
    i32.add
    global.set $__stack_pointer
  )
  (table (;0;) 1 1 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (export "memory" (memory 0))
  (export "_start" (func $_start))
  (export "__original_main" (func $__original_main))
  (data $.rodata (;0;) (i32.const 1048576) "")
)