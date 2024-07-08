(module $abi_transform_tx_kernel_get_inputs_4.wasm
  (type (;0;) (func (param i64) (result f64)))
  (type (;1;) (func (param f64 f64)))
  (type (;2;) (func (param i32) (result i32)))
  (type (;3;) (func (param i32)))
  (type (;4;) (func (param i32 i32) (result i32)))
  (type (;5;) (func (param i32 i32 i32 i32)))
  (type (;6;) (func (param i32 i32 i32) (result i32)))
  (type (;7;) (func (param i32 i32 i32)))
  (type (;8;) (func (param i32 i32)))
  (type (;9;) (func))
  (import "miden:stdlib/intrinsics_felt" "from_u64_unchecked" (func $miden_stdlib_sys::intrinsics::felt::extern_from_u64_unchecked (;0;) (type 0)))
  (import "miden:stdlib/intrinsics_felt" "assert_eq" (func $miden_stdlib_sys::intrinsics::felt::extern_assert_eq (;1;) (type 1)))
  (import "miden::note" "get_inputs<0x0000000000000000000000000000000000000000000000000000000000000000>" (func $miden_tx_kernel_sys::externs::extern_note_get_inputs (;2;) (type 2)))
  (func $entrypoint (;3;) (type 3) (param i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 1
    i32.const 4
    i32.add
    call $miden_tx_kernel_sys::get_inputs
    block ;; label = @1
      local.get 1
      i32.load offset=12
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=8
      local.tee 3
      f64.load
      i64.const 4294967295
      call $miden_stdlib_sys::intrinsics::felt::extern_from_u64_unchecked
      call $miden_stdlib_sys::intrinsics::felt::extern_assert_eq
      local.get 2
      i32.const 1
      i32.eq
      br_if 0 (;@1;)
      local.get 3
      f64.load offset=8
      i64.const 1
      call $miden_stdlib_sys::intrinsics::felt::extern_from_u64_unchecked
      call $miden_stdlib_sys::intrinsics::felt::extern_assert_eq
      local.get 2
      i32.const 2
      i32.le_u
      br_if 0 (;@1;)
      local.get 3
      f64.load offset=16
      i64.const 0
      call $miden_stdlib_sys::intrinsics::felt::extern_from_u64_unchecked
      call $miden_stdlib_sys::intrinsics::felt::extern_assert_eq
      local.get 2
      i32.const 3
      i32.eq
      br_if 0 (;@1;)
      local.get 3
      f64.load offset=24
      i64.const 4294967295
      call $miden_stdlib_sys::intrinsics::felt::extern_from_u64_unchecked
      call $miden_stdlib_sys::intrinsics::felt::extern_assert_eq
      local.get 0
      i32.const 8
      i32.add
      local.get 1
      i32.const 4
      i32.add
      i32.const 8
      i32.add
      i32.load
      i32.store
      local.get 0
      local.get 1
      i64.load offset=4 align=4
      i64.store align=4
      local.get 1
      i32.const 16
      i32.add
      global.set $__stack_pointer
      return
    end
    unreachable
    unreachable
  )
  (func $__rust_alloc (;4;) (type 4) (param i32 i32) (result i32)
    i32.const 1048576
    local.get 1
    local.get 0
    call $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::alloc
  )
  (func $__rust_alloc_zeroed (;5;) (type 4) (param i32 i32) (result i32)
    block ;; label = @1
      i32.const 1048576
      local.get 1
      local.get 0
      call $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::alloc
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const 0
      local.get 0
      memory.fill
    end
    local.get 1
  )
  (func $wee_alloc::neighbors::Neighbors<T>::remove (;6;) (type 3) (param i32)
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
  (func $<wee_alloc::LargeAllocPolicy as wee_alloc::AllocPolicy>::new_cell_for_free_list (;7;) (type 5) (param i32 i32 i32 i32)
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
  (func $wee_alloc::alloc_first_fit (;8;) (type 6) (param i32 i32 i32) (result i32)
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
  (func $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::alloc (;9;) (type 6) (param i32 i32 i32) (result i32)
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
  (func $miden_tx_kernel_sys::get_inputs (;10;) (type 3) (param i32)
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
      call $miden_tx_kernel_sys::externs::extern_note_get_inputs
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
  (func $alloc::raw_vec::RawVec<T,A>::try_allocate_in (;11;) (type 7) (param i32 i32 i32)
    (local i32)
    block ;; label = @1
      block ;; label = @2
        local.get 1
        br_if 0 (;@2;)
        local.get 0
        i64.const 17179869184
        i64.store offset=4 align=4
        i32.const 0
        local.set 1
        br 1 (;@1;)
      end
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i32.const 536870912
          i32.lt_u
          br_if 0 (;@3;)
          local.get 0
          i32.const 0
          i32.store offset=4
          br 1 (;@2;)
        end
        local.get 1
        i32.const 2
        i32.shl
        local.set 3
        block ;; label = @3
          block ;; label = @4
            local.get 2
            br_if 0 (;@4;)
            i32.const 0
            i32.load8_u offset=1048580
            drop
            local.get 3
            i32.const 4
            call $__rust_alloc
            local.set 2
            br 1 (;@3;)
          end
          local.get 3
          i32.const 4
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
        i32.const 4
        i32.store offset=4
      end
      i32.const 1
      local.set 1
    end
    local.get 0
    local.get 1
    i32.store
  )
  (func $alloc::alloc::handle_alloc_error (;12;) (type 8) (param i32 i32)
    unreachable
    unreachable
  )
  (func $alloc::raw_vec::capacity_overflow (;13;) (type 9)
    unreachable
    unreachable
  )
  (table (;0;) 1 1 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (export "memory" (memory 0))
  (export "entrypoint" (func $entrypoint))
)