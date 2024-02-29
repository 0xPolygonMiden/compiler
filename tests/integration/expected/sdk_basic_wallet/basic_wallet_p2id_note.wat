(component
  (type (;0;)
    (instance
      (type (;0;) (record (field "inner" u64)))
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
    (type (;0;) (func (param i32 i32 i32) (result i32)))
    (type (;1;) (func (param i32 i32) (result i32)))
    (type (;2;) (func (param i32)))
    (type (;3;) (func (param i64) (result i64)))
    (type (;4;) (func (result i64)))
    (type (;5;) (func (param i64 i64 i64 i64)))
    (type (;6;) (func))
    (type (;7;) (func (param i32 i32)))
    (type (;8;) (func (param i32 i32 i32)))
    (type (;9;) (func (param i32 i32 i32 i32) (result i32)))
    (type (;10;) (func (param i32 i32 i32 i32)))
    (type (;11;) (func (param i32 i32 i32 i32 i32 i32) (result i32)))
    (type (;12;) (func (param i32 i32 i32 i32 i32) (result i32)))
    (type (;13;) (func (param i64 i32 i32) (result i32)))
    (import "miden:base/note@1.0.0" "get-inputs" (func $basic_wallet_p2id_note::bindings::miden::base::note::get_inputs::wit_import (;0;) (type 2)))
    (import "miden:base/core-types@1.0.0" "account-id-from-felt" (func $basic_wallet_p2id_note::bindings::miden::base::core_types::account_id_from_felt::wit_import (;1;) (type 3)))
    (import "miden:base/account@1.0.0" "get-id" (func $basic_wallet_p2id_note::bindings::miden::base::account::get_id::wit_import (;2;) (type 4)))
    (import "miden:base/note@1.0.0" "get-assets" (func $basic_wallet_p2id_note::bindings::miden::base::note::get_assets::wit_import (;3;) (type 2)))
    (import "miden:basic-wallet/basic-wallet@1.0.0" "receive-asset" (func $basic_wallet_p2id_note::bindings::miden::basic_wallet::basic_wallet::receive_asset::wit_import (;4;) (type 5)))
    (func $__wasm_call_ctors (;5;) (type 6))
    (func $<alloc::alloc::Global as core::alloc::Allocator>::deallocate (;6;) (type 7) (param i32 i32)
      block ;; label = @1
        local.get 1
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        local.get 1
        i32.const 8
        call $__rust_dealloc
      end
    )
    (func $__rust_dealloc (;7;) (type 8) (param i32 i32 i32)
      i32.const 1048888
      local.get 0
      local.get 2
      local.get 1
      call $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::dealloc
    )
    (func $rust_begin_unwind (;8;) (type 2) (param i32)
      unreachable
      unreachable
    )
    (func $miden:base/note-script@1.0.0#note-script (;9;) (type 6)
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
        local.tee 1
        i32.load
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=8
        local.tee 3
        i64.load
        call $basic_wallet_p2id_note::bindings::miden::base::core_types::account_id_from_felt::wit_import
        drop
        call $basic_wallet_p2id_note::bindings::miden::base::account::get_id::wit_import
        drop
        local.get 0
        i32.const 8
        i32.add
        call $basic_wallet_p2id_note::bindings::miden::base::note::get_assets::wit_import
        local.get 1
        i32.load
        local.tee 4
        i32.const 5
        i32.shl
        local.set 5
        local.get 0
        i32.load offset=8
        local.tee 6
        local.set 1
        loop ;; label = @2
          block ;; label = @3
            local.get 5
            br_if 0 (;@3;)
            block ;; label = @4
              local.get 4
              i32.eqz
              br_if 0 (;@4;)
              local.get 6
              local.get 4
              i32.const 5
              i32.shl
              call $<alloc::alloc::Global as core::alloc::Allocator>::deallocate
            end
            local.get 3
            local.get 2
            i32.const 3
            i32.shl
            call $<alloc::alloc::Global as core::alloc::Allocator>::deallocate
            local.get 0
            i32.const 16
            i32.add
            global.set $__stack_pointer
            return
          end
          local.get 1
          i64.load
          local.get 1
          i64.load offset=8
          local.get 1
          i64.load offset=16
          local.get 1
          i64.load offset=24
          call $basic_wallet_p2id_note::bindings::miden::basic_wallet::basic_wallet::receive_asset::wit_import
          local.get 5
          i32.const -32
          i32.add
          local.set 5
          local.get 1
          i32.const 32
          i32.add
          local.set 1
          br 0 (;@2;)
        end
      end
      i32.const 0
      i32.const 0
      i32.const 1048588
      call $core::panicking::panic_bounds_check
      unreachable
    )
    (func $__rust_alloc (;10;) (type 1) (param i32 i32) (result i32)
      i32.const 1048888
      local.get 1
      local.get 0
      call $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::alloc
    )
    (func $__rust_realloc (;11;) (type 9) (param i32 i32 i32 i32) (result i32)
      (local i32)
      block ;; label = @1
        i32.const 1048888
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
        i32.const 1048888
        local.get 0
        local.get 2
        local.get 1
        call $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::dealloc
      end
      local.get 4
    )
    (func $wee_alloc::neighbors::Neighbors<T>::remove (;12;) (type 2) (param i32)
      (local i32 i32 i32)
      block ;; label = @1
        local.get 0
        i32.load
        local.tee 1
        i32.const -4
        i32.and
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        i32.const 0
        local.get 2
        local.get 1
        i32.const 2
        i32.and
        select
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 2
        local.get 2
        i32.load offset=4
        i32.const 3
        i32.and
        local.get 0
        i32.load offset=4
        i32.const -4
        i32.and
        i32.or
        i32.store offset=4
        local.get 0
        i32.load
        local.set 1
      end
      block ;; label = @1
        local.get 0
        i32.load offset=4
        local.tee 2
        i32.const -4
        i32.and
        local.tee 3
        i32.eqz
        br_if 0 (;@1;)
        local.get 3
        local.get 3
        i32.load
        i32.const 3
        i32.and
        local.get 1
        i32.const -4
        i32.and
        i32.or
        i32.store
        local.get 0
        i32.load offset=4
        local.set 2
        local.get 0
        i32.load
        local.set 1
      end
      local.get 0
      local.get 2
      i32.const 3
      i32.and
      i32.store offset=4
      local.get 0
      local.get 1
      i32.const 3
      i32.and
      i32.store
    )
    (func $<wee_alloc::LargeAllocPolicy as wee_alloc::AllocPolicy>::new_cell_for_free_list (;13;) (type 10) (param i32 i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          local.get 2
          i32.const 2
          i32.shl
          local.tee 2
          local.get 3
          i32.const 3
          i32.shl
          i32.const 512
          i32.add
          local.tee 3
          local.get 2
          local.get 3
          i32.gt_u
          select
          i32.const 65543
          i32.add
          local.tee 3
          i32.const 16
          i32.shr_u
          memory.grow
          local.tee 2
          i32.const -1
          i32.ne
          br_if 0 (;@2;)
          i32.const 1
          local.set 3
          i32.const 0
          local.set 2
          br 1 (;@1;)
        end
        local.get 2
        i32.const 16
        i32.shl
        local.tee 2
        i64.const 0
        i64.store offset=4 align=4
        local.get 2
        local.get 2
        local.get 3
        i32.const -65536
        i32.and
        i32.add
        i32.const 2
        i32.or
        i32.store
        i32.const 0
        local.set 3
      end
      local.get 0
      local.get 2
      i32.store offset=4
      local.get 0
      local.get 3
      i32.store
    )
    (func $wee_alloc::alloc_first_fit (;14;) (type 0) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32)
      local.get 1
      i32.const -1
      i32.add
      local.set 3
      i32.const 0
      local.set 4
      i32.const 0
      local.get 1
      i32.sub
      local.set 5
      local.get 0
      i32.const 2
      i32.shl
      local.set 6
      local.get 2
      i32.load
      local.set 0
      loop (result i32) ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 0
            i32.eqz
            br_if 0 (;@3;)
            local.get 0
            local.set 1
            block ;; label = @4
              block ;; label = @5
                loop ;; label = @6
                  block ;; label = @7
                    local.get 1
                    i32.load offset=8
                    local.tee 0
                    i32.const 1
                    i32.and
                    br_if 0 (;@7;)
                    local.get 1
                    i32.load
                    i32.const -4
                    i32.and
                    local.tee 7
                    local.get 1
                    i32.const 8
                    i32.add
                    local.tee 8
                    i32.sub
                    local.get 6
                    i32.lt_u
                    br_if 5 (;@2;)
                    block ;; label = @8
                      local.get 8
                      i32.const 72
                      i32.add
                      local.get 7
                      local.get 6
                      i32.sub
                      local.get 5
                      i32.and
                      local.tee 7
                      i32.le_u
                      br_if 0 (;@8;)
                      local.get 3
                      local.get 8
                      i32.and
                      br_if 6 (;@2;)
                      local.get 2
                      local.get 0
                      i32.const -4
                      i32.and
                      i32.store
                      local.get 1
                      i32.load
                      local.set 2
                      local.get 1
                      local.set 0
                      br 4 (;@4;)
                    end
                    i32.const 0
                    local.set 2
                    local.get 7
                    i32.const 0
                    i32.store
                    local.get 7
                    i32.const -8
                    i32.add
                    local.tee 0
                    i64.const 0
                    i64.store align=4
                    local.get 0
                    local.get 1
                    i32.load
                    i32.const -4
                    i32.and
                    i32.store
                    block ;; label = @8
                      local.get 1
                      i32.load
                      local.tee 8
                      i32.const -4
                      i32.and
                      local.tee 6
                      i32.eqz
                      br_if 0 (;@8;)
                      i32.const 0
                      local.get 6
                      local.get 8
                      i32.const 2
                      i32.and
                      select
                      local.tee 8
                      i32.eqz
                      br_if 0 (;@8;)
                      local.get 8
                      local.get 8
                      i32.load offset=4
                      i32.const 3
                      i32.and
                      local.get 0
                      i32.or
                      i32.store offset=4
                      local.get 0
                      i32.load offset=4
                      i32.const 3
                      i32.and
                      local.set 2
                    end
                    local.get 0
                    local.get 2
                    local.get 1
                    i32.or
                    i32.store offset=4
                    local.get 1
                    local.get 1
                    i32.load offset=8
                    i32.const -2
                    i32.and
                    i32.store offset=8
                    local.get 1
                    local.get 1
                    i32.load
                    local.tee 2
                    i32.const 3
                    i32.and
                    local.get 0
                    i32.or
                    local.tee 8
                    i32.store
                    local.get 2
                    i32.const 2
                    i32.and
                    br_if 2 (;@5;)
                    local.get 0
                    i32.load
                    local.set 2
                    br 3 (;@4;)
                  end
                  local.get 1
                  local.get 0
                  i32.const -2
                  i32.and
                  i32.store offset=8
                  block ;; label = @7
                    block ;; label = @8
                      local.get 1
                      i32.load offset=4
                      i32.const -4
                      i32.and
                      local.tee 0
                      br_if 0 (;@8;)
                      i32.const 0
                      local.set 0
                      br 1 (;@7;)
                    end
                    i32.const 0
                    local.get 0
                    local.get 0
                    i32.load8_u
                    i32.const 1
                    i32.and
                    select
                    local.set 0
                  end
                  local.get 1
                  call $wee_alloc::neighbors::Neighbors<T>::remove
                  block ;; label = @7
                    local.get 1
                    i32.load8_u
                    i32.const 2
                    i32.and
                    i32.eqz
                    br_if 0 (;@7;)
                    local.get 0
                    local.get 0
                    i32.load
                    i32.const 2
                    i32.or
                    i32.store
                  end
                  local.get 2
                  local.get 0
                  i32.store
                  local.get 0
                  local.set 1
                  br 0 (;@6;)
                end
              end
              local.get 1
              local.get 8
              i32.const -3
              i32.and
              i32.store
              local.get 0
              i32.load
              i32.const 2
              i32.or
              local.set 2
            end
            local.get 0
            local.get 2
            i32.const 1
            i32.or
            i32.store
            local.get 0
            i32.const 8
            i32.add
            local.set 4
          end
          local.get 4
          return
        end
        local.get 2
        local.get 0
        i32.store
        br 0 (;@1;)
      end
    )
    (func $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::alloc (;15;) (type 0) (param i32 i32 i32) (result i32)
      (local i32 i32)
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
          i32.const 2
          i32.shr_u
          local.tee 4
          local.get 1
          local.get 3
          i32.const 12
          i32.add
          call $wee_alloc::alloc_first_fit
          local.tee 2
          br_if 0 (;@2;)
          local.get 3
          local.get 3
          local.get 4
          local.get 1
          call $<wee_alloc::LargeAllocPolicy as wee_alloc::AllocPolicy>::new_cell_for_free_list
          i32.const 0
          local.set 2
          local.get 3
          i32.load
          br_if 0 (;@2;)
          local.get 3
          i32.load offset=4
          local.tee 2
          local.get 3
          i32.load offset=12
          i32.store offset=8
          local.get 3
          local.get 2
          i32.store offset=12
          local.get 4
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
    (func $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::dealloc (;16;) (type 10) (param i32 i32 i32 i32)
      (local i32 i32 i32)
      block ;; label = @1
        local.get 1
        i32.eqz
        br_if 0 (;@1;)
        local.get 3
        i32.eqz
        br_if 0 (;@1;)
        local.get 1
        i32.const 0
        i32.store
        local.get 1
        i32.const -8
        i32.add
        local.tee 3
        local.get 3
        i32.load
        local.tee 4
        i32.const -2
        i32.and
        i32.store
        local.get 0
        i32.load
        local.set 5
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 3
              i32.const 4
              i32.add
              i32.load
              i32.const -4
              i32.and
              local.tee 6
              i32.eqz
              br_if 0 (;@4;)
              local.get 6
              i32.load8_u
              i32.const 1
              i32.and
              br_if 0 (;@4;)
              local.get 3
              call $wee_alloc::neighbors::Neighbors<T>::remove
              local.get 3
              i32.load8_u
              i32.const 2
              i32.and
              i32.eqz
              br_if 1 (;@3;)
              local.get 6
              local.get 6
              i32.load
              i32.const 2
              i32.or
              i32.store
              br 1 (;@3;)
            end
            block ;; label = @4
              block ;; label = @5
                local.get 4
                i32.const -4
                i32.and
                local.tee 6
                i32.eqz
                br_if 0 (;@5;)
                i32.const 0
                local.get 6
                local.get 4
                i32.const 2
                i32.and
                select
                local.tee 4
                i32.eqz
                br_if 0 (;@5;)
                local.get 4
                i32.load8_u
                i32.const 1
                i32.and
                i32.eqz
                br_if 1 (;@4;)
              end
              local.get 1
              local.get 5
              i32.store
              br 2 (;@2;)
            end
            local.get 1
            local.get 4
            i32.load offset=8
            i32.const -4
            i32.and
            i32.store
            local.get 4
            local.get 3
            i32.const 1
            i32.or
            i32.store offset=8
          end
          local.get 5
          local.set 3
        end
        local.get 0
        local.get 3
        i32.store
      end
    )
    (func $wit_bindgen::rt::run_ctors_once (;17;) (type 6)
      block ;; label = @1
        i32.const 0
        i32.load8_u offset=1048893
        br_if 0 (;@1;)
        call $__wasm_call_ctors
        i32.const 0
        i32.const 1
        i32.store8 offset=1048893
      end
    )
    (func $cabi_realloc (;18;) (type 9) (param i32 i32 i32 i32) (result i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 1
            br_if 0 (;@3;)
            local.get 3
            i32.eqz
            br_if 2 (;@1;)
            i32.const 0
            i32.load8_u offset=1048892
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
    (func $core::ptr::drop_in_place<core::fmt::Error> (;19;) (type 2) (param i32))
    (func $core::panicking::panic_fmt (;20;) (type 7) (param i32 i32)
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
      i32.const 1048604
      i32.store offset=16
      local.get 2
      i32.const 1048604
      i32.store offset=12
      local.get 2
      i32.const 12
      i32.add
      call $rust_begin_unwind
      unreachable
    )
    (func $core::panicking::panic_bounds_check (;21;) (type 8) (param i32 i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      local.get 1
      i32.store offset=4
      local.get 3
      local.get 0
      i32.store
      local.get 3
      i32.const 8
      i32.add
      i32.const 12
      i32.add
      i64.const 2
      i64.store align=4
      local.get 3
      i32.const 32
      i32.add
      i32.const 12
      i32.add
      i32.const 1
      i32.store
      local.get 3
      i32.const 2
      i32.store offset=12
      local.get 3
      i32.const 1048672
      i32.store offset=8
      local.get 3
      i32.const 1
      i32.store offset=36
      local.get 3
      local.get 3
      i32.const 32
      i32.add
      i32.store offset=16
      local.get 3
      local.get 3
      i32.store offset=40
      local.get 3
      local.get 3
      i32.const 4
      i32.add
      i32.store offset=32
      local.get 3
      i32.const 8
      i32.add
      local.get 2
      call $core::panicking::panic_fmt
      unreachable
    )
    (func $core::fmt::num::imp::<impl core::fmt::Display for u32>::fmt (;22;) (type 1) (param i32 i32) (result i32)
      local.get 0
      i64.load32_u
      i32.const 1
      local.get 1
      call $core::fmt::num::imp::fmt_u64
    )
    (func $<T as core::any::Any>::type_id (;23;) (type 7) (param i32 i32)
      local.get 0
      i64.const -832627268303839913
      i64.store offset=8
      local.get 0
      i64.const -2179734974050036201
      i64.store
    )
    (func $core::fmt::Formatter::pad_integral (;24;) (type 11) (param i32 i32 i32 i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          local.get 1
          br_if 0 (;@2;)
          local.get 5
          i32.const 1
          i32.add
          local.set 6
          local.get 0
          i32.load offset=28
          local.set 7
          i32.const 45
          local.set 8
          br 1 (;@1;)
        end
        i32.const 43
        i32.const 1114112
        local.get 0
        i32.load offset=28
        local.tee 7
        i32.const 1
        i32.and
        local.tee 1
        select
        local.set 8
        local.get 1
        local.get 5
        i32.add
        local.set 6
      end
      block ;; label = @1
        block ;; label = @2
          local.get 7
          i32.const 4
          i32.and
          br_if 0 (;@2;)
          i32.const 0
          local.set 2
          br 1 (;@1;)
        end
        block ;; label = @2
          block ;; label = @3
            local.get 3
            i32.const 16
            i32.lt_u
            br_if 0 (;@3;)
            local.get 2
            local.get 3
            call $core::str::count::do_count_chars
            local.set 1
            br 1 (;@2;)
          end
          block ;; label = @3
            local.get 3
            br_if 0 (;@3;)
            i32.const 0
            local.set 1
            br 1 (;@2;)
          end
          local.get 3
          i32.const 3
          i32.and
          local.set 9
          block ;; label = @3
            block ;; label = @4
              local.get 3
              i32.const 4
              i32.ge_u
              br_if 0 (;@4;)
              i32.const 0
              local.set 1
              i32.const 0
              local.set 10
              br 1 (;@3;)
            end
            local.get 3
            i32.const -4
            i32.and
            local.set 11
            i32.const 0
            local.set 1
            i32.const 0
            local.set 10
            loop ;; label = @4
              local.get 1
              local.get 2
              local.get 10
              i32.add
              local.tee 12
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.get 12
              i32.const 1
              i32.add
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.get 12
              i32.const 2
              i32.add
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.get 12
              i32.const 3
              i32.add
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.set 1
              local.get 11
              local.get 10
              i32.const 4
              i32.add
              local.tee 10
              i32.ne
              br_if 0 (;@4;)
            end
          end
          local.get 9
          i32.eqz
          br_if 0 (;@2;)
          local.get 2
          local.get 10
          i32.add
          local.set 12
          loop ;; label = @3
            local.get 1
            local.get 12
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.set 1
            local.get 12
            i32.const 1
            i32.add
            local.set 12
            local.get 9
            i32.const -1
            i32.add
            local.tee 9
            br_if 0 (;@3;)
          end
        end
        local.get 1
        local.get 6
        i32.add
        local.set 6
      end
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.load
          br_if 0 (;@2;)
          i32.const 1
          local.set 1
          local.get 0
          i32.load offset=20
          local.tee 12
          local.get 0
          i32.load offset=24
          local.tee 10
          local.get 8
          local.get 2
          local.get 3
          call $core::fmt::Formatter::pad_integral::write_prefix
          br_if 1 (;@1;)
          local.get 12
          local.get 4
          local.get 5
          local.get 10
          i32.load offset=12
          call_indirect (type 0)
          return
        end
        block ;; label = @2
          local.get 0
          i32.load offset=4
          local.tee 9
          local.get 6
          i32.gt_u
          br_if 0 (;@2;)
          i32.const 1
          local.set 1
          local.get 0
          i32.load offset=20
          local.tee 12
          local.get 0
          i32.load offset=24
          local.tee 10
          local.get 8
          local.get 2
          local.get 3
          call $core::fmt::Formatter::pad_integral::write_prefix
          br_if 1 (;@1;)
          local.get 12
          local.get 4
          local.get 5
          local.get 10
          i32.load offset=12
          call_indirect (type 0)
          return
        end
        block ;; label = @2
          local.get 7
          i32.const 8
          i32.and
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=16
          local.set 11
          local.get 0
          i32.const 48
          i32.store offset=16
          local.get 0
          i32.load8_u offset=32
          local.set 7
          i32.const 1
          local.set 1
          local.get 0
          i32.const 1
          i32.store8 offset=32
          local.get 0
          i32.load offset=20
          local.tee 12
          local.get 0
          i32.load offset=24
          local.tee 10
          local.get 8
          local.get 2
          local.get 3
          call $core::fmt::Formatter::pad_integral::write_prefix
          br_if 1 (;@1;)
          local.get 9
          local.get 6
          i32.sub
          i32.const 1
          i32.add
          local.set 1
          block ;; label = @3
            loop ;; label = @4
              local.get 1
              i32.const -1
              i32.add
              local.tee 1
              i32.eqz
              br_if 1 (;@3;)
              local.get 12
              i32.const 48
              local.get 10
              i32.load offset=16
              call_indirect (type 1)
              i32.eqz
              br_if 0 (;@4;)
            end
            i32.const 1
            return
          end
          i32.const 1
          local.set 1
          local.get 12
          local.get 4
          local.get 5
          local.get 10
          i32.load offset=12
          call_indirect (type 0)
          br_if 1 (;@1;)
          local.get 0
          local.get 7
          i32.store8 offset=32
          local.get 0
          local.get 11
          i32.store offset=16
          i32.const 0
          local.set 1
          br 1 (;@1;)
        end
        local.get 9
        local.get 6
        i32.sub
        local.set 6
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 0
              i32.load8_u offset=32
              local.tee 1
              br_table 2 (;@2;) 0 (;@4;) 1 (;@3;) 0 (;@4;) 2 (;@2;)
            end
            local.get 6
            local.set 1
            i32.const 0
            local.set 6
            br 1 (;@2;)
          end
          local.get 6
          i32.const 1
          i32.shr_u
          local.set 1
          local.get 6
          i32.const 1
          i32.add
          i32.const 1
          i32.shr_u
          local.set 6
        end
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 0
        i32.const 24
        i32.add
        i32.load
        local.set 12
        local.get 0
        i32.load offset=16
        local.set 9
        local.get 0
        i32.load offset=20
        local.set 10
        block ;; label = @2
          loop ;; label = @3
            local.get 1
            i32.const -1
            i32.add
            local.tee 1
            i32.eqz
            br_if 1 (;@2;)
            local.get 10
            local.get 9
            local.get 12
            i32.load offset=16
            call_indirect (type 1)
            i32.eqz
            br_if 0 (;@3;)
          end
          i32.const 1
          return
        end
        i32.const 1
        local.set 1
        local.get 10
        local.get 12
        local.get 8
        local.get 2
        local.get 3
        call $core::fmt::Formatter::pad_integral::write_prefix
        br_if 0 (;@1;)
        local.get 10
        local.get 4
        local.get 5
        local.get 12
        i32.load offset=12
        call_indirect (type 0)
        br_if 0 (;@1;)
        i32.const 0
        local.set 1
        loop ;; label = @2
          block ;; label = @3
            local.get 6
            local.get 1
            i32.ne
            br_if 0 (;@3;)
            local.get 6
            local.get 6
            i32.lt_u
            return
          end
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 10
          local.get 9
          local.get 12
          i32.load offset=16
          call_indirect (type 1)
          i32.eqz
          br_if 0 (;@2;)
        end
        local.get 1
        i32.const -1
        i32.add
        local.get 6
        i32.lt_u
        return
      end
      local.get 1
    )
    (func $core::str::count::do_count_chars (;25;) (type 1) (param i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          local.get 1
          local.get 0
          i32.const 3
          i32.add
          i32.const -4
          i32.and
          local.tee 2
          local.get 0
          i32.sub
          local.tee 3
          i32.lt_u
          br_if 0 (;@2;)
          local.get 1
          local.get 3
          i32.sub
          local.tee 4
          i32.const 4
          i32.lt_u
          br_if 0 (;@2;)
          local.get 4
          i32.const 3
          i32.and
          local.set 5
          i32.const 0
          local.set 6
          i32.const 0
          local.set 1
          block ;; label = @3
            local.get 2
            local.get 0
            i32.eq
            local.tee 7
            br_if 0 (;@3;)
            i32.const 0
            local.set 1
            block ;; label = @4
              block ;; label = @5
                local.get 2
                local.get 0
                i32.const -1
                i32.xor
                i32.add
                i32.const 3
                i32.ge_u
                br_if 0 (;@5;)
                i32.const 0
                local.set 8
                br 1 (;@4;)
              end
              i32.const 0
              local.set 8
              loop ;; label = @5
                local.get 1
                local.get 0
                local.get 8
                i32.add
                local.tee 9
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.get 9
                i32.const 1
                i32.add
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.get 9
                i32.const 2
                i32.add
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.get 9
                i32.const 3
                i32.add
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.set 1
                local.get 8
                i32.const 4
                i32.add
                local.tee 8
                br_if 0 (;@5;)
              end
            end
            local.get 7
            br_if 0 (;@3;)
            local.get 0
            local.get 2
            i32.sub
            local.set 2
            local.get 0
            local.get 8
            i32.add
            local.set 9
            loop ;; label = @4
              local.get 1
              local.get 9
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.set 1
              local.get 9
              i32.const 1
              i32.add
              local.set 9
              local.get 2
              i32.const 1
              i32.add
              local.tee 2
              br_if 0 (;@4;)
            end
          end
          local.get 0
          local.get 3
          i32.add
          local.set 8
          block ;; label = @3
            local.get 5
            i32.eqz
            br_if 0 (;@3;)
            local.get 8
            local.get 4
            i32.const -4
            i32.and
            i32.add
            local.tee 9
            i32.load8_s
            i32.const -65
            i32.gt_s
            local.set 6
            local.get 5
            i32.const 1
            i32.eq
            br_if 0 (;@3;)
            local.get 6
            local.get 9
            i32.load8_s offset=1
            i32.const -65
            i32.gt_s
            i32.add
            local.set 6
            local.get 5
            i32.const 2
            i32.eq
            br_if 0 (;@3;)
            local.get 6
            local.get 9
            i32.load8_s offset=2
            i32.const -65
            i32.gt_s
            i32.add
            local.set 6
          end
          local.get 4
          i32.const 2
          i32.shr_u
          local.set 3
          local.get 6
          local.get 1
          i32.add
          local.set 2
          loop ;; label = @3
            local.get 8
            local.set 6
            local.get 3
            i32.eqz
            br_if 2 (;@1;)
            local.get 3
            i32.const 192
            local.get 3
            i32.const 192
            i32.lt_u
            select
            local.tee 4
            i32.const 3
            i32.and
            local.set 7
            local.get 4
            i32.const 2
            i32.shl
            local.set 5
            i32.const 0
            local.set 9
            block ;; label = @4
              local.get 4
              i32.const 4
              i32.lt_u
              br_if 0 (;@4;)
              local.get 6
              local.get 5
              i32.const 1008
              i32.and
              i32.add
              local.set 0
              i32.const 0
              local.set 9
              local.get 6
              local.set 1
              loop ;; label = @5
                local.get 1
                i32.const 12
                i32.add
                i32.load
                local.tee 8
                i32.const -1
                i32.xor
                i32.const 7
                i32.shr_u
                local.get 8
                i32.const 6
                i32.shr_u
                i32.or
                i32.const 16843009
                i32.and
                local.get 1
                i32.const 8
                i32.add
                i32.load
                local.tee 8
                i32.const -1
                i32.xor
                i32.const 7
                i32.shr_u
                local.get 8
                i32.const 6
                i32.shr_u
                i32.or
                i32.const 16843009
                i32.and
                local.get 1
                i32.const 4
                i32.add
                i32.load
                local.tee 8
                i32.const -1
                i32.xor
                i32.const 7
                i32.shr_u
                local.get 8
                i32.const 6
                i32.shr_u
                i32.or
                i32.const 16843009
                i32.and
                local.get 1
                i32.load
                local.tee 8
                i32.const -1
                i32.xor
                i32.const 7
                i32.shr_u
                local.get 8
                i32.const 6
                i32.shr_u
                i32.or
                i32.const 16843009
                i32.and
                local.get 9
                i32.add
                i32.add
                i32.add
                i32.add
                local.set 9
                local.get 1
                i32.const 16
                i32.add
                local.tee 1
                local.get 0
                i32.ne
                br_if 0 (;@5;)
              end
            end
            local.get 3
            local.get 4
            i32.sub
            local.set 3
            local.get 6
            local.get 5
            i32.add
            local.set 8
            local.get 9
            i32.const 8
            i32.shr_u
            i32.const 16711935
            i32.and
            local.get 9
            i32.const 16711935
            i32.and
            i32.add
            i32.const 65537
            i32.mul
            i32.const 16
            i32.shr_u
            local.get 2
            i32.add
            local.set 2
            local.get 7
            i32.eqz
            br_if 0 (;@3;)
          end
          local.get 6
          local.get 4
          i32.const 252
          i32.and
          i32.const 2
          i32.shl
          i32.add
          local.tee 9
          i32.load
          local.tee 1
          i32.const -1
          i32.xor
          i32.const 7
          i32.shr_u
          local.get 1
          i32.const 6
          i32.shr_u
          i32.or
          i32.const 16843009
          i32.and
          local.set 1
          block ;; label = @3
            local.get 7
            i32.const 1
            i32.eq
            br_if 0 (;@3;)
            local.get 9
            i32.load offset=4
            local.tee 8
            i32.const -1
            i32.xor
            i32.const 7
            i32.shr_u
            local.get 8
            i32.const 6
            i32.shr_u
            i32.or
            i32.const 16843009
            i32.and
            local.get 1
            i32.add
            local.set 1
            local.get 7
            i32.const 2
            i32.eq
            br_if 0 (;@3;)
            local.get 9
            i32.load offset=8
            local.tee 9
            i32.const -1
            i32.xor
            i32.const 7
            i32.shr_u
            local.get 9
            i32.const 6
            i32.shr_u
            i32.or
            i32.const 16843009
            i32.and
            local.get 1
            i32.add
            local.set 1
          end
          local.get 1
          i32.const 8
          i32.shr_u
          i32.const 459007
          i32.and
          local.get 1
          i32.const 16711935
          i32.and
          i32.add
          i32.const 65537
          i32.mul
          i32.const 16
          i32.shr_u
          local.get 2
          i32.add
          return
        end
        block ;; label = @2
          local.get 1
          br_if 0 (;@2;)
          i32.const 0
          return
        end
        local.get 1
        i32.const 3
        i32.and
        local.set 8
        block ;; label = @2
          block ;; label = @3
            local.get 1
            i32.const 4
            i32.ge_u
            br_if 0 (;@3;)
            i32.const 0
            local.set 2
            i32.const 0
            local.set 9
            br 1 (;@2;)
          end
          local.get 1
          i32.const -4
          i32.and
          local.set 3
          i32.const 0
          local.set 2
          i32.const 0
          local.set 9
          loop ;; label = @3
            local.get 2
            local.get 0
            local.get 9
            i32.add
            local.tee 1
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 1
            i32.const 1
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 1
            i32.const 2
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 1
            i32.const 3
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.set 2
            local.get 3
            local.get 9
            i32.const 4
            i32.add
            local.tee 9
            i32.ne
            br_if 0 (;@3;)
          end
        end
        local.get 8
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        local.get 9
        i32.add
        local.set 1
        loop ;; label = @2
          local.get 2
          local.get 1
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.set 2
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 8
          i32.const -1
          i32.add
          local.tee 8
          br_if 0 (;@2;)
        end
      end
      local.get 2
    )
    (func $core::fmt::Formatter::pad_integral::write_prefix (;26;) (type 12) (param i32 i32 i32 i32 i32) (result i32)
      (local i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 2
            i32.const 1114112
            i32.eq
            br_if 0 (;@3;)
            i32.const 1
            local.set 5
            local.get 0
            local.get 2
            local.get 1
            i32.load offset=16
            call_indirect (type 1)
            br_if 1 (;@2;)
          end
          local.get 3
          br_if 1 (;@1;)
          i32.const 0
          local.set 5
        end
        local.get 5
        return
      end
      local.get 0
      local.get 3
      local.get 4
      local.get 1
      i32.load offset=12
      call_indirect (type 0)
    )
    (func $core::fmt::num::imp::fmt_u64 (;27;) (type 13) (param i64 i32 i32) (result i32)
      (local i32 i32 i64 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      i32.const 39
      local.set 4
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i64.const 10000
          i64.ge_u
          br_if 0 (;@2;)
          local.get 0
          local.set 5
          br 1 (;@1;)
        end
        i32.const 39
        local.set 4
        loop ;; label = @2
          local.get 3
          i32.const 9
          i32.add
          local.get 4
          i32.add
          local.tee 6
          i32.const -4
          i32.add
          local.get 0
          local.get 0
          i64.const 10000
          i64.div_u
          local.tee 5
          i64.const 10000
          i64.mul
          i64.sub
          i32.wrap_i64
          local.tee 7
          i32.const 65535
          i32.and
          i32.const 100
          i32.div_u
          local.tee 8
          i32.const 1
          i32.shl
          i32.const 1048688
          i32.add
          i32.load16_u align=1
          i32.store16 align=1
          local.get 6
          i32.const -2
          i32.add
          local.get 7
          local.get 8
          i32.const 100
          i32.mul
          i32.sub
          i32.const 65535
          i32.and
          i32.const 1
          i32.shl
          i32.const 1048688
          i32.add
          i32.load16_u align=1
          i32.store16 align=1
          local.get 4
          i32.const -4
          i32.add
          local.set 4
          local.get 0
          i64.const 99999999
          i64.gt_u
          local.set 6
          local.get 5
          local.set 0
          local.get 6
          br_if 0 (;@2;)
        end
      end
      block ;; label = @1
        local.get 5
        i32.wrap_i64
        local.tee 6
        i32.const 99
        i32.le_u
        br_if 0 (;@1;)
        local.get 3
        i32.const 9
        i32.add
        local.get 4
        i32.const -2
        i32.add
        local.tee 4
        i32.add
        local.get 5
        i32.wrap_i64
        local.tee 6
        local.get 6
        i32.const 65535
        i32.and
        i32.const 100
        i32.div_u
        local.tee 6
        i32.const 100
        i32.mul
        i32.sub
        i32.const 65535
        i32.and
        i32.const 1
        i32.shl
        i32.const 1048688
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
      end
      block ;; label = @1
        block ;; label = @2
          local.get 6
          i32.const 10
          i32.lt_u
          br_if 0 (;@2;)
          local.get 3
          i32.const 9
          i32.add
          local.get 4
          i32.const -2
          i32.add
          local.tee 4
          i32.add
          local.get 6
          i32.const 1
          i32.shl
          i32.const 1048688
          i32.add
          i32.load16_u align=1
          i32.store16 align=1
          br 1 (;@1;)
        end
        local.get 3
        i32.const 9
        i32.add
        local.get 4
        i32.const -1
        i32.add
        local.tee 4
        i32.add
        local.get 6
        i32.const 48
        i32.add
        i32.store8
      end
      local.get 2
      local.get 1
      i32.const 1048604
      i32.const 0
      local.get 3
      i32.const 9
      i32.add
      local.get 4
      i32.add
      i32.const 39
      local.get 4
      i32.sub
      call $core::fmt::Formatter::pad_integral
      local.set 4
      local.get 3
      i32.const 48
      i32.add
      global.set $__stack_pointer
      local.get 4
    )
    (table (;0;) 4 4 funcref)
    (memory (;0;) 17)
    (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
    (export "memory" (memory 0))
    (export "miden:base/note-script@1.0.0#note-script" (func $miden:base/note-script@1.0.0#note-script))
    (export "cabi_realloc" (func $cabi_realloc))
    (elem (;0;) (i32.const 1) func $core::fmt::num::imp::<impl core::fmt::Display for u32>::fmt $core::ptr::drop_in_place<core::fmt::Error> $<T as core::any::Any>::type_id)
    (data $.rodata (;0;) (i32.const 1048576) "src/lib.rs\00\00\00\00\10\00\0a\00\00\00\1f\00\00\00,\00\00\00\02\00\00\00\00\00\00\00\01\00\00\00\03\00\00\00index out of bounds: the len is  but the index is \00\00,\00\10\00 \00\00\00L\00\10\00\12\00\00\0000010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899")
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