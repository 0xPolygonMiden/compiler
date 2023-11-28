(module
  (type (;0;) (func (param i32 i32 i32) (result i32)))
  (type (;1;) (func (param i32 i32) (result i32)))
  (type (;2;) (func))
  (type (;3;) (func (param i32)))
  (type (;4;) (func (param i64 i64 i64 i64)))
  (type (;5;) (func (result i32)))
  (type (;6;) (func (param i32 i32)))
  (type (;7;) (func (param i32 i32 i32)))
  (type (;8;) (func (param i32 i32 i32 i32)))
  (type (;9;) (func (param i32) (result i32)))
  (type (;10;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;11;) (func (param i32 i32 i32 i32 i32) (result i32)))
  (type (;12;) (func (param i32 i64)))
  (type (;13;) (func (param i32 i32 i32 i32 i32)))
  (type (;14;) (func (param i64) (result i64)))
  (type (;15;) (func (param i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;16;) (func (param i64 i32 i32) (result i32)))
  (import "env" "__wasm_call_dtors" (func $__wasm_call_dtors (;0;) (type 2)))
  (import "env" "__wasi_proc_exit" (func $__wasi_proc_exit (;1;) (type 3)))
  (import "env" "memcpy" (func $memcpy (;2;) (type 0)))
  (import "env" "my_wallet::receive_asset" (func $my_wallet::receive_asset (;3;) (type 4)))
  (import "env" "memset" (func $memset (;4;) (type 0)))
  (import "env" "miden::sat::note::get_assets" (func $miden::sat::note::get_assets (;5;) (type 3)))
  (func $__wasm_call_ctors (;6;) (type 2))
  (func $_start (;7;) (type 2)
    (local i32)
    block ;; label = @1
      block ;; label = @2
        i32.const 0
        i32.load offset=1049324
        br_if 0 (;@2;)
        i32.const 0
        i32.const 1
        i32.store offset=1049324
        call $__wasm_call_ctors
        call $__main_void
        local.set 0
        call $__wasm_call_dtors
        local.get 0
        br_if 1 (;@1;)
        return
      end
      unreachable
      unreachable
    end
    local.get 0
    call $__wasi_proc_exit
    unreachable
  )
  (func $__main_void (;8;) (type 5) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    local.get 0
    i32.const 8
    i32.add
    call $miden::sat::note::get_assets
    i32.const 1048576
    local.get 0
    i32.const 8
    i32.add
    call $basic_wallet::MyWallet::receive_asset
    local.get 0
    i32.const 48
    i32.add
    global.set $__stack_pointer
    i32.const 0
  )
  (func $__rust_alloc_error_handler (;9;) (type 6) (param i32 i32)
    local.get 0
    local.get 1
    call $__rdl_oom
    return
  )
  (func $<alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop (;10;) (type 3) (param i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 1
    i32.const 4
    i32.add
    local.get 0
    call $alloc::raw_vec::RawVec<T,A>::current_memory
    block ;; label = @1
      local.get 1
      i32.load offset=8
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const 12
      i32.add
      i32.load
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=4
      local.get 1
      local.get 0
      call $__rust_dealloc
    end
    local.get 1
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $__rust_dealloc (;11;) (type 7) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    call $dlmalloc::sys::enable_alloc_after_fork
    local.get 3
    i32.const 15
    i32.add
    call $<dlmalloc::global::Instance as core::ops::deref::DerefMut>::deref_mut
    local.get 0
    call $dlmalloc::dlmalloc::Dlmalloc<A>::free
    local.get 3
    i32.const 15
    i32.add
    call $<dlmalloc::global::Instance as core::ops::drop::Drop>::drop
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $<alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index (;12;) (type 0) (param i32 i32 i32) (result i32)
    (local i32)
    block ;; label = @1
      local.get 0
      i32.load offset=8
      local.tee 3
      local.get 1
      i32.gt_u
      br_if 0 (;@1;)
      local.get 1
      local.get 3
      local.get 2
      call $core::panicking::panic_bounds_check
      unreachable
    end
    local.get 0
    i32.load
    local.get 1
    i32.const 3
    i32.shl
    i32.add
  )
  (func $dlmalloc::Dlmalloc<A>::malloc (;13;) (type 0) (param i32 i32 i32) (result i32)
    block ;; label = @1
      local.get 2
      i32.const 9
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      local.get 2
      local.get 1
      call $dlmalloc::dlmalloc::Dlmalloc<A>::memalign
      return
    end
    local.get 0
    local.get 1
    call $dlmalloc::dlmalloc::Dlmalloc<A>::malloc
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::memalign (;14;) (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    block ;; label = @1
      local.get 1
      call $dlmalloc::dlmalloc::Dlmalloc<A>::min_chunk_size
      local.get 1
      i32.le_u
      br_if 0 (;@1;)
      local.get 1
      call $dlmalloc::dlmalloc::Dlmalloc<A>::min_chunk_size
      local.set 1
    end
    i32.const 0
    local.set 3
    block ;; label = @1
      local.get 0
      call $dlmalloc::dlmalloc::Dlmalloc<A>::max_request
      local.get 1
      i32.sub
      local.get 2
      i32.le_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      local.get 1
      local.get 2
      call $dlmalloc::dlmalloc::Dlmalloc<A>::request2size
      local.tee 4
      i32.add
      local.get 1
      call $dlmalloc::dlmalloc::Dlmalloc<A>::min_chunk_size
      i32.add
      i32.const -4
      i32.add
      call $dlmalloc::dlmalloc::Dlmalloc<A>::malloc
      local.tee 5
      i32.eqz
      br_if 0 (;@1;)
      local.get 5
      call $dlmalloc::dlmalloc::Chunk::from_mem
      local.set 2
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i32.const -1
          i32.add
          local.tee 3
          local.get 5
          i32.and
          br_if 0 (;@3;)
          local.get 2
          local.set 1
          br 1 (;@2;)
        end
        local.get 3
        local.get 5
        i32.add
        i32.const 0
        local.get 1
        i32.sub
        i32.and
        call $dlmalloc::dlmalloc::Chunk::from_mem
        local.set 3
        local.get 1
        call $dlmalloc::dlmalloc::Dlmalloc<A>::min_chunk_size
        local.set 5
        local.get 2
        call $dlmalloc::dlmalloc::Chunk::size
        local.get 3
        i32.const 0
        local.get 1
        local.get 3
        local.get 2
        i32.sub
        local.get 5
        i32.gt_u
        select
        i32.add
        local.tee 1
        local.get 2
        i32.sub
        local.tee 3
        i32.sub
        local.set 5
        block ;; label = @3
          local.get 2
          call $dlmalloc::dlmalloc::Chunk::mmapped
          br_if 0 (;@3;)
          local.get 1
          local.get 5
          call $dlmalloc::dlmalloc::Chunk::set_inuse
          local.get 2
          local.get 3
          call $dlmalloc::dlmalloc::Chunk::set_inuse
          local.get 0
          local.get 2
          local.get 3
          call $dlmalloc::dlmalloc::Dlmalloc<A>::dispose_chunk
          br 1 (;@2;)
        end
        local.get 2
        i32.load
        local.set 2
        local.get 1
        local.get 5
        i32.store offset=4
        local.get 1
        local.get 2
        local.get 3
        i32.add
        i32.store
      end
      block ;; label = @2
        local.get 1
        call $dlmalloc::dlmalloc::Chunk::mmapped
        br_if 0 (;@2;)
        local.get 1
        call $dlmalloc::dlmalloc::Chunk::size
        local.tee 2
        local.get 1
        call $dlmalloc::dlmalloc::Dlmalloc<A>::min_chunk_size
        local.get 4
        i32.add
        i32.le_u
        br_if 0 (;@2;)
        local.get 1
        local.get 4
        call $dlmalloc::dlmalloc::Chunk::plus_offset
        local.set 3
        local.get 1
        local.get 4
        call $dlmalloc::dlmalloc::Chunk::set_inuse
        local.get 3
        local.get 2
        local.get 4
        i32.sub
        local.tee 2
        call $dlmalloc::dlmalloc::Chunk::set_inuse
        local.get 0
        local.get 3
        local.get 2
        call $dlmalloc::dlmalloc::Dlmalloc<A>::dispose_chunk
      end
      local.get 1
      call $dlmalloc::dlmalloc::Chunk::to_mem
      local.set 3
      local.get 1
      call $dlmalloc::dlmalloc::Chunk::mmapped
      drop
    end
    local.get 3
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::malloc (;15;) (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 1
            i32.const 245
            i32.lt_u
            br_if 0 (;@4;)
            i32.const 0
            local.set 2
            local.get 0
            call $dlmalloc::dlmalloc::Dlmalloc<A>::max_request
            local.get 1
            i32.le_u
            br_if 2 (;@2;)
            local.get 0
            local.get 1
            call $dlmalloc::dlmalloc::Dlmalloc<A>::pad_request
            local.set 1
            local.get 0
            i32.load offset=412
            i32.eqz
            br_if 1 (;@3;)
            local.get 0
            local.get 1
            call $dlmalloc::dlmalloc::Dlmalloc<A>::tmalloc_large
            local.tee 2
            br_if 2 (;@2;)
            br 1 (;@3;)
          end
          local.get 0
          local.get 1
          call $dlmalloc::dlmalloc::Dlmalloc<A>::request2size
          local.set 1
          block ;; label = @4
            local.get 0
            i32.load offset=408
            local.get 1
            i32.const 3
            i32.shr_u
            local.tee 3
            i32.shr_u
            local.tee 2
            i32.const 3
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            local.get 0
            local.get 2
            i32.const -1
            i32.xor
            i32.const 1
            i32.and
            local.get 3
            i32.add
            local.tee 2
            i32.const 3
            i32.shl
            local.tee 3
            i32.add
            local.tee 1
            i32.const 144
            i32.add
            local.get 1
            i32.const 152
            i32.add
            i32.load
            local.tee 1
            local.get 2
            call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_first_small_chunk
            local.get 1
            local.get 3
            call $dlmalloc::dlmalloc::Chunk::set_inuse_and_pinuse
            local.get 1
            call $dlmalloc::dlmalloc::Chunk::to_mem
            return
          end
          local.get 1
          local.get 0
          i32.load offset=416
          i32.le_u
          br_if 0 (;@3;)
          block ;; label = @4
            local.get 2
            br_if 0 (;@4;)
            local.get 0
            i32.load offset=412
            i32.eqz
            br_if 1 (;@3;)
            local.get 0
            local.get 1
            call $dlmalloc::dlmalloc::Dlmalloc<A>::tmalloc_small
            local.tee 2
            i32.eqz
            br_if 1 (;@3;)
            br 2 (;@2;)
          end
          local.get 0
          local.get 0
          i32.const 1
          local.get 3
          i32.const 31
          i32.and
          local.tee 3
          i32.shl
          call $dlmalloc::dlmalloc::left_bits
          local.get 2
          local.get 3
          i32.shl
          i32.and
          call $dlmalloc::dlmalloc::least_bit
          i32.ctz
          local.tee 3
          i32.const 3
          i32.shl
          local.tee 4
          i32.add
          local.tee 2
          i32.const 144
          i32.add
          local.get 2
          i32.const 152
          i32.add
          i32.load
          local.tee 2
          local.get 3
          call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_first_small_chunk
          local.get 2
          local.get 1
          call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_inuse_chunk
          local.get 2
          local.get 1
          call $dlmalloc::dlmalloc::Chunk::plus_offset
          local.tee 3
          local.get 4
          local.get 1
          i32.sub
          local.tee 1
          call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_free_chunk
          local.get 0
          local.get 3
          local.get 1
          call $dlmalloc::dlmalloc::Dlmalloc<A>::replace_dv
          local.get 2
          call $dlmalloc::dlmalloc::Chunk::to_mem
          return
        end
        local.get 0
        i32.load offset=416
        local.tee 3
        local.get 1
        i32.ge_u
        br_if 1 (;@1;)
        block ;; label = @3
          local.get 0
          i32.load offset=420
          local.tee 2
          local.get 1
          i32.gt_u
          br_if 0 (;@3;)
          local.get 0
          local.get 1
          call $dlmalloc::dlmalloc::Dlmalloc<A>::sys_alloc
          return
        end
        local.get 0
        local.get 2
        local.get 1
        i32.sub
        local.tee 3
        i32.store offset=420
        local.get 0
        local.get 0
        i32.load offset=428
        local.tee 2
        local.get 1
        call $dlmalloc::dlmalloc::Chunk::plus_offset
        local.tee 4
        i32.store offset=428
        local.get 4
        local.get 3
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 2
        local.get 1
        call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_inuse_chunk
        local.get 2
        call $dlmalloc::dlmalloc::Chunk::to_mem
        local.set 2
      end
      local.get 2
      return
    end
    local.get 0
    i32.load offset=424
    local.set 2
    block ;; label = @1
      block ;; label = @2
        local.get 3
        local.get 1
        i32.sub
        local.tee 3
        local.get 0
        call $dlmalloc::dlmalloc::Dlmalloc<A>::min_chunk_size
        i32.lt_u
        br_if 0 (;@2;)
        local.get 2
        local.get 1
        call $dlmalloc::dlmalloc::Chunk::plus_offset
        local.set 4
        local.get 0
        local.get 3
        i32.store offset=416
        local.get 0
        local.get 4
        i32.store offset=424
        local.get 4
        local.get 3
        call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_free_chunk
        local.get 2
        local.get 1
        call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_inuse_chunk
        br 1 (;@1;)
      end
      local.get 0
      i32.const 0
      i32.store offset=424
      local.get 0
      i32.load offset=416
      local.set 1
      local.get 0
      i32.const 0
      i32.store offset=416
      local.get 2
      local.get 1
      call $dlmalloc::dlmalloc::Chunk::set_inuse_and_pinuse
    end
    local.get 2
    call $dlmalloc::dlmalloc::Chunk::to_mem
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::mmap_align (;16;) (type 1) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    call $<dlmalloc::sys::System as dlmalloc::Allocator>::page_size
    call $dlmalloc::dlmalloc::align_up
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::replace_dv (;17;) (type 7) (param i32 i32 i32)
    (local i32)
    block ;; label = @1
      local.get 0
      i32.load offset=416
      local.tee 3
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 0
      i32.load offset=424
      local.get 3
      call $dlmalloc::dlmalloc::Dlmalloc<A>::insert_small_chunk
    end
    local.get 0
    local.get 1
    i32.store offset=424
    local.get 0
    local.get 2
    i32.store offset=416
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::insert_small_chunk (;18;) (type 7) (param i32 i32 i32)
    (local i32 i32)
    local.get 0
    local.get 2
    i32.const -8
    i32.and
    i32.add
    i32.const 144
    i32.add
    local.set 3
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.load offset=408
        local.tee 4
        i32.const 1
        local.get 2
        i32.const 3
        i32.shr_u
        i32.shl
        local.tee 2
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.load offset=8
        local.set 0
        br 1 (;@1;)
      end
      local.get 0
      local.get 4
      local.get 2
      i32.or
      i32.store offset=408
      local.get 3
      local.set 0
    end
    local.get 3
    local.get 1
    i32.store offset=8
    local.get 0
    local.get 1
    i32.store offset=12
    local.get 1
    local.get 3
    i32.store offset=12
    local.get 1
    local.get 0
    i32.store offset=8
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::add_segment (;19;) (type 8) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i64)
    local.get 0
    local.get 0
    i32.load offset=428
    local.tee 4
    call $dlmalloc::dlmalloc::Dlmalloc<A>::segment_holding
    call $dlmalloc::dlmalloc::Segment::top
    local.tee 5
    i32.const 20
    i32.const 8
    call $dlmalloc::dlmalloc::align_up
    local.tee 6
    i32.sub
    i32.const -23
    i32.add
    local.set 7
    local.get 4
    local.get 7
    local.get 7
    local.get 7
    call $dlmalloc::dlmalloc::Chunk::to_mem
    call $dlmalloc::dlmalloc::Dlmalloc<A>::align_offset_usize
    i32.add
    local.tee 7
    local.get 7
    local.get 4
    local.get 7
    call $dlmalloc::dlmalloc::Dlmalloc<A>::min_chunk_size
    i32.add
    i32.lt_u
    select
    local.tee 8
    call $dlmalloc::dlmalloc::Chunk::to_mem
    local.set 9
    local.get 8
    local.get 6
    call $dlmalloc::dlmalloc::Chunk::plus_offset
    local.set 7
    local.get 0
    local.get 1
    local.get 2
    local.get 7
    call $dlmalloc::dlmalloc::Dlmalloc<A>::top_foot_size
    i32.sub
    call $dlmalloc::dlmalloc::Dlmalloc<A>::init_top
    local.get 8
    local.get 6
    call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_inuse_chunk
    local.get 0
    i64.load offset=128 align=4
    local.set 10
    local.get 9
    i32.const 8
    i32.add
    local.get 0
    i32.const 136
    i32.add
    local.tee 6
    i64.load align=4
    i64.store align=4
    local.get 9
    local.get 10
    i64.store align=4
    local.get 0
    i32.const 140
    i32.add
    local.get 3
    i32.store
    local.get 0
    i32.const 132
    i32.add
    local.get 2
    i32.store
    local.get 0
    local.get 1
    i32.store offset=128
    local.get 6
    local.get 9
    i32.store
    loop ;; label = @1
      local.get 7
      i32.const 4
      call $dlmalloc::dlmalloc::Chunk::plus_offset
      local.set 9
      local.get 7
      call $dlmalloc::dlmalloc::Chunk::fencepost_head
      i32.store offset=4
      local.get 9
      local.set 7
      local.get 9
      i32.const 4
      i32.add
      local.get 5
      i32.lt_u
      br_if 0 (;@1;)
    end
    block ;; label = @1
      local.get 8
      local.get 4
      i32.eq
      br_if 0 (;@1;)
      local.get 8
      local.get 4
      i32.sub
      local.set 7
      local.get 4
      local.get 7
      local.get 4
      local.get 7
      call $dlmalloc::dlmalloc::Chunk::plus_offset
      call $dlmalloc::dlmalloc::Chunk::set_free_with_pinuse
      local.get 0
      local.get 4
      local.get 7
      call $dlmalloc::dlmalloc::Dlmalloc<A>::insert_chunk
    end
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::segment_holding (;20;) (type 1) (param i32 i32) (result i32)
    local.get 0
    i32.const 128
    i32.add
    local.set 0
    block ;; label = @1
      loop ;; label = @2
        local.get 0
        i32.eqz
        br_if 1 (;@1;)
        block ;; label = @3
          local.get 0
          i32.load
          local.get 1
          i32.gt_u
          br_if 0 (;@3;)
          local.get 0
          call $dlmalloc::dlmalloc::Segment::top
          local.get 1
          i32.gt_u
          br_if 2 (;@1;)
        end
        local.get 0
        i32.load offset=8
        local.set 0
        br 0 (;@2;)
      end
    end
    local.get 0
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::align_offset_usize (;21;) (type 1) (param i32 i32) (result i32)
    local.get 1
    i32.const 8
    call $dlmalloc::dlmalloc::align_up
    local.get 1
    i32.sub
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::min_chunk_size (;22;) (type 9) (param i32) (result i32)
    i32.const 16
    i32.const 8
    call $dlmalloc::dlmalloc::align_up
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::top_foot_size (;23;) (type 9) (param i32) (result i32)
    (local i32)
    local.get 1
    call $dlmalloc::dlmalloc::Chunk::mem_offset
    call $dlmalloc::dlmalloc::Dlmalloc<A>::align_offset_usize
    i32.const 20
    i32.const 8
    call $dlmalloc::dlmalloc::align_up
    i32.add
    local.get 1
    call $dlmalloc::dlmalloc::Dlmalloc<A>::min_chunk_size
    i32.add
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::init_top (;24;) (type 7) (param i32 i32 i32)
    (local i32)
    local.get 1
    local.get 0
    local.get 1
    call $dlmalloc::dlmalloc::Chunk::to_mem
    call $dlmalloc::dlmalloc::Dlmalloc<A>::align_offset_usize
    local.tee 3
    call $dlmalloc::dlmalloc::Chunk::plus_offset
    local.set 1
    local.get 0
    local.get 2
    local.get 3
    i32.sub
    local.tee 2
    i32.store offset=420
    local.get 0
    local.get 1
    i32.store offset=428
    local.get 1
    local.get 2
    i32.const 1
    i32.or
    i32.store offset=4
    local.get 0
    call $dlmalloc::dlmalloc::Dlmalloc<A>::top_foot_size
    local.set 3
    local.get 1
    local.get 2
    call $dlmalloc::dlmalloc::Chunk::plus_offset
    local.set 1
    local.get 0
    i32.const 2097152
    i32.store offset=440
    local.get 1
    local.get 3
    i32.store offset=4
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::insert_chunk (;25;) (type 7) (param i32 i32 i32)
    block ;; label = @1
      local.get 2
      i32.const 256
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      local.get 2
      call $dlmalloc::dlmalloc::Dlmalloc<A>::insert_large_chunk
      return
    end
    local.get 0
    local.get 1
    local.get 2
    call $dlmalloc::dlmalloc::Dlmalloc<A>::insert_small_chunk
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::max_request (;26;) (type 9) (param i32) (result i32)
    (local i32 i32)
    local.get 1
    call $dlmalloc::dlmalloc::Dlmalloc<A>::top_foot_size
    local.set 1
    i32.const 0
    local.get 1
    call $dlmalloc::dlmalloc::Dlmalloc<A>::min_chunk_size
    i32.const 2
    i32.shl
    i32.sub
    local.tee 2
    i32.const -65544
    local.get 1
    i32.sub
    i32.const -9
    i32.and
    i32.const -3
    i32.add
    local.tee 1
    local.get 2
    local.get 1
    i32.lt_u
    select
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::min_request (;27;) (type 9) (param i32) (result i32)
    (local i32)
    local.get 1
    call $dlmalloc::dlmalloc::Dlmalloc<A>::min_chunk_size
    i32.const -5
    i32.add
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::mmap_resize (;28;) (type 10) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32)
    i32.const 0
    local.set 4
    local.get 1
    call $dlmalloc::dlmalloc::Chunk::size
    local.set 5
    block ;; label = @1
      local.get 2
      i32.const 256
      i32.lt_u
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 5
        local.get 2
        i32.const 4
        i32.add
        i32.lt_u
        br_if 0 (;@2;)
        local.get 5
        local.get 2
        i32.sub
        i32.const 131073
        i32.ge_u
        br_if 0 (;@2;)
        local.get 1
        return
      end
      local.get 0
      local.get 1
      local.get 1
      i32.load
      local.tee 6
      i32.sub
      local.get 5
      local.get 6
      i32.add
      i32.const 16
      i32.add
      local.tee 5
      local.get 0
      local.get 2
      i32.const 31
      i32.add
      call $dlmalloc::dlmalloc::Dlmalloc<A>::mmap_align
      local.tee 1
      local.get 3
      call $<dlmalloc::sys::System as dlmalloc::Allocator>::remap
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.get 6
      i32.add
      local.tee 4
      local.get 1
      local.get 6
      i32.sub
      local.tee 3
      i32.const -16
      i32.add
      local.tee 6
      i32.store offset=4
      call $dlmalloc::dlmalloc::Chunk::fencepost_head
      local.set 7
      local.get 4
      local.get 6
      call $dlmalloc::dlmalloc::Chunk::plus_offset
      local.get 7
      i32.store offset=4
      local.get 4
      local.get 3
      i32.const -12
      i32.add
      call $dlmalloc::dlmalloc::Chunk::plus_offset
      i32.const 0
      i32.store offset=4
      local.get 0
      local.get 0
      i32.load offset=432
      local.get 1
      local.get 5
      i32.sub
      i32.add
      local.tee 1
      i32.store offset=432
      local.get 0
      local.get 0
      i32.load offset=444
      local.tee 5
      local.get 2
      local.get 2
      local.get 5
      i32.gt_u
      select
      i32.store offset=444
      local.get 0
      local.get 0
      i32.load offset=436
      local.tee 2
      local.get 1
      local.get 2
      local.get 1
      i32.gt_u
      select
      i32.store offset=436
    end
    local.get 4
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::pad_request (;29;) (type 1) (param i32 i32) (result i32)
    local.get 1
    i32.const 4
    i32.add
    i32.const 8
    call $dlmalloc::dlmalloc::align_up
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::insert_large_chunk (;30;) (type 7) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32)
    local.get 1
    i64.const 0
    i64.store offset=16 align=4
    local.get 1
    local.get 3
    local.get 2
    call $dlmalloc::dlmalloc::Dlmalloc<A>::compute_tree_index
    local.tee 3
    i32.store offset=28
    local.get 0
    local.get 3
    i32.const 2
    i32.shl
    i32.add
    local.set 4
    local.get 1
    call $dlmalloc::dlmalloc::TreeChunk::chunk
    local.set 5
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.load offset=412
        local.tee 6
        i32.const 1
        local.get 3
        i32.shl
        local.tee 7
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i32.load
        local.set 0
        local.get 2
        local.get 3
        call $dlmalloc::dlmalloc::leftshift_for_tree_index
        i32.shl
        local.set 3
        loop ;; label = @3
          block ;; label = @4
            local.get 0
            local.tee 4
            call $dlmalloc::dlmalloc::TreeChunk::chunk
            call $dlmalloc::dlmalloc::Chunk::size
            local.get 2
            i32.ne
            br_if 0 (;@4;)
            local.get 4
            call $dlmalloc::dlmalloc::TreeChunk::chunk
            local.tee 3
            i32.load offset=8
            local.tee 0
            local.get 5
            i32.store offset=12
            local.get 3
            local.get 5
            i32.store offset=8
            local.get 5
            local.get 3
            i32.store offset=12
            local.get 5
            local.get 0
            i32.store offset=8
            local.get 1
            i32.const 0
            i32.store offset=24
            return
          end
          local.get 3
          i32.const 29
          i32.shr_u
          local.set 0
          local.get 3
          i32.const 1
          i32.shl
          local.set 3
          local.get 4
          local.get 0
          i32.const 4
          i32.and
          i32.add
          i32.const 16
          i32.add
          local.tee 6
          i32.load
          local.tee 0
          br_if 0 (;@3;)
        end
        local.get 6
        local.get 1
        i32.store
        local.get 1
        local.get 4
        i32.store offset=24
        br 1 (;@1;)
      end
      local.get 4
      local.get 1
      i32.store
      local.get 1
      local.get 4
      i32.store offset=24
      local.get 0
      local.get 6
      local.get 7
      i32.or
      i32.store offset=412
    end
    local.get 5
    local.get 5
    i32.store offset=8
    local.get 5
    local.get 5
    i32.store offset=12
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::request2size (;31;) (type 1) (param i32 i32) (result i32)
    block ;; label = @1
      local.get 1
      call $dlmalloc::dlmalloc::Dlmalloc<A>::min_request
      local.get 1
      i32.gt_u
      br_if 0 (;@1;)
      local.get 1
      local.get 1
      call $dlmalloc::dlmalloc::Dlmalloc<A>::pad_request
      return
    end
    local.get 1
    call $dlmalloc::dlmalloc::Dlmalloc<A>::min_chunk_size
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_chunk (;32;) (type 7) (param i32 i32 i32)
    block ;; label = @1
      local.get 2
      i32.const 256
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_large_chunk
      return
    end
    local.get 0
    local.get 1
    local.get 2
    call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_small_chunk
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_large_chunk (;33;) (type 6) (param i32 i32)
    (local i32 i32 i32 i32 i32)
    local.get 1
    i32.load offset=24
    local.set 2
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          call $dlmalloc::dlmalloc::TreeChunk::next
          local.get 1
          i32.ne
          br_if 0 (;@3;)
          local.get 1
          i32.const 20
          i32.const 16
          local.get 1
          i32.const 20
          i32.add
          local.tee 3
          i32.load
          local.tee 4
          select
          i32.add
          i32.load
          local.tee 5
          br_if 1 (;@2;)
          i32.const 0
          local.set 4
          br 2 (;@1;)
        end
        local.get 1
        call $dlmalloc::dlmalloc::TreeChunk::prev
        local.tee 5
        local.get 1
        call $dlmalloc::dlmalloc::TreeChunk::next
        local.tee 4
        call $dlmalloc::dlmalloc::TreeChunk::chunk
        i32.store offset=12
        local.get 4
        local.get 5
        call $dlmalloc::dlmalloc::TreeChunk::chunk
        i32.store offset=8
        br 1 (;@1;)
      end
      local.get 3
      local.get 1
      i32.const 16
      i32.add
      local.get 4
      select
      local.set 3
      loop ;; label = @2
        local.get 3
        local.set 6
        block ;; label = @3
          local.get 5
          local.tee 4
          i32.const 20
          i32.add
          local.tee 3
          i32.load
          local.tee 5
          br_if 0 (;@3;)
          local.get 4
          i32.const 16
          i32.add
          local.set 3
          local.get 4
          i32.load offset=16
          local.set 5
        end
        local.get 5
        br_if 0 (;@2;)
      end
      local.get 6
      i32.const 0
      i32.store
    end
    block ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      block ;; label = @2
        block ;; label = @3
          local.get 0
          local.get 1
          i32.load offset=28
          local.tee 3
          i32.const 2
          i32.shl
          i32.add
          local.tee 5
          i32.load
          local.get 1
          i32.eq
          br_if 0 (;@3;)
          local.get 2
          i32.const 16
          i32.const 20
          local.get 2
          i32.load offset=16
          local.get 1
          i32.eq
          select
          i32.add
          local.get 4
          i32.store
          local.get 4
          i32.eqz
          br_if 2 (;@1;)
          br 1 (;@2;)
        end
        local.get 5
        local.get 4
        i32.store
        local.get 4
        br_if 0 (;@2;)
        local.get 0
        local.get 0
        i32.load offset=412
        i32.const -2
        local.get 3
        i32.rotl
        i32.and
        i32.store offset=412
        return
      end
      local.get 4
      local.get 2
      i32.store offset=24
      block ;; label = @2
        local.get 1
        i32.load offset=16
        local.tee 5
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        local.get 5
        i32.store offset=16
        local.get 5
        local.get 4
        i32.store offset=24
      end
      local.get 1
      i32.const 20
      i32.add
      i32.load
      local.tee 5
      i32.eqz
      br_if 0 (;@1;)
      local.get 4
      i32.const 20
      i32.add
      local.get 5
      i32.store
      local.get 5
      local.get 4
      i32.store offset=24
    end
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_small_chunk (;34;) (type 7) (param i32 i32 i32)
    (local i32)
    block ;; label = @1
      local.get 1
      i32.load offset=12
      local.tee 3
      local.get 1
      i32.load offset=8
      local.tee 1
      i32.eq
      br_if 0 (;@1;)
      local.get 1
      local.get 3
      i32.store offset=12
      local.get 3
      local.get 1
      i32.store offset=8
      return
    end
    local.get 0
    local.get 0
    i32.load offset=408
    i32.const -2
    local.get 2
    i32.const 3
    i32.shr_u
    i32.rotl
    i32.and
    i32.store offset=408
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::dispose_chunk (;35;) (type 7) (param i32 i32 i32)
    (local i32 i32)
    local.get 1
    local.get 2
    call $dlmalloc::dlmalloc::Chunk::plus_offset
    local.set 3
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          call $dlmalloc::dlmalloc::Chunk::pinuse
          br_if 0 (;@3;)
          local.get 1
          i32.load
          local.set 4
          block ;; label = @4
            block ;; label = @5
              local.get 1
              call $dlmalloc::dlmalloc::Chunk::mmapped
              br_if 0 (;@5;)
              local.get 4
              local.get 2
              i32.add
              local.set 2
              local.get 1
              local.get 4
              call $dlmalloc::dlmalloc::Chunk::minus_offset
              local.tee 1
              local.get 0
              i32.load offset=424
              i32.ne
              br_if 1 (;@4;)
              local.get 3
              i32.load offset=4
              i32.const 3
              i32.and
              i32.const 3
              i32.ne
              br_if 2 (;@3;)
              local.get 0
              local.get 2
              i32.store offset=416
              local.get 1
              local.get 2
              local.get 3
              call $dlmalloc::dlmalloc::Chunk::set_free_with_pinuse
              return
            end
            local.get 0
            local.get 1
            local.get 4
            i32.sub
            local.get 2
            local.get 4
            i32.add
            i32.const 16
            i32.add
            local.tee 1
            call $<dlmalloc::sys::System as dlmalloc::Allocator>::free
            i32.eqz
            br_if 2 (;@2;)
            local.get 0
            local.get 0
            i32.load offset=432
            local.get 1
            i32.sub
            i32.store offset=432
            return
          end
          local.get 0
          local.get 1
          local.get 4
          call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_chunk
        end
        block ;; label = @3
          local.get 3
          call $dlmalloc::dlmalloc::Chunk::cinuse
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          local.get 2
          local.get 3
          call $dlmalloc::dlmalloc::Chunk::set_free_with_pinuse
          br 2 (;@1;)
        end
        block ;; label = @3
          block ;; label = @4
            local.get 3
            local.get 0
            i32.load offset=428
            i32.eq
            br_if 0 (;@4;)
            local.get 3
            local.get 0
            i32.load offset=424
            i32.eq
            br_if 1 (;@3;)
            local.get 0
            local.get 3
            local.get 3
            call $dlmalloc::dlmalloc::Chunk::size
            local.tee 4
            call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_chunk
            local.get 1
            local.get 4
            local.get 2
            i32.add
            local.tee 2
            call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_free_chunk
            local.get 1
            local.get 0
            i32.load offset=424
            i32.ne
            br_if 3 (;@1;)
            local.get 0
            local.get 2
            i32.store offset=416
            br 2 (;@2;)
          end
          local.get 0
          local.get 1
          i32.store offset=428
          local.get 0
          local.get 0
          i32.load offset=420
          local.get 2
          i32.add
          local.tee 2
          i32.store offset=420
          local.get 1
          local.get 2
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 1
          local.get 0
          i32.load offset=424
          i32.ne
          br_if 1 (;@2;)
          local.get 0
          i32.const 0
          i32.store offset=416
          local.get 0
          i32.const 0
          i32.store offset=424
          return
        end
        local.get 0
        local.get 1
        i32.store offset=424
        local.get 0
        local.get 0
        i32.load offset=416
        local.get 2
        i32.add
        local.tee 2
        i32.store offset=416
        local.get 1
        local.get 2
        call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_free_chunk
        return
      end
      return
    end
    local.get 0
    local.get 1
    local.get 2
    call $dlmalloc::dlmalloc::Dlmalloc<A>::insert_chunk
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::prepend_alloc (;36;) (type 10) (param i32 i32 i32 i32) (result i32)
    (local i32 i32)
    local.get 2
    local.get 1
    call $dlmalloc::dlmalloc::Dlmalloc<A>::align_as_chunk
    local.set 1
    local.get 2
    local.get 2
    call $dlmalloc::dlmalloc::Dlmalloc<A>::align_as_chunk
    local.set 2
    local.get 1
    local.get 3
    call $dlmalloc::dlmalloc::Chunk::plus_offset
    local.set 4
    local.get 1
    local.get 3
    call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_inuse_chunk
    local.get 2
    local.get 1
    local.get 3
    i32.add
    i32.sub
    local.set 3
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 2
            local.get 0
            i32.load offset=428
            i32.eq
            br_if 0 (;@4;)
            local.get 2
            local.get 0
            i32.load offset=424
            i32.eq
            br_if 1 (;@3;)
            local.get 2
            call $dlmalloc::dlmalloc::Chunk::inuse
            br_if 2 (;@2;)
            local.get 0
            local.get 2
            local.get 2
            call $dlmalloc::dlmalloc::Chunk::size
            local.tee 5
            call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_chunk
            local.get 5
            local.get 3
            i32.add
            local.set 3
            local.get 2
            local.get 5
            call $dlmalloc::dlmalloc::Chunk::plus_offset
            local.set 2
            br 2 (;@2;)
          end
          local.get 0
          local.get 4
          i32.store offset=428
          local.get 0
          local.get 0
          i32.load offset=420
          local.get 3
          i32.add
          local.tee 2
          i32.store offset=420
          local.get 4
          local.get 2
          i32.const 1
          i32.or
          i32.store offset=4
          br 2 (;@1;)
        end
        local.get 0
        local.get 4
        i32.store offset=424
        local.get 0
        local.get 0
        i32.load offset=416
        local.get 3
        i32.add
        local.tee 2
        i32.store offset=416
        local.get 4
        local.get 2
        call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_free_chunk
        br 1 (;@1;)
      end
      local.get 4
      local.get 3
      local.get 2
      call $dlmalloc::dlmalloc::Chunk::set_free_with_pinuse
      local.get 0
      local.get 4
      local.get 3
      call $dlmalloc::dlmalloc::Dlmalloc<A>::insert_chunk
    end
    local.get 1
    call $dlmalloc::dlmalloc::Chunk::to_mem
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::align_as_chunk (;37;) (type 1) (param i32 i32) (result i32)
    local.get 1
    local.get 1
    local.get 1
    call $dlmalloc::dlmalloc::Chunk::to_mem
    call $dlmalloc::dlmalloc::Dlmalloc<A>::align_offset_usize
    i32.add
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::tmalloc_large (;38;) (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    i32.const 0
    local.set 2
    i32.const 0
    local.get 1
    i32.sub
    local.set 3
    block ;; label = @1
      block ;; label = @2
        local.get 0
        local.get 4
        local.get 1
        call $dlmalloc::dlmalloc::Dlmalloc<A>::compute_tree_index
        local.tee 5
        i32.const 2
        i32.shl
        i32.add
        i32.load
        local.tee 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        local.get 5
        call $dlmalloc::dlmalloc::leftshift_for_tree_index
        i32.shl
        local.set 6
        i32.const 0
        local.set 7
        loop ;; label = @3
          block ;; label = @4
            local.get 4
            call $dlmalloc::dlmalloc::TreeChunk::chunk
            call $dlmalloc::dlmalloc::Chunk::size
            local.tee 8
            local.get 1
            i32.lt_u
            br_if 0 (;@4;)
            local.get 8
            local.get 1
            i32.sub
            local.tee 8
            local.get 3
            i32.ge_u
            br_if 0 (;@4;)
            local.get 8
            local.set 3
            local.get 4
            local.set 7
            local.get 8
            br_if 0 (;@4;)
            i32.const 0
            local.set 3
            local.get 4
            local.set 7
            br 3 (;@1;)
          end
          local.get 4
          i32.const 20
          i32.add
          i32.load
          local.tee 8
          local.get 2
          local.get 8
          local.get 4
          local.get 6
          i32.const 29
          i32.shr_u
          i32.const 4
          i32.and
          i32.add
          i32.const 16
          i32.add
          i32.load
          local.tee 4
          i32.ne
          select
          local.get 2
          local.get 8
          select
          local.set 2
          local.get 6
          i32.const 1
          i32.shl
          local.set 6
          local.get 4
          br_if 0 (;@3;)
        end
        block ;; label = @3
          local.get 2
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          local.set 4
          br 2 (;@1;)
        end
        i32.const 0
        local.set 4
        local.get 7
        br_if 1 (;@1;)
      end
      i32.const 0
      local.set 7
      block ;; label = @2
        i32.const 1
        local.get 5
        i32.shl
        call $dlmalloc::dlmalloc::left_bits
        local.get 0
        i32.load offset=412
        i32.and
        local.tee 4
        br_if 0 (;@2;)
        i32.const 0
        local.set 4
        br 1 (;@1;)
      end
      local.get 0
      local.get 4
      call $dlmalloc::dlmalloc::least_bit
      i32.ctz
      i32.const 2
      i32.shl
      i32.add
      i32.load
      local.set 4
    end
    loop (result i32) ;; label = @1
      block ;; label = @2
        local.get 4
        br_if 0 (;@2;)
        i32.const 0
        local.set 4
        block ;; label = @3
          local.get 7
          i32.eqz
          br_if 0 (;@3;)
          block ;; label = @4
            local.get 0
            i32.load offset=416
            local.tee 2
            local.get 1
            i32.lt_u
            br_if 0 (;@4;)
            local.get 3
            local.get 2
            local.get 1
            i32.sub
            i32.ge_u
            br_if 1 (;@3;)
          end
          local.get 7
          call $dlmalloc::dlmalloc::TreeChunk::chunk
          local.tee 4
          local.get 1
          call $dlmalloc::dlmalloc::Chunk::plus_offset
          local.set 2
          local.get 0
          local.get 7
          call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_large_chunk
          block ;; label = @4
            block ;; label = @5
              local.get 3
              local.get 4
              call $dlmalloc::dlmalloc::Dlmalloc<A>::min_chunk_size
              i32.lt_u
              br_if 0 (;@5;)
              local.get 4
              local.get 1
              call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_inuse_chunk
              local.get 2
              local.get 3
              call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_free_chunk
              local.get 0
              local.get 2
              local.get 3
              call $dlmalloc::dlmalloc::Dlmalloc<A>::insert_chunk
              br 1 (;@4;)
            end
            local.get 4
            local.get 3
            local.get 1
            i32.add
            call $dlmalloc::dlmalloc::Chunk::set_inuse_and_pinuse
          end
          local.get 4
          call $dlmalloc::dlmalloc::Chunk::to_mem
          local.set 4
        end
        local.get 4
        return
      end
      local.get 4
      local.get 7
      local.get 4
      call $dlmalloc::dlmalloc::TreeChunk::chunk
      call $dlmalloc::dlmalloc::Chunk::size
      local.tee 2
      local.get 1
      i32.ge_u
      local.get 2
      local.get 1
      i32.sub
      local.tee 2
      local.get 3
      i32.lt_u
      i32.and
      local.tee 6
      select
      local.set 7
      local.get 2
      local.get 3
      local.get 6
      select
      local.set 3
      local.get 4
      call $dlmalloc::dlmalloc::TreeChunk::leftmost_child
      local.set 4
      br 0 (;@1;)
    end
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::compute_tree_index (;39;) (type 1) (param i32 i32) (result i32)
    (local i32)
    i32.const 0
    local.set 2
    block ;; label = @1
      local.get 1
      i32.const 256
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 31
      local.set 2
      local.get 1
      i32.const 16777216
      i32.ge_u
      br_if 0 (;@1;)
      local.get 1
      i32.const 6
      local.get 1
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
    local.get 2
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::tmalloc_small (;40;) (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32 i32)
    local.get 0
    local.get 0
    i32.load offset=412
    call $dlmalloc::dlmalloc::least_bit
    i32.ctz
    i32.const 2
    i32.shl
    i32.add
    i32.load
    local.tee 2
    call $dlmalloc::dlmalloc::TreeChunk::chunk
    call $dlmalloc::dlmalloc::Chunk::size
    local.get 1
    i32.sub
    local.set 3
    local.get 2
    local.set 4
    block ;; label = @1
      loop ;; label = @2
        local.get 2
        call $dlmalloc::dlmalloc::TreeChunk::leftmost_child
        local.tee 2
        i32.eqz
        br_if 1 (;@1;)
        local.get 2
        call $dlmalloc::dlmalloc::TreeChunk::chunk
        call $dlmalloc::dlmalloc::Chunk::size
        local.get 1
        i32.sub
        local.tee 5
        local.get 3
        local.get 5
        local.get 3
        i32.lt_u
        local.tee 5
        select
        local.set 3
        local.get 2
        local.get 4
        local.get 5
        select
        local.set 4
        br 0 (;@2;)
      end
    end
    local.get 4
    call $dlmalloc::dlmalloc::TreeChunk::chunk
    local.tee 2
    local.get 1
    call $dlmalloc::dlmalloc::Chunk::plus_offset
    local.set 5
    local.get 0
    local.get 4
    call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_large_chunk
    block ;; label = @1
      block ;; label = @2
        local.get 3
        local.get 2
        call $dlmalloc::dlmalloc::Dlmalloc<A>::min_chunk_size
        i32.lt_u
        br_if 0 (;@2;)
        local.get 5
        call $dlmalloc::dlmalloc::TreeChunk::chunk
        local.set 4
        local.get 2
        local.get 1
        call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_inuse_chunk
        local.get 4
        local.get 3
        call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_free_chunk
        local.get 0
        local.get 4
        local.get 3
        call $dlmalloc::dlmalloc::Dlmalloc<A>::replace_dv
        br 1 (;@1;)
      end
      local.get 2
      local.get 3
      local.get 1
      i32.add
      call $dlmalloc::dlmalloc::Chunk::set_inuse_and_pinuse
    end
    local.get 2
    call $dlmalloc::dlmalloc::Chunk::to_mem
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::has_segment_link (;41;) (type 1) (param i32 i32) (result i32)
    local.get 0
    i32.const 128
    i32.add
    local.set 0
    loop (result i32) ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 0
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          local.get 0
          call $dlmalloc::dlmalloc::Segment::holds
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 0
        i32.const 0
        i32.ne
        return
      end
      local.get 0
      i32.load offset=8
      local.set 0
      br 0 (;@1;)
    end
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::calloc_must_clear (;42;) (type 1) (param i32 i32) (result i32)
    (local i32)
    i32.const 1
    local.set 2
    block ;; label = @1
      local.get 0
      call $<dlmalloc::sys::System as dlmalloc::Allocator>::allocates_zeros
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      call $dlmalloc::dlmalloc::Chunk::from_mem
      call $dlmalloc::dlmalloc::Chunk::mmapped
      i32.const 1
      i32.xor
      local.set 2
    end
    local.get 2
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::try_realloc_chunk (;43;) (type 10) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32)
    local.get 1
    local.get 1
    call $dlmalloc::dlmalloc::Chunk::size
    local.tee 4
    call $dlmalloc::dlmalloc::Chunk::plus_offset
    local.set 5
    block ;; label = @1
      local.get 1
      call $dlmalloc::dlmalloc::Chunk::mmapped
      br_if 0 (;@1;)
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 4
                  local.get 2
                  i32.ge_u
                  br_if 0 (;@7;)
                  local.get 5
                  local.get 0
                  i32.load offset=428
                  i32.eq
                  br_if 3 (;@4;)
                  local.get 5
                  local.get 0
                  i32.load offset=424
                  i32.eq
                  br_if 2 (;@5;)
                  i32.const 0
                  local.set 3
                  local.get 5
                  call $dlmalloc::dlmalloc::Chunk::cinuse
                  br_if 5 (;@2;)
                  local.get 5
                  call $dlmalloc::dlmalloc::Chunk::size
                  local.tee 6
                  local.get 4
                  i32.add
                  local.tee 4
                  local.get 2
                  i32.lt_u
                  br_if 5 (;@2;)
                  local.get 0
                  local.get 5
                  local.get 6
                  call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_chunk
                  local.get 4
                  local.get 2
                  i32.sub
                  local.tee 5
                  local.get 1
                  call $dlmalloc::dlmalloc::Dlmalloc<A>::min_chunk_size
                  i32.lt_u
                  br_if 1 (;@6;)
                  local.get 1
                  local.get 2
                  call $dlmalloc::dlmalloc::Chunk::plus_offset
                  local.set 4
                  local.get 1
                  local.get 2
                  call $dlmalloc::dlmalloc::Chunk::set_inuse
                  local.get 4
                  local.get 5
                  call $dlmalloc::dlmalloc::Chunk::set_inuse
                  local.get 0
                  local.get 4
                  local.get 5
                  call $dlmalloc::dlmalloc::Dlmalloc<A>::dispose_chunk
                  br 4 (;@3;)
                end
                local.get 4
                local.get 2
                i32.sub
                local.tee 4
                local.get 1
                call $dlmalloc::dlmalloc::Dlmalloc<A>::min_chunk_size
                i32.lt_u
                br_if 3 (;@3;)
                local.get 1
                local.get 2
                call $dlmalloc::dlmalloc::Chunk::plus_offset
                local.set 5
                local.get 1
                local.get 2
                call $dlmalloc::dlmalloc::Chunk::set_inuse
                local.get 5
                local.get 4
                call $dlmalloc::dlmalloc::Chunk::set_inuse
                local.get 0
                local.get 5
                local.get 4
                call $dlmalloc::dlmalloc::Dlmalloc<A>::dispose_chunk
                br 3 (;@3;)
              end
              local.get 1
              local.get 4
              call $dlmalloc::dlmalloc::Chunk::set_inuse
              br 2 (;@3;)
            end
            i32.const 0
            local.set 3
            local.get 0
            i32.load offset=416
            local.get 4
            i32.add
            local.tee 4
            local.get 2
            i32.lt_u
            br_if 2 (;@2;)
            block ;; label = @5
              block ;; label = @6
                local.get 4
                local.get 2
                i32.sub
                local.tee 5
                local.get 1
                call $dlmalloc::dlmalloc::Dlmalloc<A>::min_chunk_size
                i32.ge_u
                br_if 0 (;@6;)
                local.get 1
                local.get 4
                call $dlmalloc::dlmalloc::Chunk::set_inuse
                i32.const 0
                local.set 5
                i32.const 0
                local.set 4
                br 1 (;@5;)
              end
              local.get 1
              local.get 2
              call $dlmalloc::dlmalloc::Chunk::plus_offset
              local.tee 4
              local.get 5
              call $dlmalloc::dlmalloc::Chunk::plus_offset
              local.set 3
              local.get 1
              local.get 2
              call $dlmalloc::dlmalloc::Chunk::set_inuse
              local.get 4
              local.get 5
              call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_free_chunk
              local.get 3
              call $dlmalloc::dlmalloc::Chunk::clear_pinuse
            end
            local.get 0
            local.get 4
            i32.store offset=424
            local.get 0
            local.get 5
            i32.store offset=416
            br 1 (;@3;)
          end
          i32.const 0
          local.set 3
          local.get 0
          i32.load offset=420
          local.get 4
          i32.add
          local.tee 4
          local.get 2
          i32.le_u
          br_if 1 (;@2;)
          local.get 1
          local.get 2
          call $dlmalloc::dlmalloc::Chunk::plus_offset
          local.set 5
          local.get 1
          local.get 2
          call $dlmalloc::dlmalloc::Chunk::set_inuse
          local.get 5
          local.get 4
          local.get 2
          i32.sub
          local.tee 2
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 0
          local.get 2
          i32.store offset=420
          local.get 0
          local.get 5
          i32.store offset=428
        end
        local.get 1
        local.set 3
      end
      local.get 3
      return
    end
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    call $dlmalloc::dlmalloc::Dlmalloc<A>::mmap_resize
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::release_unused_segments (;44;) (type 9) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    local.get 0
    i32.const 128
    i32.add
    local.set 1
    local.get 0
    i32.const 136
    i32.add
    i32.load
    local.set 2
    i32.const 0
    local.set 3
    i32.const 0
    local.set 4
    loop ;; label = @1
      local.get 1
      local.set 5
      block ;; label = @2
        loop ;; label = @3
          local.get 2
          local.tee 1
          i32.eqz
          br_if 1 (;@2;)
          local.get 1
          i32.load offset=8
          local.set 2
          local.get 1
          i32.load offset=4
          local.set 6
          local.get 1
          i32.load
          local.set 7
          block ;; label = @4
            block ;; label = @5
              local.get 0
              local.get 1
              call $dlmalloc::dlmalloc::Segment::can_release_part
              i32.eqz
              br_if 0 (;@5;)
              local.get 1
              call $dlmalloc::dlmalloc::Segment::is_extern
              br_if 0 (;@5;)
              local.get 1
              local.get 7
              call $dlmalloc::dlmalloc::Dlmalloc<A>::align_as_chunk
              local.tee 8
              call $dlmalloc::dlmalloc::Chunk::size
              local.set 9
              local.get 1
              call $dlmalloc::dlmalloc::Dlmalloc<A>::top_foot_size
              local.set 10
              local.get 8
              call $dlmalloc::dlmalloc::Chunk::inuse
              br_if 0 (;@5;)
              local.get 8
              local.get 9
              i32.add
              local.get 7
              local.get 6
              local.get 10
              i32.sub
              i32.add
              i32.lt_u
              br_if 0 (;@5;)
              block ;; label = @6
                block ;; label = @7
                  local.get 8
                  local.get 0
                  i32.load offset=424
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 0
                  local.get 8
                  call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_large_chunk
                  br 1 (;@6;)
                end
                local.get 0
                i32.const 0
                i32.store offset=416
                local.get 0
                i32.const 0
                i32.store offset=424
              end
              local.get 0
              local.get 7
              local.get 6
              call $<dlmalloc::sys::System as dlmalloc::Allocator>::free
              br_if 1 (;@4;)
              local.get 0
              local.get 8
              local.get 9
              call $dlmalloc::dlmalloc::Dlmalloc<A>::insert_large_chunk
            end
            local.get 3
            i32.const 1
            i32.add
            local.set 3
            br 3 (;@1;)
          end
          local.get 0
          local.get 0
          i32.load offset=432
          local.get 6
          i32.sub
          i32.store offset=432
          local.get 5
          local.get 2
          i32.store offset=8
          local.get 3
          i32.const 1
          i32.add
          local.set 3
          local.get 6
          local.get 4
          i32.add
          local.set 4
          br 0 (;@3;)
        end
      end
    end
    local.get 0
    local.get 3
    i32.const 4095
    local.get 3
    i32.const 4095
    i32.gt_u
    select
    i32.store offset=448
    local.get 4
  )
  (func $dlmalloc::dlmalloc::Segment::can_release_part (;45;) (type 1) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.load offset=12
    i32.const 1
    i32.shr_u
    call $<dlmalloc::sys::System as dlmalloc::Allocator>::can_release_part
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_first_small_chunk (;46;) (type 8) (param i32 i32 i32 i32)
    block ;; label = @1
      local.get 2
      i32.load offset=8
      local.tee 2
      local.get 1
      i32.eq
      br_if 0 (;@1;)
      local.get 2
      local.get 1
      i32.store offset=12
      local.get 1
      local.get 2
      i32.store offset=8
      return
    end
    local.get 0
    local.get 0
    i32.load offset=408
    i32.const -2
    local.get 3
    i32.rotl
    i32.and
    i32.store offset=408
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::free (;47;) (type 6) (param i32 i32)
    (local i32 i32 i32)
    local.get 1
    call $dlmalloc::dlmalloc::Chunk::from_mem
    local.set 1
    local.get 1
    local.get 1
    call $dlmalloc::dlmalloc::Chunk::size
    local.tee 2
    call $dlmalloc::dlmalloc::Chunk::plus_offset
    local.set 3
    block ;; label = @1
      block ;; label = @2
        local.get 1
        call $dlmalloc::dlmalloc::Chunk::pinuse
        br_if 0 (;@2;)
        local.get 1
        i32.load
        local.set 4
        block ;; label = @3
          block ;; label = @4
            local.get 1
            call $dlmalloc::dlmalloc::Chunk::mmapped
            br_if 0 (;@4;)
            local.get 4
            local.get 2
            i32.add
            local.set 2
            local.get 1
            local.get 4
            call $dlmalloc::dlmalloc::Chunk::minus_offset
            local.tee 1
            local.get 0
            i32.load offset=424
            i32.ne
            br_if 1 (;@3;)
            local.get 3
            i32.load offset=4
            i32.const 3
            i32.and
            i32.const 3
            i32.ne
            br_if 2 (;@2;)
            local.get 0
            local.get 2
            i32.store offset=416
            local.get 1
            local.get 2
            local.get 3
            call $dlmalloc::dlmalloc::Chunk::set_free_with_pinuse
            return
          end
          local.get 0
          local.get 1
          local.get 4
          i32.sub
          local.get 2
          local.get 4
          i32.add
          i32.const 16
          i32.add
          local.tee 1
          call $<dlmalloc::sys::System as dlmalloc::Allocator>::free
          i32.eqz
          br_if 2 (;@1;)
          local.get 0
          local.get 0
          i32.load offset=432
          local.get 1
          i32.sub
          i32.store offset=432
          return
        end
        local.get 0
        local.get 1
        local.get 4
        call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_chunk
      end
      block ;; label = @2
        block ;; label = @3
          local.get 3
          call $dlmalloc::dlmalloc::Chunk::cinuse
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          local.get 2
          local.get 3
          call $dlmalloc::dlmalloc::Chunk::set_free_with_pinuse
          br 1 (;@2;)
        end
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 3
                local.get 0
                i32.load offset=428
                i32.eq
                br_if 0 (;@6;)
                local.get 3
                local.get 0
                i32.load offset=424
                i32.eq
                br_if 1 (;@5;)
                local.get 0
                local.get 3
                local.get 3
                call $dlmalloc::dlmalloc::Chunk::size
                local.tee 4
                call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_chunk
                local.get 1
                local.get 4
                local.get 2
                i32.add
                local.tee 2
                call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_free_chunk
                local.get 1
                local.get 0
                i32.load offset=424
                i32.ne
                br_if 4 (;@2;)
                local.get 0
                local.get 2
                i32.store offset=416
                return
              end
              local.get 0
              local.get 1
              i32.store offset=428
              local.get 0
              local.get 0
              i32.load offset=420
              local.get 2
              i32.add
              local.tee 2
              i32.store offset=420
              local.get 1
              local.get 2
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 1
              local.get 0
              i32.load offset=424
              i32.eq
              br_if 1 (;@4;)
              br 2 (;@3;)
            end
            local.get 0
            local.get 1
            i32.store offset=424
            local.get 0
            local.get 0
            i32.load offset=416
            local.get 2
            i32.add
            local.tee 2
            i32.store offset=416
            local.get 1
            local.get 2
            call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_free_chunk
            return
          end
          local.get 0
          i32.const 0
          i32.store offset=416
          local.get 0
          i32.const 0
          i32.store offset=424
        end
        local.get 0
        i32.load offset=440
        local.get 2
        i32.ge_u
        br_if 1 (;@1;)
        local.get 0
        i32.const 0
        call $dlmalloc::dlmalloc::Dlmalloc<A>::sys_trim
        drop
        return
      end
      block ;; label = @2
        local.get 2
        i32.const 256
        i32.lt_u
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        local.get 2
        call $dlmalloc::dlmalloc::Dlmalloc<A>::insert_large_chunk
        local.get 0
        local.get 0
        i32.load offset=448
        i32.const -1
        i32.add
        local.tee 1
        i32.store offset=448
        local.get 1
        br_if 1 (;@1;)
        local.get 0
        call $dlmalloc::dlmalloc::Dlmalloc<A>::release_unused_segments
        drop
        return
      end
      local.get 0
      local.get 1
      local.get 2
      call $dlmalloc::dlmalloc::Dlmalloc<A>::insert_small_chunk
    end
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::sys_trim (;48;) (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32 i32)
    i32.const 0
    local.set 2
    block ;; label = @1
      local.get 0
      call $dlmalloc::dlmalloc::Dlmalloc<A>::max_request
      local.get 1
      i32.le_u
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=428
      local.tee 3
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      call $dlmalloc::dlmalloc::Dlmalloc<A>::top_foot_size
      local.set 4
      i32.const 0
      local.set 2
      block ;; label = @2
        local.get 0
        i32.load offset=420
        local.tee 5
        local.get 4
        local.get 1
        i32.add
        local.tee 1
        i32.le_u
        br_if 0 (;@2;)
        local.get 0
        local.get 3
        call $dlmalloc::dlmalloc::Dlmalloc<A>::segment_holding
        local.tee 3
        call $dlmalloc::dlmalloc::Segment::is_extern
        br_if 0 (;@2;)
        local.get 0
        local.get 3
        call $dlmalloc::dlmalloc::Segment::can_release_part
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.load offset=4
        local.get 5
        local.get 1
        i32.sub
        i32.const 65535
        i32.add
        i32.const -65536
        i32.and
        i32.const -65536
        i32.add
        local.tee 1
        i32.lt_u
        br_if 0 (;@2;)
        local.get 0
        local.get 3
        call $dlmalloc::dlmalloc::Dlmalloc<A>::has_segment_link
        br_if 0 (;@2;)
        local.get 0
        local.get 3
        i32.load
        local.get 3
        i32.load offset=4
        local.tee 4
        local.get 4
        local.get 1
        i32.sub
        call $<dlmalloc::sys::System as dlmalloc::Allocator>::free_part
        local.set 4
        local.get 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        local.get 3
        i32.load offset=4
        local.get 1
        i32.sub
        i32.store offset=4
        local.get 0
        local.get 0
        i32.load offset=432
        local.get 1
        i32.sub
        i32.store offset=432
        local.get 0
        local.get 0
        i32.load offset=428
        local.get 0
        i32.load offset=420
        local.get 1
        i32.sub
        call $dlmalloc::dlmalloc::Dlmalloc<A>::init_top
        local.get 1
        local.set 2
      end
      block ;; label = @2
        local.get 0
        call $dlmalloc::dlmalloc::Dlmalloc<A>::release_unused_segments
        local.get 2
        i32.add
        local.tee 1
        br_if 0 (;@2;)
        local.get 0
        i32.load offset=420
        local.get 0
        i32.load offset=440
        i32.le_u
        br_if 0 (;@2;)
        local.get 0
        i32.const -1
        i32.store offset=440
      end
      local.get 1
      i32.const 0
      i32.ne
      local.set 2
    end
    local.get 2
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::sys_alloc (;49;) (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 4
    i32.add
    local.get 0
    local.get 1
    local.get 3
    call $dlmalloc::dlmalloc::Dlmalloc<A>::top_foot_size
    i32.add
    i32.const 8
    i32.add
    i32.const 65536
    call $dlmalloc::dlmalloc::align_up
    call $<dlmalloc::sys::System as dlmalloc::Allocator>::alloc
    i32.const 0
    local.set 4
    block ;; label = @1
      local.get 2
      i32.load offset=4
      local.tee 5
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i32.load offset=12
      local.set 6
      local.get 0
      local.get 0
      i32.load offset=432
      local.get 2
      i32.load offset=8
      local.tee 7
      i32.add
      local.tee 3
      i32.store offset=432
      local.get 0
      local.get 0
      i32.load offset=436
      local.tee 8
      local.get 3
      local.get 8
      local.get 3
      i32.gt_u
      select
      i32.store offset=436
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    local.get 0
                    i32.load offset=428
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 0
                    i32.const 128
                    i32.add
                    local.tee 8
                    local.set 3
                    loop ;; label = @9
                      local.get 3
                      i32.eqz
                      br_if 3 (;@6;)
                      local.get 5
                      local.get 3
                      call $dlmalloc::dlmalloc::Segment::top
                      i32.eq
                      br_if 2 (;@7;)
                      local.get 3
                      i32.load offset=8
                      local.set 3
                      br 0 (;@9;)
                    end
                  end
                  local.get 0
                  i32.load offset=444
                  local.tee 3
                  i32.eqz
                  br_if 3 (;@4;)
                  local.get 5
                  local.get 3
                  i32.lt_u
                  br_if 3 (;@4;)
                  br 4 (;@3;)
                end
                local.get 3
                call $dlmalloc::dlmalloc::Segment::is_extern
                br_if 0 (;@6;)
                local.get 3
                call $dlmalloc::dlmalloc::Segment::sys_flags
                local.get 6
                i32.ne
                br_if 0 (;@6;)
                local.get 3
                local.get 0
                i32.load offset=428
                call $dlmalloc::dlmalloc::Segment::holds
                br_if 1 (;@5;)
              end
              local.get 0
              local.get 0
              i32.load offset=444
              local.tee 3
              local.get 5
              local.get 5
              local.get 3
              i32.gt_u
              select
              i32.store offset=444
              local.get 5
              local.get 7
              i32.add
              local.set 3
              block ;; label = @6
                block ;; label = @7
                  loop ;; label = @8
                    local.get 8
                    i32.eqz
                    br_if 1 (;@7;)
                    block ;; label = @9
                      local.get 8
                      i32.load
                      local.get 3
                      i32.eq
                      br_if 0 (;@9;)
                      local.get 8
                      i32.load offset=8
                      local.set 8
                      br 1 (;@8;)
                    end
                  end
                  local.get 8
                  call $dlmalloc::dlmalloc::Segment::is_extern
                  br_if 0 (;@7;)
                  local.get 8
                  call $dlmalloc::dlmalloc::Segment::sys_flags
                  local.get 6
                  i32.eq
                  br_if 1 (;@6;)
                end
                local.get 0
                local.get 5
                local.get 7
                local.get 6
                call $dlmalloc::dlmalloc::Dlmalloc<A>::add_segment
                br 4 (;@2;)
              end
              local.get 8
              i32.load
              local.set 3
              local.get 8
              local.get 5
              i32.store
              local.get 8
              local.get 8
              i32.load offset=4
              local.get 7
              i32.add
              i32.store offset=4
              local.get 0
              local.get 5
              local.get 3
              local.get 1
              call $dlmalloc::dlmalloc::Dlmalloc<A>::prepend_alloc
              local.set 4
              br 4 (;@1;)
            end
            local.get 3
            local.get 3
            i32.load offset=4
            local.get 7
            i32.add
            i32.store offset=4
            local.get 0
            local.get 0
            i32.load offset=428
            local.get 0
            i32.load offset=420
            local.get 7
            i32.add
            call $dlmalloc::dlmalloc::Dlmalloc<A>::init_top
            br 2 (;@2;)
          end
          local.get 0
          local.get 5
          i32.store offset=444
        end
        local.get 0
        i32.const 4095
        i32.store offset=448
        local.get 0
        local.get 5
        i32.store offset=128
        local.get 0
        i32.const 140
        i32.add
        local.get 6
        i32.store
        local.get 0
        i32.const 132
        i32.add
        local.get 7
        i32.store
        local.get 0
        call $dlmalloc::dlmalloc::Dlmalloc<A>::init_bins
        local.get 0
        local.get 5
        local.get 7
        local.get 3
        call $dlmalloc::dlmalloc::Dlmalloc<A>::top_foot_size
        i32.sub
        call $dlmalloc::dlmalloc::Dlmalloc<A>::init_top
      end
      local.get 0
      i32.load offset=420
      local.tee 3
      local.get 1
      i32.le_u
      br_if 0 (;@1;)
      local.get 0
      local.get 3
      local.get 1
      i32.sub
      local.tee 8
      i32.store offset=420
      local.get 0
      local.get 0
      i32.load offset=428
      local.tee 3
      local.get 1
      call $dlmalloc::dlmalloc::Chunk::plus_offset
      local.tee 5
      i32.store offset=428
      local.get 5
      local.get 8
      i32.const 1
      i32.or
      i32.store offset=4
      local.get 3
      local.get 1
      call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_inuse_chunk
      local.get 3
      call $dlmalloc::dlmalloc::Chunk::to_mem
      local.set 4
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 4
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::realloc (;50;) (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    i32.const 0
    local.set 3
    block ;; label = @1
      block ;; label = @2
        local.get 0
        call $dlmalloc::dlmalloc::Dlmalloc<A>::max_request
        local.get 2
        i32.le_u
        br_if 0 (;@2;)
        local.get 2
        local.get 2
        call $dlmalloc::dlmalloc::Dlmalloc<A>::request2size
        local.set 4
        local.get 0
        local.get 1
        call $dlmalloc::dlmalloc::Chunk::from_mem
        local.tee 5
        local.get 4
        i32.const 1
        call $dlmalloc::dlmalloc::Dlmalloc<A>::try_realloc_chunk
        local.tee 4
        br_if 1 (;@1;)
        local.get 0
        local.get 2
        call $dlmalloc::dlmalloc::Dlmalloc<A>::malloc
        local.tee 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        local.get 1
        local.get 5
        call $dlmalloc::dlmalloc::Chunk::size
        i32.const -8
        i32.const -4
        local.get 5
        call $dlmalloc::dlmalloc::Chunk::mmapped
        select
        i32.add
        local.tee 3
        local.get 2
        local.get 3
        local.get 2
        i32.lt_u
        select
        call $memcpy
        local.set 2
        local.get 0
        local.get 1
        call $dlmalloc::dlmalloc::Dlmalloc<A>::free
        local.get 2
        local.set 3
      end
      local.get 3
      return
    end
    local.get 4
    call $dlmalloc::dlmalloc::Chunk::mmapped
    drop
    local.get 4
    call $dlmalloc::dlmalloc::Chunk::to_mem
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::init_bins (;51;) (type 3) (param i32)
    (local i32 i32 i32)
    i32.const 0
    local.set 1
    loop ;; label = @1
      block ;; label = @2
        local.get 1
        i32.const 256
        i32.ne
        br_if 0 (;@2;)
        return
      end
      local.get 0
      local.get 1
      i32.add
      local.tee 2
      i32.const 152
      i32.add
      local.get 2
      i32.const 144
      i32.add
      local.tee 3
      i32.store
      local.get 2
      i32.const 156
      i32.add
      local.get 3
      i32.store
      local.get 1
      i32.const 8
      i32.add
      local.set 1
      br 0 (;@1;)
    end
  )
  (func $rust_begin_unwind (;52;) (type 3) (param i32)
    loop ;; label = @1
      br 0 (;@1;)
    end
  )
  (func $basic_wallet::MyWallet::receive_asset (;53;) (type 6) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 8
    i32.add
    local.get 1
    call $<miden::asset::Asset as miden::serialization::FeltSerialize>::to_felts
    block ;; label = @1
      local.get 2
      i32.load offset=16
      i32.const 16
      i32.lt_u
      br_if 0 (;@1;)
      local.get 2
      i32.const 32
      i32.add
      i64.const 0
      i64.store align=4
      local.get 2
      i32.const 1
      i32.store offset=24
      local.get 2
      i32.const 1048616
      i32.store offset=20
      local.get 2
      local.get 2
      i32.const 44
      i32.add
      i32.store offset=28
      local.get 2
      i32.const 20
      i32.add
      i32.const 1048636
      call $core::panicking::panic_fmt
      unreachable
    end
    i64.const 1
    call $miden::note::Tag::new
    local.get 2
    i32.const 8
    i32.add
    i32.const 0
    i32.const 1048652
    call $<alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
    i64.load
    local.get 2
    i32.const 8
    i32.add
    i32.const 1
    i32.const 1048668
    call $<alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
    i64.load
    i64.const 0
    call $miden::note::Tag::new
    call $my_wallet::receive_asset
    local.get 2
    i32.const 8
    i32.add
    call $<alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
    local.get 2
    i32.const 48
    i32.add
    global.set $__stack_pointer
  )
  (func $__rust_alloc (;54;) (type 1) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    call $dlmalloc::sys::enable_alloc_after_fork
    local.get 2
    i32.const 15
    i32.add
    call $<dlmalloc::global::Instance as core::ops::deref::DerefMut>::deref_mut
    local.get 0
    local.get 1
    call $dlmalloc::Dlmalloc<A>::malloc
    local.set 1
    local.get 2
    i32.const 15
    i32.add
    call $<dlmalloc::global::Instance as core::ops::drop::Drop>::drop
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $__rust_realloc (;55;) (type 10) (param i32 i32 i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    call $dlmalloc::sys::enable_alloc_after_fork
    local.get 4
    i32.const 15
    i32.add
    call $<dlmalloc::global::Instance as core::ops::deref::DerefMut>::deref_mut
    local.set 5
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 2
          i32.const 9
          i32.lt_u
          br_if 0 (;@3;)
          local.get 5
          local.get 3
          local.get 2
          call $dlmalloc::Dlmalloc<A>::malloc
          local.tee 2
          br_if 1 (;@2;)
          i32.const 0
          local.set 2
          br 2 (;@1;)
        end
        local.get 5
        local.get 0
        local.get 3
        call $dlmalloc::dlmalloc::Dlmalloc<A>::realloc
        local.set 2
        br 1 (;@1;)
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
      drop
      local.get 5
      local.get 0
      call $dlmalloc::dlmalloc::Dlmalloc<A>::free
    end
    local.get 4
    i32.const 15
    i32.add
    call $<dlmalloc::global::Instance as core::ops::drop::Drop>::drop
    local.get 4
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 2
  )
  (func $__rust_alloc_zeroed (;56;) (type 1) (param i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    call $dlmalloc::sys::enable_alloc_after_fork
    block ;; label = @1
      local.get 2
      i32.const 15
      i32.add
      call $<dlmalloc::global::Instance as core::ops::deref::DerefMut>::deref_mut
      local.tee 3
      local.get 0
      local.get 1
      call $dlmalloc::Dlmalloc<A>::malloc
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      local.get 1
      call $dlmalloc::dlmalloc::Dlmalloc<A>::calloc_must_clear
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const 0
      local.get 0
      call $memset
      drop
    end
    local.get 2
    i32.const 15
    i32.add
    call $<dlmalloc::global::Instance as core::ops::drop::Drop>::drop
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $dlmalloc::dlmalloc::align_up (;57;) (type 1) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.add
    i32.const -1
    i32.add
    i32.const 0
    local.get 1
    i32.sub
    i32.and
  )
  (func $dlmalloc::dlmalloc::left_bits (;58;) (type 9) (param i32) (result i32)
    local.get 0
    i32.const 1
    i32.shl
    local.tee 0
    i32.const 0
    local.get 0
    i32.sub
    i32.or
  )
  (func $dlmalloc::dlmalloc::least_bit (;59;) (type 9) (param i32) (result i32)
    i32.const 0
    local.get 0
    i32.sub
    local.get 0
    i32.and
  )
  (func $dlmalloc::dlmalloc::leftshift_for_tree_index (;60;) (type 9) (param i32) (result i32)
    i32.const 0
    i32.const 25
    local.get 0
    i32.const 1
    i32.shr_u
    i32.sub
    local.get 0
    i32.const 31
    i32.eq
    select
  )
  (func $dlmalloc::dlmalloc::Chunk::fencepost_head (;61;) (type 5) (result i32)
    i32.const 7
  )
  (func $dlmalloc::dlmalloc::Chunk::size (;62;) (type 9) (param i32) (result i32)
    local.get 0
    i32.load offset=4
    i32.const -8
    i32.and
  )
  (func $dlmalloc::dlmalloc::Chunk::cinuse (;63;) (type 9) (param i32) (result i32)
    local.get 0
    i32.load8_u offset=4
    i32.const 2
    i32.and
    i32.const 1
    i32.shr_u
  )
  (func $dlmalloc::dlmalloc::Chunk::pinuse (;64;) (type 9) (param i32) (result i32)
    local.get 0
    i32.load offset=4
    i32.const 1
    i32.and
  )
  (func $dlmalloc::dlmalloc::Chunk::clear_pinuse (;65;) (type 3) (param i32)
    local.get 0
    local.get 0
    i32.load offset=4
    i32.const -2
    i32.and
    i32.store offset=4
  )
  (func $dlmalloc::dlmalloc::Chunk::inuse (;66;) (type 9) (param i32) (result i32)
    local.get 0
    i32.load offset=4
    i32.const 3
    i32.and
    i32.const 1
    i32.ne
  )
  (func $dlmalloc::dlmalloc::Chunk::mmapped (;67;) (type 9) (param i32) (result i32)
    local.get 0
    i32.load8_u offset=4
    i32.const 3
    i32.and
    i32.eqz
  )
  (func $dlmalloc::dlmalloc::Chunk::set_inuse (;68;) (type 6) (param i32 i32)
    local.get 0
    local.get 0
    i32.load offset=4
    i32.const 1
    i32.and
    local.get 1
    i32.or
    i32.const 2
    i32.or
    i32.store offset=4
    local.get 0
    local.get 1
    i32.add
    local.tee 0
    local.get 0
    i32.load offset=4
    i32.const 1
    i32.or
    i32.store offset=4
  )
  (func $dlmalloc::dlmalloc::Chunk::set_inuse_and_pinuse (;69;) (type 6) (param i32 i32)
    local.get 0
    local.get 1
    i32.const 3
    i32.or
    i32.store offset=4
    local.get 0
    local.get 1
    i32.add
    local.tee 1
    local.get 1
    i32.load offset=4
    i32.const 1
    i32.or
    i32.store offset=4
  )
  (func $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_inuse_chunk (;70;) (type 6) (param i32 i32)
    local.get 0
    local.get 1
    i32.const 3
    i32.or
    i32.store offset=4
  )
  (func $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_free_chunk (;71;) (type 6) (param i32 i32)
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
  )
  (func $dlmalloc::dlmalloc::Chunk::set_free_with_pinuse (;72;) (type 7) (param i32 i32 i32)
    local.get 2
    local.get 2
    i32.load offset=4
    i32.const -2
    i32.and
    i32.store offset=4
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
  )
  (func $dlmalloc::dlmalloc::Chunk::plus_offset (;73;) (type 1) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.add
  )
  (func $dlmalloc::dlmalloc::Chunk::minus_offset (;74;) (type 1) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.sub
  )
  (func $dlmalloc::dlmalloc::Chunk::to_mem (;75;) (type 9) (param i32) (result i32)
    local.get 0
    i32.const 8
    i32.add
  )
  (func $dlmalloc::dlmalloc::Chunk::mem_offset (;76;) (type 5) (result i32)
    i32.const 8
  )
  (func $dlmalloc::dlmalloc::Chunk::from_mem (;77;) (type 9) (param i32) (result i32)
    local.get 0
    i32.const -8
    i32.add
  )
  (func $dlmalloc::dlmalloc::TreeChunk::leftmost_child (;78;) (type 9) (param i32) (result i32)
    (local i32)
    block ;; label = @1
      local.get 0
      i32.load offset=16
      local.tee 1
      br_if 0 (;@1;)
      local.get 0
      i32.const 20
      i32.add
      i32.load
      local.set 1
    end
    local.get 1
  )
  (func $dlmalloc::dlmalloc::TreeChunk::chunk (;79;) (type 9) (param i32) (result i32)
    local.get 0
  )
  (func $dlmalloc::dlmalloc::TreeChunk::next (;80;) (type 9) (param i32) (result i32)
    local.get 0
    i32.load offset=12
  )
  (func $dlmalloc::dlmalloc::TreeChunk::prev (;81;) (type 9) (param i32) (result i32)
    local.get 0
    i32.load offset=8
  )
  (func $dlmalloc::dlmalloc::Segment::is_extern (;82;) (type 9) (param i32) (result i32)
    local.get 0
    i32.load offset=12
    i32.const 1
    i32.and
  )
  (func $dlmalloc::dlmalloc::Segment::sys_flags (;83;) (type 9) (param i32) (result i32)
    local.get 0
    i32.load offset=12
    i32.const 1
    i32.shr_u
  )
  (func $dlmalloc::dlmalloc::Segment::holds (;84;) (type 1) (param i32 i32) (result i32)
    (local i32 i32)
    i32.const 0
    local.set 2
    block ;; label = @1
      local.get 0
      i32.load
      local.tee 3
      local.get 1
      i32.gt_u
      br_if 0 (;@1;)
      local.get 3
      local.get 0
      i32.load offset=4
      i32.add
      local.get 1
      i32.gt_u
      local.set 2
    end
    local.get 2
  )
  (func $dlmalloc::dlmalloc::Segment::top (;85;) (type 9) (param i32) (result i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    i32.add
  )
  (func $<dlmalloc::global::Instance as core::ops::deref::DerefMut>::deref_mut (;86;) (type 9) (param i32) (result i32)
    i32.const 1049332
  )
  (func $<dlmalloc::global::Instance as core::ops::drop::Drop>::drop (;87;) (type 3) (param i32))
  (func $<dlmalloc::sys::System as dlmalloc::Allocator>::alloc (;88;) (type 7) (param i32 i32 i32)
    (local i32)
    local.get 2
    i32.const 16
    i32.shr_u
    memory.grow
    local.set 3
    local.get 0
    i32.const 0
    i32.store offset=8
    local.get 0
    i32.const 0
    local.get 2
    i32.const -65536
    i32.and
    local.get 3
    i32.const -1
    i32.eq
    local.tee 2
    select
    i32.store offset=4
    local.get 0
    i32.const 0
    local.get 3
    i32.const 16
    i32.shl
    local.get 2
    select
    i32.store
  )
  (func $<dlmalloc::sys::System as dlmalloc::Allocator>::remap (;89;) (type 11) (param i32 i32 i32 i32 i32) (result i32)
    i32.const 0
  )
  (func $<dlmalloc::sys::System as dlmalloc::Allocator>::free_part (;90;) (type 10) (param i32 i32 i32 i32) (result i32)
    i32.const 0
  )
  (func $<dlmalloc::sys::System as dlmalloc::Allocator>::free (;91;) (type 0) (param i32 i32 i32) (result i32)
    i32.const 0
  )
  (func $<dlmalloc::sys::System as dlmalloc::Allocator>::can_release_part (;92;) (type 1) (param i32 i32) (result i32)
    i32.const 0
  )
  (func $<dlmalloc::sys::System as dlmalloc::Allocator>::allocates_zeros (;93;) (type 9) (param i32) (result i32)
    i32.const 1
  )
  (func $<dlmalloc::sys::System as dlmalloc::Allocator>::page_size (;94;) (type 9) (param i32) (result i32)
    i32.const 65536
  )
  (func $dlmalloc::sys::enable_alloc_after_fork (;95;) (type 2))
  (func $<alloc::vec::Vec<T,A> as alloc::vec::spec_extend::SpecExtend<&T,core::slice::iter::Iter<T>>>::spec_extend (;96;) (type 7) (param i32 i32 i32)
    (local i32 i32)
    local.get 0
    local.get 2
    local.get 1
    i32.sub
    local.tee 2
    i32.const 3
    i32.shr_u
    local.tee 3
    call $alloc::vec::Vec<T,A>::reserve
    local.get 0
    i32.load
    local.get 0
    i32.load offset=8
    local.tee 4
    i32.const 3
    i32.shl
    i32.add
    local.get 1
    local.get 2
    call $memcpy
    drop
    local.get 0
    local.get 4
    local.get 3
    i32.add
    i32.store offset=8
  )
  (func $alloc::vec::Vec<T,A>::reserve (;97;) (type 6) (param i32 i32)
    (local i32)
    block ;; label = @1
      local.get 0
      i32.load offset=4
      local.get 0
      i32.load offset=8
      local.tee 2
      i32.sub
      local.get 1
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      local.get 2
      local.get 1
      call $alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle
    end
  )
  (func $alloc::vec::Vec<T,A>::push (;98;) (type 12) (param i32 i64)
    (local i32)
    block ;; label = @1
      local.get 0
      i32.load offset=8
      local.tee 2
      local.get 0
      i32.load offset=4
      i32.ne
      br_if 0 (;@1;)
      local.get 0
      local.get 2
      call $alloc::raw_vec::RawVec<T,A>::reserve_for_push
      local.get 0
      i32.load offset=8
      local.set 2
    end
    local.get 0
    local.get 2
    i32.const 1
    i32.add
    i32.store offset=8
    local.get 0
    i32.load
    local.get 2
    i32.const 3
    i32.shl
    i32.add
    local.get 1
    i64.store
  )
  (func $alloc::raw_vec::RawVec<T,A>::reserve_for_push (;99;) (type 6) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 8
    i32.add
    local.get 0
    local.get 1
    i32.const 1
    call $alloc::raw_vec::RawVec<T,A>::grow_amortized
    local.get 2
    i32.load offset=8
    local.get 2
    i32.load offset=12
    call $alloc::raw_vec::handle_reserve
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle (;100;) (type 7) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 8
    i32.add
    local.get 0
    local.get 1
    local.get 2
    call $alloc::raw_vec::RawVec<T,A>::grow_amortized
    local.get 3
    i32.load offset=8
    local.get 3
    i32.load offset=12
    call $alloc::raw_vec::handle_reserve
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $alloc::alloc::Global::alloc_impl (;101;) (type 8) (param i32 i32 i32 i32)
    block ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 3
        br_if 0 (;@2;)
        i32.const 0
        i32.load8_u offset=1049329
        drop
        local.get 2
        local.get 1
        call $__rust_alloc
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      call $__rust_alloc_zeroed
      local.set 1
    end
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
  )
  (func $alloc::raw_vec::finish_grow (;102;) (type 13) (param i32 i32 i32 i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 1
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const -1
            i32.le_s
            br_if 1 (;@3;)
            block ;; label = @5
              block ;; label = @6
                local.get 3
                i32.load offset=4
                i32.eqz
                br_if 0 (;@6;)
                block ;; label = @7
                  local.get 3
                  i32.const 8
                  i32.add
                  i32.load
                  local.tee 6
                  br_if 0 (;@7;)
                  local.get 5
                  i32.const 8
                  i32.add
                  local.get 1
                  local.get 2
                  i32.const 0
                  call $alloc::alloc::Global::alloc_impl
                  local.get 5
                  i32.load offset=12
                  local.set 6
                  local.get 5
                  i32.load offset=8
                  local.set 3
                  br 2 (;@5;)
                end
                local.get 3
                i32.load
                local.get 6
                local.get 1
                local.get 2
                call $__rust_realloc
                local.set 3
                local.get 2
                local.set 6
                br 1 (;@5;)
              end
              local.get 5
              local.get 1
              local.get 2
              call $<alloc::alloc::Global as core::alloc::Allocator>::allocate
              local.get 5
              i32.load offset=4
              local.set 6
              local.get 5
              i32.load
              local.set 3
            end
            block ;; label = @5
              local.get 3
              i32.eqz
              br_if 0 (;@5;)
              local.get 0
              local.get 3
              i32.store offset=4
              local.get 0
              i32.const 8
              i32.add
              local.get 6
              i32.store
              i32.const 0
              local.set 2
              br 4 (;@1;)
            end
            local.get 0
            local.get 1
            i32.store offset=4
            local.get 0
            i32.const 8
            i32.add
            local.get 2
            i32.store
            br 2 (;@2;)
          end
          local.get 0
          i32.const 0
          i32.store offset=4
          local.get 0
          i32.const 8
          i32.add
          local.get 2
          i32.store
          br 1 (;@2;)
        end
        local.get 0
        i32.const 0
        i32.store offset=4
      end
      i32.const 1
      local.set 2
    end
    local.get 0
    local.get 2
    i32.store
    local.get 5
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $<alloc::alloc::Global as core::alloc::Allocator>::allocate (;103;) (type 7) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 8
    i32.add
    local.get 1
    local.get 2
    i32.const 0
    call $alloc::alloc::Global::alloc_impl
    local.get 3
    i32.load offset=12
    local.set 2
    local.get 0
    local.get 3
    i32.load offset=8
    i32.store
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $alloc::raw_vec::handle_reserve (;104;) (type 6) (param i32 i32)
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.const -2147483647
        i32.eq
        br_if 0 (;@2;)
        local.get 0
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        local.get 1
        call $alloc::alloc::handle_alloc_error
        unreachable
      end
      return
    end
    call $alloc::raw_vec::capacity_overflow
    unreachable
  )
  (func $alloc::raw_vec::RawVec<T,A>::allocate_in (;105;) (type 7) (param i32 i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 1
            br_if 0 (;@4;)
            i32.const 8
            local.set 2
            br 1 (;@3;)
          end
          local.get 1
          i32.const 268435455
          i32.gt_u
          br_if 1 (;@2;)
          local.get 1
          i32.const 3
          i32.shl
          local.tee 4
          i32.const -1
          i32.le_s
          br_if 1 (;@2;)
          block ;; label = @4
            block ;; label = @5
              local.get 2
              br_if 0 (;@5;)
              local.get 3
              i32.const 8
              i32.add
              i32.const 8
              local.get 4
              call $<alloc::alloc::Global as core::alloc::Allocator>::allocate
              local.get 3
              i32.load offset=8
              local.set 2
              br 1 (;@4;)
            end
            local.get 3
            i32.const 8
            local.get 4
            i32.const 1
            call $alloc::alloc::Global::alloc_impl
            local.get 3
            i32.load
            local.set 2
          end
          local.get 2
          i32.eqz
          br_if 2 (;@1;)
        end
        local.get 0
        local.get 1
        i32.store offset=4
        local.get 0
        local.get 2
        i32.store
        local.get 3
        i32.const 16
        i32.add
        global.set $__stack_pointer
        return
      end
      call $alloc::raw_vec::capacity_overflow
      unreachable
    end
    i32.const 8
    local.get 4
    call $alloc::alloc::handle_alloc_error
    unreachable
  )
  (func $alloc::raw_vec::RawVec<T,A>::current_memory (;106;) (type 6) (param i32 i32)
    (local i32)
    block ;; label = @1
      local.get 1
      i32.load offset=4
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const 8
      i32.store offset=4
      local.get 0
      local.get 2
      i32.const 3
      i32.shl
      i32.store offset=8
      local.get 0
      local.get 1
      i32.load
      i32.store
      return
    end
    local.get 0
    i32.const 0
    i32.store offset=4
  )
  (func $alloc::raw_vec::RawVec<T,A>::grow_amortized (;107;) (type 8) (param i32 i32 i32 i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    i32.const 0
    local.set 5
    block ;; label = @1
      local.get 2
      local.get 3
      i32.add
      local.tee 3
      local.get 2
      i32.lt_u
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=4
      local.tee 2
      i32.const 1
      i32.shl
      local.tee 5
      local.get 3
      local.get 5
      local.get 3
      i32.gt_u
      select
      local.tee 3
      i32.const 4
      local.get 3
      i32.const 4
      i32.gt_u
      select
      local.tee 3
      i32.const 3
      i32.shl
      local.set 5
      local.get 3
      i32.const 268435456
      i32.lt_u
      i32.const 3
      i32.shl
      local.set 6
      block ;; label = @2
        block ;; label = @3
          local.get 2
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          i32.const 8
          i32.store offset=24
          local.get 4
          local.get 2
          i32.const 3
          i32.shl
          i32.store offset=28
          local.get 4
          local.get 1
          i32.load
          i32.store offset=20
          br 1 (;@2;)
        end
        local.get 4
        i32.const 0
        i32.store offset=24
      end
      local.get 4
      i32.const 8
      i32.add
      local.get 6
      local.get 5
      local.get 4
      i32.const 20
      i32.add
      local.get 4
      call $alloc::raw_vec::finish_grow
      local.get 4
      i32.load offset=12
      local.set 5
      block ;; label = @2
        local.get 4
        i32.load offset=8
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i32.const 16
        i32.add
        i32.load
        local.set 3
        br 1 (;@1;)
      end
      local.get 1
      local.get 3
      i32.store offset=4
      local.get 1
      local.get 5
      i32.store
      i32.const -2147483647
      local.set 5
    end
    local.get 0
    local.get 3
    i32.store offset=4
    local.get 0
    local.get 5
    i32.store
    local.get 4
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $<miden::asset::Asset as miden::serialization::FeltSerialize>::to_felts (;108;) (type 6) (param i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        local.get 1
        i32.load
        br_if 0 (;@2;)
        local.get 2
        i32.const 8
        i32.add
        i32.const 2
        i32.const 0
        call $alloc::raw_vec::RawVec<T,A>::allocate_in
        local.get 2
        i32.const 20
        i32.add
        i32.const 8
        i32.add
        local.tee 3
        i32.const 0
        i32.store
        local.get 2
        local.get 2
        i64.load offset=8
        i64.store offset=20 align=4
        local.get 2
        i32.const 20
        i32.add
        local.get 1
        i64.load offset=8
        call $alloc::vec::Vec<T,A>::push
        local.get 2
        i32.const 20
        i32.add
        local.get 1
        i32.const 16
        i32.add
        i64.load
        call $alloc::vec::Vec<T,A>::push
        local.get 0
        i32.const 8
        i32.add
        local.get 3
        i32.load
        i32.store
        local.get 0
        local.get 2
        i64.load offset=20 align=4
        i64.store align=4
        br 1 (;@1;)
      end
      local.get 0
      local.get 1
      i32.const 8
      i32.add
      call $<miden::felt::Word as miden::serialization::FeltSerialize>::to_felts
    end
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $<miden::felt::Word as miden::serialization::FeltSerialize>::to_felts (;109;) (type 6) (param i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 8
    i32.add
    i32.const 4
    i32.const 0
    call $alloc::raw_vec::RawVec<T,A>::allocate_in
    local.get 2
    i32.const 20
    i32.add
    i32.const 8
    i32.add
    local.tee 3
    i32.const 0
    i32.store
    local.get 2
    local.get 2
    i64.load offset=8
    i64.store offset=20 align=4
    local.get 2
    i32.const 20
    i32.add
    local.get 1
    local.get 1
    i32.const 32
    i32.add
    call $<alloc::vec::Vec<T,A> as alloc::vec::spec_extend::SpecExtend<&T,core::slice::iter::Iter<T>>>::spec_extend
    local.get 0
    i32.const 8
    i32.add
    local.get 3
    i32.load
    i32.store
    local.get 0
    local.get 2
    i64.load offset=20 align=4
    i64.store align=4
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $miden::note::Tag::new (;110;) (type 14) (param i64) (result i64)
    local.get 0
  )
  (func $miden::sat::note::get_assets (;111;) (type 3) (param i32)
    local.get 0
    call $miden::sat::note::get_assets
  )
  (func $core::fmt::Arguments::new_v1 (;112;) (type 13) (param i32 i32 i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    block ;; label = @1
      local.get 2
      local.get 4
      i32.lt_u
      br_if 0 (;@1;)
      local.get 4
      i32.const 1
      i32.add
      local.get 2
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      i32.const 0
      i32.store offset=16
      local.get 0
      local.get 2
      i32.store offset=4
      local.get 0
      local.get 1
      i32.store
      local.get 0
      local.get 3
      i32.store offset=8
      local.get 0
      i32.const 12
      i32.add
      local.get 4
      i32.store
      local.get 5
      i32.const 32
      i32.add
      global.set $__stack_pointer
      return
    end
    local.get 5
    i32.const 20
    i32.add
    i64.const 0
    i64.store align=4
    local.get 5
    i32.const 1
    i32.store offset=12
    local.get 5
    i32.const 1048772
    i32.store offset=8
    local.get 5
    i32.const 1048780
    i32.store offset=16
    local.get 5
    i32.const 8
    i32.add
    i32.const 1048780
    call $core::panicking::panic_fmt
    unreachable
  )
  (func $alloc::alloc::handle_alloc_error (;113;) (type 6) (param i32 i32)
    local.get 0
    local.get 1
    call $alloc::alloc::handle_alloc_error::rt_error
    unreachable
  )
  (func $alloc::raw_vec::capacity_overflow (;114;) (type 2)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    local.get 0
    i32.const 20
    i32.add
    i64.const 0
    i64.store align=4
    local.get 0
    i32.const 1
    i32.store offset=12
    local.get 0
    i32.const 1048844
    i32.store offset=8
    local.get 0
    i32.const 1048780
    i32.store offset=16
    local.get 0
    i32.const 8
    i32.add
    i32.const 1048852
    call $core::panicking::panic_fmt
    unreachable
  )
  (func $alloc::alloc::handle_alloc_error::rt_error (;115;) (type 6) (param i32 i32)
    local.get 1
    local.get 0
    call $__rust_alloc_error_handler
    unreachable
  )
  (func $__rdl_oom (;116;) (type 6) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 0
    i32.store offset=12
    block ;; label = @1
      i32.const 0
      i32.load8_u offset=1049328
      br_if 0 (;@1;)
      local.get 2
      i32.const 1
      i32.store offset=44
      local.get 2
      local.get 2
      i32.const 12
      i32.add
      i32.store offset=40
      local.get 2
      i32.const 16
      i32.add
      i32.const 1048928
      i32.const 2
      local.get 2
      i32.const 40
      i32.add
      i32.const 1
      call $core::fmt::Arguments::new_v1
      local.get 2
      i32.const 16
      i32.add
      i32.const 1048944
      call $core::panicking::panic_nounwind_fmt
      unreachable
    end
    local.get 2
    i32.const 1
    i32.store offset=44
    local.get 2
    local.get 2
    i32.const 12
    i32.add
    i32.store offset=40
    local.get 2
    i32.const 16
    i32.add
    i32.const 1048928
    i32.const 2
    local.get 2
    i32.const 40
    i32.add
    i32.const 1
    call $core::fmt::Arguments::new_v1
    local.get 2
    i32.const 16
    i32.add
    i32.const 1048960
    call $core::panicking::panic_fmt
    unreachable
  )
  (func $core::ptr::drop_in_place<core::fmt::Error> (;117;) (type 3) (param i32))
  (func $core::panicking::panic_fmt (;118;) (type 6) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 0
    i32.store offset=24
    local.get 2
    i32.const 1049024
    i32.store offset=16
    local.get 2
    i32.const 1048976
    i32.store offset=12
    local.get 2
    i32.const 1
    i32.store8 offset=28
    local.get 2
    local.get 1
    i32.store offset=20
    local.get 2
    i32.const 12
    i32.add
    call $rust_begin_unwind
    unreachable
  )
  (func $core::panicking::panic_bounds_check (;119;) (type 7) (param i32 i32 i32)
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
    i32.const 44
    i32.add
    i32.const 1
    i32.store
    local.get 3
    i32.const 1
    i32.store offset=36
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
    i32.const 1049092
    i32.const 2
    local.get 3
    i32.const 32
    i32.add
    i32.const 2
    call $#func121<core::fmt::Arguments::new_v1>
    local.get 3
    i32.const 8
    i32.add
    local.get 2
    call $core::panicking::panic_fmt
    unreachable
  )
  (func $core::fmt::num::imp::<impl core::fmt::Display for u32>::fmt (;120;) (type 1) (param i32 i32) (result i32)
    local.get 0
    i64.load32_u
    i32.const 1
    local.get 1
    call $core::fmt::num::imp::fmt_u64
  )
  (func $#func121<core::fmt::Arguments::new_v1> (@name "core::fmt::Arguments::new_v1") (;121;) (type 13) (param i32 i32 i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    block ;; label = @1
      local.get 2
      local.get 4
      i32.lt_u
      br_if 0 (;@1;)
      local.get 4
      i32.const 1
      i32.add
      local.get 2
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      i32.const 0
      i32.store offset=16
      local.get 0
      local.get 2
      i32.store offset=4
      local.get 0
      local.get 1
      i32.store
      local.get 0
      local.get 3
      i32.store offset=8
      local.get 0
      i32.const 12
      i32.add
      local.get 4
      i32.store
      local.get 5
      i32.const 32
      i32.add
      global.set $__stack_pointer
      return
    end
    local.get 5
    i32.const 20
    i32.add
    i64.const 0
    i64.store align=4
    local.get 5
    i32.const 1
    i32.store offset=12
    local.get 5
    i32.const 1048988
    i32.store offset=8
    local.get 5
    i32.const 1048976
    i32.store offset=16
    local.get 5
    i32.const 8
    i32.add
    i32.const 1049308
    call $core::panicking::panic_fmt
    unreachable
  )
  (func $core::fmt::Formatter::pad_integral (;122;) (type 15) (param i32 i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        local.get 1
        i32.eqz
        br_if 0 (;@2;)
        i32.const 43
        i32.const 1114112
        local.get 0
        i32.load offset=28
        local.tee 6
        i32.const 1
        i32.and
        local.tee 1
        select
        local.set 7
        local.get 1
        local.get 5
        i32.add
        local.set 8
        br 1 (;@1;)
      end
      local.get 5
      i32.const 1
      i32.add
      local.set 8
      local.get 0
      i32.load offset=28
      local.set 6
      i32.const 45
      local.set 7
    end
    block ;; label = @1
      block ;; label = @2
        local.get 6
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
      local.get 8
      i32.add
      local.set 8
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
        local.get 7
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
        local.get 8
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
        local.get 7
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
        local.get 6
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
        local.set 6
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
        local.get 7
        local.get 2
        local.get 3
        call $core::fmt::Formatter::pad_integral::write_prefix
        br_if 1 (;@1;)
        local.get 9
        local.get 8
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
        local.get 6
        i32.store8 offset=32
        local.get 0
        local.get 11
        i32.store offset=16
        i32.const 0
        local.set 1
        br 1 (;@1;)
      end
      local.get 9
      local.get 8
      i32.sub
      local.set 8
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 0
            i32.load8_u offset=32
            local.tee 1
            br_table 2 (;@2;) 0 (;@4;) 1 (;@3;) 0 (;@4;) 2 (;@2;)
          end
          local.get 8
          local.set 1
          i32.const 0
          local.set 8
          br 1 (;@2;)
        end
        local.get 8
        i32.const 1
        i32.shr_u
        local.set 1
        local.get 8
        i32.const 1
        i32.add
        i32.const 1
        i32.shr_u
        local.set 8
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
      local.get 7
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
          local.get 8
          local.get 1
          i32.ne
          br_if 0 (;@3;)
          local.get 8
          local.get 8
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
      local.get 8
      i32.lt_u
      return
    end
    local.get 1
  )
  (func $<T as core::any::Any>::type_id (;123;) (type 6) (param i32 i32)
    local.get 0
    i64.const -3751304911407043677
    i64.store offset=8
    local.get 0
    i64.const 118126004786499436
    i64.store
  )
  (func $core::panicking::panic_nounwind_fmt (;124;) (type 6) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 0
    i32.store offset=24
    local.get 2
    i32.const 1049024
    i32.store offset=16
    local.get 2
    i32.const 1048976
    i32.store offset=12
    local.get 2
    i32.const 0
    i32.store8 offset=28
    local.get 2
    local.get 1
    i32.store offset=20
    local.get 2
    i32.const 12
    i32.add
    call $rust_begin_unwind
    unreachable
  )
  (func $core::str::count::do_count_chars (;125;) (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
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
          local.set 5
          local.get 4
          i32.const 2
          i32.shl
          local.set 7
          block ;; label = @4
            block ;; label = @5
              local.get 4
              i32.const 252
              i32.and
              local.tee 10
              br_if 0 (;@5;)
              i32.const 0
              local.set 9
              br 1 (;@4;)
            end
            local.get 6
            local.get 10
            i32.const 2
            i32.shl
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
          local.get 7
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
          local.get 5
          i32.eqz
          br_if 0 (;@3;)
        end
        local.get 6
        local.get 10
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
          local.get 5
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
          local.get 5
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
        local.set 2
        br 1 (;@1;)
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
  (func $core::fmt::Formatter::pad_integral::write_prefix (;126;) (type 11) (param i32 i32 i32 i32 i32) (result i32)
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
  (func $core::fmt::num::imp::fmt_u64 (;127;) (type 16) (param i64 i32 i32) (result i32)
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
        i32.const 1049108
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
        i32.const 1049108
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
      i32.const 1049108
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
        i32.const 1049108
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
    i32.const 1048976
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
  (export "_start" (func $_start))
  (export "__main_void" (func $__main_void))
  (elem (;0;) (i32.const 1) func $core::fmt::num::imp::<impl core::fmt::Display for u32>::fmt $core::ptr::drop_in_place<core::fmt::Error> $<T as core::any::Any>::type_id)
  (data $.rodata (;0;) (i32.const 1048576) "not yet implemented: use advice provider\00\00\10\00(\00\00\00src/lib.rs\00\000\00\10\00\0a\00\00\00C\00\00\00\0d\00\00\000\00\10\00\0a\00\00\00@\00\00\006\00\00\000\00\10\00\0a\00\00\00@\00\00\00@\00\00\00/rustc/c469197b19d53a6c45378568f73c00986b20a5a5/library/core/src/fmt/mod.rsinvalid args\00\b7\00\10\00\0c\00\00\00l\00\10\00K\00\00\005\01\00\00\0d\00\00\00library/alloc/src/raw_vec.rscapacity overflow\00\00\00\f8\00\10\00\11\00\00\00\dc\00\10\00\1c\00\00\00\16\02\00\00\05\00\00\00library/alloc/src/alloc.rsmemory allocation of  bytes failed>\01\10\00\15\00\00\00S\01\10\00\0d\00\00\00$\01\10\00\1a\00\00\00\8e\01\00\00\0d\00\00\00$\01\10\00\1a\00\00\00\8c\01\00\00\0d\00\00\00invalid args\90\01\10\00\0c\00\00\00library/core/src/fmt/mod.rs\00\02\00\00\00\00\00\00\00\01\00\00\00\03\00\00\00index out of bounds: the len is  but the index is \00\00\d0\01\10\00 \00\00\00\f0\01\10\00\12\00\00\0000010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899\a4\01\10\00\1b\00\00\005\01\00\00\0d\00\00\00")
)