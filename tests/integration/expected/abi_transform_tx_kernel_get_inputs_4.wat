(module $abi_transform_tx_kernel_get_inputs_4.wasm
  (type (;0;) (func (result i32)))
  (type (;1;) (func (param i32) (result i32)))
  (type (;2;) (func (param i32)))
  (type (;3;) (func (param i32 i32) (result i32)))
  (type (;4;) (func (param i32 i32 i32) (result i32)))
  (type (;5;) (func (param i32 i32 i32)))
  (type (;6;) (func (param i32 i32)))
  (import "intrinsics::mem" "heap_base" (func $miden_sdk_alloc::heap_base (;0;) (type 0)))
  (import "miden::note" "get_inputs<0x0000000000000000000000000000000000000000000000000000000000000000>" (func $miden_base_sys::bindings::tx::externs::extern_note_get_inputs (;1;) (type 1)))
  (func $entrypoint (;2;) (type 2) (param i32)
    local.get 0
    call $miden_base_sys::bindings::tx::get_inputs
  )
  (func $__rust_alloc (;3;) (type 3) (param i32 i32) (result i32)
    i32.const 1048576
    local.get 1
    local.get 0
    call $<miden_sdk_alloc::BumpAlloc as core::alloc::global::GlobalAlloc>::alloc
  )
  (func $__rust_alloc_zeroed (;4;) (type 3) (param i32 i32) (result i32)
    block ;; label = @1
      i32.const 1048576
      local.get 1
      local.get 0
      call $<miden_sdk_alloc::BumpAlloc as core::alloc::global::GlobalAlloc>::alloc
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
  (func $<miden_sdk_alloc::BumpAlloc as core::alloc::global::GlobalAlloc>::alloc (;5;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32)
    block ;; label = @1
      local.get 1
      i32.const 32
      local.get 1
      i32.const 32
      i32.gt_u
      select
      local.tee 1
      i32.popcnt
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      i32.const -2147483648
      local.get 1
      i32.sub
      local.get 2
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 0
      local.set 3
      local.get 1
      local.get 2
      i32.add
      i32.const -1
      i32.add
      i32.const 0
      local.get 1
      i32.sub
      i32.and
      local.set 2
      block ;; label = @2
        local.get 0
        i32.load
        br_if 0 (;@2;)
        local.get 0
        call $miden_sdk_alloc::heap_base
        memory.size
        i32.const 16
        i32.shl
        i32.add
        i32.store
      end
      block ;; label = @2
        i32.const 268435456
        local.get 0
        i32.load
        local.tee 4
        i32.sub
        local.get 2
        i32.lt_u
        br_if 0 (;@2;)
        local.get 0
        local.get 4
        local.get 2
        i32.add
        i32.store
        local.get 4
        local.get 1
        i32.add
        local.set 3
      end
      local.get 3
      return
    end
    unreachable
    unreachable
  )
  (func $miden_base_sys::bindings::tx::get_inputs (;6;) (type 2) (param i32)
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
    i32.load offset=8
    local.set 2
    block ;; label = @1
      local.get 1
      i32.load offset=4
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.get 1
      i32.load offset=12
      call $alloc::raw_vec::handle_error
      unreachable
    end
    local.get 0
    local.get 1
    i32.load offset=12
    local.tee 3
    i32.const 4
    i32.shr_u
    call $miden_base_sys::bindings::tx::externs::extern_note_get_inputs
    i32.store offset=8
    local.get 0
    local.get 3
    i32.store offset=4
    local.get 0
    local.get 2
    i32.store
    local.get 1
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $alloc::raw_vec::RawVec<T,A>::try_allocate_in (;7;) (type 5) (param i32 i32 i32)
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
  (func $alloc::raw_vec::handle_error (;8;) (type 6) (param i32 i32)
    unreachable
    unreachable
  )
  (table (;0;) 1 1 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (export "memory" (memory 0))
  (export "entrypoint" (func $entrypoint))
)