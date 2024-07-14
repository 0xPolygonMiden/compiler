(module $test_rust_dff35dcb9118143f33663da3a7daff983fab22cc980ad18f297e4d3f9b3a3424.wasm
  (type (;0;) (func (param i32)))
  (type (;1;) (func (param i64 i64) (result i64)))
  (type (;2;) (func (param i32 i32)))
  (type (;3;) (func (param i32 i32 i32)))
  (func $rust_begin_unwind (;0;) (type 0) (param i32)
    loop ;; label = @1
      br 0 (;@1;)
    end
  )
  (func $entrypoint (;1;) (type 1) (param i64 i64) (result i64)
    block ;; label = @1
      local.get 1
      i64.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i64.rem_u
      return
    end
    i32.const 1048800
    i32.const 57
    i32.const 1048780
    call $core::panicking::panic
    unreachable
  )
  (func $core::ptr::drop_in_place<core::fmt::Error> (;2;) (type 0) (param i32))
  (func $core::panicking::panic_fmt (;3;) (type 2) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 1
    i32.store16 offset=28
    local.get 2
    local.get 1
    i32.store offset=24
    local.get 2
    local.get 0
    i32.store offset=20
    local.get 2
    i32.const 1048860
    i32.store offset=16
    local.get 2
    i32.const 1048860
    i32.store offset=12
    local.get 2
    i32.const 12
    i32.add
    call $rust_begin_unwind
    unreachable
  )
  (func $core::panicking::panic (;4;) (type 3) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 1
    i32.store offset=4
    local.get 3
    i64.const 0
    i64.store offset=12 align=4
    local.get 3
    i32.const 1048860
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
  (func $<T as core::any::Any>::type_id (;5;) (type 2) (param i32 i32)
    local.get 0
    i64.const -2331727641711382032
    i64.store offset=8
    local.get 0
    i64.const -4483515439147723121
    i64.store
  )
  (table (;0;) 3 3 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (global (;1;) i32 i32.const 1048876)
  (global (;2;) i32 i32.const 1048880)
  (export "memory" (memory 0))
  (export "entrypoint" (func $entrypoint))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (elem (;0;) (i32.const 1) func $core::ptr::drop_in_place<core::fmt::Error> $<T as core::any::Any>::type_id)
  (data $.rodata (;0;) (i32.const 1048576) "/var/folders/h7/2tf4y2v50ks4hh8j3975l77w0000gn/T/test_rust_dff35dcb9118143f33663da3a7daff983fab22cc980ad18f297e4d3f9b3a3424/test_rust_dff35dcb9118143f33663da3a7daff983fab22cc980ad18f297e4d3f9b3a3424.rs\00\00\00\00\00\10\00\c9\00\00\00\0b\00\00\00C\00\00\00\00\00\00\00attempt to calculate the remainder with a divisor of zero\00\00\00\01\00\00\00\00\00\00\00\01\00\00\00\02\00\00\00")
)