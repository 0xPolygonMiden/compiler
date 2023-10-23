(module
  (type (;0;) (func (param i32 i32 i32)))
  (type (;1;) (func (param i32 i32)))
  (type (;2;) (func (param i32) (result i32)))
  (type (;3;) (func (param i32 i32) (result i32)))
  (type (;4;) (func (param i32 i32 i32) (result i32)))
  (type (;5;) (func (result i32)))
  (type (;6;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;7;) (func (param i32 i32 i32 i32)))
  (type (;8;) (func (param i32)))
  (type (;9;) (func))
  (type (;10;) (func (param i32 i32 i32 i32 i32) (result i32)))
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::dispose_chunk (;0;) (type 0) (param i32 i32 i32)
    (local i32 i32 i32 i32)
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
            local.get 1
            call $dlmalloc::dlmalloc::Chunk::mmapped
            br_if 0 (;@4;)
            local.get 4
            local.get 2
            i32.add
            local.set 2
            block ;; label = @5
              local.get 1
              local.get 4
              call $dlmalloc::dlmalloc::Chunk::minus_offset
              local.tee 1
              local.get 0
              i32.load offset=424
              i32.ne
              br_if 0 (;@5;)
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
            block ;; label = @5
              local.get 4
              i32.const 256
              i32.lt_u
              br_if 0 (;@5;)
              local.get 0
              local.get 1
              call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_large_chunk
              br 2 (;@3;)
            end
            block ;; label = @5
              local.get 1
              i32.load offset=12
              local.tee 5
              local.get 1
              i32.load offset=8
              local.tee 6
              i32.eq
              br_if 0 (;@5;)
              local.get 6
              local.get 5
              i32.store offset=12
              local.get 5
              local.get 6
              i32.store offset=8
              br 2 (;@3;)
            end
            local.get 0
            local.get 0
            i32.load offset=408
            i32.const -2
            local.get 4
            i32.const 3
            i32.shr_u
            i32.rotl
            i32.and
            i32.store offset=408
            br 1 (;@3;)
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
          br_if 1 (;@2;)
          local.get 0
          local.get 0
          i32.load offset=432
          local.get 1
          i32.sub
          i32.store offset=432
          return
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
            local.get 3
            call $dlmalloc::dlmalloc::Chunk::size
            local.tee 4
            local.get 2
            i32.add
            local.set 2
            block ;; label = @5
              block ;; label = @6
                local.get 4
                i32.const 256
                i32.lt_u
                br_if 0 (;@6;)
                local.get 0
                local.get 3
                call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_large_chunk
                br 1 (;@5;)
              end
              block ;; label = @6
                local.get 3
                i32.load offset=12
                local.tee 5
                local.get 3
                i32.load offset=8
                local.tee 3
                i32.eq
                br_if 0 (;@6;)
                local.get 3
                local.get 5
                i32.store offset=12
                local.get 5
                local.get 3
                i32.store offset=8
                br 1 (;@5;)
              end
              local.get 0
              local.get 0
              i32.load offset=408
              i32.const -2
              local.get 4
              i32.const 3
              i32.shr_u
              i32.rotl
              i32.and
              i32.store offset=408
            end
            local.get 1
            local.get 2
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
        local.set 2
        br 1 (;@1;)
      end
      local.get 0
      local.get 4
      local.get 2
      i32.or
      i32.store offset=408
      local.get 3
      local.set 2
    end
    local.get 3
    local.get 1
    i32.store offset=8
    local.get 2
    local.get 1
    i32.store offset=12
    local.get 1
    local.get 3
    i32.store offset=12
    local.get 1
    local.get 2
    i32.store offset=8
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_large_chunk (;1;) (type 1) (param i32 i32)
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
          local.set 3
          br 2 (;@1;)
        end
        local.get 1
        call $dlmalloc::dlmalloc::TreeChunk::prev
        local.tee 5
        local.get 1
        call $dlmalloc::dlmalloc::TreeChunk::next
        local.tee 3
        call $dlmalloc::dlmalloc::TreeChunk::chunk
        i32.store offset=12
        local.get 3
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
      local.set 4
      loop ;; label = @2
        local.get 4
        local.set 6
        local.get 5
        local.tee 3
        i32.const 20
        i32.add
        local.tee 5
        local.get 3
        i32.const 16
        i32.add
        local.get 5
        i32.load
        local.tee 5
        select
        local.set 4
        local.get 3
        i32.const 20
        i32.const 16
        local.get 5
        select
        i32.add
        i32.load
        local.tee 5
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
          local.tee 4
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
          local.get 3
          i32.store
          local.get 3
          br_if 1 (;@2;)
          br 2 (;@1;)
        end
        local.get 5
        local.get 3
        i32.store
        local.get 3
        br_if 0 (;@2;)
        local.get 0
        local.get 0
        i32.load offset=412
        i32.const -2
        local.get 4
        i32.rotl
        i32.and
        i32.store offset=412
        return
      end
      local.get 3
      local.get 2
      i32.store offset=24
      block ;; label = @2
        local.get 1
        i32.load offset=16
        local.tee 5
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        local.get 5
        i32.store offset=16
        local.get 5
        local.get 3
        i32.store offset=24
      end
      local.get 1
      i32.const 20
      i32.add
      i32.load
      local.tee 5
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.const 20
      i32.add
      local.get 5
      i32.store
      local.get 5
      local.get 3
      i32.store offset=24
      return
    end
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::insert_large_chunk (;2;) (type 0) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32)
    i32.const 0
    local.set 3
    block ;; label = @1
      local.get 2
      i32.const 256
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 31
      local.set 3
      local.get 2
      i32.const 16777215
      i32.gt_u
      br_if 0 (;@1;)
      local.get 2
      i32.const 6
      local.get 2
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
    local.get 1
    i64.const 0
    i64.store offset=16 align=4
    local.get 1
    local.get 3
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
      local.get 0
      local.get 6
      local.get 7
      i32.or
      i32.store offset=412
      local.get 1
      local.get 4
      i32.store offset=24
      local.get 4
      local.get 1
      i32.store
    end
    local.get 5
    local.get 5
    i32.store offset=8
    local.get 5
    local.get 5
    i32.store offset=12
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::release_unused_segments (;3;) (type 2) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.const 136
        i32.add
        i32.load
        local.tee 1
        br_if 0 (;@2;)
        i32.const 0
        local.set 2
        i32.const 0
        local.set 3
        br 1 (;@1;)
      end
      local.get 0
      i32.const 128
      i32.add
      local.set 4
      i32.const 0
      local.set 3
      i32.const 0
      local.set 2
      loop ;; label = @2
        local.get 1
        local.tee 5
        i32.load offset=8
        local.set 1
        local.get 5
        i32.load offset=4
        local.set 6
        local.get 5
        i32.load
        local.set 7
        block ;; label = @3
          block ;; label = @4
            local.get 0
            local.get 5
            i32.load offset=12
            i32.const 1
            i32.shr_u
            call $<dlmalloc::sys::System as dlmalloc::Allocator>::can_release_part
            i32.eqz
            br_if 0 (;@4;)
            local.get 5
            call $dlmalloc::dlmalloc::Segment::is_extern
            br_if 0 (;@4;)
            local.get 7
            local.get 7
            call $dlmalloc::dlmalloc::Chunk::to_mem
            local.tee 8
            i32.const 8
            call $dlmalloc::dlmalloc::align_up
            local.get 8
            i32.sub
            i32.add
            local.tee 8
            call $dlmalloc::dlmalloc::Chunk::size
            local.set 9
            call $dlmalloc::dlmalloc::Chunk::mem_offset
            local.tee 10
            i32.const 8
            call $dlmalloc::dlmalloc::align_up
            local.set 11
            i32.const 20
            i32.const 8
            call $dlmalloc::dlmalloc::align_up
            local.set 12
            i32.const 16
            i32.const 8
            call $dlmalloc::dlmalloc::align_up
            local.set 13
            local.get 8
            call $dlmalloc::dlmalloc::Chunk::inuse
            br_if 0 (;@4;)
            local.get 8
            local.get 9
            i32.add
            local.get 7
            local.get 10
            local.get 6
            i32.add
            local.get 11
            local.get 12
            i32.add
            local.get 13
            i32.add
            i32.sub
            i32.add
            i32.lt_u
            br_if 0 (;@4;)
            block ;; label = @5
              block ;; label = @6
                local.get 8
                local.get 0
                i32.load offset=424
                i32.eq
                br_if 0 (;@6;)
                local.get 0
                local.get 8
                call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_large_chunk
                br 1 (;@5;)
              end
              local.get 0
              i32.const 0
              i32.store offset=416
              local.get 0
              i32.const 0
              i32.store offset=424
            end
            block ;; label = @5
              local.get 0
              local.get 7
              local.get 6
              call $<dlmalloc::sys::System as dlmalloc::Allocator>::free
              br_if 0 (;@5;)
              local.get 0
              local.get 8
              local.get 9
              call $dlmalloc::dlmalloc::Dlmalloc<A>::insert_large_chunk
              br 1 (;@4;)
            end
            local.get 0
            local.get 0
            i32.load offset=432
            local.get 6
            i32.sub
            i32.store offset=432
            local.get 4
            local.get 1
            i32.store offset=8
            local.get 6
            local.get 3
            i32.add
            local.set 3
            br 1 (;@3;)
          end
          local.get 5
          local.set 4
        end
        local.get 2
        i32.const 1
        i32.add
        local.set 2
        local.get 1
        br_if 0 (;@2;)
      end
    end
    local.get 0
    local.get 2
    i32.const 4095
    local.get 2
    i32.const 4095
    i32.gt_u
    select
    i32.store offset=448
    local.get 3
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::free (;4;) (type 1) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32)
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
          local.get 1
          call $dlmalloc::dlmalloc::Chunk::mmapped
          br_if 0 (;@3;)
          local.get 4
          local.get 2
          i32.add
          local.set 2
          block ;; label = @4
            local.get 1
            local.get 4
            call $dlmalloc::dlmalloc::Chunk::minus_offset
            local.tee 1
            local.get 0
            i32.load offset=424
            i32.ne
            br_if 0 (;@4;)
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
          block ;; label = @4
            local.get 4
            i32.const 256
            i32.lt_u
            br_if 0 (;@4;)
            local.get 0
            local.get 1
            call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_large_chunk
            br 2 (;@2;)
          end
          block ;; label = @4
            local.get 1
            i32.load offset=12
            local.tee 5
            local.get 1
            i32.load offset=8
            local.tee 6
            i32.eq
            br_if 0 (;@4;)
            local.get 6
            local.get 5
            i32.store offset=12
            local.get 5
            local.get 6
            i32.store offset=8
            br 2 (;@2;)
          end
          local.get 0
          local.get 0
          i32.load offset=408
          i32.const -2
          local.get 4
          i32.const 3
          i32.shr_u
          i32.rotl
          i32.and
          i32.store offset=408
          br 1 (;@2;)
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
        br_if 1 (;@1;)
        local.get 0
        local.get 0
        i32.load offset=432
        local.get 1
        i32.sub
        i32.store offset=432
        return
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
                local.get 3
                call $dlmalloc::dlmalloc::Chunk::size
                local.tee 4
                local.get 2
                i32.add
                local.set 2
                block ;; label = @7
                  block ;; label = @8
                    local.get 4
                    i32.const 256
                    i32.lt_u
                    br_if 0 (;@8;)
                    local.get 0
                    local.get 3
                    call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_large_chunk
                    br 1 (;@7;)
                  end
                  block ;; label = @8
                    local.get 3
                    i32.load offset=12
                    local.tee 5
                    local.get 3
                    i32.load offset=8
                    local.tee 3
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 3
                    local.get 5
                    i32.store offset=12
                    local.get 5
                    local.get 3
                    i32.store offset=8
                    br 1 (;@7;)
                  end
                  local.get 0
                  local.get 0
                  i32.load offset=408
                  i32.const -2
                  local.get 4
                  i32.const 3
                  i32.shr_u
                  i32.rotl
                  i32.and
                  i32.store offset=408
                end
                local.get 1
                local.get 2
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
        local.get 2
        local.get 0
        i32.load offset=440
        i32.le_u
        br_if 1 (;@1;)
        call $dlmalloc::dlmalloc::Chunk::mem_offset
        local.tee 1
        i32.const 8
        call $dlmalloc::dlmalloc::align_up
        local.set 2
        i32.const 20
        i32.const 8
        call $dlmalloc::dlmalloc::align_up
        local.set 3
        i32.const 16
        i32.const 8
        call $dlmalloc::dlmalloc::align_up
        local.set 4
        i32.const 0
        i32.const 16
        i32.const 8
        call $dlmalloc::dlmalloc::align_up
        i32.const 2
        i32.shl
        i32.sub
        local.tee 5
        local.get 1
        local.get 4
        local.get 2
        local.get 3
        i32.add
        i32.add
        i32.sub
        i32.const -65544
        i32.add
        i32.const -9
        i32.and
        i32.const -3
        i32.add
        local.tee 1
        local.get 5
        local.get 1
        i32.lt_u
        select
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.load offset=428
        local.tee 2
        i32.eqz
        br_if 1 (;@1;)
        call $dlmalloc::dlmalloc::Chunk::mem_offset
        local.tee 1
        i32.const 8
        call $dlmalloc::dlmalloc::align_up
        local.set 3
        i32.const 20
        i32.const 8
        call $dlmalloc::dlmalloc::align_up
        local.set 5
        i32.const 16
        i32.const 8
        call $dlmalloc::dlmalloc::align_up
        local.set 6
        i32.const 0
        local.set 4
        block ;; label = @3
          local.get 0
          i32.load offset=420
          local.tee 7
          local.get 6
          local.get 5
          local.get 3
          local.get 1
          i32.sub
          i32.add
          i32.add
          local.tee 1
          i32.le_u
          br_if 0 (;@3;)
          local.get 7
          local.get 1
          i32.sub
          i32.const 65535
          i32.add
          i32.const -65536
          i32.and
          local.tee 6
          i32.const -65536
          i32.add
          local.set 5
          local.get 0
          i32.const 128
          i32.add
          local.tee 3
          local.set 1
          block ;; label = @4
            loop ;; label = @5
              block ;; label = @6
                local.get 1
                i32.load
                local.get 2
                i32.gt_u
                br_if 0 (;@6;)
                local.get 1
                call $dlmalloc::dlmalloc::Segment::top
                local.get 2
                i32.gt_u
                br_if 2 (;@4;)
              end
              local.get 1
              i32.load offset=8
              local.tee 1
              br_if 0 (;@5;)
            end
            i32.const 0
            local.set 1
          end
          i32.const 0
          local.set 4
          local.get 1
          call $dlmalloc::dlmalloc::Segment::is_extern
          br_if 0 (;@3;)
          i32.const 0
          local.set 4
          local.get 0
          local.get 1
          i32.load offset=12
          i32.const 1
          i32.shr_u
          call $<dlmalloc::sys::System as dlmalloc::Allocator>::can_release_part
          i32.eqz
          br_if 0 (;@3;)
          i32.const 0
          local.set 4
          local.get 1
          i32.load offset=4
          local.get 5
          i32.lt_u
          br_if 0 (;@3;)
          loop ;; label = @4
            block ;; label = @5
              local.get 1
              local.get 3
              call $dlmalloc::dlmalloc::Segment::holds
              i32.eqz
              br_if 0 (;@5;)
              i32.const 0
              local.set 4
              br 2 (;@3;)
            end
            local.get 3
            i32.load offset=8
            local.tee 3
            br_if 0 (;@4;)
          end
          local.get 0
          local.get 1
          i32.load
          local.get 1
          i32.load offset=4
          local.tee 2
          local.get 2
          local.get 5
          i32.sub
          call $<dlmalloc::sys::System as dlmalloc::Allocator>::free_part
          local.set 2
          i32.const 0
          local.set 4
          local.get 5
          i32.eqz
          br_if 0 (;@3;)
          i32.const 0
          local.set 4
          local.get 2
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          local.get 1
          i32.load offset=4
          local.get 5
          i32.sub
          i32.store offset=4
          local.get 0
          local.get 0
          i32.load offset=432
          local.get 5
          i32.sub
          i32.store offset=432
          local.get 0
          i32.load offset=420
          local.set 2
          local.get 0
          i32.load offset=428
          local.set 1
          local.get 0
          local.get 1
          local.get 1
          call $dlmalloc::dlmalloc::Chunk::to_mem
          local.tee 3
          i32.const 8
          call $dlmalloc::dlmalloc::align_up
          local.get 3
          i32.sub
          local.tee 3
          call $dlmalloc::dlmalloc::Chunk::plus_offset
          local.tee 1
          i32.store offset=428
          local.get 0
          local.get 2
          local.get 6
          local.get 3
          i32.add
          i32.sub
          i32.const 65536
          i32.add
          local.tee 2
          i32.store offset=420
          local.get 1
          local.get 2
          i32.const 1
          i32.or
          i32.store offset=4
          call $dlmalloc::dlmalloc::Chunk::mem_offset
          local.tee 3
          i32.const 8
          call $dlmalloc::dlmalloc::align_up
          local.set 4
          i32.const 20
          i32.const 8
          call $dlmalloc::dlmalloc::align_up
          local.set 6
          i32.const 16
          i32.const 8
          call $dlmalloc::dlmalloc::align_up
          local.set 7
          local.get 1
          local.get 2
          call $dlmalloc::dlmalloc::Chunk::plus_offset
          local.set 1
          local.get 0
          i32.const 2097152
          i32.store offset=440
          local.get 1
          local.get 7
          local.get 6
          local.get 4
          local.get 3
          i32.sub
          i32.add
          i32.add
          i32.store offset=4
          local.get 5
          local.set 4
        end
        local.get 0
        call $dlmalloc::dlmalloc::Dlmalloc<A>::release_unused_segments
        i32.const 0
        local.get 4
        i32.sub
        i32.ne
        br_if 1 (;@1;)
        local.get 0
        i32.load offset=420
        local.get 0
        i32.load offset=440
        i32.le_u
        br_if 1 (;@1;)
        local.get 0
        i32.const -1
        i32.store offset=440
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
      local.get 2
      i32.const -8
      i32.and
      i32.add
      i32.const 144
      i32.add
      local.set 3
      block ;; label = @2
        block ;; label = @3
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
          br_if 0 (;@3;)
          local.get 3
          i32.load offset=8
          local.set 0
          br 1 (;@2;)
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
    end
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::malloc (;5;) (type 3) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 1
                i32.const 245
                i32.lt_u
                br_if 0 (;@6;)
                call $dlmalloc::dlmalloc::Chunk::mem_offset
                local.tee 3
                i32.const 8
                call $dlmalloc::dlmalloc::align_up
                local.set 4
                i32.const 20
                i32.const 8
                call $dlmalloc::dlmalloc::align_up
                local.set 5
                i32.const 16
                i32.const 8
                call $dlmalloc::dlmalloc::align_up
                local.set 6
                i32.const 0
                local.set 7
                i32.const 0
                i32.const 16
                i32.const 8
                call $dlmalloc::dlmalloc::align_up
                i32.const 2
                i32.shl
                i32.sub
                local.tee 8
                local.get 3
                local.get 6
                local.get 4
                local.get 5
                i32.add
                i32.add
                i32.sub
                i32.const -65544
                i32.add
                i32.const -9
                i32.and
                i32.const -3
                i32.add
                local.tee 3
                local.get 8
                local.get 3
                i32.lt_u
                select
                local.get 1
                i32.le_u
                br_if 5 (;@1;)
                local.get 1
                i32.const 4
                i32.add
                i32.const 8
                call $dlmalloc::dlmalloc::align_up
                local.set 3
                local.get 0
                i32.load offset=412
                local.tee 9
                i32.eqz
                br_if 4 (;@2;)
                i32.const 0
                local.set 1
                i32.const 0
                local.set 10
                block ;; label = @7
                  local.get 3
                  i32.const 256
                  i32.lt_u
                  br_if 0 (;@7;)
                  i32.const 31
                  local.set 10
                  local.get 3
                  i32.const 16777215
                  i32.gt_u
                  br_if 0 (;@7;)
                  local.get 3
                  i32.const 6
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
                  local.set 10
                end
                i32.const 0
                local.get 3
                i32.sub
                local.set 4
                block ;; label = @7
                  local.get 0
                  local.get 10
                  i32.const 2
                  i32.shl
                  i32.add
                  i32.load
                  local.tee 7
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 5
                  br 2 (;@5;)
                end
                local.get 3
                local.get 10
                call $dlmalloc::dlmalloc::leftshift_for_tree_index
                i32.shl
                local.set 6
                i32.const 0
                local.set 1
                i32.const 0
                local.set 5
                loop ;; label = @7
                  block ;; label = @8
                    local.get 7
                    call $dlmalloc::dlmalloc::TreeChunk::chunk
                    call $dlmalloc::dlmalloc::Chunk::size
                    local.tee 8
                    local.get 3
                    i32.lt_u
                    br_if 0 (;@8;)
                    local.get 8
                    local.get 3
                    i32.sub
                    local.tee 8
                    local.get 4
                    i32.ge_u
                    br_if 0 (;@8;)
                    local.get 8
                    local.set 4
                    local.get 7
                    local.set 5
                    local.get 8
                    br_if 0 (;@8;)
                    i32.const 0
                    local.set 4
                    local.get 7
                    local.set 5
                    local.get 7
                    local.set 1
                    br 4 (;@4;)
                  end
                  local.get 7
                  i32.const 20
                  i32.add
                  i32.load
                  local.tee 8
                  local.get 1
                  local.get 8
                  local.get 7
                  local.get 6
                  i32.const 29
                  i32.shr_u
                  i32.const 4
                  i32.and
                  i32.add
                  i32.const 16
                  i32.add
                  i32.load
                  local.tee 7
                  i32.ne
                  select
                  local.get 1
                  local.get 8
                  select
                  local.set 1
                  local.get 6
                  i32.const 1
                  i32.shl
                  local.set 6
                  local.get 7
                  i32.eqz
                  br_if 2 (;@5;)
                  br 0 (;@7;)
                end
              end
              i32.const 16
              local.get 1
              i32.const 4
              i32.add
              i32.const 16
              i32.const 8
              call $dlmalloc::dlmalloc::align_up
              i32.const -5
              i32.add
              local.get 1
              i32.gt_u
              select
              i32.const 8
              call $dlmalloc::dlmalloc::align_up
              local.set 3
              block ;; label = @6
                local.get 0
                i32.load offset=408
                local.tee 5
                local.get 3
                i32.const 3
                i32.shr_u
                local.tee 4
                i32.shr_u
                local.tee 1
                i32.const 3
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                block ;; label = @7
                  block ;; label = @8
                    local.get 0
                    local.get 1
                    i32.const -1
                    i32.xor
                    i32.const 1
                    i32.and
                    local.get 4
                    i32.add
                    local.tee 3
                    i32.const 3
                    i32.shl
                    i32.add
                    local.tee 7
                    i32.const 152
                    i32.add
                    i32.load
                    local.tee 1
                    i32.load offset=8
                    local.tee 4
                    local.get 7
                    i32.const 144
                    i32.add
                    local.tee 7
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 4
                    local.get 7
                    i32.store offset=12
                    local.get 7
                    local.get 4
                    i32.store offset=8
                    br 1 (;@7;)
                  end
                  local.get 0
                  local.get 5
                  i32.const -2
                  local.get 3
                  i32.rotl
                  i32.and
                  i32.store offset=408
                end
                local.get 1
                local.get 3
                i32.const 3
                i32.shl
                call $dlmalloc::dlmalloc::Chunk::set_inuse_and_pinuse
                local.get 1
                call $dlmalloc::dlmalloc::Chunk::to_mem
                local.set 7
                br 5 (;@1;)
              end
              local.get 3
              local.get 0
              i32.load offset=416
              i32.le_u
              br_if 3 (;@2;)
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            local.get 1
                            br_if 0 (;@12;)
                            local.get 0
                            i32.load offset=412
                            local.tee 1
                            i32.eqz
                            br_if 10 (;@2;)
                            local.get 0
                            local.get 1
                            call $dlmalloc::dlmalloc::least_bit
                            i32.ctz
                            i32.const 2
                            i32.shl
                            i32.add
                            i32.load
                            local.tee 7
                            call $dlmalloc::dlmalloc::TreeChunk::chunk
                            call $dlmalloc::dlmalloc::Chunk::size
                            local.get 3
                            i32.sub
                            local.set 4
                            block ;; label = @13
                              local.get 7
                              call $dlmalloc::dlmalloc::TreeChunk::leftmost_child
                              local.tee 1
                              i32.eqz
                              br_if 0 (;@13;)
                              loop ;; label = @14
                                local.get 1
                                call $dlmalloc::dlmalloc::TreeChunk::chunk
                                call $dlmalloc::dlmalloc::Chunk::size
                                local.get 3
                                i32.sub
                                local.tee 5
                                local.get 4
                                local.get 5
                                local.get 4
                                i32.lt_u
                                local.tee 5
                                select
                                local.set 4
                                local.get 1
                                local.get 7
                                local.get 5
                                select
                                local.set 7
                                local.get 1
                                call $dlmalloc::dlmalloc::TreeChunk::leftmost_child
                                local.tee 1
                                br_if 0 (;@14;)
                              end
                            end
                            local.get 7
                            call $dlmalloc::dlmalloc::TreeChunk::chunk
                            local.tee 1
                            local.get 3
                            call $dlmalloc::dlmalloc::Chunk::plus_offset
                            local.set 5
                            local.get 0
                            local.get 7
                            call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_large_chunk
                            local.get 4
                            i32.const 16
                            i32.const 8
                            call $dlmalloc::dlmalloc::align_up
                            i32.lt_u
                            br_if 2 (;@10;)
                            local.get 5
                            call $dlmalloc::dlmalloc::TreeChunk::chunk
                            local.set 5
                            local.get 1
                            local.get 3
                            call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_inuse_chunk
                            local.get 5
                            local.get 4
                            call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_free_chunk
                            local.get 0
                            i32.load offset=416
                            local.tee 8
                            br_if 1 (;@11;)
                            br 5 (;@7;)
                          end
                          block ;; label = @12
                            block ;; label = @13
                              local.get 0
                              i32.const 144
                              i32.add
                              local.tee 8
                              i32.const 1
                              local.get 4
                              i32.const 31
                              i32.and
                              local.tee 4
                              i32.shl
                              call $dlmalloc::dlmalloc::left_bits
                              local.get 1
                              local.get 4
                              i32.shl
                              i32.and
                              call $dlmalloc::dlmalloc::least_bit
                              i32.ctz
                              local.tee 7
                              i32.const 3
                              i32.shl
                              i32.add
                              local.tee 4
                              i32.load offset=8
                              local.tee 1
                              i32.load offset=8
                              local.tee 5
                              local.get 4
                              i32.eq
                              br_if 0 (;@13;)
                              local.get 5
                              local.get 4
                              i32.store offset=12
                              local.get 4
                              local.get 5
                              i32.store offset=8
                              br 1 (;@12;)
                            end
                            local.get 0
                            local.get 0
                            i32.load offset=408
                            i32.const -2
                            local.get 7
                            i32.rotl
                            i32.and
                            i32.store offset=408
                          end
                          local.get 1
                          local.get 3
                          call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_inuse_chunk
                          local.get 1
                          local.get 3
                          call $dlmalloc::dlmalloc::Chunk::plus_offset
                          local.tee 5
                          local.get 7
                          i32.const 3
                          i32.shl
                          local.get 3
                          i32.sub
                          local.tee 6
                          call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_free_chunk
                          local.get 0
                          i32.load offset=416
                          local.tee 7
                          br_if 2 (;@9;)
                          br 3 (;@8;)
                        end
                        local.get 0
                        local.get 8
                        i32.const -8
                        i32.and
                        i32.add
                        i32.const 144
                        i32.add
                        local.set 6
                        local.get 0
                        i32.load offset=424
                        local.set 7
                        block ;; label = @11
                          block ;; label = @12
                            local.get 0
                            i32.load offset=408
                            local.tee 10
                            i32.const 1
                            local.get 8
                            i32.const 3
                            i32.shr_u
                            i32.shl
                            local.tee 8
                            i32.and
                            i32.eqz
                            br_if 0 (;@12;)
                            local.get 6
                            i32.load offset=8
                            local.set 8
                            br 1 (;@11;)
                          end
                          local.get 0
                          local.get 10
                          local.get 8
                          i32.or
                          i32.store offset=408
                          local.get 6
                          local.set 8
                        end
                        local.get 6
                        local.get 7
                        i32.store offset=8
                        local.get 8
                        local.get 7
                        i32.store offset=12
                        local.get 7
                        local.get 6
                        i32.store offset=12
                        local.get 7
                        local.get 8
                        i32.store offset=8
                        br 3 (;@7;)
                      end
                      local.get 1
                      local.get 4
                      local.get 3
                      i32.add
                      call $dlmalloc::dlmalloc::Chunk::set_inuse_and_pinuse
                      br 3 (;@6;)
                    end
                    local.get 8
                    local.get 7
                    i32.const -8
                    i32.and
                    i32.add
                    local.set 4
                    local.get 0
                    i32.load offset=424
                    local.set 3
                    block ;; label = @9
                      block ;; label = @10
                        local.get 0
                        i32.load offset=408
                        local.tee 8
                        i32.const 1
                        local.get 7
                        i32.const 3
                        i32.shr_u
                        i32.shl
                        local.tee 7
                        i32.and
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 4
                        i32.load offset=8
                        local.set 7
                        br 1 (;@9;)
                      end
                      local.get 0
                      local.get 8
                      local.get 7
                      i32.or
                      i32.store offset=408
                      local.get 4
                      local.set 7
                    end
                    local.get 4
                    local.get 3
                    i32.store offset=8
                    local.get 7
                    local.get 3
                    i32.store offset=12
                    local.get 3
                    local.get 4
                    i32.store offset=12
                    local.get 3
                    local.get 7
                    i32.store offset=8
                  end
                  local.get 0
                  local.get 5
                  i32.store offset=424
                  local.get 0
                  local.get 6
                  i32.store offset=416
                  local.get 1
                  call $dlmalloc::dlmalloc::Chunk::to_mem
                  local.set 7
                  br 6 (;@1;)
                end
                local.get 0
                local.get 5
                i32.store offset=424
                local.get 0
                local.get 4
                i32.store offset=416
              end
              local.get 1
              call $dlmalloc::dlmalloc::Chunk::to_mem
              local.tee 7
              i32.eqz
              br_if 3 (;@2;)
              br 4 (;@1;)
            end
            block ;; label = @5
              local.get 1
              local.get 5
              i32.or
              br_if 0 (;@5;)
              i32.const 1
              local.get 10
              i32.shl
              call $dlmalloc::dlmalloc::left_bits
              local.get 9
              i32.and
              local.tee 1
              i32.eqz
              br_if 3 (;@2;)
              local.get 0
              local.get 1
              call $dlmalloc::dlmalloc::least_bit
              i32.ctz
              i32.const 2
              i32.shl
              i32.add
              i32.load
              local.set 1
              i32.const 0
              local.set 5
            end
            local.get 1
            i32.eqz
            br_if 1 (;@3;)
          end
          loop ;; label = @4
            local.get 1
            local.get 5
            local.get 1
            call $dlmalloc::dlmalloc::TreeChunk::chunk
            call $dlmalloc::dlmalloc::Chunk::size
            local.tee 7
            local.get 3
            i32.ge_u
            local.get 7
            local.get 3
            i32.sub
            local.tee 7
            local.get 4
            i32.lt_u
            i32.and
            local.tee 6
            select
            local.set 5
            local.get 7
            local.get 4
            local.get 6
            select
            local.set 4
            local.get 1
            call $dlmalloc::dlmalloc::TreeChunk::leftmost_child
            local.tee 1
            br_if 0 (;@4;)
          end
        end
        local.get 5
        i32.eqz
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 0
          i32.load offset=416
          local.tee 1
          local.get 3
          i32.lt_u
          br_if 0 (;@3;)
          local.get 4
          local.get 1
          local.get 3
          i32.sub
          i32.ge_u
          br_if 1 (;@2;)
        end
        local.get 5
        call $dlmalloc::dlmalloc::TreeChunk::chunk
        local.tee 1
        local.get 3
        call $dlmalloc::dlmalloc::Chunk::plus_offset
        local.set 7
        local.get 0
        local.get 5
        call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_large_chunk
        block ;; label = @3
          block ;; label = @4
            local.get 4
            i32.const 16
            i32.const 8
            call $dlmalloc::dlmalloc::align_up
            i32.lt_u
            br_if 0 (;@4;)
            local.get 1
            local.get 3
            call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_inuse_chunk
            local.get 7
            local.get 4
            call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_free_chunk
            block ;; label = @5
              local.get 4
              i32.const 256
              i32.lt_u
              br_if 0 (;@5;)
              local.get 0
              local.get 7
              local.get 4
              call $dlmalloc::dlmalloc::Dlmalloc<A>::insert_large_chunk
              br 2 (;@3;)
            end
            local.get 0
            local.get 4
            i32.const -8
            i32.and
            i32.add
            i32.const 144
            i32.add
            local.set 5
            block ;; label = @5
              block ;; label = @6
                local.get 0
                i32.load offset=408
                local.tee 6
                i32.const 1
                local.get 4
                i32.const 3
                i32.shr_u
                i32.shl
                local.tee 4
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                local.get 5
                i32.load offset=8
                local.set 4
                br 1 (;@5;)
              end
              local.get 0
              local.get 6
              local.get 4
              i32.or
              i32.store offset=408
              local.get 5
              local.set 4
            end
            local.get 5
            local.get 7
            i32.store offset=8
            local.get 4
            local.get 7
            i32.store offset=12
            local.get 7
            local.get 5
            i32.store offset=12
            local.get 7
            local.get 4
            i32.store offset=8
            br 1 (;@3;)
          end
          local.get 1
          local.get 4
          local.get 3
          i32.add
          call $dlmalloc::dlmalloc::Chunk::set_inuse_and_pinuse
        end
        local.get 1
        call $dlmalloc::dlmalloc::Chunk::to_mem
        local.tee 7
        br_if 1 (;@1;)
      end
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      local.get 0
                      i32.load offset=416
                      local.tee 4
                      local.get 3
                      i32.ge_u
                      br_if 0 (;@9;)
                      block ;; label = @10
                        local.get 0
                        i32.load offset=420
                        local.tee 1
                        local.get 3
                        i32.gt_u
                        br_if 0 (;@10;)
                        local.get 2
                        i32.const 4
                        i32.add
                        local.get 0
                        local.get 3
                        call $dlmalloc::dlmalloc::Chunk::mem_offset
                        local.tee 1
                        i32.sub
                        local.get 1
                        i32.const 8
                        call $dlmalloc::dlmalloc::align_up
                        i32.add
                        i32.const 20
                        i32.const 8
                        call $dlmalloc::dlmalloc::align_up
                        i32.add
                        i32.const 16
                        i32.const 8
                        call $dlmalloc::dlmalloc::align_up
                        i32.add
                        i32.const 8
                        i32.add
                        i32.const 65536
                        call $dlmalloc::dlmalloc::align_up
                        call $<dlmalloc::sys::System as dlmalloc::Allocator>::alloc
                        i32.const 0
                        local.set 7
                        local.get 2
                        i32.load offset=4
                        local.tee 8
                        i32.eqz
                        br_if 9 (;@1;)
                        local.get 2
                        i32.load offset=12
                        local.set 11
                        local.get 0
                        local.get 0
                        i32.load offset=432
                        local.get 2
                        i32.load offset=8
                        local.tee 10
                        i32.add
                        local.tee 1
                        i32.store offset=432
                        local.get 0
                        local.get 0
                        i32.load offset=436
                        local.tee 4
                        local.get 1
                        local.get 4
                        local.get 1
                        i32.gt_u
                        select
                        i32.store offset=436
                        block ;; label = @11
                          local.get 0
                          i32.load offset=428
                          br_if 0 (;@11;)
                          local.get 0
                          i32.load offset=444
                          local.tee 1
                          i32.eqz
                          br_if 3 (;@8;)
                          local.get 8
                          local.get 1
                          i32.lt_u
                          br_if 3 (;@8;)
                          br 8 (;@3;)
                        end
                        local.get 0
                        i32.const 128
                        i32.add
                        local.tee 9
                        local.set 1
                        block ;; label = @11
                          block ;; label = @12
                            loop ;; label = @13
                              local.get 8
                              local.get 1
                              call $dlmalloc::dlmalloc::Segment::top
                              i32.eq
                              br_if 1 (;@12;)
                              local.get 1
                              i32.load offset=8
                              local.tee 1
                              br_if 0 (;@13;)
                              br 2 (;@11;)
                            end
                          end
                          local.get 1
                          call $dlmalloc::dlmalloc::Segment::is_extern
                          br_if 0 (;@11;)
                          local.get 1
                          call $dlmalloc::dlmalloc::Segment::sys_flags
                          local.get 11
                          i32.ne
                          br_if 0 (;@11;)
                          local.get 1
                          local.get 0
                          i32.load offset=428
                          call $dlmalloc::dlmalloc::Segment::holds
                          br_if 4 (;@7;)
                        end
                        local.get 0
                        local.get 0
                        i32.load offset=444
                        local.tee 1
                        local.get 8
                        local.get 8
                        local.get 1
                        i32.gt_u
                        select
                        i32.store offset=444
                        local.get 8
                        local.get 10
                        i32.add
                        local.set 4
                        local.get 9
                        local.set 1
                        block ;; label = @11
                          block ;; label = @12
                            block ;; label = @13
                              loop ;; label = @14
                                local.get 1
                                i32.load
                                local.get 4
                                i32.eq
                                br_if 1 (;@13;)
                                local.get 1
                                i32.load offset=8
                                local.tee 1
                                br_if 0 (;@14;)
                                br 2 (;@12;)
                              end
                            end
                            local.get 1
                            call $dlmalloc::dlmalloc::Segment::is_extern
                            br_if 0 (;@12;)
                            local.get 1
                            call $dlmalloc::dlmalloc::Segment::sys_flags
                            local.get 11
                            i32.eq
                            br_if 1 (;@11;)
                          end
                          local.get 0
                          i32.load offset=428
                          local.set 5
                          local.get 9
                          local.set 1
                          block ;; label = @12
                            loop ;; label = @13
                              block ;; label = @14
                                local.get 1
                                i32.load
                                local.get 5
                                i32.gt_u
                                br_if 0 (;@14;)
                                local.get 1
                                call $dlmalloc::dlmalloc::Segment::top
                                local.get 5
                                i32.gt_u
                                br_if 2 (;@12;)
                              end
                              local.get 1
                              i32.load offset=8
                              local.tee 1
                              br_if 0 (;@13;)
                            end
                            i32.const 0
                            local.set 1
                          end
                          local.get 1
                          call $dlmalloc::dlmalloc::Segment::top
                          local.tee 6
                          i32.const 20
                          i32.const 8
                          call $dlmalloc::dlmalloc::align_up
                          local.tee 12
                          i32.sub
                          i32.const -23
                          i32.add
                          local.set 1
                          local.get 5
                          local.get 1
                          local.get 1
                          call $dlmalloc::dlmalloc::Chunk::to_mem
                          local.tee 4
                          i32.const 8
                          call $dlmalloc::dlmalloc::align_up
                          local.get 4
                          i32.sub
                          i32.add
                          local.tee 1
                          local.get 1
                          local.get 5
                          i32.const 16
                          i32.const 8
                          call $dlmalloc::dlmalloc::align_up
                          i32.add
                          i32.lt_u
                          select
                          local.tee 13
                          call $dlmalloc::dlmalloc::Chunk::to_mem
                          local.set 4
                          local.get 13
                          local.get 12
                          call $dlmalloc::dlmalloc::Chunk::plus_offset
                          local.set 1
                          call $dlmalloc::dlmalloc::Chunk::mem_offset
                          local.tee 14
                          i32.const 8
                          call $dlmalloc::dlmalloc::align_up
                          local.set 15
                          i32.const 20
                          i32.const 8
                          call $dlmalloc::dlmalloc::align_up
                          local.set 16
                          i32.const 16
                          i32.const 8
                          call $dlmalloc::dlmalloc::align_up
                          local.set 17
                          local.get 0
                          local.get 8
                          local.get 8
                          call $dlmalloc::dlmalloc::Chunk::to_mem
                          local.tee 18
                          i32.const 8
                          call $dlmalloc::dlmalloc::align_up
                          local.get 18
                          i32.sub
                          local.tee 19
                          call $dlmalloc::dlmalloc::Chunk::plus_offset
                          local.tee 18
                          i32.store offset=428
                          local.get 0
                          local.get 14
                          local.get 10
                          i32.add
                          local.get 17
                          local.get 15
                          local.get 16
                          i32.add
                          i32.add
                          local.get 19
                          i32.add
                          i32.sub
                          local.tee 14
                          i32.store offset=420
                          local.get 18
                          local.get 14
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          call $dlmalloc::dlmalloc::Chunk::mem_offset
                          local.tee 15
                          i32.const 8
                          call $dlmalloc::dlmalloc::align_up
                          local.set 16
                          i32.const 20
                          i32.const 8
                          call $dlmalloc::dlmalloc::align_up
                          local.set 17
                          i32.const 16
                          i32.const 8
                          call $dlmalloc::dlmalloc::align_up
                          local.set 19
                          local.get 18
                          local.get 14
                          call $dlmalloc::dlmalloc::Chunk::plus_offset
                          local.set 18
                          local.get 0
                          i32.const 2097152
                          i32.store offset=440
                          local.get 18
                          local.get 19
                          local.get 17
                          local.get 16
                          local.get 15
                          i32.sub
                          i32.add
                          i32.add
                          i32.store offset=4
                          local.get 13
                          local.get 12
                          call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_inuse_chunk
                          local.get 9
                          i64.load align=4
                          local.set 20
                          local.get 4
                          i32.const 8
                          i32.add
                          local.get 9
                          i32.const 8
                          i32.add
                          i64.load align=4
                          i64.store align=4
                          local.get 4
                          local.get 20
                          i64.store align=4
                          local.get 0
                          i32.const 140
                          i32.add
                          local.get 11
                          i32.store
                          local.get 0
                          i32.const 132
                          i32.add
                          local.get 10
                          i32.store
                          local.get 0
                          local.get 8
                          i32.store offset=128
                          local.get 0
                          i32.const 136
                          i32.add
                          local.get 4
                          i32.store
                          loop ;; label = @12
                            local.get 1
                            i32.const 4
                            call $dlmalloc::dlmalloc::Chunk::plus_offset
                            local.set 4
                            local.get 1
                            call $dlmalloc::dlmalloc::Chunk::fencepost_head
                            i32.store offset=4
                            local.get 4
                            local.set 1
                            local.get 4
                            i32.const 4
                            i32.add
                            local.get 6
                            i32.lt_u
                            br_if 0 (;@12;)
                          end
                          local.get 13
                          local.get 5
                          i32.eq
                          br_if 9 (;@2;)
                          local.get 13
                          local.get 5
                          i32.sub
                          local.set 1
                          local.get 5
                          local.get 1
                          local.get 5
                          local.get 1
                          call $dlmalloc::dlmalloc::Chunk::plus_offset
                          call $dlmalloc::dlmalloc::Chunk::set_free_with_pinuse
                          block ;; label = @12
                            local.get 1
                            i32.const 256
                            i32.lt_u
                            br_if 0 (;@12;)
                            local.get 0
                            local.get 5
                            local.get 1
                            call $dlmalloc::dlmalloc::Dlmalloc<A>::insert_large_chunk
                            br 10 (;@2;)
                          end
                          local.get 0
                          local.get 1
                          i32.const -8
                          i32.and
                          i32.add
                          i32.const 144
                          i32.add
                          local.set 4
                          block ;; label = @12
                            block ;; label = @13
                              local.get 0
                              i32.load offset=408
                              local.tee 6
                              i32.const 1
                              local.get 1
                              i32.const 3
                              i32.shr_u
                              i32.shl
                              local.tee 1
                              i32.and
                              i32.eqz
                              br_if 0 (;@13;)
                              local.get 4
                              i32.load offset=8
                              local.set 1
                              br 1 (;@12;)
                            end
                            local.get 0
                            local.get 6
                            local.get 1
                            i32.or
                            i32.store offset=408
                            local.get 4
                            local.set 1
                          end
                          local.get 4
                          local.get 5
                          i32.store offset=8
                          local.get 1
                          local.get 5
                          i32.store offset=12
                          local.get 5
                          local.get 4
                          i32.store offset=12
                          local.get 5
                          local.get 1
                          i32.store offset=8
                          br 9 (;@2;)
                        end
                        local.get 1
                        i32.load
                        local.set 5
                        local.get 1
                        local.get 8
                        i32.store
                        local.get 1
                        local.get 1
                        i32.load offset=4
                        local.get 10
                        i32.add
                        i32.store offset=4
                        local.get 8
                        call $dlmalloc::dlmalloc::Chunk::to_mem
                        local.tee 1
                        i32.const 8
                        call $dlmalloc::dlmalloc::align_up
                        local.set 4
                        local.get 5
                        call $dlmalloc::dlmalloc::Chunk::to_mem
                        local.tee 6
                        i32.const 8
                        call $dlmalloc::dlmalloc::align_up
                        local.set 10
                        local.get 8
                        local.get 4
                        local.get 1
                        i32.sub
                        i32.add
                        local.tee 4
                        local.get 3
                        call $dlmalloc::dlmalloc::Chunk::plus_offset
                        local.set 7
                        local.get 4
                        local.get 3
                        call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_inuse_chunk
                        local.get 5
                        local.get 10
                        local.get 6
                        i32.sub
                        i32.add
                        local.tee 1
                        local.get 3
                        local.get 4
                        i32.add
                        i32.sub
                        local.set 3
                        block ;; label = @11
                          local.get 1
                          local.get 0
                          i32.load offset=428
                          i32.eq
                          br_if 0 (;@11;)
                          local.get 1
                          local.get 0
                          i32.load offset=424
                          i32.eq
                          br_if 5 (;@6;)
                          local.get 1
                          call $dlmalloc::dlmalloc::Chunk::inuse
                          br_if 7 (;@4;)
                          block ;; label = @12
                            block ;; label = @13
                              local.get 1
                              call $dlmalloc::dlmalloc::Chunk::size
                              local.tee 5
                              i32.const 256
                              i32.lt_u
                              br_if 0 (;@13;)
                              local.get 0
                              local.get 1
                              call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_large_chunk
                              br 1 (;@12;)
                            end
                            block ;; label = @13
                              local.get 1
                              i32.load offset=12
                              local.tee 6
                              local.get 1
                              i32.load offset=8
                              local.tee 8
                              i32.eq
                              br_if 0 (;@13;)
                              local.get 8
                              local.get 6
                              i32.store offset=12
                              local.get 6
                              local.get 8
                              i32.store offset=8
                              br 1 (;@12;)
                            end
                            local.get 0
                            local.get 0
                            i32.load offset=408
                            i32.const -2
                            local.get 5
                            i32.const 3
                            i32.shr_u
                            i32.rotl
                            i32.and
                            i32.store offset=408
                          end
                          local.get 5
                          local.get 3
                          i32.add
                          local.set 3
                          local.get 1
                          local.get 5
                          call $dlmalloc::dlmalloc::Chunk::plus_offset
                          local.set 1
                          br 7 (;@4;)
                        end
                        local.get 0
                        local.get 7
                        i32.store offset=428
                        local.get 0
                        local.get 0
                        i32.load offset=420
                        local.get 3
                        i32.add
                        local.tee 1
                        i32.store offset=420
                        local.get 7
                        local.get 1
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        local.get 4
                        call $dlmalloc::dlmalloc::Chunk::to_mem
                        local.set 7
                        br 9 (;@1;)
                      end
                      local.get 0
                      local.get 1
                      local.get 3
                      i32.sub
                      local.tee 4
                      i32.store offset=420
                      local.get 0
                      local.get 0
                      i32.load offset=428
                      local.tee 1
                      local.get 3
                      call $dlmalloc::dlmalloc::Chunk::plus_offset
                      local.tee 7
                      i32.store offset=428
                      local.get 7
                      local.get 4
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 1
                      local.get 3
                      call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_inuse_chunk
                      local.get 1
                      call $dlmalloc::dlmalloc::Chunk::to_mem
                      local.set 7
                      br 8 (;@1;)
                    end
                    local.get 0
                    i32.load offset=424
                    local.set 1
                    local.get 4
                    local.get 3
                    i32.sub
                    local.tee 4
                    i32.const 16
                    i32.const 8
                    call $dlmalloc::dlmalloc::align_up
                    i32.lt_u
                    br_if 3 (;@5;)
                    local.get 1
                    local.get 3
                    call $dlmalloc::dlmalloc::Chunk::plus_offset
                    local.set 7
                    local.get 0
                    local.get 4
                    i32.store offset=416
                    local.get 0
                    local.get 7
                    i32.store offset=424
                    local.get 7
                    local.get 4
                    call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_free_chunk
                    local.get 1
                    local.get 3
                    call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_inuse_chunk
                    local.get 1
                    call $dlmalloc::dlmalloc::Chunk::to_mem
                    local.set 7
                    br 7 (;@1;)
                  end
                  local.get 0
                  local.get 8
                  i32.store offset=444
                  br 4 (;@3;)
                end
                local.get 1
                local.get 1
                i32.load offset=4
                local.get 10
                i32.add
                i32.store offset=4
                local.get 0
                local.get 0
                i32.load offset=428
                local.get 0
                i32.load offset=420
                local.get 10
                i32.add
                call $dlmalloc::dlmalloc::Dlmalloc<A>::init_top
                br 4 (;@2;)
              end
              local.get 0
              local.get 7
              i32.store offset=424
              local.get 0
              local.get 0
              i32.load offset=416
              local.get 3
              i32.add
              local.tee 1
              i32.store offset=416
              local.get 7
              local.get 1
              call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_free_chunk
              local.get 4
              call $dlmalloc::dlmalloc::Chunk::to_mem
              local.set 7
              br 4 (;@1;)
            end
            local.get 0
            i32.const 0
            i32.store offset=424
            local.get 0
            i32.load offset=416
            local.set 3
            local.get 0
            i32.const 0
            i32.store offset=416
            local.get 1
            local.get 3
            call $dlmalloc::dlmalloc::Chunk::set_inuse_and_pinuse
            local.get 1
            call $dlmalloc::dlmalloc::Chunk::to_mem
            local.set 7
            br 3 (;@1;)
          end
          local.get 7
          local.get 3
          local.get 1
          call $dlmalloc::dlmalloc::Chunk::set_free_with_pinuse
          block ;; label = @4
            local.get 3
            i32.const 256
            i32.lt_u
            br_if 0 (;@4;)
            local.get 0
            local.get 7
            local.get 3
            call $dlmalloc::dlmalloc::Dlmalloc<A>::insert_large_chunk
            local.get 4
            call $dlmalloc::dlmalloc::Chunk::to_mem
            local.set 7
            br 3 (;@1;)
          end
          local.get 0
          local.get 3
          i32.const -8
          i32.and
          i32.add
          i32.const 144
          i32.add
          local.set 1
          block ;; label = @4
            block ;; label = @5
              local.get 0
              i32.load offset=408
              local.tee 5
              i32.const 1
              local.get 3
              i32.const 3
              i32.shr_u
              i32.shl
              local.tee 3
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              local.get 1
              i32.load offset=8
              local.set 3
              br 1 (;@4;)
            end
            local.get 0
            local.get 5
            local.get 3
            i32.or
            i32.store offset=408
            local.get 1
            local.set 3
          end
          local.get 1
          local.get 7
          i32.store offset=8
          local.get 3
          local.get 7
          i32.store offset=12
          local.get 7
          local.get 1
          i32.store offset=12
          local.get 7
          local.get 3
          i32.store offset=8
          local.get 4
          call $dlmalloc::dlmalloc::Chunk::to_mem
          local.set 7
          br 2 (;@1;)
        end
        local.get 0
        i32.const 4095
        i32.store offset=448
        local.get 0
        local.get 8
        i32.store offset=128
        local.get 0
        i32.const 140
        i32.add
        local.get 11
        i32.store
        local.get 0
        i32.const 132
        i32.add
        local.get 10
        i32.store
        i32.const 0
        local.set 4
        loop ;; label = @3
          local.get 0
          local.get 4
          i32.add
          local.tee 1
          i32.const 164
          i32.add
          local.get 1
          i32.const 152
          i32.add
          local.tee 5
          i32.store
          local.get 5
          local.get 1
          i32.const 144
          i32.add
          local.tee 6
          i32.store
          local.get 1
          i32.const 156
          i32.add
          local.get 6
          i32.store
          local.get 1
          i32.const 172
          i32.add
          local.get 1
          i32.const 160
          i32.add
          local.tee 6
          i32.store
          local.get 6
          local.get 5
          i32.store
          local.get 1
          i32.const 180
          i32.add
          local.get 1
          i32.const 168
          i32.add
          local.tee 5
          i32.store
          local.get 5
          local.get 6
          i32.store
          local.get 1
          i32.const 176
          i32.add
          local.get 5
          i32.store
          local.get 4
          i32.const 32
          i32.add
          local.tee 4
          i32.const 256
          i32.ne
          br_if 0 (;@3;)
        end
        call $dlmalloc::dlmalloc::Chunk::mem_offset
        local.tee 4
        i32.const 8
        call $dlmalloc::dlmalloc::align_up
        local.set 5
        i32.const 20
        i32.const 8
        call $dlmalloc::dlmalloc::align_up
        local.set 6
        i32.const 16
        i32.const 8
        call $dlmalloc::dlmalloc::align_up
        local.set 9
        local.get 0
        local.get 8
        local.get 8
        call $dlmalloc::dlmalloc::Chunk::to_mem
        local.tee 1
        i32.const 8
        call $dlmalloc::dlmalloc::align_up
        local.get 1
        i32.sub
        local.tee 13
        call $dlmalloc::dlmalloc::Chunk::plus_offset
        local.tee 1
        i32.store offset=428
        local.get 0
        local.get 4
        local.get 10
        i32.add
        local.get 9
        local.get 5
        local.get 6
        i32.add
        i32.add
        local.get 13
        i32.add
        i32.sub
        local.tee 4
        i32.store offset=420
        local.get 1
        local.get 4
        i32.const 1
        i32.or
        i32.store offset=4
        call $dlmalloc::dlmalloc::Chunk::mem_offset
        local.tee 5
        i32.const 8
        call $dlmalloc::dlmalloc::align_up
        local.set 6
        i32.const 20
        i32.const 8
        call $dlmalloc::dlmalloc::align_up
        local.set 8
        i32.const 16
        i32.const 8
        call $dlmalloc::dlmalloc::align_up
        local.set 10
        local.get 1
        local.get 4
        call $dlmalloc::dlmalloc::Chunk::plus_offset
        local.set 1
        local.get 0
        i32.const 2097152
        i32.store offset=440
        local.get 1
        local.get 10
        local.get 8
        local.get 6
        local.get 5
        i32.sub
        i32.add
        i32.add
        i32.store offset=4
      end
      local.get 0
      i32.load offset=420
      local.tee 1
      local.get 3
      i32.le_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      local.get 3
      i32.sub
      local.tee 4
      i32.store offset=420
      local.get 0
      local.get 0
      i32.load offset=428
      local.tee 1
      local.get 3
      call $dlmalloc::dlmalloc::Chunk::plus_offset
      local.tee 7
      i32.store offset=428
      local.get 7
      local.get 4
      i32.const 1
      i32.or
      i32.store offset=4
      local.get 1
      local.get 3
      call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_inuse_chunk
      local.get 1
      call $dlmalloc::dlmalloc::Chunk::to_mem
      local.set 7
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 7
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::init_top (;6;) (type 0) (param i32 i32 i32)
    (local i32 i32 i32 i32)
    local.get 1
    local.get 1
    call $dlmalloc::dlmalloc::Chunk::to_mem
    local.tee 3
    i32.const 8
    call $dlmalloc::dlmalloc::align_up
    local.get 3
    i32.sub
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
    call $dlmalloc::dlmalloc::Chunk::mem_offset
    local.tee 3
    i32.const 8
    call $dlmalloc::dlmalloc::align_up
    local.set 4
    i32.const 20
    i32.const 8
    call $dlmalloc::dlmalloc::align_up
    local.set 5
    i32.const 16
    i32.const 8
    call $dlmalloc::dlmalloc::align_up
    local.set 6
    local.get 1
    local.get 2
    call $dlmalloc::dlmalloc::Chunk::plus_offset
    local.set 1
    local.get 0
    i32.const 2097152
    i32.store offset=440
    local.get 1
    local.get 6
    local.get 5
    local.get 4
    local.get 3
    i32.sub
    i32.add
    i32.add
    i32.store offset=4
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::memalign (;7;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      i32.const 16
      i32.const 8
      call $dlmalloc::dlmalloc::align_up
      local.get 1
      i32.le_u
      br_if 0 (;@1;)
      i32.const 16
      i32.const 8
      call $dlmalloc::dlmalloc::align_up
      local.set 1
    end
    call $dlmalloc::dlmalloc::Chunk::mem_offset
    local.tee 3
    i32.const 8
    call $dlmalloc::dlmalloc::align_up
    local.set 4
    i32.const 20
    i32.const 8
    call $dlmalloc::dlmalloc::align_up
    local.set 5
    i32.const 16
    i32.const 8
    call $dlmalloc::dlmalloc::align_up
    local.set 6
    i32.const 0
    local.set 7
    block ;; label = @1
      i32.const 0
      i32.const 16
      i32.const 8
      call $dlmalloc::dlmalloc::align_up
      i32.const 2
      i32.shl
      i32.sub
      local.tee 8
      local.get 3
      local.get 6
      local.get 4
      local.get 5
      i32.add
      i32.add
      i32.sub
      i32.const -65544
      i32.add
      i32.const -9
      i32.and
      i32.const -3
      i32.add
      local.tee 3
      local.get 8
      local.get 3
      i32.lt_u
      select
      local.get 1
      i32.sub
      local.get 2
      i32.le_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.const 16
      local.get 2
      i32.const 4
      i32.add
      i32.const 16
      i32.const 8
      call $dlmalloc::dlmalloc::align_up
      i32.const -5
      i32.add
      local.get 2
      i32.gt_u
      select
      i32.const 8
      call $dlmalloc::dlmalloc::align_up
      local.tee 4
      i32.add
      i32.const 16
      i32.const 8
      call $dlmalloc::dlmalloc::align_up
      i32.add
      i32.const -4
      i32.add
      call $dlmalloc::dlmalloc::Dlmalloc<A>::malloc
      local.tee 3
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      call $dlmalloc::dlmalloc::Chunk::from_mem
      local.set 2
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i32.const -1
          i32.add
          local.tee 7
          local.get 3
          i32.and
          br_if 0 (;@3;)
          local.get 2
          local.set 1
          br 1 (;@2;)
        end
        local.get 7
        local.get 3
        i32.add
        i32.const 0
        local.get 1
        i32.sub
        i32.and
        call $dlmalloc::dlmalloc::Chunk::from_mem
        local.set 7
        i32.const 16
        i32.const 8
        call $dlmalloc::dlmalloc::align_up
        local.set 3
        local.get 2
        call $dlmalloc::dlmalloc::Chunk::size
        local.get 7
        i32.const 0
        local.get 1
        local.get 7
        local.get 2
        i32.sub
        local.get 3
        i32.gt_u
        select
        i32.add
        local.tee 1
        local.get 2
        i32.sub
        local.tee 7
        i32.sub
        local.set 3
        block ;; label = @3
          local.get 2
          call $dlmalloc::dlmalloc::Chunk::mmapped
          br_if 0 (;@3;)
          local.get 1
          local.get 3
          call $dlmalloc::dlmalloc::Chunk::set_inuse
          local.get 2
          local.get 7
          call $dlmalloc::dlmalloc::Chunk::set_inuse
          local.get 0
          local.get 2
          local.get 7
          call $dlmalloc::dlmalloc::Dlmalloc<A>::dispose_chunk
          br 1 (;@2;)
        end
        local.get 2
        i32.load
        local.set 2
        local.get 1
        local.get 3
        i32.store offset=4
        local.get 1
        local.get 2
        local.get 7
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
        i32.const 16
        i32.const 8
        call $dlmalloc::dlmalloc::align_up
        local.get 4
        i32.add
        i32.le_u
        br_if 0 (;@2;)
        local.get 1
        local.get 4
        call $dlmalloc::dlmalloc::Chunk::plus_offset
        local.set 7
        local.get 1
        local.get 4
        call $dlmalloc::dlmalloc::Chunk::set_inuse
        local.get 7
        local.get 2
        local.get 4
        i32.sub
        local.tee 2
        call $dlmalloc::dlmalloc::Chunk::set_inuse
        local.get 0
        local.get 7
        local.get 2
        call $dlmalloc::dlmalloc::Dlmalloc<A>::dispose_chunk
      end
      local.get 1
      call $dlmalloc::dlmalloc::Chunk::to_mem
      local.set 7
      local.get 1
      call $dlmalloc::dlmalloc::Chunk::mmapped
      drop
    end
    local.get 7
  )
  (func $__main (;8;) (type 5) (result i32)
    call $vec_alloc
  )
  (func $__rust_alloc (;9;) (type 3) (param i32 i32) (result i32)
    (local i32 i32)
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
    local.set 3
    block ;; label = @1
      block ;; label = @2
        local.get 1
        i32.const 9
        i32.lt_u
        br_if 0 (;@2;)
        local.get 3
        local.get 1
        local.get 0
        call $dlmalloc::dlmalloc::Dlmalloc<A>::memalign
        local.set 1
        br 1 (;@1;)
      end
      local.get 3
      local.get 0
      call $dlmalloc::dlmalloc::Dlmalloc<A>::malloc
      local.set 1
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
  (func $__rust_dealloc (;10;) (type 0) (param i32 i32 i32)
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
  (func $__rust_realloc (;11;) (type 6) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
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
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 2
                i32.const 9
                i32.lt_u
                br_if 0 (;@6;)
                local.get 5
                local.get 2
                local.get 3
                call $dlmalloc::dlmalloc::Dlmalloc<A>::memalign
                local.tee 2
                br_if 1 (;@5;)
                i32.const 0
                local.set 2
                br 5 (;@1;)
              end
              call $dlmalloc::dlmalloc::Chunk::mem_offset
              local.tee 1
              i32.const 8
              call $dlmalloc::dlmalloc::align_up
              local.set 6
              i32.const 20
              i32.const 8
              call $dlmalloc::dlmalloc::align_up
              local.set 7
              i32.const 16
              i32.const 8
              call $dlmalloc::dlmalloc::align_up
              local.set 8
              i32.const 0
              local.set 2
              i32.const 0
              i32.const 16
              i32.const 8
              call $dlmalloc::dlmalloc::align_up
              i32.const 2
              i32.shl
              i32.sub
              local.tee 9
              local.get 1
              local.get 8
              local.get 6
              local.get 7
              i32.add
              i32.add
              i32.sub
              i32.const -65544
              i32.add
              i32.const -9
              i32.and
              i32.const -3
              i32.add
              local.tee 1
              local.get 9
              local.get 1
              i32.lt_u
              select
              local.get 3
              i32.le_u
              br_if 4 (;@1;)
              i32.const 16
              local.get 3
              i32.const 4
              i32.add
              i32.const 16
              i32.const 8
              call $dlmalloc::dlmalloc::align_up
              i32.const -5
              i32.add
              local.get 3
              i32.gt_u
              select
              i32.const 8
              call $dlmalloc::dlmalloc::align_up
              local.set 6
              local.get 0
              call $dlmalloc::dlmalloc::Chunk::from_mem
              local.set 1
              local.get 1
              local.get 1
              call $dlmalloc::dlmalloc::Chunk::size
              local.tee 7
              call $dlmalloc::dlmalloc::Chunk::plus_offset
              local.set 8
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            local.get 1
                            call $dlmalloc::dlmalloc::Chunk::mmapped
                            br_if 0 (;@12;)
                            local.get 7
                            local.get 6
                            i32.ge_u
                            br_if 4 (;@8;)
                            local.get 8
                            local.get 5
                            i32.load offset=428
                            i32.eq
                            br_if 6 (;@6;)
                            local.get 8
                            local.get 5
                            i32.load offset=424
                            i32.eq
                            br_if 3 (;@9;)
                            local.get 8
                            call $dlmalloc::dlmalloc::Chunk::cinuse
                            br_if 9 (;@3;)
                            local.get 8
                            call $dlmalloc::dlmalloc::Chunk::size
                            local.tee 9
                            local.get 7
                            i32.add
                            local.tee 7
                            local.get 6
                            i32.lt_u
                            br_if 9 (;@3;)
                            local.get 7
                            local.get 6
                            i32.sub
                            local.set 10
                            local.get 9
                            i32.const 256
                            i32.lt_u
                            br_if 1 (;@11;)
                            local.get 5
                            local.get 8
                            call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_large_chunk
                            br 2 (;@10;)
                          end
                          local.get 1
                          call $dlmalloc::dlmalloc::Chunk::size
                          local.set 7
                          local.get 6
                          i32.const 256
                          i32.lt_u
                          br_if 8 (;@3;)
                          block ;; label = @12
                            local.get 7
                            local.get 6
                            i32.const 4
                            i32.add
                            i32.lt_u
                            br_if 0 (;@12;)
                            local.get 7
                            local.get 6
                            i32.sub
                            i32.const 131073
                            i32.lt_u
                            br_if 5 (;@7;)
                          end
                          local.get 5
                          local.get 1
                          local.get 1
                          i32.load
                          local.tee 8
                          i32.sub
                          local.get 7
                          local.get 8
                          i32.add
                          i32.const 16
                          i32.add
                          local.tee 9
                          local.get 6
                          i32.const 31
                          i32.add
                          local.get 5
                          call $<dlmalloc::sys::System as dlmalloc::Allocator>::page_size
                          call $dlmalloc::dlmalloc::align_up
                          local.tee 7
                          i32.const 1
                          call $<dlmalloc::sys::System as dlmalloc::Allocator>::remap
                          local.tee 6
                          i32.eqz
                          br_if 8 (;@3;)
                          local.get 6
                          local.get 8
                          i32.add
                          local.tee 1
                          local.get 7
                          local.get 8
                          i32.sub
                          local.tee 3
                          i32.const -16
                          i32.add
                          local.tee 2
                          i32.store offset=4
                          call $dlmalloc::dlmalloc::Chunk::fencepost_head
                          local.set 0
                          local.get 1
                          local.get 2
                          call $dlmalloc::dlmalloc::Chunk::plus_offset
                          local.get 0
                          i32.store offset=4
                          local.get 1
                          local.get 3
                          i32.const -12
                          i32.add
                          call $dlmalloc::dlmalloc::Chunk::plus_offset
                          i32.const 0
                          i32.store offset=4
                          local.get 5
                          local.get 5
                          i32.load offset=432
                          local.get 7
                          local.get 9
                          i32.sub
                          i32.add
                          local.tee 3
                          i32.store offset=432
                          local.get 5
                          local.get 5
                          i32.load offset=444
                          local.tee 2
                          local.get 6
                          local.get 6
                          local.get 2
                          i32.gt_u
                          select
                          i32.store offset=444
                          local.get 5
                          local.get 5
                          i32.load offset=436
                          local.tee 2
                          local.get 3
                          local.get 2
                          local.get 3
                          i32.gt_u
                          select
                          i32.store offset=436
                          br 9 (;@2;)
                        end
                        block ;; label = @11
                          local.get 8
                          i32.load offset=12
                          local.tee 11
                          local.get 8
                          i32.load offset=8
                          local.tee 8
                          i32.eq
                          br_if 0 (;@11;)
                          local.get 8
                          local.get 11
                          i32.store offset=12
                          local.get 11
                          local.get 8
                          i32.store offset=8
                          br 1 (;@10;)
                        end
                        local.get 5
                        local.get 5
                        i32.load offset=408
                        i32.const -2
                        local.get 9
                        i32.const 3
                        i32.shr_u
                        i32.rotl
                        i32.and
                        i32.store offset=408
                      end
                      block ;; label = @10
                        local.get 10
                        i32.const 16
                        i32.const 8
                        call $dlmalloc::dlmalloc::align_up
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 1
                        local.get 6
                        call $dlmalloc::dlmalloc::Chunk::plus_offset
                        local.set 7
                        local.get 1
                        local.get 6
                        call $dlmalloc::dlmalloc::Chunk::set_inuse
                        local.get 7
                        local.get 10
                        call $dlmalloc::dlmalloc::Chunk::set_inuse
                        local.get 5
                        local.get 7
                        local.get 10
                        call $dlmalloc::dlmalloc::Dlmalloc<A>::dispose_chunk
                        local.get 1
                        br_if 8 (;@2;)
                        br 7 (;@3;)
                      end
                      local.get 1
                      local.get 7
                      call $dlmalloc::dlmalloc::Chunk::set_inuse
                      local.get 1
                      br_if 7 (;@2;)
                      br 6 (;@3;)
                    end
                    local.get 5
                    i32.load offset=416
                    local.get 7
                    i32.add
                    local.tee 7
                    local.get 6
                    i32.lt_u
                    br_if 5 (;@3;)
                    block ;; label = @9
                      block ;; label = @10
                        local.get 7
                        local.get 6
                        i32.sub
                        local.tee 8
                        i32.const 16
                        i32.const 8
                        call $dlmalloc::dlmalloc::align_up
                        i32.ge_u
                        br_if 0 (;@10;)
                        local.get 1
                        local.get 7
                        call $dlmalloc::dlmalloc::Chunk::set_inuse
                        i32.const 0
                        local.set 8
                        i32.const 0
                        local.set 7
                        br 1 (;@9;)
                      end
                      local.get 1
                      local.get 6
                      call $dlmalloc::dlmalloc::Chunk::plus_offset
                      local.tee 7
                      local.get 8
                      call $dlmalloc::dlmalloc::Chunk::plus_offset
                      local.set 9
                      local.get 1
                      local.get 6
                      call $dlmalloc::dlmalloc::Chunk::set_inuse
                      local.get 7
                      local.get 8
                      call $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_free_chunk
                      local.get 9
                      call $dlmalloc::dlmalloc::Chunk::clear_pinuse
                    end
                    local.get 5
                    local.get 7
                    i32.store offset=424
                    local.get 5
                    local.get 8
                    i32.store offset=416
                    local.get 1
                    br_if 6 (;@2;)
                    br 5 (;@3;)
                  end
                  local.get 7
                  local.get 6
                  i32.sub
                  local.tee 7
                  i32.const 16
                  i32.const 8
                  call $dlmalloc::dlmalloc::align_up
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 1
                  local.get 6
                  call $dlmalloc::dlmalloc::Chunk::plus_offset
                  local.set 8
                  local.get 1
                  local.get 6
                  call $dlmalloc::dlmalloc::Chunk::set_inuse
                  local.get 8
                  local.get 7
                  call $dlmalloc::dlmalloc::Chunk::set_inuse
                  local.get 5
                  local.get 8
                  local.get 7
                  call $dlmalloc::dlmalloc::Dlmalloc<A>::dispose_chunk
                end
                local.get 1
                br_if 4 (;@2;)
                br 3 (;@3;)
              end
              local.get 5
              i32.load offset=420
              local.get 7
              i32.add
              local.tee 7
              local.get 6
              i32.gt_u
              br_if 1 (;@4;)
              br 2 (;@3;)
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
            br 3 (;@1;)
          end
          local.get 1
          local.get 6
          call $dlmalloc::dlmalloc::Chunk::plus_offset
          local.set 8
          local.get 1
          local.get 6
          call $dlmalloc::dlmalloc::Chunk::set_inuse
          local.get 5
          local.get 8
          i32.store offset=428
          local.get 8
          local.get 7
          local.get 6
          i32.sub
          local.tee 6
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 5
          local.get 6
          i32.store offset=420
          local.get 1
          br_if 1 (;@2;)
        end
        local.get 5
        local.get 3
        call $dlmalloc::dlmalloc::Dlmalloc<A>::malloc
        local.tee 6
        i32.eqz
        br_if 1 (;@1;)
        local.get 6
        local.get 0
        local.get 1
        call $dlmalloc::dlmalloc::Chunk::size
        i32.const -8
        i32.const -4
        local.get 1
        call $dlmalloc::dlmalloc::Chunk::mmapped
        select
        i32.add
        local.tee 2
        local.get 3
        local.get 2
        local.get 3
        i32.lt_u
        select
        call $memcpy
        local.set 3
        local.get 5
        local.get 0
        call $dlmalloc::dlmalloc::Dlmalloc<A>::free
        local.get 3
        local.set 2
        br 1 (;@1;)
      end
      local.get 1
      call $dlmalloc::dlmalloc::Chunk::mmapped
      drop
      local.get 1
      call $dlmalloc::dlmalloc::Chunk::to_mem
      local.set 2
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
  (func $__rust_alloc_error_handler (;12;) (type 1) (param i32 i32)
    local.get 0
    local.get 1
    call $__rdl_oom
    return
  )
  (func $alloc::raw_vec::finish_grow (;13;) (type 7) (param i32 i32 i32 i32)
    (local i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          i32.const -1
          i32.le_s
          br_if 1 (;@2;)
          block ;; label = @4
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
                  local.tee 4
                  br_if 0 (;@7;)
                  block ;; label = @8
                    local.get 2
                    br_if 0 (;@8;)
                    local.get 1
                    local.set 3
                    br 4 (;@4;)
                  end
                  i32.const 0
                  i32.load8_u offset=1048576
                  drop
                  br 2 (;@5;)
                end
                local.get 3
                i32.load
                local.get 4
                local.get 1
                local.get 2
                call $__rust_realloc
                local.set 3
                br 2 (;@4;)
              end
              block ;; label = @6
                local.get 2
                br_if 0 (;@6;)
                local.get 1
                local.set 3
                br 2 (;@4;)
              end
              i32.const 0
              i32.load8_u offset=1048576
              drop
            end
            local.get 2
            local.get 1
            call $__rust_alloc
            local.set 3
          end
          block ;; label = @4
            local.get 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            local.get 3
            i32.store offset=4
            local.get 0
            i32.const 8
            i32.add
            local.get 2
            i32.store
            local.get 0
            i32.const 0
            i32.store
            return
          end
          local.get 0
          local.get 1
          i32.store offset=4
          local.get 0
          i32.const 8
          i32.add
          local.get 2
          i32.store
          br 2 (;@1;)
        end
        local.get 0
        i32.const 0
        i32.store offset=4
        local.get 0
        i32.const 8
        i32.add
        local.get 2
        i32.store
        br 1 (;@1;)
      end
      local.get 0
      i32.const 0
      i32.store offset=4
    end
    local.get 0
    i32.const 1
    i32.store
  )
  (func $alloc::raw_vec::RawVec<T,A>::reserve_for_push (;14;) (type 8) (param i32)
    (local i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 0
    i32.load offset=4
    local.tee 2
    i32.const 1
    i32.shl
    local.tee 3
    i32.const 4
    local.get 3
    i32.const 4
    i32.gt_u
    select
    local.tee 3
    i32.const 2
    i32.shl
    local.set 4
    local.get 3
    i32.const 536870912
    i32.lt_u
    i32.const 2
    i32.shl
    local.set 5
    block ;; label = @1
      block ;; label = @2
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.const 4
        i32.store offset=24
        local.get 1
        local.get 2
        i32.const 2
        i32.shl
        i32.store offset=28
        local.get 1
        local.get 0
        i32.load
        i32.store offset=20
        br 1 (;@1;)
      end
      local.get 1
      i32.const 0
      i32.store offset=24
    end
    local.get 1
    i32.const 8
    i32.add
    local.get 5
    local.get 4
    local.get 1
    i32.const 20
    i32.add
    call $alloc::raw_vec::finish_grow
    local.get 1
    i32.load offset=12
    local.set 2
    block ;; label = @1
      block ;; label = @2
        local.get 1
        i32.load offset=8
        br_if 0 (;@2;)
        local.get 0
        local.get 3
        i32.store offset=4
        local.get 0
        local.get 2
        i32.store
        br 1 (;@1;)
      end
      local.get 2
      i32.const -2147483647
      i32.eq
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 1
        i32.const 16
        i32.add
        i32.load
        call $alloc::alloc::handle_alloc_error
        unreachable
      end
      call $alloc::raw_vec::capacity_overflow
      unreachable
    end
    local.get 1
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $vec_alloc (;15;) (type 5) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    local.get 0
    i32.const 0
    i32.store offset=12
    local.get 0
    i64.const 4
    i64.store offset=4 align=4
    local.get 0
    i32.const 4
    i32.add
    call $alloc::raw_vec::RawVec<T,A>::reserve_for_push
    local.get 0
    i32.load offset=4
    local.tee 1
    local.get 0
    i32.load offset=12
    local.tee 2
    i32.const 2
    i32.shl
    i32.add
    i32.const 1
    i32.store
    block ;; label = @1
      local.get 2
      i32.const -1
      i32.eq
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 0
        i32.load offset=8
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        local.get 2
        i32.const 2
        i32.shl
        i32.const 4
        call $__rust_dealloc
      end
      local.get 0
      i32.const 16
      i32.add
      global.set $__stack_pointer
      i32.const 1
      return
    end
    unreachable
    unreachable
  )
  (func $alloc::raw_vec::capacity_overflow (;16;) (type 9)
    unreachable
    unreachable
  )
  (func $alloc::alloc::handle_alloc_error (;17;) (type 1) (param i32 i32)
    local.get 0
    local.get 1
    call $alloc::alloc::handle_alloc_error::rt_error
    unreachable
  )
  (func $alloc::alloc::handle_alloc_error::rt_error (;18;) (type 1) (param i32 i32)
    local.get 1
    local.get 0
    call $__rust_alloc_error_handler
    unreachable
  )
  (func $__rdl_oom (;19;) (type 1) (param i32 i32)
    unreachable
    unreachable
  )
  (func $dlmalloc::dlmalloc::align_up (;20;) (type 3) (param i32 i32) (result i32)
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
  (func $dlmalloc::dlmalloc::left_bits (;21;) (type 2) (param i32) (result i32)
    local.get 0
    i32.const 1
    i32.shl
    local.tee 0
    i32.const 0
    local.get 0
    i32.sub
    i32.or
  )
  (func $dlmalloc::dlmalloc::least_bit (;22;) (type 2) (param i32) (result i32)
    i32.const 0
    local.get 0
    i32.sub
    local.get 0
    i32.and
  )
  (func $dlmalloc::dlmalloc::leftshift_for_tree_index (;23;) (type 2) (param i32) (result i32)
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
  (func $dlmalloc::dlmalloc::Chunk::fencepost_head (;24;) (type 5) (result i32)
    i32.const 7
  )
  (func $dlmalloc::dlmalloc::Chunk::size (;25;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load offset=4
    i32.const -8
    i32.and
  )
  (func $dlmalloc::dlmalloc::Chunk::cinuse (;26;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load8_u offset=4
    i32.const 2
    i32.and
    i32.const 1
    i32.shr_u
  )
  (func $dlmalloc::dlmalloc::Chunk::pinuse (;27;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load offset=4
    i32.const 1
    i32.and
  )
  (func $dlmalloc::dlmalloc::Chunk::clear_pinuse (;28;) (type 8) (param i32)
    local.get 0
    local.get 0
    i32.load offset=4
    i32.const -2
    i32.and
    i32.store offset=4
  )
  (func $dlmalloc::dlmalloc::Chunk::inuse (;29;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load offset=4
    i32.const 3
    i32.and
    i32.const 1
    i32.ne
  )
  (func $dlmalloc::dlmalloc::Chunk::mmapped (;30;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load8_u offset=4
    i32.const 3
    i32.and
    i32.eqz
  )
  (func $dlmalloc::dlmalloc::Chunk::set_inuse (;31;) (type 1) (param i32 i32)
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
  (func $dlmalloc::dlmalloc::Chunk::set_inuse_and_pinuse (;32;) (type 1) (param i32 i32)
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
  (func $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_inuse_chunk (;33;) (type 1) (param i32 i32)
    local.get 0
    local.get 1
    i32.const 3
    i32.or
    i32.store offset=4
  )
  (func $dlmalloc::dlmalloc::Chunk::set_size_and_pinuse_of_free_chunk (;34;) (type 1) (param i32 i32)
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
  (func $dlmalloc::dlmalloc::Chunk::set_free_with_pinuse (;35;) (type 0) (param i32 i32 i32)
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
  (func $dlmalloc::dlmalloc::Chunk::plus_offset (;36;) (type 3) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.add
  )
  (func $dlmalloc::dlmalloc::Chunk::minus_offset (;37;) (type 3) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.sub
  )
  (func $dlmalloc::dlmalloc::Chunk::to_mem (;38;) (type 2) (param i32) (result i32)
    local.get 0
    i32.const 8
    i32.add
  )
  (func $dlmalloc::dlmalloc::Chunk::mem_offset (;39;) (type 5) (result i32)
    i32.const 8
  )
  (func $dlmalloc::dlmalloc::Chunk::from_mem (;40;) (type 2) (param i32) (result i32)
    local.get 0
    i32.const -8
    i32.add
  )
  (func $dlmalloc::dlmalloc::TreeChunk::leftmost_child (;41;) (type 2) (param i32) (result i32)
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
  (func $dlmalloc::dlmalloc::TreeChunk::chunk (;42;) (type 2) (param i32) (result i32)
    local.get 0
  )
  (func $dlmalloc::dlmalloc::TreeChunk::next (;43;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load offset=12
  )
  (func $dlmalloc::dlmalloc::TreeChunk::prev (;44;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load offset=8
  )
  (func $dlmalloc::dlmalloc::Segment::is_extern (;45;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load offset=12
    i32.const 1
    i32.and
  )
  (func $dlmalloc::dlmalloc::Segment::sys_flags (;46;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load offset=12
    i32.const 1
    i32.shr_u
  )
  (func $dlmalloc::dlmalloc::Segment::holds (;47;) (type 3) (param i32 i32) (result i32)
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
  (func $dlmalloc::dlmalloc::Segment::top (;48;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    i32.add
  )
  (func $<dlmalloc::global::Instance as core::ops::deref::DerefMut>::deref_mut (;49;) (type 2) (param i32) (result i32)
    i32.const 1048580
  )
  (func $<dlmalloc::global::Instance as core::ops::drop::Drop>::drop (;50;) (type 8) (param i32))
  (func $<dlmalloc::sys::System as dlmalloc::Allocator>::alloc (;51;) (type 0) (param i32 i32 i32)
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
  (func $<dlmalloc::sys::System as dlmalloc::Allocator>::remap (;52;) (type 10) (param i32 i32 i32 i32 i32) (result i32)
    i32.const 0
  )
  (func $<dlmalloc::sys::System as dlmalloc::Allocator>::free_part (;53;) (type 6) (param i32 i32 i32 i32) (result i32)
    i32.const 0
  )
  (func $<dlmalloc::sys::System as dlmalloc::Allocator>::free (;54;) (type 4) (param i32 i32 i32) (result i32)
    i32.const 0
  )
  (func $<dlmalloc::sys::System as dlmalloc::Allocator>::can_release_part (;55;) (type 3) (param i32 i32) (result i32)
    i32.const 0
  )
  (func $<dlmalloc::sys::System as dlmalloc::Allocator>::page_size (;56;) (type 2) (param i32) (result i32)
    i32.const 65536
  )
  (func $dlmalloc::sys::enable_alloc_after_fork (;57;) (type 9))
  (func $memcpy (;58;) (type 4) (param i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    call $compiler_builtins::mem::memcpy
  )
  (func $compiler_builtins::mem::memcpy (;59;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        local.get 2
        i32.const 15
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        local.set 3
        br 1 (;@1;)
      end
      local.get 0
      i32.const 0
      local.get 0
      i32.sub
      i32.const 3
      i32.and
      local.tee 4
      i32.add
      local.set 5
      block ;; label = @2
        local.get 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        local.set 3
        local.get 1
        local.set 6
        loop ;; label = @3
          local.get 3
          local.get 6
          i32.load8_u
          i32.store8
          local.get 6
          i32.const 1
          i32.add
          local.set 6
          local.get 3
          i32.const 1
          i32.add
          local.tee 3
          local.get 5
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 5
      local.get 2
      local.get 4
      i32.sub
      local.tee 7
      i32.const -4
      i32.and
      local.tee 8
      i32.add
      local.set 3
      block ;; label = @2
        block ;; label = @3
          local.get 1
          local.get 4
          i32.add
          local.tee 9
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 8
          i32.const 1
          i32.lt_s
          br_if 1 (;@2;)
          local.get 9
          i32.const 3
          i32.shl
          local.tee 6
          i32.const 24
          i32.and
          local.set 2
          local.get 9
          i32.const -4
          i32.and
          local.tee 10
          i32.const 4
          i32.add
          local.set 1
          i32.const 0
          local.get 6
          i32.sub
          i32.const 24
          i32.and
          local.set 4
          local.get 10
          i32.load
          local.set 6
          loop ;; label = @4
            local.get 5
            local.get 6
            local.get 2
            i32.shr_u
            local.get 1
            i32.load
            local.tee 6
            local.get 4
            i32.shl
            i32.or
            i32.store
            local.get 1
            i32.const 4
            i32.add
            local.set 1
            local.get 5
            i32.const 4
            i32.add
            local.tee 5
            local.get 3
            i32.lt_u
            br_if 0 (;@4;)
            br 2 (;@2;)
          end
        end
        local.get 8
        i32.const 1
        i32.lt_s
        br_if 0 (;@2;)
        local.get 9
        local.set 1
        loop ;; label = @3
          local.get 5
          local.get 1
          i32.load
          i32.store
          local.get 1
          i32.const 4
          i32.add
          local.set 1
          local.get 5
          i32.const 4
          i32.add
          local.tee 5
          local.get 3
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 7
      i32.const 3
      i32.and
      local.set 2
      local.get 9
      local.get 8
      i32.add
      local.set 1
    end
    block ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      local.get 2
      i32.add
      local.set 5
      loop ;; label = @2
        local.get 3
        local.get 1
        i32.load8_u
        i32.store8
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 3
        i32.const 1
        i32.add
        local.tee 3
        local.get 5
        i32.lt_u
        br_if 0 (;@2;)
      end
    end
    local.get 0
  )
  (table (;0;) 1 1 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (global (;1;) i32 i32.const 1049032)
  (global (;2;) i32 i32.const 1049040)
  (export "memory" (memory 0))
  (export "__main" (func $__main))
  (export "vec_alloc" (func $vec_alloc))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
)