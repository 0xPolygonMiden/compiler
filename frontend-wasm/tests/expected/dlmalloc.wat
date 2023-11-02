(module
  (type (;0;) (func (param i32 i32 i32)))
  (type (;1;) (func (param i32 i32)))
  (type (;2;) (func (param i32 i32) (result i32)))
  (type (;3;) (func (param i32 i32 i32) (result i32)))
  (type (;4;) (func (result i32)))
  (type (;5;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;6;) (func (param i32 i32 i32 i32)))
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::dispose_chunk (;0;) (type 0) (param i32 i32 i32)
    (local i32 i32 i32 i32)
    local.get 1
    local.get 2
    i32.add
    local.set 3
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i32.load offset=4
          local.tee 4
          i32.const 1
          i32.and
          br_if 0 (;@3;)
          local.get 4
          i32.const 3
          i32.and
          i32.eqz
          br_if 1 (;@2;)
          local.get 1
          i32.load
          local.tee 4
          local.get 2
          i32.add
          local.set 2
          block ;; label = @4
            local.get 1
            local.get 4
            i32.sub
            local.tee 1
            local.get 0
            i32.load offset=424
            i32.ne
            br_if 0 (;@4;)
            local.get 3
            i32.load offset=4
            local.tee 4
            i32.const 3
            i32.and
            i32.const 3
            i32.ne
            br_if 1 (;@3;)
            local.get 3
            local.get 4
            i32.const -2
            i32.and
            i32.store offset=4
            local.get 1
            local.get 2
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 0
            local.get 2
            i32.store offset=416
            local.get 3
            local.get 2
            i32.store
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
            br 1 (;@3;)
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
            br 1 (;@3;)
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
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 3
              i32.load offset=4
              local.tee 4
              i32.const 2
              i32.and
              br_if 0 (;@5;)
              local.get 3
              local.get 0
              i32.load offset=428
              i32.eq
              br_if 2 (;@3;)
              local.get 3
              local.get 0
              i32.load offset=424
              i32.eq
              br_if 4 (;@1;)
              local.get 4
              i32.const -8
              i32.and
              local.tee 5
              local.get 2
              i32.add
              local.set 2
              block ;; label = @6
                block ;; label = @7
                  local.get 5
                  i32.const 256
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 0
                  local.get 3
                  call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_large_chunk
                  br 1 (;@6;)
                end
                block ;; label = @7
                  local.get 3
                  i32.load offset=12
                  local.tee 5
                  local.get 3
                  i32.load offset=8
                  local.tee 3
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 3
                  local.get 5
                  i32.store offset=12
                  local.get 5
                  local.get 3
                  i32.store offset=8
                  br 1 (;@6;)
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
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 1
              local.get 2
              i32.add
              local.get 2
              i32.store
              local.get 1
              local.get 0
              i32.load offset=424
              i32.ne
              br_if 1 (;@4;)
              local.get 0
              local.get 2
              i32.store offset=416
              return
            end
            local.get 3
            local.get 4
            i32.const -2
            i32.and
            i32.store offset=4
            local.get 1
            local.get 2
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 1
            local.get 2
            i32.add
            local.get 2
            i32.store
          end
          block ;; label = @4
            local.get 2
            i32.const 256
            i32.lt_u
            br_if 0 (;@4;)
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
          block ;; label = @4
            block ;; label = @5
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
              br_if 0 (;@5;)
              local.get 0
              local.get 4
              local.get 2
              i32.or
              i32.store offset=408
              local.get 3
              local.set 2
              br 1 (;@4;)
            end
            local.get 3
            i32.load offset=8
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
        i32.ne
        br_if 0 (;@2;)
        local.get 0
        i32.const 0
        i32.store offset=416
        local.get 0
        i32.const 0
        i32.store offset=424
      end
      return
    end
    local.get 1
    local.get 0
    i32.load offset=416
    local.get 2
    i32.add
    local.tee 2
    i32.const 1
    i32.or
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store offset=424
    local.get 0
    local.get 2
    i32.store offset=416
    local.get 1
    local.get 2
    i32.add
    local.get 2
    i32.store
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
          i32.load offset=12
          local.tee 3
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
        i32.load offset=8
        local.tee 5
        local.get 3
        i32.store offset=12
        local.get 3
        local.get 5
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
    (local i32 i32 i32 i32)
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
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.load offset=412
        local.tee 5
        i32.const 1
        local.get 3
        i32.shl
        local.tee 6
        i32.and
        br_if 0 (;@2;)
        local.get 0
        local.get 5
        local.get 6
        i32.or
        i32.store offset=412
        local.get 1
        local.get 4
        i32.store offset=24
        local.get 4
        local.get 1
        i32.store
        br 1 (;@1;)
      end
      local.get 2
      i32.const 0
      i32.const 25
      local.get 3
      i32.const 1
      i32.shr_u
      i32.sub
      i32.const 31
      i32.and
      local.get 3
      i32.const 31
      i32.eq
      select
      i32.shl
      local.set 3
      local.get 4
      i32.load
      local.set 4
      loop ;; label = @2
        block ;; label = @3
          local.get 4
          local.tee 0
          i32.load offset=4
          i32.const -8
          i32.and
          local.get 2
          i32.ne
          br_if 0 (;@3;)
          local.get 0
          i32.load offset=8
          local.tee 3
          local.get 1
          i32.store offset=12
          local.get 0
          local.get 1
          i32.store offset=8
          local.get 1
          i32.const 0
          i32.store offset=24
          local.get 1
          local.get 0
          i32.store offset=12
          local.get 1
          local.get 3
          i32.store offset=8
          return
        end
        local.get 3
        i32.const 29
        i32.shr_u
        local.set 4
        local.get 3
        i32.const 1
        i32.shl
        local.set 3
        local.get 0
        local.get 4
        i32.const 4
        i32.and
        i32.add
        i32.const 16
        i32.add
        local.tee 5
        i32.load
        local.tee 4
        br_if 0 (;@2;)
      end
      local.get 5
      local.get 1
      i32.store
      local.get 1
      local.get 0
      i32.store offset=24
    end
    local.get 1
    local.get 1
    i32.store offset=12
    local.get 1
    local.get 1
    i32.store offset=8
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::free (;3;) (type 1) (param i32 i32)
    (local i32 i32 i32 i32 i32)
    local.get 1
    i32.const -8
    i32.add
    local.tee 2
    local.get 1
    i32.const -4
    i32.add
    i32.load
    local.tee 3
    i32.const -8
    i32.and
    local.tee 1
    i32.add
    local.set 4
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 3
                i32.const 1
                i32.and
                br_if 0 (;@6;)
                local.get 3
                i32.const 3
                i32.and
                i32.eqz
                br_if 1 (;@5;)
                local.get 2
                i32.load
                local.tee 3
                local.get 1
                i32.add
                local.set 1
                block ;; label = @7
                  local.get 2
                  local.get 3
                  i32.sub
                  local.tee 2
                  local.get 0
                  i32.load offset=424
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 4
                  i32.load offset=4
                  local.tee 3
                  i32.const 3
                  i32.and
                  i32.const 3
                  i32.ne
                  br_if 1 (;@6;)
                  local.get 4
                  local.get 3
                  i32.const -2
                  i32.and
                  i32.store offset=4
                  local.get 2
                  local.get 1
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 0
                  local.get 1
                  i32.store offset=416
                  local.get 4
                  local.get 1
                  i32.store
                  return
                end
                block ;; label = @7
                  local.get 3
                  i32.const 256
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 0
                  local.get 2
                  call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_large_chunk
                  br 1 (;@6;)
                end
                block ;; label = @7
                  local.get 2
                  i32.load offset=12
                  local.tee 5
                  local.get 2
                  i32.load offset=8
                  local.tee 6
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 6
                  local.get 5
                  i32.store offset=12
                  local.get 5
                  local.get 6
                  i32.store offset=8
                  br 1 (;@6;)
                end
                local.get 0
                local.get 0
                i32.load offset=408
                i32.const -2
                local.get 3
                i32.const 3
                i32.shr_u
                i32.rotl
                i32.and
                i32.store offset=408
              end
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    local.get 4
                    i32.load offset=4
                    local.tee 3
                    i32.const 2
                    i32.and
                    br_if 0 (;@8;)
                    local.get 4
                    local.get 0
                    i32.load offset=428
                    i32.eq
                    br_if 2 (;@6;)
                    local.get 4
                    local.get 0
                    i32.load offset=424
                    i32.eq
                    br_if 7 (;@1;)
                    local.get 3
                    i32.const -8
                    i32.and
                    local.tee 5
                    local.get 1
                    i32.add
                    local.set 1
                    block ;; label = @9
                      block ;; label = @10
                        local.get 5
                        i32.const 256
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 0
                        local.get 4
                        call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_large_chunk
                        br 1 (;@9;)
                      end
                      block ;; label = @10
                        local.get 4
                        i32.load offset=12
                        local.tee 5
                        local.get 4
                        i32.load offset=8
                        local.tee 4
                        i32.eq
                        br_if 0 (;@10;)
                        local.get 4
                        local.get 5
                        i32.store offset=12
                        local.get 5
                        local.get 4
                        i32.store offset=8
                        br 1 (;@9;)
                      end
                      local.get 0
                      local.get 0
                      i32.load offset=408
                      i32.const -2
                      local.get 3
                      i32.const 3
                      i32.shr_u
                      i32.rotl
                      i32.and
                      i32.store offset=408
                    end
                    local.get 2
                    local.get 1
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    local.get 2
                    local.get 1
                    i32.add
                    local.get 1
                    i32.store
                    local.get 2
                    local.get 0
                    i32.load offset=424
                    i32.ne
                    br_if 1 (;@7;)
                    local.get 0
                    local.get 1
                    i32.store offset=416
                    return
                  end
                  local.get 4
                  local.get 3
                  i32.const -2
                  i32.and
                  i32.store offset=4
                  local.get 2
                  local.get 1
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 2
                  local.get 1
                  i32.add
                  local.get 1
                  i32.store
                end
                local.get 1
                i32.const 256
                i32.lt_u
                br_if 2 (;@4;)
                local.get 0
                local.get 2
                local.get 1
                call $dlmalloc::dlmalloc::Dlmalloc<A>::insert_large_chunk
                local.get 0
                local.get 0
                i32.load offset=448
                i32.const -1
                i32.add
                local.tee 2
                i32.store offset=448
                local.get 2
                br_if 1 (;@5;)
                local.get 0
                i32.const 136
                i32.add
                i32.load
                local.tee 1
                br_if 3 (;@3;)
                i32.const 0
                local.set 2
                br 4 (;@2;)
              end
              local.get 0
              local.get 2
              i32.store offset=428
              local.get 0
              local.get 0
              i32.load offset=420
              local.get 1
              i32.add
              local.tee 1
              i32.store offset=420
              local.get 2
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=4
              block ;; label = @6
                local.get 2
                local.get 0
                i32.load offset=424
                i32.ne
                br_if 0 (;@6;)
                local.get 0
                i32.const 0
                i32.store offset=416
                local.get 0
                i32.const 0
                i32.store offset=424
              end
              local.get 1
              local.get 0
              i32.load offset=440
              i32.le_u
              br_if 0 (;@5;)
              block ;; label = @6
                local.get 1
                i32.const 41
                i32.lt_u
                br_if 0 (;@6;)
                local.get 0
                i32.const 128
                i32.add
                local.set 1
                loop ;; label = @7
                  block ;; label = @8
                    local.get 1
                    i32.load
                    local.tee 4
                    local.get 2
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 4
                    local.get 1
                    i32.load offset=4
                    i32.add
                    local.get 2
                    i32.gt_u
                    br_if 2 (;@6;)
                  end
                  local.get 1
                  i32.load offset=8
                  local.tee 1
                  br_if 0 (;@7;)
                end
              end
              block ;; label = @6
                block ;; label = @7
                  local.get 0
                  i32.const 136
                  i32.add
                  i32.load
                  local.tee 1
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 2
                  br 1 (;@6;)
                end
                i32.const 0
                local.set 2
                loop ;; label = @7
                  local.get 2
                  i32.const 1
                  i32.add
                  local.set 2
                  local.get 1
                  i32.load offset=8
                  local.tee 1
                  br_if 0 (;@7;)
                end
              end
              local.get 0
              i32.const -1
              i32.store offset=440
              local.get 0
              local.get 2
              i32.const 4095
              local.get 2
              i32.const 4095
              i32.gt_u
              select
              i32.store offset=448
            end
            return
          end
          local.get 0
          local.get 1
          i32.const -8
          i32.and
          i32.add
          i32.const 144
          i32.add
          local.set 4
          block ;; label = @4
            block ;; label = @5
              local.get 0
              i32.load offset=408
              local.tee 3
              i32.const 1
              local.get 1
              i32.const 3
              i32.shr_u
              i32.shl
              local.tee 1
              i32.and
              br_if 0 (;@5;)
              local.get 0
              local.get 3
              local.get 1
              i32.or
              i32.store offset=408
              local.get 4
              local.set 0
              br 1 (;@4;)
            end
            local.get 4
            i32.load offset=8
            local.set 0
          end
          local.get 4
          local.get 2
          i32.store offset=8
          local.get 0
          local.get 2
          i32.store offset=12
          local.get 2
          local.get 4
          i32.store offset=12
          local.get 2
          local.get 0
          i32.store offset=8
          return
        end
        i32.const 0
        local.set 2
        loop ;; label = @3
          local.get 2
          i32.const 1
          i32.add
          local.set 2
          local.get 1
          i32.load offset=8
          local.tee 1
          br_if 0 (;@3;)
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
      return
    end
    local.get 2
    local.get 0
    i32.load offset=416
    local.get 1
    i32.add
    local.tee 1
    i32.const 1
    i32.or
    i32.store offset=4
    local.get 0
    local.get 2
    i32.store offset=424
    local.get 0
    local.get 1
    i32.store offset=416
    local.get 2
    local.get 1
    i32.add
    local.get 1
    i32.store
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::malloc (;4;) (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i64)
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
                i32.const 0
                local.set 2
                local.get 1
                i32.const -65587
                i32.ge_u
                br_if 5 (;@1;)
                local.get 1
                i32.const 11
                i32.add
                local.tee 3
                i32.const -8
                i32.and
                local.set 4
                local.get 0
                i32.load offset=412
                local.tee 5
                i32.eqz
                br_if 4 (;@2;)
                i32.const 0
                local.set 1
                i32.const 0
                local.set 6
                block ;; label = @7
                  local.get 4
                  i32.const 256
                  i32.lt_u
                  br_if 0 (;@7;)
                  i32.const 31
                  local.set 6
                  local.get 4
                  i32.const 16777215
                  i32.gt_u
                  br_if 0 (;@7;)
                  local.get 4
                  i32.const 6
                  local.get 3
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
                  local.set 6
                end
                i32.const 0
                local.get 4
                i32.sub
                local.set 3
                block ;; label = @7
                  local.get 0
                  local.get 6
                  i32.const 2
                  i32.shl
                  i32.add
                  i32.load
                  local.tee 7
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 8
                  br 2 (;@5;)
                end
                i32.const 0
                local.set 1
                local.get 4
                i32.const 0
                i32.const 25
                local.get 6
                i32.const 1
                i32.shr_u
                i32.sub
                local.get 6
                i32.const 31
                i32.eq
                select
                i32.shl
                local.set 2
                i32.const 0
                local.set 8
                loop ;; label = @7
                  block ;; label = @8
                    local.get 7
                    i32.load offset=4
                    i32.const -8
                    i32.and
                    local.tee 9
                    local.get 4
                    i32.lt_u
                    br_if 0 (;@8;)
                    local.get 9
                    local.get 4
                    i32.sub
                    local.tee 9
                    local.get 3
                    i32.ge_u
                    br_if 0 (;@8;)
                    local.get 9
                    local.set 3
                    local.get 7
                    local.set 8
                    local.get 9
                    br_if 0 (;@8;)
                    i32.const 0
                    local.set 3
                    local.get 7
                    local.set 8
                    local.get 7
                    local.set 1
                    br 4 (;@4;)
                  end
                  local.get 7
                  i32.const 20
                  i32.add
                  i32.load
                  local.tee 9
                  local.get 1
                  local.get 9
                  local.get 7
                  local.get 2
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
                  local.get 9
                  select
                  local.set 1
                  local.get 2
                  i32.const 1
                  i32.shl
                  local.set 2
                  local.get 7
                  i32.eqz
                  br_if 2 (;@5;)
                  br 0 (;@7;)
                end
              end
              block ;; label = @6
                local.get 0
                i32.load offset=408
                local.tee 8
                i32.const 16
                local.get 1
                i32.const 11
                i32.add
                i32.const -8
                i32.and
                local.get 1
                i32.const 11
                i32.lt_u
                select
                local.tee 4
                i32.const 3
                i32.shr_u
                local.tee 3
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
                    local.get 3
                    i32.add
                    local.tee 4
                    i32.const 3
                    i32.shl
                    i32.add
                    local.tee 7
                    i32.const 152
                    i32.add
                    i32.load
                    local.tee 1
                    i32.load offset=8
                    local.tee 3
                    local.get 7
                    i32.const 144
                    i32.add
                    local.tee 7
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 3
                    local.get 7
                    i32.store offset=12
                    local.get 7
                    local.get 3
                    i32.store offset=8
                    br 1 (;@7;)
                  end
                  local.get 0
                  local.get 8
                  i32.const -2
                  local.get 4
                  i32.rotl
                  i32.and
                  i32.store offset=408
                end
                local.get 1
                local.get 4
                i32.const 3
                i32.shl
                local.tee 4
                i32.const 3
                i32.or
                i32.store offset=4
                local.get 1
                local.get 4
                i32.add
                local.tee 4
                local.get 4
                i32.load offset=4
                i32.const 1
                i32.or
                i32.store offset=4
                local.get 1
                i32.const 8
                i32.add
                return
              end
              local.get 4
              local.get 0
              i32.load offset=416
              i32.le_u
              br_if 3 (;@2;)
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    local.get 1
                    br_if 0 (;@8;)
                    local.get 0
                    i32.load offset=412
                    local.tee 1
                    i32.eqz
                    br_if 6 (;@2;)
                    local.get 0
                    local.get 1
                    i32.ctz
                    i32.const 2
                    i32.shl
                    i32.add
                    i32.load
                    local.tee 7
                    i32.load offset=4
                    i32.const -8
                    i32.and
                    local.get 4
                    i32.sub
                    local.set 3
                    local.get 7
                    local.set 8
                    block ;; label = @9
                      block ;; label = @10
                        loop ;; label = @11
                          block ;; label = @12
                            local.get 7
                            i32.load offset=16
                            local.tee 1
                            br_if 0 (;@12;)
                            local.get 7
                            i32.const 20
                            i32.add
                            i32.load
                            local.tee 1
                            br_if 0 (;@12;)
                            local.get 0
                            local.get 8
                            call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_large_chunk
                            local.get 3
                            i32.const 16
                            i32.lt_u
                            br_if 3 (;@9;)
                            local.get 8
                            local.get 4
                            i32.const 3
                            i32.or
                            i32.store offset=4
                            local.get 8
                            local.get 4
                            i32.add
                            local.tee 4
                            local.get 3
                            i32.const 1
                            i32.or
                            i32.store offset=4
                            local.get 4
                            local.get 3
                            i32.add
                            local.get 3
                            i32.store
                            local.get 0
                            i32.load offset=416
                            local.tee 2
                            br_if 2 (;@10;)
                            br 5 (;@7;)
                          end
                          local.get 1
                          i32.load offset=4
                          i32.const -8
                          i32.and
                          local.get 4
                          i32.sub
                          local.tee 7
                          local.get 3
                          local.get 7
                          local.get 3
                          i32.lt_u
                          local.tee 7
                          select
                          local.set 3
                          local.get 1
                          local.get 8
                          local.get 7
                          select
                          local.set 8
                          local.get 1
                          local.set 7
                          br 0 (;@11;)
                        end
                      end
                      local.get 0
                      local.get 2
                      i32.const -8
                      i32.and
                      i32.add
                      i32.const 144
                      i32.add
                      local.set 7
                      local.get 0
                      i32.load offset=424
                      local.set 1
                      block ;; label = @10
                        block ;; label = @11
                          local.get 0
                          i32.load offset=408
                          local.tee 9
                          i32.const 1
                          local.get 2
                          i32.const 3
                          i32.shr_u
                          i32.shl
                          local.tee 2
                          i32.and
                          br_if 0 (;@11;)
                          local.get 0
                          local.get 9
                          local.get 2
                          i32.or
                          i32.store offset=408
                          local.get 7
                          local.set 2
                          br 1 (;@10;)
                        end
                        local.get 7
                        i32.load offset=8
                        local.set 2
                      end
                      local.get 7
                      local.get 1
                      i32.store offset=8
                      local.get 2
                      local.get 1
                      i32.store offset=12
                      local.get 1
                      local.get 7
                      i32.store offset=12
                      local.get 1
                      local.get 2
                      i32.store offset=8
                      br 2 (;@7;)
                    end
                    local.get 8
                    local.get 3
                    local.get 4
                    i32.add
                    local.tee 1
                    i32.const 3
                    i32.or
                    i32.store offset=4
                    local.get 8
                    local.get 1
                    i32.add
                    local.tee 1
                    local.get 1
                    i32.load offset=4
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    br 2 (;@6;)
                  end
                  block ;; label = @8
                    block ;; label = @9
                      local.get 0
                      i32.const 144
                      i32.add
                      local.tee 9
                      local.get 1
                      local.get 3
                      i32.shl
                      i32.const 2
                      local.get 3
                      i32.shl
                      local.tee 1
                      i32.const 0
                      local.get 1
                      i32.sub
                      i32.or
                      i32.and
                      i32.ctz
                      local.tee 7
                      i32.const 3
                      i32.shl
                      i32.add
                      local.tee 3
                      i32.load offset=8
                      local.tee 1
                      i32.load offset=8
                      local.tee 2
                      local.get 3
                      i32.eq
                      br_if 0 (;@9;)
                      local.get 2
                      local.get 3
                      i32.store offset=12
                      local.get 3
                      local.get 2
                      i32.store offset=8
                      br 1 (;@8;)
                    end
                    local.get 0
                    local.get 8
                    i32.const -2
                    local.get 7
                    i32.rotl
                    i32.and
                    i32.store offset=408
                  end
                  local.get 1
                  local.get 4
                  i32.const 3
                  i32.or
                  i32.store offset=4
                  local.get 1
                  local.get 4
                  i32.add
                  local.tee 2
                  local.get 7
                  i32.const 3
                  i32.shl
                  local.tee 7
                  local.get 4
                  i32.sub
                  local.tee 3
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 1
                  local.get 7
                  i32.add
                  local.get 3
                  i32.store
                  block ;; label = @8
                    local.get 0
                    i32.load offset=416
                    local.tee 8
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 9
                    local.get 8
                    i32.const -8
                    i32.and
                    i32.add
                    local.set 7
                    local.get 0
                    i32.load offset=424
                    local.set 4
                    block ;; label = @9
                      block ;; label = @10
                        local.get 0
                        i32.load offset=408
                        local.tee 9
                        i32.const 1
                        local.get 8
                        i32.const 3
                        i32.shr_u
                        i32.shl
                        local.tee 8
                        i32.and
                        br_if 0 (;@10;)
                        local.get 0
                        local.get 9
                        local.get 8
                        i32.or
                        i32.store offset=408
                        local.get 7
                        local.set 8
                        br 1 (;@9;)
                      end
                      local.get 7
                      i32.load offset=8
                      local.set 8
                    end
                    local.get 7
                    local.get 4
                    i32.store offset=8
                    local.get 8
                    local.get 4
                    i32.store offset=12
                    local.get 4
                    local.get 7
                    i32.store offset=12
                    local.get 4
                    local.get 8
                    i32.store offset=8
                  end
                  local.get 0
                  local.get 2
                  i32.store offset=424
                  local.get 0
                  local.get 3
                  i32.store offset=416
                  local.get 1
                  i32.const 8
                  i32.add
                  return
                end
                local.get 0
                local.get 4
                i32.store offset=424
                local.get 0
                local.get 3
                i32.store offset=416
              end
              local.get 8
              i32.const 8
              i32.add
              return
            end
            block ;; label = @5
              local.get 1
              local.get 8
              i32.or
              br_if 0 (;@5;)
              i32.const 0
              local.set 8
              i32.const 2
              local.get 6
              i32.shl
              local.tee 1
              i32.const 0
              local.get 1
              i32.sub
              i32.or
              local.get 5
              i32.and
              local.tee 1
              i32.eqz
              br_if 3 (;@2;)
              local.get 0
              local.get 1
              i32.ctz
              i32.const 2
              i32.shl
              i32.add
              i32.load
              local.set 1
            end
            local.get 1
            i32.eqz
            br_if 1 (;@3;)
          end
          loop ;; label = @4
            local.get 1
            local.get 8
            local.get 1
            i32.load offset=4
            i32.const -8
            i32.and
            local.tee 7
            local.get 4
            i32.sub
            local.tee 9
            local.get 3
            i32.lt_u
            local.tee 6
            select
            local.set 5
            local.get 7
            local.get 4
            i32.lt_u
            local.set 2
            local.get 9
            local.get 3
            local.get 6
            select
            local.set 9
            block ;; label = @5
              local.get 1
              i32.load offset=16
              local.tee 7
              br_if 0 (;@5;)
              local.get 1
              i32.const 20
              i32.add
              i32.load
              local.set 7
            end
            local.get 8
            local.get 5
            local.get 2
            select
            local.set 8
            local.get 3
            local.get 9
            local.get 2
            select
            local.set 3
            local.get 7
            local.set 1
            local.get 7
            br_if 0 (;@4;)
          end
        end
        local.get 8
        i32.eqz
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 0
          i32.load offset=416
          local.tee 1
          local.get 4
          i32.lt_u
          br_if 0 (;@3;)
          local.get 3
          local.get 1
          local.get 4
          i32.sub
          i32.ge_u
          br_if 1 (;@2;)
        end
        local.get 0
        local.get 8
        call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_large_chunk
        block ;; label = @3
          block ;; label = @4
            local.get 3
            i32.const 16
            i32.lt_u
            br_if 0 (;@4;)
            local.get 8
            local.get 4
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 8
            local.get 4
            i32.add
            local.tee 1
            local.get 3
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 1
            local.get 3
            i32.add
            local.get 3
            i32.store
            block ;; label = @5
              local.get 3
              i32.const 256
              i32.lt_u
              br_if 0 (;@5;)
              local.get 0
              local.get 1
              local.get 3
              call $dlmalloc::dlmalloc::Dlmalloc<A>::insert_large_chunk
              br 2 (;@3;)
            end
            local.get 0
            local.get 3
            i32.const -8
            i32.and
            i32.add
            i32.const 144
            i32.add
            local.set 4
            block ;; label = @5
              block ;; label = @6
                local.get 0
                i32.load offset=408
                local.tee 7
                i32.const 1
                local.get 3
                i32.const 3
                i32.shr_u
                i32.shl
                local.tee 3
                i32.and
                br_if 0 (;@6;)
                local.get 0
                local.get 7
                local.get 3
                i32.or
                i32.store offset=408
                local.get 4
                local.set 3
                br 1 (;@5;)
              end
              local.get 4
              i32.load offset=8
              local.set 3
            end
            local.get 4
            local.get 1
            i32.store offset=8
            local.get 3
            local.get 1
            i32.store offset=12
            local.get 1
            local.get 4
            i32.store offset=12
            local.get 1
            local.get 3
            i32.store offset=8
            br 1 (;@3;)
          end
          local.get 8
          local.get 3
          local.get 4
          i32.add
          local.tee 1
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 8
          local.get 1
          i32.add
          local.tee 1
          local.get 1
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
        end
        local.get 8
        i32.const 8
        i32.add
        return
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
                      local.tee 1
                      local.get 4
                      i32.ge_u
                      br_if 0 (;@9;)
                      block ;; label = @10
                        local.get 0
                        i32.load offset=420
                        local.tee 1
                        local.get 4
                        i32.gt_u
                        br_if 0 (;@10;)
                        i32.const 0
                        local.set 2
                        local.get 4
                        i32.const 65583
                        i32.add
                        local.tee 3
                        i32.const 16
                        i32.shr_u
                        memory.grow
                        local.tee 1
                        i32.const -1
                        i32.eq
                        local.tee 7
                        br_if 9 (;@1;)
                        local.get 1
                        i32.const 16
                        i32.shl
                        local.tee 9
                        i32.eqz
                        br_if 9 (;@1;)
                        local.get 0
                        local.get 0
                        i32.load offset=432
                        i32.const 0
                        local.get 3
                        i32.const -65536
                        i32.and
                        local.get 7
                        select
                        local.tee 6
                        i32.add
                        local.tee 1
                        i32.store offset=432
                        local.get 0
                        local.get 0
                        i32.load offset=436
                        local.tee 3
                        local.get 1
                        local.get 3
                        local.get 1
                        i32.gt_u
                        select
                        i32.store offset=436
                        block ;; label = @11
                          local.get 0
                          i32.load offset=428
                          local.tee 3
                          br_if 0 (;@11;)
                          block ;; label = @12
                            block ;; label = @13
                              local.get 0
                              i32.load offset=444
                              local.tee 1
                              i32.eqz
                              br_if 0 (;@13;)
                              local.get 1
                              local.get 9
                              i32.le_u
                              br_if 1 (;@12;)
                            end
                            local.get 0
                            local.get 9
                            i32.store offset=444
                          end
                          local.get 0
                          i32.const 4095
                          i32.store offset=448
                          local.get 0
                          local.get 9
                          i32.store offset=128
                          i32.const 0
                          local.set 3
                          local.get 0
                          i32.const 140
                          i32.add
                          i32.const 0
                          i32.store
                          local.get 0
                          i32.const 132
                          i32.add
                          local.get 6
                          i32.store
                          loop ;; label = @12
                            local.get 0
                            local.get 3
                            i32.add
                            local.tee 1
                            i32.const 164
                            i32.add
                            local.get 1
                            i32.const 152
                            i32.add
                            local.tee 7
                            i32.store
                            local.get 7
                            local.get 1
                            i32.const 144
                            i32.add
                            local.tee 8
                            i32.store
                            local.get 1
                            i32.const 156
                            i32.add
                            local.get 8
                            i32.store
                            local.get 1
                            i32.const 172
                            i32.add
                            local.get 1
                            i32.const 160
                            i32.add
                            local.tee 8
                            i32.store
                            local.get 8
                            local.get 7
                            i32.store
                            local.get 1
                            i32.const 180
                            i32.add
                            local.get 1
                            i32.const 168
                            i32.add
                            local.tee 7
                            i32.store
                            local.get 7
                            local.get 8
                            i32.store
                            local.get 1
                            i32.const 176
                            i32.add
                            local.get 7
                            i32.store
                            local.get 3
                            i32.const 32
                            i32.add
                            local.tee 3
                            i32.const 256
                            i32.ne
                            br_if 0 (;@12;)
                          end
                          local.get 9
                          local.get 6
                          i32.const -40
                          i32.add
                          local.tee 1
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          local.get 0
                          local.get 9
                          i32.store offset=428
                          local.get 0
                          i32.const 2097152
                          i32.store offset=440
                          local.get 0
                          local.get 1
                          i32.store offset=420
                          local.get 9
                          local.get 1
                          i32.add
                          i32.const 40
                          i32.store offset=4
                          br 9 (;@2;)
                        end
                        local.get 0
                        i32.const 128
                        i32.add
                        local.tee 5
                        local.set 1
                        block ;; label = @11
                          block ;; label = @12
                            loop ;; label = @13
                              local.get 1
                              i32.load
                              local.tee 7
                              local.get 1
                              i32.load offset=4
                              local.tee 8
                              i32.add
                              local.get 9
                              i32.eq
                              br_if 1 (;@12;)
                              local.get 1
                              i32.load offset=8
                              local.tee 1
                              br_if 0 (;@13;)
                              br 2 (;@11;)
                            end
                          end
                          local.get 3
                          local.get 9
                          i32.ge_u
                          br_if 0 (;@11;)
                          local.get 7
                          local.get 3
                          i32.gt_u
                          br_if 0 (;@11;)
                          local.get 1
                          i32.load offset=12
                          i32.eqz
                          br_if 3 (;@8;)
                        end
                        local.get 0
                        local.get 0
                        i32.load offset=444
                        local.tee 1
                        local.get 9
                        local.get 1
                        local.get 9
                        i32.lt_u
                        select
                        i32.store offset=444
                        local.get 9
                        local.get 6
                        i32.add
                        local.set 7
                        local.get 5
                        local.set 1
                        block ;; label = @11
                          block ;; label = @12
                            block ;; label = @13
                              loop ;; label = @14
                                local.get 1
                                i32.load
                                local.get 7
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
                            i32.load offset=12
                            i32.eqz
                            br_if 1 (;@11;)
                          end
                          local.get 5
                          local.set 1
                          block ;; label = @12
                            loop ;; label = @13
                              block ;; label = @14
                                local.get 1
                                i32.load
                                local.tee 7
                                local.get 3
                                i32.gt_u
                                br_if 0 (;@14;)
                                local.get 7
                                local.get 1
                                i32.load offset=4
                                i32.add
                                local.tee 7
                                local.get 3
                                i32.gt_u
                                br_if 2 (;@12;)
                              end
                              local.get 1
                              i32.load offset=8
                              local.set 1
                              br 0 (;@13;)
                            end
                          end
                          local.get 9
                          local.get 6
                          i32.const -40
                          i32.add
                          local.tee 1
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          local.get 0
                          local.get 9
                          i32.store offset=428
                          local.get 0
                          i32.const 2097152
                          i32.store offset=440
                          local.get 0
                          local.get 1
                          i32.store offset=420
                          local.get 9
                          local.get 1
                          i32.add
                          i32.const 40
                          i32.store offset=4
                          local.get 3
                          local.get 7
                          i32.const -32
                          i32.add
                          i32.const -8
                          i32.and
                          i32.const -8
                          i32.add
                          local.tee 1
                          local.get 1
                          local.get 3
                          i32.const 16
                          i32.add
                          i32.lt_u
                          select
                          local.tee 8
                          i32.const 27
                          i32.store offset=4
                          local.get 5
                          i64.load align=4
                          local.set 10
                          local.get 8
                          i32.const 16
                          i32.add
                          local.get 5
                          i32.const 8
                          i32.add
                          i64.load align=4
                          i64.store align=4
                          local.get 8
                          local.get 10
                          i64.store offset=8 align=4
                          local.get 0
                          i32.const 140
                          i32.add
                          i32.const 0
                          i32.store
                          local.get 0
                          i32.const 132
                          i32.add
                          local.get 6
                          i32.store
                          local.get 0
                          local.get 9
                          i32.store offset=128
                          local.get 0
                          i32.const 136
                          i32.add
                          local.get 8
                          i32.const 8
                          i32.add
                          i32.store
                          local.get 8
                          i32.const 28
                          i32.add
                          local.set 1
                          loop ;; label = @12
                            local.get 1
                            i32.const 7
                            i32.store
                            local.get 1
                            i32.const 4
                            i32.add
                            local.tee 1
                            local.get 7
                            i32.lt_u
                            br_if 0 (;@12;)
                          end
                          local.get 8
                          local.get 3
                          i32.eq
                          br_if 9 (;@2;)
                          local.get 8
                          local.get 8
                          i32.load offset=4
                          i32.const -2
                          i32.and
                          i32.store offset=4
                          local.get 3
                          local.get 8
                          local.get 3
                          i32.sub
                          local.tee 1
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          local.get 8
                          local.get 1
                          i32.store
                          block ;; label = @12
                            local.get 1
                            i32.const 256
                            i32.lt_u
                            br_if 0 (;@12;)
                            local.get 0
                            local.get 3
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
                          local.set 7
                          block ;; label = @12
                            block ;; label = @13
                              local.get 0
                              i32.load offset=408
                              local.tee 8
                              i32.const 1
                              local.get 1
                              i32.const 3
                              i32.shr_u
                              i32.shl
                              local.tee 1
                              i32.and
                              br_if 0 (;@13;)
                              local.get 0
                              local.get 8
                              local.get 1
                              i32.or
                              i32.store offset=408
                              local.get 7
                              local.set 1
                              br 1 (;@12;)
                            end
                            local.get 7
                            i32.load offset=8
                            local.set 1
                          end
                          local.get 7
                          local.get 3
                          i32.store offset=8
                          local.get 1
                          local.get 3
                          i32.store offset=12
                          local.get 3
                          local.get 7
                          i32.store offset=12
                          local.get 3
                          local.get 1
                          i32.store offset=8
                          br 9 (;@2;)
                        end
                        local.get 1
                        local.get 9
                        i32.store
                        local.get 1
                        local.get 1
                        i32.load offset=4
                        local.get 6
                        i32.add
                        i32.store offset=4
                        local.get 9
                        local.get 4
                        i32.const 3
                        i32.or
                        i32.store offset=4
                        local.get 7
                        local.get 9
                        local.get 4
                        i32.add
                        local.tee 1
                        i32.sub
                        local.set 4
                        local.get 7
                        local.get 0
                        i32.load offset=428
                        i32.eq
                        br_if 3 (;@7;)
                        local.get 7
                        local.get 0
                        i32.load offset=424
                        i32.eq
                        br_if 4 (;@6;)
                        block ;; label = @11
                          local.get 7
                          i32.load offset=4
                          local.tee 3
                          i32.const 3
                          i32.and
                          i32.const 1
                          i32.ne
                          br_if 0 (;@11;)
                          block ;; label = @12
                            block ;; label = @13
                              local.get 3
                              i32.const -8
                              i32.and
                              local.tee 8
                              i32.const 256
                              i32.lt_u
                              br_if 0 (;@13;)
                              local.get 0
                              local.get 7
                              call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_large_chunk
                              br 1 (;@12;)
                            end
                            block ;; label = @13
                              local.get 7
                              i32.load offset=12
                              local.tee 2
                              local.get 7
                              i32.load offset=8
                              local.tee 6
                              i32.eq
                              br_if 0 (;@13;)
                              local.get 6
                              local.get 2
                              i32.store offset=12
                              local.get 2
                              local.get 6
                              i32.store offset=8
                              br 1 (;@12;)
                            end
                            local.get 0
                            local.get 0
                            i32.load offset=408
                            i32.const -2
                            local.get 3
                            i32.const 3
                            i32.shr_u
                            i32.rotl
                            i32.and
                            i32.store offset=408
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
                        local.get 1
                        local.get 4
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        local.get 1
                        local.get 4
                        i32.add
                        local.get 4
                        i32.store
                        block ;; label = @11
                          local.get 4
                          i32.const 256
                          i32.lt_u
                          br_if 0 (;@11;)
                          local.get 0
                          local.get 1
                          local.get 4
                          call $dlmalloc::dlmalloc::Dlmalloc<A>::insert_large_chunk
                          br 8 (;@3;)
                        end
                        local.get 0
                        local.get 4
                        i32.const -8
                        i32.and
                        i32.add
                        i32.const 144
                        i32.add
                        local.set 3
                        block ;; label = @11
                          block ;; label = @12
                            local.get 0
                            i32.load offset=408
                            local.tee 7
                            i32.const 1
                            local.get 4
                            i32.const 3
                            i32.shr_u
                            i32.shl
                            local.tee 4
                            i32.and
                            br_if 0 (;@12;)
                            local.get 0
                            local.get 7
                            local.get 4
                            i32.or
                            i32.store offset=408
                            local.get 3
                            local.set 4
                            br 1 (;@11;)
                          end
                          local.get 3
                          i32.load offset=8
                          local.set 4
                        end
                        local.get 3
                        local.get 1
                        i32.store offset=8
                        local.get 4
                        local.get 1
                        i32.store offset=12
                        local.get 1
                        local.get 3
                        i32.store offset=12
                        local.get 1
                        local.get 4
                        i32.store offset=8
                        br 7 (;@3;)
                      end
                      local.get 0
                      local.get 1
                      local.get 4
                      i32.sub
                      local.tee 3
                      i32.store offset=420
                      local.get 0
                      local.get 0
                      i32.load offset=428
                      local.tee 1
                      local.get 4
                      i32.add
                      local.tee 7
                      i32.store offset=428
                      local.get 7
                      local.get 3
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 1
                      local.get 4
                      i32.const 3
                      i32.or
                      i32.store offset=4
                      local.get 1
                      i32.const 8
                      i32.add
                      local.set 2
                      br 8 (;@1;)
                    end
                    local.get 0
                    i32.load offset=424
                    local.set 3
                    local.get 1
                    local.get 4
                    i32.sub
                    local.tee 7
                    i32.const 16
                    i32.lt_u
                    br_if 3 (;@5;)
                    local.get 0
                    local.get 7
                    i32.store offset=416
                    local.get 0
                    local.get 3
                    local.get 4
                    i32.add
                    local.tee 8
                    i32.store offset=424
                    local.get 8
                    local.get 7
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    local.get 3
                    local.get 1
                    i32.add
                    local.get 7
                    i32.store
                    local.get 3
                    local.get 4
                    i32.const 3
                    i32.or
                    i32.store offset=4
                    br 4 (;@4;)
                  end
                  local.get 1
                  local.get 8
                  local.get 6
                  i32.add
                  i32.store offset=4
                  local.get 0
                  i32.load offset=428
                  local.tee 1
                  i32.const 15
                  i32.add
                  i32.const -8
                  i32.and
                  local.tee 3
                  i32.const -8
                  i32.add
                  local.tee 7
                  local.get 1
                  local.get 3
                  i32.sub
                  local.get 0
                  i32.load offset=420
                  local.get 6
                  i32.add
                  local.tee 3
                  i32.add
                  i32.const 8
                  i32.add
                  local.tee 8
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 0
                  i32.const 2097152
                  i32.store offset=440
                  local.get 0
                  local.get 7
                  i32.store offset=428
                  local.get 0
                  local.get 8
                  i32.store offset=420
                  local.get 1
                  local.get 3
                  i32.add
                  i32.const 40
                  i32.store offset=4
                  br 5 (;@2;)
                end
                local.get 0
                local.get 1
                i32.store offset=428
                local.get 0
                local.get 0
                i32.load offset=420
                local.get 4
                i32.add
                local.tee 4
                i32.store offset=420
                local.get 1
                local.get 4
                i32.const 1
                i32.or
                i32.store offset=4
                br 3 (;@3;)
              end
              local.get 1
              local.get 0
              i32.load offset=416
              local.get 4
              i32.add
              local.tee 4
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              local.get 1
              i32.store offset=424
              local.get 0
              local.get 4
              i32.store offset=416
              local.get 1
              local.get 4
              i32.add
              local.get 4
              i32.store
              br 2 (;@3;)
            end
            local.get 0
            i32.const 0
            i32.store offset=424
            local.get 0
            i32.const 0
            i32.store offset=416
            local.get 3
            local.get 1
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 3
            local.get 1
            i32.add
            local.tee 1
            local.get 1
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
          end
          local.get 3
          i32.const 8
          i32.add
          return
        end
        local.get 9
        i32.const 8
        i32.add
        return
      end
      local.get 0
      i32.load offset=420
      local.tee 1
      local.get 4
      i32.le_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      local.get 4
      i32.sub
      local.tee 3
      i32.store offset=420
      local.get 0
      local.get 0
      i32.load offset=428
      local.tee 1
      local.get 4
      i32.add
      local.tee 7
      i32.store offset=428
      local.get 7
      local.get 3
      i32.const 1
      i32.or
      i32.store offset=4
      local.get 1
      local.get 4
      i32.const 3
      i32.or
      i32.store offset=4
      local.get 1
      i32.const 8
      i32.add
      return
    end
    local.get 2
  )
  (func $dlmalloc::dlmalloc::Dlmalloc<A>::memalign (;5;) (type 3) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    i32.const 0
    local.set 3
    block ;; label = @1
      i32.const -65587
      local.get 1
      i32.const 16
      local.get 1
      i32.const 16
      i32.gt_u
      select
      local.tee 1
      i32.sub
      local.get 2
      i32.le_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.const 16
      local.get 2
      i32.const 11
      i32.add
      i32.const -8
      i32.and
      local.get 2
      i32.const 11
      i32.lt_u
      select
      local.tee 4
      i32.add
      i32.const 12
      i32.add
      call $dlmalloc::dlmalloc::Dlmalloc<A>::malloc
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i32.const -8
      i32.add
      local.set 3
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i32.const -1
          i32.add
          local.tee 5
          local.get 2
          i32.and
          br_if 0 (;@3;)
          local.get 3
          local.set 1
          br 1 (;@2;)
        end
        local.get 2
        i32.const -4
        i32.add
        local.tee 6
        i32.load
        local.tee 7
        i32.const -8
        i32.and
        local.get 5
        local.get 2
        i32.add
        i32.const 0
        local.get 1
        i32.sub
        i32.and
        i32.const -8
        i32.add
        local.tee 2
        i32.const 0
        local.get 1
        local.get 2
        local.get 3
        i32.sub
        i32.const 16
        i32.gt_u
        select
        i32.add
        local.tee 1
        local.get 3
        i32.sub
        local.tee 2
        i32.sub
        local.set 5
        block ;; label = @3
          local.get 7
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          local.get 5
          local.get 1
          i32.load offset=4
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store offset=4
          local.get 1
          local.get 5
          i32.add
          local.tee 5
          local.get 5
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 6
          local.get 2
          local.get 6
          i32.load
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get 3
          local.get 2
          i32.add
          local.tee 5
          local.get 5
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 0
          local.get 3
          local.get 2
          call $dlmalloc::dlmalloc::Dlmalloc<A>::dispose_chunk
          br 1 (;@2;)
        end
        local.get 3
        i32.load
        local.set 3
        local.get 1
        local.get 5
        i32.store offset=4
        local.get 1
        local.get 3
        local.get 2
        i32.add
        i32.store
      end
      block ;; label = @2
        local.get 1
        i32.load offset=4
        local.tee 2
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        i32.const -8
        i32.and
        local.tee 3
        local.get 4
        i32.const 16
        i32.add
        i32.le_u
        br_if 0 (;@2;)
        local.get 1
        local.get 4
        local.get 2
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store offset=4
        local.get 1
        local.get 4
        i32.add
        local.tee 2
        local.get 3
        local.get 4
        i32.sub
        local.tee 4
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 1
        local.get 3
        i32.add
        local.tee 3
        local.get 3
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 0
        local.get 2
        local.get 4
        call $dlmalloc::dlmalloc::Dlmalloc<A>::dispose_chunk
      end
      local.get 1
      i32.const 8
      i32.add
      local.set 3
    end
    local.get 3
  )
  (func $__main (;6;) (type 4) (result i32)
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
    i32.const 0
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
        i32.eqz
        br_if 0 (;@2;)
        i32.const 1048580
        local.get 1
        call $dlmalloc::dlmalloc::Dlmalloc<A>::free
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
  (func $__rust_realloc (;7;) (type 5) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 2
            i32.const 9
            i32.lt_u
            br_if 0 (;@4;)
            i32.const 1048580
            local.get 2
            local.get 3
            call $dlmalloc::dlmalloc::Dlmalloc<A>::memalign
            local.tee 2
            br_if 1 (;@3;)
            i32.const 0
            return
          end
          i32.const 0
          local.set 2
          local.get 3
          i32.const -65588
          i32.gt_u
          br_if 1 (;@2;)
          i32.const 16
          local.get 3
          i32.const 11
          i32.add
          i32.const -8
          i32.and
          local.get 3
          i32.const 11
          i32.lt_u
          select
          local.set 1
          local.get 0
          i32.const -4
          i32.add
          local.tee 4
          i32.load
          local.tee 5
          i32.const -8
          i32.and
          local.set 6
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          local.get 5
                          i32.const 3
                          i32.and
                          i32.eqz
                          br_if 0 (;@11;)
                          local.get 0
                          i32.const -8
                          i32.add
                          local.tee 7
                          local.get 6
                          i32.add
                          local.set 8
                          local.get 6
                          local.get 1
                          i32.ge_u
                          br_if 1 (;@10;)
                          local.get 8
                          i32.const 0
                          i32.load offset=1049008
                          i32.eq
                          br_if 6 (;@5;)
                          local.get 8
                          i32.const 0
                          i32.load offset=1049004
                          i32.eq
                          br_if 4 (;@7;)
                          local.get 8
                          i32.load offset=4
                          local.tee 5
                          i32.const 2
                          i32.and
                          br_if 7 (;@4;)
                          local.get 5
                          i32.const -8
                          i32.and
                          local.tee 9
                          local.get 6
                          i32.add
                          local.tee 6
                          local.get 1
                          i32.lt_u
                          br_if 7 (;@4;)
                          local.get 6
                          local.get 1
                          i32.sub
                          local.set 3
                          local.get 9
                          i32.const 256
                          i32.lt_u
                          br_if 2 (;@9;)
                          i32.const 1048580
                          local.get 8
                          call $dlmalloc::dlmalloc::Dlmalloc<A>::unlink_large_chunk
                          br 3 (;@8;)
                        end
                        local.get 1
                        i32.const 256
                        i32.lt_u
                        br_if 6 (;@4;)
                        local.get 6
                        local.get 1
                        i32.const 4
                        i32.or
                        i32.lt_u
                        br_if 6 (;@4;)
                        local.get 6
                        local.get 1
                        i32.sub
                        i32.const 131073
                        i32.ge_u
                        br_if 6 (;@4;)
                        local.get 0
                        return
                      end
                      local.get 6
                      local.get 1
                      i32.sub
                      local.tee 3
                      i32.const 16
                      i32.ge_u
                      br_if 3 (;@6;)
                      local.get 0
                      return
                    end
                    block ;; label = @9
                      local.get 8
                      i32.load offset=12
                      local.tee 2
                      local.get 8
                      i32.load offset=8
                      local.tee 8
                      i32.eq
                      br_if 0 (;@9;)
                      local.get 8
                      local.get 2
                      i32.store offset=12
                      local.get 2
                      local.get 8
                      i32.store offset=8
                      br 1 (;@8;)
                    end
                    i32.const 0
                    i32.const 0
                    i32.load offset=1048988
                    i32.const -2
                    local.get 5
                    i32.const 3
                    i32.shr_u
                    i32.rotl
                    i32.and
                    i32.store offset=1048988
                  end
                  block ;; label = @8
                    local.get 3
                    i32.const 16
                    i32.lt_u
                    br_if 0 (;@8;)
                    local.get 4
                    local.get 1
                    local.get 4
                    i32.load
                    i32.const 1
                    i32.and
                    i32.or
                    i32.const 2
                    i32.or
                    i32.store
                    local.get 7
                    local.get 1
                    i32.add
                    local.tee 2
                    local.get 3
                    i32.const 3
                    i32.or
                    i32.store offset=4
                    local.get 7
                    local.get 6
                    i32.add
                    local.tee 1
                    local.get 1
                    i32.load offset=4
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    i32.const 1048580
                    local.get 2
                    local.get 3
                    call $dlmalloc::dlmalloc::Dlmalloc<A>::dispose_chunk
                    local.get 0
                    return
                  end
                  local.get 4
                  local.get 6
                  local.get 4
                  i32.load
                  i32.const 1
                  i32.and
                  i32.or
                  i32.const 2
                  i32.or
                  i32.store
                  local.get 7
                  local.get 6
                  i32.add
                  local.tee 3
                  local.get 3
                  i32.load offset=4
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 0
                  return
                end
                i32.const 0
                i32.load offset=1048996
                local.get 6
                i32.add
                local.tee 6
                local.get 1
                i32.lt_u
                br_if 2 (;@4;)
                block ;; label = @7
                  block ;; label = @8
                    local.get 6
                    local.get 1
                    i32.sub
                    local.tee 3
                    i32.const 15
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 4
                    local.get 5
                    i32.const 1
                    i32.and
                    local.get 6
                    i32.or
                    i32.const 2
                    i32.or
                    i32.store
                    local.get 7
                    local.get 6
                    i32.add
                    local.tee 3
                    local.get 3
                    i32.load offset=4
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    i32.const 0
                    local.set 3
                    i32.const 0
                    local.set 2
                    br 1 (;@7;)
                  end
                  local.get 4
                  local.get 1
                  local.get 5
                  i32.const 1
                  i32.and
                  i32.or
                  i32.const 2
                  i32.or
                  i32.store
                  local.get 7
                  local.get 1
                  i32.add
                  local.tee 2
                  local.get 3
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 7
                  local.get 6
                  i32.add
                  local.tee 1
                  local.get 3
                  i32.store
                  local.get 1
                  local.get 1
                  i32.load offset=4
                  i32.const -2
                  i32.and
                  i32.store offset=4
                end
                i32.const 0
                local.get 2
                i32.store offset=1049004
                i32.const 0
                local.get 3
                i32.store offset=1048996
                local.get 0
                return
              end
              local.get 4
              local.get 1
              local.get 5
              i32.const 1
              i32.and
              i32.or
              i32.const 2
              i32.or
              i32.store
              local.get 7
              local.get 1
              i32.add
              local.tee 2
              local.get 3
              i32.const 3
              i32.or
              i32.store offset=4
              local.get 8
              local.get 8
              i32.load offset=4
              i32.const 1
              i32.or
              i32.store offset=4
              i32.const 1048580
              local.get 2
              local.get 3
              call $dlmalloc::dlmalloc::Dlmalloc<A>::dispose_chunk
              local.get 0
              return
            end
            i32.const 0
            i32.load offset=1049000
            local.get 6
            i32.add
            local.tee 6
            local.get 1
            i32.gt_u
            br_if 3 (;@1;)
          end
          i32.const 1048580
          local.get 3
          call $dlmalloc::dlmalloc::Dlmalloc<A>::malloc
          local.tee 1
          i32.eqz
          br_if 1 (;@2;)
          local.get 1
          local.get 0
          i32.const -4
          i32.const -8
          local.get 4
          i32.load
          local.tee 2
          i32.const 3
          i32.and
          select
          local.get 2
          i32.const -8
          i32.and
          i32.add
          local.tee 2
          local.get 3
          local.get 2
          local.get 3
          i32.lt_u
          select
          call $memcpy
          local.set 3
          i32.const 1048580
          local.get 0
          call $dlmalloc::dlmalloc::Dlmalloc<A>::free
          local.get 3
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
        drop
        i32.const 1048580
        local.get 0
        call $dlmalloc::dlmalloc::Dlmalloc<A>::free
      end
      local.get 2
      return
    end
    local.get 4
    local.get 1
    local.get 5
    i32.const 1
    i32.and
    i32.or
    i32.const 2
    i32.or
    i32.store
    i32.const 0
    local.get 7
    local.get 1
    i32.add
    local.tee 3
    i32.store offset=1049008
    i32.const 0
    local.get 6
    local.get 1
    i32.sub
    local.tee 2
    i32.store offset=1049000
    local.get 3
    local.get 2
    i32.const 1
    i32.or
    i32.store offset=4
    local.get 0
  )
  (func $alloc::raw_vec::finish_grow (;8;) (type 6) (param i32 i32 i32 i32)
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
              local.get 3
              i32.load offset=4
              i32.eqz
              br_if 0 (;@5;)
              block ;; label = @6
                local.get 3
                i32.const 8
                i32.add
                i32.load
                local.tee 4
                br_if 0 (;@6;)
                block ;; label = @7
                  local.get 2
                  br_if 0 (;@7;)
                  local.get 1
                  local.set 3
                  br 3 (;@4;)
                end
                i32.const 0
                i32.load8_u offset=1048576
                drop
                block ;; label = @7
                  local.get 1
                  i32.const 9
                  i32.lt_u
                  br_if 0 (;@7;)
                  i32.const 1048580
                  local.get 1
                  local.get 2
                  call $dlmalloc::dlmalloc::Dlmalloc<A>::memalign
                  local.set 3
                  br 3 (;@4;)
                end
                i32.const 1048580
                local.get 2
                call $dlmalloc::dlmalloc::Dlmalloc<A>::malloc
                local.set 3
                br 2 (;@4;)
              end
              local.get 3
              i32.load
              local.get 4
              local.get 1
              local.get 2
              call $__rust_realloc
              local.set 3
              br 1 (;@4;)
            end
            block ;; label = @5
              local.get 2
              br_if 0 (;@5;)
              local.get 1
              local.set 3
              br 1 (;@4;)
            end
            i32.const 0
            i32.load8_u offset=1048576
            drop
            block ;; label = @5
              local.get 1
              i32.const 9
              i32.lt_u
              br_if 0 (;@5;)
              i32.const 1048580
              local.get 1
              local.get 2
              call $dlmalloc::dlmalloc::Dlmalloc<A>::memalign
              local.set 3
              br 1 (;@4;)
            end
            i32.const 1048580
            local.get 2
            call $dlmalloc::dlmalloc::Dlmalloc<A>::malloc
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
  (func $alloc::raw_vec::RawVec<T,A>::reserve_for_push (;9;) (type 1) (param i32 i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        local.get 1
        i32.const 1
        i32.add
        local.tee 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.load offset=4
        local.tee 3
        i32.const 1
        i32.shl
        local.tee 4
        local.get 1
        local.get 4
        local.get 1
        i32.gt_u
        select
        local.tee 1
        i32.const 4
        local.get 1
        i32.const 4
        i32.gt_u
        select
        local.tee 1
        i32.const 2
        i32.shl
        local.set 4
        local.get 1
        i32.const 536870912
        i32.lt_u
        i32.const 2
        i32.shl
        local.set 5
        block ;; label = @3
          block ;; label = @4
            local.get 3
            br_if 0 (;@4;)
            local.get 2
            i32.const 0
            i32.store offset=24
            br 1 (;@3;)
          end
          local.get 2
          i32.const 4
          i32.store offset=24
          local.get 2
          local.get 3
          i32.const 2
          i32.shl
          i32.store offset=28
          local.get 2
          local.get 0
          i32.load
          i32.store offset=20
        end
        local.get 2
        i32.const 8
        i32.add
        local.get 5
        local.get 4
        local.get 2
        i32.const 20
        i32.add
        call $alloc::raw_vec::finish_grow
        local.get 2
        i32.load offset=12
        local.set 3
        block ;; label = @3
          local.get 2
          i32.load offset=8
          br_if 0 (;@3;)
          local.get 0
          local.get 1
          i32.store offset=4
          local.get 0
          local.get 3
          i32.store
          br 2 (;@1;)
        end
        local.get 3
        i32.const -2147483647
        i32.eq
        br_if 1 (;@1;)
      end
      unreachable
      unreachable
    end
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $vec_alloc (;10;) (type 4) (result i32)
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
    i32.const 0
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
        i32.eqz
        br_if 0 (;@2;)
        i32.const 1048580
        local.get 1
        call $dlmalloc::dlmalloc::Dlmalloc<A>::free
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
  (func $compiler_builtins::mem::memcpy (;11;) (type 3) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        local.get 2
        i32.const 16
        i32.ge_u
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
  (func $memcpy (;12;) (type 3) (param i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    call $compiler_builtins::mem::memcpy
  )
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