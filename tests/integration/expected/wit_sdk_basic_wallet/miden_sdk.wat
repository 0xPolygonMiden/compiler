(component
  (core module (;0;)
    (type (;0;) (func))
    (type (;1;) (func (param i32)))
    (type (;2;) (func (param i32 i32) (result i32)))
    (type (;3;) (func (param i32 i32 i32 i32) (result i32)))
    (type (;4;) (func (param i64) (result i64)))
    (type (;5;) (func (param i64 i64 i64 i64) (result i32)))
    (type (;6;) (func (param i32 i64 i64 i64 i64) (result i32)))
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
    (func $miden_sdk::bindings::__link_custom_section_describing_imports (;2;) (type 0))
    (func $__rust_alloc (;3;) (type 2) (param i32 i32) (result i32)
      i32.const 1048652
      local.get 1
      local.get 0
      call $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::alloc
    )
    (func $__rust_realloc (;4;) (type 3) (param i32 i32 i32 i32) (result i32)
      (local i32)
      block ;; label = @1
        i32.const 1048652
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
        i32.const 1048652
        local.get 0
        local.get 2
        local.get 1
        call $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::dealloc
      end
      local.get 4
    )
    (func $miden:base/core-types@1.0.0#account-id-from-felt (;5;) (type 4) (param i64) (result i64)
      call $wit_bindgen_rt::run_ctors_once
      local.get 0
    )
    (func $miden:base/types@1.0.0#from-core-asset (;6;) (type 5) (param i64 i64 i64 i64) (result i32)
      call $wit_bindgen_rt::run_ctors_once
      i32.const 1048584
      i32.const 19
      i32.const 1048616
      call $core::panicking::panic
      unreachable
    )
    (func $miden:base/types@1.0.0#to-core-asset (;7;) (type 6) (param i32 i64 i64 i64 i64) (result i32)
      call $wit_bindgen_rt::run_ctors_once
      i32.const 1048584
      i32.const 19
      i32.const 1048632
      call $core::panicking::panic
      unreachable
    )
    (func $wit_bindgen_rt::cabi_realloc (;8;) (type 3) (param i32 i32 i32 i32) (result i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 1
            br_if 0 (;@3;)
            local.get 3
            i32.eqz
            br_if 2 (;@1;)
            i32.const 0
            i32.load8_u offset=1048656
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
    (func $wit_bindgen_rt::run_ctors_once (;9;) (type 0)
      block ;; label = @1
        i32.const 0
        i32.load8_u offset=1048657
        br_if 0 (;@1;)
        call $__wasm_call_ctors
        i32.const 0
        i32.const 1
        i32.store8 offset=1048657
      end
    )
    (func $wee_alloc::alloc_first_fit (;10;) (type 7) (param i32 i32 i32) (result i32)
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
    (func $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::alloc (;11;) (type 7) (param i32 i32 i32) (result i32)
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
    (func $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::dealloc (;12;) (type 8) (param i32 i32 i32 i32)
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
    (func $core::panicking::panic_fmt (;13;) (type 9) (param i32 i32)
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
    (func $cabi_realloc (;15;) (type 3) (param i32 i32 i32 i32) (result i32)
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      call $wit_bindgen_rt::cabi_realloc
    )
    (table (;0;) 3 3 funcref)
    (memory (;0;) 17)
    (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
    (export "memory" (memory 0))
    (export "miden:base/core-types@1.0.0#account-id-from-felt" (func $miden:base/core-types@1.0.0#account-id-from-felt))
    (export "miden:base/types@1.0.0#from-core-asset" (func $miden:base/types@1.0.0#from-core-asset))
    (export "miden:base/types@1.0.0#to-core-asset" (func $miden:base/types@1.0.0#to-core-asset))
    (export "cabi_realloc" (func $cabi_realloc))
    (export "cabi_realloc_wit_bindgen_0_28_0" (func $wit_bindgen_rt::cabi_realloc))
    (elem (;0;) (i32.const 1) func $miden_sdk::bindings::__link_custom_section_describing_imports $cabi_realloc)
    (data $.rodata (;0;) (i32.const 1048576) "\01\00\00\00\01\00\00\00not yet implementedsrc/lib.rs\00\00\00\1b\00\10\00\0a\00\00\00!\00\00\00\09\00\00\00\1b\00\10\00\0a\00\00\00%\00\00\00\09\00\00\00\02\00\00\00")
  )
  (core instance (;0;) (instantiate 0))
  (alias core export 0 "memory" (core memory (;0;)))
  (alias core export 0 "cabi_realloc" (core func (;0;)))
  (type (;0;) (record (field "inner" u64)))
  (type (;1;) (record (field "inner" 0)))
  (type (;2;) (func (param "felt" 0) (result 1)))
  (alias core export 0 "miden:base/core-types@1.0.0#account-id-from-felt" (core func (;1;)))
  (func (;0;) (type 2) (canon lift (core func 1)))
  (component (;0;)
    (type (;0;) (record (field "inner" u64)))
    (import "import-type-felt" (type (;1;) (eq 0)))
    (type (;2;) (record (field "inner" 1)))
    (import "import-type-account-id" (type (;3;) (eq 2)))
    (type (;4;) (func (param "felt" 1) (result 3)))
    (import "import-func-account-id-from-felt" (func (;0;) (type 4)))
    (type (;5;) (record (field "inner" u64)))
    (export (;6;) "felt" (type 5))
    (type (;7;) (tuple 6 6 6 6))
    (export (;8;) "word" (type 7))
    (type (;9;) (record (field "inner" 6)))
    (export (;10;) "account-id" (type 9))
    (type (;11;) (record (field "inner" 8)))
    (export (;12;) "recipient" (type 11))
    (type (;13;) (record (field "inner" 6)))
    (export (;14;) "tag" (type 13))
    (type (;15;) (record (field "inner" 8)))
    (export (;16;) "core-asset" (type 15))
    (type (;17;) (record (field "inner" 6)))
    (export (;18;) "nonce" (type 17))
    (type (;19;) (record (field "inner" 8)))
    (export (;20;) "account-hash" (type 19))
    (type (;21;) (record (field "inner" 8)))
    (export (;22;) "block-hash" (type 21))
    (type (;23;) (record (field "inner" 8)))
    (export (;24;) "storage-value" (type 23))
    (type (;25;) (record (field "inner" 8)))
    (export (;26;) "storage-root" (type 25))
    (type (;27;) (record (field "inner" 8)))
    (export (;28;) "account-code-root" (type 27))
    (type (;29;) (record (field "inner" 8)))
    (export (;30;) "vault-commitment" (type 29))
    (type (;31;) (record (field "inner" 6)))
    (export (;32;) "note-id" (type 31))
    (type (;33;) (func (param "felt" 6) (result 10)))
    (export (;1;) "account-id-from-felt" (func 0) (func (type 33)))
  )
  (instance (;0;) (instantiate 0
      (with "import-func-account-id-from-felt" (func 0))
      (with "import-type-felt" (type 0))
      (with "import-type-account-id" (type 1))
    )
  )
  (export (;1;) "miden:base/core-types@1.0.0" (instance 0))
  (alias export 1 "core-asset" (type (;3;)))
  (alias export 1 "account-id" (type (;4;)))
  (type (;5;) (record (field "asset" 4) (field "amount" u64)))
  (alias export 1 "word" (type (;6;)))
  (type (;7;) (record (field "inner" 6)))
  (type (;8;) (variant (case "fungible" 5) (case "non-fungible" 7)))
  (type (;9;) (func (param "core-asset" 3) (result 8)))
  (alias core export 0 "miden:base/types@1.0.0#from-core-asset" (core func (;2;)))
  (func (;1;) (type 9) (canon lift (core func 2) (memory 0)))
  (type (;10;) (func (param "asset" 8) (result 3)))
  (alias core export 0 "miden:base/types@1.0.0#to-core-asset" (core func (;3;)))
  (func (;2;) (type 10) (canon lift (core func 3) (memory 0)))
  (alias export 1 "felt" (type (;11;)))
  (component (;1;)
    (type (;0;) (record (field "inner" u64)))
    (import "import-type-felt" (type (;1;) (eq 0)))
    (type (;2;) (record (field "inner" 1)))
    (import "import-type-account-id" (type (;3;) (eq 2)))
    (type (;4;) (tuple 1 1 1 1))
    (import "import-type-word" (type (;5;) (eq 4)))
    (type (;6;) (record (field "inner" 5)))
    (import "import-type-core-asset" (type (;7;) (eq 6)))
    (import "import-type-core-asset0" (type (;8;) (eq 7)))
    (import "import-type-account-id0" (type (;9;) (eq 3)))
    (type (;10;) (record (field "asset" 9) (field "amount" u64)))
    (import "import-type-fungible-asset" (type (;11;) (eq 10)))
    (import "import-type-word0" (type (;12;) (eq 5)))
    (type (;13;) (record (field "inner" 12)))
    (import "import-type-non-fungible-asset" (type (;14;) (eq 13)))
    (type (;15;) (variant (case "fungible" 11) (case "non-fungible" 14)))
    (import "import-type-asset" (type (;16;) (eq 15)))
    (type (;17;) (func (param "core-asset" 8) (result 16)))
    (import "import-func-from-core-asset" (func (;0;) (type 17)))
    (type (;18;) (func (param "asset" 16) (result 8)))
    (import "import-func-to-core-asset" (func (;1;) (type 18)))
    (export (;19;) "felt" (type 1))
    (export (;20;) "account-id" (type 3))
    (export (;21;) "word" (type 5))
    (export (;22;) "core-asset" (type 7))
    (type (;23;) (record (field "asset" 20) (field "amount" u64)))
    (export (;24;) "fungible-asset" (type 23))
    (type (;25;) (record (field "inner" 21)))
    (export (;26;) "non-fungible-asset" (type 25))
    (type (;27;) (variant (case "fungible" 24) (case "non-fungible" 26)))
    (export (;28;) "asset" (type 27))
    (type (;29;) (func (param "core-asset" 22) (result 28)))
    (export (;2;) "from-core-asset" (func 0) (func (type 29)))
    (type (;30;) (func (param "asset" 28) (result 22)))
    (export (;3;) "to-core-asset" (func 1) (func (type 30)))
  )
  (instance (;2;) (instantiate 1
      (with "import-func-from-core-asset" (func 1))
      (with "import-func-to-core-asset" (func 2))
      (with "import-type-felt" (type 11))
      (with "import-type-account-id" (type 4))
      (with "import-type-word" (type 6))
      (with "import-type-core-asset" (type 3))
      (with "import-type-core-asset0" (type 3))
      (with "import-type-account-id0" (type 4))
      (with "import-type-fungible-asset" (type 5))
      (with "import-type-word0" (type 6))
      (with "import-type-non-fungible-asset" (type 7))
      (with "import-type-asset" (type 8))
    )
  )
  (export (;3;) "miden:base/types@1.0.0" (instance 2))
)