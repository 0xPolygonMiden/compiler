(module $abi_transform_stdlib_blake3_hash.wasm
  (type (;0;) (func (param i64) (result f64)))
  (type (;1;) (func (param f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 i32)))
  (type (;2;) (func (param f64) (result i64)))
  (type (;3;) (func (param i32 i32 i32)))
  (type (;4;) (func (param i32 i32 i32 i32 i32)))
  (import "miden:prelude/intrinsics_felt" "from_u64_unchecked" (func $miden_prelude::intrinsics::felt::extern_from_u64_unchecked (;0;) (type 0)))
  (import "std::crypto_hashes" "blake3_hash_2to1<0x0000000000000000000000000000000000000000000000000000000000000000>" (func $miden_prelude::stdlib::crypto::hashes::extern_blake3_hash_2to1 (;1;) (type 1)))
  (import "miden:prelude/intrinsics_felt" "as_u64" (func $miden_prelude::intrinsics::felt::extern_as_u64 (;2;) (type 2)))
  (func $entrypoint (;3;) (type 3) (param i32 i32 i32)
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
                        i32.const 1048620
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
  (func $core::slice::<impl [T]>::copy_from_slice::len_mismatch_fail (;4;) (type 3) (param i32 i32 i32)
    unreachable
    unreachable
  )
  (func $core::slice::<impl [T]>::copy_from_slice (;5;) (type 4) (param i32 i32 i32 i32 i32)
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
  (export "entrypoint" (func $entrypoint))
  (data $.rodata (;0;) (i32.const 1048576) "~/sdk/prelude/src/stdlib/crypto/hashes.rs\00\00\00\00\00\10\00)\00\00\00\d1\00\00\00(\00\00\00")
)