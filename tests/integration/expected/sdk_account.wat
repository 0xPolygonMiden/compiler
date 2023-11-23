(module
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func (param i32 i64 i32)))
  (type (;2;) (func (param i32 i32 i32)))
  (type (;3;) (func (param i64) (result i64)))
  (import "env" "miden::sat::account::add_asset" (func $miden::sat::account::add_asset (;0;) (type 0)))
  (import "env" "miden::sat::account::remove_asset" (func $miden::sat::account::remove_asset (;1;) (type 0)))
  (import "env" "miden::sat::tx::create_note" (func $miden::sat::tx::create_note (;2;) (type 1)))
  (func $receive_asset (;3;) (type 0) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 1
    call $miden::account::add_asset
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $send_asset (;4;) (type 2) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 1
    call $miden::account::remove_asset
    local.get 3
    i64.const 4
    call $miden::note::Tag::new
    local.get 2
    call $miden::tx::create_note
    local.get 3
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $miden::account::add_asset (;5;) (type 0) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 24
    i32.add
    local.get 1
    i32.const 24
    i32.add
    i64.load
    i64.store
    local.get 2
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load
    i64.store
    local.get 2
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load
    i64.store
    local.get 2
    local.get 1
    i64.load
    i64.store
    local.get 0
    local.get 2
    call $miden::sat::account::add_asset
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $miden::account::remove_asset (;6;) (type 0) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 24
    i32.add
    local.get 1
    i32.const 24
    i32.add
    i64.load
    i64.store
    local.get 2
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load
    i64.store
    local.get 2
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load
    i64.store
    local.get 2
    local.get 1
    i64.load
    i64.store
    local.get 0
    local.get 2
    call $miden::sat::account::remove_asset
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $miden::note::Tag::new (;7;) (type 3) (param i64) (result i64)
    local.get 0
  )
  (func $miden::tx::create_note (;8;) (type 1) (param i32 i64 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 32
    i32.add
    i32.const 24
    i32.add
    local.get 0
    i32.const 24
    i32.add
    i64.load
    i64.store
    local.get 3
    i32.const 32
    i32.add
    i32.const 16
    i32.add
    local.get 0
    i32.const 16
    i32.add
    i64.load
    i64.store
    local.get 3
    i32.const 32
    i32.add
    i32.const 8
    i32.add
    local.get 0
    i32.const 8
    i32.add
    i64.load
    i64.store
    local.get 3
    local.get 0
    i64.load
    i64.store offset=32
    local.get 3
    i32.const 8
    i32.add
    local.get 2
    i32.const 8
    i32.add
    i64.load
    i64.store
    local.get 3
    i32.const 16
    i32.add
    local.get 2
    i32.const 16
    i32.add
    i64.load
    i64.store
    local.get 3
    i32.const 24
    i32.add
    local.get 2
    i32.const 24
    i32.add
    i64.load
    i64.store
    local.get 3
    local.get 2
    i64.load
    i64.store
    local.get 3
    i32.const 32
    i32.add
    local.get 1
    local.get 3
    call $miden::sat::tx::create_note
    local.get 3
    i32.const 64
    i32.add
    global.set $__stack_pointer
  )
  (table (;0;) 1 1 funcref)
  (memory (;0;) 16)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (export "memory" (memory 0))
  (export "receive_asset" (func $receive_asset))
  (export "send_asset" (func $send_asset))
)