(module
  (type (;0;) (func (param i32 i32 i32) (result i32)))
  (type (;1;) (func (param i32 i32)))
  (type (;2;) (func (param i32 i32 i32)))
  (type (;3;) (func (param i32 i32 i32 i32)))
  (type (;4;) (func (param i32 i32) (result i32)))
  (type (;5;) (func (param i32) (result i32)))
  (type (;6;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;7;) (func (param i32)))
  (type (;8;) (func (param i32 i64)))
  (type (;9;) (func (param i32 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64)))
  (type (;10;) (func (result i32)))
  (type (;11;) (func))
  (type (;12;) (func (param i64) (result i64)))
  (import "env" "memcpy" (func $memcpy (;0;) (type 0)))
  (import "env" "memset" (func $memset (;1;) (type 0)))
  (import "env" "miden::sat::account::add_asset" (func $miden::sat::account::add_asset (;2;) (type 1)))
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::replace_dv (;3;) (type 2) (param i32 i32 i32)
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
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::insert_small_chunk (;4;) (type 2) (param i32 i32 i32)
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
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::add_segment (;5;) (type 3) (param i32 i32 i32 i32)
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
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::segment_holding (;6;) (type 4) (param i32 i32) (result i32)
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
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::align_offset_usize (;7;) (type 4) (param i32 i32) (result i32)
    local.get 1
    i32.const 8
    call $dlmalloc::dlmalloc::align_up
    local.get 1
    i32.sub
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::min_chunk_size (;8;) (type 5) (param i32) (result i32)
    i32.const 16
    i32.const 8
    call $dlmalloc::dlmalloc::align_up
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::top_foot_size (;9;) (type 5) (param i32) (result i32)
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
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::init_top (;10;) (type 2) (param i32 i32 i32)
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
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::insert_chunk (;11;) (type 2) (param i32 i32 i32)
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
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::max_request (;12;) (type 5) (param i32) (result i32)
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
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::min_request (;13;) (type 5) (param i32) (result i32)
    (local i32)
    local.get 1
    call $dlmalloc::dlmalloc::Dlmalloc<A>::min_chunk_size
    i32.const -5
    i32.add
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::pad_request (;14;) (type 4) (param i32 i32) (result i32)
    local.get 1
    i32.const 4
    i32.add
    i32.const 8
    call $dlmalloc::dlmalloc::align_up
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::insert_large_chunk (;15;) (type 2) (param i32 i32 i32)
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
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::request2size (;16;) (type 4) (param i32 i32) (result i32)
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
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_chunk (;17;) (type 2) (param i32 i32 i32)
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
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_large_chunk (;18;) (type 1) (param i32 i32)
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
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_small_chunk (;19;) (type 2) (param i32 i32 i32)
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
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::dispose_chunk (;20;) (type 2) (param i32 i32 i32)
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
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::prepend_alloc (;21;) (type 6) (param i32 i32 i32 i32) (result i32)
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
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::align_as_chunk (;22;) (type 4) (param i32 i32) (result i32)
    local.get 1
    local.get 1
    local.get 1
    call $dlmalloc::dlmalloc::Chunk::to_mem
    call $dlmalloc::dlmalloc::Dlmalloc<A>::align_offset_usize
    i32.add
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::tmalloc_large (;23;) (type 4) (param i32 i32) (result i32)
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
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::compute_tree_index (;24;) (type 4) (param i32 i32) (result i32)
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
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::tmalloc_small (;25;) (type 4) (param i32 i32) (result i32)
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
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::has_segment_link (;26;) (type 4) (param i32 i32) (result i32)
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
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::calloc_must_clear (;27;) (type 4) (param i32 i32) (result i32)
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
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::release_unused_segments (;28;) (type 5) (param i32) (result i32)
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
  (func $dlmalloc::dlmalloc::Segment::can_release_part (;29;) (type 4) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.load offset=12
    i32.const 1
    i32.shr_u
    call $<dlmalloc::sys::System as dlmalloc::Allocator>::can_release_part
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_first_small_chunk (;30;) (type 3) (param i32 i32 i32 i32)
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
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::free (;31;) (type 1) (param i32 i32)
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
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::sys_trim (;32;) (type 4) (param i32 i32) (result i32)
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
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::malloc (;33;) (type 4) (param i32 i32) (result i32)
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
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::sys_alloc (;34;) (type 4) (param i32 i32) (result i32)
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
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::memalign (;35;) (type 0) (param i32 i32 i32) (result i32)
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
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::init_bins (;36;) (type 7) (param i32)
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
  (func $<miden::asset::_::<impl serde::de::Deserialize for miden::asset::Asset>::deserialize::__Visitor as serde::de::Visitor>::visit_enum (;37;) (type 1) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 96
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i32.const 0
    local.set 3
    i32.const 0
    local.set 4
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          loop ;; label = @4
            i32.const 5
            local.set 5
            local.get 3
            i32.const 35
            i32.eq
            br_if 1 (;@3;)
            local.get 2
            i32.const 8
            i32.add
            local.get 1
            call $<postcard::de::flavors::Slice as postcard::de::flavors::Flavor>::pop
            local.get 2
            i32.load8_u offset=9
            local.set 6
            block ;; label = @5
              local.get 2
              i32.load8_u offset=8
              i32.const 1
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              local.get 6
              local.set 5
              br 2 (;@3;)
            end
            local.get 6
            i32.const 127
            i32.and
            local.get 3
            i32.shl
            local.get 4
            i32.or
            local.set 4
            local.get 3
            i32.const 7
            i32.add
            local.tee 7
            local.set 3
            local.get 6
            i32.extend8_s
            i32.const 0
            i32.lt_s
            br_if 0 (;@4;)
          end
          block ;; label = @4
            local.get 7
            i32.const 35
            i32.ne
            br_if 0 (;@4;)
            local.get 6
            i32.const 255
            i32.and
            i32.const 15
            i32.gt_u
            br_if 1 (;@3;)
          end
          local.get 2
          local.get 4
          call $<serde::de::value::U32Deserializer<E> as serde::de::Deserializer>::deserialize_any
          local.get 2
          i32.load8_u offset=1
          local.set 5
          local.get 2
          i32.load8_u
          i32.const 1
          i32.and
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 0
        i32.const 2
        i32.store
        local.get 0
        local.get 5
        i32.store8 offset=4
        br 1 (;@1;)
      end
      block ;; label = @2
        local.get 5
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 2
        i32.const 16
        i32.add
        local.get 1
        call $miden::asset::_::<impl serde::de::Deserialize for miden::asset::FungibleAsset>::deserialize
        block ;; label = @3
          block ;; label = @4
            local.get 2
            i32.load8_u offset=16
            br_if 0 (;@4;)
            local.get 0
            local.get 2
            i64.load offset=24
            i64.store offset=8
            local.get 0
            local.get 2
            i32.const 32
            i32.add
            i64.load
            i64.store offset=16
            i32.const 0
            local.set 3
            br 1 (;@3;)
          end
          local.get 0
          local.get 2
          i32.load8_u offset=17
          i32.store8 offset=4
          i32.const 2
          local.set 3
        end
        local.get 0
        local.get 3
        i32.store
        br 1 (;@1;)
      end
      local.get 2
      i32.const 16
      i32.add
      local.get 1
      call $miden::asset::_::<impl serde::de::Deserialize for miden::asset::NonFungibleAsset>::deserialize
      block ;; label = @2
        block ;; label = @3
          local.get 2
          i32.load8_u offset=16
          br_if 0 (;@3;)
          local.get 2
          i32.const 88
          i32.add
          local.get 2
          i32.const 48
          i32.add
          i64.load
          i64.store align=4
          local.get 2
          i32.const 80
          i32.add
          local.get 2
          i32.const 40
          i32.add
          i64.load
          i64.store align=4
          local.get 2
          i32.const 72
          i32.add
          local.get 2
          i32.const 32
          i32.add
          i64.load
          i64.store align=4
          local.get 2
          local.get 2
          i64.load offset=24
          i64.store offset=64 align=4
          local.get 0
          i32.const 4
          i32.add
          local.get 2
          i32.const 60
          i32.add
          i32.const 36
          call $memcpy
          drop
          i32.const 1
          local.set 3
          br 1 (;@2;)
        end
        local.get 0
        local.get 2
        i32.load8_u offset=17
        i32.store8 offset=4
        i32.const 2
        local.set 3
      end
      local.get 0
      local.get 3
      i32.store
    end
    local.get 2
    i32.const 96
    i32.add
    global.set $__stack_pointer
  )
  (func $<postcard::de::flavors::Slice as postcard::de::flavors::Flavor>::pop (;38;) (type 1) (param i32 i32)
    (local i32 i32)
    block ;; label = @1
      block ;; label = @2
        local.get 1
        i32.load
        local.tee 2
        local.get 1
        i32.load offset=4
        i32.eq
        local.tee 3
        i32.eqz
        br_if 0 (;@2;)
        i32.const 4
        local.set 1
        br 1 (;@1;)
      end
      local.get 1
      local.get 2
      i32.const 1
      i32.add
      i32.store
      local.get 2
      i32.load8_u
      local.set 1
    end
    local.get 0
    local.get 1
    i32.store8 offset=1
    local.get 0
    local.get 3
    i32.store8
  )
  (func $<serde::de::value::U32Deserializer<E> as serde::de::Deserializer>::deserialize_any (;39;) (type 1) (param i32 i32)
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
    call $serde::de::Visitor::visit_u32
    local.get 2
    i32.load8_u offset=9
    local.set 1
    local.get 0
    local.get 2
    i32.load8_u offset=8
    i32.const 1
    i32.and
    i32.store8
    local.get 0
    local.get 1
    i32.store8 offset=1
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $miden::asset::_::<impl serde::de::Deserialize for miden::asset::FungibleAsset>::deserialize (;40;) (type 1) (param i32 i32)
    (local i32 i64)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 1
    call $miden::account::_::<impl serde::de::Deserialize for miden::account::AccountId>::deserialize
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 2
          i32.load8_u
          br_if 0 (;@3;)
          local.get 2
          i64.load offset=8
          local.set 3
          local.get 2
          local.get 1
          call $serde::de::impls::<impl serde::de::Deserialize for u64>::deserialize
          block ;; label = @4
            local.get 2
            i32.load8_u
            br_if 0 (;@4;)
            local.get 0
            i32.const 16
            i32.add
            local.get 2
            i64.load offset=8
            i64.store
            local.get 0
            local.get 3
            i64.store offset=8
            i32.const 0
            local.set 1
            br 3 (;@1;)
          end
          local.get 0
          local.get 2
          i32.load8_u offset=1
          i32.store8 offset=1
          br 1 (;@2;)
        end
        local.get 0
        local.get 2
        i32.load8_u offset=1
        i32.store8 offset=1
      end
      i32.const 1
      local.set 1
    end
    local.get 0
    local.get 1
    i32.store8
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $miden::asset::_::<impl serde::de::Deserialize for miden::asset::NonFungibleAsset>::deserialize (;41;) (type 1) (param i32 i32)
    (local i32 i64 i64 i64 i64)
    global.get $__stack_pointer
    i32.const 80
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 40
    i32.add
    local.get 1
    call $miden::felt::_::<impl serde::de::Deserialize for miden::felt::Word>::deserialize
    block ;; label = @1
      block ;; label = @2
        local.get 2
        i32.load8_u offset=40
        br_if 0 (;@2;)
        local.get 2
        i32.const 32
        i32.add
        local.get 2
        i32.const 40
        i32.add
        i32.const 32
        i32.add
        i64.load
        local.tee 3
        i64.store align=2
        local.get 2
        i32.const 24
        i32.add
        local.get 2
        i32.const 40
        i32.add
        i32.const 24
        i32.add
        i64.load
        local.tee 4
        i64.store align=2
        local.get 2
        i32.const 16
        i32.add
        local.get 2
        i32.const 40
        i32.add
        i32.const 16
        i32.add
        i64.load
        local.tee 5
        i64.store align=2
        local.get 2
        local.get 2
        i64.load offset=48
        local.tee 6
        i64.store offset=8 align=2
        local.get 0
        i32.const 32
        i32.add
        local.get 3
        i64.store align=2
        local.get 0
        i32.const 24
        i32.add
        local.get 4
        i64.store align=2
        local.get 0
        i32.const 16
        i32.add
        local.get 5
        i64.store align=2
        local.get 0
        local.get 6
        i64.store offset=8 align=2
        i32.const 0
        local.set 1
        br 1 (;@1;)
      end
      local.get 0
      local.get 2
      i32.load8_u offset=41
      i32.store8 offset=1
      i32.const 1
      local.set 1
    end
    local.get 0
    local.get 1
    i32.store8
    local.get 2
    i32.const 80
    i32.add
    global.set $__stack_pointer
  )
  (func $<miden::asset::_::<impl serde::de::Deserialize for miden::asset::Asset>::deserialize::__FieldVisitor as serde::de::Visitor>::visit_u64 (;42;) (type 8) (param i32 i64)
    (local i32 i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i64.const 1
          i64.gt_u
          br_if 0 (;@3;)
          i32.const 0
          local.set 2
          i32.const 0
          local.set 3
          local.get 1
          i32.wrap_i64
          br_table 2 (;@1;) 1 (;@2;) 2 (;@1;)
        end
        i32.const 1
        local.set 2
        i32.const 14
        local.set 3
        br 1 (;@1;)
      end
      i32.const 1
      local.set 3
    end
    local.get 0
    local.get 3
    i32.store8 offset=1
    local.get 0
    local.get 2
    i32.store8
  )
  (func $alloc::raw_vec::RawVec<T,A>::allocate_in (;43;) (type 2) (param i32 i32 i32)
    (local i32 i32)
    i32.const 8
    local.set 3
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i32.eqz
          br_if 0 (;@3;)
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
          local.get 4
          i32.eqz
          br_if 0 (;@3;)
          block ;; label = @4
            block ;; label = @5
              local.get 2
              br_if 0 (;@5;)
              i32.const 0
              i32.load8_u offset=1048576
              drop
              local.get 4
              i32.const 8
              call $__rust_alloc
              local.set 3
              br 1 (;@4;)
            end
            local.get 4
            i32.const 8
            call $__rust_alloc_zeroed
            local.set 3
          end
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
        end
        local.get 0
        local.get 1
        i32.store offset=4
        local.get 0
        local.get 3
        i32.store
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
  (func $__rust_alloc (;44;) (type 4) (param i32 i32) (result i32)
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
  (func $__rust_alloc_zeroed (;45;) (type 4) (param i32 i32) (result i32)
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
  (func $miden::felt::_::<impl serde::de::Deserialize for miden::felt::Felt>::deserialize (;46;) (type 1) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 1
    call $serde::de::impls::<impl serde::de::Deserialize for u64>::deserialize
    block ;; label = @1
      block ;; label = @2
        local.get 2
        i32.load8_u
        br_if 0 (;@2;)
        local.get 0
        local.get 2
        i64.load offset=8
        i64.store offset=8
        i32.const 0
        local.set 1
        br 1 (;@1;)
      end
      local.get 0
      local.get 2
      i32.load8_u offset=1
      i32.store8 offset=1
      i32.const 1
      local.set 1
    end
    local.get 0
    local.get 1
    i32.store8
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $serde::de::impls::<impl serde::de::Deserialize for u64>::deserialize (;47;) (type 1) (param i32 i32)
    (local i32 i64 i64 i32 i32 i64)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i64.const 0
    local.set 3
    i64.const 0
    local.set 4
    block ;; label = @1
      block ;; label = @2
        loop ;; label = @3
          i32.const 5
          local.set 5
          local.get 3
          i64.const 70
          i64.eq
          br_if 1 (;@2;)
          local.get 2
          i32.const 8
          i32.add
          local.get 1
          call $<postcard::de::flavors::Slice as postcard::de::flavors::Flavor>::pop
          local.get 2
          i32.load8_u offset=9
          local.set 6
          block ;; label = @4
            local.get 2
            i32.load8_u offset=8
            i32.const 1
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            local.set 5
            br 2 (;@2;)
          end
          local.get 6
          i64.extend_i32_u
          i64.const 127
          i64.and
          local.get 3
          i64.shl
          local.get 4
          i64.or
          local.set 4
          local.get 3
          i64.const 7
          i64.add
          local.tee 7
          local.set 3
          local.get 6
          i32.extend8_s
          i32.const 0
          i32.lt_s
          br_if 0 (;@3;)
        end
        block ;; label = @3
          local.get 7
          i64.const 70
          i64.ne
          br_if 0 (;@3;)
          local.get 6
          i32.const 255
          i32.and
          i32.const 1
          i32.gt_u
          br_if 1 (;@2;)
        end
        local.get 0
        local.get 4
        i64.store offset=8
        i32.const 0
        local.set 6
        br 1 (;@1;)
      end
      local.get 0
      local.get 5
      i32.store8 offset=1
      i32.const 1
      local.set 6
    end
    local.get 0
    local.get 6
    i32.store8
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $miden::felt::_::<impl serde::de::Deserialize for miden::felt::Word>::deserialize (;48;) (type 1) (param i32 i32)
    (local i32 i64 i64 i64 i64)
    global.get $__stack_pointer
    i32.const 80
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 40
    i32.add
    local.get 1
    call $serde::de::impls::<impl serde::de::Deserialize for [T; 4]>::deserialize
    block ;; label = @1
      block ;; label = @2
        local.get 2
        i32.load8_u offset=40
        br_if 0 (;@2;)
        local.get 2
        i32.const 32
        i32.add
        local.get 2
        i32.const 40
        i32.add
        i32.const 32
        i32.add
        i64.load
        local.tee 3
        i64.store align=2
        local.get 2
        i32.const 24
        i32.add
        local.get 2
        i32.const 40
        i32.add
        i32.const 24
        i32.add
        i64.load
        local.tee 4
        i64.store align=2
        local.get 2
        i32.const 16
        i32.add
        local.get 2
        i32.const 40
        i32.add
        i32.const 16
        i32.add
        i64.load
        local.tee 5
        i64.store align=2
        local.get 2
        local.get 2
        i64.load offset=48
        local.tee 6
        i64.store offset=8 align=2
        local.get 0
        i32.const 32
        i32.add
        local.get 3
        i64.store align=2
        local.get 0
        i32.const 24
        i32.add
        local.get 4
        i64.store align=2
        local.get 0
        i32.const 16
        i32.add
        local.get 5
        i64.store align=2
        local.get 0
        local.get 6
        i64.store offset=8 align=2
        i32.const 0
        local.set 1
        br 1 (;@1;)
      end
      local.get 0
      local.get 2
      i32.load8_u offset=41
      i32.store8 offset=1
      i32.const 1
      local.set 1
    end
    local.get 0
    local.get 1
    i32.store8
    local.get 2
    i32.const 80
    i32.add
    global.set $__stack_pointer
  )
  (func $serde::de::impls::<impl serde::de::Deserialize for [T; 4]>::deserialize (;49;) (type 1) (param i32 i32)
    (local i32 i64 i64 i64 i64)
    global.get $__stack_pointer
    i32.const 80
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 4
    i32.store offset=12
    local.get 2
    local.get 1
    i32.store offset=8
    local.get 2
    i32.const 16
    i32.add
    local.get 2
    i32.const 8
    i32.add
    call $serde::de::SeqAccess::next_element
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 2
            i64.load offset=16
            local.tee 3
            i64.const 2
            i64.eq
            br_if 0 (;@4;)
            block ;; label = @5
              local.get 3
              i64.eqz
              i32.eqz
              br_if 0 (;@5;)
              local.get 0
              i32.const 3585
              i32.store16
              br 4 (;@1;)
            end
            local.get 2
            i64.load offset=24
            local.set 4
            local.get 2
            i32.const 32
            i32.add
            local.get 2
            i32.const 8
            i32.add
            call $serde::de::SeqAccess::next_element
            local.get 2
            i64.load offset=32
            local.tee 3
            i64.const 2
            i64.eq
            br_if 1 (;@3;)
            block ;; label = @5
              local.get 3
              i64.eqz
              i32.eqz
              br_if 0 (;@5;)
              local.get 0
              i32.const 3585
              i32.store16
              br 4 (;@1;)
            end
            local.get 2
            i64.load offset=40
            local.set 5
            local.get 2
            i32.const 48
            i32.add
            local.get 2
            i32.const 8
            i32.add
            call $serde::de::SeqAccess::next_element
            local.get 2
            i64.load offset=48
            local.tee 3
            i64.const 2
            i64.eq
            br_if 2 (;@2;)
            block ;; label = @5
              local.get 3
              i64.eqz
              i32.eqz
              br_if 0 (;@5;)
              local.get 0
              i32.const 3585
              i32.store16
              br 4 (;@1;)
            end
            local.get 2
            i64.load offset=56
            local.set 6
            local.get 2
            i32.const 64
            i32.add
            local.get 2
            i32.const 8
            i32.add
            call $serde::de::SeqAccess::next_element
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 2
                  i64.load offset=64
                  local.tee 3
                  i64.const 2
                  i64.gt_u
                  br_if 0 (;@7;)
                  i32.const 14
                  local.set 1
                  local.get 3
                  i32.wrap_i64
                  br_table 2 (;@5;) 0 (;@7;) 1 (;@6;) 2 (;@5;)
                end
                local.get 0
                local.get 4
                i64.store offset=8
                local.get 0
                i32.const 0
                i32.store8
                local.get 0
                i32.const 32
                i32.add
                local.get 2
                i64.load offset=72
                i64.store
                local.get 0
                i32.const 24
                i32.add
                local.get 6
                i64.store
                local.get 0
                i32.const 16
                i32.add
                local.get 5
                i64.store
                br 5 (;@1;)
              end
              local.get 2
              i32.load8_u offset=72
              local.set 1
            end
            local.get 0
            i32.const 1
            i32.store8
            local.get 0
            local.get 1
            i32.store8 offset=1
            br 3 (;@1;)
          end
          local.get 0
          local.get 2
          i32.load8_u offset=24
          i32.store8 offset=1
          local.get 0
          i32.const 1
          i32.store8
          br 2 (;@1;)
        end
        local.get 0
        local.get 2
        i32.load8_u offset=40
        i32.store8 offset=1
        local.get 0
        i32.const 1
        i32.store8
        br 1 (;@1;)
      end
      local.get 0
      local.get 2
      i32.load8_u offset=56
      i32.store8 offset=1
      local.get 0
      i32.const 1
      i32.store8
    end
    local.get 2
    i32.const 80
    i32.add
    global.set $__stack_pointer
  )
  (func $miden::account::_::<impl serde::de::Deserialize for miden::account::AccountId>::deserialize (;50;) (type 1) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 1
    call $miden::felt::_::<impl serde::de::Deserialize for miden::felt::Felt>::deserialize
    block ;; label = @1
      block ;; label = @2
        local.get 2
        i32.load8_u
        br_if 0 (;@2;)
        local.get 0
        local.get 2
        i64.load offset=8
        i64.store offset=8
        i32.const 0
        local.set 1
        br 1 (;@1;)
      end
      local.get 0
      local.get 2
      i32.load8_u offset=1
      i32.store8 offset=1
      i32.const 1
      local.set 1
    end
    local.get 0
    local.get 1
    i32.store8
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $serde::de::SeqAccess::next_element (;51;) (type 1) (param i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        local.get 1
        i32.load offset=4
        local.tee 3
        br_if 0 (;@2;)
        local.get 0
        i64.const 0
        i64.store
        br 1 (;@1;)
      end
      local.get 1
      local.get 3
      i32.const -1
      i32.add
      i32.store offset=4
      local.get 2
      local.get 1
      i32.load
      call $miden::felt::_::<impl serde::de::Deserialize for miden::felt::Felt>::deserialize
      block ;; label = @2
        local.get 2
        i32.load8_u
        br_if 0 (;@2;)
        local.get 0
        local.get 2
        i64.load offset=8
        i64.store offset=8
        local.get 0
        i64.const 1
        i64.store
        br 1 (;@1;)
      end
      local.get 2
      i32.load8_u offset=1
      local.set 1
      local.get 0
      i64.const 2
      i64.store
      local.get 0
      local.get 1
      i32.store8 offset=8
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $serde::de::Visitor::visit_u32 (;52;) (type 1) (param i32 i32)
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
    i64.extend_i32_u
    call $<miden::asset::_::<impl serde::de::Deserialize for miden::asset::Asset>::deserialize::__FieldVisitor as serde::de::Visitor>::visit_u64
    local.get 2
    i32.load8_u offset=9
    local.set 1
    local.get 0
    local.get 2
    i32.load8_u offset=8
    i32.const 1
    i32.and
    i32.store8
    local.get 0
    local.get 1
    i32.store8 offset=1
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $dlmalloc::Dlmalloc<A>::malloc (;53;) (type 0) (param i32 i32 i32) (result i32)
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
  (func $postcard::de::from_bytes (;54;) (type 2) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 8
    i32.add
    local.get 1
    local.get 2
    call $postcard::de::deserializer::Deserializer<postcard::de::flavors::Slice>::from_bytes
    local.get 3
    local.get 3
    i64.load offset=8
    i64.store offset=16 align=4
    local.get 3
    i32.const 24
    i32.add
    local.get 3
    i32.const 16
    i32.add
    call $basic_wallet::MyWallet::receive_asset_account_export::_::<impl serde::de::Deserialize for basic_wallet::MyWallet::receive_asset_account_export::Args>::deserialize
    block ;; label = @1
      block ;; label = @2
        local.get 3
        i32.load offset=24
        local.tee 2
        i32.const 2
        i32.eq
        br_if 0 (;@2;)
        local.get 3
        i32.load8_u offset=28
        local.set 1
        local.get 0
        i32.const 5
        i32.add
        local.get 3
        i32.const 24
        i32.add
        i32.const 5
        i32.or
        i32.const 35
        call $memcpy
        drop
        local.get 0
        local.get 1
        i32.store8 offset=4
        br 1 (;@1;)
      end
      local.get 0
      local.get 3
      i32.load8_u offset=28
      i32.store8 offset=4
    end
    local.get 0
    local.get 2
    i32.store
    local.get 3
    i32.const 64
    i32.add
    global.set $__stack_pointer
  )
  (func $basic_wallet::MyWallet::receive_asset_account_export::_::<impl serde::de::Deserialize for basic_wallet::MyWallet::receive_asset_account_export::Args>::deserialize (;55;) (type 1) (param i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 8
    i32.add
    local.get 1
    call $<miden::asset::_::<impl serde::de::Deserialize for miden::asset::Asset>::deserialize::__Visitor as serde::de::Visitor>::visit_enum
    block ;; label = @1
      block ;; label = @2
        local.get 2
        i32.load offset=8
        local.tee 1
        i32.const 2
        i32.eq
        br_if 0 (;@2;)
        local.get 2
        i32.load8_u offset=12
        local.set 3
        local.get 0
        i32.const 5
        i32.add
        local.get 2
        i32.const 8
        i32.add
        i32.const 5
        i32.or
        i32.const 35
        call $memcpy
        drop
        local.get 0
        local.get 3
        i32.store8 offset=4
        br 1 (;@1;)
      end
      local.get 0
      local.get 2
      i32.load8_u offset=12
      i32.store8 offset=4
    end
    local.get 0
    local.get 1
    i32.store
    local.get 2
    i32.const 48
    i32.add
    global.set $__stack_pointer
  )
  (func $basic_wallet::MyWallet::receive_asset (;56;) (type 1) (param i32 i32)
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
    call $miden::account::add_asset
    local.get 2
    i32.const 48
    i32.add
    global.set $__stack_pointer
  )
  (func $my_wallet::receive_asset (;57;) (type 9) (param i32 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 112
    i32.sub
    local.tee 17
    global.set $__stack_pointer
    block ;; label = @1
      i64.const 1
      call $miden::note::Tag::new
      local.get 1
      i64.ne
      br_if 0 (;@1;)
      local.get 17
      i32.const 8
      i32.add
      i32.const 3
      i32.const 0
      call $alloc::raw_vec::RawVec<T,A>::allocate_in
      local.get 17
      i32.load offset=12
      local.set 18
      local.get 17
      i32.load offset=8
      local.tee 19
      local.get 3
      i64.store offset=8
      local.get 19
      local.get 2
      i64.store
      local.get 19
      local.get 4
      i64.store offset=16
      local.get 17
      i32.const 3
      i32.store offset=80
      local.get 17
      local.get 18
      i32.store offset=76
      local.get 17
      local.get 19
      i32.store offset=72
      local.get 17
      i32.const 60
      i32.add
      local.get 17
      i32.const 72
      i32.add
      call $miden::serialization::bytes_to_felts
      local.get 17
      i32.const 72
      i32.add
      local.get 17
      i32.load offset=60
      local.get 17
      i32.load offset=68
      call $postcard::de::from_bytes
      local.get 17
      i32.load offset=72
      i32.const 2
      i32.eq
      br_if 0 (;@1;)
      local.get 17
      i32.const 16
      i32.add
      local.get 17
      i32.const 72
      i32.add
      i32.const 40
      call $memcpy
      drop
      local.get 17
      i32.const 60
      i32.add
      call $<alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
      local.get 17
      i32.const 60
      i32.add
      call $<alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
      local.get 17
      local.get 17
      i32.const 16
      i32.add
      call $basic_wallet::MyWallet::receive_asset
      local.get 17
      i32.const 112
      i32.add
      global.set $__stack_pointer
      return
    end
    unreachable
    unreachable
  )
  (func $__rust_dealloc (;58;) (type 2) (param i32 i32 i32)
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
  (func $__rust_alloc_error_handler (;59;) (type 1) (param i32 i32)
    local.get 0
    local.get 1
    call $alloc::alloc::handle_alloc_error::ct_error
    return
  )
  (func $postcard::de::deserializer::Deserializer<postcard::de::flavors::Slice>::from_bytes (;60;) (type 2) (param i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    i32.add
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
  )
  (func $dlmalloc::dlmalloc::align_up (;61;) (type 4) (param i32 i32) (result i32)
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
  (func $dlmalloc::dlmalloc::left_bits (;62;) (type 5) (param i32) (result i32)
    local.get 0
    i32.const 1
    i32.shl
    local.tee 0
    i32.const 0
    local.get 0
    i32.sub
    i32.or
  )
  (func $dlmalloc::dlmalloc::least_bit (;63;) (type 5) (param i32) (result i32)
    i32.const 0
    local.get 0
    i32.sub
    local.get 0
    i32.and
  )
  (func $dlmalloc::dlmalloc::leftshift_for_tree_index (;64;) (type 5) (param i32) (result i32)
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
  (func $dlmalloc::dlmalloc::Chunk::fencepost_head (;65;) (type 10) (result i32)
    i32.const 7
  )
  (func $dlmalloc::dlmalloc::Chunk::size (;66;) (type 5) (param i32) (result i32)
    local.get 0
    i32.load offset=4
    i32.const -8
    i32.and
  )
  (func $dlmalloc::dlmalloc::Chunk::cinuse (;67;) (type 5) (param i32) (result i32)
    local.get 0
    i32.load8_u offset=4
    i32.const 2
    i32.and
    i32.const 1
    i32.shr_u
  )
  (func $dlmalloc::dlmalloc::Chunk::pinuse (;68;) (type 5) (param i32) (result i32)
    local.get 0
    i32.load offset=4
    i32.const 1
    i32.and
  )
  (func $dlmalloc::dlmalloc::Chunk::inuse (;69;) (type 5) (param i32) (result i32)
    local.get 0
    i32.load offset=4
    i32.const 3
    i32.and
    i32.const 1
    i32.ne
  )
  (func $dlmalloc::dlmalloc::Chunk::mmapped (;70;) (type 5) (param i32) (result i32)
    local.get 0
    i32.load8_u offset=4
    i32.const 3
    i32.and
    i32.eqz
  )
  (func $dlmalloc::dlmalloc::Chunk::set_inuse (;71;) (type 1) (param i32 i32)
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
  (func $dlmalloc::dlmalloc::Chunk::set_inuse_and_pinuse (;72;) (type 1) (param i32 i32)
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
  (func $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_inuse_chunk (;73;) (type 1) (param i32 i32)
    local.get 0
    local.get 1
    i32.const 3
    i32.or
    i32.store offset=4
  )
  (func $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_free_chunk (;74;) (type 1) (param i32 i32)
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
  (func $dlmalloc::dlmalloc::Chunk::set_free_with_pinuse (;75;) (type 2) (param i32 i32 i32)
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
  (func $dlmalloc::dlmalloc::Chunk::plus_offset (;76;) (type 4) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.add
  )
  (func $dlmalloc::dlmalloc::Chunk::minus_offset (;77;) (type 4) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.sub
  )
  (func $dlmalloc::dlmalloc::Chunk::to_mem (;78;) (type 5) (param i32) (result i32)
    local.get 0
    i32.const 8
    i32.add
  )
  (func $dlmalloc::dlmalloc::Chunk::mem_offset (;79;) (type 10) (result i32)
    i32.const 8
  )
  (func $dlmalloc::dlmalloc::Chunk::from_mem (;80;) (type 5) (param i32) (result i32)
    local.get 0
    i32.const -8
    i32.add
  )
  (func $dlmalloc::dlmalloc::TreeChunk::leftmost_child (;81;) (type 5) (param i32) (result i32)
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
  (func $dlmalloc::dlmalloc::TreeChunk::chunk (;82;) (type 5) (param i32) (result i32)
    local.get 0
  )
  (func $dlmalloc::dlmalloc::TreeChunk::next (;83;) (type 5) (param i32) (result i32)
    local.get 0
    i32.load offset=12
  )
  (func $dlmalloc::dlmalloc::TreeChunk::prev (;84;) (type 5) (param i32) (result i32)
    local.get 0
    i32.load offset=8
  )
  (func $dlmalloc::dlmalloc::Segment::is_extern (;85;) (type 5) (param i32) (result i32)
    local.get 0
    i32.load offset=12
    i32.const 1
    i32.and
  )
  (func $dlmalloc::dlmalloc::Segment::sys_flags (;86;) (type 5) (param i32) (result i32)
    local.get 0
    i32.load offset=12
    i32.const 1
    i32.shr_u
  )
  (func $dlmalloc::dlmalloc::Segment::holds (;87;) (type 4) (param i32 i32) (result i32)
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
  (func $dlmalloc::dlmalloc::Segment::top (;88;) (type 5) (param i32) (result i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    i32.add
  )
  (func $<dlmalloc::global::Instance as core::ops::deref::DerefMut>::deref_mut (;89;) (type 5) (param i32) (result i32)
    i32.const 1048580
  )
  (func $<dlmalloc::global::Instance as core::ops::drop::Drop>::drop (;90;) (type 7) (param i32))
  (func $<dlmalloc::sys::System as dlmalloc::Allocator>::alloc (;91;) (type 2) (param i32 i32 i32)
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
  (func $<dlmalloc::sys::System as dlmalloc::Allocator>::free_part (;92;) (type 6) (param i32 i32 i32 i32) (result i32)
    i32.const 0
  )
  (func $<dlmalloc::sys::System as dlmalloc::Allocator>::free (;93;) (type 0) (param i32 i32 i32) (result i32)
    i32.const 0
  )
  (func $<dlmalloc::sys::System as dlmalloc::Allocator>::can_release_part (;94;) (type 4) (param i32 i32) (result i32)
    i32.const 0
  )
  (func $<dlmalloc::sys::System as dlmalloc::Allocator>::allocates_zeros (;95;) (type 5) (param i32) (result i32)
    i32.const 1
  )
  (func $dlmalloc::sys::enable_alloc_after_fork (;96;) (type 11))
  (func $miden::note::Tag::new (;97;) (type 12) (param i64) (result i64)
    local.get 0
  )
  (func $miden::serialization::bytes_to_felts (;98;) (type 1) (param i32 i32)
    unreachable
    unreachable
  )
  (func $miden::account::add_asset (;99;) (type 1) (param i32 i32)
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
    i32.const 40
    call $memcpy
    drop
    local.get 0
    local.get 2
    i32.const 8
    i32.add
    call $miden::sat::account::add_asset
    local.get 2
    i32.const 48
    i32.add
    global.set $__stack_pointer
  )
  (func $<alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop (;100;) (type 7) (param i32))
  (func $alloc::alloc::handle_alloc_error (;101;) (type 1) (param i32 i32)
    local.get 0
    local.get 1
    call $alloc::alloc::handle_alloc_error::rt_error
    unreachable
  )
  (func $alloc::alloc::handle_alloc_error::rt_error (;102;) (type 1) (param i32 i32)
    local.get 1
    local.get 0
    call $__rust_alloc_error_handler
    unreachable
  )
  (func $alloc::alloc::handle_alloc_error::ct_error (;103;) (type 1) (param i32 i32)
    unreachable
    unreachable
  )
  (func $<alloc::alloc::Global as core::alloc::Allocator>::deallocate (;104;) (type 2) (param i32 i32 i32)
    block ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 2
      local.get 1
      call $__rust_dealloc
    end
  )
  (func $<alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop (;105;) (type 7) (param i32)
    (local i32)
    block ;; label = @1
      local.get 0
      i32.load offset=4
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load
      i32.const 1
      local.get 1
      call $<alloc::alloc::Global as core::alloc::Allocator>::deallocate
    end
  )
  (func $alloc::raw_vec::capacity_overflow (;106;) (type 11)
    unreachable
    unreachable
  )
  (table (;0;) 1 1 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (export "memory" (memory 0))
  (export "my_wallet::receive_asset" (func $my_wallet::receive_asset))
)