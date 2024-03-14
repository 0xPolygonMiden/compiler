(component
  (type (;0;)
    (instance
      (type (;0;) (record (field "inner" float64)))
      (export (;1;) "felt" (type (eq 0)))
      (type (;2;) (record (field "inner" 1)))
      (export (;3;) "account-id" (type (eq 2)))
      (type (;4;) (tuple 1 1 1 1))
      (export (;5;) "word" (type (eq 4)))
      (type (;6;) (record (field "inner" 5)))
      (export (;7;) "core-asset" (type (eq 6)))
      (type (;8;) (func (param "felt" 1) (result 3)))
      (export (;0;) "account-id-from-felt" (func (type 8)))
    )
  )
  (import "miden:base/core-types@1.0.0" (instance (;0;) (type 0)))
  (alias export 0 "account-id" (type (;1;)))
  (type (;2;)
    (instance
      (alias outer 1 1 (type (;0;)))
      (export (;1;) "account-id" (type (eq 0)))
      (type (;2;) (func (result 1)))
      (export (;0;) "get-id" (func (type 2)))
    )
  )
  (import "miden:base/account@1.0.0" (instance (;1;) (type 2)))
  (alias export 0 "felt" (type (;3;)))
  (alias export 0 "core-asset" (type (;4;)))
  (type (;5;)
    (instance
      (alias outer 1 3 (type (;0;)))
      (export (;1;) "felt" (type (eq 0)))
      (alias outer 1 4 (type (;2;)))
      (export (;3;) "core-asset" (type (eq 2)))
      (type (;4;) (list 1))
      (type (;5;) (func (result 4)))
      (export (;0;) "get-inputs" (func (type 5)))
      (type (;6;) (list 3))
      (type (;7;) (func (result 6)))
      (export (;1;) "get-assets" (func (type 7)))
    )
  )
  (import "miden:base/note@1.0.0" (instance (;2;) (type 5)))
  (alias export 0 "core-asset" (type (;6;)))
  (type (;7;)
    (instance
      (alias outer 1 6 (type (;0;)))
      (export (;1;) "core-asset" (type (eq 0)))
      (type (;2;) (func (param "core-asset" 1)))
      (export (;0;) "receive-asset" (func (type 2)))
    )
  )
  (import "miden:basic-wallet/basic-wallet@1.0.0" (instance (;3;) (type 7)))
  (core module (;0;)
    (type (;0;) (func (param i32)))
    (type (;1;) (func (param f64) (result f64)))
    (type (;2;) (func (result f64)))
    (type (;3;) (func (param f64 f64 f64 f64)))
    (type (;4;) (func))
    (type (;5;) (func (param i32 i32) (result i32)))
    (type (;6;) (func (param i32 i32 i32 i32) (result i32)))
    (type (;7;) (func (param i32 i32 i32) (result i32)))
    (type (;8;) (func (param i32 i32 i32 i32)))
    (import "miden:base/note@1.0.0" "get-inputs" (func $basic_wallet_p2id_note::bindings::miden::base::note::get_inputs::wit_import (;0;) (type 0)))
    (import "miden:base/core-types@1.0.0" "account-id-from-felt" (func $basic_wallet_p2id_note::bindings::miden::base::core_types::account_id_from_felt::wit_import (;1;) (type 1)))
    (import "miden:base/account@1.0.0" "get-id" (func $basic_wallet_p2id_note::bindings::miden::base::account::get_id::wit_import (;2;) (type 2)))
    (import "miden:base/note@1.0.0" "get-assets" (func $basic_wallet_p2id_note::bindings::miden::base::note::get_assets::wit_import (;3;) (type 0)))
    (import "miden:basic-wallet/basic-wallet@1.0.0" "receive-asset" (func $basic_wallet_p2id_note::bindings::miden::basic_wallet::basic_wallet::receive_asset::wit_import (;4;) (type 3)))
    (func $__wasm_call_ctors (;5;) (type 4))
    (func $miden:base/note-script@1.0.0#note-script (;6;) (type 4)
      (local i32 i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 0
      global.set $__stack_pointer
      call $wit_bindgen::rt::run_ctors_once
      local.get 0
      i32.const 8
      i32.add
      call $basic_wallet_p2id_note::bindings::miden::base::note::get_inputs::wit_import
      block ;; label = @1
        local.get 0
        i32.const 12
        i32.add
        i32.load
        local.tee 1
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=8
        local.tee 2
        f64.load
        call $basic_wallet_p2id_note::bindings::miden::base::core_types::account_id_from_felt::wit_import
        call $basic_wallet_p2id_note::bindings::miden::base::account::get_id::wit_import
        f64.ne
        br_if 0 (;@1;)
        local.get 0
        i32.const 8
        i32.add
        call $basic_wallet_p2id_note::bindings::miden::base::note::get_assets::wit_import
        block ;; label = @2
          local.get 0
          i32.const 12
          i32.add
          i32.load
          local.tee 3
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=8
          local.tee 4
          local.get 3
          i32.const 5
          i32.shl
          i32.add
          local.set 5
          local.get 4
          local.set 6
          loop ;; label = @3
            local.get 6
            f64.load
            local.get 6
            f64.load offset=8
            local.get 6
            f64.load offset=16
            local.get 6
            f64.load offset=24
            call $basic_wallet_p2id_note::bindings::miden::basic_wallet::basic_wallet::receive_asset::wit_import
            local.get 6
            i32.const 32
            i32.add
            local.tee 6
            local.get 5
            i32.ne
            br_if 0 (;@3;)
          end
          i32.const 1048576
          local.get 4
          i32.const 8
          local.get 3
          i32.const 5
          i32.shl
          call $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::dealloc
        end
        i32.const 1048576
        local.get 2
        i32.const 8
        local.get 1
        i32.const 3
        i32.shl
        call $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::dealloc
        local.get 0
        i32.const 16
        i32.add
        global.set $__stack_pointer
        return
      end
      unreachable
      unreachable
    )
    (func $__rust_alloc (;7;) (type 5) (param i32 i32) (result i32)
      i32.const 1048576
      local.get 1
      local.get 0
      call $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::alloc
    )
    (func $__rust_realloc (;8;) (type 6) (param i32 i32 i32 i32) (result i32)
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
    (func $wee_alloc::alloc_first_fit (;9;) (type 7) (param i32 i32 i32) (result i32)
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
    (func $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::alloc (;10;) (type 7) (param i32 i32 i32) (result i32)
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
    (func $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::dealloc (;11;) (type 8) (param i32 i32 i32 i32)
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
    (func $wit_bindgen::rt::run_ctors_once (;12;) (type 4)
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
    (func $cabi_realloc (;13;) (type 6) (param i32 i32 i32 i32) (result i32)
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
    (export "miden:base/note-script@1.0.0#note-script" (func $miden:base/note-script@1.0.0#note-script))
    (export "cabi_realloc" (func $cabi_realloc))
  )
  (core module (;1;)
    (type (;0;) (func (param i32)))
    (func $indirect-miden:base/note@1.0.0-get-inputs (;0;) (type 0) (param i32)
      local.get 0
      i32.const 0
      call_indirect (type 0)
    )
    (func $indirect-miden:base/note@1.0.0-get-assets (;1;) (type 0) (param i32)
      local.get 0
      i32.const 1
      call_indirect (type 0)
    )
    (table (;0;) 2 2 funcref)
    (export "0" (func $indirect-miden:base/note@1.0.0-get-inputs))
    (export "1" (func $indirect-miden:base/note@1.0.0-get-assets))
    (export "$imports" (table 0))
  )
  (core module (;2;)
    (type (;0;) (func (param i32)))
    (import "" "0" (func (;0;) (type 0)))
    (import "" "1" (func (;1;) (type 0)))
    (import "" "$imports" (table (;0;) 2 2 funcref))
    (elem (;0;) (i32.const 0) func 0 1)
  )
  (core instance (;0;) (instantiate 1))
  (alias core export 0 "0" (core func (;0;)))
  (alias core export 0 "1" (core func (;1;)))
  (core instance (;1;)
    (export "get-inputs" (func 0))
    (export "get-assets" (func 1))
  )
  (alias export 0 "account-id-from-felt" (func (;0;)))
  (core func (;2;) (canon lower (func 0)))
  (core instance (;2;)
    (export "account-id-from-felt" (func 2))
  )
  (alias export 1 "get-id" (func (;1;)))
  (core func (;3;) (canon lower (func 1)))
  (core instance (;3;)
    (export "get-id" (func 3))
  )
  (alias export 3 "receive-asset" (func (;2;)))
  (core func (;4;) (canon lower (func 2)))
  (core instance (;4;)
    (export "receive-asset" (func 4))
  )
  (core instance (;5;) (instantiate 0
      (with "miden:base/note@1.0.0" (instance 1))
      (with "miden:base/core-types@1.0.0" (instance 2))
      (with "miden:base/account@1.0.0" (instance 3))
      (with "miden:basic-wallet/basic-wallet@1.0.0" (instance 4))
    )
  )
  (alias core export 5 "memory" (core memory (;0;)))
  (alias core export 5 "cabi_realloc" (core func (;5;)))
  (alias core export 0 "$imports" (core table (;0;)))
  (alias export 2 "get-inputs" (func (;3;)))
  (core func (;6;) (canon lower (func 3) (memory 0) (realloc 5)))
  (alias export 2 "get-assets" (func (;4;)))
  (core func (;7;) (canon lower (func 4) (memory 0) (realloc 5)))
  (core instance (;6;)
    (export "$imports" (table 0))
    (export "0" (func 6))
    (export "1" (func 7))
  )
  (core instance (;7;) (instantiate 2
      (with "" (instance 6))
    )
  )
  (type (;8;) (func))
  (alias core export 5 "miden:base/note-script@1.0.0#note-script" (core func (;8;)))
  (func (;5;) (type 8) (canon lift (core func 8)))
  (component (;0;)
    (type (;0;) (func))
    (import "import-func-note-script" (func (;0;) (type 0)))
    (type (;1;) (func))
    (export (;1;) "note-script" (func 0) (func (type 1)))
  )
  (instance (;4;) (instantiate 0
      (with "import-func-note-script" (func 5))
    )
  )
  (export (;5;) "miden:base/note-script@1.0.0" (instance 4))
)