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
    (type (;7;) (func (param i32 i32 i32)))
    (type (;8;) (func (param i32 i32 i32 i32) (result i32)))
    (type (;9;) (func (param i32 i32 i32 i32)))
    (type (;10;) (func (param i32 i32)))
    (type (;11;) (func (param i32 i32 i32 i32 i32) (result i32)))
    (type (;12;) (func (param i32 i32 i32 i32 i32 i32 i32)))
    (type (;13;) (func (param i32) (result i32)))
    (type (;14;) (func (param i32 i32 i32 i32 i32 i32) (result i32)))
    (type (;15;) (func (param i64 i32 i32) (result i32)))
    (import "miden:base/note@1.0.0" "get-inputs" (func $basic_wallet_p2id_note::bindings::miden::base::note::get_inputs::wit_import (;0;) (type 2)))
    (import "miden:base/core-types@1.0.0" "account-id-from-felt" (func $basic_wallet_p2id_note::bindings::miden::base::core_types::account_id_from_felt::wit_import (;1;) (type 3)))
    (import "miden:base/account@1.0.0" "get-id" (func $basic_wallet_p2id_note::bindings::miden::base::account::get_id::wit_import (;2;) (type 4)))
    (import "miden:base/note@1.0.0" "get-assets" (func $basic_wallet_p2id_note::bindings::miden::base::note::get_assets::wit_import (;3;) (type 2)))
    (import "miden:basic-wallet/basic-wallet@1.0.0" "receive-asset" (func $basic_wallet_p2id_note::bindings::miden::basic_wallet::basic_wallet::receive_asset::wit_import (;4;) (type 5)))
    (func $__wasm_call_ctors (;5;) (type 6))
    (func $<&T as core::fmt::Debug>::fmt (;6;) (type 1) (param i32 i32) (result i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 0
      i32.load
      local.set 0
      local.get 2
      i32.const 8
      i32.add
      local.get 1
      i32.const 1048621
      i32.const 9
      call $core::fmt::Formatter::debug_struct
      local.get 2
      i32.const 8
      i32.add
      i32.const 1048616
      i32.const 5
      local.get 0
      i32.const 1048632
      call $core::fmt::builders::DebugStruct::field
      call $core::fmt::builders::DebugStruct::finish
      local.set 0
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $core::fmt::num::<impl core::fmt::Debug for u64>::fmt (;7;) (type 1) (param i32 i32) (result i32)
      (local i32)
      block ;; label = @1
        local.get 1
        i32.load offset=28
        local.tee 2
        i32.const 16
        i32.and
        br_if 0 (;@1;)
        block ;; label = @2
          local.get 2
          i32.const 32
          i32.and
          br_if 0 (;@2;)
          local.get 0
          local.get 1
          call $core::fmt::num::imp::<impl core::fmt::Display for u64>::fmt
          return
        end
        local.get 0
        local.get 1
        call $core::fmt::num::<impl core::fmt::UpperHex for i64>::fmt
        return
      end
      local.get 0
      local.get 1
      call $core::fmt::num::<impl core::fmt::LowerHex for i64>::fmt
    )
    (func $core::panicking::assert_failed (;8;) (type 7) (param i32 i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      local.get 1
      i32.store offset=12
      local.get 3
      local.get 0
      i32.store offset=8
      i32.const 0
      local.get 3
      i32.const 8
      i32.add
      i32.const 1048576
      local.get 3
      i32.const 12
      i32.add
      i32.const 1048576
      local.get 2
      i32.const 1048696
      call $core::panicking::assert_failed_inner
      unreachable
    )
    (func $rust_begin_unwind (;9;) (type 2) (param i32)
      loop ;; label = @1
        br 0 (;@1;)
      end
    )
    (func $<basic_wallet_p2id_note::bindings::miden::base::core_types::Felt as core::fmt::Debug>::fmt (;10;) (type 1) (param i32 i32) (result i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 2
      i32.const 8
      i32.add
      local.get 1
      i32.const 1048596
      i32.const 4
      call $core::fmt::Formatter::debug_struct
      local.get 2
      i32.const 8
      i32.add
      i32.const 1048616
      i32.const 5
      local.get 0
      i32.const 1048600
      call $core::fmt::builders::DebugStruct::field
      call $core::fmt::builders::DebugStruct::finish
      local.set 1
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 1
    )
    (func $basic_wallet_p2id_note::bindings::__link_custom_section_describing_imports (;11;) (type 6))
    (func $__rust_alloc (;12;) (type 1) (param i32 i32) (result i32)
      i32.const 1049280
      local.get 1
      local.get 0
      call $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::alloc
    )
    (func $__rust_realloc (;13;) (type 8) (param i32 i32 i32 i32) (result i32)
      (local i32)
      block ;; label = @1
        i32.const 1049280
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
        i32.const 1049280
        local.get 0
        local.get 2
        local.get 1
        call $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::dealloc
      end
      local.get 4
    )
    (func $miden:base/note-script@1.0.0#note-script (;14;) (type 6)
      (local i32 i32 i32 i64 i64 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 0
      global.set $__stack_pointer
      call $wit_bindgen_rt::run_ctors_once
      local.get 0
      i64.const 0
      i64.store offset=24
      local.get 0
      i32.const 24
      i32.add
      call $basic_wallet_p2id_note::bindings::miden::base::note::get_inputs::wit_import
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.load offset=28
          local.tee 1
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          local.get 0
          i32.load offset=24
          local.tee 2
          i64.load
          call $basic_wallet_p2id_note::bindings::miden::base::core_types::account_id_from_felt::wit_import
          local.tee 3
          i64.store offset=8
          local.get 0
          call $basic_wallet_p2id_note::bindings::miden::base::account::get_id::wit_import
          local.tee 4
          i64.store offset=16
          local.get 4
          local.get 3
          i64.ne
          br_if 1 (;@1;)
          local.get 0
          i64.const 0
          i64.store offset=24
          local.get 0
          i32.const 24
          i32.add
          call $basic_wallet_p2id_note::bindings::miden::base::note::get_assets::wit_import
          block ;; label = @3
            local.get 0
            i32.load offset=28
            local.tee 5
            i32.eqz
            br_if 0 (;@3;)
            local.get 0
            i32.load offset=24
            local.tee 6
            local.get 5
            i32.const 5
            i32.shl
            i32.add
            local.set 7
            local.get 6
            local.set 8
            loop ;; label = @4
              local.get 8
              i64.load
              local.get 8
              i64.load offset=8
              local.get 8
              i64.load offset=16
              local.get 8
              i64.load offset=24
              call $basic_wallet_p2id_note::bindings::miden::basic_wallet::basic_wallet::receive_asset::wit_import
              local.get 8
              i32.const 32
              i32.add
              local.tee 8
              local.get 7
              i32.ne
              br_if 0 (;@4;)
            end
            i32.const 1049280
            local.get 6
            i32.const 8
            local.get 5
            i32.const 5
            i32.shl
            call $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::dealloc
          end
          i32.const 1049280
          local.get 2
          i32.const 8
          local.get 1
          i32.const 3
          i32.shl
          call $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::dealloc
          local.get 0
          i32.const 48
          i32.add
          global.set $__stack_pointer
          return
        end
        i32.const 0
        i32.const 0
        i32.const 1048680
        call $core::panicking::panic_bounds_check
        unreachable
      end
      local.get 0
      i32.const 0
      i32.store offset=24
      local.get 0
      i32.const 16
      i32.add
      local.get 0
      i32.const 8
      i32.add
      local.get 0
      i32.const 24
      i32.add
      call $core::panicking::assert_failed
      unreachable
    )
    (func $wit_bindgen_rt::cabi_realloc (;15;) (type 8) (param i32 i32 i32 i32) (result i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 1
            br_if 0 (;@3;)
            local.get 3
            i32.eqz
            br_if 2 (;@1;)
            i32.const 0
            i32.load8_u offset=1049284
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
      end
      local.get 2
    )
    (func $wit_bindgen_rt::run_ctors_once (;16;) (type 6)
      block ;; label = @1
        i32.const 0
        i32.load8_u offset=1049285
        br_if 0 (;@1;)
        call $__wasm_call_ctors
        i32.const 0
        i32.const 1
        i32.store8 offset=1049285
      end
    )
    (func $wee_alloc::alloc_first_fit (;17;) (type 0) (param i32 i32 i32) (result i32)
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
    (func $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::alloc (;18;) (type 0) (param i32 i32 i32) (result i32)
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
    (func $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::dealloc (;19;) (type 9) (param i32 i32 i32 i32)
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
                local.get 1
                i32.const -4
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
              i32.const 2
              i32.and
              br_if 1 (;@3;)
              local.get 5
              i32.const -4
              i32.and
              local.tee 5
              i32.eqz
              br_if 1 (;@3;)
              local.get 5
              i32.load8_u
              i32.const 1
              i32.and
              br_if 1 (;@3;)
              local.get 1
              local.get 5
              i32.load offset=8
              i32.const -4
              i32.and
              i32.store
              local.get 5
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
    (func $core::panicking::panic_fmt (;20;) (type 10) (param i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 2
      i32.const 16
      i32.add
      local.get 0
      i32.const 16
      i32.add
      i64.load align=4
      i64.store
      local.get 2
      i32.const 8
      i32.add
      local.get 0
      i32.const 8
      i32.add
      i64.load align=4
      i64.store
      local.get 2
      i32.const 1
      i32.store16 offset=28
      local.get 2
      local.get 1
      i32.store offset=24
      local.get 2
      local.get 0
      i64.load align=4
      i64.store
      local.get 2
      call $rust_begin_unwind
      unreachable
    )
    (func $core::slice::index::slice_start_index_len_fail (;21;) (type 7) (param i32 i32 i32)
      (local i32 i64)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      local.get 0
      i32.store
      local.get 3
      local.get 1
      i32.store offset=4
      local.get 3
      i32.const 2
      i32.store offset=12
      local.get 3
      i32.const 1049264
      i32.store offset=8
      local.get 3
      i64.const 2
      i64.store offset=20 align=4
      local.get 3
      i32.const 6
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.tee 4
      local.get 3
      i32.const 4
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=40
      local.get 3
      local.get 4
      local.get 3
      i64.extend_i32_u
      i64.or
      i64.store offset=32
      local.get 3
      local.get 3
      i32.const 32
      i32.add
      i32.store offset=16
      local.get 3
      i32.const 8
      i32.add
      local.get 2
      call $core::panicking::panic_fmt
      unreachable
    )
    (func $core::panicking::panic_bounds_check (;22;) (type 7) (param i32 i32 i32)
      (local i32 i64)
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
      i32.const 2
      i32.store offset=12
      local.get 3
      i32.const 1048768
      i32.store offset=8
      local.get 3
      i64.const 2
      i64.store offset=20 align=4
      local.get 3
      i32.const 6
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.tee 4
      local.get 3
      i64.extend_i32_u
      i64.or
      i64.store offset=40
      local.get 3
      local.get 4
      local.get 3
      i32.const 4
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=32
      local.get 3
      local.get 3
      i32.const 32
      i32.add
      i32.store offset=16
      local.get 3
      i32.const 8
      i32.add
      local.get 2
      call $core::panicking::panic_fmt
      unreachable
    )
    (func $core::fmt::Formatter::pad (;23;) (type 0) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32)
      local.get 0
      i32.load offset=8
      local.set 3
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.load
          local.tee 4
          br_if 0 (;@2;)
          local.get 3
          i32.const 1
          i32.and
          i32.eqz
          br_if 1 (;@1;)
        end
        block ;; label = @2
          local.get 3
          i32.const 1
          i32.and
          i32.eqz
          br_if 0 (;@2;)
          local.get 1
          local.get 2
          i32.add
          local.set 5
          block ;; label = @3
            block ;; label = @4
              local.get 0
              i32.load offset=12
              local.tee 6
              br_if 0 (;@4;)
              i32.const 0
              local.set 7
              local.get 1
              local.set 8
              br 1 (;@3;)
            end
            i32.const 0
            local.set 7
            local.get 1
            local.set 8
            loop ;; label = @4
              local.get 8
              local.tee 3
              local.get 5
              i32.eq
              br_if 2 (;@2;)
              block ;; label = @5
                block ;; label = @6
                  local.get 3
                  i32.load8_s
                  local.tee 8
                  i32.const -1
                  i32.le_s
                  br_if 0 (;@6;)
                  local.get 3
                  i32.const 1
                  i32.add
                  local.set 8
                  br 1 (;@5;)
                end
                block ;; label = @6
                  local.get 8
                  i32.const -32
                  i32.ge_u
                  br_if 0 (;@6;)
                  local.get 3
                  i32.const 2
                  i32.add
                  local.set 8
                  br 1 (;@5;)
                end
                block ;; label = @6
                  local.get 8
                  i32.const -16
                  i32.ge_u
                  br_if 0 (;@6;)
                  local.get 3
                  i32.const 3
                  i32.add
                  local.set 8
                  br 1 (;@5;)
                end
                local.get 3
                i32.const 4
                i32.add
                local.set 8
              end
              local.get 8
              local.get 3
              i32.sub
              local.get 7
              i32.add
              local.set 7
              local.get 6
              i32.const -1
              i32.add
              local.tee 6
              br_if 0 (;@4;)
            end
          end
          local.get 8
          local.get 5
          i32.eq
          br_if 0 (;@2;)
          block ;; label = @3
            local.get 8
            i32.load8_s
            local.tee 3
            i32.const -1
            i32.gt_s
            br_if 0 (;@3;)
            local.get 3
            i32.const -32
            i32.lt_u
            drop
          end
          block ;; label = @3
            block ;; label = @4
              local.get 7
              i32.eqz
              br_if 0 (;@4;)
              block ;; label = @5
                local.get 7
                local.get 2
                i32.ge_u
                br_if 0 (;@5;)
                local.get 1
                local.get 7
                i32.add
                i32.load8_s
                i32.const -65
                i32.gt_s
                br_if 1 (;@4;)
                i32.const 0
                local.set 3
                br 2 (;@3;)
              end
              local.get 7
              local.get 2
              i32.eq
              br_if 0 (;@4;)
              i32.const 0
              local.set 3
              br 1 (;@3;)
            end
            local.get 1
            local.set 3
          end
          local.get 7
          local.get 2
          local.get 3
          select
          local.set 2
          local.get 3
          local.get 1
          local.get 3
          select
          local.set 1
        end
        block ;; label = @2
          local.get 4
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=20
          local.get 1
          local.get 2
          local.get 0
          i32.load offset=24
          i32.load offset=12
          call_indirect (type 0)
          return
        end
        local.get 0
        i32.load offset=4
        local.set 4
        block ;; label = @2
          block ;; label = @3
            local.get 2
            i32.const 16
            i32.lt_u
            br_if 0 (;@3;)
            local.get 1
            local.get 2
            call $core::str::count::do_count_chars
            local.set 3
            br 1 (;@2;)
          end
          block ;; label = @3
            local.get 2
            br_if 0 (;@3;)
            i32.const 0
            local.set 3
            br 1 (;@2;)
          end
          local.get 2
          i32.const 3
          i32.and
          local.set 6
          block ;; label = @3
            block ;; label = @4
              local.get 2
              i32.const 4
              i32.ge_u
              br_if 0 (;@4;)
              i32.const 0
              local.set 3
              i32.const 0
              local.set 7
              br 1 (;@3;)
            end
            local.get 2
            i32.const 12
            i32.and
            local.set 5
            i32.const 0
            local.set 3
            i32.const 0
            local.set 7
            loop ;; label = @4
              local.get 3
              local.get 1
              local.get 7
              i32.add
              local.tee 8
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.get 8
              i32.const 1
              i32.add
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.get 8
              i32.const 2
              i32.add
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.get 8
              i32.const 3
              i32.add
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.set 3
              local.get 5
              local.get 7
              i32.const 4
              i32.add
              local.tee 7
              i32.ne
              br_if 0 (;@4;)
            end
          end
          local.get 6
          i32.eqz
          br_if 0 (;@2;)
          local.get 1
          local.get 7
          i32.add
          local.set 8
          loop ;; label = @3
            local.get 3
            local.get 8
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.set 3
            local.get 8
            i32.const 1
            i32.add
            local.set 8
            local.get 6
            i32.const -1
            i32.add
            local.tee 6
            br_if 0 (;@3;)
          end
        end
        block ;; label = @2
          block ;; label = @3
            local.get 4
            local.get 3
            i32.le_u
            br_if 0 (;@3;)
            local.get 4
            local.get 3
            i32.sub
            local.set 5
            i32.const 0
            local.set 3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  local.get 0
                  i32.load8_u offset=32
                  br_table 2 (;@4;) 0 (;@6;) 1 (;@5;) 2 (;@4;) 2 (;@4;)
                end
                local.get 5
                local.set 3
                i32.const 0
                local.set 5
                br 1 (;@4;)
              end
              local.get 5
              i32.const 1
              i32.shr_u
              local.set 3
              local.get 5
              i32.const 1
              i32.add
              i32.const 1
              i32.shr_u
              local.set 5
            end
            local.get 3
            i32.const 1
            i32.add
            local.set 3
            local.get 0
            i32.load offset=16
            local.set 6
            local.get 0
            i32.load offset=24
            local.set 8
            local.get 0
            i32.load offset=20
            local.set 7
            loop ;; label = @4
              local.get 3
              i32.const -1
              i32.add
              local.tee 3
              i32.eqz
              br_if 2 (;@2;)
              local.get 7
              local.get 6
              local.get 8
              i32.load offset=16
              call_indirect (type 1)
              i32.eqz
              br_if 0 (;@4;)
            end
            i32.const 1
            return
          end
          local.get 0
          i32.load offset=20
          local.get 1
          local.get 2
          local.get 0
          i32.load offset=24
          i32.load offset=12
          call_indirect (type 0)
          return
        end
        block ;; label = @2
          local.get 7
          local.get 1
          local.get 2
          local.get 8
          i32.load offset=12
          call_indirect (type 0)
          i32.eqz
          br_if 0 (;@2;)
          i32.const 1
          return
        end
        i32.const 0
        local.set 3
        loop ;; label = @2
          block ;; label = @3
            local.get 5
            local.get 3
            i32.ne
            br_if 0 (;@3;)
            local.get 5
            local.get 5
            i32.lt_u
            return
          end
          local.get 3
          i32.const 1
          i32.add
          local.set 3
          local.get 7
          local.get 6
          local.get 8
          i32.load offset=16
          call_indirect (type 1)
          i32.eqz
          br_if 0 (;@2;)
        end
        local.get 3
        i32.const -1
        i32.add
        local.get 5
        i32.lt_u
        return
      end
      local.get 0
      i32.load offset=20
      local.get 1
      local.get 2
      local.get 0
      i32.load offset=24
      i32.load offset=12
      call_indirect (type 0)
    )
    (func $core::fmt::num::imp::<impl core::fmt::Display for u32>::fmt (;24;) (type 1) (param i32 i32) (result i32)
      local.get 0
      i64.load32_u
      i32.const 1
      local.get 1
      call $core::fmt::num::imp::fmt_u64
    )
    (func $core::fmt::write (;25;) (type 0) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      i32.const 3
      i32.store8 offset=44
      local.get 3
      i32.const 32
      i32.store offset=28
      i32.const 0
      local.set 4
      local.get 3
      i32.const 0
      i32.store offset=40
      local.get 3
      local.get 1
      i32.store offset=36
      local.get 3
      local.get 0
      i32.store offset=32
      local.get 3
      i32.const 0
      i32.store offset=20
      local.get 3
      i32.const 0
      i32.store offset=12
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                local.get 2
                i32.load offset=16
                local.tee 5
                br_if 0 (;@5;)
                local.get 2
                i32.load offset=12
                local.tee 0
                i32.eqz
                br_if 1 (;@4;)
                local.get 2
                i32.load offset=8
                local.set 1
                local.get 0
                i32.const 3
                i32.shl
                local.set 6
                local.get 0
                i32.const -1
                i32.add
                i32.const 536870911
                i32.and
                i32.const 1
                i32.add
                local.set 4
                local.get 2
                i32.load
                local.set 0
                loop ;; label = @6
                  block ;; label = @7
                    local.get 0
                    i32.const 4
                    i32.add
                    i32.load
                    local.tee 7
                    i32.eqz
                    br_if 0 (;@7;)
                    local.get 3
                    i32.load offset=32
                    local.get 0
                    i32.load
                    local.get 7
                    local.get 3
                    i32.load offset=36
                    i32.load offset=12
                    call_indirect (type 0)
                    br_if 4 (;@3;)
                  end
                  local.get 1
                  i32.load
                  local.get 3
                  i32.const 12
                  i32.add
                  local.get 1
                  i32.load offset=4
                  call_indirect (type 1)
                  br_if 3 (;@3;)
                  local.get 1
                  i32.const 8
                  i32.add
                  local.set 1
                  local.get 0
                  i32.const 8
                  i32.add
                  local.set 0
                  local.get 6
                  i32.const -8
                  i32.add
                  local.tee 6
                  br_if 0 (;@6;)
                  br 2 (;@4;)
                end
              end
              local.get 2
              i32.load offset=20
              local.tee 1
              i32.eqz
              br_if 0 (;@4;)
              local.get 1
              i32.const 5
              i32.shl
              local.set 8
              local.get 1
              i32.const -1
              i32.add
              i32.const 134217727
              i32.and
              i32.const 1
              i32.add
              local.set 4
              local.get 2
              i32.load offset=8
              local.set 9
              local.get 2
              i32.load
              local.set 0
              i32.const 0
              local.set 6
              loop ;; label = @5
                block ;; label = @6
                  local.get 0
                  i32.const 4
                  i32.add
                  i32.load
                  local.tee 1
                  i32.eqz
                  br_if 0 (;@6;)
                  local.get 3
                  i32.load offset=32
                  local.get 0
                  i32.load
                  local.get 1
                  local.get 3
                  i32.load offset=36
                  i32.load offset=12
                  call_indirect (type 0)
                  br_if 3 (;@3;)
                end
                local.get 3
                local.get 5
                local.get 6
                i32.add
                local.tee 1
                i32.const 16
                i32.add
                i32.load
                i32.store offset=28
                local.get 3
                local.get 1
                i32.const 28
                i32.add
                i32.load8_u
                i32.store8 offset=44
                local.get 3
                local.get 1
                i32.const 24
                i32.add
                i32.load
                i32.store offset=40
                local.get 1
                i32.const 12
                i32.add
                i32.load
                local.set 7
                i32.const 0
                local.set 10
                i32.const 0
                local.set 11
                block ;; label = @6
                  block ;; label = @7
                    block ;; label = @8
                      local.get 1
                      i32.const 8
                      i32.add
                      i32.load
                      br_table 1 (;@7;) 0 (;@8;) 2 (;@6;) 1 (;@7;)
                    end
                    local.get 7
                    i32.const 3
                    i32.shl
                    local.set 12
                    i32.const 0
                    local.set 11
                    local.get 9
                    local.get 12
                    i32.add
                    local.tee 12
                    i32.load offset=4
                    br_if 1 (;@6;)
                    local.get 12
                    i32.load
                    local.set 7
                  end
                  i32.const 1
                  local.set 11
                end
                local.get 3
                local.get 7
                i32.store offset=16
                local.get 3
                local.get 11
                i32.store offset=12
                local.get 1
                i32.const 4
                i32.add
                i32.load
                local.set 7
                block ;; label = @6
                  block ;; label = @7
                    block ;; label = @8
                      local.get 1
                      i32.load
                      br_table 1 (;@7;) 0 (;@8;) 2 (;@6;) 1 (;@7;)
                    end
                    local.get 7
                    i32.const 3
                    i32.shl
                    local.set 11
                    local.get 9
                    local.get 11
                    i32.add
                    local.tee 11
                    i32.load offset=4
                    br_if 1 (;@6;)
                    local.get 11
                    i32.load
                    local.set 7
                  end
                  i32.const 1
                  local.set 10
                end
                local.get 3
                local.get 7
                i32.store offset=24
                local.get 3
                local.get 10
                i32.store offset=20
                local.get 9
                local.get 1
                i32.const 20
                i32.add
                i32.load
                i32.const 3
                i32.shl
                i32.add
                local.tee 1
                i32.load
                local.get 3
                i32.const 12
                i32.add
                local.get 1
                i32.load offset=4
                call_indirect (type 1)
                br_if 2 (;@3;)
                local.get 0
                i32.const 8
                i32.add
                local.set 0
                local.get 8
                local.get 6
                i32.const 32
                i32.add
                local.tee 6
                i32.ne
                br_if 0 (;@5;)
              end
            end
            local.get 4
            local.get 2
            i32.load offset=4
            i32.ge_u
            br_if 1 (;@2;)
            local.get 3
            i32.load offset=32
            local.get 2
            i32.load
            local.get 4
            i32.const 3
            i32.shl
            i32.add
            local.tee 1
            i32.load
            local.get 1
            i32.load offset=4
            local.get 3
            i32.load offset=36
            i32.load offset=12
            call_indirect (type 0)
            i32.eqz
            br_if 1 (;@2;)
          end
          i32.const 1
          local.set 1
          br 1 (;@1;)
        end
        i32.const 0
        local.set 1
      end
      local.get 3
      i32.const 48
      i32.add
      global.set $__stack_pointer
      local.get 1
    )
    (func $core::fmt::builders::DebugStruct::field (;26;) (type 11) (param i32 i32 i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i64)
      global.get $__stack_pointer
      i32.const 64
      i32.sub
      local.tee 5
      global.set $__stack_pointer
      i32.const 1
      local.set 6
      block ;; label = @1
        local.get 0
        i32.load8_u offset=4
        br_if 0 (;@1;)
        local.get 0
        i32.load8_u offset=5
        local.set 7
        block ;; label = @2
          local.get 0
          i32.load
          local.tee 8
          i32.load offset=28
          local.tee 9
          i32.const 4
          i32.and
          br_if 0 (;@2;)
          i32.const 1
          local.set 6
          local.get 8
          i32.load offset=20
          i32.const 1048963
          i32.const 1048960
          local.get 7
          i32.const 1
          i32.and
          local.tee 7
          select
          i32.const 2
          i32.const 3
          local.get 7
          select
          local.get 8
          i32.load offset=24
          i32.load offset=12
          call_indirect (type 0)
          br_if 1 (;@1;)
          local.get 8
          i32.load offset=20
          local.get 1
          local.get 2
          local.get 8
          i32.load offset=24
          i32.load offset=12
          call_indirect (type 0)
          br_if 1 (;@1;)
          local.get 8
          i32.load offset=20
          i32.const 1048928
          i32.const 2
          local.get 8
          i32.load offset=24
          i32.load offset=12
          call_indirect (type 0)
          br_if 1 (;@1;)
          local.get 3
          local.get 8
          local.get 4
          i32.load offset=12
          call_indirect (type 1)
          local.set 6
          br 1 (;@1;)
        end
        i32.const 1
        local.set 6
        block ;; label = @2
          local.get 7
          i32.const 1
          i32.and
          br_if 0 (;@2;)
          local.get 8
          i32.load offset=20
          i32.const 1048965
          i32.const 3
          local.get 8
          i32.load offset=24
          i32.load offset=12
          call_indirect (type 0)
          br_if 1 (;@1;)
          local.get 8
          i32.load offset=28
          local.set 9
        end
        i32.const 1
        local.set 6
        local.get 5
        i32.const 1
        i32.store8 offset=27
        local.get 5
        local.get 8
        i64.load offset=20 align=4
        i64.store offset=12 align=4
        local.get 5
        i32.const 1048932
        i32.store offset=52
        local.get 5
        local.get 5
        i32.const 27
        i32.add
        i32.store offset=20
        local.get 5
        local.get 8
        i64.load offset=8 align=4
        i64.store offset=36 align=4
        local.get 8
        i64.load align=4
        local.set 10
        local.get 5
        local.get 9
        i32.store offset=56
        local.get 5
        local.get 8
        i32.load offset=16
        i32.store offset=44
        local.get 5
        local.get 8
        i32.load8_u offset=32
        i32.store8 offset=60
        local.get 5
        local.get 10
        i64.store offset=28 align=4
        local.get 5
        local.get 5
        i32.const 12
        i32.add
        i32.store offset=48
        local.get 5
        i32.const 12
        i32.add
        local.get 1
        local.get 2
        call $<core::fmt::builders::PadAdapter as core::fmt::Write>::write_str
        br_if 0 (;@1;)
        local.get 5
        i32.const 12
        i32.add
        i32.const 1048928
        i32.const 2
        call $<core::fmt::builders::PadAdapter as core::fmt::Write>::write_str
        br_if 0 (;@1;)
        local.get 3
        local.get 5
        i32.const 28
        i32.add
        local.get 4
        i32.load offset=12
        call_indirect (type 1)
        br_if 0 (;@1;)
        local.get 5
        i32.load offset=48
        i32.const 1048968
        i32.const 2
        local.get 5
        i32.load offset=52
        i32.load offset=12
        call_indirect (type 0)
        local.set 6
      end
      local.get 0
      i32.const 1
      i32.store8 offset=5
      local.get 0
      local.get 6
      i32.store8 offset=4
      local.get 5
      i32.const 64
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $<&T as core::fmt::Display>::fmt (;27;) (type 1) (param i32 i32) (result i32)
      local.get 1
      local.get 0
      i32.load
      local.get 0
      i32.load offset=4
      call $core::fmt::Formatter::pad
    )
    (func $core::panicking::assert_failed_inner (;28;) (type 12) (param i32 i32 i32 i32 i32 i32 i32)
      (local i32 i64)
      global.get $__stack_pointer
      i32.const 112
      i32.sub
      local.tee 7
      global.set $__stack_pointer
      local.get 7
      local.get 2
      i32.store offset=12
      local.get 7
      local.get 1
      i32.store offset=8
      local.get 7
      local.get 4
      i32.store offset=20
      local.get 7
      local.get 3
      i32.store offset=16
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 0
              i32.const 255
              i32.and
              br_table 0 (;@4;) 1 (;@3;) 2 (;@2;) 0 (;@4;)
            end
            local.get 7
            i32.const 1048784
            i32.store offset=24
            i32.const 2
            local.set 2
            br 2 (;@1;)
          end
          local.get 7
          i32.const 1048786
          i32.store offset=24
          i32.const 2
          local.set 2
          br 1 (;@1;)
        end
        local.get 7
        i32.const 1048788
        i32.store offset=24
        i32.const 7
        local.set 2
      end
      local.get 7
      local.get 2
      i32.store offset=28
      block ;; label = @1
        local.get 5
        i32.load
        br_if 0 (;@1;)
        local.get 7
        i32.const 3
        i32.store offset=92
        local.get 7
        i32.const 1048844
        i32.store offset=88
        local.get 7
        i64.const 3
        i64.store offset=100 align=4
        local.get 7
        i32.const 7
        i64.extend_i32_u
        i64.const 32
        i64.shl
        local.tee 8
        local.get 7
        i32.const 16
        i32.add
        i64.extend_i32_u
        i64.or
        i64.store offset=72
        local.get 7
        local.get 8
        local.get 7
        i32.const 8
        i32.add
        i64.extend_i32_u
        i64.or
        i64.store offset=64
        local.get 7
        i32.const 8
        i64.extend_i32_u
        i64.const 32
        i64.shl
        local.get 7
        i32.const 24
        i32.add
        i64.extend_i32_u
        i64.or
        i64.store offset=56
        local.get 7
        local.get 7
        i32.const 56
        i32.add
        i32.store offset=96
        local.get 7
        i32.const 88
        i32.add
        local.get 6
        call $core::panicking::panic_fmt
        unreachable
      end
      local.get 7
      i32.const 32
      i32.add
      i32.const 16
      i32.add
      local.get 5
      i32.const 16
      i32.add
      i64.load align=4
      i64.store
      local.get 7
      i32.const 32
      i32.add
      i32.const 8
      i32.add
      local.get 5
      i32.const 8
      i32.add
      i64.load align=4
      i64.store
      local.get 7
      local.get 5
      i64.load align=4
      i64.store offset=32
      local.get 7
      i32.const 4
      i32.store offset=92
      local.get 7
      i32.const 1048896
      i32.store offset=88
      local.get 7
      i64.const 4
      i64.store offset=100 align=4
      local.get 7
      i32.const 7
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.tee 8
      local.get 7
      i32.const 16
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=80
      local.get 7
      local.get 8
      local.get 7
      i32.const 8
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=72
      local.get 7
      i32.const 9
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get 7
      i32.const 32
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=64
      local.get 7
      i32.const 8
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get 7
      i32.const 24
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=56
      local.get 7
      local.get 7
      i32.const 56
      i32.add
      i32.store offset=96
      local.get 7
      i32.const 88
      i32.add
      local.get 6
      call $core::panicking::panic_fmt
      unreachable
    )
    (func $<&T as core::fmt::Debug>::fmt (;29;) (type 1) (param i32 i32) (result i32)
      local.get 0
      i32.load
      local.get 1
      local.get 0
      i32.load offset=4
      i32.load offset=12
      call_indirect (type 1)
    )
    (func $<core::fmt::Arguments as core::fmt::Display>::fmt (;30;) (type 1) (param i32 i32) (result i32)
      local.get 1
      i32.load offset=20
      local.get 1
      i32.load offset=24
      local.get 0
      call $core::fmt::write
    )
    (func $<core::fmt::builders::PadAdapter as core::fmt::Write>::write_str (;31;) (type 0) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
      local.get 1
      i32.const -1
      i32.add
      local.set 3
      local.get 0
      i32.load offset=4
      local.set 4
      local.get 0
      i32.load
      local.set 5
      local.get 0
      i32.load offset=8
      local.set 6
      i32.const 0
      local.set 7
      i32.const 0
      local.set 8
      i32.const 0
      local.set 9
      i32.const 0
      local.set 10
      block ;; label = @1
        loop ;; label = @2
          local.get 10
          i32.const 1
          i32.and
          br_if 1 (;@1;)
          block ;; label = @3
            block ;; label = @4
              local.get 9
              local.get 2
              i32.gt_u
              br_if 0 (;@4;)
              loop ;; label = @5
                local.get 1
                local.get 9
                i32.add
                local.set 11
                block ;; label = @6
                  block ;; label = @7
                    block ;; label = @8
                      block ;; label = @9
                        local.get 2
                        local.get 9
                        i32.sub
                        local.tee 12
                        i32.const 7
                        i32.gt_u
                        br_if 0 (;@9;)
                        local.get 2
                        local.get 9
                        i32.ne
                        br_if 1 (;@8;)
                        local.get 2
                        local.set 9
                        br 5 (;@4;)
                      end
                      block ;; label = @9
                        block ;; label = @10
                          local.get 11
                          i32.const 3
                          i32.add
                          i32.const -4
                          i32.and
                          local.tee 13
                          local.get 11
                          i32.sub
                          local.tee 14
                          i32.eqz
                          br_if 0 (;@10;)
                          i32.const 0
                          local.set 0
                          loop ;; label = @11
                            local.get 11
                            local.get 0
                            i32.add
                            i32.load8_u
                            i32.const 10
                            i32.eq
                            br_if 5 (;@6;)
                            local.get 14
                            local.get 0
                            i32.const 1
                            i32.add
                            local.tee 0
                            i32.ne
                            br_if 0 (;@11;)
                          end
                          local.get 14
                          local.get 12
                          i32.const -8
                          i32.add
                          local.tee 15
                          i32.le_u
                          br_if 1 (;@9;)
                          br 3 (;@7;)
                        end
                        local.get 12
                        i32.const -8
                        i32.add
                        local.set 15
                      end
                      loop ;; label = @9
                        i32.const 16843008
                        local.get 13
                        i32.load
                        local.tee 0
                        i32.const 168430090
                        i32.xor
                        i32.sub
                        local.get 0
                        i32.or
                        i32.const 16843008
                        local.get 13
                        i32.const 4
                        i32.add
                        i32.load
                        local.tee 0
                        i32.const 168430090
                        i32.xor
                        i32.sub
                        local.get 0
                        i32.or
                        i32.and
                        i32.const -2139062144
                        i32.and
                        i32.const -2139062144
                        i32.ne
                        br_if 2 (;@7;)
                        local.get 13
                        i32.const 8
                        i32.add
                        local.set 13
                        local.get 14
                        i32.const 8
                        i32.add
                        local.tee 14
                        local.get 15
                        i32.le_u
                        br_if 0 (;@9;)
                        br 2 (;@7;)
                      end
                    end
                    i32.const 0
                    local.set 0
                    loop ;; label = @8
                      local.get 11
                      local.get 0
                      i32.add
                      i32.load8_u
                      i32.const 10
                      i32.eq
                      br_if 2 (;@6;)
                      local.get 12
                      local.get 0
                      i32.const 1
                      i32.add
                      local.tee 0
                      i32.ne
                      br_if 0 (;@8;)
                    end
                    local.get 2
                    local.set 9
                    br 3 (;@4;)
                  end
                  block ;; label = @7
                    local.get 14
                    local.get 12
                    i32.ne
                    br_if 0 (;@7;)
                    local.get 2
                    local.set 9
                    br 3 (;@4;)
                  end
                  loop ;; label = @7
                    block ;; label = @8
                      local.get 11
                      local.get 14
                      i32.add
                      i32.load8_u
                      i32.const 10
                      i32.ne
                      br_if 0 (;@8;)
                      local.get 14
                      local.set 0
                      br 2 (;@6;)
                    end
                    local.get 12
                    local.get 14
                    i32.const 1
                    i32.add
                    local.tee 14
                    i32.ne
                    br_if 0 (;@7;)
                  end
                  local.get 2
                  local.set 9
                  br 2 (;@4;)
                end
                local.get 0
                local.get 9
                i32.add
                local.tee 14
                i32.const 1
                i32.add
                local.set 9
                block ;; label = @6
                  local.get 14
                  local.get 2
                  i32.ge_u
                  br_if 0 (;@6;)
                  local.get 11
                  local.get 0
                  i32.add
                  i32.load8_u
                  i32.const 10
                  i32.ne
                  br_if 0 (;@6;)
                  local.get 9
                  local.set 11
                  local.get 9
                  local.set 0
                  br 3 (;@3;)
                end
                local.get 9
                local.get 2
                i32.le_u
                br_if 0 (;@5;)
              end
            end
            i32.const 1
            local.set 10
            local.get 8
            local.set 11
            local.get 2
            local.set 0
            local.get 8
            local.get 2
            i32.eq
            br_if 2 (;@1;)
          end
          block ;; label = @3
            block ;; label = @4
              local.get 6
              i32.load8_u
              i32.eqz
              br_if 0 (;@4;)
              local.get 5
              i32.const 1048956
              i32.const 4
              local.get 4
              i32.load offset=12
              call_indirect (type 0)
              br_if 1 (;@3;)
            end
            local.get 0
            local.get 8
            i32.sub
            local.set 13
            i32.const 0
            local.set 14
            block ;; label = @4
              local.get 0
              local.get 8
              i32.eq
              br_if 0 (;@4;)
              local.get 3
              local.get 0
              i32.add
              i32.load8_u
              i32.const 10
              i32.eq
              local.set 14
            end
            local.get 1
            local.get 8
            i32.add
            local.set 0
            local.get 6
            local.get 14
            i32.store8
            local.get 11
            local.set 8
            local.get 5
            local.get 0
            local.get 13
            local.get 4
            i32.load offset=12
            call_indirect (type 0)
            i32.eqz
            br_if 1 (;@2;)
          end
        end
        i32.const 1
        local.set 7
      end
      local.get 7
    )
    (func $<core::fmt::builders::PadAdapter as core::fmt::Write>::write_char (;32;) (type 1) (param i32 i32) (result i32)
      (local i32 i32)
      local.get 0
      i32.load offset=4
      local.set 2
      local.get 0
      i32.load
      local.set 3
      block ;; label = @1
        local.get 0
        i32.load offset=8
        local.tee 0
        i32.load8_u
        i32.eqz
        br_if 0 (;@1;)
        local.get 3
        i32.const 1048956
        i32.const 4
        local.get 2
        i32.load offset=12
        call_indirect (type 0)
        i32.eqz
        br_if 0 (;@1;)
        i32.const 1
        return
      end
      local.get 0
      local.get 1
      i32.const 10
      i32.eq
      i32.store8
      local.get 3
      local.get 1
      local.get 2
      i32.load offset=16
      call_indirect (type 1)
    )
    (func $core::fmt::builders::DebugStruct::finish (;33;) (type 13) (param i32) (result i32)
      (local i32 i32)
      local.get 0
      i32.load8_u offset=4
      local.tee 1
      local.set 2
      block ;; label = @1
        local.get 0
        i32.load8_u offset=5
        i32.eqz
        br_if 0 (;@1;)
        i32.const 1
        local.set 2
        block ;; label = @2
          local.get 1
          i32.const 1
          i32.and
          br_if 0 (;@2;)
          block ;; label = @3
            local.get 0
            i32.load
            local.tee 2
            i32.load8_u offset=28
            i32.const 4
            i32.and
            br_if 0 (;@3;)
            local.get 2
            i32.load offset=20
            i32.const 1048971
            i32.const 2
            local.get 2
            i32.load offset=24
            i32.load offset=12
            call_indirect (type 0)
            local.set 2
            br 1 (;@2;)
          end
          local.get 2
          i32.load offset=20
          i32.const 1048970
          i32.const 1
          local.get 2
          i32.load offset=24
          i32.load offset=12
          call_indirect (type 0)
          local.set 2
        end
        local.get 0
        local.get 2
        i32.store8 offset=4
      end
      local.get 2
      i32.const 1
      i32.and
    )
    (func $core::fmt::Formatter::pad_integral (;34;) (type 14) (param i32 i32 i32 i32 i32 i32) (result i32)
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
            i32.const 12
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
        local.get 0
        i32.load
        br_if 0 (;@1;)
        block ;; label = @2
          local.get 0
          i32.load offset=20
          local.tee 1
          local.get 0
          i32.load offset=24
          local.tee 12
          local.get 8
          local.get 2
          local.get 3
          call $core::fmt::Formatter::pad_integral::write_prefix
          i32.eqz
          br_if 0 (;@2;)
          i32.const 1
          return
        end
        local.get 1
        local.get 4
        local.get 5
        local.get 12
        i32.load offset=12
        call_indirect (type 0)
        return
      end
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 0
              i32.load offset=4
              local.tee 1
              local.get 6
              i32.gt_u
              br_if 0 (;@4;)
              local.get 0
              i32.load offset=20
              local.tee 1
              local.get 0
              i32.load offset=24
              local.tee 12
              local.get 8
              local.get 2
              local.get 3
              call $core::fmt::Formatter::pad_integral::write_prefix
              i32.eqz
              br_if 1 (;@3;)
              i32.const 1
              return
            end
            local.get 7
            i32.const 8
            i32.and
            i32.eqz
            br_if 1 (;@2;)
            local.get 0
            i32.load offset=16
            local.set 9
            local.get 0
            i32.const 48
            i32.store offset=16
            local.get 0
            i32.load8_u offset=32
            local.set 7
            i32.const 1
            local.set 11
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
            br_if 2 (;@1;)
            local.get 1
            local.get 6
            i32.sub
            i32.const 1
            i32.add
            local.set 1
            block ;; label = @4
              loop ;; label = @5
                local.get 1
                i32.const -1
                i32.add
                local.tee 1
                i32.eqz
                br_if 1 (;@4;)
                local.get 12
                i32.const 48
                local.get 10
                i32.load offset=16
                call_indirect (type 1)
                i32.eqz
                br_if 0 (;@5;)
              end
              i32.const 1
              return
            end
            block ;; label = @4
              local.get 12
              local.get 4
              local.get 5
              local.get 10
              i32.load offset=12
              call_indirect (type 0)
              i32.eqz
              br_if 0 (;@4;)
              i32.const 1
              return
            end
            local.get 0
            local.get 7
            i32.store8 offset=32
            local.get 0
            local.get 9
            i32.store offset=16
            i32.const 0
            return
          end
          local.get 1
          local.get 4
          local.get 5
          local.get 12
          i32.load offset=12
          call_indirect (type 0)
          local.set 11
          br 1 (;@1;)
        end
        local.get 1
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
        i32.load offset=16
        local.set 9
        local.get 0
        i32.load offset=24
        local.set 12
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
        local.set 11
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
      local.get 11
    )
    (func $core::fmt::Write::write_fmt (;35;) (type 1) (param i32 i32) (result i32)
      local.get 0
      i32.const 1048932
      local.get 1
      call $core::fmt::write
    )
    (func $core::str::count::do_count_chars (;36;) (type 1) (param i32 i32) (result i32)
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
                local.get 0
                local.get 2
                i32.sub
                local.tee 8
                i32.const -4
                i32.le_u
                br_if 0 (;@5;)
                i32.const 0
                local.set 9
                br 1 (;@4;)
              end
              i32.const 0
              local.set 9
              loop ;; label = @5
                local.get 1
                local.get 0
                local.get 9
                i32.add
                local.tee 2
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.get 2
                i32.const 1
                i32.add
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.get 2
                i32.const 2
                i32.add
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.get 2
                i32.const 3
                i32.add
                i32.load8_s
                i32.const -65
                i32.gt_s
                i32.add
                local.set 1
                local.get 9
                i32.const 4
                i32.add
                local.tee 9
                br_if 0 (;@5;)
              end
            end
            local.get 7
            br_if 0 (;@3;)
            local.get 0
            local.get 9
            i32.add
            local.set 2
            loop ;; label = @4
              local.get 1
              local.get 2
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.set 1
              local.get 2
              i32.const 1
              i32.add
              local.set 2
              local.get 8
              i32.const 1
              i32.add
              local.tee 8
              br_if 0 (;@4;)
            end
          end
          local.get 0
          local.get 3
          i32.add
          local.set 9
          block ;; label = @3
            local.get 5
            i32.eqz
            br_if 0 (;@3;)
            local.get 9
            local.get 4
            i32.const -4
            i32.and
            i32.add
            local.tee 2
            i32.load8_s
            i32.const -65
            i32.gt_s
            local.set 6
            local.get 5
            i32.const 1
            i32.eq
            br_if 0 (;@3;)
            local.get 6
            local.get 2
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
            local.get 2
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
          local.set 8
          loop ;; label = @3
            local.get 9
            local.set 4
            local.get 3
            i32.eqz
            br_if 2 (;@1;)
            local.get 3
            i32.const 192
            local.get 3
            i32.const 192
            i32.lt_u
            select
            local.tee 6
            i32.const 3
            i32.and
            local.set 7
            local.get 6
            i32.const 2
            i32.shl
            local.set 5
            i32.const 0
            local.set 2
            block ;; label = @4
              local.get 3
              i32.const 4
              i32.lt_u
              br_if 0 (;@4;)
              local.get 4
              local.get 5
              i32.const 1008
              i32.and
              i32.add
              local.set 0
              i32.const 0
              local.set 2
              local.get 4
              local.set 1
              loop ;; label = @5
                local.get 1
                i32.load offset=12
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
                i32.load offset=4
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
                i32.load
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
                local.get 2
                i32.add
                i32.add
                i32.add
                i32.add
                local.set 2
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
            local.get 6
            i32.sub
            local.set 3
            local.get 4
            local.get 5
            i32.add
            local.set 9
            local.get 2
            i32.const 8
            i32.shr_u
            i32.const 16711935
            i32.and
            local.get 2
            i32.const 16711935
            i32.and
            i32.add
            i32.const 65537
            i32.mul
            i32.const 16
            i32.shr_u
            local.get 8
            i32.add
            local.set 8
            local.get 7
            i32.eqz
            br_if 0 (;@3;)
          end
          local.get 4
          local.get 6
          i32.const 252
          i32.and
          i32.const 2
          i32.shl
          i32.add
          local.tee 2
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
            local.get 2
            i32.load offset=4
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
            local.get 7
            i32.const 2
            i32.eq
            br_if 0 (;@3;)
            local.get 2
            i32.load offset=8
            local.tee 2
            i32.const -1
            i32.xor
            i32.const 7
            i32.shr_u
            local.get 2
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
          local.get 8
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
        local.set 9
        block ;; label = @2
          block ;; label = @3
            local.get 1
            i32.const 4
            i32.ge_u
            br_if 0 (;@3;)
            i32.const 0
            local.set 8
            i32.const 0
            local.set 2
            br 1 (;@2;)
          end
          local.get 1
          i32.const -4
          i32.and
          local.set 3
          i32.const 0
          local.set 8
          i32.const 0
          local.set 2
          loop ;; label = @3
            local.get 8
            local.get 0
            local.get 2
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
            local.set 8
            local.get 3
            local.get 2
            i32.const 4
            i32.add
            local.tee 2
            i32.ne
            br_if 0 (;@3;)
          end
        end
        local.get 9
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        local.get 2
        i32.add
        local.set 1
        loop ;; label = @2
          local.get 8
          local.get 1
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.set 8
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 9
          i32.const -1
          i32.add
          local.tee 9
          br_if 0 (;@2;)
        end
      end
      local.get 8
    )
    (func $core::fmt::Formatter::pad_integral::write_prefix (;37;) (type 11) (param i32 i32 i32 i32 i32) (result i32)
      block ;; label = @1
        local.get 2
        i32.const 1114112
        i32.eq
        br_if 0 (;@1;)
        local.get 0
        local.get 2
        local.get 1
        i32.load offset=16
        call_indirect (type 1)
        i32.eqz
        br_if 0 (;@1;)
        i32.const 1
        return
      end
      block ;; label = @1
        local.get 3
        br_if 0 (;@1;)
        i32.const 0
        return
      end
      local.get 0
      local.get 3
      local.get 4
      local.get 1
      i32.load offset=12
      call_indirect (type 0)
    )
    (func $core::fmt::Formatter::debug_struct (;38;) (type 9) (param i32 i32 i32 i32)
      local.get 1
      i32.load offset=20
      local.get 2
      local.get 3
      local.get 1
      i32.load offset=24
      i32.load offset=12
      call_indirect (type 0)
      local.set 3
      local.get 0
      i32.const 0
      i32.store8 offset=5
      local.get 0
      local.get 3
      i32.store8 offset=4
      local.get 0
      local.get 1
      i32.store
    )
    (func $core::fmt::num::imp::<impl core::fmt::Display for u64>::fmt (;39;) (type 1) (param i32 i32) (result i32)
      local.get 0
      i64.load
      i32.const 1
      local.get 1
      call $core::fmt::num::imp::fmt_u64
    )
    (func $core::fmt::num::imp::fmt_u64 (;40;) (type 15) (param i64 i32 i32) (result i32)
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
          i32.const 1049010
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
          i32.const 1049010
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
        block ;; label = @2
          local.get 5
          i64.const 99
          i64.gt_u
          br_if 0 (;@2;)
          local.get 5
          i32.wrap_i64
          local.set 6
          br 1 (;@1;)
        end
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
        i32.const 1049010
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
          i32.const 1049010
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
        i32.or
        i32.store8
      end
      local.get 2
      local.get 1
      i32.const 1
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
    (func $core::fmt::num::<impl core::fmt::LowerHex for i64>::fmt (;41;) (type 1) (param i32 i32) (result i32)
      (local i32 i64 i32)
      global.get $__stack_pointer
      i32.const 128
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 0
      i64.load
      local.set 3
      i32.const 0
      local.set 0
      loop ;; label = @1
        local.get 2
        local.get 0
        i32.add
        i32.const 127
        i32.add
        local.get 3
        i32.wrap_i64
        i32.const 15
        i32.and
        local.tee 4
        i32.const 48
        i32.or
        local.get 4
        i32.const 87
        i32.add
        local.get 4
        i32.const 10
        i32.lt_u
        select
        i32.store8
        local.get 0
        i32.const -1
        i32.add
        local.set 0
        local.get 3
        i64.const 16
        i64.lt_u
        local.set 4
        local.get 3
        i64.const 4
        i64.shr_u
        local.set 3
        local.get 4
        i32.eqz
        br_if 0 (;@1;)
      end
      block ;; label = @1
        local.get 0
        i32.const 128
        i32.add
        local.tee 4
        i32.const 129
        i32.lt_u
        br_if 0 (;@1;)
        local.get 4
        i32.const 128
        i32.const 1048992
        call $core::slice::index::slice_start_index_len_fail
        unreachable
      end
      local.get 1
      i32.const 1
      i32.const 1049008
      i32.const 2
      local.get 2
      local.get 0
      i32.add
      i32.const 128
      i32.add
      i32.const 0
      local.get 0
      i32.sub
      call $core::fmt::Formatter::pad_integral
      local.set 0
      local.get 2
      i32.const 128
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $core::fmt::num::<impl core::fmt::UpperHex for i64>::fmt (;42;) (type 1) (param i32 i32) (result i32)
      (local i32 i64 i32)
      global.get $__stack_pointer
      i32.const 128
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 0
      i64.load
      local.set 3
      i32.const 0
      local.set 0
      loop ;; label = @1
        local.get 2
        local.get 0
        i32.add
        i32.const 127
        i32.add
        local.get 3
        i32.wrap_i64
        i32.const 15
        i32.and
        local.tee 4
        i32.const 48
        i32.or
        local.get 4
        i32.const 55
        i32.add
        local.get 4
        i32.const 10
        i32.lt_u
        select
        i32.store8
        local.get 0
        i32.const -1
        i32.add
        local.set 0
        local.get 3
        i64.const 16
        i64.lt_u
        local.set 4
        local.get 3
        i64.const 4
        i64.shr_u
        local.set 3
        local.get 4
        i32.eqz
        br_if 0 (;@1;)
      end
      block ;; label = @1
        local.get 0
        i32.const 128
        i32.add
        local.tee 4
        i32.const 129
        i32.lt_u
        br_if 0 (;@1;)
        local.get 4
        i32.const 128
        i32.const 1048992
        call $core::slice::index::slice_start_index_len_fail
        unreachable
      end
      local.get 1
      i32.const 1
      i32.const 1049008
      i32.const 2
      local.get 2
      local.get 0
      i32.add
      i32.const 128
      i32.add
      i32.const 0
      local.get 0
      i32.sub
      call $core::fmt::Formatter::pad_integral
      local.set 0
      local.get 2
      i32.const 128
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $cabi_realloc (;43;) (type 8) (param i32 i32 i32 i32) (result i32)
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      call $wit_bindgen_rt::cabi_realloc
    )
    (table (;0;) 13 13 funcref)
    (memory (;0;) 17)
    (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
    (export "memory" (memory 0))
    (export "miden:base/note-script@1.0.0#note-script" (func $miden:base/note-script@1.0.0#note-script))
    (export "cabi_realloc" (func $cabi_realloc))
    (export "cabi_realloc_wit_bindgen_0_28_0" (func $wit_bindgen_rt::cabi_realloc))
    (elem (;0;) (i32.const 1) func $<&T as core::fmt::Debug>::fmt $basic_wallet_p2id_note::bindings::__link_custom_section_describing_imports $core::fmt::num::<impl core::fmt::Debug for u64>::fmt $<basic_wallet_p2id_note::bindings::miden::base::core_types::Felt as core::fmt::Debug>::fmt $cabi_realloc $core::fmt::num::imp::<impl core::fmt::Display for u32>::fmt $<&T as core::fmt::Debug>::fmt $<&T as core::fmt::Display>::fmt $<core::fmt::Arguments as core::fmt::Display>::fmt $<core::fmt::builders::PadAdapter as core::fmt::Write>::write_str $<core::fmt::builders::PadAdapter as core::fmt::Write>::write_char $core::fmt::Write::write_fmt)
    (data $.rodata (;0;) (i32.const 1048576) "\00\00\00\00\04\00\00\00\04\00\00\00\01\00\00\00\02\00\00\00Felt\00\00\00\00\08\00\00\00\08\00\00\00\03\00\00\00innerAccountId\00\00\00\00\00\00\08\00\00\00\08\00\00\00\04\00\00\00\02\00\00\00\02\00\00\00\02\00\00\00\02\00\00\00\02\00\00\00src/lib.rs\00\00\5c\00\10\00\0a\00\00\00!\00\00\00,\00\00\00\5c\00\10\00\0a\00\00\00$\00\00\00\09\00\00\00\05\00\00\00index out of bounds: the len is  but the index is \00\00\8c\00\10\00 \00\00\00\ac\00\10\00\12\00\00\00==!=matchesassertion `left  right` failed\0a  left: \0a right: \00\db\00\10\00\10\00\00\00\eb\00\10\00\17\00\00\00\02\01\10\00\09\00\00\00 right` failed: \0a  left: \00\00\00\db\00\10\00\10\00\00\00$\01\10\00\10\00\00\004\01\10\00\09\00\00\00\02\01\10\00\09\00\00\00: \00\00\00\00\00\00\0c\00\00\00\04\00\00\00\0a\00\00\00\0b\00\00\00\0c\00\00\00     { ,  {\0a,\0a} }core/src/fmt/num.rs\8d\01\10\00\13\00\00\00f\00\00\00\17\00\00\000x00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899range start index  out of range for slice of length \00\00z\02\10\00\12\00\00\00\8c\02\10\00\22\00\00\00")
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