(module $miden_sdk_account_test.wasm
  (type (;0;) (func (result f64)))
  (type (;1;) (func (param i64) (result f64)))
  (type (;2;) (func (param f64 f64) (result f64)))
  (type (;3;) (func (param f64) (result i64)))
  (type (;4;) (func (param f64 f64) (result i32)))
  (type (;5;) (func (param f64) (result i32)))
  (type (;6;) (func (param f64)))
  (type (;7;) (func (param f64) (result f64)))
  (type (;8;) (func (param f64 f64)))
  (type (;9;) (func (param f64 f64 f64 f64 f64 f64 f64 f64 i32)))
  (type (;10;) (func (param f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 i32)))
  (type (;11;) (func (param f64 f64 f64 f64 f64 f64 f64 f64)))
  (type (;12;) (func (param i32) (result i32)))
  (type (;13;) (func (param f64 f64 f64 f64 i32)))
  (type (;14;) (func (param f64 f64 i32)))
  (type (;15;) (func (param f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 i32)))
  (type (;16;) (func (param i32)))
  (type (;17;) (func (param i32 i32 i32) (result i32)))
  (type (;18;) (func (param i32 i32 i32)))
  (type (;19;) (func (param i32 i32)))
  (type (;20;) (func (param i32 f64)))
  (type (;21;) (func (param i32 i32) (result i32)))
  (type (;22;) (func (param i32 i32 i32 i32)))
  (type (;23;) (func (param i32) (result f64)))
  (type (;24;) (func))
  (type (;25;) (func (param i32 i32 i32 i32 i32)))
  (import "miden:tx_kernel/account" "get_id<0x0000000000000000000000000000000000000000000000000000000000000000>" (func $miden_sdk_tx_kernel::externs::extern_account_get_id (;0;) (type 0)))
  (import "miden:prelude/intrinsics_felt" "from_u64_unchecked" (func $miden_prelude::intrinsics::felt::extern_from_u64_unchecked (;1;) (type 1)))
  (import "miden:prelude/intrinsics_felt" "add" (func $miden_prelude::intrinsics::felt::extern_add (;2;) (type 2)))
  (import "miden:prelude/intrinsics_felt" "as_u64" (func $miden_prelude::intrinsics::felt::extern_as_u64 (;3;) (type 3)))
  (import "miden:prelude/intrinsics_felt" "gt" (func $miden_prelude::intrinsics::felt::extern_gt (;4;) (type 4)))
  (import "miden:prelude/intrinsics_felt" "lt" (func $miden_prelude::intrinsics::felt::extern_lt (;5;) (type 4)))
  (import "miden:prelude/intrinsics_felt" "le" (func $miden_prelude::intrinsics::felt::extern_le (;6;) (type 4)))
  (import "miden:prelude/intrinsics_felt" "ge" (func $miden_prelude::intrinsics::felt::extern_ge (;7;) (type 4)))
  (import "miden:prelude/intrinsics_felt" "eq" (func $miden_prelude::intrinsics::felt::extern_eq (;8;) (type 4)))
  (import "miden:prelude/intrinsics_felt" "is_odd" (func $miden_prelude::intrinsics::felt::extern_is_odd (;9;) (type 5)))
  (import "miden:prelude/intrinsics_felt" "assertz" (func $miden_prelude::intrinsics::felt::extern_assertz (;10;) (type 6)))
  (import "miden:prelude/intrinsics_felt" "assert" (func $miden_prelude::intrinsics::felt::extern_assert (;11;) (type 6)))
  (import "miden:prelude/intrinsics_felt" "inv" (func $miden_prelude::intrinsics::felt::extern_inv (;12;) (type 7)))
  (import "miden:prelude/intrinsics_felt" "exp" (func $miden_prelude::intrinsics::felt::extern_exp (;13;) (type 2)))
  (import "miden:prelude/intrinsics_felt" "sub" (func $miden_prelude::intrinsics::felt::extern_sub (;14;) (type 2)))
  (import "miden:prelude/intrinsics_felt" "pow2" (func $miden_prelude::intrinsics::felt::extern_pow2 (;15;) (type 7)))
  (import "miden:prelude/intrinsics_felt" "mul" (func $miden_prelude::intrinsics::felt::extern_mul (;16;) (type 2)))
  (import "miden:prelude/intrinsics_felt" "div" (func $miden_prelude::intrinsics::felt::extern_div (;17;) (type 2)))
  (import "miden:prelude/intrinsics_felt" "assert_eq" (func $miden_prelude::intrinsics::felt::extern_assert_eq (;18;) (type 8)))
  (import "miden:prelude/intrinsics_felt" "neg" (func $miden_prelude::intrinsics::felt::extern_neg (;19;) (type 7)))
  (import "miden:prelude/std_crypto_hashes" "blake3_hash_1to1<0x0000000000000000000000000000000000000000000000000000000000000000>" (func $miden_prelude::stdlib::crypto::hashes::extern_blake3_hash_1to1 (;20;) (type 9)))
  (import "miden:prelude/std_crypto_hashes" "blake3_hash_2to1<0x0000000000000000000000000000000000000000000000000000000000000000>" (func $miden_prelude::stdlib::crypto::hashes::extern_blake3_hash_2to1 (;21;) (type 10)))
  (import "miden:prelude/std_crypto_dsa" "rpo_falcon512_verify<0x0000000000000000000000000000000000000000000000000000000000000000>" (func $miden_prelude::stdlib::crypto::dsa::extern_rpo_falcon512_verify (;22;) (type 11)))
  (import "miden:tx_kernel/note" "get_inputs<0x0000000000000000000000000000000000000000000000000000000000000000>" (func $miden_sdk_tx_kernel::externs::extern_note_get_inputs (;23;) (type 12)))
  (import "miden:tx_kernel/account" "add_asset<0x0000000000000000000000000000000000000000000000000000000000000000>" (func $miden_sdk_tx_kernel::externs::extern_account_add_asset (;24;) (type 13)))
  (import "miden:prelude/std_mem" "pipe_words_to_memory<0x0000000000000000000000000000000000000000000000000000000000000000>" (func $miden_prelude::stdlib::mem::extern_pipe_words_to_memory (;25;) (type 14)))
  (import "miden:prelude/std_mem" "pipe_double_words_to_memory<0x0000000000000000000000000000000000000000000000000000000000000000>" (func $miden_prelude::stdlib::mem::extern_pipe_double_words_to_memory (;26;) (type 15)))
  (func $<<alloc::vec::into_iter::IntoIter<T,A> as core::ops::drop::Drop>::drop::DropGuard<T,A> as core::ops::drop::Drop>::drop (;27;) (type 16) (param i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 1
    local.get 0
    i32.load
    local.tee 0
    i32.load
    i32.store offset=12
    local.get 1
    local.get 0
    i32.load offset=8
    i32.store offset=8
    local.get 1
    i32.const 8
    i32.add
    call $<alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
    local.get 1
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $<alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop (;28;) (type 16) (param i32)
    (local i32)
    block ;; label = @1
      local.get 0
      i32.load
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.get 1
      i32.const 3
      i32.shl
      i32.const 8
      call $__rust_dealloc
    end
  )
  (func $core::alloc::global::GlobalAlloc::alloc_zeroed (;29;) (type 17) (param i32 i32 i32) (result i32)
    block ;; label = @1
      local.get 0
      local.get 1
      local.get 2
      call $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::alloc
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const 0
      local.get 2
      memory.fill
    end
    local.get 1
  )
  (func $__rust_dealloc (;30;) (type 18) (param i32 i32 i32)
    i32.const 1048652
    local.get 0
    local.get 2
    local.get 1
    call $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::dealloc
  )
  (func $<alloc::vec::into_iter::IntoIter<T,A> as core::ops::drop::Drop>::drop (;31;) (type 16) (param i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.const 12
    i32.add
    call $<<alloc::vec::into_iter::IntoIter<T,A> as core::ops::drop::Drop>::drop::DropGuard<T,A> as core::ops::drop::Drop>::drop
    local.get 1
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $get_wallet_magic_number (;32;) (type 0) (result f64)
    (local f64)
    call $miden_sdk_tx_kernel::externs::extern_account_get_id
    local.set 0
    i64.const 42
    call $miden_prelude::intrinsics::felt::extern_from_u64_unchecked
    local.get 0
    call $miden_prelude::intrinsics::felt::extern_add
  )
  (func $test_add_asset (;33;) (type 0) (result f64)
    (local i32 f64 f64 f64)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    i64.const 1
    call $miden_prelude::intrinsics::felt::extern_from_u64_unchecked
    local.set 1
    i64.const 2
    call $miden_prelude::intrinsics::felt::extern_from_u64_unchecked
    local.set 2
    i64.const 3
    call $miden_prelude::intrinsics::felt::extern_from_u64_unchecked
    local.set 3
    local.get 0
    i64.const 4
    call $miden_prelude::intrinsics::felt::extern_from_u64_unchecked
    f64.store offset=24
    local.get 0
    local.get 3
    f64.store offset=16
    local.get 0
    local.get 2
    f64.store offset=8
    local.get 0
    local.get 1
    f64.store
    local.get 0
    i32.const 32
    i32.add
    local.get 0
    call $miden_sdk_tx_kernel::add_asset
    local.get 0
    f64.load offset=32
    local.set 1
    local.get 0
    i32.const 64
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $test_felt_ops_smoke (;34;) (type 2) (param f64 f64) (result f64)
    (local i64)
    local.get 0
    call $miden_prelude::intrinsics::felt::extern_as_u64
    local.set 2
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 0
                local.get 1
                call $miden_prelude::intrinsics::felt::extern_gt
                br_if 0 (;@6;)
                local.get 1
                local.get 0
                call $miden_prelude::intrinsics::felt::extern_lt
                br_if 1 (;@5;)
                local.get 1
                local.get 0
                call $miden_prelude::intrinsics::felt::extern_le
                br_if 2 (;@4;)
                local.get 0
                local.get 1
                call $miden_prelude::intrinsics::felt::extern_ge
                br_if 3 (;@3;)
                local.get 0
                local.get 1
                call $miden_prelude::intrinsics::felt::extern_eq
                i32.const 1
                i32.eq
                br_if 4 (;@2;)
                local.get 0
                local.get 1
                call $miden_prelude::intrinsics::felt::extern_eq
                i32.const 1
                i32.ne
                br_if 5 (;@1;)
                block ;; label = @7
                  local.get 1
                  call $miden_prelude::intrinsics::felt::extern_is_odd
                  br_if 0 (;@7;)
                  local.get 1
                  call $miden_prelude::intrinsics::felt::extern_assertz
                  local.get 0
                  return
                end
                local.get 0
                call $miden_prelude::intrinsics::felt::extern_assert
                local.get 1
                return
              end
              local.get 0
              call $miden_prelude::intrinsics::felt::extern_inv
              local.get 1
              call $miden_prelude::intrinsics::felt::extern_add
              return
            end
            local.get 0
            local.get 1
            call $miden_prelude::intrinsics::felt::extern_exp
            local.get 1
            call $miden_prelude::intrinsics::felt::extern_sub
            return
          end
          local.get 0
          call $miden_prelude::intrinsics::felt::extern_pow2
          local.get 1
          call $miden_prelude::intrinsics::felt::extern_mul
          return
        end
        local.get 1
        local.get 0
        call $miden_prelude::intrinsics::felt::extern_div
        return
      end
      local.get 0
      local.get 1
      call $miden_prelude::intrinsics::felt::extern_assert_eq
      local.get 0
      local.get 2
      call $miden_prelude::intrinsics::felt::extern_from_u64_unchecked
      call $miden_prelude::intrinsics::felt::extern_add
      return
    end
    local.get 0
    call $miden_prelude::intrinsics::felt::extern_neg
  )
  (func $note_script (;35;) (type 0) (result f64)
    (local i32 f64 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    i64.const 0
    call $miden_prelude::intrinsics::felt::extern_from_u64_unchecked
    local.set 1
    local.get 0
    call $miden_sdk_tx_kernel::get_inputs
    local.get 0
    i32.load
    local.set 2
    local.get 0
    local.get 0
    i32.load offset=4
    local.tee 3
    local.get 0
    i32.load offset=8
    i32.const 3
    i32.shl
    local.tee 4
    i32.add
    local.tee 5
    i32.store offset=12
    local.get 0
    local.get 2
    i32.store offset=8
    local.get 0
    local.get 3
    i32.store
    loop (result f64) ;; label = @1
      block ;; label = @2
        local.get 4
        br_if 0 (;@2;)
        local.get 0
        local.get 5
        i32.store offset=4
        local.get 0
        call $<alloc::vec::into_iter::IntoIter<T,A> as core::ops::drop::Drop>::drop
        local.get 0
        i32.const 16
        i32.add
        global.set $__stack_pointer
        local.get 1
        return
      end
      local.get 4
      i32.const -8
      i32.add
      local.set 4
      local.get 1
      local.get 3
      f64.load
      call $miden_prelude::intrinsics::felt::extern_add
      local.set 1
      local.get 3
      i32.const 8
      i32.add
      local.set 3
      br 0 (;@1;)
    end
  )
  (func $test_blake3_hash_1to1 (;36;) (type 19) (param i32 i32)
    (local i32 i32 f64)
    global.get $__stack_pointer
    i32.const 240
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i32.const 0
    local.set 3
    i64.const 0
    call $miden_prelude::intrinsics::felt::extern_from_u64_unchecked
    local.set 4
    loop ;; label = @1
      block ;; label = @2
        local.get 3
        i32.const 64
        i32.ne
        br_if 0 (;@2;)
        i32.const 0
        local.set 3
        block ;; label = @3
          loop ;; label = @4
            block ;; label = @5
              local.get 3
              i32.const 64
              i32.ne
              br_if 0 (;@5;)
              local.get 2
              f64.load offset=8
              local.get 2
              f64.load offset=16
              local.get 2
              f64.load offset=24
              local.get 2
              f64.load offset=32
              local.get 2
              f64.load offset=40
              local.get 2
              f64.load offset=48
              local.get 2
              f64.load offset=56
              local.get 2
              f64.load offset=64
              local.get 2
              i32.const 72
              i32.add
              call $miden_prelude::stdlib::crypto::hashes::extern_blake3_hash_1to1
              local.get 2
              i32.const 136
              i32.add
              local.get 2
              i32.const 72
              i32.add
              i32.const 64
              memory.copy
              local.get 2
              i32.const 224
              i32.add
              i64.const 0
              i64.store
              local.get 2
              i32.const 216
              i32.add
              i64.const 0
              i64.store
              local.get 2
              i32.const 200
              i32.add
              i32.const 8
              i32.add
              i64.const 0
              i64.store
              local.get 2
              i64.const 0
              i64.store offset=200
              local.get 2
              i32.const 136
              i32.add
              local.set 1
              i32.const 0
              local.set 3
              loop ;; label = @6
                local.get 3
                i32.const 32
                i32.eq
                br_if 3 (;@3;)
                local.get 2
                local.get 1
                f64.load
                call $miden_prelude::intrinsics::felt::extern_as_u64
                i64.store offset=232
                local.get 2
                i32.const 200
                i32.add
                local.get 3
                i32.add
                i32.const 4
                local.get 2
                i32.const 232
                i32.add
                i32.const 4
                i32.const 1048620
                call $core::slice::<impl [T]>::copy_from_slice
                local.get 3
                i32.const 4
                i32.add
                local.set 3
                local.get 1
                i32.const 8
                i32.add
                local.set 1
                br 0 (;@6;)
              end
            end
            local.get 2
            i32.const 8
            i32.add
            local.get 3
            i32.add
            local.get 1
            i64.load32_u align=1
            call $miden_prelude::intrinsics::felt::extern_from_u64_unchecked
            f64.store
            local.get 3
            i32.const 8
            i32.add
            local.set 3
            local.get 1
            i32.const 4
            i32.add
            local.set 1
            br 0 (;@4;)
          end
        end
        local.get 0
        local.get 2
        i64.load offset=200
        i64.store align=1
        local.get 0
        i32.const 24
        i32.add
        local.get 2
        i32.const 200
        i32.add
        i32.const 24
        i32.add
        i64.load
        i64.store align=1
        local.get 0
        i32.const 16
        i32.add
        local.get 2
        i32.const 200
        i32.add
        i32.const 16
        i32.add
        i64.load
        i64.store align=1
        local.get 0
        i32.const 8
        i32.add
        local.get 2
        i32.const 200
        i32.add
        i32.const 8
        i32.add
        i64.load
        i64.store align=1
        local.get 2
        i32.const 240
        i32.add
        global.set $__stack_pointer
        return
      end
      local.get 2
      i32.const 8
      i32.add
      local.get 3
      i32.add
      local.get 4
      f64.store
      local.get 3
      i32.const 8
      i32.add
      local.set 3
      br 0 (;@1;)
    end
  )
  (func $test_blake3_hash_2to1 (;37;) (type 18) (param i32 i32 i32)
    (local i32 i32 f64)
    global.get $__stack_pointer
    i32.const 432
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    i32.const 0
    local.set 4
    i64.const 0
    call $miden_prelude::intrinsics::felt::extern_from_u64_unchecked
    local.set 5
    loop ;; label = @1
      block ;; label = @2
        local.get 4
        i32.const 64
        i32.ne
        br_if 0 (;@2;)
        i32.const 0
        local.set 4
        loop ;; label = @3
          block ;; label = @4
            local.get 4
            i32.const 64
            i32.ne
            br_if 0 (;@4;)
            i32.const 0
            local.set 4
            i64.const 0
            call $miden_prelude::intrinsics::felt::extern_from_u64_unchecked
            local.set 5
            loop ;; label = @5
              block ;; label = @6
                local.get 4
                i32.const 64
                i32.ne
                br_if 0 (;@6;)
                i32.const 0
                local.set 4
                block ;; label = @7
                  loop ;; label = @8
                    block ;; label = @9
                      local.get 4
                      i32.const 64
                      i32.ne
                      br_if 0 (;@9;)
                      local.get 3
                      f64.load offset=8
                      local.get 3
                      f64.load offset=16
                      local.get 3
                      f64.load offset=24
                      local.get 3
                      f64.load offset=32
                      local.get 3
                      f64.load offset=40
                      local.get 3
                      f64.load offset=48
                      local.get 3
                      f64.load offset=56
                      local.get 3
                      f64.load offset=64
                      local.get 3
                      f64.load offset=72
                      local.get 3
                      f64.load offset=80
                      local.get 3
                      f64.load offset=88
                      local.get 3
                      f64.load offset=96
                      local.get 3
                      f64.load offset=104
                      local.get 3
                      f64.load offset=112
                      local.get 3
                      f64.load offset=120
                      local.get 3
                      f64.load offset=128
                      local.get 3
                      i32.const 136
                      i32.add
                      call $miden_prelude::stdlib::crypto::hashes::extern_blake3_hash_2to1
                      local.get 3
                      i32.const 264
                      i32.add
                      local.get 3
                      i32.const 136
                      i32.add
                      i32.const 128
                      memory.copy
                      local.get 3
                      i32.const 416
                      i32.add
                      i64.const 0
                      i64.store
                      local.get 3
                      i32.const 408
                      i32.add
                      i64.const 0
                      i64.store
                      local.get 3
                      i32.const 392
                      i32.add
                      i32.const 8
                      i32.add
                      i64.const 0
                      i64.store
                      local.get 3
                      i64.const 0
                      i64.store offset=392
                      local.get 3
                      i32.const 264
                      i32.add
                      local.set 2
                      i32.const 0
                      local.set 4
                      loop ;; label = @10
                        local.get 4
                        i32.const 32
                        i32.eq
                        br_if 3 (;@7;)
                        local.get 3
                        local.get 2
                        f64.load
                        call $miden_prelude::intrinsics::felt::extern_as_u64
                        i64.store offset=424
                        local.get 3
                        i32.const 392
                        i32.add
                        local.get 4
                        i32.add
                        i32.const 4
                        local.get 3
                        i32.const 424
                        i32.add
                        i32.const 4
                        i32.const 1048636
                        call $core::slice::<impl [T]>::copy_from_slice
                        local.get 4
                        i32.const 4
                        i32.add
                        local.set 4
                        local.get 2
                        i32.const 8
                        i32.add
                        local.set 2
                        br 0 (;@10;)
                      end
                    end
                    local.get 3
                    i32.const 72
                    i32.add
                    local.get 4
                    i32.add
                    local.get 2
                    i64.load32_u align=1
                    call $miden_prelude::intrinsics::felt::extern_from_u64_unchecked
                    f64.store
                    local.get 4
                    i32.const 8
                    i32.add
                    local.set 4
                    local.get 2
                    i32.const 4
                    i32.add
                    local.set 2
                    br 0 (;@8;)
                  end
                end
                local.get 0
                local.get 3
                i64.load offset=392
                i64.store align=1
                local.get 0
                i32.const 24
                i32.add
                local.get 3
                i32.const 392
                i32.add
                i32.const 24
                i32.add
                i64.load
                i64.store align=1
                local.get 0
                i32.const 16
                i32.add
                local.get 3
                i32.const 392
                i32.add
                i32.const 16
                i32.add
                i64.load
                i64.store align=1
                local.get 0
                i32.const 8
                i32.add
                local.get 3
                i32.const 392
                i32.add
                i32.const 8
                i32.add
                i64.load
                i64.store align=1
                local.get 3
                i32.const 432
                i32.add
                global.set $__stack_pointer
                return
              end
              local.get 3
              i32.const 72
              i32.add
              local.get 4
              i32.add
              local.get 5
              f64.store
              local.get 4
              i32.const 8
              i32.add
              local.set 4
              br 0 (;@5;)
            end
          end
          local.get 3
          i32.const 8
          i32.add
          local.get 4
          i32.add
          local.get 1
          i64.load32_u align=1
          call $miden_prelude::intrinsics::felt::extern_from_u64_unchecked
          f64.store
          local.get 4
          i32.const 8
          i32.add
          local.set 4
          local.get 1
          i32.const 4
          i32.add
          local.set 1
          br 0 (;@3;)
        end
      end
      local.get 3
      i32.const 8
      i32.add
      local.get 4
      i32.add
      local.get 5
      f64.store
      local.get 4
      i32.const 8
      i32.add
      local.set 4
      br 0 (;@1;)
    end
  )
  (func $test_rpo_falcon512_verify (;38;) (type 19) (param i32 i32)
    local.get 0
    f64.load
    local.get 0
    f64.load offset=8
    local.get 0
    f64.load offset=16
    local.get 0
    f64.load offset=24
    local.get 1
    f64.load
    local.get 1
    f64.load offset=8
    local.get 1
    f64.load offset=16
    local.get 1
    f64.load offset=24
    call $miden_prelude::stdlib::crypto::dsa::extern_rpo_falcon512_verify
  )
  (func $test_pipe_words_to_memory (;39;) (type 20) (param i32 f64)
    local.get 0
    local.get 1
    call $miden_prelude::stdlib::mem::pipe_words_to_memory
  )
  (func $test_pipe_double_words_to_memory (;40;) (type 20) (param i32 f64)
    local.get 0
    local.get 1
    call $miden_prelude::stdlib::mem::pipe_double_words_to_memory
  )
  (func $__rust_alloc (;41;) (type 21) (param i32 i32) (result i32)
    i32.const 1048652
    local.get 1
    local.get 0
    call $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::alloc
  )
  (func $__rust_alloc_zeroed (;42;) (type 21) (param i32 i32) (result i32)
    i32.const 1048652
    local.get 1
    local.get 0
    call $core::alloc::global::GlobalAlloc::alloc_zeroed
  )
  (func $wee_alloc::neighbors::Neighbors<T>::remove (;43;) (type 16) (param i32)
    (local i32 i32 i32)
    block ;; label = @1
      local.get 0
      i32.load
      local.tee 1
      i32.const 2
      i32.and
      br_if 0 (;@1;)
      local.get 1
      i32.const -4
      i32.and
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
  (func $<wee_alloc::LargeAllocPolicy as wee_alloc::AllocPolicy>::new_cell_for_free_list (;44;) (type 22) (param i32 i32 i32 i32)
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
  (func $wee_alloc::alloc_first_fit (;45;) (type 17) (param i32 i32 i32) (result i32)
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
                    i32.const 2
                    i32.and
                    br_if 0 (;@8;)
                    local.get 8
                    i32.const -4
                    i32.and
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
  (func $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::alloc (;46;) (type 17) (param i32 i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 1
    i32.const 1
    local.get 1
    i32.const 1
    i32.gt_u
    select
    local.set 1
    block ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
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
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        local.get 3
        i32.load offset=12
        i32.store
        local.get 2
        local.set 1
        br 1 (;@1;)
      end
      local.get 3
      local.get 3
      local.get 4
      local.get 1
      call $<wee_alloc::LargeAllocPolicy as wee_alloc::AllocPolicy>::new_cell_for_free_list
      block ;; label = @2
        block ;; label = @3
          local.get 3
          i32.load
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          local.get 3
          i32.load offset=12
          i32.store
          br 1 (;@2;)
        end
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
        local.set 1
        local.get 0
        local.get 3
        i32.load offset=12
        i32.store
        local.get 1
        br_if 1 (;@1;)
      end
      i32.const 0
      local.set 1
    end
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::dealloc (;47;) (type 22) (param i32 i32 i32 i32)
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
              i32.const 2
              i32.and
              br_if 0 (;@5;)
              local.get 4
              i32.const -4
              i32.and
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
  (func $miden_sdk_tx_kernel::get_inputs (;48;) (type 16) (param i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 1
    i32.const 4
    i32.add
    i32.const 256
    i32.const 0
    call $alloc::raw_vec::RawVec<T,A>::try_allocate_in
    local.get 1
    i32.load offset=12
    local.set 2
    local.get 1
    i32.load offset=8
    local.set 3
    block ;; label = @1
      block ;; label = @2
        local.get 1
        i32.load offset=4
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.eqz
        br_if 1 (;@1;)
        local.get 3
        local.get 2
        call $alloc::alloc::handle_alloc_error
        unreachable
      end
      local.get 2
      call $miden_sdk_tx_kernel::externs::extern_note_get_inputs
      drop
      local.get 0
      i32.const 0
      i32.store offset=8
      local.get 0
      local.get 2
      i32.store offset=4
      local.get 0
      local.get 3
      i32.store
      local.get 1
      i32.const 16
      i32.add
      global.set $__stack_pointer
      return
    end
    call $alloc::raw_vec::capacity_overflow
    unreachable
  )
  (func $miden_sdk_tx_kernel::add_asset (;49;) (type 19) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 1
    f64.load
    local.get 1
    f64.load offset=8
    local.get 1
    f64.load offset=16
    local.get 1
    f64.load offset=24
    local.get 2
    call $miden_sdk_tx_kernel::externs::extern_account_add_asset
    local.get 0
    i32.const 24
    i32.add
    local.get 2
    i32.const 24
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 16
    i32.add
    local.get 2
    i32.const 16
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 8
    i32.add
    local.get 2
    i32.const 8
    i32.add
    i64.load
    i64.store
    local.get 0
    local.get 2
    i64.load
    i64.store
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $alloc::vec::Vec<T>::with_capacity (;50;) (type 19) (param i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 4
    i32.add
    local.get 1
    i32.const 0
    call $alloc::raw_vec::RawVec<T,A>::try_allocate_in
    local.get 2
    i32.load offset=12
    local.set 3
    local.get 2
    i32.load offset=8
    local.set 1
    block ;; label = @1
      block ;; label = @2
        local.get 2
        i32.load offset=4
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.eqz
        br_if 1 (;@1;)
        local.get 1
        local.get 3
        call $alloc::alloc::handle_alloc_error
        unreachable
      end
      local.get 0
      i32.const 0
      i32.store offset=8
      local.get 0
      local.get 3
      i32.store offset=4
      local.get 0
      local.get 1
      i32.store
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      return
    end
    call $alloc::raw_vec::capacity_overflow
    unreachable
  )
  (func $alloc::raw_vec::RawVec<T,A>::try_allocate_in (;51;) (type 18) (param i32 i32 i32)
    (local i32)
    block ;; label = @1
      block ;; label = @2
        local.get 1
        br_if 0 (;@2;)
        local.get 0
        i64.const 34359738368
        i64.store offset=4 align=4
        i32.const 0
        local.set 1
        br 1 (;@1;)
      end
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i32.const 268435456
          i32.lt_u
          br_if 0 (;@3;)
          local.get 0
          i32.const 0
          i32.store offset=4
          br 1 (;@2;)
        end
        local.get 1
        i32.const 3
        i32.shl
        local.set 3
        block ;; label = @3
          block ;; label = @4
            local.get 2
            br_if 0 (;@4;)
            i32.const 0
            i32.load8_u offset=1048656
            drop
            local.get 3
            i32.const 8
            call $__rust_alloc
            local.set 2
            br 1 (;@3;)
          end
          local.get 3
          i32.const 8
          call $__rust_alloc_zeroed
          local.set 2
        end
        block ;; label = @3
          local.get 2
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          local.get 2
          i32.store offset=8
          local.get 0
          local.get 1
          i32.store offset=4
          i32.const 0
          local.set 1
          br 2 (;@1;)
        end
        local.get 0
        local.get 3
        i32.store offset=8
        local.get 0
        i32.const 8
        i32.store offset=4
      end
      i32.const 1
      local.set 1
    end
    local.get 0
    local.get 1
    i32.store
  )
  (func $<miden_prelude::intrinsics::felt::Felt as core::convert::From<u32>>::from (;52;) (type 23) (param i32) (result f64)
    local.get 0
    i64.extend_i32_u
    call $miden_prelude::intrinsics::felt::extern_from_u64_unchecked
  )
  (func $miden_prelude::stdlib::mem::pipe_words_to_memory (;53;) (type 20) (param i32 f64)
    (local i32)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 52
    i32.add
    local.get 1
    call $miden_prelude::intrinsics::felt::extern_as_u64
    i32.wrap_i64
    i32.const 2
    i32.shl
    call $alloc::vec::Vec<T>::with_capacity
    local.get 1
    local.get 2
    i32.load offset=56
    call $<miden_prelude::intrinsics::felt::Felt as core::convert::From<u32>>::from
    local.get 2
    i32.const 8
    i32.add
    call $miden_prelude::stdlib::mem::extern_pipe_words_to_memory
    local.get 0
    local.get 2
    f64.load offset=32
    f64.store offset=24
    local.get 0
    local.get 2
    f64.load offset=24
    f64.store offset=16
    local.get 0
    local.get 2
    f64.load offset=16
    f64.store offset=8
    local.get 0
    local.get 2
    f64.load offset=8
    f64.store
    local.get 0
    local.get 2
    i64.load offset=52 align=4
    i64.store offset=32 align=4
    local.get 0
    i32.const 40
    i32.add
    local.get 2
    i32.const 60
    i32.add
    i32.load
    i32.store
    local.get 2
    i32.const 64
    i32.add
    global.set $__stack_pointer
  )
  (func $miden_prelude::stdlib::mem::pipe_double_words_to_memory (;54;) (type 20) (param i32 f64)
    (local i32 i32 f64 f64)
    global.get $__stack_pointer
    i32.const 128
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 12
    i32.add
    local.get 1
    call $miden_prelude::intrinsics::felt::extern_as_u64
    i32.wrap_i64
    i32.const 2
    i32.shl
    local.tee 3
    call $alloc::vec::Vec<T>::with_capacity
    local.get 2
    i32.load offset=16
    call $<miden_prelude::intrinsics::felt::Felt as core::convert::From<u32>>::from
    local.tee 4
    local.get 3
    call $<miden_prelude::intrinsics::felt::Felt as core::convert::From<u32>>::from
    call $miden_prelude::intrinsics::felt::extern_add
    local.set 5
    i64.const 0
    call $miden_prelude::intrinsics::felt::extern_from_u64_unchecked
    local.tee 1
    local.get 1
    local.get 1
    local.get 1
    local.get 1
    local.get 1
    local.get 1
    local.get 1
    local.get 1
    local.get 1
    local.get 1
    local.get 1
    local.get 4
    local.get 5
    local.get 2
    i32.const 24
    i32.add
    call $miden_prelude::stdlib::mem::extern_pipe_double_words_to_memory
    local.get 0
    local.get 2
    f64.load offset=80
    f64.store offset=24
    local.get 0
    local.get 2
    f64.load offset=72
    f64.store offset=16
    local.get 0
    local.get 2
    f64.load offset=64
    f64.store offset=8
    local.get 0
    local.get 2
    f64.load offset=56
    f64.store
    local.get 0
    local.get 2
    i64.load offset=12 align=4
    i64.store offset=32 align=4
    local.get 0
    i32.const 40
    i32.add
    local.get 2
    i32.const 20
    i32.add
    i32.load
    i32.store
    local.get 2
    i32.const 128
    i32.add
    global.set $__stack_pointer
  )
  (func $alloc::alloc::handle_alloc_error (;55;) (type 19) (param i32 i32)
    unreachable
    unreachable
  )
  (func $alloc::raw_vec::capacity_overflow (;56;) (type 24)
    unreachable
    unreachable
  )
  (func $core::slice::<impl [T]>::copy_from_slice::len_mismatch_fail (;57;) (type 18) (param i32 i32 i32)
    unreachable
    unreachable
  )
  (func $core::slice::<impl [T]>::copy_from_slice (;58;) (type 25) (param i32 i32 i32 i32 i32)
    block ;; label = @1
      local.get 1
      local.get 3
      i32.ne
      br_if 0 (;@1;)
      local.get 0
      local.get 2
      local.get 1
      memory.copy
      return
    end
    local.get 1
    local.get 1
    local.get 1
    call $core::slice::<impl [T]>::copy_from_slice::len_mismatch_fail
    unreachable
  )
  (table (;0;) 1 1 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (export "memory" (memory 0))
  (export "get_wallet_magic_number" (func $get_wallet_magic_number))
  (export "test_add_asset" (func $test_add_asset))
  (export "test_felt_ops_smoke" (func $test_felt_ops_smoke))
  (export "note_script" (func $note_script))
  (export "test_blake3_hash_1to1" (func $test_blake3_hash_1to1))
  (export "test_blake3_hash_2to1" (func $test_blake3_hash_2to1))
  (export "test_rpo_falcon512_verify" (func $test_rpo_falcon512_verify))
  (export "test_pipe_words_to_memory" (func $test_pipe_words_to_memory))
  (export "test_pipe_double_words_to_memory" (func $test_pipe_double_words_to_memory))
  (data $.rodata (;0;) (i32.const 1048576) "~/sdk/prelude/src/stdlib/crypto/hashes.rs\00\00\00\00\00\10\00)\00\00\00\8a\00\00\00(\00\00\00\00\00\10\00)\00\00\00\d0\00\00\00(\00\00\00")
)