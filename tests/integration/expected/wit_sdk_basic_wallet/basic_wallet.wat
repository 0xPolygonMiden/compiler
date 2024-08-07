(module $basic_wallet.wasm
  (type (;0;) (func (param i64 i64 i64 i64 i32)))
  (type (;1;) (func (param i64 i64 i64 i64 i64 i64 i64 i64 i64) (result i64)))
  (type (;2;) (func))
  (type (;3;) (func (param i64 i64 i64 i64)))
  (type (;4;) (func (param i64 i64 i64 i64 i64 i64 i64 i64 i64)))
  (type (;5;) (func (param i32 i32) (result i32)))
  (type (;6;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;7;) (func (param i32 i32 i32) (result i32)))
  (type (;8;) (func (param i32 i32 i32 i32)))
  (import "miden:base/account@1.0.0" "add-asset" (func $basic_wallet::bindings::miden::base::account::add_asset::wit_import (;0;) (type 0)))
  (import "miden:base/account@1.0.0" "remove-asset" (func $basic_wallet::bindings::miden::base::account::remove_asset::wit_import (;1;) (type 0)))
  (import "miden:base/tx@1.0.0" "create-note" (func $basic_wallet::bindings::miden::base::tx::create_note::wit_import (;2;) (type 1)))
  (func $__wasm_call_ctors (;3;) (type 2))
  (func $miden:basic-wallet/basic-wallet@1.0.0#receive-asset (;4;) (type 3) (param i64 i64 i64 i64)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    call $wit_bindgen::rt::run_ctors_once
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    local.get 4
    call $basic_wallet::bindings::miden::base::account::add_asset::wit_import
    local.get 4
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $miden:basic-wallet/basic-wallet@1.0.0#send-asset (;5;) (type 4) (param i64 i64 i64 i64 i64 i64 i64 i64 i64)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 9
    global.set $__stack_pointer
    call $wit_bindgen::rt::run_ctors_once
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    local.get 9
    call $basic_wallet::bindings::miden::base::account::remove_asset::wit_import
    local.get 9
    i64.load
    local.get 9
    i32.const 8
    i32.add
    i64.load
    local.get 9
    i32.const 16
    i32.add
    i64.load
    local.get 9
    i32.const 24
    i32.add
    i64.load
    local.get 4
    local.get 5
    local.get 6
    local.get 7
    local.get 8
    call $basic_wallet::bindings::miden::base::tx::create_note::wit_import
    drop
    local.get 9
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $__rust_alloc (;6;) (type 5) (param i32 i32) (result i32)
    i32.const 1048576
    local.get 1
    local.get 0
    call $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::alloc
  )
  (func $__rust_realloc (;7;) (type 6) (param i32 i32 i32 i32) (result i32)
    (local i32)
    block ;; label = @1
      i32.const 1048576
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
      i32.const 1048576
      local.get 0
      local.get 2
      local.get 1
      call $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::dealloc
    end
    local.get 4
  )
  (func $wee_alloc::alloc_first_fit (;8;) (type 7) (param i32 i32 i32) (result i32)
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
  (func $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::alloc (;9;) (type 7) (param i32 i32 i32) (result i32)
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
  (func $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::dealloc (;10;) (type 8) (param i32 i32 i32 i32)
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
  (func $wit_bindgen::rt::run_ctors_once (;11;) (type 2)
    block ;; label = @1
      i32.const 0
      i32.load8_u offset=1048581
      br_if 0 (;@1;)
      call $__wasm_call_ctors
      i32.const 0
      i32.const 1
      i32.store8 offset=1048581
    end
  )
  (func $cabi_realloc (;12;) (type 6) (param i32 i32 i32 i32) (result i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          br_if 0 (;@3;)
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          i32.const 0
          i32.load8_u offset=1048580
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
  (table (;0;) 1 1 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (export "memory" (memory 0))
  (export "miden:basic-wallet/basic-wallet@1.0.0#receive-asset" (func $miden:basic-wallet/basic-wallet@1.0.0#receive-asset))
  (export "miden:basic-wallet/basic-wallet@1.0.0#send-asset" (func $miden:basic-wallet/basic-wallet@1.0.0#send-asset))
  (export "cabi_realloc" (func $cabi_realloc))
)