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
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$13dispose_chunk17h577eb103dc04307bE (;0;) (type 0) (param i32 i32 i32)
    (local i32 i32 i32 i32)
    local.get 1
    local.get 2
    call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
    local.set 3
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          call $_ZN8dlmalloc8dlmalloc5Chunk6pinuse17h92d5107047b03ba7E
          br_if 0 (;@3;)
          local.get 1
          i32.load
          local.set 4
          block ;; label = @4
            local.get 1
            call $_ZN8dlmalloc8dlmalloc5Chunk7mmapped17h1a9959fbf47496c3E
            br_if 0 (;@4;)
            local.get 4
            local.get 2
            i32.add
            local.set 2
            block ;; label = @5
              local.get 1
              local.get 4
              call $_ZN8dlmalloc8dlmalloc5Chunk12minus_offset17h7c3eec81761249d9E
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
              call $_ZN8dlmalloc8dlmalloc5Chunk20set_free_with_pinuse17h5d876ea751634e99E
              return
            end
            block ;; label = @5
              local.get 4
              i32.const 256
              i32.lt_u
              br_if 0 (;@5;)
              local.get 0
              local.get 1
              call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18unlink_large_chunk17h2e279402ce6356d4E
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
          call $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$4free17h993c5f05ba1214bcE
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
          call $_ZN8dlmalloc8dlmalloc5Chunk6cinuse17h58499de57c2d37e2E
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          local.get 2
          local.get 3
          call $_ZN8dlmalloc8dlmalloc5Chunk20set_free_with_pinuse17h5d876ea751634e99E
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
            call $_ZN8dlmalloc8dlmalloc5Chunk4size17h77d1c406ab42db33E
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
                call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18unlink_large_chunk17h2e279402ce6356d4E
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
            call $_ZN8dlmalloc8dlmalloc5Chunk33set_size_and_pinuse_of_free_chunk17ha971516d0be71949E
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
        call $_ZN8dlmalloc8dlmalloc5Chunk33set_size_and_pinuse_of_free_chunk17ha971516d0be71949E
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
      call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18insert_large_chunk17h8e77460818b80af0E
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
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18unlink_large_chunk17h2e279402ce6356d4E (;1;) (type 1) (param i32 i32)
    (local i32 i32 i32 i32 i32)
    local.get 1
    i32.load offset=24
    local.set 2
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          call $_ZN8dlmalloc8dlmalloc9TreeChunk4next17he250edbec5d87123E
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
        call $_ZN8dlmalloc8dlmalloc9TreeChunk4prev17h7a0f1d46544cc14aE
        local.tee 5
        local.get 1
        call $_ZN8dlmalloc8dlmalloc9TreeChunk4next17he250edbec5d87123E
        local.tee 3
        call $_ZN8dlmalloc8dlmalloc9TreeChunk5chunk17h4efd58110bb4b6e5E
        i32.store offset=12
        local.get 3
        local.get 5
        call $_ZN8dlmalloc8dlmalloc9TreeChunk5chunk17h4efd58110bb4b6e5E
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
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18insert_large_chunk17h8e77460818b80af0E (;2;) (type 0) (param i32 i32 i32)
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
    call $_ZN8dlmalloc8dlmalloc9TreeChunk5chunk17h4efd58110bb4b6e5E
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
        call $_ZN8dlmalloc8dlmalloc24leftshift_for_tree_index17h31d064fdd867f502E
        i32.shl
        local.set 3
        loop ;; label = @3
          block ;; label = @4
            local.get 0
            local.tee 4
            call $_ZN8dlmalloc8dlmalloc9TreeChunk5chunk17h4efd58110bb4b6e5E
            call $_ZN8dlmalloc8dlmalloc5Chunk4size17h77d1c406ab42db33E
            local.get 2
            i32.ne
            br_if 0 (;@4;)
            local.get 4
            call $_ZN8dlmalloc8dlmalloc9TreeChunk5chunk17h4efd58110bb4b6e5E
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
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$23release_unused_segments17h25622465f0742468E (;3;) (type 2) (param i32) (result i32)
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
            call $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$16can_release_part17h43bfb7d8666fcc31E
            i32.eqz
            br_if 0 (;@4;)
            local.get 5
            call $_ZN8dlmalloc8dlmalloc7Segment9is_extern17h6f6db2c70b891fd9E
            br_if 0 (;@4;)
            local.get 7
            local.get 7
            call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h75497733644e1d6cE
            local.tee 8
            i32.const 8
            call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
            local.get 8
            i32.sub
            i32.add
            local.tee 8
            call $_ZN8dlmalloc8dlmalloc5Chunk4size17h77d1c406ab42db33E
            local.set 9
            call $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h86551c33e07de253E
            local.tee 10
            i32.const 8
            call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
            local.set 11
            i32.const 20
            i32.const 8
            call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
            local.set 12
            i32.const 16
            i32.const 8
            call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
            local.set 13
            local.get 8
            call $_ZN8dlmalloc8dlmalloc5Chunk5inuse17h2d327e4c36b84dfeE
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
                call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18unlink_large_chunk17h2e279402ce6356d4E
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
              call $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$4free17h993c5f05ba1214bcE
              br_if 0 (;@5;)
              local.get 0
              local.get 8
              local.get 9
              call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18insert_large_chunk17h8e77460818b80af0E
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
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$4free17h501de2a6604ba1ffE (;4;) (type 1) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    local.get 1
    call $_ZN8dlmalloc8dlmalloc5Chunk8from_mem17h11dd30c74f483706E
    local.set 1
    local.get 1
    local.get 1
    call $_ZN8dlmalloc8dlmalloc5Chunk4size17h77d1c406ab42db33E
    local.tee 2
    call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
    local.set 3
    block ;; label = @1
      block ;; label = @2
        local.get 1
        call $_ZN8dlmalloc8dlmalloc5Chunk6pinuse17h92d5107047b03ba7E
        br_if 0 (;@2;)
        local.get 1
        i32.load
        local.set 4
        block ;; label = @3
          local.get 1
          call $_ZN8dlmalloc8dlmalloc5Chunk7mmapped17h1a9959fbf47496c3E
          br_if 0 (;@3;)
          local.get 4
          local.get 2
          i32.add
          local.set 2
          block ;; label = @4
            local.get 1
            local.get 4
            call $_ZN8dlmalloc8dlmalloc5Chunk12minus_offset17h7c3eec81761249d9E
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
            call $_ZN8dlmalloc8dlmalloc5Chunk20set_free_with_pinuse17h5d876ea751634e99E
            return
          end
          block ;; label = @4
            local.get 4
            i32.const 256
            i32.lt_u
            br_if 0 (;@4;)
            local.get 0
            local.get 1
            call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18unlink_large_chunk17h2e279402ce6356d4E
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
        call $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$4free17h993c5f05ba1214bcE
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
          call $_ZN8dlmalloc8dlmalloc5Chunk6cinuse17h58499de57c2d37e2E
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          local.get 2
          local.get 3
          call $_ZN8dlmalloc8dlmalloc5Chunk20set_free_with_pinuse17h5d876ea751634e99E
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
                call $_ZN8dlmalloc8dlmalloc5Chunk4size17h77d1c406ab42db33E
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
                    call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18unlink_large_chunk17h2e279402ce6356d4E
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
                call $_ZN8dlmalloc8dlmalloc5Chunk33set_size_and_pinuse_of_free_chunk17ha971516d0be71949E
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
            call $_ZN8dlmalloc8dlmalloc5Chunk33set_size_and_pinuse_of_free_chunk17ha971516d0be71949E
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
        call $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h86551c33e07de253E
        local.tee 1
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
        local.set 2
        i32.const 20
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
        local.set 3
        i32.const 16
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
        local.set 4
        i32.const 0
        i32.const 16
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
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
        call $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h86551c33e07de253E
        local.tee 1
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
        local.set 3
        i32.const 20
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
        local.set 5
        i32.const 16
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
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
                call $_ZN8dlmalloc8dlmalloc7Segment3top17he7e9e2493151d036E
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
          call $_ZN8dlmalloc8dlmalloc7Segment9is_extern17h6f6db2c70b891fd9E
          br_if 0 (;@3;)
          i32.const 0
          local.set 4
          local.get 0
          local.get 1
          i32.load offset=12
          i32.const 1
          i32.shr_u
          call $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$16can_release_part17h43bfb7d8666fcc31E
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
              call $_ZN8dlmalloc8dlmalloc7Segment5holds17h8f6de4ee6718009bE
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
          call $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$9free_part17h74489c9e7a3aa967E
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
          call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h75497733644e1d6cE
          local.tee 3
          i32.const 8
          call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
          local.get 3
          i32.sub
          local.tee 3
          call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
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
          call $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h86551c33e07de253E
          local.tee 3
          i32.const 8
          call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
          local.set 4
          i32.const 20
          i32.const 8
          call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
          local.set 6
          i32.const 16
          i32.const 8
          call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
          local.set 7
          local.get 1
          local.get 2
          call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
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
        call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$23release_unused_segments17h25622465f0742468E
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
        call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18insert_large_chunk17h8e77460818b80af0E
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
        call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$23release_unused_segments17h25622465f0742468E
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
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$6malloc17h1ae2390053e3628cE (;5;) (type 3) (param i32 i32) (result i32)
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
                call $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h86551c33e07de253E
                local.tee 3
                i32.const 8
                call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
                local.set 4
                i32.const 20
                i32.const 8
                call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
                local.set 5
                i32.const 16
                i32.const 8
                call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
                local.set 6
                i32.const 0
                local.set 7
                i32.const 0
                i32.const 16
                i32.const 8
                call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
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
                call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
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
                call $_ZN8dlmalloc8dlmalloc24leftshift_for_tree_index17h31d064fdd867f502E
                i32.shl
                local.set 6
                i32.const 0
                local.set 1
                i32.const 0
                local.set 5
                loop ;; label = @7
                  block ;; label = @8
                    local.get 7
                    call $_ZN8dlmalloc8dlmalloc9TreeChunk5chunk17h4efd58110bb4b6e5E
                    call $_ZN8dlmalloc8dlmalloc5Chunk4size17h77d1c406ab42db33E
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
              call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
              i32.const -5
              i32.add
              local.get 1
              i32.gt_u
              select
              i32.const 8
              call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
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
                call $_ZN8dlmalloc8dlmalloc5Chunk20set_inuse_and_pinuse17h515e5a69a6d1edc6E
                local.get 1
                call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h75497733644e1d6cE
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
                            call $_ZN8dlmalloc8dlmalloc9least_bit17h4bca52ead665dc5aE
                            i32.ctz
                            i32.const 2
                            i32.shl
                            i32.add
                            i32.load
                            local.tee 7
                            call $_ZN8dlmalloc8dlmalloc9TreeChunk5chunk17h4efd58110bb4b6e5E
                            call $_ZN8dlmalloc8dlmalloc5Chunk4size17h77d1c406ab42db33E
                            local.get 3
                            i32.sub
                            local.set 4
                            block ;; label = @13
                              local.get 7
                              call $_ZN8dlmalloc8dlmalloc9TreeChunk14leftmost_child17h20605933c801b44bE
                              local.tee 1
                              i32.eqz
                              br_if 0 (;@13;)
                              loop ;; label = @14
                                local.get 1
                                call $_ZN8dlmalloc8dlmalloc9TreeChunk5chunk17h4efd58110bb4b6e5E
                                call $_ZN8dlmalloc8dlmalloc5Chunk4size17h77d1c406ab42db33E
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
                                call $_ZN8dlmalloc8dlmalloc9TreeChunk14leftmost_child17h20605933c801b44bE
                                local.tee 1
                                br_if 0 (;@14;)
                              end
                            end
                            local.get 7
                            call $_ZN8dlmalloc8dlmalloc9TreeChunk5chunk17h4efd58110bb4b6e5E
                            local.tee 1
                            local.get 3
                            call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
                            local.set 5
                            local.get 0
                            local.get 7
                            call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18unlink_large_chunk17h2e279402ce6356d4E
                            local.get 4
                            i32.const 16
                            i32.const 8
                            call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
                            i32.lt_u
                            br_if 2 (;@10;)
                            local.get 5
                            call $_ZN8dlmalloc8dlmalloc9TreeChunk5chunk17h4efd58110bb4b6e5E
                            local.set 5
                            local.get 1
                            local.get 3
                            call $_ZN8dlmalloc8dlmalloc5Chunk34set_size_and_pinuse_of_inuse_chunk17h4acf6d59020bd397E
                            local.get 5
                            local.get 4
                            call $_ZN8dlmalloc8dlmalloc5Chunk33set_size_and_pinuse_of_free_chunk17ha971516d0be71949E
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
                              call $_ZN8dlmalloc8dlmalloc9left_bits17hb6cbe146b8019d98E
                              local.get 1
                              local.get 4
                              i32.shl
                              i32.and
                              call $_ZN8dlmalloc8dlmalloc9least_bit17h4bca52ead665dc5aE
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
                          call $_ZN8dlmalloc8dlmalloc5Chunk34set_size_and_pinuse_of_inuse_chunk17h4acf6d59020bd397E
                          local.get 1
                          local.get 3
                          call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
                          local.tee 5
                          local.get 7
                          i32.const 3
                          i32.shl
                          local.get 3
                          i32.sub
                          local.tee 6
                          call $_ZN8dlmalloc8dlmalloc5Chunk33set_size_and_pinuse_of_free_chunk17ha971516d0be71949E
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
                      call $_ZN8dlmalloc8dlmalloc5Chunk20set_inuse_and_pinuse17h515e5a69a6d1edc6E
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
                  call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h75497733644e1d6cE
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
              call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h75497733644e1d6cE
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
              call $_ZN8dlmalloc8dlmalloc9left_bits17hb6cbe146b8019d98E
              local.get 9
              i32.and
              local.tee 1
              i32.eqz
              br_if 3 (;@2;)
              local.get 0
              local.get 1
              call $_ZN8dlmalloc8dlmalloc9least_bit17h4bca52ead665dc5aE
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
            call $_ZN8dlmalloc8dlmalloc9TreeChunk5chunk17h4efd58110bb4b6e5E
            call $_ZN8dlmalloc8dlmalloc5Chunk4size17h77d1c406ab42db33E
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
            call $_ZN8dlmalloc8dlmalloc9TreeChunk14leftmost_child17h20605933c801b44bE
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
        call $_ZN8dlmalloc8dlmalloc9TreeChunk5chunk17h4efd58110bb4b6e5E
        local.tee 1
        local.get 3
        call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
        local.set 7
        local.get 0
        local.get 5
        call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18unlink_large_chunk17h2e279402ce6356d4E
        block ;; label = @3
          block ;; label = @4
            local.get 4
            i32.const 16
            i32.const 8
            call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
            i32.lt_u
            br_if 0 (;@4;)
            local.get 1
            local.get 3
            call $_ZN8dlmalloc8dlmalloc5Chunk34set_size_and_pinuse_of_inuse_chunk17h4acf6d59020bd397E
            local.get 7
            local.get 4
            call $_ZN8dlmalloc8dlmalloc5Chunk33set_size_and_pinuse_of_free_chunk17ha971516d0be71949E
            block ;; label = @5
              local.get 4
              i32.const 256
              i32.lt_u
              br_if 0 (;@5;)
              local.get 0
              local.get 7
              local.get 4
              call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18insert_large_chunk17h8e77460818b80af0E
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
          call $_ZN8dlmalloc8dlmalloc5Chunk20set_inuse_and_pinuse17h515e5a69a6d1edc6E
        end
        local.get 1
        call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h75497733644e1d6cE
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
                        call $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h86551c33e07de253E
                        local.tee 1
                        i32.sub
                        local.get 1
                        i32.const 8
                        call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
                        i32.add
                        i32.const 20
                        i32.const 8
                        call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
                        i32.add
                        i32.const 16
                        i32.const 8
                        call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
                        i32.add
                        i32.const 8
                        i32.add
                        i32.const 65536
                        call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
                        call $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$5alloc17hdbf1e2bcc01bc909E
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
                              call $_ZN8dlmalloc8dlmalloc7Segment3top17he7e9e2493151d036E
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
                          call $_ZN8dlmalloc8dlmalloc7Segment9is_extern17h6f6db2c70b891fd9E
                          br_if 0 (;@11;)
                          local.get 1
                          call $_ZN8dlmalloc8dlmalloc7Segment9sys_flags17h224550055bf7775bE
                          local.get 11
                          i32.ne
                          br_if 0 (;@11;)
                          local.get 1
                          local.get 0
                          i32.load offset=428
                          call $_ZN8dlmalloc8dlmalloc7Segment5holds17h8f6de4ee6718009bE
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
                            call $_ZN8dlmalloc8dlmalloc7Segment9is_extern17h6f6db2c70b891fd9E
                            br_if 0 (;@12;)
                            local.get 1
                            call $_ZN8dlmalloc8dlmalloc7Segment9sys_flags17h224550055bf7775bE
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
                                call $_ZN8dlmalloc8dlmalloc7Segment3top17he7e9e2493151d036E
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
                          call $_ZN8dlmalloc8dlmalloc7Segment3top17he7e9e2493151d036E
                          local.tee 6
                          i32.const 20
                          i32.const 8
                          call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
                          local.tee 12
                          i32.sub
                          i32.const -23
                          i32.add
                          local.set 1
                          local.get 5
                          local.get 1
                          local.get 1
                          call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h75497733644e1d6cE
                          local.tee 4
                          i32.const 8
                          call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
                          local.get 4
                          i32.sub
                          i32.add
                          local.tee 1
                          local.get 1
                          local.get 5
                          i32.const 16
                          i32.const 8
                          call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
                          i32.add
                          i32.lt_u
                          select
                          local.tee 13
                          call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h75497733644e1d6cE
                          local.set 4
                          local.get 13
                          local.get 12
                          call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
                          local.set 1
                          call $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h86551c33e07de253E
                          local.tee 14
                          i32.const 8
                          call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
                          local.set 15
                          i32.const 20
                          i32.const 8
                          call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
                          local.set 16
                          i32.const 16
                          i32.const 8
                          call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
                          local.set 17
                          local.get 0
                          local.get 8
                          local.get 8
                          call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h75497733644e1d6cE
                          local.tee 18
                          i32.const 8
                          call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
                          local.get 18
                          i32.sub
                          local.tee 19
                          call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
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
                          call $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h86551c33e07de253E
                          local.tee 15
                          i32.const 8
                          call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
                          local.set 16
                          i32.const 20
                          i32.const 8
                          call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
                          local.set 17
                          i32.const 16
                          i32.const 8
                          call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
                          local.set 19
                          local.get 18
                          local.get 14
                          call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
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
                          call $_ZN8dlmalloc8dlmalloc5Chunk34set_size_and_pinuse_of_inuse_chunk17h4acf6d59020bd397E
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
                            call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
                            local.set 4
                            local.get 1
                            call $_ZN8dlmalloc8dlmalloc5Chunk14fencepost_head17he07aaa52f3b50dfdE
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
                          call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
                          call $_ZN8dlmalloc8dlmalloc5Chunk20set_free_with_pinuse17h5d876ea751634e99E
                          block ;; label = @12
                            local.get 1
                            i32.const 256
                            i32.lt_u
                            br_if 0 (;@12;)
                            local.get 0
                            local.get 5
                            local.get 1
                            call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18insert_large_chunk17h8e77460818b80af0E
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
                        call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h75497733644e1d6cE
                        local.tee 1
                        i32.const 8
                        call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
                        local.set 4
                        local.get 5
                        call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h75497733644e1d6cE
                        local.tee 6
                        i32.const 8
                        call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
                        local.set 10
                        local.get 8
                        local.get 4
                        local.get 1
                        i32.sub
                        i32.add
                        local.tee 4
                        local.get 3
                        call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
                        local.set 7
                        local.get 4
                        local.get 3
                        call $_ZN8dlmalloc8dlmalloc5Chunk34set_size_and_pinuse_of_inuse_chunk17h4acf6d59020bd397E
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
                          call $_ZN8dlmalloc8dlmalloc5Chunk5inuse17h2d327e4c36b84dfeE
                          br_if 7 (;@4;)
                          block ;; label = @12
                            block ;; label = @13
                              local.get 1
                              call $_ZN8dlmalloc8dlmalloc5Chunk4size17h77d1c406ab42db33E
                              local.tee 5
                              i32.const 256
                              i32.lt_u
                              br_if 0 (;@13;)
                              local.get 0
                              local.get 1
                              call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18unlink_large_chunk17h2e279402ce6356d4E
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
                          call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
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
                        call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h75497733644e1d6cE
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
                      call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
                      local.tee 7
                      i32.store offset=428
                      local.get 7
                      local.get 4
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 1
                      local.get 3
                      call $_ZN8dlmalloc8dlmalloc5Chunk34set_size_and_pinuse_of_inuse_chunk17h4acf6d59020bd397E
                      local.get 1
                      call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h75497733644e1d6cE
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
                    call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
                    i32.lt_u
                    br_if 3 (;@5;)
                    local.get 1
                    local.get 3
                    call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
                    local.set 7
                    local.get 0
                    local.get 4
                    i32.store offset=416
                    local.get 0
                    local.get 7
                    i32.store offset=424
                    local.get 7
                    local.get 4
                    call $_ZN8dlmalloc8dlmalloc5Chunk33set_size_and_pinuse_of_free_chunk17ha971516d0be71949E
                    local.get 1
                    local.get 3
                    call $_ZN8dlmalloc8dlmalloc5Chunk34set_size_and_pinuse_of_inuse_chunk17h4acf6d59020bd397E
                    local.get 1
                    call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h75497733644e1d6cE
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
                call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$8init_top17he4cefe3b36a3bd87E
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
              call $_ZN8dlmalloc8dlmalloc5Chunk33set_size_and_pinuse_of_free_chunk17ha971516d0be71949E
              local.get 4
              call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h75497733644e1d6cE
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
            call $_ZN8dlmalloc8dlmalloc5Chunk20set_inuse_and_pinuse17h515e5a69a6d1edc6E
            local.get 1
            call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h75497733644e1d6cE
            local.set 7
            br 3 (;@1;)
          end
          local.get 7
          local.get 3
          local.get 1
          call $_ZN8dlmalloc8dlmalloc5Chunk20set_free_with_pinuse17h5d876ea751634e99E
          block ;; label = @4
            local.get 3
            i32.const 256
            i32.lt_u
            br_if 0 (;@4;)
            local.get 0
            local.get 7
            local.get 3
            call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18insert_large_chunk17h8e77460818b80af0E
            local.get 4
            call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h75497733644e1d6cE
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
          call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h75497733644e1d6cE
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
        call $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h86551c33e07de253E
        local.tee 4
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
        local.set 5
        i32.const 20
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
        local.set 6
        i32.const 16
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
        local.set 9
        local.get 0
        local.get 8
        local.get 8
        call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h75497733644e1d6cE
        local.tee 1
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
        local.get 1
        i32.sub
        local.tee 13
        call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
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
        call $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h86551c33e07de253E
        local.tee 5
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
        local.set 6
        i32.const 20
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
        local.set 8
        i32.const 16
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
        local.set 10
        local.get 1
        local.get 4
        call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
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
      call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
      local.tee 7
      i32.store offset=428
      local.get 7
      local.get 4
      i32.const 1
      i32.or
      i32.store offset=4
      local.get 1
      local.get 3
      call $_ZN8dlmalloc8dlmalloc5Chunk34set_size_and_pinuse_of_inuse_chunk17h4acf6d59020bd397E
      local.get 1
      call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h75497733644e1d6cE
      local.set 7
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 7
  )
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$8init_top17he4cefe3b36a3bd87E (;6;) (type 0) (param i32 i32 i32)
    (local i32 i32 i32 i32)
    local.get 1
    local.get 1
    call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h75497733644e1d6cE
    local.tee 3
    i32.const 8
    call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
    local.get 3
    i32.sub
    local.tee 3
    call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
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
    call $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h86551c33e07de253E
    local.tee 3
    i32.const 8
    call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
    local.set 4
    i32.const 20
    i32.const 8
    call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
    local.set 5
    i32.const 16
    i32.const 8
    call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
    local.set 6
    local.get 1
    local.get 2
    call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
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
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$8memalign17he8794c5d1cb954f9E (;7;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      i32.const 16
      i32.const 8
      call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
      local.get 1
      i32.le_u
      br_if 0 (;@1;)
      i32.const 16
      i32.const 8
      call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
      local.set 1
    end
    call $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h86551c33e07de253E
    local.tee 3
    i32.const 8
    call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
    local.set 4
    i32.const 20
    i32.const 8
    call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
    local.set 5
    i32.const 16
    i32.const 8
    call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
    local.set 6
    i32.const 0
    local.set 7
    block ;; label = @1
      i32.const 0
      i32.const 16
      i32.const 8
      call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
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
      call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
      i32.const -5
      i32.add
      local.get 2
      i32.gt_u
      select
      i32.const 8
      call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
      local.tee 4
      i32.add
      i32.const 16
      i32.const 8
      call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
      i32.add
      i32.const -4
      i32.add
      call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$6malloc17h1ae2390053e3628cE
      local.tee 3
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      call $_ZN8dlmalloc8dlmalloc5Chunk8from_mem17h11dd30c74f483706E
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
        call $_ZN8dlmalloc8dlmalloc5Chunk8from_mem17h11dd30c74f483706E
        local.set 7
        i32.const 16
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
        local.set 3
        local.get 2
        call $_ZN8dlmalloc8dlmalloc5Chunk4size17h77d1c406ab42db33E
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
          call $_ZN8dlmalloc8dlmalloc5Chunk7mmapped17h1a9959fbf47496c3E
          br_if 0 (;@3;)
          local.get 1
          local.get 3
          call $_ZN8dlmalloc8dlmalloc5Chunk9set_inuse17h4282057414c4e601E
          local.get 2
          local.get 7
          call $_ZN8dlmalloc8dlmalloc5Chunk9set_inuse17h4282057414c4e601E
          local.get 0
          local.get 2
          local.get 7
          call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$13dispose_chunk17h577eb103dc04307bE
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
        call $_ZN8dlmalloc8dlmalloc5Chunk7mmapped17h1a9959fbf47496c3E
        br_if 0 (;@2;)
        local.get 1
        call $_ZN8dlmalloc8dlmalloc5Chunk4size17h77d1c406ab42db33E
        local.tee 2
        i32.const 16
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
        local.get 4
        i32.add
        i32.le_u
        br_if 0 (;@2;)
        local.get 1
        local.get 4
        call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
        local.set 7
        local.get 1
        local.get 4
        call $_ZN8dlmalloc8dlmalloc5Chunk9set_inuse17h4282057414c4e601E
        local.get 7
        local.get 2
        local.get 4
        i32.sub
        local.tee 2
        call $_ZN8dlmalloc8dlmalloc5Chunk9set_inuse17h4282057414c4e601E
        local.get 0
        local.get 7
        local.get 2
        call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$13dispose_chunk17h577eb103dc04307bE
      end
      local.get 1
      call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h75497733644e1d6cE
      local.set 7
      local.get 1
      call $_ZN8dlmalloc8dlmalloc5Chunk7mmapped17h1a9959fbf47496c3E
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
    call $_ZN8dlmalloc3sys23enable_alloc_after_fork17h64eb1fc0ff7b2689E
    local.get 2
    i32.const 15
    i32.add
    call $_ZN73_$LT$dlmalloc..global..Instance$u20$as$u20$core..ops..deref..DerefMut$GT$9deref_mut17h16955ef502c5c4e5E
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
        call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$8memalign17he8794c5d1cb954f9E
        local.set 1
        br 1 (;@1;)
      end
      local.get 3
      local.get 0
      call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$6malloc17h1ae2390053e3628cE
      local.set 1
    end
    local.get 2
    i32.const 15
    i32.add
    call $_ZN68_$LT$dlmalloc..global..Instance$u20$as$u20$core..ops..drop..Drop$GT$4drop17he19d8d9c8ea92454E
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
    call $_ZN8dlmalloc3sys23enable_alloc_after_fork17h64eb1fc0ff7b2689E
    local.get 3
    i32.const 15
    i32.add
    call $_ZN73_$LT$dlmalloc..global..Instance$u20$as$u20$core..ops..deref..DerefMut$GT$9deref_mut17h16955ef502c5c4e5E
    local.get 0
    call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$4free17h501de2a6604ba1ffE
    local.get 3
    i32.const 15
    i32.add
    call $_ZN68_$LT$dlmalloc..global..Instance$u20$as$u20$core..ops..drop..Drop$GT$4drop17he19d8d9c8ea92454E
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
    call $_ZN8dlmalloc3sys23enable_alloc_after_fork17h64eb1fc0ff7b2689E
    local.get 4
    i32.const 15
    i32.add
    call $_ZN73_$LT$dlmalloc..global..Instance$u20$as$u20$core..ops..deref..DerefMut$GT$9deref_mut17h16955ef502c5c4e5E
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
                call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$8memalign17he8794c5d1cb954f9E
                local.tee 2
                br_if 1 (;@5;)
                i32.const 0
                local.set 2
                br 5 (;@1;)
              end
              call $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h86551c33e07de253E
              local.tee 1
              i32.const 8
              call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
              local.set 6
              i32.const 20
              i32.const 8
              call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
              local.set 7
              i32.const 16
              i32.const 8
              call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
              local.set 8
              i32.const 0
              local.set 2
              i32.const 0
              i32.const 16
              i32.const 8
              call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
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
              call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
              i32.const -5
              i32.add
              local.get 3
              i32.gt_u
              select
              i32.const 8
              call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
              local.set 6
              local.get 0
              call $_ZN8dlmalloc8dlmalloc5Chunk8from_mem17h11dd30c74f483706E
              local.set 1
              local.get 1
              local.get 1
              call $_ZN8dlmalloc8dlmalloc5Chunk4size17h77d1c406ab42db33E
              local.tee 7
              call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
              local.set 8
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            local.get 1
                            call $_ZN8dlmalloc8dlmalloc5Chunk7mmapped17h1a9959fbf47496c3E
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
                            call $_ZN8dlmalloc8dlmalloc5Chunk6cinuse17h58499de57c2d37e2E
                            br_if 9 (;@3;)
                            local.get 8
                            call $_ZN8dlmalloc8dlmalloc5Chunk4size17h77d1c406ab42db33E
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
                            call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18unlink_large_chunk17h2e279402ce6356d4E
                            br 2 (;@10;)
                          end
                          local.get 1
                          call $_ZN8dlmalloc8dlmalloc5Chunk4size17h77d1c406ab42db33E
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
                          call $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$9page_size17h0fdd55b2693d440cE
                          call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
                          local.tee 7
                          i32.const 1
                          call $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$5remap17hf5ff3c6a92680f40E
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
                          call $_ZN8dlmalloc8dlmalloc5Chunk14fencepost_head17he07aaa52f3b50dfdE
                          local.set 0
                          local.get 1
                          local.get 2
                          call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
                          local.get 0
                          i32.store offset=4
                          local.get 1
                          local.get 3
                          i32.const -12
                          i32.add
                          call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
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
                        call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 1
                        local.get 6
                        call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
                        local.set 7
                        local.get 1
                        local.get 6
                        call $_ZN8dlmalloc8dlmalloc5Chunk9set_inuse17h4282057414c4e601E
                        local.get 7
                        local.get 10
                        call $_ZN8dlmalloc8dlmalloc5Chunk9set_inuse17h4282057414c4e601E
                        local.get 5
                        local.get 7
                        local.get 10
                        call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$13dispose_chunk17h577eb103dc04307bE
                        local.get 1
                        br_if 8 (;@2;)
                        br 7 (;@3;)
                      end
                      local.get 1
                      local.get 7
                      call $_ZN8dlmalloc8dlmalloc5Chunk9set_inuse17h4282057414c4e601E
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
                        call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
                        i32.ge_u
                        br_if 0 (;@10;)
                        local.get 1
                        local.get 7
                        call $_ZN8dlmalloc8dlmalloc5Chunk9set_inuse17h4282057414c4e601E
                        i32.const 0
                        local.set 8
                        i32.const 0
                        local.set 7
                        br 1 (;@9;)
                      end
                      local.get 1
                      local.get 6
                      call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
                      local.tee 7
                      local.get 8
                      call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
                      local.set 9
                      local.get 1
                      local.get 6
                      call $_ZN8dlmalloc8dlmalloc5Chunk9set_inuse17h4282057414c4e601E
                      local.get 7
                      local.get 8
                      call $_ZN8dlmalloc8dlmalloc5Chunk33set_size_and_pinuse_of_free_chunk17ha971516d0be71949E
                      local.get 9
                      call $_ZN8dlmalloc8dlmalloc5Chunk12clear_pinuse17h3c1a99d0f5bddc22E
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
                  call $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 1
                  local.get 6
                  call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
                  local.set 8
                  local.get 1
                  local.get 6
                  call $_ZN8dlmalloc8dlmalloc5Chunk9set_inuse17h4282057414c4e601E
                  local.get 8
                  local.get 7
                  call $_ZN8dlmalloc8dlmalloc5Chunk9set_inuse17h4282057414c4e601E
                  local.get 5
                  local.get 8
                  local.get 7
                  call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$13dispose_chunk17h577eb103dc04307bE
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
            call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$4free17h501de2a6604ba1ffE
            br 3 (;@1;)
          end
          local.get 1
          local.get 6
          call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E
          local.set 8
          local.get 1
          local.get 6
          call $_ZN8dlmalloc8dlmalloc5Chunk9set_inuse17h4282057414c4e601E
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
        call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$6malloc17h1ae2390053e3628cE
        local.tee 6
        i32.eqz
        br_if 1 (;@1;)
        local.get 6
        local.get 0
        local.get 1
        call $_ZN8dlmalloc8dlmalloc5Chunk4size17h77d1c406ab42db33E
        i32.const -8
        i32.const -4
        local.get 1
        call $_ZN8dlmalloc8dlmalloc5Chunk7mmapped17h1a9959fbf47496c3E
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
        call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$4free17h501de2a6604ba1ffE
        local.get 3
        local.set 2
        br 1 (;@1;)
      end
      local.get 1
      call $_ZN8dlmalloc8dlmalloc5Chunk7mmapped17h1a9959fbf47496c3E
      drop
      local.get 1
      call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h75497733644e1d6cE
      local.set 2
    end
    local.get 4
    i32.const 15
    i32.add
    call $_ZN68_$LT$dlmalloc..global..Instance$u20$as$u20$core..ops..drop..Drop$GT$4drop17he19d8d9c8ea92454E
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
  (func $_ZN5alloc7raw_vec11finish_grow17hcefa6a06206fd52bE (;13;) (type 7) (param i32 i32 i32 i32)
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
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17h2205b68aee7ddaceE (;14;) (type 8) (param i32)
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
    call $_ZN5alloc7raw_vec11finish_grow17hcefa6a06206fd52bE
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
        call $_ZN5alloc5alloc18handle_alloc_error17h4f3cb0c5afb21c76E
        unreachable
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17h6c250c8ca346b5adE
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
    call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17h2205b68aee7ddaceE
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
  (func $_ZN5alloc7raw_vec17capacity_overflow17h6c250c8ca346b5adE (;16;) (type 9)
    unreachable
    unreachable
  )
  (func $_ZN5alloc5alloc18handle_alloc_error17h4f3cb0c5afb21c76E (;17;) (type 1) (param i32 i32)
    local.get 0
    local.get 1
    call $_ZN5alloc5alloc18handle_alloc_error8rt_error17h63de615f6e977af2E
    unreachable
  )
  (func $_ZN5alloc5alloc18handle_alloc_error8rt_error17h63de615f6e977af2E (;18;) (type 1) (param i32 i32)
    local.get 1
    local.get 0
    call $__rust_alloc_error_handler
    unreachable
  )
  (func $__rdl_oom (;19;) (type 1) (param i32 i32)
    unreachable
    unreachable
  )
  (func $_ZN8dlmalloc8dlmalloc8align_up17hacb462cafc347c13E (;20;) (type 3) (param i32 i32) (result i32)
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
  (func $_ZN8dlmalloc8dlmalloc9left_bits17hb6cbe146b8019d98E (;21;) (type 2) (param i32) (result i32)
    local.get 0
    i32.const 1
    i32.shl
    local.tee 0
    i32.const 0
    local.get 0
    i32.sub
    i32.or
  )
  (func $_ZN8dlmalloc8dlmalloc9least_bit17h4bca52ead665dc5aE (;22;) (type 2) (param i32) (result i32)
    i32.const 0
    local.get 0
    i32.sub
    local.get 0
    i32.and
  )
  (func $_ZN8dlmalloc8dlmalloc24leftshift_for_tree_index17h31d064fdd867f502E (;23;) (type 2) (param i32) (result i32)
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
  (func $_ZN8dlmalloc8dlmalloc5Chunk14fencepost_head17he07aaa52f3b50dfdE (;24;) (type 5) (result i32)
    i32.const 7
  )
  (func $_ZN8dlmalloc8dlmalloc5Chunk4size17h77d1c406ab42db33E (;25;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load offset=4
    i32.const -8
    i32.and
  )
  (func $_ZN8dlmalloc8dlmalloc5Chunk6cinuse17h58499de57c2d37e2E (;26;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load8_u offset=4
    i32.const 2
    i32.and
    i32.const 1
    i32.shr_u
  )
  (func $_ZN8dlmalloc8dlmalloc5Chunk6pinuse17h92d5107047b03ba7E (;27;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load offset=4
    i32.const 1
    i32.and
  )
  (func $_ZN8dlmalloc8dlmalloc5Chunk12clear_pinuse17h3c1a99d0f5bddc22E (;28;) (type 8) (param i32)
    local.get 0
    local.get 0
    i32.load offset=4
    i32.const -2
    i32.and
    i32.store offset=4
  )
  (func $_ZN8dlmalloc8dlmalloc5Chunk5inuse17h2d327e4c36b84dfeE (;29;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load offset=4
    i32.const 3
    i32.and
    i32.const 1
    i32.ne
  )
  (func $_ZN8dlmalloc8dlmalloc5Chunk7mmapped17h1a9959fbf47496c3E (;30;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load8_u offset=4
    i32.const 3
    i32.and
    i32.eqz
  )
  (func $_ZN8dlmalloc8dlmalloc5Chunk9set_inuse17h4282057414c4e601E (;31;) (type 1) (param i32 i32)
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
  (func $_ZN8dlmalloc8dlmalloc5Chunk20set_inuse_and_pinuse17h515e5a69a6d1edc6E (;32;) (type 1) (param i32 i32)
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
  (func $_ZN8dlmalloc8dlmalloc5Chunk34set_size_and_pinuse_of_inuse_chunk17h4acf6d59020bd397E (;33;) (type 1) (param i32 i32)
    local.get 0
    local.get 1
    i32.const 3
    i32.or
    i32.store offset=4
  )
  (func $_ZN8dlmalloc8dlmalloc5Chunk33set_size_and_pinuse_of_free_chunk17ha971516d0be71949E (;34;) (type 1) (param i32 i32)
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
  (func $_ZN8dlmalloc8dlmalloc5Chunk20set_free_with_pinuse17h5d876ea751634e99E (;35;) (type 0) (param i32 i32 i32)
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
  (func $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h6e6d06559ad34b15E (;36;) (type 3) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.add
  )
  (func $_ZN8dlmalloc8dlmalloc5Chunk12minus_offset17h7c3eec81761249d9E (;37;) (type 3) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.sub
  )
  (func $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h75497733644e1d6cE (;38;) (type 2) (param i32) (result i32)
    local.get 0
    i32.const 8
    i32.add
  )
  (func $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h86551c33e07de253E (;39;) (type 5) (result i32)
    i32.const 8
  )
  (func $_ZN8dlmalloc8dlmalloc5Chunk8from_mem17h11dd30c74f483706E (;40;) (type 2) (param i32) (result i32)
    local.get 0
    i32.const -8
    i32.add
  )
  (func $_ZN8dlmalloc8dlmalloc9TreeChunk14leftmost_child17h20605933c801b44bE (;41;) (type 2) (param i32) (result i32)
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
  (func $_ZN8dlmalloc8dlmalloc9TreeChunk5chunk17h4efd58110bb4b6e5E (;42;) (type 2) (param i32) (result i32)
    local.get 0
  )
  (func $_ZN8dlmalloc8dlmalloc9TreeChunk4next17he250edbec5d87123E (;43;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load offset=12
  )
  (func $_ZN8dlmalloc8dlmalloc9TreeChunk4prev17h7a0f1d46544cc14aE (;44;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load offset=8
  )
  (func $_ZN8dlmalloc8dlmalloc7Segment9is_extern17h6f6db2c70b891fd9E (;45;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load offset=12
    i32.const 1
    i32.and
  )
  (func $_ZN8dlmalloc8dlmalloc7Segment9sys_flags17h224550055bf7775bE (;46;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load offset=12
    i32.const 1
    i32.shr_u
  )
  (func $_ZN8dlmalloc8dlmalloc7Segment5holds17h8f6de4ee6718009bE (;47;) (type 3) (param i32 i32) (result i32)
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
  (func $_ZN8dlmalloc8dlmalloc7Segment3top17he7e9e2493151d036E (;48;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    i32.add
  )
  (func $_ZN73_$LT$dlmalloc..global..Instance$u20$as$u20$core..ops..deref..DerefMut$GT$9deref_mut17h16955ef502c5c4e5E (;49;) (type 2) (param i32) (result i32)
    i32.const 1048580
  )
  (func $_ZN68_$LT$dlmalloc..global..Instance$u20$as$u20$core..ops..drop..Drop$GT$4drop17he19d8d9c8ea92454E (;50;) (type 8) (param i32))
  (func $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$5alloc17hdbf1e2bcc01bc909E (;51;) (type 0) (param i32 i32 i32)
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
  (func $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$5remap17hf5ff3c6a92680f40E (;52;) (type 10) (param i32 i32 i32 i32 i32) (result i32)
    i32.const 0
  )
  (func $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$9free_part17h74489c9e7a3aa967E (;53;) (type 6) (param i32 i32 i32 i32) (result i32)
    i32.const 0
  )
  (func $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$4free17h993c5f05ba1214bcE (;54;) (type 4) (param i32 i32 i32) (result i32)
    i32.const 0
  )
  (func $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$16can_release_part17h43bfb7d8666fcc31E (;55;) (type 3) (param i32 i32) (result i32)
    i32.const 0
  )
  (func $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$9page_size17h0fdd55b2693d440cE (;56;) (type 2) (param i32) (result i32)
    i32.const 65536
  )
  (func $_ZN8dlmalloc3sys23enable_alloc_after_fork17h64eb1fc0ff7b2689E (;57;) (type 9))
  (func $memcpy (;58;) (type 4) (param i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    call $_ZN17compiler_builtins3mem6memcpy17h7b83c85e899060b3E
  )
  (func $_ZN17compiler_builtins3mem6memcpy17h7b83c85e899060b3E (;59;) (type 4) (param i32 i32 i32) (result i32)
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