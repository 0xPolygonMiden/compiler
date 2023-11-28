(module
  (type (;0;) (func (result i32)))
  (type (;1;) (func))
  (type (;2;) (func (param i32)))
  (type (;3;) (func (param i32 i32 i64 i32)))
  (type (;4;) (func (param i32 i64 i64)))
  (type (;5;) (func (param i32 i64 i64 i64 i64)))
  (type (;6;) (func (param i32 i32)))
  (type (;7;) (func (param i64) (result i64)))
  (import "env" "__main_void" (func $__main_void (;0;) (type 0)))
  (import "env" "__wasm_call_dtors" (func $__wasm_call_dtors (;1;) (type 1)))
  (import "env" "__wasi_proc_exit" (func $__wasi_proc_exit (;2;) (type 2)))
  (import "env" "miden::eoa::basic::auth_tx_rpo_falcon512" (func $miden::eoa::basic::auth_tx_rpo_falcon512 (;3;) (type 1)))
  (func $__wasm_call_ctors (;4;) (type 1))
  (func $_start (;5;) (type 1)
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
  (func $__original_main (;6;) (type 0) (result i32)
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
  (func $basic_wallet::MyWallet::send_asset (;7;) (type 3) (param i32 i32 i64 i32)
    unreachable
    unreachable
  )
  (func $miden::asset::FungibleAsset::new (;8;) (type 4) (param i32 i64 i64)
    local.get 0
    local.get 2
    i64.store offset=8
    local.get 0
    local.get 1
    i64.store
  )
  (func $miden::felt::Word::from_felts (;9;) (type 5) (param i32 i64 i64 i64 i64)
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
  (func $<miden::note::Recipient as core::convert::From<miden::felt::Word>>::from (;10;) (type 6) (param i32 i32)
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
  (func $miden::note::Tag::new (;11;) (type 7) (param i64) (result i64)
    local.get 0
  )
  (func $miden::eoa::basic::auth_tx_rpo_falcon512 (;12;) (type 1)
    call $miden::eoa::basic::auth_tx_rpo_falcon512
  )
  (table (;0;) 1 1 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (export "memory" (memory 0))
  (export "_start" (func $_start))
  (export "__original_main" (func $__original_main))
  (data $.rodata (;0;) (i32.const 1048576) "")
)