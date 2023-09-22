(module
  (type (;0;) (func (param i32 i32 i32)))
  (type (;1;) (func (param i32 i32)))
  (type (;2;) (func (param i32) (result i32)))
  (type (;3;) (func (param i32 i32) (result i32)))
  (type (;4;) (func (param i32 i32 i32) (result i32)))
  (type (;5;) (func (param i32)))
  (type (;6;) (func (result i32)))
  (type (;7;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;8;) (func))
  (type (;9;) (func (param i32 i32 i32 i32 i32) (result i32)))
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$13dispose_chunk17h1e388daf1185cdd1E (;0;) (type 0) (param i32 i32 i32)
    (local i32 i32 i32 i32)
    local.get 1
    local.get 2
    call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
    local.set 3
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          call $_ZN8dlmalloc8dlmalloc5Chunk6pinuse17hd290f2ba612e4cd6E
          br_if 0 (;@3;)
          local.get 1
          i32.load
          local.set 4
          block ;; label = @4
            local.get 1
            call $_ZN8dlmalloc8dlmalloc5Chunk7mmapped17h3cc0fa31d2a7a730E
            br_if 0 (;@4;)
            local.get 4
            local.get 2
            i32.add
            local.set 2
            block ;; label = @5
              local.get 1
              local.get 4
              call $_ZN8dlmalloc8dlmalloc5Chunk12minus_offset17hc8694da7b578c86dE
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
              call $_ZN8dlmalloc8dlmalloc5Chunk20set_free_with_pinuse17h7e233c8b10f845e1E
              return
            end
            block ;; label = @5
              local.get 4
              i32.const 256
              i32.lt_u
              br_if 0 (;@5;)
              local.get 0
              local.get 1
              call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18unlink_large_chunk17hd11f7eca1b1c75b8E
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
          call $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$4free17h35a14ba6044af2d8E
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
          call $_ZN8dlmalloc8dlmalloc5Chunk6cinuse17hbaa15675ecfdf704E
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          local.get 2
          local.get 3
          call $_ZN8dlmalloc8dlmalloc5Chunk20set_free_with_pinuse17h7e233c8b10f845e1E
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
            call $_ZN8dlmalloc8dlmalloc5Chunk4size17ha05d6c4faa787634E
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
                call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18unlink_large_chunk17hd11f7eca1b1c75b8E
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
            call $_ZN8dlmalloc8dlmalloc5Chunk33set_size_and_pinuse_of_free_chunk17h683dbff7fd1b8b1aE
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
        call $_ZN8dlmalloc8dlmalloc5Chunk33set_size_and_pinuse_of_free_chunk17h683dbff7fd1b8b1aE
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
      call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18insert_large_chunk17ha38a2d2b2d7c9e1cE
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
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18unlink_large_chunk17hd11f7eca1b1c75b8E (;1;) (type 1) (param i32 i32)
    (local i32 i32 i32 i32 i32)
    local.get 1
    i32.load offset=24
    local.set 2
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          call $_ZN8dlmalloc8dlmalloc9TreeChunk4next17h7d15145567f19ebbE
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
        call $_ZN8dlmalloc8dlmalloc9TreeChunk4prev17hcf95d98862e98be6E
        local.tee 5
        local.get 1
        call $_ZN8dlmalloc8dlmalloc9TreeChunk4next17h7d15145567f19ebbE
        local.tee 3
        call $_ZN8dlmalloc8dlmalloc9TreeChunk5chunk17h337b3f398402a12dE
        i32.store offset=12
        local.get 3
        local.get 5
        call $_ZN8dlmalloc8dlmalloc9TreeChunk5chunk17h337b3f398402a12dE
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
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18insert_large_chunk17ha38a2d2b2d7c9e1cE (;2;) (type 0) (param i32 i32 i32)
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
    call $_ZN8dlmalloc8dlmalloc9TreeChunk5chunk17h337b3f398402a12dE
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
        call $_ZN8dlmalloc8dlmalloc24leftshift_for_tree_index17h59b2e6a96fd4d5a4E
        i32.shl
        local.set 3
        loop ;; label = @3
          block ;; label = @4
            local.get 0
            local.tee 4
            call $_ZN8dlmalloc8dlmalloc9TreeChunk5chunk17h337b3f398402a12dE
            call $_ZN8dlmalloc8dlmalloc5Chunk4size17ha05d6c4faa787634E
            local.get 2
            i32.ne
            br_if 0 (;@4;)
            local.get 4
            call $_ZN8dlmalloc8dlmalloc9TreeChunk5chunk17h337b3f398402a12dE
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
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$23release_unused_segments17hfe2f7b40ba6d4449E (;3;) (type 2) (param i32) (result i32)
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
            call $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$16can_release_part17hecdc06996915370fE
            i32.eqz
            br_if 0 (;@4;)
            local.get 5
            call $_ZN8dlmalloc8dlmalloc7Segment9is_extern17h0810f1513c6e5931E
            br_if 0 (;@4;)
            local.get 7
            local.get 7
            call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h1d583dda34ec8e6fE
            local.tee 8
            i32.const 8
            call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
            local.get 8
            i32.sub
            i32.add
            local.tee 8
            call $_ZN8dlmalloc8dlmalloc5Chunk4size17ha05d6c4faa787634E
            local.set 9
            call $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h2627fcfde9fd9cb1E
            local.tee 10
            i32.const 8
            call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
            local.set 11
            i32.const 20
            i32.const 8
            call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
            local.set 12
            i32.const 16
            i32.const 8
            call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
            local.set 13
            local.get 8
            call $_ZN8dlmalloc8dlmalloc5Chunk5inuse17h39e6dae44e7f41daE
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
                call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18unlink_large_chunk17hd11f7eca1b1c75b8E
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
              call $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$4free17h35a14ba6044af2d8E
              br_if 0 (;@5;)
              local.get 0
              local.get 8
              local.get 9
              call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18insert_large_chunk17ha38a2d2b2d7c9e1cE
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
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$4free17h7ea880c3526c5524E (;4;) (type 1) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    local.get 1
    call $_ZN8dlmalloc8dlmalloc5Chunk8from_mem17hc2f0084eaea44926E
    local.set 1
    local.get 1
    local.get 1
    call $_ZN8dlmalloc8dlmalloc5Chunk4size17ha05d6c4faa787634E
    local.tee 2
    call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
    local.set 3
    block ;; label = @1
      block ;; label = @2
        local.get 1
        call $_ZN8dlmalloc8dlmalloc5Chunk6pinuse17hd290f2ba612e4cd6E
        br_if 0 (;@2;)
        local.get 1
        i32.load
        local.set 4
        block ;; label = @3
          local.get 1
          call $_ZN8dlmalloc8dlmalloc5Chunk7mmapped17h3cc0fa31d2a7a730E
          br_if 0 (;@3;)
          local.get 4
          local.get 2
          i32.add
          local.set 2
          block ;; label = @4
            local.get 1
            local.get 4
            call $_ZN8dlmalloc8dlmalloc5Chunk12minus_offset17hc8694da7b578c86dE
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
            call $_ZN8dlmalloc8dlmalloc5Chunk20set_free_with_pinuse17h7e233c8b10f845e1E
            return
          end
          block ;; label = @4
            local.get 4
            i32.const 256
            i32.lt_u
            br_if 0 (;@4;)
            local.get 0
            local.get 1
            call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18unlink_large_chunk17hd11f7eca1b1c75b8E
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
        call $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$4free17h35a14ba6044af2d8E
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
          call $_ZN8dlmalloc8dlmalloc5Chunk6cinuse17hbaa15675ecfdf704E
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          local.get 2
          local.get 3
          call $_ZN8dlmalloc8dlmalloc5Chunk20set_free_with_pinuse17h7e233c8b10f845e1E
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
                call $_ZN8dlmalloc8dlmalloc5Chunk4size17ha05d6c4faa787634E
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
                    call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18unlink_large_chunk17hd11f7eca1b1c75b8E
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
                call $_ZN8dlmalloc8dlmalloc5Chunk33set_size_and_pinuse_of_free_chunk17h683dbff7fd1b8b1aE
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
            call $_ZN8dlmalloc8dlmalloc5Chunk33set_size_and_pinuse_of_free_chunk17h683dbff7fd1b8b1aE
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
        call $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h2627fcfde9fd9cb1E
        local.tee 1
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
        local.set 2
        i32.const 20
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
        local.set 3
        i32.const 16
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
        local.set 4
        i32.const 0
        i32.const 16
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
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
        call $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h2627fcfde9fd9cb1E
        local.tee 1
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
        local.set 3
        i32.const 20
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
        local.set 5
        i32.const 16
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
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
                call $_ZN8dlmalloc8dlmalloc7Segment3top17hc9cd36f864405361E
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
          call $_ZN8dlmalloc8dlmalloc7Segment9is_extern17h0810f1513c6e5931E
          br_if 0 (;@3;)
          i32.const 0
          local.set 4
          local.get 0
          local.get 1
          i32.load offset=12
          i32.const 1
          i32.shr_u
          call $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$16can_release_part17hecdc06996915370fE
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
              call $_ZN8dlmalloc8dlmalloc7Segment5holds17h4192d12beb9dcf77E
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
          call $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$9free_part17hf77ea82ca2f1faf0E
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
          call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h1d583dda34ec8e6fE
          local.tee 3
          i32.const 8
          call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
          local.get 3
          i32.sub
          local.tee 3
          call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
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
          call $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h2627fcfde9fd9cb1E
          local.tee 3
          i32.const 8
          call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
          local.set 4
          i32.const 20
          i32.const 8
          call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
          local.set 6
          i32.const 16
          i32.const 8
          call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
          local.set 7
          local.get 1
          local.get 2
          call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
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
        call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$23release_unused_segments17hfe2f7b40ba6d4449E
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
        call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18insert_large_chunk17ha38a2d2b2d7c9e1cE
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
        call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$23release_unused_segments17hfe2f7b40ba6d4449E
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
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$6malloc17hf43138a3d10fc105E (;5;) (type 3) (param i32 i32) (result i32)
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
                call $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h2627fcfde9fd9cb1E
                local.tee 3
                i32.const 8
                call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
                local.set 4
                i32.const 20
                i32.const 8
                call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
                local.set 5
                i32.const 16
                i32.const 8
                call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
                local.set 6
                i32.const 0
                local.set 7
                i32.const 0
                i32.const 16
                i32.const 8
                call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
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
                call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
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
                call $_ZN8dlmalloc8dlmalloc24leftshift_for_tree_index17h59b2e6a96fd4d5a4E
                i32.shl
                local.set 6
                i32.const 0
                local.set 1
                i32.const 0
                local.set 5
                loop ;; label = @7
                  block ;; label = @8
                    local.get 7
                    call $_ZN8dlmalloc8dlmalloc9TreeChunk5chunk17h337b3f398402a12dE
                    call $_ZN8dlmalloc8dlmalloc5Chunk4size17ha05d6c4faa787634E
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
              call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
              i32.const -5
              i32.add
              local.get 1
              i32.gt_u
              select
              i32.const 8
              call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
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
                call $_ZN8dlmalloc8dlmalloc5Chunk20set_inuse_and_pinuse17hdbccaa0197e7f920E
                local.get 1
                call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h1d583dda34ec8e6fE
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
                            call $_ZN8dlmalloc8dlmalloc9least_bit17h7307b3bedf4731dcE
                            i32.ctz
                            i32.const 2
                            i32.shl
                            i32.add
                            i32.load
                            local.tee 7
                            call $_ZN8dlmalloc8dlmalloc9TreeChunk5chunk17h337b3f398402a12dE
                            call $_ZN8dlmalloc8dlmalloc5Chunk4size17ha05d6c4faa787634E
                            local.get 3
                            i32.sub
                            local.set 4
                            block ;; label = @13
                              local.get 7
                              call $_ZN8dlmalloc8dlmalloc9TreeChunk14leftmost_child17hf6c05ef2deebc535E
                              local.tee 1
                              i32.eqz
                              br_if 0 (;@13;)
                              loop ;; label = @14
                                local.get 1
                                call $_ZN8dlmalloc8dlmalloc9TreeChunk5chunk17h337b3f398402a12dE
                                call $_ZN8dlmalloc8dlmalloc5Chunk4size17ha05d6c4faa787634E
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
                                call $_ZN8dlmalloc8dlmalloc9TreeChunk14leftmost_child17hf6c05ef2deebc535E
                                local.tee 1
                                br_if 0 (;@14;)
                              end
                            end
                            local.get 7
                            call $_ZN8dlmalloc8dlmalloc9TreeChunk5chunk17h337b3f398402a12dE
                            local.tee 1
                            local.get 3
                            call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
                            local.set 5
                            local.get 0
                            local.get 7
                            call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18unlink_large_chunk17hd11f7eca1b1c75b8E
                            local.get 4
                            i32.const 16
                            i32.const 8
                            call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
                            i32.lt_u
                            br_if 2 (;@10;)
                            local.get 5
                            call $_ZN8dlmalloc8dlmalloc9TreeChunk5chunk17h337b3f398402a12dE
                            local.set 5
                            local.get 1
                            local.get 3
                            call $_ZN8dlmalloc8dlmalloc5Chunk34set_size_and_pinuse_of_inuse_chunk17hf8c39f7e3e10a4ffE
                            local.get 5
                            local.get 4
                            call $_ZN8dlmalloc8dlmalloc5Chunk33set_size_and_pinuse_of_free_chunk17h683dbff7fd1b8b1aE
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
                              call $_ZN8dlmalloc8dlmalloc9left_bits17h955e87685b8ece02E
                              local.get 1
                              local.get 4
                              i32.shl
                              i32.and
                              call $_ZN8dlmalloc8dlmalloc9least_bit17h7307b3bedf4731dcE
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
                          call $_ZN8dlmalloc8dlmalloc5Chunk34set_size_and_pinuse_of_inuse_chunk17hf8c39f7e3e10a4ffE
                          local.get 1
                          local.get 3
                          call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
                          local.tee 5
                          local.get 7
                          i32.const 3
                          i32.shl
                          local.get 3
                          i32.sub
                          local.tee 6
                          call $_ZN8dlmalloc8dlmalloc5Chunk33set_size_and_pinuse_of_free_chunk17h683dbff7fd1b8b1aE
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
                      call $_ZN8dlmalloc8dlmalloc5Chunk20set_inuse_and_pinuse17hdbccaa0197e7f920E
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
                  call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h1d583dda34ec8e6fE
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
              call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h1d583dda34ec8e6fE
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
              call $_ZN8dlmalloc8dlmalloc9left_bits17h955e87685b8ece02E
              local.get 9
              i32.and
              local.tee 1
              i32.eqz
              br_if 3 (;@2;)
              local.get 0
              local.get 1
              call $_ZN8dlmalloc8dlmalloc9least_bit17h7307b3bedf4731dcE
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
            call $_ZN8dlmalloc8dlmalloc9TreeChunk5chunk17h337b3f398402a12dE
            call $_ZN8dlmalloc8dlmalloc5Chunk4size17ha05d6c4faa787634E
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
            call $_ZN8dlmalloc8dlmalloc9TreeChunk14leftmost_child17hf6c05ef2deebc535E
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
        call $_ZN8dlmalloc8dlmalloc9TreeChunk5chunk17h337b3f398402a12dE
        local.tee 1
        local.get 3
        call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
        local.set 7
        local.get 0
        local.get 5
        call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18unlink_large_chunk17hd11f7eca1b1c75b8E
        block ;; label = @3
          block ;; label = @4
            local.get 4
            i32.const 16
            i32.const 8
            call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
            i32.lt_u
            br_if 0 (;@4;)
            local.get 1
            local.get 3
            call $_ZN8dlmalloc8dlmalloc5Chunk34set_size_and_pinuse_of_inuse_chunk17hf8c39f7e3e10a4ffE
            local.get 7
            local.get 4
            call $_ZN8dlmalloc8dlmalloc5Chunk33set_size_and_pinuse_of_free_chunk17h683dbff7fd1b8b1aE
            block ;; label = @5
              local.get 4
              i32.const 256
              i32.lt_u
              br_if 0 (;@5;)
              local.get 0
              local.get 7
              local.get 4
              call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18insert_large_chunk17ha38a2d2b2d7c9e1cE
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
          call $_ZN8dlmalloc8dlmalloc5Chunk20set_inuse_and_pinuse17hdbccaa0197e7f920E
        end
        local.get 1
        call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h1d583dda34ec8e6fE
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
                        call $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h2627fcfde9fd9cb1E
                        local.tee 1
                        i32.sub
                        local.get 1
                        i32.const 8
                        call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
                        i32.add
                        i32.const 20
                        i32.const 8
                        call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
                        i32.add
                        i32.const 16
                        i32.const 8
                        call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
                        i32.add
                        i32.const 8
                        i32.add
                        i32.const 65536
                        call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
                        call $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$5alloc17h16d966d43fa28c14E
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
                              call $_ZN8dlmalloc8dlmalloc7Segment3top17hc9cd36f864405361E
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
                          call $_ZN8dlmalloc8dlmalloc7Segment9is_extern17h0810f1513c6e5931E
                          br_if 0 (;@11;)
                          local.get 1
                          call $_ZN8dlmalloc8dlmalloc7Segment9sys_flags17hc63bc32fe8553ea7E
                          local.get 11
                          i32.ne
                          br_if 0 (;@11;)
                          local.get 1
                          local.get 0
                          i32.load offset=428
                          call $_ZN8dlmalloc8dlmalloc7Segment5holds17h4192d12beb9dcf77E
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
                            call $_ZN8dlmalloc8dlmalloc7Segment9is_extern17h0810f1513c6e5931E
                            br_if 0 (;@12;)
                            local.get 1
                            call $_ZN8dlmalloc8dlmalloc7Segment9sys_flags17hc63bc32fe8553ea7E
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
                                call $_ZN8dlmalloc8dlmalloc7Segment3top17hc9cd36f864405361E
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
                          call $_ZN8dlmalloc8dlmalloc7Segment3top17hc9cd36f864405361E
                          local.tee 6
                          i32.const 20
                          i32.const 8
                          call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
                          local.tee 12
                          i32.sub
                          i32.const -23
                          i32.add
                          local.set 1
                          local.get 5
                          local.get 1
                          local.get 1
                          call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h1d583dda34ec8e6fE
                          local.tee 4
                          i32.const 8
                          call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
                          local.get 4
                          i32.sub
                          i32.add
                          local.tee 1
                          local.get 1
                          local.get 5
                          i32.const 16
                          i32.const 8
                          call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
                          i32.add
                          i32.lt_u
                          select
                          local.tee 13
                          call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h1d583dda34ec8e6fE
                          local.set 4
                          local.get 13
                          local.get 12
                          call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
                          local.set 1
                          call $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h2627fcfde9fd9cb1E
                          local.tee 14
                          i32.const 8
                          call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
                          local.set 15
                          i32.const 20
                          i32.const 8
                          call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
                          local.set 16
                          i32.const 16
                          i32.const 8
                          call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
                          local.set 17
                          local.get 0
                          local.get 8
                          local.get 8
                          call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h1d583dda34ec8e6fE
                          local.tee 18
                          i32.const 8
                          call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
                          local.get 18
                          i32.sub
                          local.tee 19
                          call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
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
                          call $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h2627fcfde9fd9cb1E
                          local.tee 15
                          i32.const 8
                          call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
                          local.set 16
                          i32.const 20
                          i32.const 8
                          call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
                          local.set 17
                          i32.const 16
                          i32.const 8
                          call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
                          local.set 19
                          local.get 18
                          local.get 14
                          call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
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
                          call $_ZN8dlmalloc8dlmalloc5Chunk34set_size_and_pinuse_of_inuse_chunk17hf8c39f7e3e10a4ffE
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
                            call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
                            local.set 4
                            local.get 1
                            call $_ZN8dlmalloc8dlmalloc5Chunk14fencepost_head17h06a0db07d0fb90cdE
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
                          call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
                          call $_ZN8dlmalloc8dlmalloc5Chunk20set_free_with_pinuse17h7e233c8b10f845e1E
                          block ;; label = @12
                            local.get 1
                            i32.const 256
                            i32.lt_u
                            br_if 0 (;@12;)
                            local.get 0
                            local.get 5
                            local.get 1
                            call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18insert_large_chunk17ha38a2d2b2d7c9e1cE
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
                        call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h1d583dda34ec8e6fE
                        local.tee 1
                        i32.const 8
                        call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
                        local.set 4
                        local.get 5
                        call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h1d583dda34ec8e6fE
                        local.tee 6
                        i32.const 8
                        call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
                        local.set 10
                        local.get 8
                        local.get 4
                        local.get 1
                        i32.sub
                        i32.add
                        local.tee 4
                        local.get 3
                        call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
                        local.set 7
                        local.get 4
                        local.get 3
                        call $_ZN8dlmalloc8dlmalloc5Chunk34set_size_and_pinuse_of_inuse_chunk17hf8c39f7e3e10a4ffE
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
                          call $_ZN8dlmalloc8dlmalloc5Chunk5inuse17h39e6dae44e7f41daE
                          br_if 7 (;@4;)
                          block ;; label = @12
                            block ;; label = @13
                              local.get 1
                              call $_ZN8dlmalloc8dlmalloc5Chunk4size17ha05d6c4faa787634E
                              local.tee 5
                              i32.const 256
                              i32.lt_u
                              br_if 0 (;@13;)
                              local.get 0
                              local.get 1
                              call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18unlink_large_chunk17hd11f7eca1b1c75b8E
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
                          call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
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
                        call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h1d583dda34ec8e6fE
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
                      call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
                      local.tee 7
                      i32.store offset=428
                      local.get 7
                      local.get 4
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 1
                      local.get 3
                      call $_ZN8dlmalloc8dlmalloc5Chunk34set_size_and_pinuse_of_inuse_chunk17hf8c39f7e3e10a4ffE
                      local.get 1
                      call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h1d583dda34ec8e6fE
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
                    call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
                    i32.lt_u
                    br_if 3 (;@5;)
                    local.get 1
                    local.get 3
                    call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
                    local.set 7
                    local.get 0
                    local.get 4
                    i32.store offset=416
                    local.get 0
                    local.get 7
                    i32.store offset=424
                    local.get 7
                    local.get 4
                    call $_ZN8dlmalloc8dlmalloc5Chunk33set_size_and_pinuse_of_free_chunk17h683dbff7fd1b8b1aE
                    local.get 1
                    local.get 3
                    call $_ZN8dlmalloc8dlmalloc5Chunk34set_size_and_pinuse_of_inuse_chunk17hf8c39f7e3e10a4ffE
                    local.get 1
                    call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h1d583dda34ec8e6fE
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
                call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$8init_top17h72a65f462357eca1E
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
              call $_ZN8dlmalloc8dlmalloc5Chunk33set_size_and_pinuse_of_free_chunk17h683dbff7fd1b8b1aE
              local.get 4
              call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h1d583dda34ec8e6fE
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
            call $_ZN8dlmalloc8dlmalloc5Chunk20set_inuse_and_pinuse17hdbccaa0197e7f920E
            local.get 1
            call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h1d583dda34ec8e6fE
            local.set 7
            br 3 (;@1;)
          end
          local.get 7
          local.get 3
          local.get 1
          call $_ZN8dlmalloc8dlmalloc5Chunk20set_free_with_pinuse17h7e233c8b10f845e1E
          block ;; label = @4
            local.get 3
            i32.const 256
            i32.lt_u
            br_if 0 (;@4;)
            local.get 0
            local.get 7
            local.get 3
            call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18insert_large_chunk17ha38a2d2b2d7c9e1cE
            local.get 4
            call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h1d583dda34ec8e6fE
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
          call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h1d583dda34ec8e6fE
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
        call $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h2627fcfde9fd9cb1E
        local.tee 4
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
        local.set 5
        i32.const 20
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
        local.set 6
        i32.const 16
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
        local.set 9
        local.get 0
        local.get 8
        local.get 8
        call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h1d583dda34ec8e6fE
        local.tee 1
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
        local.get 1
        i32.sub
        local.tee 13
        call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
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
        call $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h2627fcfde9fd9cb1E
        local.tee 5
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
        local.set 6
        i32.const 20
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
        local.set 8
        i32.const 16
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
        local.set 10
        local.get 1
        local.get 4
        call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
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
      call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
      local.tee 7
      i32.store offset=428
      local.get 7
      local.get 4
      i32.const 1
      i32.or
      i32.store offset=4
      local.get 1
      local.get 3
      call $_ZN8dlmalloc8dlmalloc5Chunk34set_size_and_pinuse_of_inuse_chunk17hf8c39f7e3e10a4ffE
      local.get 1
      call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h1d583dda34ec8e6fE
      local.set 7
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 7
  )
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$8init_top17h72a65f462357eca1E (;6;) (type 0) (param i32 i32 i32)
    (local i32 i32 i32 i32)
    local.get 1
    local.get 1
    call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h1d583dda34ec8e6fE
    local.tee 3
    i32.const 8
    call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
    local.get 3
    i32.sub
    local.tee 3
    call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
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
    call $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h2627fcfde9fd9cb1E
    local.tee 3
    i32.const 8
    call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
    local.set 4
    i32.const 20
    i32.const 8
    call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
    local.set 5
    i32.const 16
    i32.const 8
    call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
    local.set 6
    local.get 1
    local.get 2
    call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
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
  (func $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$8memalign17hdb281e652fdf832bE (;7;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      i32.const 16
      i32.const 8
      call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
      local.get 1
      i32.le_u
      br_if 0 (;@1;)
      i32.const 16
      i32.const 8
      call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
      local.set 1
    end
    call $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h2627fcfde9fd9cb1E
    local.tee 3
    i32.const 8
    call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
    local.set 4
    i32.const 20
    i32.const 8
    call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
    local.set 5
    i32.const 16
    i32.const 8
    call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
    local.set 6
    i32.const 0
    local.set 7
    block ;; label = @1
      i32.const 0
      i32.const 16
      i32.const 8
      call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
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
      call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
      i32.const -5
      i32.add
      local.get 2
      i32.gt_u
      select
      i32.const 8
      call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
      local.tee 4
      i32.add
      i32.const 16
      i32.const 8
      call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
      i32.add
      i32.const -4
      i32.add
      call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$6malloc17hf43138a3d10fc105E
      local.tee 3
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      call $_ZN8dlmalloc8dlmalloc5Chunk8from_mem17hc2f0084eaea44926E
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
        call $_ZN8dlmalloc8dlmalloc5Chunk8from_mem17hc2f0084eaea44926E
        local.set 7
        i32.const 16
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
        local.set 3
        local.get 2
        call $_ZN8dlmalloc8dlmalloc5Chunk4size17ha05d6c4faa787634E
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
          call $_ZN8dlmalloc8dlmalloc5Chunk7mmapped17h3cc0fa31d2a7a730E
          br_if 0 (;@3;)
          local.get 1
          local.get 3
          call $_ZN8dlmalloc8dlmalloc5Chunk9set_inuse17ha6e868917121b612E
          local.get 2
          local.get 7
          call $_ZN8dlmalloc8dlmalloc5Chunk9set_inuse17ha6e868917121b612E
          local.get 0
          local.get 2
          local.get 7
          call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$13dispose_chunk17h1e388daf1185cdd1E
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
        call $_ZN8dlmalloc8dlmalloc5Chunk7mmapped17h3cc0fa31d2a7a730E
        br_if 0 (;@2;)
        local.get 1
        call $_ZN8dlmalloc8dlmalloc5Chunk4size17ha05d6c4faa787634E
        local.tee 2
        i32.const 16
        i32.const 8
        call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
        local.get 4
        i32.add
        i32.le_u
        br_if 0 (;@2;)
        local.get 1
        local.get 4
        call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
        local.set 7
        local.get 1
        local.get 4
        call $_ZN8dlmalloc8dlmalloc5Chunk9set_inuse17ha6e868917121b612E
        local.get 7
        local.get 2
        local.get 4
        i32.sub
        local.tee 2
        call $_ZN8dlmalloc8dlmalloc5Chunk9set_inuse17ha6e868917121b612E
        local.get 0
        local.get 7
        local.get 2
        call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$13dispose_chunk17h1e388daf1185cdd1E
      end
      local.get 1
      call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h1d583dda34ec8e6fE
      local.set 7
      local.get 1
      call $_ZN8dlmalloc8dlmalloc5Chunk7mmapped17h3cc0fa31d2a7a730E
      drop
    end
    local.get 7
  )
  (func $rust_begin_unwind (;8;) (type 5) (param i32)
    loop ;; label = @1
      br 0 (;@1;)
    end
  )
  (func $__main (;9;) (type 6) (result i32)
    call $vec_alloc
    i32.const 0
  )
  (func $__rust_alloc (;10;) (type 3) (param i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    call $_ZN8dlmalloc3sys23enable_alloc_after_fork17h32651baadeb17bebE
    local.get 2
    i32.const 15
    i32.add
    call $_ZN73_$LT$dlmalloc..global..Instance$u20$as$u20$core..ops..deref..DerefMut$GT$9deref_mut17hd7579f2c72609989E
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
        call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$8memalign17hdb281e652fdf832bE
        local.set 1
        br 1 (;@1;)
      end
      local.get 3
      local.get 0
      call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$6malloc17hf43138a3d10fc105E
      local.set 1
    end
    local.get 2
    i32.const 15
    i32.add
    call $_ZN68_$LT$dlmalloc..global..Instance$u20$as$u20$core..ops..drop..Drop$GT$4drop17h3f91ec35f37562f3E
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $__rust_dealloc (;11;) (type 0) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    call $_ZN8dlmalloc3sys23enable_alloc_after_fork17h32651baadeb17bebE
    local.get 3
    i32.const 15
    i32.add
    call $_ZN73_$LT$dlmalloc..global..Instance$u20$as$u20$core..ops..deref..DerefMut$GT$9deref_mut17hd7579f2c72609989E
    local.get 0
    call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$4free17h7ea880c3526c5524E
    local.get 3
    i32.const 15
    i32.add
    call $_ZN68_$LT$dlmalloc..global..Instance$u20$as$u20$core..ops..drop..Drop$GT$4drop17h3f91ec35f37562f3E
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $__rust_realloc (;12;) (type 7) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    call $_ZN8dlmalloc3sys23enable_alloc_after_fork17h32651baadeb17bebE
    local.get 4
    i32.const 15
    i32.add
    call $_ZN73_$LT$dlmalloc..global..Instance$u20$as$u20$core..ops..deref..DerefMut$GT$9deref_mut17hd7579f2c72609989E
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
                call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$8memalign17hdb281e652fdf832bE
                local.tee 2
                br_if 1 (;@5;)
                i32.const 0
                local.set 2
                br 5 (;@1;)
              end
              call $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h2627fcfde9fd9cb1E
              local.tee 1
              i32.const 8
              call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
              local.set 6
              i32.const 20
              i32.const 8
              call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
              local.set 7
              i32.const 16
              i32.const 8
              call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
              local.set 8
              i32.const 0
              local.set 2
              i32.const 0
              i32.const 16
              i32.const 8
              call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
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
              call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
              i32.const -5
              i32.add
              local.get 3
              i32.gt_u
              select
              i32.const 8
              call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
              local.set 6
              local.get 0
              call $_ZN8dlmalloc8dlmalloc5Chunk8from_mem17hc2f0084eaea44926E
              local.set 1
              local.get 1
              local.get 1
              call $_ZN8dlmalloc8dlmalloc5Chunk4size17ha05d6c4faa787634E
              local.tee 7
              call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
              local.set 8
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            local.get 1
                            call $_ZN8dlmalloc8dlmalloc5Chunk7mmapped17h3cc0fa31d2a7a730E
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
                            call $_ZN8dlmalloc8dlmalloc5Chunk6cinuse17hbaa15675ecfdf704E
                            br_if 9 (;@3;)
                            local.get 8
                            call $_ZN8dlmalloc8dlmalloc5Chunk4size17ha05d6c4faa787634E
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
                            call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$18unlink_large_chunk17hd11f7eca1b1c75b8E
                            br 2 (;@10;)
                          end
                          local.get 1
                          call $_ZN8dlmalloc8dlmalloc5Chunk4size17ha05d6c4faa787634E
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
                          call $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$9page_size17h8880243cac5b5244E
                          call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
                          local.tee 7
                          i32.const 1
                          call $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$5remap17h2399769afa8dfc54E
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
                          call $_ZN8dlmalloc8dlmalloc5Chunk14fencepost_head17h06a0db07d0fb90cdE
                          local.set 0
                          local.get 1
                          local.get 2
                          call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
                          local.get 0
                          i32.store offset=4
                          local.get 1
                          local.get 3
                          i32.const -12
                          i32.add
                          call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
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
                        call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 1
                        local.get 6
                        call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
                        local.set 7
                        local.get 1
                        local.get 6
                        call $_ZN8dlmalloc8dlmalloc5Chunk9set_inuse17ha6e868917121b612E
                        local.get 7
                        local.get 10
                        call $_ZN8dlmalloc8dlmalloc5Chunk9set_inuse17ha6e868917121b612E
                        local.get 5
                        local.get 7
                        local.get 10
                        call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$13dispose_chunk17h1e388daf1185cdd1E
                        local.get 1
                        br_if 8 (;@2;)
                        br 7 (;@3;)
                      end
                      local.get 1
                      local.get 7
                      call $_ZN8dlmalloc8dlmalloc5Chunk9set_inuse17ha6e868917121b612E
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
                        call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
                        i32.ge_u
                        br_if 0 (;@10;)
                        local.get 1
                        local.get 7
                        call $_ZN8dlmalloc8dlmalloc5Chunk9set_inuse17ha6e868917121b612E
                        i32.const 0
                        local.set 8
                        i32.const 0
                        local.set 7
                        br 1 (;@9;)
                      end
                      local.get 1
                      local.get 6
                      call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
                      local.tee 7
                      local.get 8
                      call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
                      local.set 9
                      local.get 1
                      local.get 6
                      call $_ZN8dlmalloc8dlmalloc5Chunk9set_inuse17ha6e868917121b612E
                      local.get 7
                      local.get 8
                      call $_ZN8dlmalloc8dlmalloc5Chunk33set_size_and_pinuse_of_free_chunk17h683dbff7fd1b8b1aE
                      local.get 9
                      call $_ZN8dlmalloc8dlmalloc5Chunk12clear_pinuse17h5ad7f5fb7221b1b9E
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
                  call $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 1
                  local.get 6
                  call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
                  local.set 8
                  local.get 1
                  local.get 6
                  call $_ZN8dlmalloc8dlmalloc5Chunk9set_inuse17ha6e868917121b612E
                  local.get 8
                  local.get 7
                  call $_ZN8dlmalloc8dlmalloc5Chunk9set_inuse17ha6e868917121b612E
                  local.get 5
                  local.get 8
                  local.get 7
                  call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$13dispose_chunk17h1e388daf1185cdd1E
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
            call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$4free17h7ea880c3526c5524E
            br 3 (;@1;)
          end
          local.get 1
          local.get 6
          call $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E
          local.set 8
          local.get 1
          local.get 6
          call $_ZN8dlmalloc8dlmalloc5Chunk9set_inuse17ha6e868917121b612E
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
        call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$6malloc17hf43138a3d10fc105E
        local.tee 6
        i32.eqz
        br_if 1 (;@1;)
        local.get 6
        local.get 0
        local.get 1
        call $_ZN8dlmalloc8dlmalloc5Chunk4size17ha05d6c4faa787634E
        i32.const -8
        i32.const -4
        local.get 1
        call $_ZN8dlmalloc8dlmalloc5Chunk7mmapped17h3cc0fa31d2a7a730E
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
        call $_ZN8dlmalloc8dlmalloc17Dlmalloc$LT$A$GT$4free17h7ea880c3526c5524E
        local.get 3
        local.set 2
        br 1 (;@1;)
      end
      local.get 1
      call $_ZN8dlmalloc8dlmalloc5Chunk7mmapped17h3cc0fa31d2a7a730E
      drop
      local.get 1
      call $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h1d583dda34ec8e6fE
      local.set 2
    end
    local.get 4
    i32.const 15
    i32.add
    call $_ZN68_$LT$dlmalloc..global..Instance$u20$as$u20$core..ops..drop..Drop$GT$4drop17h3f91ec35f37562f3E
    local.get 4
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 2
  )
  (func $_ZN5alloc7raw_vec11finish_grow17hbc0c83b8e0da46b6E (;13;) (type 1) (param i32 i32)
    (local i32)
    block ;; label = @1
      block ;; label = @2
        local.get 1
        i32.load offset=4
        i32.eqz
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 1
          i32.const 8
          i32.add
          i32.load
          local.tee 2
          br_if 0 (;@3;)
          i32.const 0
          i32.load8_u offset=1048704
          drop
          i32.const 16
          i32.const 4
          call $__rust_alloc
          local.set 1
          br 2 (;@1;)
        end
        local.get 1
        i32.load
        local.get 2
        i32.const 4
        i32.const 16
        call $__rust_realloc
        local.set 1
        br 1 (;@1;)
      end
      i32.const 0
      i32.load8_u offset=1048704
      drop
      i32.const 16
      i32.const 4
      call $__rust_alloc
      local.set 1
    end
    block ;; label = @1
      block ;; label = @2
        local.get 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        i32.store offset=4
        i32.const 0
        local.set 1
        br 1 (;@1;)
      end
      local.get 0
      i32.const 4
      i32.store offset=4
      i32.const 1
      local.set 1
    end
    local.get 0
    local.get 1
    i32.store
    local.get 0
    i32.const 16
    i32.store offset=8
  )
  (func $vec_alloc (;14;) (type 8)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    local.get 0
    i32.const 0
    i32.store offset=24
    local.get 0
    i32.const 8
    i32.add
    local.get 0
    i32.const 20
    i32.add
    call $_ZN5alloc7raw_vec11finish_grow17hbc0c83b8e0da46b6E
    local.get 0
    i32.load offset=12
    local.set 1
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.load offset=8
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.const -2147483647
        i32.ne
        br_if 1 (;@1;)
        i32.const 1048576
        i32.const 40
        i32.const 1048672
        call $_ZN4core9panicking5panic17h5215ce16e3cbb479E
        unreachable
      end
      local.get 1
      i32.const 16
      i32.const 4
      call $__rust_dealloc
      local.get 0
      i32.const 32
      i32.add
      global.set $__stack_pointer
      return
    end
    i32.const 1048576
    i32.const 40
    i32.const 1048656
    call $_ZN4core9panicking5panic17h5215ce16e3cbb479E
    unreachable
  )
  (func $_ZN8dlmalloc8dlmalloc8align_up17h6dc22218d4f2cf36E (;15;) (type 3) (param i32 i32) (result i32)
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
  (func $_ZN8dlmalloc8dlmalloc9left_bits17h955e87685b8ece02E (;16;) (type 2) (param i32) (result i32)
    local.get 0
    i32.const 1
    i32.shl
    local.tee 0
    i32.const 0
    local.get 0
    i32.sub
    i32.or
  )
  (func $_ZN8dlmalloc8dlmalloc9least_bit17h7307b3bedf4731dcE (;17;) (type 2) (param i32) (result i32)
    i32.const 0
    local.get 0
    i32.sub
    local.get 0
    i32.and
  )
  (func $_ZN8dlmalloc8dlmalloc24leftshift_for_tree_index17h59b2e6a96fd4d5a4E (;18;) (type 2) (param i32) (result i32)
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
  (func $_ZN8dlmalloc8dlmalloc5Chunk14fencepost_head17h06a0db07d0fb90cdE (;19;) (type 6) (result i32)
    i32.const 7
  )
  (func $_ZN8dlmalloc8dlmalloc5Chunk4size17ha05d6c4faa787634E (;20;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load offset=4
    i32.const -8
    i32.and
  )
  (func $_ZN8dlmalloc8dlmalloc5Chunk6cinuse17hbaa15675ecfdf704E (;21;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load8_u offset=4
    i32.const 2
    i32.and
    i32.const 1
    i32.shr_u
  )
  (func $_ZN8dlmalloc8dlmalloc5Chunk6pinuse17hd290f2ba612e4cd6E (;22;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load offset=4
    i32.const 1
    i32.and
  )
  (func $_ZN8dlmalloc8dlmalloc5Chunk12clear_pinuse17h5ad7f5fb7221b1b9E (;23;) (type 5) (param i32)
    local.get 0
    local.get 0
    i32.load offset=4
    i32.const -2
    i32.and
    i32.store offset=4
  )
  (func $_ZN8dlmalloc8dlmalloc5Chunk5inuse17h39e6dae44e7f41daE (;24;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load offset=4
    i32.const 3
    i32.and
    i32.const 1
    i32.ne
  )
  (func $_ZN8dlmalloc8dlmalloc5Chunk7mmapped17h3cc0fa31d2a7a730E (;25;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load8_u offset=4
    i32.const 3
    i32.and
    i32.eqz
  )
  (func $_ZN8dlmalloc8dlmalloc5Chunk9set_inuse17ha6e868917121b612E (;26;) (type 1) (param i32 i32)
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
  (func $_ZN8dlmalloc8dlmalloc5Chunk20set_inuse_and_pinuse17hdbccaa0197e7f920E (;27;) (type 1) (param i32 i32)
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
  (func $_ZN8dlmalloc8dlmalloc5Chunk34set_size_and_pinuse_of_inuse_chunk17hf8c39f7e3e10a4ffE (;28;) (type 1) (param i32 i32)
    local.get 0
    local.get 1
    i32.const 3
    i32.or
    i32.store offset=4
  )
  (func $_ZN8dlmalloc8dlmalloc5Chunk33set_size_and_pinuse_of_free_chunk17h683dbff7fd1b8b1aE (;29;) (type 1) (param i32 i32)
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
  (func $_ZN8dlmalloc8dlmalloc5Chunk20set_free_with_pinuse17h7e233c8b10f845e1E (;30;) (type 0) (param i32 i32 i32)
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
  (func $_ZN8dlmalloc8dlmalloc5Chunk11plus_offset17h63f43155e8f8d2f1E (;31;) (type 3) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.add
  )
  (func $_ZN8dlmalloc8dlmalloc5Chunk12minus_offset17hc8694da7b578c86dE (;32;) (type 3) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.sub
  )
  (func $_ZN8dlmalloc8dlmalloc5Chunk6to_mem17h1d583dda34ec8e6fE (;33;) (type 2) (param i32) (result i32)
    local.get 0
    i32.const 8
    i32.add
  )
  (func $_ZN8dlmalloc8dlmalloc5Chunk10mem_offset17h2627fcfde9fd9cb1E (;34;) (type 6) (result i32)
    i32.const 8
  )
  (func $_ZN8dlmalloc8dlmalloc5Chunk8from_mem17hc2f0084eaea44926E (;35;) (type 2) (param i32) (result i32)
    local.get 0
    i32.const -8
    i32.add
  )
  (func $_ZN8dlmalloc8dlmalloc9TreeChunk14leftmost_child17hf6c05ef2deebc535E (;36;) (type 2) (param i32) (result i32)
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
  (func $_ZN8dlmalloc8dlmalloc9TreeChunk5chunk17h337b3f398402a12dE (;37;) (type 2) (param i32) (result i32)
    local.get 0
  )
  (func $_ZN8dlmalloc8dlmalloc9TreeChunk4next17h7d15145567f19ebbE (;38;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load offset=12
  )
  (func $_ZN8dlmalloc8dlmalloc9TreeChunk4prev17hcf95d98862e98be6E (;39;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load offset=8
  )
  (func $_ZN8dlmalloc8dlmalloc7Segment9is_extern17h0810f1513c6e5931E (;40;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load offset=12
    i32.const 1
    i32.and
  )
  (func $_ZN8dlmalloc8dlmalloc7Segment9sys_flags17hc63bc32fe8553ea7E (;41;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load offset=12
    i32.const 1
    i32.shr_u
  )
  (func $_ZN8dlmalloc8dlmalloc7Segment5holds17h4192d12beb9dcf77E (;42;) (type 3) (param i32 i32) (result i32)
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
  (func $_ZN8dlmalloc8dlmalloc7Segment3top17hc9cd36f864405361E (;43;) (type 2) (param i32) (result i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    i32.add
  )
  (func $_ZN73_$LT$dlmalloc..global..Instance$u20$as$u20$core..ops..deref..DerefMut$GT$9deref_mut17hd7579f2c72609989E (;44;) (type 2) (param i32) (result i32)
    i32.const 1048708
  )
  (func $_ZN68_$LT$dlmalloc..global..Instance$u20$as$u20$core..ops..drop..Drop$GT$4drop17h3f91ec35f37562f3E (;45;) (type 5) (param i32))
  (func $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$5alloc17h16d966d43fa28c14E (;46;) (type 0) (param i32 i32 i32)
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
  (func $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$5remap17h2399769afa8dfc54E (;47;) (type 9) (param i32 i32 i32 i32 i32) (result i32)
    i32.const 0
  )
  (func $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$9free_part17hf77ea82ca2f1faf0E (;48;) (type 7) (param i32 i32 i32 i32) (result i32)
    i32.const 0
  )
  (func $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$4free17h35a14ba6044af2d8E (;49;) (type 4) (param i32 i32 i32) (result i32)
    i32.const 0
  )
  (func $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$16can_release_part17hecdc06996915370fE (;50;) (type 3) (param i32 i32) (result i32)
    i32.const 0
  )
  (func $_ZN61_$LT$dlmalloc..sys..System$u20$as$u20$dlmalloc..Allocator$GT$9page_size17h8880243cac5b5244E (;51;) (type 2) (param i32) (result i32)
    i32.const 65536
  )
  (func $_ZN8dlmalloc3sys23enable_alloc_after_fork17h32651baadeb17bebE (;52;) (type 8))
  (func $_ZN4core3ptr88drop_in_place$LT$core..panic..panic_info..PanicInfo..internal_constructor..NoPayload$GT$17h45eb6ad4d711d518E (;53;) (type 5) (param i32))
  (func $_ZN4core9panicking9panic_fmt17hb5bbe835c959a199E (;54;) (type 1) (param i32 i32)
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
    i32.const 1048688
    i32.store offset=16
    local.get 2
    i32.const 1048688
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
  (func $_ZN4core9panicking5panic17h5215ce16e3cbb479E (;55;) (type 0) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 12
    i32.add
    i64.const 0
    i64.store align=4
    local.get 3
    i32.const 1
    i32.store offset=4
    local.get 3
    i32.const 1048688
    i32.store offset=8
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
    call $_ZN4core9panicking9panic_fmt17hb5bbe835c959a199E
    unreachable
  )
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h680f3467e2b82a44E (;56;) (type 1) (param i32 i32)
    local.get 0
    i64.const -4393008357061442826
    i64.store offset=8
    local.get 0
    i64.const 4385061917285681825
    i64.store
  )
  (func $memcpy (;57;) (type 4) (param i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    call $_ZN17compiler_builtins3mem6memcpy17hf24b14b321f74b3fE
  )
  (func $_ZN17compiler_builtins3mem6memcpy17hf24b14b321f74b3fE (;58;) (type 4) (param i32 i32 i32) (result i32)
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
  (table (;0;) 3 3 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (global (;1;) i32 i32.const 1049160)
  (global (;2;) i32 i32.const 1049168)
  (export "memory" (memory 0))
  (export "__main" (func $__main))
  (export "vec_alloc" (func $vec_alloc))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (elem (;0;) (i32.const 1) func $_ZN4core3ptr88drop_in_place$LT$core..panic..panic_info..PanicInfo..internal_constructor..NoPayload$GT$17h45eb6ad4d711d518E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h680f3467e2b82a44E)
  (data $.rodata (;0;) (i32.const 1048576) "internal error: entered unreachable codetests/rust-wasm-tests/src/dlmalloc.rs\00\00\00(\00\10\00%\00\00\00\08\00\00\00\09\00\00\00(\00\10\00%\00\00\00\0b\00\00\00\09\00\00\00\01\00\00\00\00\00\00\00\01\00\00\00\02\00\00\00")
)