(module $miden_sdk.wasm
  (type (;0;) (func))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (param i64) (result i64)))
  (type (;3;) (func (param i64 i64 i64 i64) (result i32)))
  (type (;4;) (func (param i32 i64 i64 i64 i64) (result i32)))
  (type (;5;) (func (param i32 i32) (result i32)))
  (type (;6;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;7;) (func (param i32 i32 i32) (result i32)))
  (type (;8;) (func (param i32 i32 i32 i32)))
  (type (;9;) (func (param i32 i32)))
  (type (;10;) (func (param i32 i32 i32)))
  (func $__wasm_call_ctors (;0;) (type 0))
  (func $rust_begin_unwind (;1;) (type 1) (param i32)
    loop ;; label = @1
      br 0 (;@1;)
    end
  )
  (func $miden:base/core-types@1.0.0#account-id-from-felt (;2;) (type 2) (param i64) (result i64)
    call $wit_bindgen::rt::run_ctors_once
    local.get 0
  )
  (func $miden:base/types@1.0.0#from-core-asset (;3;) (type 3) (param i64 i64 i64 i64) (result i32)
    call $wit_bindgen::rt::run_ctors_once
    i32.const 1048576
    i32.const 19
    i32.const 1048608
    call $core::panicking::panic
    unreachable
  )
  (func $miden:base/types@1.0.0#to-core-asset (;4;) (type 4) (param i32 i64 i64 i64 i64) (result i32)
    call $wit_bindgen::rt::run_ctors_once
    i32.const 1048576
    i32.const 19
    i32.const 1048624
    call $core::panicking::panic
    unreachable
  )
  (func $__rust_alloc (;5;) (type 5) (param i32 i32) (result i32)
    i32.const 1048656
    local.get 1
    local.get 0
    call $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::alloc
  )
  (func $__rust_realloc (;6;) (type 6) (param i32 i32 i32 i32) (result i32)
    (local i32)
    block ;; label = @1
      i32.const 1048656
      local.get 2
      local.get 3
      call $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::alloc
      local.tee 4
      i32.eqz
      br_if 0 (;@1;)
      local.get 4
      local.get 0
      local.get 1
      local.get 3
      local.get 1
      local.get 3
      i32.lt_u
      select
      memory.copy
      i32.const 1048656
      local.get 0
      local.get 2
      local.get 1
      call $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::dealloc
    end
    local.get 4
  )
  (func $wee_alloc::alloc_first_fit (;7;) (type 7) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      local.get 2
      i32.load
      local.tee 3
      br_if 0 (;@1;)
      i32.const 0
      return
    end
    local.get 1
    i32.const -1
    i32.add
    local.set 4
    i32.const 0
    local.get 1
    i32.sub
    local.set 5
    local.get 0
    i32.const 2
    i32.shl
    local.set 6
    loop ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 3
          i32.load offset=8
          local.tee 1
          i32.const 1
          i32.and
          br_if 0 (;@3;)
          local.get 3
          i32.const 8
          i32.add
          local.set 0
          br 1 (;@2;)
        end
        loop ;; label = @3
          local.get 3
          local.get 1
          i32.const -2
          i32.and
          i32.store offset=8
          block ;; label = @4
            block ;; label = @5
              local.get 3
              i32.load offset=4
              local.tee 7
              i32.const -4
              i32.and
              local.tee 0
              br_if 0 (;@5;)
              i32.const 0
              local.set 8
              br 1 (;@4;)
            end
            i32.const 0
            local.get 0
            local.get 0
            i32.load8_u
            i32.const 1
            i32.and
            select
            local.set 8
          end
          block ;; label = @4
            local.get 3
            i32.load
            local.tee 1
            i32.const -4
            i32.and
            local.tee 9
            i32.eqz
            br_if 0 (;@4;)
            local.get 1
            i32.const 2
            i32.and
            br_if 0 (;@4;)
            local.get 9
            local.get 9
            i32.load offset=4
            i32.const 3
            i32.and
            local.get 0
            i32.or
            i32.store offset=4
            local.get 3
            i32.load offset=4
            local.tee 7
            i32.const -4
            i32.and
            local.set 0
            local.get 3
            i32.load
            local.set 1
          end
          block ;; label = @4
            local.get 0
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            local.get 0
            i32.load
            i32.const 3
            i32.and
            local.get 1
            i32.const -4
            i32.and
            i32.or
            i32.store
            local.get 3
            i32.load offset=4
            local.set 7
            local.get 3
            i32.load
            local.set 1
          end
          local.get 3
          local.get 7
          i32.const 3
          i32.and
          i32.store offset=4
          local.get 3
          local.get 1
          i32.const 3
          i32.and
          i32.store
          block ;; label = @4
            local.get 1
            i32.const 2
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 8
            local.get 8
            i32.load
            i32.const 2
            i32.or
            i32.store
          end
          local.get 2
          local.get 8
          i32.store
          local.get 8
          local.set 3
          local.get 8
          i32.load offset=8
          local.tee 1
          i32.const 1
          i32.and
          br_if 0 (;@3;)
        end
        local.get 8
        i32.const 8
        i32.add
        local.set 0
        local.get 8
        local.set 3
      end
      block ;; label = @2
        local.get 3
        i32.load
        i32.const -4
        i32.and
        local.tee 8
        local.get 0
        i32.sub
        local.get 6
        i32.lt_u
        br_if 0 (;@2;)
        block ;; label = @3
          block ;; label = @4
            local.get 0
            i32.const 72
            i32.add
            local.get 8
            local.get 6
            i32.sub
            local.get 5
            i32.and
            local.tee 8
            i32.le_u
            br_if 0 (;@4;)
            local.get 4
            local.get 0
            i32.and
            br_if 2 (;@2;)
            local.get 2
            local.get 1
            i32.const -4
            i32.and
            i32.store
            local.get 3
            i32.load
            local.set 0
            local.get 3
            local.set 1
            br 1 (;@3;)
          end
          i32.const 0
          local.set 7
          local.get 8
          i32.const 0
          i32.store
          local.get 8
          i32.const -8
          i32.add
          local.tee 1
          i64.const 0
          i64.store align=4
          local.get 1
          local.get 3
          i32.load
          i32.const -4
          i32.and
          i32.store
          block ;; label = @4
            local.get 3
            i32.load
            local.tee 9
            i32.const -4
            i32.and
            local.tee 8
            i32.eqz
            br_if 0 (;@4;)
            local.get 9
            i32.const 2
            i32.and
            br_if 0 (;@4;)
            local.get 8
            local.get 8
            i32.load offset=4
            i32.const 3
            i32.and
            local.get 1
            i32.or
            i32.store offset=4
            local.get 1
            i32.load offset=4
            i32.const 3
            i32.and
            local.set 7
          end
          local.get 1
          local.get 7
          local.get 3
          i32.or
          i32.store offset=4
          local.get 0
          local.get 0
          i32.load
          i32.const -2
          i32.and
          i32.store
          local.get 3
          local.get 3
          i32.load
          local.tee 0
          i32.const 3
          i32.and
          local.get 1
          i32.or
          local.tee 8
          i32.store
          block ;; label = @4
            local.get 0
            i32.const 2
            i32.and
            br_if 0 (;@4;)
            local.get 1
            i32.load
            local.set 0
            br 1 (;@3;)
          end
          local.get 3
          local.get 8
          i32.const -3
          i32.and
          i32.store
          local.get 1
          i32.load
          i32.const 2
          i32.or
          local.set 0
        end
        local.get 1
        local.get 0
        i32.const 1
        i32.or
        i32.store
        local.get 1
        i32.const 8
        i32.add
        return
      end
      local.get 2
      local.get 1
      i32.store
      local.get 1
      local.set 3
      local.get 1
      br_if 0 (;@1;)
    end
    i32.const 0
  )
  (func $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::alloc (;8;) (type 7) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        local.get 2
        br_if 0 (;@2;)
        local.get 1
        local.set 2
        br 1 (;@1;)
      end
      local.get 3
      local.get 0
      i32.load
      i32.store offset=12
      block ;; label = @2
        local.get 2
        i32.const 3
        i32.add
        local.tee 4
        i32.const 2
        i32.shr_u
        local.tee 5
        local.get 1
        local.get 3
        i32.const 12
        i32.add
        call $wee_alloc::alloc_first_fit
        local.tee 2
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 4
          i32.const -4
          i32.and
          local.tee 2
          local.get 1
          i32.const 3
          i32.shl
          i32.const 512
          i32.add
          local.tee 4
          local.get 2
          local.get 4
          i32.gt_u
          select
          i32.const 65543
          i32.add
          local.tee 4
          i32.const 16
          i32.shr_u
          memory.grow
          local.tee 2
          i32.const -1
          i32.ne
          br_if 0 (;@3;)
          i32.const 0
          local.set 2
          br 1 (;@2;)
        end
        local.get 2
        i32.const 16
        i32.shl
        local.tee 2
        i32.const 0
        i32.store offset=4
        local.get 2
        local.get 3
        i32.load offset=12
        i32.store offset=8
        local.get 2
        local.get 2
        local.get 4
        i32.const -65536
        i32.and
        i32.add
        i32.const 2
        i32.or
        i32.store
        local.get 3
        local.get 2
        i32.store offset=12
        local.get 5
        local.get 1
        local.get 3
        i32.const 12
        i32.add
        call $wee_alloc::alloc_first_fit
        local.set 2
      end
      local.get 0
      local.get 3
      i32.load offset=12
      i32.store
    end
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 2
  )
  (func $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::dealloc (;9;) (type 8) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      local.get 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load
      local.set 4
      local.get 1
      i32.const 0
      i32.store
      local.get 1
      i32.const -8
      i32.add
      local.tee 3
      local.get 3
      i32.load
      local.tee 5
      i32.const -2
      i32.and
      local.tee 6
      i32.store
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 3
                  i32.const 4
                  i32.add
                  local.tee 7
                  i32.load
                  i32.const -4
                  i32.and
                  local.tee 8
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 8
                  i32.load
                  local.tee 9
                  i32.const 1
                  i32.and
                  i32.eqz
                  br_if 1 (;@6;)
                end
                local.get 5
                i32.const -4
                i32.and
                local.tee 8
                i32.eqz
                br_if 3 (;@3;)
                local.get 5
                i32.const 2
                i32.and
                i32.eqz
                br_if 1 (;@5;)
                br 3 (;@3;)
              end
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    local.get 5
                    i32.const -4
                    i32.and
                    local.tee 10
                    br_if 0 (;@8;)
                    local.get 8
                    local.set 1
                    br 1 (;@7;)
                  end
                  local.get 8
                  local.set 1
                  local.get 5
                  i32.const 2
                  i32.and
                  br_if 0 (;@7;)
                  local.get 10
                  local.get 10
                  i32.load offset=4
                  i32.const 3
                  i32.and
                  local.get 8
                  i32.or
                  i32.store offset=4
                  local.get 3
                  i32.load
                  local.set 6
                  local.get 7
                  i32.load
                  local.tee 5
                  i32.const -4
                  i32.and
                  local.tee 1
                  i32.eqz
                  br_if 1 (;@6;)
                  local.get 1
                  i32.load
                  local.set 9
                end
                local.get 1
                local.get 6
                i32.const -4
                i32.and
                local.get 9
                i32.const 3
                i32.and
                i32.or
                i32.store
                local.get 7
                i32.load
                local.set 5
                local.get 3
                i32.load
                local.set 6
              end
              local.get 7
              local.get 5
              i32.const 3
              i32.and
              i32.store
              local.get 3
              local.get 6
              i32.const 3
              i32.and
              i32.store
              local.get 6
              i32.const 2
              i32.and
              i32.eqz
              br_if 1 (;@4;)
              local.get 8
              local.get 8
              i32.load
              i32.const 2
              i32.or
              i32.store
              br 1 (;@4;)
            end
            local.get 8
            i32.load8_u
            i32.const 1
            i32.and
            br_if 1 (;@3;)
            local.get 1
            local.get 8
            i32.load offset=8
            i32.const -4
            i32.and
            i32.store
            local.get 8
            local.get 3
            i32.const 1
            i32.or
            i32.store offset=8
          end
          local.get 4
          local.set 3
          br 1 (;@2;)
        end
        local.get 1
        local.get 4
        i32.store
      end
      local.get 0
      local.get 3
      i32.store
    end
  )
  (func $wit_bindgen::rt::run_ctors_once (;10;) (type 0)
    block ;; label = @1
      i32.const 0
      i32.load8_u offset=1048661
      br_if 0 (;@1;)
      call $__wasm_call_ctors
      i32.const 0
      i32.const 1
      i32.store8 offset=1048661
    end
  )
  (func $cabi_realloc (;11;) (type 6) (param i32 i32 i32 i32) (result i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          br_if 0 (;@3;)
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          i32.const 0
          i32.load8_u offset=1048660
          drop
          local.get 3
          local.get 2
          call $__rust_alloc
          local.set 2
          br 1 (;@2;)
        end
        local.get 0
        local.get 1
        local.get 2
        local.get 3
        call $__rust_realloc
        local.set 2
      end
      local.get 2
      br_if 0 (;@1;)
      unreachable
      unreachable
    end
    local.get 2
  )
  (func $core::ptr::drop_in_place<core::fmt::Error> (;12;) (type 1) (param i32))
  (func $core::panicking::panic_fmt (;13;) (type 9) (param i32 i32)
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
    i32.const 1048640
    i32.store offset=16
    local.get 2
    i32.const 1
    i32.store offset=12
    local.get 2
    i32.const 12
    i32.add
    call $rust_begin_unwind
    unreachable
  )
  (func $core::panicking::panic (;14;) (type 10) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 0
    i32.store offset=16
    local.get 3
    i32.const 1
    i32.store offset=4
    local.get 3
    i64.const 4
    i64.store offset=8 align=4
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
  (func $<T as core::any::Any>::type_id (;15;) (type 9) (param i32 i32)
    local.get 0
    i64.const -6527957459535493887
    i64.store offset=8
    local.get 0
    i64.const -7007892379802179865
    i64.store
  )
  (table (;0;) 3 3 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (export "memory" (memory 0))
  (export "miden:base/core-types@1.0.0#account-id-from-felt" (func $miden:base/core-types@1.0.0#account-id-from-felt))
  (export "miden:base/types@1.0.0#from-core-asset" (func $miden:base/types@1.0.0#from-core-asset))
  (export "miden:base/types@1.0.0#to-core-asset" (func $miden:base/types@1.0.0#to-core-asset))
  (export "cabi_realloc" (func $cabi_realloc))
  (elem (;0;) (i32.const 1) func $core::ptr::drop_in_place<core::fmt::Error> $<T as core::any::Any>::type_id)
  (data $.rodata (;0;) (i32.const 1048576) "not yet implementedsrc/lib.rs\00\00\00\13\00\10\00\0a\00\00\00\1d\00\00\00\09\00\00\00\13\00\10\00\0a\00\00\00!\00\00\00\09\00\00\00\01\00\00\00\00\00\00\00\01\00\00\00\02\00\00\00")
)