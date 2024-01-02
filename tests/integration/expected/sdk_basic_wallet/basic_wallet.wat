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
    (type (;6;) (func (param i32) (result i32)))
    (type (;7;) (func (param i32)))
    (type (;8;) (func (param i32 i32)))
    (type (;9;) (func (param i32 i32 i32) (result i32)))
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
      (local i32)
      local.get 0
      local.get 1
      call $__rdl_alloc
      local.set 2
      local.get 2
      return
    )
    (func $__rust_realloc (;7;) (type 5) (param i32 i32 i32 i32) (result i32)
      (local i32)
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      call $__rdl_realloc
      local.set 4
      local.get 4
      return
    )
    (func $wit_bindgen::rt::run_ctors_once (;8;) (type 2)
      block ;; label = @1
        i32.const 0
        i32.load8_u offset=1048577
        br_if 0 (;@1;)
        call $__wasm_call_ctors
        i32.const 0
        i32.const 1
        i32.store8 offset=1048577
      end
    )
    (func $cabi_realloc (;9;) (type 5) (param i32 i32 i32 i32) (result i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 1
            br_if 0 (;@3;)
            local.get 3
            i32.eqz
            br_if 2 (;@1;)
            i32.const 0
            i32.load8_u offset=1048576
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
    (func $__rdl_alloc (;10;) (type 4) (param i32 i32) (result i32)
      block ;; label = @1
        block ;; label = @2
          local.get 1
          i32.const 8
          i32.gt_u
          br_if 0 (;@2;)
          local.get 1
          local.get 0
          i32.le_u
          br_if 1 (;@1;)
        end
        local.get 1
        local.get 0
        call $aligned_alloc
        return
      end
      local.get 0
      call $malloc
    )
    (func $__rdl_realloc (;11;) (type 5) (param i32 i32 i32 i32) (result i32)
      block ;; label = @1
        block ;; label = @2
          local.get 2
          i32.const 8
          i32.gt_u
          br_if 0 (;@2;)
          local.get 2
          local.get 3
          i32.le_u
          br_if 1 (;@1;)
        end
        block ;; label = @2
          local.get 2
          local.get 3
          call $aligned_alloc
          local.tee 2
          br_if 0 (;@2;)
          i32.const 0
          return
        end
        local.get 2
        local.get 0
        local.get 1
        local.get 3
        local.get 1
        local.get 3
        i32.lt_u
        select
        call $memcpy
        local.set 3
        local.get 0
        call $free
        local.get 3
        return
      end
      local.get 0
      local.get 3
      call $realloc
    )
    (func $malloc (;12;) (type 6) (param i32) (result i32)
      local.get 0
      call $dlmalloc
    )
    (func $dlmalloc (;13;) (type 6) (param i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      block ;; label = @1
        i32.const 0
        i32.load offset=1048604
        local.tee 2
        br_if 0 (;@1;)
        block ;; label = @2
          block ;; label = @3
            i32.const 0
            i32.load offset=1049052
            local.tee 3
            i32.eqz
            br_if 0 (;@3;)
            i32.const 0
            i32.load offset=1049056
            local.set 4
            br 1 (;@2;)
          end
          i32.const 0
          i64.const -1
          i64.store offset=1049064 align=4
          i32.const 0
          i64.const 281474976776192
          i64.store offset=1049056 align=4
          i32.const 0
          local.get 1
          i32.const 8
          i32.add
          i32.const -16
          i32.and
          i32.const 1431655768
          i32.xor
          local.tee 3
          i32.store offset=1049052
          i32.const 0
          i32.const 0
          i32.store offset=1049072
          i32.const 0
          i32.const 0
          i32.store offset=1049024
          i32.const 65536
          local.set 4
        end
        i32.const 0
        local.set 2
        i32.const 1114112
        i32.const 1049088
        local.get 4
        i32.add
        i32.const -1
        i32.add
        i32.const 0
        local.get 4
        i32.sub
        i32.and
        i32.const 1114112
        select
        i32.const 1049088
        i32.sub
        local.tee 5
        i32.const 89
        i32.lt_u
        br_if 0 (;@1;)
        i32.const 0
        local.set 4
        i32.const 0
        local.get 5
        i32.store offset=1049032
        i32.const 0
        i32.const 1049088
        i32.store offset=1049028
        i32.const 0
        i32.const 1049088
        i32.store offset=1048596
        i32.const 0
        local.get 3
        i32.store offset=1048616
        i32.const 0
        i32.const -1
        i32.store offset=1048612
        loop ;; label = @2
          local.get 4
          i32.const 1048640
          i32.add
          local.get 4
          i32.const 1048628
          i32.add
          local.tee 3
          i32.store
          local.get 3
          local.get 4
          i32.const 1048620
          i32.add
          local.tee 6
          i32.store
          local.get 4
          i32.const 1048632
          i32.add
          local.get 6
          i32.store
          local.get 4
          i32.const 1048648
          i32.add
          local.get 4
          i32.const 1048636
          i32.add
          local.tee 6
          i32.store
          local.get 6
          local.get 3
          i32.store
          local.get 4
          i32.const 1048656
          i32.add
          local.get 4
          i32.const 1048644
          i32.add
          local.tee 3
          i32.store
          local.get 3
          local.get 6
          i32.store
          local.get 4
          i32.const 1048652
          i32.add
          local.get 3
          i32.store
          local.get 4
          i32.const 32
          i32.add
          local.tee 4
          i32.const 256
          i32.ne
          br_if 0 (;@2;)
        end
        i32.const 1049088
        i32.const -8
        i32.const 1049088
        i32.sub
        i32.const 15
        i32.and
        i32.const 0
        i32.const 1049088
        i32.const 8
        i32.add
        i32.const 15
        i32.and
        select
        local.tee 4
        i32.add
        local.tee 2
        i32.const 4
        i32.add
        local.get 5
        i32.const -56
        i32.add
        local.tee 3
        local.get 4
        i32.sub
        local.tee 4
        i32.const 1
        i32.or
        i32.store
        i32.const 0
        i32.const 0
        i32.load offset=1049068
        i32.store offset=1048608
        i32.const 0
        local.get 4
        i32.store offset=1048592
        i32.const 0
        local.get 2
        i32.store offset=1048604
        i32.const 1049088
        local.get 3
        i32.add
        i32.const 56
        i32.store offset=4
      end
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    block ;; label = @8
                      block ;; label = @9
                        block ;; label = @10
                          block ;; label = @11
                            block ;; label = @12
                              local.get 0
                              i32.const 236
                              i32.gt_u
                              br_if 0 (;@12;)
                              block ;; label = @13
                                i32.const 0
                                i32.load offset=1048580
                                local.tee 7
                                i32.const 16
                                local.get 0
                                i32.const 19
                                i32.add
                                i32.const -16
                                i32.and
                                local.get 0
                                i32.const 11
                                i32.lt_u
                                select
                                local.tee 5
                                i32.const 3
                                i32.shr_u
                                local.tee 3
                                i32.shr_u
                                local.tee 4
                                i32.const 3
                                i32.and
                                i32.eqz
                                br_if 0 (;@13;)
                                block ;; label = @14
                                  block ;; label = @15
                                    local.get 4
                                    i32.const 1
                                    i32.and
                                    local.get 3
                                    i32.or
                                    i32.const 1
                                    i32.xor
                                    local.tee 6
                                    i32.const 3
                                    i32.shl
                                    local.tee 3
                                    i32.const 1048620
                                    i32.add
                                    local.tee 4
                                    local.get 3
                                    i32.const 1048628
                                    i32.add
                                    i32.load
                                    local.tee 3
                                    i32.load offset=8
                                    local.tee 5
                                    i32.ne
                                    br_if 0 (;@15;)
                                    i32.const 0
                                    local.get 7
                                    i32.const -2
                                    local.get 6
                                    i32.rotl
                                    i32.and
                                    i32.store offset=1048580
                                    br 1 (;@14;)
                                  end
                                  local.get 4
                                  local.get 5
                                  i32.store offset=8
                                  local.get 5
                                  local.get 4
                                  i32.store offset=12
                                end
                                local.get 3
                                i32.const 8
                                i32.add
                                local.set 4
                                local.get 3
                                local.get 6
                                i32.const 3
                                i32.shl
                                local.tee 6
                                i32.const 3
                                i32.or
                                i32.store offset=4
                                local.get 3
                                local.get 6
                                i32.add
                                local.tee 3
                                local.get 3
                                i32.load offset=4
                                i32.const 1
                                i32.or
                                i32.store offset=4
                                br 12 (;@1;)
                              end
                              local.get 5
                              i32.const 0
                              i32.load offset=1048588
                              local.tee 8
                              i32.le_u
                              br_if 1 (;@11;)
                              block ;; label = @13
                                local.get 4
                                i32.eqz
                                br_if 0 (;@13;)
                                block ;; label = @14
                                  block ;; label = @15
                                    local.get 4
                                    local.get 3
                                    i32.shl
                                    i32.const 2
                                    local.get 3
                                    i32.shl
                                    local.tee 4
                                    i32.const 0
                                    local.get 4
                                    i32.sub
                                    i32.or
                                    i32.and
                                    local.tee 4
                                    i32.const 0
                                    local.get 4
                                    i32.sub
                                    i32.and
                                    i32.ctz
                                    local.tee 3
                                    i32.const 3
                                    i32.shl
                                    local.tee 4
                                    i32.const 1048620
                                    i32.add
                                    local.tee 6
                                    local.get 4
                                    i32.const 1048628
                                    i32.add
                                    i32.load
                                    local.tee 4
                                    i32.load offset=8
                                    local.tee 0
                                    i32.ne
                                    br_if 0 (;@15;)
                                    i32.const 0
                                    local.get 7
                                    i32.const -2
                                    local.get 3
                                    i32.rotl
                                    i32.and
                                    local.tee 7
                                    i32.store offset=1048580
                                    br 1 (;@14;)
                                  end
                                  local.get 6
                                  local.get 0
                                  i32.store offset=8
                                  local.get 0
                                  local.get 6
                                  i32.store offset=12
                                end
                                local.get 4
                                local.get 5
                                i32.const 3
                                i32.or
                                i32.store offset=4
                                local.get 4
                                local.get 3
                                i32.const 3
                                i32.shl
                                local.tee 3
                                i32.add
                                local.get 3
                                local.get 5
                                i32.sub
                                local.tee 6
                                i32.store
                                local.get 4
                                local.get 5
                                i32.add
                                local.tee 0
                                local.get 6
                                i32.const 1
                                i32.or
                                i32.store offset=4
                                block ;; label = @14
                                  local.get 8
                                  i32.eqz
                                  br_if 0 (;@14;)
                                  local.get 8
                                  i32.const -8
                                  i32.and
                                  i32.const 1048620
                                  i32.add
                                  local.set 5
                                  i32.const 0
                                  i32.load offset=1048600
                                  local.set 3
                                  block ;; label = @15
                                    block ;; label = @16
                                      local.get 7
                                      i32.const 1
                                      local.get 8
                                      i32.const 3
                                      i32.shr_u
                                      i32.shl
                                      local.tee 9
                                      i32.and
                                      br_if 0 (;@16;)
                                      i32.const 0
                                      local.get 7
                                      local.get 9
                                      i32.or
                                      i32.store offset=1048580
                                      local.get 5
                                      local.set 9
                                      br 1 (;@15;)
                                    end
                                    local.get 5
                                    i32.load offset=8
                                    local.set 9
                                  end
                                  local.get 9
                                  local.get 3
                                  i32.store offset=12
                                  local.get 5
                                  local.get 3
                                  i32.store offset=8
                                  local.get 3
                                  local.get 5
                                  i32.store offset=12
                                  local.get 3
                                  local.get 9
                                  i32.store offset=8
                                end
                                local.get 4
                                i32.const 8
                                i32.add
                                local.set 4
                                i32.const 0
                                local.get 0
                                i32.store offset=1048600
                                i32.const 0
                                local.get 6
                                i32.store offset=1048588
                                br 12 (;@1;)
                              end
                              i32.const 0
                              i32.load offset=1048584
                              local.tee 10
                              i32.eqz
                              br_if 1 (;@11;)
                              local.get 10
                              i32.const 0
                              local.get 10
                              i32.sub
                              i32.and
                              i32.ctz
                              i32.const 2
                              i32.shl
                              i32.const 1048884
                              i32.add
                              i32.load
                              local.tee 0
                              i32.load offset=4
                              i32.const -8
                              i32.and
                              local.get 5
                              i32.sub
                              local.set 3
                              local.get 0
                              local.set 6
                              block ;; label = @13
                                loop ;; label = @14
                                  block ;; label = @15
                                    local.get 6
                                    i32.load offset=16
                                    local.tee 4
                                    br_if 0 (;@15;)
                                    local.get 6
                                    i32.const 20
                                    i32.add
                                    i32.load
                                    local.tee 4
                                    i32.eqz
                                    br_if 2 (;@13;)
                                  end
                                  local.get 4
                                  i32.load offset=4
                                  i32.const -8
                                  i32.and
                                  local.get 5
                                  i32.sub
                                  local.tee 6
                                  local.get 3
                                  local.get 6
                                  local.get 3
                                  i32.lt_u
                                  local.tee 6
                                  select
                                  local.set 3
                                  local.get 4
                                  local.get 0
                                  local.get 6
                                  select
                                  local.set 0
                                  local.get 4
                                  local.set 6
                                  br 0 (;@14;)
                                end
                              end
                              local.get 0
                              i32.load offset=24
                              local.set 11
                              block ;; label = @13
                                local.get 0
                                i32.load offset=12
                                local.tee 9
                                local.get 0
                                i32.eq
                                br_if 0 (;@13;)
                                local.get 0
                                i32.load offset=8
                                local.tee 4
                                i32.const 0
                                i32.load offset=1048596
                                i32.lt_u
                                drop
                                local.get 9
                                local.get 4
                                i32.store offset=8
                                local.get 4
                                local.get 9
                                i32.store offset=12
                                br 11 (;@2;)
                              end
                              block ;; label = @13
                                local.get 0
                                i32.const 20
                                i32.add
                                local.tee 6
                                i32.load
                                local.tee 4
                                br_if 0 (;@13;)
                                local.get 0
                                i32.load offset=16
                                local.tee 4
                                i32.eqz
                                br_if 3 (;@10;)
                                local.get 0
                                i32.const 16
                                i32.add
                                local.set 6
                              end
                              loop ;; label = @13
                                local.get 6
                                local.set 2
                                local.get 4
                                local.tee 9
                                i32.const 20
                                i32.add
                                local.tee 6
                                i32.load
                                local.tee 4
                                br_if 0 (;@13;)
                                local.get 9
                                i32.const 16
                                i32.add
                                local.set 6
                                local.get 9
                                i32.load offset=16
                                local.tee 4
                                br_if 0 (;@13;)
                              end
                              local.get 2
                              i32.const 0
                              i32.store
                              br 10 (;@2;)
                            end
                            i32.const -1
                            local.set 5
                            local.get 0
                            i32.const -65
                            i32.gt_u
                            br_if 0 (;@11;)
                            local.get 0
                            i32.const 19
                            i32.add
                            local.tee 4
                            i32.const -16
                            i32.and
                            local.set 5
                            i32.const 0
                            i32.load offset=1048584
                            local.tee 10
                            i32.eqz
                            br_if 0 (;@11;)
                            i32.const 0
                            local.set 8
                            block ;; label = @12
                              local.get 5
                              i32.const 256
                              i32.lt_u
                              br_if 0 (;@12;)
                              i32.const 31
                              local.set 8
                              local.get 5
                              i32.const 16777215
                              i32.gt_u
                              br_if 0 (;@12;)
                              local.get 5
                              i32.const 38
                              local.get 4
                              i32.const 8
                              i32.shr_u
                              i32.clz
                              local.tee 4
                              i32.sub
                              i32.shr_u
                              i32.const 1
                              i32.and
                              local.get 4
                              i32.const 1
                              i32.shl
                              i32.sub
                              i32.const 62
                              i32.add
                              local.set 8
                            end
                            i32.const 0
                            local.get 5
                            i32.sub
                            local.set 3
                            block ;; label = @12
                              block ;; label = @13
                                block ;; label = @14
                                  block ;; label = @15
                                    local.get 8
                                    i32.const 2
                                    i32.shl
                                    i32.const 1048884
                                    i32.add
                                    i32.load
                                    local.tee 6
                                    br_if 0 (;@15;)
                                    i32.const 0
                                    local.set 4
                                    i32.const 0
                                    local.set 9
                                    br 1 (;@14;)
                                  end
                                  i32.const 0
                                  local.set 4
                                  local.get 5
                                  i32.const 0
                                  i32.const 25
                                  local.get 8
                                  i32.const 1
                                  i32.shr_u
                                  i32.sub
                                  local.get 8
                                  i32.const 31
                                  i32.eq
                                  select
                                  i32.shl
                                  local.set 0
                                  i32.const 0
                                  local.set 9
                                  loop ;; label = @15
                                    block ;; label = @16
                                      local.get 6
                                      i32.load offset=4
                                      i32.const -8
                                      i32.and
                                      local.get 5
                                      i32.sub
                                      local.tee 7
                                      local.get 3
                                      i32.ge_u
                                      br_if 0 (;@16;)
                                      local.get 7
                                      local.set 3
                                      local.get 6
                                      local.set 9
                                      local.get 7
                                      br_if 0 (;@16;)
                                      i32.const 0
                                      local.set 3
                                      local.get 6
                                      local.set 9
                                      local.get 6
                                      local.set 4
                                      br 3 (;@13;)
                                    end
                                    local.get 4
                                    local.get 6
                                    i32.const 20
                                    i32.add
                                    i32.load
                                    local.tee 7
                                    local.get 7
                                    local.get 6
                                    local.get 0
                                    i32.const 29
                                    i32.shr_u
                                    i32.const 4
                                    i32.and
                                    i32.add
                                    i32.const 16
                                    i32.add
                                    i32.load
                                    local.tee 6
                                    i32.eq
                                    select
                                    local.get 4
                                    local.get 7
                                    select
                                    local.set 4
                                    local.get 0
                                    i32.const 1
                                    i32.shl
                                    local.set 0
                                    local.get 6
                                    br_if 0 (;@15;)
                                  end
                                end
                                block ;; label = @14
                                  local.get 4
                                  local.get 9
                                  i32.or
                                  br_if 0 (;@14;)
                                  i32.const 0
                                  local.set 9
                                  i32.const 2
                                  local.get 8
                                  i32.shl
                                  local.tee 4
                                  i32.const 0
                                  local.get 4
                                  i32.sub
                                  i32.or
                                  local.get 10
                                  i32.and
                                  local.tee 4
                                  i32.eqz
                                  br_if 3 (;@11;)
                                  local.get 4
                                  i32.const 0
                                  local.get 4
                                  i32.sub
                                  i32.and
                                  i32.ctz
                                  i32.const 2
                                  i32.shl
                                  i32.const 1048884
                                  i32.add
                                  i32.load
                                  local.set 4
                                end
                                local.get 4
                                i32.eqz
                                br_if 1 (;@12;)
                              end
                              loop ;; label = @13
                                local.get 4
                                i32.load offset=4
                                i32.const -8
                                i32.and
                                local.get 5
                                i32.sub
                                local.tee 7
                                local.get 3
                                i32.lt_u
                                local.set 0
                                block ;; label = @14
                                  local.get 4
                                  i32.load offset=16
                                  local.tee 6
                                  br_if 0 (;@14;)
                                  local.get 4
                                  i32.const 20
                                  i32.add
                                  i32.load
                                  local.set 6
                                end
                                local.get 7
                                local.get 3
                                local.get 0
                                select
                                local.set 3
                                local.get 4
                                local.get 9
                                local.get 0
                                select
                                local.set 9
                                local.get 6
                                local.set 4
                                local.get 6
                                br_if 0 (;@13;)
                              end
                            end
                            local.get 9
                            i32.eqz
                            br_if 0 (;@11;)
                            local.get 3
                            i32.const 0
                            i32.load offset=1048588
                            local.get 5
                            i32.sub
                            i32.ge_u
                            br_if 0 (;@11;)
                            local.get 9
                            i32.load offset=24
                            local.set 2
                            block ;; label = @12
                              local.get 9
                              i32.load offset=12
                              local.tee 0
                              local.get 9
                              i32.eq
                              br_if 0 (;@12;)
                              local.get 9
                              i32.load offset=8
                              local.tee 4
                              i32.const 0
                              i32.load offset=1048596
                              i32.lt_u
                              drop
                              local.get 0
                              local.get 4
                              i32.store offset=8
                              local.get 4
                              local.get 0
                              i32.store offset=12
                              br 9 (;@3;)
                            end
                            block ;; label = @12
                              local.get 9
                              i32.const 20
                              i32.add
                              local.tee 6
                              i32.load
                              local.tee 4
                              br_if 0 (;@12;)
                              local.get 9
                              i32.load offset=16
                              local.tee 4
                              i32.eqz
                              br_if 3 (;@9;)
                              local.get 9
                              i32.const 16
                              i32.add
                              local.set 6
                            end
                            loop ;; label = @12
                              local.get 6
                              local.set 7
                              local.get 4
                              local.tee 0
                              i32.const 20
                              i32.add
                              local.tee 6
                              i32.load
                              local.tee 4
                              br_if 0 (;@12;)
                              local.get 0
                              i32.const 16
                              i32.add
                              local.set 6
                              local.get 0
                              i32.load offset=16
                              local.tee 4
                              br_if 0 (;@12;)
                            end
                            local.get 7
                            i32.const 0
                            i32.store
                            br 8 (;@3;)
                          end
                          block ;; label = @11
                            i32.const 0
                            i32.load offset=1048588
                            local.tee 4
                            local.get 5
                            i32.lt_u
                            br_if 0 (;@11;)
                            i32.const 0
                            i32.load offset=1048600
                            local.set 3
                            block ;; label = @12
                              block ;; label = @13
                                local.get 4
                                local.get 5
                                i32.sub
                                local.tee 6
                                i32.const 16
                                i32.lt_u
                                br_if 0 (;@13;)
                                local.get 3
                                local.get 5
                                i32.add
                                local.tee 0
                                local.get 6
                                i32.const 1
                                i32.or
                                i32.store offset=4
                                local.get 3
                                local.get 4
                                i32.add
                                local.get 6
                                i32.store
                                local.get 3
                                local.get 5
                                i32.const 3
                                i32.or
                                i32.store offset=4
                                br 1 (;@12;)
                              end
                              local.get 3
                              local.get 4
                              i32.const 3
                              i32.or
                              i32.store offset=4
                              local.get 3
                              local.get 4
                              i32.add
                              local.tee 4
                              local.get 4
                              i32.load offset=4
                              i32.const 1
                              i32.or
                              i32.store offset=4
                              i32.const 0
                              local.set 0
                              i32.const 0
                              local.set 6
                            end
                            i32.const 0
                            local.get 6
                            i32.store offset=1048588
                            i32.const 0
                            local.get 0
                            i32.store offset=1048600
                            local.get 3
                            i32.const 8
                            i32.add
                            local.set 4
                            br 10 (;@1;)
                          end
                          block ;; label = @11
                            i32.const 0
                            i32.load offset=1048592
                            local.tee 6
                            local.get 5
                            i32.le_u
                            br_if 0 (;@11;)
                            local.get 2
                            local.get 5
                            i32.add
                            local.tee 4
                            local.get 6
                            local.get 5
                            i32.sub
                            local.tee 3
                            i32.const 1
                            i32.or
                            i32.store offset=4
                            i32.const 0
                            local.get 4
                            i32.store offset=1048604
                            i32.const 0
                            local.get 3
                            i32.store offset=1048592
                            local.get 2
                            local.get 5
                            i32.const 3
                            i32.or
                            i32.store offset=4
                            local.get 2
                            i32.const 8
                            i32.add
                            local.set 4
                            br 10 (;@1;)
                          end
                          block ;; label = @11
                            block ;; label = @12
                              i32.const 0
                              i32.load offset=1049052
                              i32.eqz
                              br_if 0 (;@12;)
                              i32.const 0
                              i32.load offset=1049060
                              local.set 3
                              br 1 (;@11;)
                            end
                            i32.const 0
                            i64.const -1
                            i64.store offset=1049064 align=4
                            i32.const 0
                            i64.const 281474976776192
                            i64.store offset=1049056 align=4
                            i32.const 0
                            local.get 1
                            i32.const 12
                            i32.add
                            i32.const -16
                            i32.and
                            i32.const 1431655768
                            i32.xor
                            i32.store offset=1049052
                            i32.const 0
                            i32.const 0
                            i32.store offset=1049072
                            i32.const 0
                            i32.const 0
                            i32.store offset=1049024
                            i32.const 65536
                            local.set 3
                          end
                          i32.const 0
                          local.set 4
                          block ;; label = @11
                            local.get 3
                            local.get 5
                            i32.const 71
                            i32.add
                            local.tee 8
                            i32.add
                            local.tee 0
                            i32.const 0
                            local.get 3
                            i32.sub
                            local.tee 7
                            i32.and
                            local.tee 9
                            local.get 5
                            i32.gt_u
                            br_if 0 (;@11;)
                            i32.const 0
                            i32.const 48
                            i32.store offset=1049076
                            br 10 (;@1;)
                          end
                          block ;; label = @11
                            i32.const 0
                            i32.load offset=1049020
                            local.tee 4
                            i32.eqz
                            br_if 0 (;@11;)
                            block ;; label = @12
                              i32.const 0
                              i32.load offset=1049012
                              local.tee 3
                              local.get 9
                              i32.add
                              local.tee 10
                              local.get 3
                              i32.le_u
                              br_if 0 (;@12;)
                              local.get 10
                              local.get 4
                              i32.le_u
                              br_if 1 (;@11;)
                            end
                            i32.const 0
                            local.set 4
                            i32.const 0
                            i32.const 48
                            i32.store offset=1049076
                            br 10 (;@1;)
                          end
                          i32.const 0
                          i32.load8_u offset=1049024
                          i32.const 4
                          i32.and
                          br_if 4 (;@6;)
                          block ;; label = @11
                            block ;; label = @12
                              block ;; label = @13
                                local.get 2
                                i32.eqz
                                br_if 0 (;@13;)
                                i32.const 1049028
                                local.set 4
                                loop ;; label = @14
                                  block ;; label = @15
                                    local.get 4
                                    i32.load
                                    local.tee 3
                                    local.get 2
                                    i32.gt_u
                                    br_if 0 (;@15;)
                                    local.get 3
                                    local.get 4
                                    i32.load offset=4
                                    i32.add
                                    local.get 2
                                    i32.gt_u
                                    br_if 3 (;@12;)
                                  end
                                  local.get 4
                                  i32.load offset=8
                                  local.tee 4
                                  br_if 0 (;@14;)
                                end
                              end
                              i32.const 0
                              call $sbrk
                              local.tee 0
                              i32.const -1
                              i32.eq
                              br_if 5 (;@7;)
                              local.get 9
                              local.set 7
                              block ;; label = @13
                                i32.const 0
                                i32.load offset=1049056
                                local.tee 4
                                i32.const -1
                                i32.add
                                local.tee 3
                                local.get 0
                                i32.and
                                i32.eqz
                                br_if 0 (;@13;)
                                local.get 9
                                local.get 0
                                i32.sub
                                local.get 3
                                local.get 0
                                i32.add
                                i32.const 0
                                local.get 4
                                i32.sub
                                i32.and
                                i32.add
                                local.set 7
                              end
                              local.get 7
                              local.get 5
                              i32.le_u
                              br_if 5 (;@7;)
                              local.get 7
                              i32.const 2147483646
                              i32.gt_u
                              br_if 5 (;@7;)
                              block ;; label = @13
                                i32.const 0
                                i32.load offset=1049020
                                local.tee 4
                                i32.eqz
                                br_if 0 (;@13;)
                                i32.const 0
                                i32.load offset=1049012
                                local.tee 3
                                local.get 7
                                i32.add
                                local.tee 6
                                local.get 3
                                i32.le_u
                                br_if 6 (;@7;)
                                local.get 6
                                local.get 4
                                i32.gt_u
                                br_if 6 (;@7;)
                              end
                              local.get 7
                              call $sbrk
                              local.tee 4
                              local.get 0
                              i32.ne
                              br_if 1 (;@11;)
                              br 7 (;@5;)
                            end
                            local.get 0
                            local.get 6
                            i32.sub
                            local.get 7
                            i32.and
                            local.tee 7
                            i32.const 2147483646
                            i32.gt_u
                            br_if 4 (;@7;)
                            local.get 7
                            call $sbrk
                            local.tee 0
                            local.get 4
                            i32.load
                            local.get 4
                            i32.load offset=4
                            i32.add
                            i32.eq
                            br_if 3 (;@8;)
                            local.get 0
                            local.set 4
                          end
                          block ;; label = @11
                            local.get 4
                            i32.const -1
                            i32.eq
                            br_if 0 (;@11;)
                            local.get 5
                            i32.const 72
                            i32.add
                            local.get 7
                            i32.le_u
                            br_if 0 (;@11;)
                            block ;; label = @12
                              local.get 8
                              local.get 7
                              i32.sub
                              i32.const 0
                              i32.load offset=1049060
                              local.tee 3
                              i32.add
                              i32.const 0
                              local.get 3
                              i32.sub
                              i32.and
                              local.tee 3
                              i32.const 2147483646
                              i32.le_u
                              br_if 0 (;@12;)
                              local.get 4
                              local.set 0
                              br 7 (;@5;)
                            end
                            block ;; label = @12
                              local.get 3
                              call $sbrk
                              i32.const -1
                              i32.eq
                              br_if 0 (;@12;)
                              local.get 3
                              local.get 7
                              i32.add
                              local.set 7
                              local.get 4
                              local.set 0
                              br 7 (;@5;)
                            end
                            i32.const 0
                            local.get 7
                            i32.sub
                            call $sbrk
                            drop
                            br 4 (;@7;)
                          end
                          local.get 4
                          local.set 0
                          local.get 4
                          i32.const -1
                          i32.ne
                          br_if 5 (;@5;)
                          br 3 (;@7;)
                        end
                        i32.const 0
                        local.set 9
                        br 7 (;@2;)
                      end
                      i32.const 0
                      local.set 0
                      br 5 (;@3;)
                    end
                    local.get 0
                    i32.const -1
                    i32.ne
                    br_if 2 (;@5;)
                  end
                  i32.const 0
                  i32.const 0
                  i32.load offset=1049024
                  i32.const 4
                  i32.or
                  i32.store offset=1049024
                end
                local.get 9
                i32.const 2147483646
                i32.gt_u
                br_if 1 (;@4;)
                local.get 9
                call $sbrk
                local.set 0
                i32.const 0
                call $sbrk
                local.set 4
                local.get 0
                i32.const -1
                i32.eq
                br_if 1 (;@4;)
                local.get 4
                i32.const -1
                i32.eq
                br_if 1 (;@4;)
                local.get 0
                local.get 4
                i32.ge_u
                br_if 1 (;@4;)
                local.get 4
                local.get 0
                i32.sub
                local.tee 7
                local.get 5
                i32.const 56
                i32.add
                i32.le_u
                br_if 1 (;@4;)
              end
              i32.const 0
              i32.const 0
              i32.load offset=1049012
              local.get 7
              i32.add
              local.tee 4
              i32.store offset=1049012
              block ;; label = @5
                local.get 4
                i32.const 0
                i32.load offset=1049016
                i32.le_u
                br_if 0 (;@5;)
                i32.const 0
                local.get 4
                i32.store offset=1049016
              end
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    block ;; label = @8
                      i32.const 0
                      i32.load offset=1048604
                      local.tee 3
                      i32.eqz
                      br_if 0 (;@8;)
                      i32.const 1049028
                      local.set 4
                      loop ;; label = @9
                        local.get 0
                        local.get 4
                        i32.load
                        local.tee 6
                        local.get 4
                        i32.load offset=4
                        local.tee 9
                        i32.add
                        i32.eq
                        br_if 2 (;@7;)
                        local.get 4
                        i32.load offset=8
                        local.tee 4
                        br_if 0 (;@9;)
                        br 3 (;@6;)
                      end
                    end
                    block ;; label = @8
                      block ;; label = @9
                        i32.const 0
                        i32.load offset=1048596
                        local.tee 4
                        i32.eqz
                        br_if 0 (;@9;)
                        local.get 0
                        local.get 4
                        i32.ge_u
                        br_if 1 (;@8;)
                      end
                      i32.const 0
                      local.get 0
                      i32.store offset=1048596
                    end
                    i32.const 0
                    local.set 4
                    i32.const 0
                    local.get 7
                    i32.store offset=1049032
                    i32.const 0
                    local.get 0
                    i32.store offset=1049028
                    i32.const 0
                    i32.const -1
                    i32.store offset=1048612
                    i32.const 0
                    i32.const 0
                    i32.load offset=1049052
                    i32.store offset=1048616
                    i32.const 0
                    i32.const 0
                    i32.store offset=1049040
                    loop ;; label = @8
                      local.get 4
                      i32.const 1048640
                      i32.add
                      local.get 4
                      i32.const 1048628
                      i32.add
                      local.tee 3
                      i32.store
                      local.get 3
                      local.get 4
                      i32.const 1048620
                      i32.add
                      local.tee 6
                      i32.store
                      local.get 4
                      i32.const 1048632
                      i32.add
                      local.get 6
                      i32.store
                      local.get 4
                      i32.const 1048648
                      i32.add
                      local.get 4
                      i32.const 1048636
                      i32.add
                      local.tee 6
                      i32.store
                      local.get 6
                      local.get 3
                      i32.store
                      local.get 4
                      i32.const 1048656
                      i32.add
                      local.get 4
                      i32.const 1048644
                      i32.add
                      local.tee 3
                      i32.store
                      local.get 3
                      local.get 6
                      i32.store
                      local.get 4
                      i32.const 1048652
                      i32.add
                      local.get 3
                      i32.store
                      local.get 4
                      i32.const 32
                      i32.add
                      local.tee 4
                      i32.const 256
                      i32.ne
                      br_if 0 (;@8;)
                    end
                    local.get 0
                    i32.const -8
                    local.get 0
                    i32.sub
                    i32.const 15
                    i32.and
                    i32.const 0
                    local.get 0
                    i32.const 8
                    i32.add
                    i32.const 15
                    i32.and
                    select
                    local.tee 4
                    i32.add
                    local.tee 3
                    local.get 7
                    i32.const -56
                    i32.add
                    local.tee 6
                    local.get 4
                    i32.sub
                    local.tee 4
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    i32.const 0
                    i32.const 0
                    i32.load offset=1049068
                    i32.store offset=1048608
                    i32.const 0
                    local.get 4
                    i32.store offset=1048592
                    i32.const 0
                    local.get 3
                    i32.store offset=1048604
                    local.get 0
                    local.get 6
                    i32.add
                    i32.const 56
                    i32.store offset=4
                    br 2 (;@5;)
                  end
                  local.get 4
                  i32.load8_u offset=12
                  i32.const 8
                  i32.and
                  br_if 0 (;@6;)
                  local.get 3
                  local.get 6
                  i32.lt_u
                  br_if 0 (;@6;)
                  local.get 3
                  local.get 0
                  i32.ge_u
                  br_if 0 (;@6;)
                  local.get 3
                  i32.const -8
                  local.get 3
                  i32.sub
                  i32.const 15
                  i32.and
                  i32.const 0
                  local.get 3
                  i32.const 8
                  i32.add
                  i32.const 15
                  i32.and
                  select
                  local.tee 6
                  i32.add
                  local.tee 0
                  i32.const 0
                  i32.load offset=1048592
                  local.get 7
                  i32.add
                  local.tee 2
                  local.get 6
                  i32.sub
                  local.tee 6
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 4
                  local.get 9
                  local.get 7
                  i32.add
                  i32.store offset=4
                  i32.const 0
                  i32.const 0
                  i32.load offset=1049068
                  i32.store offset=1048608
                  i32.const 0
                  local.get 6
                  i32.store offset=1048592
                  i32.const 0
                  local.get 0
                  i32.store offset=1048604
                  local.get 3
                  local.get 2
                  i32.add
                  i32.const 56
                  i32.store offset=4
                  br 1 (;@5;)
                end
                block ;; label = @6
                  local.get 0
                  i32.const 0
                  i32.load offset=1048596
                  local.tee 9
                  i32.ge_u
                  br_if 0 (;@6;)
                  i32.const 0
                  local.get 0
                  i32.store offset=1048596
                  local.get 0
                  local.set 9
                end
                local.get 0
                local.get 7
                i32.add
                local.set 6
                i32.const 1049028
                local.set 4
                block ;; label = @6
                  block ;; label = @7
                    block ;; label = @8
                      block ;; label = @9
                        block ;; label = @10
                          block ;; label = @11
                            block ;; label = @12
                              loop ;; label = @13
                                local.get 4
                                i32.load
                                local.get 6
                                i32.eq
                                br_if 1 (;@12;)
                                local.get 4
                                i32.load offset=8
                                local.tee 4
                                br_if 0 (;@13;)
                                br 2 (;@11;)
                              end
                            end
                            local.get 4
                            i32.load8_u offset=12
                            i32.const 8
                            i32.and
                            i32.eqz
                            br_if 1 (;@10;)
                          end
                          i32.const 1049028
                          local.set 4
                          loop ;; label = @11
                            block ;; label = @12
                              local.get 4
                              i32.load
                              local.tee 6
                              local.get 3
                              i32.gt_u
                              br_if 0 (;@12;)
                              local.get 6
                              local.get 4
                              i32.load offset=4
                              i32.add
                              local.tee 6
                              local.get 3
                              i32.gt_u
                              br_if 3 (;@9;)
                            end
                            local.get 4
                            i32.load offset=8
                            local.set 4
                            br 0 (;@11;)
                          end
                        end
                        local.get 4
                        local.get 0
                        i32.store
                        local.get 4
                        local.get 4
                        i32.load offset=4
                        local.get 7
                        i32.add
                        i32.store offset=4
                        local.get 0
                        i32.const -8
                        local.get 0
                        i32.sub
                        i32.const 15
                        i32.and
                        i32.const 0
                        local.get 0
                        i32.const 8
                        i32.add
                        i32.const 15
                        i32.and
                        select
                        i32.add
                        local.tee 2
                        local.get 5
                        i32.const 3
                        i32.or
                        i32.store offset=4
                        local.get 6
                        i32.const -8
                        local.get 6
                        i32.sub
                        i32.const 15
                        i32.and
                        i32.const 0
                        local.get 6
                        i32.const 8
                        i32.add
                        i32.const 15
                        i32.and
                        select
                        i32.add
                        local.tee 7
                        local.get 2
                        local.get 5
                        i32.add
                        local.tee 5
                        i32.sub
                        local.set 4
                        block ;; label = @10
                          local.get 7
                          local.get 3
                          i32.ne
                          br_if 0 (;@10;)
                          i32.const 0
                          local.get 5
                          i32.store offset=1048604
                          i32.const 0
                          i32.const 0
                          i32.load offset=1048592
                          local.get 4
                          i32.add
                          local.tee 4
                          i32.store offset=1048592
                          local.get 5
                          local.get 4
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          br 3 (;@7;)
                        end
                        block ;; label = @10
                          local.get 7
                          i32.const 0
                          i32.load offset=1048600
                          i32.ne
                          br_if 0 (;@10;)
                          i32.const 0
                          local.get 5
                          i32.store offset=1048600
                          i32.const 0
                          i32.const 0
                          i32.load offset=1048588
                          local.get 4
                          i32.add
                          local.tee 4
                          i32.store offset=1048588
                          local.get 5
                          local.get 4
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          local.get 5
                          local.get 4
                          i32.add
                          local.get 4
                          i32.store
                          br 3 (;@7;)
                        end
                        block ;; label = @10
                          local.get 7
                          i32.load offset=4
                          local.tee 3
                          i32.const 3
                          i32.and
                          i32.const 1
                          i32.ne
                          br_if 0 (;@10;)
                          local.get 3
                          i32.const -8
                          i32.and
                          local.set 8
                          block ;; label = @11
                            block ;; label = @12
                              local.get 3
                              i32.const 255
                              i32.gt_u
                              br_if 0 (;@12;)
                              local.get 7
                              i32.load offset=8
                              local.tee 6
                              local.get 3
                              i32.const 3
                              i32.shr_u
                              local.tee 9
                              i32.const 3
                              i32.shl
                              i32.const 1048620
                              i32.add
                              local.tee 0
                              i32.eq
                              drop
                              block ;; label = @13
                                local.get 7
                                i32.load offset=12
                                local.tee 3
                                local.get 6
                                i32.ne
                                br_if 0 (;@13;)
                                i32.const 0
                                i32.const 0
                                i32.load offset=1048580
                                i32.const -2
                                local.get 9
                                i32.rotl
                                i32.and
                                i32.store offset=1048580
                                br 2 (;@11;)
                              end
                              local.get 3
                              local.get 0
                              i32.eq
                              drop
                              local.get 3
                              local.get 6
                              i32.store offset=8
                              local.get 6
                              local.get 3
                              i32.store offset=12
                              br 1 (;@11;)
                            end
                            local.get 7
                            i32.load offset=24
                            local.set 10
                            block ;; label = @12
                              block ;; label = @13
                                local.get 7
                                i32.load offset=12
                                local.tee 0
                                local.get 7
                                i32.eq
                                br_if 0 (;@13;)
                                local.get 7
                                i32.load offset=8
                                local.tee 3
                                local.get 9
                                i32.lt_u
                                drop
                                local.get 0
                                local.get 3
                                i32.store offset=8
                                local.get 3
                                local.get 0
                                i32.store offset=12
                                br 1 (;@12;)
                              end
                              block ;; label = @13
                                local.get 7
                                i32.const 20
                                i32.add
                                local.tee 3
                                i32.load
                                local.tee 6
                                br_if 0 (;@13;)
                                local.get 7
                                i32.const 16
                                i32.add
                                local.tee 3
                                i32.load
                                local.tee 6
                                br_if 0 (;@13;)
                                i32.const 0
                                local.set 0
                                br 1 (;@12;)
                              end
                              loop ;; label = @13
                                local.get 3
                                local.set 9
                                local.get 6
                                local.tee 0
                                i32.const 20
                                i32.add
                                local.tee 3
                                i32.load
                                local.tee 6
                                br_if 0 (;@13;)
                                local.get 0
                                i32.const 16
                                i32.add
                                local.set 3
                                local.get 0
                                i32.load offset=16
                                local.tee 6
                                br_if 0 (;@13;)
                              end
                              local.get 9
                              i32.const 0
                              i32.store
                            end
                            local.get 10
                            i32.eqz
                            br_if 0 (;@11;)
                            block ;; label = @12
                              block ;; label = @13
                                local.get 7
                                local.get 7
                                i32.load offset=28
                                local.tee 6
                                i32.const 2
                                i32.shl
                                i32.const 1048884
                                i32.add
                                local.tee 3
                                i32.load
                                i32.ne
                                br_if 0 (;@13;)
                                local.get 3
                                local.get 0
                                i32.store
                                local.get 0
                                br_if 1 (;@12;)
                                i32.const 0
                                i32.const 0
                                i32.load offset=1048584
                                i32.const -2
                                local.get 6
                                i32.rotl
                                i32.and
                                i32.store offset=1048584
                                br 2 (;@11;)
                              end
                              local.get 10
                              i32.const 16
                              i32.const 20
                              local.get 10
                              i32.load offset=16
                              local.get 7
                              i32.eq
                              select
                              i32.add
                              local.get 0
                              i32.store
                              local.get 0
                              i32.eqz
                              br_if 1 (;@11;)
                            end
                            local.get 0
                            local.get 10
                            i32.store offset=24
                            block ;; label = @12
                              local.get 7
                              i32.load offset=16
                              local.tee 3
                              i32.eqz
                              br_if 0 (;@12;)
                              local.get 0
                              local.get 3
                              i32.store offset=16
                              local.get 3
                              local.get 0
                              i32.store offset=24
                            end
                            local.get 7
                            i32.load offset=20
                            local.tee 3
                            i32.eqz
                            br_if 0 (;@11;)
                            local.get 0
                            i32.const 20
                            i32.add
                            local.get 3
                            i32.store
                            local.get 3
                            local.get 0
                            i32.store offset=24
                          end
                          local.get 8
                          local.get 4
                          i32.add
                          local.set 4
                          local.get 7
                          local.get 8
                          i32.add
                          local.tee 7
                          i32.load offset=4
                          local.set 3
                        end
                        local.get 7
                        local.get 3
                        i32.const -2
                        i32.and
                        i32.store offset=4
                        local.get 5
                        local.get 4
                        i32.add
                        local.get 4
                        i32.store
                        local.get 5
                        local.get 4
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        block ;; label = @10
                          local.get 4
                          i32.const 255
                          i32.gt_u
                          br_if 0 (;@10;)
                          local.get 4
                          i32.const -8
                          i32.and
                          i32.const 1048620
                          i32.add
                          local.set 3
                          block ;; label = @11
                            block ;; label = @12
                              i32.const 0
                              i32.load offset=1048580
                              local.tee 6
                              i32.const 1
                              local.get 4
                              i32.const 3
                              i32.shr_u
                              i32.shl
                              local.tee 4
                              i32.and
                              br_if 0 (;@12;)
                              i32.const 0
                              local.get 6
                              local.get 4
                              i32.or
                              i32.store offset=1048580
                              local.get 3
                              local.set 4
                              br 1 (;@11;)
                            end
                            local.get 3
                            i32.load offset=8
                            local.set 4
                          end
                          local.get 4
                          local.get 5
                          i32.store offset=12
                          local.get 3
                          local.get 5
                          i32.store offset=8
                          local.get 5
                          local.get 3
                          i32.store offset=12
                          local.get 5
                          local.get 4
                          i32.store offset=8
                          br 3 (;@7;)
                        end
                        i32.const 31
                        local.set 3
                        block ;; label = @10
                          local.get 4
                          i32.const 16777215
                          i32.gt_u
                          br_if 0 (;@10;)
                          local.get 4
                          i32.const 38
                          local.get 4
                          i32.const 8
                          i32.shr_u
                          i32.clz
                          local.tee 3
                          i32.sub
                          i32.shr_u
                          i32.const 1
                          i32.and
                          local.get 3
                          i32.const 1
                          i32.shl
                          i32.sub
                          i32.const 62
                          i32.add
                          local.set 3
                        end
                        local.get 5
                        local.get 3
                        i32.store offset=28
                        local.get 5
                        i64.const 0
                        i64.store offset=16 align=4
                        local.get 3
                        i32.const 2
                        i32.shl
                        i32.const 1048884
                        i32.add
                        local.set 6
                        block ;; label = @10
                          i32.const 0
                          i32.load offset=1048584
                          local.tee 0
                          i32.const 1
                          local.get 3
                          i32.shl
                          local.tee 9
                          i32.and
                          br_if 0 (;@10;)
                          local.get 6
                          local.get 5
                          i32.store
                          i32.const 0
                          local.get 0
                          local.get 9
                          i32.or
                          i32.store offset=1048584
                          local.get 5
                          local.get 6
                          i32.store offset=24
                          local.get 5
                          local.get 5
                          i32.store offset=8
                          local.get 5
                          local.get 5
                          i32.store offset=12
                          br 3 (;@7;)
                        end
                        local.get 4
                        i32.const 0
                        i32.const 25
                        local.get 3
                        i32.const 1
                        i32.shr_u
                        i32.sub
                        local.get 3
                        i32.const 31
                        i32.eq
                        select
                        i32.shl
                        local.set 3
                        local.get 6
                        i32.load
                        local.set 0
                        loop ;; label = @10
                          local.get 0
                          local.tee 6
                          i32.load offset=4
                          i32.const -8
                          i32.and
                          local.get 4
                          i32.eq
                          br_if 2 (;@8;)
                          local.get 3
                          i32.const 29
                          i32.shr_u
                          local.set 0
                          local.get 3
                          i32.const 1
                          i32.shl
                          local.set 3
                          local.get 6
                          local.get 0
                          i32.const 4
                          i32.and
                          i32.add
                          i32.const 16
                          i32.add
                          local.tee 9
                          i32.load
                          local.tee 0
                          br_if 0 (;@10;)
                        end
                        local.get 9
                        local.get 5
                        i32.store
                        local.get 5
                        local.get 6
                        i32.store offset=24
                        local.get 5
                        local.get 5
                        i32.store offset=12
                        local.get 5
                        local.get 5
                        i32.store offset=8
                        br 2 (;@7;)
                      end
                      local.get 0
                      i32.const -8
                      local.get 0
                      i32.sub
                      i32.const 15
                      i32.and
                      i32.const 0
                      local.get 0
                      i32.const 8
                      i32.add
                      i32.const 15
                      i32.and
                      select
                      local.tee 4
                      i32.add
                      local.tee 2
                      local.get 7
                      i32.const -56
                      i32.add
                      local.tee 9
                      local.get 4
                      i32.sub
                      local.tee 4
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 0
                      local.get 9
                      i32.add
                      i32.const 56
                      i32.store offset=4
                      local.get 3
                      local.get 6
                      i32.const 55
                      local.get 6
                      i32.sub
                      i32.const 15
                      i32.and
                      i32.const 0
                      local.get 6
                      i32.const -55
                      i32.add
                      i32.const 15
                      i32.and
                      select
                      i32.add
                      i32.const -63
                      i32.add
                      local.tee 9
                      local.get 9
                      local.get 3
                      i32.const 16
                      i32.add
                      i32.lt_u
                      select
                      local.tee 9
                      i32.const 35
                      i32.store offset=4
                      i32.const 0
                      i32.const 0
                      i32.load offset=1049068
                      i32.store offset=1048608
                      i32.const 0
                      local.get 4
                      i32.store offset=1048592
                      i32.const 0
                      local.get 2
                      i32.store offset=1048604
                      local.get 9
                      i32.const 16
                      i32.add
                      i32.const 0
                      i64.load offset=1049036 align=4
                      i64.store align=4
                      local.get 9
                      i32.const 0
                      i64.load offset=1049028 align=4
                      i64.store offset=8 align=4
                      i32.const 0
                      local.get 9
                      i32.const 8
                      i32.add
                      i32.store offset=1049036
                      i32.const 0
                      local.get 7
                      i32.store offset=1049032
                      i32.const 0
                      local.get 0
                      i32.store offset=1049028
                      i32.const 0
                      i32.const 0
                      i32.store offset=1049040
                      local.get 9
                      i32.const 36
                      i32.add
                      local.set 4
                      loop ;; label = @9
                        local.get 4
                        i32.const 7
                        i32.store
                        local.get 4
                        i32.const 4
                        i32.add
                        local.tee 4
                        local.get 6
                        i32.lt_u
                        br_if 0 (;@9;)
                      end
                      local.get 9
                      local.get 3
                      i32.eq
                      br_if 3 (;@5;)
                      local.get 9
                      local.get 9
                      i32.load offset=4
                      i32.const -2
                      i32.and
                      i32.store offset=4
                      local.get 9
                      local.get 9
                      local.get 3
                      i32.sub
                      local.tee 0
                      i32.store
                      local.get 3
                      local.get 0
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      block ;; label = @9
                        local.get 0
                        i32.const 255
                        i32.gt_u
                        br_if 0 (;@9;)
                        local.get 0
                        i32.const -8
                        i32.and
                        i32.const 1048620
                        i32.add
                        local.set 4
                        block ;; label = @10
                          block ;; label = @11
                            i32.const 0
                            i32.load offset=1048580
                            local.tee 6
                            i32.const 1
                            local.get 0
                            i32.const 3
                            i32.shr_u
                            i32.shl
                            local.tee 0
                            i32.and
                            br_if 0 (;@11;)
                            i32.const 0
                            local.get 6
                            local.get 0
                            i32.or
                            i32.store offset=1048580
                            local.get 4
                            local.set 6
                            br 1 (;@10;)
                          end
                          local.get 4
                          i32.load offset=8
                          local.set 6
                        end
                        local.get 6
                        local.get 3
                        i32.store offset=12
                        local.get 4
                        local.get 3
                        i32.store offset=8
                        local.get 3
                        local.get 4
                        i32.store offset=12
                        local.get 3
                        local.get 6
                        i32.store offset=8
                        br 4 (;@5;)
                      end
                      i32.const 31
                      local.set 4
                      block ;; label = @9
                        local.get 0
                        i32.const 16777215
                        i32.gt_u
                        br_if 0 (;@9;)
                        local.get 0
                        i32.const 38
                        local.get 0
                        i32.const 8
                        i32.shr_u
                        i32.clz
                        local.tee 4
                        i32.sub
                        i32.shr_u
                        i32.const 1
                        i32.and
                        local.get 4
                        i32.const 1
                        i32.shl
                        i32.sub
                        i32.const 62
                        i32.add
                        local.set 4
                      end
                      local.get 3
                      local.get 4
                      i32.store offset=28
                      local.get 3
                      i64.const 0
                      i64.store offset=16 align=4
                      local.get 4
                      i32.const 2
                      i32.shl
                      i32.const 1048884
                      i32.add
                      local.set 6
                      block ;; label = @9
                        i32.const 0
                        i32.load offset=1048584
                        local.tee 9
                        i32.const 1
                        local.get 4
                        i32.shl
                        local.tee 7
                        i32.and
                        br_if 0 (;@9;)
                        local.get 6
                        local.get 3
                        i32.store
                        i32.const 0
                        local.get 9
                        local.get 7
                        i32.or
                        i32.store offset=1048584
                        local.get 3
                        local.get 6
                        i32.store offset=24
                        local.get 3
                        local.get 3
                        i32.store offset=8
                        local.get 3
                        local.get 3
                        i32.store offset=12
                        br 4 (;@5;)
                      end
                      local.get 0
                      i32.const 0
                      i32.const 25
                      local.get 4
                      i32.const 1
                      i32.shr_u
                      i32.sub
                      local.get 4
                      i32.const 31
                      i32.eq
                      select
                      i32.shl
                      local.set 4
                      local.get 6
                      i32.load
                      local.set 9
                      loop ;; label = @9
                        local.get 9
                        local.tee 6
                        i32.load offset=4
                        i32.const -8
                        i32.and
                        local.get 0
                        i32.eq
                        br_if 3 (;@6;)
                        local.get 4
                        i32.const 29
                        i32.shr_u
                        local.set 9
                        local.get 4
                        i32.const 1
                        i32.shl
                        local.set 4
                        local.get 6
                        local.get 9
                        i32.const 4
                        i32.and
                        i32.add
                        i32.const 16
                        i32.add
                        local.tee 7
                        i32.load
                        local.tee 9
                        br_if 0 (;@9;)
                      end
                      local.get 7
                      local.get 3
                      i32.store
                      local.get 3
                      local.get 6
                      i32.store offset=24
                      local.get 3
                      local.get 3
                      i32.store offset=12
                      local.get 3
                      local.get 3
                      i32.store offset=8
                      br 3 (;@5;)
                    end
                    local.get 6
                    i32.load offset=8
                    local.tee 4
                    local.get 5
                    i32.store offset=12
                    local.get 6
                    local.get 5
                    i32.store offset=8
                    local.get 5
                    i32.const 0
                    i32.store offset=24
                    local.get 5
                    local.get 6
                    i32.store offset=12
                    local.get 5
                    local.get 4
                    i32.store offset=8
                  end
                  local.get 2
                  i32.const 8
                  i32.add
                  local.set 4
                  br 5 (;@1;)
                end
                local.get 6
                i32.load offset=8
                local.tee 4
                local.get 3
                i32.store offset=12
                local.get 6
                local.get 3
                i32.store offset=8
                local.get 3
                i32.const 0
                i32.store offset=24
                local.get 3
                local.get 6
                i32.store offset=12
                local.get 3
                local.get 4
                i32.store offset=8
              end
              i32.const 0
              i32.load offset=1048592
              local.tee 4
              local.get 5
              i32.le_u
              br_if 0 (;@4;)
              i32.const 0
              i32.load offset=1048604
              local.tee 3
              local.get 5
              i32.add
              local.tee 6
              local.get 4
              local.get 5
              i32.sub
              local.tee 4
              i32.const 1
              i32.or
              i32.store offset=4
              i32.const 0
              local.get 4
              i32.store offset=1048592
              i32.const 0
              local.get 6
              i32.store offset=1048604
              local.get 3
              local.get 5
              i32.const 3
              i32.or
              i32.store offset=4
              local.get 3
              i32.const 8
              i32.add
              local.set 4
              br 3 (;@1;)
            end
            i32.const 0
            local.set 4
            i32.const 0
            i32.const 48
            i32.store offset=1049076
            br 2 (;@1;)
          end
          block ;; label = @3
            local.get 2
            i32.eqz
            br_if 0 (;@3;)
            block ;; label = @4
              block ;; label = @5
                local.get 9
                local.get 9
                i32.load offset=28
                local.tee 6
                i32.const 2
                i32.shl
                i32.const 1048884
                i32.add
                local.tee 4
                i32.load
                i32.ne
                br_if 0 (;@5;)
                local.get 4
                local.get 0
                i32.store
                local.get 0
                br_if 1 (;@4;)
                i32.const 0
                local.get 10
                i32.const -2
                local.get 6
                i32.rotl
                i32.and
                local.tee 10
                i32.store offset=1048584
                br 2 (;@3;)
              end
              local.get 2
              i32.const 16
              i32.const 20
              local.get 2
              i32.load offset=16
              local.get 9
              i32.eq
              select
              i32.add
              local.get 0
              i32.store
              local.get 0
              i32.eqz
              br_if 1 (;@3;)
            end
            local.get 0
            local.get 2
            i32.store offset=24
            block ;; label = @4
              local.get 9
              i32.load offset=16
              local.tee 4
              i32.eqz
              br_if 0 (;@4;)
              local.get 0
              local.get 4
              i32.store offset=16
              local.get 4
              local.get 0
              i32.store offset=24
            end
            local.get 9
            i32.const 20
            i32.add
            i32.load
            local.tee 4
            i32.eqz
            br_if 0 (;@3;)
            local.get 0
            i32.const 20
            i32.add
            local.get 4
            i32.store
            local.get 4
            local.get 0
            i32.store offset=24
          end
          block ;; label = @3
            block ;; label = @4
              local.get 3
              i32.const 15
              i32.gt_u
              br_if 0 (;@4;)
              local.get 9
              local.get 3
              local.get 5
              i32.add
              local.tee 4
              i32.const 3
              i32.or
              i32.store offset=4
              local.get 9
              local.get 4
              i32.add
              local.tee 4
              local.get 4
              i32.load offset=4
              i32.const 1
              i32.or
              i32.store offset=4
              br 1 (;@3;)
            end
            local.get 9
            local.get 5
            i32.add
            local.tee 0
            local.get 3
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 9
            local.get 5
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 0
            local.get 3
            i32.add
            local.get 3
            i32.store
            block ;; label = @4
              local.get 3
              i32.const 255
              i32.gt_u
              br_if 0 (;@4;)
              local.get 3
              i32.const -8
              i32.and
              i32.const 1048620
              i32.add
              local.set 4
              block ;; label = @5
                block ;; label = @6
                  i32.const 0
                  i32.load offset=1048580
                  local.tee 6
                  i32.const 1
                  local.get 3
                  i32.const 3
                  i32.shr_u
                  i32.shl
                  local.tee 3
                  i32.and
                  br_if 0 (;@6;)
                  i32.const 0
                  local.get 6
                  local.get 3
                  i32.or
                  i32.store offset=1048580
                  local.get 4
                  local.set 3
                  br 1 (;@5;)
                end
                local.get 4
                i32.load offset=8
                local.set 3
              end
              local.get 3
              local.get 0
              i32.store offset=12
              local.get 4
              local.get 0
              i32.store offset=8
              local.get 0
              local.get 4
              i32.store offset=12
              local.get 0
              local.get 3
              i32.store offset=8
              br 1 (;@3;)
            end
            i32.const 31
            local.set 4
            block ;; label = @4
              local.get 3
              i32.const 16777215
              i32.gt_u
              br_if 0 (;@4;)
              local.get 3
              i32.const 38
              local.get 3
              i32.const 8
              i32.shr_u
              i32.clz
              local.tee 4
              i32.sub
              i32.shr_u
              i32.const 1
              i32.and
              local.get 4
              i32.const 1
              i32.shl
              i32.sub
              i32.const 62
              i32.add
              local.set 4
            end
            local.get 0
            local.get 4
            i32.store offset=28
            local.get 0
            i64.const 0
            i64.store offset=16 align=4
            local.get 4
            i32.const 2
            i32.shl
            i32.const 1048884
            i32.add
            local.set 6
            block ;; label = @4
              local.get 10
              i32.const 1
              local.get 4
              i32.shl
              local.tee 5
              i32.and
              br_if 0 (;@4;)
              local.get 6
              local.get 0
              i32.store
              i32.const 0
              local.get 10
              local.get 5
              i32.or
              i32.store offset=1048584
              local.get 0
              local.get 6
              i32.store offset=24
              local.get 0
              local.get 0
              i32.store offset=8
              local.get 0
              local.get 0
              i32.store offset=12
              br 1 (;@3;)
            end
            local.get 3
            i32.const 0
            i32.const 25
            local.get 4
            i32.const 1
            i32.shr_u
            i32.sub
            local.get 4
            i32.const 31
            i32.eq
            select
            i32.shl
            local.set 4
            local.get 6
            i32.load
            local.set 5
            block ;; label = @4
              loop ;; label = @5
                local.get 5
                local.tee 6
                i32.load offset=4
                i32.const -8
                i32.and
                local.get 3
                i32.eq
                br_if 1 (;@4;)
                local.get 4
                i32.const 29
                i32.shr_u
                local.set 5
                local.get 4
                i32.const 1
                i32.shl
                local.set 4
                local.get 6
                local.get 5
                i32.const 4
                i32.and
                i32.add
                i32.const 16
                i32.add
                local.tee 7
                i32.load
                local.tee 5
                br_if 0 (;@5;)
              end
              local.get 7
              local.get 0
              i32.store
              local.get 0
              local.get 6
              i32.store offset=24
              local.get 0
              local.get 0
              i32.store offset=12
              local.get 0
              local.get 0
              i32.store offset=8
              br 1 (;@3;)
            end
            local.get 6
            i32.load offset=8
            local.tee 4
            local.get 0
            i32.store offset=12
            local.get 6
            local.get 0
            i32.store offset=8
            local.get 0
            i32.const 0
            i32.store offset=24
            local.get 0
            local.get 6
            i32.store offset=12
            local.get 0
            local.get 4
            i32.store offset=8
          end
          local.get 9
          i32.const 8
          i32.add
          local.set 4
          br 1 (;@1;)
        end
        block ;; label = @2
          local.get 11
          i32.eqz
          br_if 0 (;@2;)
          block ;; label = @3
            block ;; label = @4
              local.get 0
              local.get 0
              i32.load offset=28
              local.tee 6
              i32.const 2
              i32.shl
              i32.const 1048884
              i32.add
              local.tee 4
              i32.load
              i32.ne
              br_if 0 (;@4;)
              local.get 4
              local.get 9
              i32.store
              local.get 9
              br_if 1 (;@3;)
              i32.const 0
              local.get 10
              i32.const -2
              local.get 6
              i32.rotl
              i32.and
              i32.store offset=1048584
              br 2 (;@2;)
            end
            local.get 11
            i32.const 16
            i32.const 20
            local.get 11
            i32.load offset=16
            local.get 0
            i32.eq
            select
            i32.add
            local.get 9
            i32.store
            local.get 9
            i32.eqz
            br_if 1 (;@2;)
          end
          local.get 9
          local.get 11
          i32.store offset=24
          block ;; label = @3
            local.get 0
            i32.load offset=16
            local.tee 4
            i32.eqz
            br_if 0 (;@3;)
            local.get 9
            local.get 4
            i32.store offset=16
            local.get 4
            local.get 9
            i32.store offset=24
          end
          local.get 0
          i32.const 20
          i32.add
          i32.load
          local.tee 4
          i32.eqz
          br_if 0 (;@2;)
          local.get 9
          i32.const 20
          i32.add
          local.get 4
          i32.store
          local.get 4
          local.get 9
          i32.store offset=24
        end
        block ;; label = @2
          block ;; label = @3
            local.get 3
            i32.const 15
            i32.gt_u
            br_if 0 (;@3;)
            local.get 0
            local.get 3
            local.get 5
            i32.add
            local.tee 4
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 0
            local.get 4
            i32.add
            local.tee 4
            local.get 4
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            br 1 (;@2;)
          end
          local.get 0
          local.get 5
          i32.add
          local.tee 6
          local.get 3
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 0
          local.get 5
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 6
          local.get 3
          i32.add
          local.get 3
          i32.store
          block ;; label = @3
            local.get 8
            i32.eqz
            br_if 0 (;@3;)
            local.get 8
            i32.const -8
            i32.and
            i32.const 1048620
            i32.add
            local.set 5
            i32.const 0
            i32.load offset=1048600
            local.set 4
            block ;; label = @4
              block ;; label = @5
                i32.const 1
                local.get 8
                i32.const 3
                i32.shr_u
                i32.shl
                local.tee 9
                local.get 7
                i32.and
                br_if 0 (;@5;)
                i32.const 0
                local.get 9
                local.get 7
                i32.or
                i32.store offset=1048580
                local.get 5
                local.set 9
                br 1 (;@4;)
              end
              local.get 5
              i32.load offset=8
              local.set 9
            end
            local.get 9
            local.get 4
            i32.store offset=12
            local.get 5
            local.get 4
            i32.store offset=8
            local.get 4
            local.get 5
            i32.store offset=12
            local.get 4
            local.get 9
            i32.store offset=8
          end
          i32.const 0
          local.get 6
          i32.store offset=1048600
          i32.const 0
          local.get 3
          i32.store offset=1048588
        end
        local.get 0
        i32.const 8
        i32.add
        local.set 4
      end
      local.get 1
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 4
    )
    (func $free (;14;) (type 7) (param i32)
      local.get 0
      call $dlfree
    )
    (func $dlfree (;15;) (type 7) (param i32)
      (local i32 i32 i32 i32 i32 i32 i32)
      block ;; label = @1
        local.get 0
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.const -8
        i32.add
        local.tee 1
        local.get 0
        i32.const -4
        i32.add
        i32.load
        local.tee 2
        i32.const -8
        i32.and
        local.tee 0
        i32.add
        local.set 3
        block ;; label = @2
          local.get 2
          i32.const 1
          i32.and
          br_if 0 (;@2;)
          local.get 2
          i32.const 3
          i32.and
          i32.eqz
          br_if 1 (;@1;)
          local.get 1
          local.get 1
          i32.load
          local.tee 2
          i32.sub
          local.tee 1
          i32.const 0
          i32.load offset=1048596
          local.tee 4
          i32.lt_u
          br_if 1 (;@1;)
          local.get 2
          local.get 0
          i32.add
          local.set 0
          block ;; label = @3
            local.get 1
            i32.const 0
            i32.load offset=1048600
            i32.eq
            br_if 0 (;@3;)
            block ;; label = @4
              local.get 2
              i32.const 255
              i32.gt_u
              br_if 0 (;@4;)
              local.get 1
              i32.load offset=8
              local.tee 4
              local.get 2
              i32.const 3
              i32.shr_u
              local.tee 5
              i32.const 3
              i32.shl
              i32.const 1048620
              i32.add
              local.tee 6
              i32.eq
              drop
              block ;; label = @5
                local.get 1
                i32.load offset=12
                local.tee 2
                local.get 4
                i32.ne
                br_if 0 (;@5;)
                i32.const 0
                i32.const 0
                i32.load offset=1048580
                i32.const -2
                local.get 5
                i32.rotl
                i32.and
                i32.store offset=1048580
                br 3 (;@2;)
              end
              local.get 2
              local.get 6
              i32.eq
              drop
              local.get 2
              local.get 4
              i32.store offset=8
              local.get 4
              local.get 2
              i32.store offset=12
              br 2 (;@2;)
            end
            local.get 1
            i32.load offset=24
            local.set 7
            block ;; label = @4
              block ;; label = @5
                local.get 1
                i32.load offset=12
                local.tee 6
                local.get 1
                i32.eq
                br_if 0 (;@5;)
                local.get 1
                i32.load offset=8
                local.tee 2
                local.get 4
                i32.lt_u
                drop
                local.get 6
                local.get 2
                i32.store offset=8
                local.get 2
                local.get 6
                i32.store offset=12
                br 1 (;@4;)
              end
              block ;; label = @5
                local.get 1
                i32.const 20
                i32.add
                local.tee 2
                i32.load
                local.tee 4
                br_if 0 (;@5;)
                local.get 1
                i32.const 16
                i32.add
                local.tee 2
                i32.load
                local.tee 4
                br_if 0 (;@5;)
                i32.const 0
                local.set 6
                br 1 (;@4;)
              end
              loop ;; label = @5
                local.get 2
                local.set 5
                local.get 4
                local.tee 6
                i32.const 20
                i32.add
                local.tee 2
                i32.load
                local.tee 4
                br_if 0 (;@5;)
                local.get 6
                i32.const 16
                i32.add
                local.set 2
                local.get 6
                i32.load offset=16
                local.tee 4
                br_if 0 (;@5;)
              end
              local.get 5
              i32.const 0
              i32.store
            end
            local.get 7
            i32.eqz
            br_if 1 (;@2;)
            block ;; label = @4
              block ;; label = @5
                local.get 1
                local.get 1
                i32.load offset=28
                local.tee 4
                i32.const 2
                i32.shl
                i32.const 1048884
                i32.add
                local.tee 2
                i32.load
                i32.ne
                br_if 0 (;@5;)
                local.get 2
                local.get 6
                i32.store
                local.get 6
                br_if 1 (;@4;)
                i32.const 0
                i32.const 0
                i32.load offset=1048584
                i32.const -2
                local.get 4
                i32.rotl
                i32.and
                i32.store offset=1048584
                br 3 (;@2;)
              end
              local.get 7
              i32.const 16
              i32.const 20
              local.get 7
              i32.load offset=16
              local.get 1
              i32.eq
              select
              i32.add
              local.get 6
              i32.store
              local.get 6
              i32.eqz
              br_if 2 (;@2;)
            end
            local.get 6
            local.get 7
            i32.store offset=24
            block ;; label = @4
              local.get 1
              i32.load offset=16
              local.tee 2
              i32.eqz
              br_if 0 (;@4;)
              local.get 6
              local.get 2
              i32.store offset=16
              local.get 2
              local.get 6
              i32.store offset=24
            end
            local.get 1
            i32.load offset=20
            local.tee 2
            i32.eqz
            br_if 1 (;@2;)
            local.get 6
            i32.const 20
            i32.add
            local.get 2
            i32.store
            local.get 2
            local.get 6
            i32.store offset=24
            br 1 (;@2;)
          end
          local.get 3
          i32.load offset=4
          local.tee 2
          i32.const 3
          i32.and
          i32.const 3
          i32.ne
          br_if 0 (;@2;)
          local.get 3
          local.get 2
          i32.const -2
          i32.and
          i32.store offset=4
          i32.const 0
          local.get 0
          i32.store offset=1048588
          local.get 1
          local.get 0
          i32.add
          local.get 0
          i32.store
          local.get 1
          local.get 0
          i32.const 1
          i32.or
          i32.store offset=4
          return
        end
        local.get 1
        local.get 3
        i32.ge_u
        br_if 0 (;@1;)
        local.get 3
        i32.load offset=4
        local.tee 2
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@1;)
        block ;; label = @2
          block ;; label = @3
            local.get 2
            i32.const 2
            i32.and
            br_if 0 (;@3;)
            block ;; label = @4
              local.get 3
              i32.const 0
              i32.load offset=1048604
              i32.ne
              br_if 0 (;@4;)
              i32.const 0
              local.get 1
              i32.store offset=1048604
              i32.const 0
              i32.const 0
              i32.load offset=1048592
              local.get 0
              i32.add
              local.tee 0
              i32.store offset=1048592
              local.get 1
              local.get 0
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 1
              i32.const 0
              i32.load offset=1048600
              i32.ne
              br_if 3 (;@1;)
              i32.const 0
              i32.const 0
              i32.store offset=1048588
              i32.const 0
              i32.const 0
              i32.store offset=1048600
              return
            end
            block ;; label = @4
              local.get 3
              i32.const 0
              i32.load offset=1048600
              i32.ne
              br_if 0 (;@4;)
              i32.const 0
              local.get 1
              i32.store offset=1048600
              i32.const 0
              i32.const 0
              i32.load offset=1048588
              local.get 0
              i32.add
              local.tee 0
              i32.store offset=1048588
              local.get 1
              local.get 0
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 1
              local.get 0
              i32.add
              local.get 0
              i32.store
              return
            end
            local.get 2
            i32.const -8
            i32.and
            local.get 0
            i32.add
            local.set 0
            block ;; label = @4
              block ;; label = @5
                local.get 2
                i32.const 255
                i32.gt_u
                br_if 0 (;@5;)
                local.get 3
                i32.load offset=8
                local.tee 4
                local.get 2
                i32.const 3
                i32.shr_u
                local.tee 5
                i32.const 3
                i32.shl
                i32.const 1048620
                i32.add
                local.tee 6
                i32.eq
                drop
                block ;; label = @6
                  local.get 3
                  i32.load offset=12
                  local.tee 2
                  local.get 4
                  i32.ne
                  br_if 0 (;@6;)
                  i32.const 0
                  i32.const 0
                  i32.load offset=1048580
                  i32.const -2
                  local.get 5
                  i32.rotl
                  i32.and
                  i32.store offset=1048580
                  br 2 (;@4;)
                end
                local.get 2
                local.get 6
                i32.eq
                drop
                local.get 2
                local.get 4
                i32.store offset=8
                local.get 4
                local.get 2
                i32.store offset=12
                br 1 (;@4;)
              end
              local.get 3
              i32.load offset=24
              local.set 7
              block ;; label = @5
                block ;; label = @6
                  local.get 3
                  i32.load offset=12
                  local.tee 6
                  local.get 3
                  i32.eq
                  br_if 0 (;@6;)
                  local.get 3
                  i32.load offset=8
                  local.tee 2
                  i32.const 0
                  i32.load offset=1048596
                  i32.lt_u
                  drop
                  local.get 6
                  local.get 2
                  i32.store offset=8
                  local.get 2
                  local.get 6
                  i32.store offset=12
                  br 1 (;@5;)
                end
                block ;; label = @6
                  local.get 3
                  i32.const 20
                  i32.add
                  local.tee 2
                  i32.load
                  local.tee 4
                  br_if 0 (;@6;)
                  local.get 3
                  i32.const 16
                  i32.add
                  local.tee 2
                  i32.load
                  local.tee 4
                  br_if 0 (;@6;)
                  i32.const 0
                  local.set 6
                  br 1 (;@5;)
                end
                loop ;; label = @6
                  local.get 2
                  local.set 5
                  local.get 4
                  local.tee 6
                  i32.const 20
                  i32.add
                  local.tee 2
                  i32.load
                  local.tee 4
                  br_if 0 (;@6;)
                  local.get 6
                  i32.const 16
                  i32.add
                  local.set 2
                  local.get 6
                  i32.load offset=16
                  local.tee 4
                  br_if 0 (;@6;)
                end
                local.get 5
                i32.const 0
                i32.store
              end
              local.get 7
              i32.eqz
              br_if 0 (;@4;)
              block ;; label = @5
                block ;; label = @6
                  local.get 3
                  local.get 3
                  i32.load offset=28
                  local.tee 4
                  i32.const 2
                  i32.shl
                  i32.const 1048884
                  i32.add
                  local.tee 2
                  i32.load
                  i32.ne
                  br_if 0 (;@6;)
                  local.get 2
                  local.get 6
                  i32.store
                  local.get 6
                  br_if 1 (;@5;)
                  i32.const 0
                  i32.const 0
                  i32.load offset=1048584
                  i32.const -2
                  local.get 4
                  i32.rotl
                  i32.and
                  i32.store offset=1048584
                  br 2 (;@4;)
                end
                local.get 7
                i32.const 16
                i32.const 20
                local.get 7
                i32.load offset=16
                local.get 3
                i32.eq
                select
                i32.add
                local.get 6
                i32.store
                local.get 6
                i32.eqz
                br_if 1 (;@4;)
              end
              local.get 6
              local.get 7
              i32.store offset=24
              block ;; label = @5
                local.get 3
                i32.load offset=16
                local.tee 2
                i32.eqz
                br_if 0 (;@5;)
                local.get 6
                local.get 2
                i32.store offset=16
                local.get 2
                local.get 6
                i32.store offset=24
              end
              local.get 3
              i32.load offset=20
              local.tee 2
              i32.eqz
              br_if 0 (;@4;)
              local.get 6
              i32.const 20
              i32.add
              local.get 2
              i32.store
              local.get 2
              local.get 6
              i32.store offset=24
            end
            local.get 1
            local.get 0
            i32.add
            local.get 0
            i32.store
            local.get 1
            local.get 0
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 1
            i32.const 0
            i32.load offset=1048600
            i32.ne
            br_if 1 (;@2;)
            i32.const 0
            local.get 0
            i32.store offset=1048588
            return
          end
          local.get 3
          local.get 2
          i32.const -2
          i32.and
          i32.store offset=4
          local.get 1
          local.get 0
          i32.add
          local.get 0
          i32.store
          local.get 1
          local.get 0
          i32.const 1
          i32.or
          i32.store offset=4
        end
        block ;; label = @2
          local.get 0
          i32.const 255
          i32.gt_u
          br_if 0 (;@2;)
          local.get 0
          i32.const -8
          i32.and
          i32.const 1048620
          i32.add
          local.set 2
          block ;; label = @3
            block ;; label = @4
              i32.const 0
              i32.load offset=1048580
              local.tee 4
              i32.const 1
              local.get 0
              i32.const 3
              i32.shr_u
              i32.shl
              local.tee 0
              i32.and
              br_if 0 (;@4;)
              i32.const 0
              local.get 4
              local.get 0
              i32.or
              i32.store offset=1048580
              local.get 2
              local.set 0
              br 1 (;@3;)
            end
            local.get 2
            i32.load offset=8
            local.set 0
          end
          local.get 0
          local.get 1
          i32.store offset=12
          local.get 2
          local.get 1
          i32.store offset=8
          local.get 1
          local.get 2
          i32.store offset=12
          local.get 1
          local.get 0
          i32.store offset=8
          return
        end
        i32.const 31
        local.set 2
        block ;; label = @2
          local.get 0
          i32.const 16777215
          i32.gt_u
          br_if 0 (;@2;)
          local.get 0
          i32.const 38
          local.get 0
          i32.const 8
          i32.shr_u
          i32.clz
          local.tee 2
          i32.sub
          i32.shr_u
          i32.const 1
          i32.and
          local.get 2
          i32.const 1
          i32.shl
          i32.sub
          i32.const 62
          i32.add
          local.set 2
        end
        local.get 1
        local.get 2
        i32.store offset=28
        local.get 1
        i64.const 0
        i64.store offset=16 align=4
        local.get 2
        i32.const 2
        i32.shl
        i32.const 1048884
        i32.add
        local.set 4
        block ;; label = @2
          block ;; label = @3
            i32.const 0
            i32.load offset=1048584
            local.tee 6
            i32.const 1
            local.get 2
            i32.shl
            local.tee 3
            i32.and
            br_if 0 (;@3;)
            local.get 4
            local.get 1
            i32.store
            i32.const 0
            local.get 6
            local.get 3
            i32.or
            i32.store offset=1048584
            local.get 1
            local.get 4
            i32.store offset=24
            local.get 1
            local.get 1
            i32.store offset=8
            local.get 1
            local.get 1
            i32.store offset=12
            br 1 (;@2;)
          end
          local.get 0
          i32.const 0
          i32.const 25
          local.get 2
          i32.const 1
          i32.shr_u
          i32.sub
          local.get 2
          i32.const 31
          i32.eq
          select
          i32.shl
          local.set 2
          local.get 4
          i32.load
          local.set 6
          block ;; label = @3
            loop ;; label = @4
              local.get 6
              local.tee 4
              i32.load offset=4
              i32.const -8
              i32.and
              local.get 0
              i32.eq
              br_if 1 (;@3;)
              local.get 2
              i32.const 29
              i32.shr_u
              local.set 6
              local.get 2
              i32.const 1
              i32.shl
              local.set 2
              local.get 4
              local.get 6
              i32.const 4
              i32.and
              i32.add
              i32.const 16
              i32.add
              local.tee 3
              i32.load
              local.tee 6
              br_if 0 (;@4;)
            end
            local.get 3
            local.get 1
            i32.store
            local.get 1
            local.get 4
            i32.store offset=24
            local.get 1
            local.get 1
            i32.store offset=12
            local.get 1
            local.get 1
            i32.store offset=8
            br 1 (;@2;)
          end
          local.get 4
          i32.load offset=8
          local.tee 0
          local.get 1
          i32.store offset=12
          local.get 4
          local.get 1
          i32.store offset=8
          local.get 1
          i32.const 0
          i32.store offset=24
          local.get 1
          local.get 4
          i32.store offset=12
          local.get 1
          local.get 0
          i32.store offset=8
        end
        i32.const 0
        i32.const 0
        i32.load offset=1048612
        i32.const -1
        i32.add
        local.tee 1
        i32.const -1
        local.get 1
        select
        i32.store offset=1048612
      end
    )
    (func $realloc (;16;) (type 4) (param i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
      block ;; label = @1
        local.get 0
        br_if 0 (;@1;)
        local.get 1
        call $dlmalloc
        return
      end
      block ;; label = @1
        local.get 1
        i32.const -64
        i32.lt_u
        br_if 0 (;@1;)
        i32.const 0
        i32.const 48
        i32.store offset=1049076
        i32.const 0
        return
      end
      i32.const 16
      local.get 1
      i32.const 19
      i32.add
      i32.const -16
      i32.and
      local.get 1
      i32.const 11
      i32.lt_u
      select
      local.set 2
      local.get 0
      i32.const -4
      i32.add
      local.tee 3
      i32.load
      local.tee 4
      i32.const -8
      i32.and
      local.set 5
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 4
            i32.const 3
            i32.and
            br_if 0 (;@3;)
            local.get 2
            i32.const 256
            i32.lt_u
            br_if 1 (;@2;)
            local.get 5
            local.get 2
            i32.const 4
            i32.or
            i32.lt_u
            br_if 1 (;@2;)
            local.get 5
            local.get 2
            i32.sub
            i32.const 0
            i32.load offset=1049060
            i32.const 1
            i32.shl
            i32.le_u
            br_if 2 (;@1;)
            br 1 (;@2;)
          end
          local.get 0
          i32.const -8
          i32.add
          local.tee 6
          local.get 5
          i32.add
          local.set 7
          block ;; label = @3
            local.get 5
            local.get 2
            i32.lt_u
            br_if 0 (;@3;)
            local.get 5
            local.get 2
            i32.sub
            local.tee 1
            i32.const 16
            i32.lt_u
            br_if 2 (;@1;)
            local.get 3
            local.get 2
            local.get 4
            i32.const 1
            i32.and
            i32.or
            i32.const 2
            i32.or
            i32.store
            local.get 6
            local.get 2
            i32.add
            local.tee 2
            local.get 1
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 7
            local.get 7
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 2
            local.get 1
            call $dispose_chunk
            local.get 0
            return
          end
          block ;; label = @3
            local.get 7
            i32.const 0
            i32.load offset=1048604
            i32.ne
            br_if 0 (;@3;)
            i32.const 0
            i32.load offset=1048592
            local.get 5
            i32.add
            local.tee 5
            local.get 2
            i32.le_u
            br_if 1 (;@2;)
            local.get 3
            local.get 2
            local.get 4
            i32.const 1
            i32.and
            i32.or
            i32.const 2
            i32.or
            i32.store
            i32.const 0
            local.get 6
            local.get 2
            i32.add
            local.tee 1
            i32.store offset=1048604
            i32.const 0
            local.get 5
            local.get 2
            i32.sub
            local.tee 2
            i32.store offset=1048592
            local.get 1
            local.get 2
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 0
            return
          end
          block ;; label = @3
            local.get 7
            i32.const 0
            i32.load offset=1048600
            i32.ne
            br_if 0 (;@3;)
            i32.const 0
            i32.load offset=1048588
            local.get 5
            i32.add
            local.tee 5
            local.get 2
            i32.lt_u
            br_if 1 (;@2;)
            block ;; label = @4
              block ;; label = @5
                local.get 5
                local.get 2
                i32.sub
                local.tee 1
                i32.const 16
                i32.lt_u
                br_if 0 (;@5;)
                local.get 3
                local.get 2
                local.get 4
                i32.const 1
                i32.and
                i32.or
                i32.const 2
                i32.or
                i32.store
                local.get 6
                local.get 2
                i32.add
                local.tee 2
                local.get 1
                i32.const 1
                i32.or
                i32.store offset=4
                local.get 6
                local.get 5
                i32.add
                local.tee 5
                local.get 1
                i32.store
                local.get 5
                local.get 5
                i32.load offset=4
                i32.const -2
                i32.and
                i32.store offset=4
                br 1 (;@4;)
              end
              local.get 3
              local.get 4
              i32.const 1
              i32.and
              local.get 5
              i32.or
              i32.const 2
              i32.or
              i32.store
              local.get 6
              local.get 5
              i32.add
              local.tee 1
              local.get 1
              i32.load offset=4
              i32.const 1
              i32.or
              i32.store offset=4
              i32.const 0
              local.set 1
              i32.const 0
              local.set 2
            end
            i32.const 0
            local.get 2
            i32.store offset=1048600
            i32.const 0
            local.get 1
            i32.store offset=1048588
            local.get 0
            return
          end
          local.get 7
          i32.load offset=4
          local.tee 8
          i32.const 2
          i32.and
          br_if 0 (;@2;)
          local.get 8
          i32.const -8
          i32.and
          local.get 5
          i32.add
          local.tee 9
          local.get 2
          i32.lt_u
          br_if 0 (;@2;)
          local.get 9
          local.get 2
          i32.sub
          local.set 10
          block ;; label = @3
            block ;; label = @4
              local.get 8
              i32.const 255
              i32.gt_u
              br_if 0 (;@4;)
              local.get 7
              i32.load offset=8
              local.tee 1
              local.get 8
              i32.const 3
              i32.shr_u
              local.tee 11
              i32.const 3
              i32.shl
              i32.const 1048620
              i32.add
              local.tee 8
              i32.eq
              drop
              block ;; label = @5
                local.get 7
                i32.load offset=12
                local.tee 5
                local.get 1
                i32.ne
                br_if 0 (;@5;)
                i32.const 0
                i32.const 0
                i32.load offset=1048580
                i32.const -2
                local.get 11
                i32.rotl
                i32.and
                i32.store offset=1048580
                br 2 (;@3;)
              end
              local.get 5
              local.get 8
              i32.eq
              drop
              local.get 5
              local.get 1
              i32.store offset=8
              local.get 1
              local.get 5
              i32.store offset=12
              br 1 (;@3;)
            end
            local.get 7
            i32.load offset=24
            local.set 12
            block ;; label = @4
              block ;; label = @5
                local.get 7
                i32.load offset=12
                local.tee 8
                local.get 7
                i32.eq
                br_if 0 (;@5;)
                local.get 7
                i32.load offset=8
                local.tee 1
                i32.const 0
                i32.load offset=1048596
                i32.lt_u
                drop
                local.get 8
                local.get 1
                i32.store offset=8
                local.get 1
                local.get 8
                i32.store offset=12
                br 1 (;@4;)
              end
              block ;; label = @5
                local.get 7
                i32.const 20
                i32.add
                local.tee 1
                i32.load
                local.tee 5
                br_if 0 (;@5;)
                local.get 7
                i32.const 16
                i32.add
                local.tee 1
                i32.load
                local.tee 5
                br_if 0 (;@5;)
                i32.const 0
                local.set 8
                br 1 (;@4;)
              end
              loop ;; label = @5
                local.get 1
                local.set 11
                local.get 5
                local.tee 8
                i32.const 20
                i32.add
                local.tee 1
                i32.load
                local.tee 5
                br_if 0 (;@5;)
                local.get 8
                i32.const 16
                i32.add
                local.set 1
                local.get 8
                i32.load offset=16
                local.tee 5
                br_if 0 (;@5;)
              end
              local.get 11
              i32.const 0
              i32.store
            end
            local.get 12
            i32.eqz
            br_if 0 (;@3;)
            block ;; label = @4
              block ;; label = @5
                local.get 7
                local.get 7
                i32.load offset=28
                local.tee 5
                i32.const 2
                i32.shl
                i32.const 1048884
                i32.add
                local.tee 1
                i32.load
                i32.ne
                br_if 0 (;@5;)
                local.get 1
                local.get 8
                i32.store
                local.get 8
                br_if 1 (;@4;)
                i32.const 0
                i32.const 0
                i32.load offset=1048584
                i32.const -2
                local.get 5
                i32.rotl
                i32.and
                i32.store offset=1048584
                br 2 (;@3;)
              end
              local.get 12
              i32.const 16
              i32.const 20
              local.get 12
              i32.load offset=16
              local.get 7
              i32.eq
              select
              i32.add
              local.get 8
              i32.store
              local.get 8
              i32.eqz
              br_if 1 (;@3;)
            end
            local.get 8
            local.get 12
            i32.store offset=24
            block ;; label = @4
              local.get 7
              i32.load offset=16
              local.tee 1
              i32.eqz
              br_if 0 (;@4;)
              local.get 8
              local.get 1
              i32.store offset=16
              local.get 1
              local.get 8
              i32.store offset=24
            end
            local.get 7
            i32.load offset=20
            local.tee 1
            i32.eqz
            br_if 0 (;@3;)
            local.get 8
            i32.const 20
            i32.add
            local.get 1
            i32.store
            local.get 1
            local.get 8
            i32.store offset=24
          end
          block ;; label = @3
            local.get 10
            i32.const 15
            i32.gt_u
            br_if 0 (;@3;)
            local.get 3
            local.get 4
            i32.const 1
            i32.and
            local.get 9
            i32.or
            i32.const 2
            i32.or
            i32.store
            local.get 6
            local.get 9
            i32.add
            local.tee 1
            local.get 1
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 0
            return
          end
          local.get 3
          local.get 2
          local.get 4
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get 6
          local.get 2
          i32.add
          local.tee 1
          local.get 10
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 6
          local.get 9
          i32.add
          local.tee 2
          local.get 2
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 1
          local.get 10
          call $dispose_chunk
          local.get 0
          return
        end
        block ;; label = @2
          local.get 1
          call $dlmalloc
          local.tee 2
          br_if 0 (;@2;)
          i32.const 0
          return
        end
        local.get 2
        local.get 0
        i32.const -4
        i32.const -8
        local.get 3
        i32.load
        local.tee 5
        i32.const 3
        i32.and
        select
        local.get 5
        i32.const -8
        i32.and
        i32.add
        local.tee 5
        local.get 1
        local.get 5
        local.get 1
        i32.lt_u
        select
        call $memcpy
        local.set 1
        local.get 0
        call $dlfree
        local.get 1
        local.set 0
      end
      local.get 0
    )
    (func $dispose_chunk (;17;) (type 8) (param i32 i32)
      (local i32 i32 i32 i32 i32 i32)
      local.get 0
      local.get 1
      i32.add
      local.set 2
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.load offset=4
          local.tee 3
          i32.const 1
          i32.and
          br_if 0 (;@2;)
          local.get 3
          i32.const 3
          i32.and
          i32.eqz
          br_if 1 (;@1;)
          local.get 0
          i32.load
          local.tee 3
          local.get 1
          i32.add
          local.set 1
          block ;; label = @3
            block ;; label = @4
              local.get 0
              local.get 3
              i32.sub
              local.tee 0
              i32.const 0
              i32.load offset=1048600
              i32.eq
              br_if 0 (;@4;)
              block ;; label = @5
                local.get 3
                i32.const 255
                i32.gt_u
                br_if 0 (;@5;)
                local.get 0
                i32.load offset=8
                local.tee 4
                local.get 3
                i32.const 3
                i32.shr_u
                local.tee 5
                i32.const 3
                i32.shl
                i32.const 1048620
                i32.add
                local.tee 6
                i32.eq
                drop
                local.get 0
                i32.load offset=12
                local.tee 3
                local.get 4
                i32.ne
                br_if 2 (;@3;)
                i32.const 0
                i32.const 0
                i32.load offset=1048580
                i32.const -2
                local.get 5
                i32.rotl
                i32.and
                i32.store offset=1048580
                br 3 (;@2;)
              end
              local.get 0
              i32.load offset=24
              local.set 7
              block ;; label = @5
                block ;; label = @6
                  local.get 0
                  i32.load offset=12
                  local.tee 6
                  local.get 0
                  i32.eq
                  br_if 0 (;@6;)
                  local.get 0
                  i32.load offset=8
                  local.tee 3
                  i32.const 0
                  i32.load offset=1048596
                  i32.lt_u
                  drop
                  local.get 6
                  local.get 3
                  i32.store offset=8
                  local.get 3
                  local.get 6
                  i32.store offset=12
                  br 1 (;@5;)
                end
                block ;; label = @6
                  local.get 0
                  i32.const 20
                  i32.add
                  local.tee 3
                  i32.load
                  local.tee 4
                  br_if 0 (;@6;)
                  local.get 0
                  i32.const 16
                  i32.add
                  local.tee 3
                  i32.load
                  local.tee 4
                  br_if 0 (;@6;)
                  i32.const 0
                  local.set 6
                  br 1 (;@5;)
                end
                loop ;; label = @6
                  local.get 3
                  local.set 5
                  local.get 4
                  local.tee 6
                  i32.const 20
                  i32.add
                  local.tee 3
                  i32.load
                  local.tee 4
                  br_if 0 (;@6;)
                  local.get 6
                  i32.const 16
                  i32.add
                  local.set 3
                  local.get 6
                  i32.load offset=16
                  local.tee 4
                  br_if 0 (;@6;)
                end
                local.get 5
                i32.const 0
                i32.store
              end
              local.get 7
              i32.eqz
              br_if 2 (;@2;)
              block ;; label = @5
                block ;; label = @6
                  local.get 0
                  local.get 0
                  i32.load offset=28
                  local.tee 4
                  i32.const 2
                  i32.shl
                  i32.const 1048884
                  i32.add
                  local.tee 3
                  i32.load
                  i32.ne
                  br_if 0 (;@6;)
                  local.get 3
                  local.get 6
                  i32.store
                  local.get 6
                  br_if 1 (;@5;)
                  i32.const 0
                  i32.const 0
                  i32.load offset=1048584
                  i32.const -2
                  local.get 4
                  i32.rotl
                  i32.and
                  i32.store offset=1048584
                  br 4 (;@2;)
                end
                local.get 7
                i32.const 16
                i32.const 20
                local.get 7
                i32.load offset=16
                local.get 0
                i32.eq
                select
                i32.add
                local.get 6
                i32.store
                local.get 6
                i32.eqz
                br_if 3 (;@2;)
              end
              local.get 6
              local.get 7
              i32.store offset=24
              block ;; label = @5
                local.get 0
                i32.load offset=16
                local.tee 3
                i32.eqz
                br_if 0 (;@5;)
                local.get 6
                local.get 3
                i32.store offset=16
                local.get 3
                local.get 6
                i32.store offset=24
              end
              local.get 0
              i32.load offset=20
              local.tee 3
              i32.eqz
              br_if 2 (;@2;)
              local.get 6
              i32.const 20
              i32.add
              local.get 3
              i32.store
              local.get 3
              local.get 6
              i32.store offset=24
              br 2 (;@2;)
            end
            local.get 2
            i32.load offset=4
            local.tee 3
            i32.const 3
            i32.and
            i32.const 3
            i32.ne
            br_if 1 (;@2;)
            local.get 2
            local.get 3
            i32.const -2
            i32.and
            i32.store offset=4
            i32.const 0
            local.get 1
            i32.store offset=1048588
            local.get 2
            local.get 1
            i32.store
            local.get 0
            local.get 1
            i32.const 1
            i32.or
            i32.store offset=4
            return
          end
          local.get 3
          local.get 6
          i32.eq
          drop
          local.get 3
          local.get 4
          i32.store offset=8
          local.get 4
          local.get 3
          i32.store offset=12
        end
        block ;; label = @2
          block ;; label = @3
            local.get 2
            i32.load offset=4
            local.tee 3
            i32.const 2
            i32.and
            br_if 0 (;@3;)
            block ;; label = @4
              local.get 2
              i32.const 0
              i32.load offset=1048604
              i32.ne
              br_if 0 (;@4;)
              i32.const 0
              local.get 0
              i32.store offset=1048604
              i32.const 0
              i32.const 0
              i32.load offset=1048592
              local.get 1
              i32.add
              local.tee 1
              i32.store offset=1048592
              local.get 0
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              i32.const 0
              i32.load offset=1048600
              i32.ne
              br_if 3 (;@1;)
              i32.const 0
              i32.const 0
              i32.store offset=1048588
              i32.const 0
              i32.const 0
              i32.store offset=1048600
              return
            end
            block ;; label = @4
              local.get 2
              i32.const 0
              i32.load offset=1048600
              i32.ne
              br_if 0 (;@4;)
              i32.const 0
              local.get 0
              i32.store offset=1048600
              i32.const 0
              i32.const 0
              i32.load offset=1048588
              local.get 1
              i32.add
              local.tee 1
              i32.store offset=1048588
              local.get 0
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              local.get 1
              i32.add
              local.get 1
              i32.store
              return
            end
            local.get 3
            i32.const -8
            i32.and
            local.get 1
            i32.add
            local.set 1
            block ;; label = @4
              block ;; label = @5
                local.get 3
                i32.const 255
                i32.gt_u
                br_if 0 (;@5;)
                local.get 2
                i32.load offset=8
                local.tee 4
                local.get 3
                i32.const 3
                i32.shr_u
                local.tee 5
                i32.const 3
                i32.shl
                i32.const 1048620
                i32.add
                local.tee 6
                i32.eq
                drop
                block ;; label = @6
                  local.get 2
                  i32.load offset=12
                  local.tee 3
                  local.get 4
                  i32.ne
                  br_if 0 (;@6;)
                  i32.const 0
                  i32.const 0
                  i32.load offset=1048580
                  i32.const -2
                  local.get 5
                  i32.rotl
                  i32.and
                  i32.store offset=1048580
                  br 2 (;@4;)
                end
                local.get 3
                local.get 6
                i32.eq
                drop
                local.get 3
                local.get 4
                i32.store offset=8
                local.get 4
                local.get 3
                i32.store offset=12
                br 1 (;@4;)
              end
              local.get 2
              i32.load offset=24
              local.set 7
              block ;; label = @5
                block ;; label = @6
                  local.get 2
                  i32.load offset=12
                  local.tee 6
                  local.get 2
                  i32.eq
                  br_if 0 (;@6;)
                  local.get 2
                  i32.load offset=8
                  local.tee 3
                  i32.const 0
                  i32.load offset=1048596
                  i32.lt_u
                  drop
                  local.get 6
                  local.get 3
                  i32.store offset=8
                  local.get 3
                  local.get 6
                  i32.store offset=12
                  br 1 (;@5;)
                end
                block ;; label = @6
                  local.get 2
                  i32.const 20
                  i32.add
                  local.tee 4
                  i32.load
                  local.tee 3
                  br_if 0 (;@6;)
                  local.get 2
                  i32.const 16
                  i32.add
                  local.tee 4
                  i32.load
                  local.tee 3
                  br_if 0 (;@6;)
                  i32.const 0
                  local.set 6
                  br 1 (;@5;)
                end
                loop ;; label = @6
                  local.get 4
                  local.set 5
                  local.get 3
                  local.tee 6
                  i32.const 20
                  i32.add
                  local.tee 4
                  i32.load
                  local.tee 3
                  br_if 0 (;@6;)
                  local.get 6
                  i32.const 16
                  i32.add
                  local.set 4
                  local.get 6
                  i32.load offset=16
                  local.tee 3
                  br_if 0 (;@6;)
                end
                local.get 5
                i32.const 0
                i32.store
              end
              local.get 7
              i32.eqz
              br_if 0 (;@4;)
              block ;; label = @5
                block ;; label = @6
                  local.get 2
                  local.get 2
                  i32.load offset=28
                  local.tee 4
                  i32.const 2
                  i32.shl
                  i32.const 1048884
                  i32.add
                  local.tee 3
                  i32.load
                  i32.ne
                  br_if 0 (;@6;)
                  local.get 3
                  local.get 6
                  i32.store
                  local.get 6
                  br_if 1 (;@5;)
                  i32.const 0
                  i32.const 0
                  i32.load offset=1048584
                  i32.const -2
                  local.get 4
                  i32.rotl
                  i32.and
                  i32.store offset=1048584
                  br 2 (;@4;)
                end
                local.get 7
                i32.const 16
                i32.const 20
                local.get 7
                i32.load offset=16
                local.get 2
                i32.eq
                select
                i32.add
                local.get 6
                i32.store
                local.get 6
                i32.eqz
                br_if 1 (;@4;)
              end
              local.get 6
              local.get 7
              i32.store offset=24
              block ;; label = @5
                local.get 2
                i32.load offset=16
                local.tee 3
                i32.eqz
                br_if 0 (;@5;)
                local.get 6
                local.get 3
                i32.store offset=16
                local.get 3
                local.get 6
                i32.store offset=24
              end
              local.get 2
              i32.load offset=20
              local.tee 3
              i32.eqz
              br_if 0 (;@4;)
              local.get 6
              i32.const 20
              i32.add
              local.get 3
              i32.store
              local.get 3
              local.get 6
              i32.store offset=24
            end
            local.get 0
            local.get 1
            i32.add
            local.get 1
            i32.store
            local.get 0
            local.get 1
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 0
            i32.const 0
            i32.load offset=1048600
            i32.ne
            br_if 1 (;@2;)
            i32.const 0
            local.get 1
            i32.store offset=1048588
            return
          end
          local.get 2
          local.get 3
          i32.const -2
          i32.and
          i32.store offset=4
          local.get 0
          local.get 1
          i32.add
          local.get 1
          i32.store
          local.get 0
          local.get 1
          i32.const 1
          i32.or
          i32.store offset=4
        end
        block ;; label = @2
          local.get 1
          i32.const 255
          i32.gt_u
          br_if 0 (;@2;)
          local.get 1
          i32.const -8
          i32.and
          i32.const 1048620
          i32.add
          local.set 3
          block ;; label = @3
            block ;; label = @4
              i32.const 0
              i32.load offset=1048580
              local.tee 4
              i32.const 1
              local.get 1
              i32.const 3
              i32.shr_u
              i32.shl
              local.tee 1
              i32.and
              br_if 0 (;@4;)
              i32.const 0
              local.get 4
              local.get 1
              i32.or
              i32.store offset=1048580
              local.get 3
              local.set 1
              br 1 (;@3;)
            end
            local.get 3
            i32.load offset=8
            local.set 1
          end
          local.get 1
          local.get 0
          i32.store offset=12
          local.get 3
          local.get 0
          i32.store offset=8
          local.get 0
          local.get 3
          i32.store offset=12
          local.get 0
          local.get 1
          i32.store offset=8
          return
        end
        i32.const 31
        local.set 3
        block ;; label = @2
          local.get 1
          i32.const 16777215
          i32.gt_u
          br_if 0 (;@2;)
          local.get 1
          i32.const 38
          local.get 1
          i32.const 8
          i32.shr_u
          i32.clz
          local.tee 3
          i32.sub
          i32.shr_u
          i32.const 1
          i32.and
          local.get 3
          i32.const 1
          i32.shl
          i32.sub
          i32.const 62
          i32.add
          local.set 3
        end
        local.get 0
        local.get 3
        i32.store offset=28
        local.get 0
        i64.const 0
        i64.store offset=16 align=4
        local.get 3
        i32.const 2
        i32.shl
        i32.const 1048884
        i32.add
        local.set 4
        block ;; label = @2
          i32.const 0
          i32.load offset=1048584
          local.tee 6
          i32.const 1
          local.get 3
          i32.shl
          local.tee 2
          i32.and
          br_if 0 (;@2;)
          local.get 4
          local.get 0
          i32.store
          i32.const 0
          local.get 6
          local.get 2
          i32.or
          i32.store offset=1048584
          local.get 0
          local.get 4
          i32.store offset=24
          local.get 0
          local.get 0
          i32.store offset=8
          local.get 0
          local.get 0
          i32.store offset=12
          return
        end
        local.get 1
        i32.const 0
        i32.const 25
        local.get 3
        i32.const 1
        i32.shr_u
        i32.sub
        local.get 3
        i32.const 31
        i32.eq
        select
        i32.shl
        local.set 3
        local.get 4
        i32.load
        local.set 6
        block ;; label = @2
          loop ;; label = @3
            local.get 6
            local.tee 4
            i32.load offset=4
            i32.const -8
            i32.and
            local.get 1
            i32.eq
            br_if 1 (;@2;)
            local.get 3
            i32.const 29
            i32.shr_u
            local.set 6
            local.get 3
            i32.const 1
            i32.shl
            local.set 3
            local.get 4
            local.get 6
            i32.const 4
            i32.and
            i32.add
            i32.const 16
            i32.add
            local.tee 2
            i32.load
            local.tee 6
            br_if 0 (;@3;)
          end
          local.get 2
          local.get 0
          i32.store
          local.get 0
          local.get 4
          i32.store offset=24
          local.get 0
          local.get 0
          i32.store offset=12
          local.get 0
          local.get 0
          i32.store offset=8
          return
        end
        local.get 4
        i32.load offset=8
        local.tee 1
        local.get 0
        i32.store offset=12
        local.get 4
        local.get 0
        i32.store offset=8
        local.get 0
        i32.const 0
        i32.store offset=24
        local.get 0
        local.get 4
        i32.store offset=12
        local.get 0
        local.get 1
        i32.store offset=8
      end
    )
    (func $internal_memalign (;18;) (type 4) (param i32 i32) (result i32)
      (local i32 i32 i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.const 16
          local.get 0
          i32.const 16
          i32.gt_u
          select
          local.tee 2
          local.get 2
          i32.const -1
          i32.add
          i32.and
          br_if 0 (;@2;)
          local.get 2
          local.set 0
          br 1 (;@1;)
        end
        i32.const 32
        local.set 3
        loop ;; label = @2
          local.get 3
          local.tee 0
          i32.const 1
          i32.shl
          local.set 3
          local.get 0
          local.get 2
          i32.lt_u
          br_if 0 (;@2;)
        end
      end
      block ;; label = @1
        i32.const -64
        local.get 0
        i32.sub
        local.get 1
        i32.gt_u
        br_if 0 (;@1;)
        i32.const 0
        i32.const 48
        i32.store offset=1049076
        i32.const 0
        return
      end
      block ;; label = @1
        local.get 0
        i32.const 16
        local.get 1
        i32.const 19
        i32.add
        i32.const -16
        i32.and
        local.get 1
        i32.const 11
        i32.lt_u
        select
        local.tee 1
        i32.add
        i32.const 12
        i32.add
        call $dlmalloc
        local.tee 3
        br_if 0 (;@1;)
        i32.const 0
        return
      end
      local.get 3
      i32.const -8
      i32.add
      local.set 2
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.const -1
          i32.add
          local.get 3
          i32.and
          br_if 0 (;@2;)
          local.get 2
          local.set 0
          br 1 (;@1;)
        end
        local.get 3
        i32.const -4
        i32.add
        local.tee 4
        i32.load
        local.tee 5
        i32.const -8
        i32.and
        local.get 3
        local.get 0
        i32.add
        i32.const -1
        i32.add
        i32.const 0
        local.get 0
        i32.sub
        i32.and
        i32.const -8
        i32.add
        local.tee 3
        i32.const 0
        local.get 0
        local.get 3
        local.get 2
        i32.sub
        i32.const 15
        i32.gt_u
        select
        i32.add
        local.tee 0
        local.get 2
        i32.sub
        local.tee 3
        i32.sub
        local.set 6
        block ;; label = @2
          local.get 5
          i32.const 3
          i32.and
          br_if 0 (;@2;)
          local.get 0
          local.get 6
          i32.store offset=4
          local.get 0
          local.get 2
          i32.load
          local.get 3
          i32.add
          i32.store
          br 1 (;@1;)
        end
        local.get 0
        local.get 6
        local.get 0
        i32.load offset=4
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store offset=4
        local.get 0
        local.get 6
        i32.add
        local.tee 6
        local.get 6
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 4
        local.get 3
        local.get 4
        i32.load
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store
        local.get 2
        local.get 3
        i32.add
        local.tee 6
        local.get 6
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 2
        local.get 3
        call $dispose_chunk
      end
      block ;; label = @1
        local.get 0
        i32.load offset=4
        local.tee 3
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@1;)
        local.get 3
        i32.const -8
        i32.and
        local.tee 2
        local.get 1
        i32.const 16
        i32.add
        i32.le_u
        br_if 0 (;@1;)
        local.get 0
        local.get 1
        local.get 3
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store offset=4
        local.get 0
        local.get 1
        i32.add
        local.tee 3
        local.get 2
        local.get 1
        i32.sub
        local.tee 1
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 0
        local.get 2
        i32.add
        local.tee 2
        local.get 2
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 3
        local.get 1
        call $dispose_chunk
      end
      local.get 0
      i32.const 8
      i32.add
    )
    (func $aligned_alloc (;19;) (type 4) (param i32 i32) (result i32)
      block ;; label = @1
        local.get 0
        i32.const 16
        i32.gt_u
        br_if 0 (;@1;)
        local.get 1
        call $dlmalloc
        return
      end
      local.get 0
      local.get 1
      call $internal_memalign
    )
    (func $abort (;20;) (type 2)
      unreachable
      unreachable
    )
    (func $sbrk (;21;) (type 6) (param i32) (result i32)
      block ;; label = @1
        local.get 0
        br_if 0 (;@1;)
        memory.size
        i32.const 16
        i32.shl
        return
      end
      block ;; label = @1
        local.get 0
        i32.const 65535
        i32.and
        br_if 0 (;@1;)
        local.get 0
        i32.const -1
        i32.le_s
        br_if 0 (;@1;)
        block ;; label = @2
          local.get 0
          i32.const 16
          i32.shr_u
          memory.grow
          local.tee 0
          i32.const -1
          i32.ne
          br_if 0 (;@2;)
          i32.const 0
          i32.const 48
          i32.store offset=1049076
          i32.const -1
          return
        end
        local.get 0
        i32.const 16
        i32.shl
        return
      end
      call $abort
      unreachable
    )
    (func $memcpy (;22;) (type 9) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 2
            i32.const 32
            i32.gt_u
            br_if 0 (;@3;)
            local.get 1
            i32.const 3
            i32.and
            i32.eqz
            br_if 1 (;@2;)
            local.get 2
            i32.eqz
            br_if 1 (;@2;)
            local.get 0
            local.get 1
            i32.load8_u
            i32.store8
            local.get 2
            i32.const -1
            i32.add
            local.set 3
            local.get 0
            i32.const 1
            i32.add
            local.set 4
            local.get 1
            i32.const 1
            i32.add
            local.tee 5
            i32.const 3
            i32.and
            i32.eqz
            br_if 2 (;@1;)
            local.get 3
            i32.eqz
            br_if 2 (;@1;)
            local.get 0
            local.get 1
            i32.load8_u offset=1
            i32.store8 offset=1
            local.get 2
            i32.const -2
            i32.add
            local.set 3
            local.get 0
            i32.const 2
            i32.add
            local.set 4
            local.get 1
            i32.const 2
            i32.add
            local.tee 5
            i32.const 3
            i32.and
            i32.eqz
            br_if 2 (;@1;)
            local.get 3
            i32.eqz
            br_if 2 (;@1;)
            local.get 0
            local.get 1
            i32.load8_u offset=2
            i32.store8 offset=2
            local.get 2
            i32.const -3
            i32.add
            local.set 3
            local.get 0
            i32.const 3
            i32.add
            local.set 4
            local.get 1
            i32.const 3
            i32.add
            local.tee 5
            i32.const 3
            i32.and
            i32.eqz
            br_if 2 (;@1;)
            local.get 3
            i32.eqz
            br_if 2 (;@1;)
            local.get 0
            local.get 1
            i32.load8_u offset=3
            i32.store8 offset=3
            local.get 2
            i32.const -4
            i32.add
            local.set 3
            local.get 0
            i32.const 4
            i32.add
            local.set 4
            local.get 1
            i32.const 4
            i32.add
            local.set 5
            br 2 (;@1;)
          end
          local.get 0
          local.get 1
          local.get 2
          memory.copy
          local.get 0
          return
        end
        local.get 2
        local.set 3
        local.get 0
        local.set 4
        local.get 1
        local.set 5
      end
      block ;; label = @1
        block ;; label = @2
          local.get 4
          i32.const 3
          i32.and
          local.tee 2
          br_if 0 (;@2;)
          block ;; label = @3
            block ;; label = @4
              local.get 3
              i32.const 16
              i32.ge_u
              br_if 0 (;@4;)
              local.get 3
              local.set 2
              br 1 (;@3;)
            end
            block ;; label = @4
              local.get 3
              i32.const -16
              i32.add
              local.tee 2
              i32.const 16
              i32.and
              br_if 0 (;@4;)
              local.get 4
              local.get 5
              i64.load align=4
              i64.store align=4
              local.get 4
              local.get 5
              i64.load offset=8 align=4
              i64.store offset=8 align=4
              local.get 4
              i32.const 16
              i32.add
              local.set 4
              local.get 5
              i32.const 16
              i32.add
              local.set 5
              local.get 2
              local.set 3
            end
            local.get 2
            i32.const 16
            i32.lt_u
            br_if 0 (;@3;)
            local.get 3
            local.set 2
            loop ;; label = @4
              local.get 4
              local.get 5
              i64.load align=4
              i64.store align=4
              local.get 4
              local.get 5
              i64.load offset=8 align=4
              i64.store offset=8 align=4
              local.get 4
              local.get 5
              i64.load offset=16 align=4
              i64.store offset=16 align=4
              local.get 4
              local.get 5
              i64.load offset=24 align=4
              i64.store offset=24 align=4
              local.get 4
              i32.const 32
              i32.add
              local.set 4
              local.get 5
              i32.const 32
              i32.add
              local.set 5
              local.get 2
              i32.const -32
              i32.add
              local.tee 2
              i32.const 15
              i32.gt_u
              br_if 0 (;@4;)
            end
          end
          block ;; label = @3
            local.get 2
            i32.const 8
            i32.lt_u
            br_if 0 (;@3;)
            local.get 4
            local.get 5
            i64.load align=4
            i64.store align=4
            local.get 5
            i32.const 8
            i32.add
            local.set 5
            local.get 4
            i32.const 8
            i32.add
            local.set 4
          end
          block ;; label = @3
            local.get 2
            i32.const 4
            i32.and
            i32.eqz
            br_if 0 (;@3;)
            local.get 4
            local.get 5
            i32.load
            i32.store
            local.get 5
            i32.const 4
            i32.add
            local.set 5
            local.get 4
            i32.const 4
            i32.add
            local.set 4
          end
          block ;; label = @3
            local.get 2
            i32.const 2
            i32.and
            i32.eqz
            br_if 0 (;@3;)
            local.get 4
            local.get 5
            i32.load16_u align=1
            i32.store16 align=1
            local.get 4
            i32.const 2
            i32.add
            local.set 4
            local.get 5
            i32.const 2
            i32.add
            local.set 5
          end
          local.get 2
          i32.const 1
          i32.and
          i32.eqz
          br_if 1 (;@1;)
          local.get 4
          local.get 5
          i32.load8_u
          i32.store8
          local.get 0
          return
        end
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  local.get 3
                  i32.const 32
                  i32.lt_u
                  br_if 0 (;@6;)
                  block ;; label = @7
                    block ;; label = @8
                      local.get 2
                      i32.const -1
                      i32.add
                      br_table 3 (;@5;) 0 (;@8;) 1 (;@7;) 7 (;@1;)
                    end
                    local.get 4
                    local.get 5
                    i32.load
                    i32.store16 align=1
                    local.get 4
                    local.get 5
                    i32.const 2
                    i32.add
                    i32.load align=2
                    i32.store offset=2
                    local.get 4
                    local.get 5
                    i32.const 6
                    i32.add
                    i64.load align=2
                    i64.store offset=6 align=4
                    local.get 4
                    i32.const 18
                    i32.add
                    local.set 2
                    local.get 5
                    i32.const 18
                    i32.add
                    local.set 1
                    i32.const 14
                    local.set 6
                    local.get 5
                    i32.const 14
                    i32.add
                    i32.load align=2
                    local.set 5
                    i32.const 14
                    local.set 3
                    br 3 (;@4;)
                  end
                  local.get 4
                  local.get 5
                  i32.load
                  i32.store8
                  local.get 4
                  local.get 5
                  i32.const 1
                  i32.add
                  i32.load align=1
                  i32.store offset=1
                  local.get 4
                  local.get 5
                  i32.const 5
                  i32.add
                  i64.load align=1
                  i64.store offset=5 align=4
                  local.get 4
                  i32.const 17
                  i32.add
                  local.set 2
                  local.get 5
                  i32.const 17
                  i32.add
                  local.set 1
                  i32.const 13
                  local.set 6
                  local.get 5
                  i32.const 13
                  i32.add
                  i32.load align=1
                  local.set 5
                  i32.const 15
                  local.set 3
                  br 2 (;@4;)
                end
                block ;; label = @6
                  block ;; label = @7
                    local.get 3
                    i32.const 16
                    i32.ge_u
                    br_if 0 (;@7;)
                    local.get 4
                    local.set 2
                    local.get 5
                    local.set 1
                    br 1 (;@6;)
                  end
                  local.get 4
                  local.get 5
                  i32.load8_u
                  i32.store8
                  local.get 4
                  local.get 5
                  i32.load offset=1 align=1
                  i32.store offset=1 align=1
                  local.get 4
                  local.get 5
                  i64.load offset=5 align=1
                  i64.store offset=5 align=1
                  local.get 4
                  local.get 5
                  i32.load16_u offset=13 align=1
                  i32.store16 offset=13 align=1
                  local.get 4
                  local.get 5
                  i32.load8_u offset=15
                  i32.store8 offset=15
                  local.get 4
                  i32.const 16
                  i32.add
                  local.set 2
                  local.get 5
                  i32.const 16
                  i32.add
                  local.set 1
                end
                local.get 3
                i32.const 8
                i32.and
                br_if 2 (;@3;)
                br 3 (;@2;)
              end
              local.get 4
              local.get 5
              i32.load
              local.tee 2
              i32.store8
              local.get 4
              local.get 2
              i32.const 16
              i32.shr_u
              i32.store8 offset=2
              local.get 4
              local.get 2
              i32.const 8
              i32.shr_u
              i32.store8 offset=1
              local.get 4
              local.get 5
              i32.const 3
              i32.add
              i32.load align=1
              i32.store offset=3
              local.get 4
              local.get 5
              i32.const 7
              i32.add
              i64.load align=1
              i64.store offset=7 align=4
              local.get 4
              i32.const 19
              i32.add
              local.set 2
              local.get 5
              i32.const 19
              i32.add
              local.set 1
              i32.const 15
              local.set 6
              local.get 5
              i32.const 15
              i32.add
              i32.load align=1
              local.set 5
              i32.const 13
              local.set 3
            end
            local.get 4
            local.get 6
            i32.add
            local.get 5
            i32.store
          end
          local.get 2
          local.get 1
          i64.load align=1
          i64.store align=1
          local.get 2
          i32.const 8
          i32.add
          local.set 2
          local.get 1
          i32.const 8
          i32.add
          local.set 1
        end
        block ;; label = @2
          local.get 3
          i32.const 4
          i32.and
          i32.eqz
          br_if 0 (;@2;)
          local.get 2
          local.get 1
          i32.load align=1
          i32.store align=1
          local.get 2
          i32.const 4
          i32.add
          local.set 2
          local.get 1
          i32.const 4
          i32.add
          local.set 1
        end
        block ;; label = @2
          local.get 3
          i32.const 2
          i32.and
          i32.eqz
          br_if 0 (;@2;)
          local.get 2
          local.get 1
          i32.load16_u align=1
          i32.store16 align=1
          local.get 2
          i32.const 2
          i32.add
          local.set 2
          local.get 1
          i32.const 2
          i32.add
          local.set 1
        end
        local.get 3
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@1;)
        local.get 2
        local.get 1
        i32.load8_u
        i32.store8
      end
      local.get 0
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