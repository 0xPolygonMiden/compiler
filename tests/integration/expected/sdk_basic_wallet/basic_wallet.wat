(component
  (type (;0;)
    (instance
      (type (;0;) (record (field "inner" float64)))
      (export (;1;) "felt" (type (eq 0)))
      (type (;2;) (tuple 1 1 1 1))
      (export (;3;) "word" (type (eq 2)))
      (type (;4;) (record (field "inner" 3)))
      (export (;5;) "core-asset" (type (eq 4)))
      (type (;6;) (record (field "inner" 1)))
      (export (;7;) "tag" (type (eq 6)))
      (type (;8;) (record (field "inner" 3)))
      (export (;9;) "recipient" (type (eq 8)))
      (type (;10;) (record (field "inner" 1)))
      (export (;11;) "note-id" (type (eq 10)))
    )
  )
  (import "miden:base/core-types@1.0.0" (instance (;0;) (type 0)))
  (alias export 0 "core-asset" (type (;1;)))
  (type (;2;)
    (instance
      (alias outer 1 1 (type (;0;)))
      (export (;1;) "core-asset" (type (eq 0)))
      (type (;2;) (func (param "asset" 1) (result 1)))
      (export (;0;) "add-asset" (func (type 2)))
      (export (;1;) "remove-asset" (func (type 2)))
    )
  )
  (import "miden:base/account@1.0.0" (instance (;1;) (type 2)))
  (alias export 0 "core-asset" (type (;3;)))
  (alias export 0 "tag" (type (;4;)))
  (alias export 0 "recipient" (type (;5;)))
  (alias export 0 "note-id" (type (;6;)))
  (type (;7;)
    (instance
      (alias outer 1 3 (type (;0;)))
      (export (;1;) "core-asset" (type (eq 0)))
      (alias outer 1 4 (type (;2;)))
      (export (;3;) "tag" (type (eq 2)))
      (alias outer 1 5 (type (;4;)))
      (export (;5;) "recipient" (type (eq 4)))
      (alias outer 1 6 (type (;6;)))
      (export (;7;) "note-id" (type (eq 6)))
      (type (;8;) (func (param "asset" 1) (param "tag" 3) (param "recipient" 5) (result 7)))
      (export (;0;) "create-note" (func (type 8)))
    )
  )
  (import "miden:base/tx@1.0.0" (instance (;2;) (type 7)))
  (core module (;0;)
    (type (;0;) (func (param f64 f64 f64 f64 i32)))
    (type (;1;) (func (param f64 f64 f64 f64 f64 f64 f64 f64 f64) (result f64)))
    (type (;2;) (func))
    (type (;3;) (func (param f64 f64 f64 f64)))
    (type (;4;) (func (param f64 f64 f64 f64 f64 f64 f64 f64 f64)))
    (type (;5;) (func (param i32 i32) (result i32)))
    (type (;6;) (func (param i32 i32 i32 i32) (result i32)))
    (type (;7;) (func (param i32 i32 i32) (result i32)))
    (type (;8;) (func (param i32 i32 i32 i32)))
    (import "miden:base/account@1.0.0" "add-asset" (func $basic_wallet::bindings::miden::base::account::add_asset::wit_import (;0;) (type 0)))
    (import "miden:base/account@1.0.0" "remove-asset" (func $basic_wallet::bindings::miden::base::account::remove_asset::wit_import (;1;) (type 0)))
    (import "miden:base/tx@1.0.0" "create-note" (func $basic_wallet::bindings::miden::base::tx::create_note::wit_import (;2;) (type 1)))
    (func $__wasm_call_ctors (;3;) (type 2))
    (func $miden:basic-wallet/basic-wallet@1.0.0#receive-asset (;4;) (type 3) (param f64 f64 f64 f64)
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
    (func $miden:basic-wallet/basic-wallet@1.0.0#send-asset (;5;) (type 4) (param f64 f64 f64 f64 f64 f64 f64 f64 f64)
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
      f64.load
      local.get 9
      i32.const 8
      i32.add
      f64.load
      local.get 9
      i32.const 16
      i32.add
      f64.load
      local.get 9
      i32.const 24
      i32.add
      f64.load
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
      (local i32 i32 i32 i32 i32 i32)
      block ;; label = @1
        local.get 2
        i32.load
        local.tee 3
        i32.eqz
        br_if 0 (;@1;)
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
        loop ;; label = @2
          local.get 3
          i32.const 8
          i32.add
          local.set 7
          block ;; label = @3
            block ;; label = @4
              local.get 3
              i32.load offset=8
              local.tee 0
              i32.const 1
              i32.and
              br_if 0 (;@4;)
              local.get 3
              local.set 1
              br 1 (;@3;)
            end
            loop ;; label = @4
              local.get 7
              local.get 0
              i32.const -2
              i32.and
              i32.store
              local.get 3
              i32.load offset=4
              i32.const -4
              i32.and
              local.tee 1
              i32.load
              local.set 7
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    local.get 3
                    i32.load
                    local.tee 8
                    i32.const -4
                    i32.and
                    local.tee 0
                    br_if 0 (;@7;)
                    local.get 1
                    local.set 8
                    br 1 (;@6;)
                  end
                  block ;; label = @7
                    local.get 8
                    i32.const 2
                    i32.and
                    i32.eqz
                    br_if 0 (;@7;)
                    local.get 1
                    local.set 8
                    br 1 (;@6;)
                  end
                  local.get 0
                  local.get 0
                  i32.load offset=4
                  i32.const 3
                  i32.and
                  local.get 1
                  i32.or
                  i32.store offset=4
                  local.get 3
                  i32.load
                  local.set 0
                  local.get 3
                  i32.load offset=4
                  local.tee 7
                  i32.const -4
                  i32.and
                  local.tee 8
                  i32.eqz
                  br_if 1 (;@5;)
                  local.get 0
                  i32.const -4
                  i32.and
                  local.set 0
                  local.get 8
                  i32.load
                  local.set 7
                end
                local.get 8
                local.get 7
                i32.const 3
                i32.and
                local.get 0
                i32.or
                i32.store
                local.get 3
                i32.load offset=4
                local.set 7
                local.get 3
                i32.load
                local.set 0
              end
              local.get 3
              local.get 7
              i32.const 3
              i32.and
              i32.store offset=4
              local.get 3
              local.get 0
              i32.const 3
              i32.and
              i32.store
              block ;; label = @5
                local.get 0
                i32.const 2
                i32.and
                i32.eqz
                br_if 0 (;@5;)
                local.get 1
                local.get 1
                i32.load
                i32.const 2
                i32.or
                i32.store
              end
              local.get 2
              local.get 1
              i32.store
              local.get 1
              i32.const 8
              i32.add
              local.set 7
              local.get 1
              local.set 3
              local.get 1
              i32.load offset=8
              local.tee 0
              i32.const 1
              i32.and
              br_if 0 (;@4;)
            end
          end
          block ;; label = @3
            local.get 1
            i32.load
            i32.const -4
            i32.and
            local.tee 3
            local.get 7
            i32.sub
            local.get 6
            i32.lt_u
            br_if 0 (;@3;)
            block ;; label = @4
              block ;; label = @5
                local.get 7
                i32.const 72
                i32.add
                local.get 3
                local.get 6
                i32.sub
                local.get 5
                i32.and
                local.tee 3
                i32.le_u
                br_if 0 (;@5;)
                local.get 4
                local.get 7
                i32.and
                br_if 2 (;@3;)
                local.get 2
                local.get 0
                i32.const -4
                i32.and
                i32.store
                local.get 1
                i32.load
                local.set 0
                local.get 1
                local.set 3
                br 1 (;@4;)
              end
              i32.const 0
              local.set 0
              local.get 3
              i32.const 0
              i32.store
              local.get 3
              i32.const -8
              i32.add
              local.tee 3
              i64.const 0
              i64.store align=4
              local.get 3
              local.get 1
              i32.load
              i32.const -4
              i32.and
              i32.store
              block ;; label = @5
                local.get 1
                i32.load
                local.tee 2
                i32.const -4
                i32.and
                local.tee 8
                i32.eqz
                br_if 0 (;@5;)
                local.get 2
                i32.const 2
                i32.and
                br_if 0 (;@5;)
                local.get 8
                local.get 8
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
                local.set 0
              end
              local.get 3
              local.get 0
              local.get 1
              i32.or
              i32.store offset=4
              local.get 7
              local.get 7
              i32.load
              i32.const -2
              i32.and
              i32.store
              local.get 1
              local.get 1
              i32.load
              local.tee 0
              i32.const 3
              i32.and
              local.get 3
              i32.or
              local.tee 7
              i32.store
              block ;; label = @5
                local.get 0
                i32.const 2
                i32.and
                br_if 0 (;@5;)
                local.get 3
                i32.load
                local.set 0
                br 1 (;@4;)
              end
              local.get 1
              local.get 7
              i32.const -3
              i32.and
              i32.store
              local.get 3
              i32.load
              i32.const 2
              i32.or
              local.set 0
            end
            local.get 3
            local.get 0
            i32.const 1
            i32.or
            i32.store
            local.get 3
            i32.const 8
            i32.add
            return
          end
          local.get 2
          local.get 0
          i32.store
          local.get 0
          local.set 3
          local.get 0
          br_if 0 (;@2;)
        end
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
  (core module (;1;)
    (type (;0;) (func (param f64 f64 f64 f64 i32)))
    (func $indirect-miden:base/account@1.0.0-add-asset (;0;) (type 0) (param f64 f64 f64 f64 i32)
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      local.get 4
      i32.const 0
      call_indirect (type 0)
    )
    (func $indirect-miden:base/account@1.0.0-remove-asset (;1;) (type 0) (param f64 f64 f64 f64 i32)
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      local.get 4
      i32.const 1
      call_indirect (type 0)
    )
    (table (;0;) 2 2 funcref)
    (export "0" (func $indirect-miden:base/account@1.0.0-add-asset))
    (export "1" (func $indirect-miden:base/account@1.0.0-remove-asset))
    (export "$imports" (table 0))
  )
  (core module (;2;)
    (type (;0;) (func (param f64 f64 f64 f64 i32)))
    (import "" "0" (func (;0;) (type 0)))
    (import "" "1" (func (;1;) (type 0)))
    (import "" "$imports" (table (;0;) 2 2 funcref))
    (elem (;0;) (i32.const 0) func 0 1)
  )
  (core instance (;0;) (instantiate 1))
  (alias core export 0 "0" (core func (;0;)))
  (alias core export 0 "1" (core func (;1;)))
  (core instance (;1;)
    (export "add-asset" (func 0))
    (export "remove-asset" (func 1))
  )
  (alias export 2 "create-note" (func (;0;)))
  (core func (;2;) (canon lower (func 0)))
  (core instance (;2;)
    (export "create-note" (func 2))
  )
  (core instance (;3;) (instantiate 0
      (with "miden:base/account@1.0.0" (instance 1))
      (with "miden:base/tx@1.0.0" (instance 2))
    )
  )
  (alias core export 3 "memory" (core memory (;0;)))
  (alias core export 3 "cabi_realloc" (core func (;3;)))
  (alias core export 0 "$imports" (core table (;0;)))
  (alias export 1 "add-asset" (func (;1;)))
  (core func (;4;) (canon lower (func 1) (memory 0)))
  (alias export 1 "remove-asset" (func (;2;)))
  (core func (;5;) (canon lower (func 2) (memory 0)))
  (core instance (;4;)
    (export "$imports" (table 0))
    (export "0" (func 4))
    (export "1" (func 5))
  )
  (core instance (;5;) (instantiate 2
      (with "" (instance 4))
    )
  )
  (alias export 0 "core-asset" (type (;8;)))
  (type (;9;) (func (param "core-asset" 8)))
  (alias core export 3 "miden:basic-wallet/basic-wallet@1.0.0#receive-asset" (core func (;6;)))
  (func (;3;) (type 9) (canon lift (core func 6)))
  (alias export 0 "tag" (type (;10;)))
  (alias export 0 "recipient" (type (;11;)))
  (type (;12;) (func (param "core-asset" 8) (param "tag" 10) (param "recipient" 11)))
  (alias core export 3 "miden:basic-wallet/basic-wallet@1.0.0#send-asset" (core func (;7;)))
  (func (;4;) (type 12) (canon lift (core func 7)))
  (alias export 0 "felt" (type (;13;)))
  (alias export 0 "word" (type (;14;)))
  (alias export 0 "core-asset" (type (;15;)))
  (alias export 0 "tag" (type (;16;)))
  (alias export 0 "recipient" (type (;17;)))
  (component (;0;)
    (type (;0;) (record (field "inner" float64)))
    (import "import-type-felt" (type (;1;) (eq 0)))
    (type (;2;) (tuple 1 1 1 1))
    (import "import-type-word" (type (;3;) (eq 2)))
    (type (;4;) (record (field "inner" 3)))
    (import "import-type-core-asset" (type (;5;) (eq 4)))
    (type (;6;) (record (field "inner" 1)))
    (import "import-type-tag" (type (;7;) (eq 6)))
    (type (;8;) (record (field "inner" 3)))
    (import "import-type-recipient" (type (;9;) (eq 8)))
    (import "import-type-core-asset0" (type (;10;) (eq 5)))
    (type (;11;) (func (param "core-asset" 10)))
    (import "import-func-receive-asset" (func (;0;) (type 11)))
    (import "import-type-tag0" (type (;12;) (eq 7)))
    (import "import-type-recipient0" (type (;13;) (eq 9)))
    (type (;14;) (func (param "core-asset" 10) (param "tag" 12) (param "recipient" 13)))
    (import "import-func-send-asset" (func (;1;) (type 14)))
    (export (;15;) "core-asset" (type 5))
    (export (;16;) "tag" (type 7))
    (export (;17;) "recipient" (type 9))
    (type (;18;) (func (param "core-asset" 15)))
    (export (;2;) "receive-asset" (func 0) (func (type 18)))
    (type (;19;) (func (param "core-asset" 15) (param "tag" 16) (param "recipient" 17)))
    (export (;3;) "send-asset" (func 1) (func (type 19)))
  )
  (instance (;3;) (instantiate 0
      (with "import-func-receive-asset" (func 3))
      (with "import-func-send-asset" (func 4))
      (with "import-type-felt" (type 13))
      (with "import-type-word" (type 14))
      (with "import-type-core-asset" (type 15))
      (with "import-type-tag" (type 16))
      (with "import-type-recipient" (type 17))
      (with "import-type-core-asset0" (type 8))
      (with "import-type-tag0" (type 10))
      (with "import-type-recipient0" (type 11))
    )
  )
  (export (;4;) "miden:basic-wallet/basic-wallet@1.0.0" (instance 3))
)