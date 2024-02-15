(component
  (type (;0;)
    (instance
      (type (;0;) u64)
      (export (;1;) "felt" (type (eq 0)))
      (export (;2;) "account-id" (type (eq 1)))
      (type (;3;) (record (field "asset" 2) (field "amount" u64)))
      (export (;4;) "fungible-asset" (type (eq 3)))
      (type (;5;) (tuple 1 1 1 1))
      (export (;6;) "word" (type (eq 5)))
      (export (;7;) "non-fungible-asset" (type (eq 6)))
      (type (;8;) (variant (case "fungible" 4) (case "non-fungible" 7)))
      (export (;9;) "asset" (type (eq 8)))
      (export (;10;) "tag" (type (eq 1)))
      (export (;11;) "recipient" (type (eq 6)))
    )
  )
  (import "miden:base/types@1.0.0" (instance (;0;) (type 0)))
  (alias export 0 "asset" (type (;1;)))
  (alias export 0 "tag" (type (;2;)))
  (alias export 0 "recipient" (type (;3;)))
  (type (;4;)
    (instance
      (alias outer 1 1 (type (;0;)))
      (export (;1;) "asset" (type (eq 0)))
      (alias outer 1 2 (type (;2;)))
      (export (;3;) "tag" (type (eq 2)))
      (alias outer 1 3 (type (;4;)))
      (export (;5;) "recipient" (type (eq 4)))
      (type (;6;) (func (param "asset" 1) (result 1)))
      (export (;0;) "add-asset" (func (type 6)))
      (export (;1;) "remove-asset" (func (type 6)))
      (type (;7;) (func (param "asset" 1) (param "tag" 3) (param "recipient" 5)))
      (export (;2;) "create-note" (func (type 7)))
    )
  )
  (import "miden:base/tx-kernel@1.0.0" (instance (;1;) (type 4)))
  (core module (;0;)
    (type (;0;) (func (param i32 i64 i64 i64 i64 i32)))
    (type (;1;) (func (param i32 i64 i64 i64 i64 i64 i64 i64 i64 i64)))
    (type (;2;) (func))
    (type (;3;) (func (param i32 i64 i64 i64 i64)))
    (type (;4;) (func (param i32 i32) (result i32)))
    (type (;5;) (func (param i32 i32 i32 i32) (result i32)))
    (type (;6;) (func (param i32 i32 i32) (result i32)))
    (type (;7;) (func (param i32 i32 i32 i32)))
    (import "miden:base/tx-kernel@1.0.0" "add-asset" (func $basic_wallet::bindings::miden::base::tx_kernel::add_asset::wit_import (;0;) (type 0)))
    (import "miden:base/tx-kernel@1.0.0" "remove-asset" (func $basic_wallet::bindings::miden::base::tx_kernel::remove_asset::wit_import (;1;) (type 0)))
    (import "miden:base/tx-kernel@1.0.0" "create-note" (func $basic_wallet::bindings::miden::base::tx_kernel::create_note::wit_import (;2;) (type 1)))
    (func $__wasm_call_ctors (;3;) (type 2))
    (func $miden:basic-wallet/basic-wallet@1.0.0#receive-asset (;4;) (type 3) (param i32 i64 i64 i64 i64)
      (local i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 5
      global.set $__stack_pointer
      call $wit_bindgen::rt::run_ctors_once
      local.get 0
      i32.const 0
      i32.ne
      local.get 1
      local.get 2
      local.get 3
      i64.const 0
      local.get 0
      select
      local.get 4
      i64.const 0
      local.get 0
      select
      local.get 5
      i32.const 8
      i32.add
      call $basic_wallet::bindings::miden::base::tx_kernel::add_asset::wit_import
      local.get 5
      i32.const 48
      i32.add
      global.set $__stack_pointer
    )
    (func $miden:basic-wallet/basic-wallet@1.0.0#send-asset (;5;) (type 1) (param i32 i64 i64 i64 i64 i64 i64 i64 i64 i64)
      (local i32 i64 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 10
      global.set $__stack_pointer
      i64.const 0
      local.set 11
      i32.const 0
      local.set 12
      call $wit_bindgen::rt::run_ctors_once
      local.get 0
      i32.const 0
      i32.ne
      local.get 1
      local.get 2
      local.get 3
      i64.const 0
      local.get 0
      select
      local.get 4
      i64.const 0
      local.get 0
      select
      local.get 10
      i32.const 8
      i32.add
      call $basic_wallet::bindings::miden::base::tx_kernel::remove_asset::wit_import
      block ;; label = @1
        block ;; label = @2
          local.get 10
          i32.load8_u offset=8
          br_if 0 (;@2;)
          local.get 10
          i32.const 24
          i32.add
          i64.load
          local.set 4
          local.get 10
          i32.const 16
          i32.add
          i64.load
          local.set 3
          i64.const 0
          local.set 2
          br 1 (;@1;)
        end
        local.get 10
        i32.const 24
        i32.add
        i64.load
        local.set 4
        local.get 10
        i32.const 16
        i32.add
        i64.load
        local.set 3
        local.get 10
        i32.const 40
        i32.add
        i64.load
        local.set 11
        local.get 10
        i32.const 32
        i32.add
        i64.load
        local.set 2
        i32.const 1
        local.set 12
      end
      local.get 12
      local.get 3
      local.get 4
      local.get 2
      local.get 11
      local.get 5
      local.get 6
      local.get 7
      local.get 8
      local.get 9
      call $basic_wallet::bindings::miden::base::tx_kernel::create_note::wit_import
      local.get 10
      i32.const 48
      i32.add
      global.set $__stack_pointer
    )
    (func $__rust_alloc (;6;) (type 4) (param i32 i32) (result i32)
      i32.const 1048576
      local.get 1
      local.get 0
      call $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::alloc
    )
    (func $__rust_realloc (;7;) (type 5) (param i32 i32 i32 i32) (result i32)
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
    (func $wee_alloc::alloc_first_fit (;8;) (type 6) (param i32 i32 i32) (result i32)
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
            local.set 7
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
                local.tee 8
                i32.const -4
                i32.and
                local.tee 1
                br_if 0 (;@5;)
                i32.const 0
                local.set 7
                br 1 (;@4;)
              end
              i32.const 0
              local.get 1
              local.get 1
              i32.load8_u
              i32.const 1
              i32.and
              select
              local.set 7
            end
            block ;; label = @4
              local.get 3
              i32.load
              local.tee 0
              i32.const -4
              i32.and
              local.tee 9
              i32.eqz
              br_if 0 (;@4;)
              local.get 0
              i32.const 2
              i32.and
              br_if 0 (;@4;)
              local.get 9
              local.get 9
              i32.load offset=4
              i32.const 3
              i32.and
              local.get 1
              i32.or
              i32.store offset=4
              local.get 3
              i32.load offset=4
              local.tee 8
              i32.const -4
              i32.and
              local.set 1
              local.get 3
              i32.load
              local.set 0
            end
            block ;; label = @4
              local.get 1
              i32.eqz
              br_if 0 (;@4;)
              local.get 1
              local.get 1
              i32.load
              i32.const 3
              i32.and
              local.get 0
              i32.const -4
              i32.and
              i32.or
              i32.store
              local.get 3
              i32.load offset=4
              local.set 8
              local.get 3
              i32.load
              local.set 0
            end
            local.get 3
            local.get 8
            i32.const 3
            i32.and
            i32.store offset=4
            local.get 3
            local.get 0
            i32.const 3
            i32.and
            i32.store
            block ;; label = @4
              local.get 0
              i32.const 2
              i32.and
              i32.eqz
              br_if 0 (;@4;)
              local.get 7
              local.get 7
              i32.load
              i32.const 2
              i32.or
              i32.store
            end
            local.get 2
            local.get 7
            i32.store
            local.get 7
            local.set 3
            local.get 7
            i32.load offset=8
            local.tee 1
            i32.const 1
            i32.and
            br_if 0 (;@3;)
          end
        end
        block ;; label = @2
          local.get 7
          i32.load
          i32.const -4
          i32.and
          local.tee 0
          local.get 7
          i32.const 8
          i32.add
          local.tee 3
          i32.sub
          local.get 6
          i32.lt_u
          br_if 0 (;@2;)
          block ;; label = @3
            block ;; label = @4
              local.get 3
              i32.const 72
              i32.add
              local.get 0
              local.get 6
              i32.sub
              local.get 5
              i32.and
              local.tee 0
              i32.le_u
              br_if 0 (;@4;)
              local.get 4
              local.get 3
              i32.and
              br_if 2 (;@2;)
              local.get 2
              local.get 7
              i32.load offset=8
              i32.const -4
              i32.and
              i32.store
              local.get 7
              i32.load
              local.set 1
              local.get 7
              local.set 3
              br 1 (;@3;)
            end
            i32.const 0
            local.set 1
            local.get 0
            i32.const 0
            i32.store
            local.get 0
            i32.const -8
            i32.add
            local.tee 3
            i64.const 0
            i64.store align=4
            local.get 3
            local.get 7
            i32.load
            i32.const -4
            i32.and
            i32.store
            block ;; label = @4
              local.get 7
              i32.load
              local.tee 8
              i32.const -4
              i32.and
              local.tee 0
              i32.eqz
              br_if 0 (;@4;)
              local.get 8
              i32.const 2
              i32.and
              br_if 0 (;@4;)
              local.get 0
              local.get 0
              i32.load offset=4
              i32.const 3
              i32.and
              local.get 3
              i32.or
              i32.store offset=4
              local.get 3
              i32.load offset=4
              i32.const 3
              i32.and
              local.set 1
            end
            local.get 3
            local.get 1
            local.get 7
            i32.or
            i32.store offset=4
            local.get 7
            local.get 7
            i32.load offset=8
            i32.const -2
            i32.and
            i32.store offset=8
            local.get 7
            local.get 7
            i32.load
            local.tee 1
            i32.const 3
            i32.and
            local.get 3
            i32.or
            local.tee 0
            i32.store
            block ;; label = @4
              local.get 1
              i32.const 2
              i32.and
              br_if 0 (;@4;)
              local.get 3
              i32.load
              local.set 1
              br 1 (;@3;)
            end
            local.get 7
            local.get 0
            i32.const -3
            i32.and
            i32.store
            local.get 3
            i32.load
            i32.const 2
            i32.or
            local.set 1
          end
          local.get 3
          local.get 1
          i32.const 1
          i32.or
          i32.store
          local.get 3
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
    (func $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::alloc (;9;) (type 6) (param i32 i32 i32) (result i32)
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
    (func $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::dealloc (;10;) (type 7) (param i32 i32 i32 i32)
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
                local.get 3
                i32.const 4
                i32.add
                local.tee 7
                i32.load
                i32.const -4
                i32.and
                local.tee 8
                i32.eqz
                br_if 0 (;@5;)
                local.get 8
                i32.load
                local.tee 9
                i32.const 1
                i32.and
                br_if 0 (;@5;)
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
              local.get 5
              i32.const -4
              i32.and
              local.tee 8
              i32.eqz
              br_if 1 (;@3;)
              local.get 5
              i32.const 2
              i32.and
              br_if 1 (;@3;)
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
    (func $cabi_realloc (;12;) (type 5) (param i32 i32 i32 i32) (result i32)
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
  (core module (;1;)
    (type (;0;) (func (param i32 i64 i64 i64 i64 i32)))
    (func $indirect-miden:base/tx-kernel@1.0.0-add-asset (;0;) (type 0) (param i32 i64 i64 i64 i64 i32)
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      local.get 4
      local.get 5
      i32.const 0
      call_indirect (type 0)
    )
    (func $indirect-miden:base/tx-kernel@1.0.0-remove-asset (;1;) (type 0) (param i32 i64 i64 i64 i64 i32)
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      local.get 4
      local.get 5
      i32.const 1
      call_indirect (type 0)
    )
    (table (;0;) 2 2 funcref)
    (export "0" (func $indirect-miden:base/tx-kernel@1.0.0-add-asset))
    (export "1" (func $indirect-miden:base/tx-kernel@1.0.0-remove-asset))
    (export "$imports" (table 0))
  )
  (core module (;2;)
    (type (;0;) (func (param i32 i64 i64 i64 i64 i32)))
    (import "" "0" (func (;0;) (type 0)))
    (import "" "1" (func (;1;) (type 0)))
    (import "" "$imports" (table (;0;) 2 2 funcref))
    (elem (;0;) (i32.const 0) func 0 1)
  )
  (core instance (;0;) (instantiate 1))
  (alias core export 0 "0" (core func (;0;)))
  (alias core export 0 "1" (core func (;1;)))
  (alias export 1 "create-note" (func (;0;)))
  (core func (;2;) (canon lower (func 0)))
  (core instance (;1;)
    (export "add-asset" (func 0))
    (export "remove-asset" (func 1))
    (export "create-note" (func 2))
  )
  (core instance (;2;) (instantiate 0
      (with "miden:base/tx-kernel@1.0.0" (instance 1))
    )
  )
  (alias core export 2 "memory" (core memory (;0;)))
  (alias core export 2 "cabi_realloc" (core func (;3;)))
  (alias core export 0 "$imports" (core table (;0;)))
  (alias export 1 "add-asset" (func (;1;)))
  (core func (;4;) (canon lower (func 1) (memory 0)))
  (alias export 1 "remove-asset" (func (;2;)))
  (core func (;5;) (canon lower (func 2) (memory 0)))
  (core instance (;3;)
    (export "$imports" (table 0))
    (export "0" (func 4))
    (export "1" (func 5))
  )
  (core instance (;4;) (instantiate 2
      (with "" (instance 3))
    )
  )
  (component (;0;))
  (instance (;2;) (instantiate 0))
  (export (;3;) "miden:base/account@1.0.0" (instance 2))
  (alias export 0 "asset" (type (;5;)))
  (type (;6;) (func (param "asset" 5)))
  (alias core export 2 "miden:basic-wallet/basic-wallet@1.0.0#receive-asset" (core func (;6;)))
  (func (;3;) (type 6) (canon lift (core func 6)))
  (alias export 0 "tag" (type (;7;)))
  (alias export 0 "recipient" (type (;8;)))
  (type (;9;) (func (param "asset" 5) (param "tag" 7) (param "recipient" 8)))
  (alias core export 2 "miden:basic-wallet/basic-wallet@1.0.0#send-asset" (core func (;7;)))
  (func (;4;) (type 9) (canon lift (core func 7)))
  (alias export 0 "felt" (type (;10;)))
  (alias export 0 "account-id" (type (;11;)))
  (alias export 0 "fungible-asset" (type (;12;)))
  (alias export 0 "word" (type (;13;)))
  (alias export 0 "non-fungible-asset" (type (;14;)))
  (alias export 0 "asset" (type (;15;)))
  (alias export 0 "tag" (type (;16;)))
  (alias export 0 "recipient" (type (;17;)))
  (component (;1;)
    (type (;0;) u64)
    (import "import-type-felt" (type (;1;) (eq 0)))
    (import "import-type-account-id" (type (;2;) (eq 1)))
    (type (;3;) (record (field "asset" 2) (field "amount" u64)))
    (import "import-type-fungible-asset" (type (;4;) (eq 3)))
    (type (;5;) (tuple 1 1 1 1))
    (import "import-type-word" (type (;6;) (eq 5)))
    (import "import-type-non-fungible-asset" (type (;7;) (eq 6)))
    (type (;8;) (variant (case "fungible" 4) (case "non-fungible" 7)))
    (import "import-type-asset" (type (;9;) (eq 8)))
    (import "import-type-tag" (type (;10;) (eq 1)))
    (import "import-type-recipient" (type (;11;) (eq 6)))
    (import "import-type-asset0" (type (;12;) (eq 9)))
    (type (;13;) (func (param "asset" 12)))
    (import "import-func-receive-asset" (func (;0;) (type 13)))
    (import "import-type-tag0" (type (;14;) (eq 10)))
    (import "import-type-recipient0" (type (;15;) (eq 11)))
    (type (;16;) (func (param "asset" 12) (param "tag" 14) (param "recipient" 15)))
    (import "import-func-send-asset" (func (;1;) (type 16)))
    (export (;17;) "asset" (type 9))
    (export (;18;) "tag" (type 10))
    (export (;19;) "recipient" (type 11))
    (type (;20;) (func (param "asset" 17)))
    (export (;2;) "receive-asset" (func 0) (func (type 20)))
    (type (;21;) (func (param "asset" 17) (param "tag" 18) (param "recipient" 19)))
    (export (;3;) "send-asset" (func 1) (func (type 21)))
  )
  (instance (;4;) (instantiate 1
      (with "import-func-receive-asset" (func 3))
      (with "import-func-send-asset" (func 4))
      (with "import-type-felt" (type 10))
      (with "import-type-account-id" (type 11))
      (with "import-type-fungible-asset" (type 12))
      (with "import-type-word" (type 13))
      (with "import-type-non-fungible-asset" (type 14))
      (with "import-type-asset" (type 15))
      (with "import-type-tag" (type 16))
      (with "import-type-recipient" (type 17))
      (with "import-type-asset0" (type 5))
      (with "import-type-tag0" (type 7))
      (with "import-type-recipient0" (type 8))
    )
  )
  (export (;5;) "miden:basic-wallet/basic-wallet@1.0.0" (instance 4))
)