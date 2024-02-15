(component
  (core module (;0;)
    (type (;0;) (func))
    (type (;1;) (func (param i32 i32) (result i32)))
    (type (;2;) (func (param i32 i32 i32 i32) (result i32)))
    (type (;3;) (func (param i32) (result i32)))
    (type (;4;) (func (param i32)))
    (type (;5;) (func (param i32 i32)))
    (type (;6;) (func (param i32 i32 i32) (result i32)))
    (func $__wasm_call_ctors (;0;) (type 0))
    (func $miden:add/add@1.0.0#add (;1;) (type 1) (param i32 i32) (result i32)
      call $wit_bindgen::rt::run_ctors_once
      local.get 1
      local.get 0
      i32.add
    )
    (func $__rust_alloc (;2;) (type 1) (param i32 i32) (result i32)
      (local i32)
      local.get 0
      local.get 1
      call $__rdl_alloc
      local.set 2
      local.get 2
      return
    )
    (func $__rust_realloc (;3;) (type 2) (param i32 i32 i32 i32) (result i32)
      (local i32)
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      call $__rdl_realloc
      local.set 4
      local.get 4
      return
    )
    (func $wit_bindgen::rt::run_ctors_once (;4;) (type 0)
      block ;; label = @1
        i32.const 0
        i32.load8_u offset=1048577
        br_if 0 (;@1;)
        call $__wasm_call_ctors
        i32.const 0
        i32.const 1
        i32.store8 offset=1048577
      end
    )
    (func $cabi_realloc (;5;) (type 2) (param i32 i32 i32 i32) (result i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 1
            br_if 0 (;@3;)
            local.get 3
            i32.eqz
            br_if 2 (;@1;)
            i32.const 0
            i32.load8_u offset=1048576
            drop
            local.get 3
            local.get 2
            call $__rust_alloc
            local.set 2
            br 1 (;@2;)
          end
          local.get 0
          local.get 1
          local.get 2
          local.get 3
          call $__rust_realloc
          local.set 2
        end
        local.get 2
        br_if 0 (;@1;)
        unreachable
        unreachable
      end
      local.get 2
    )
    (func $__rdl_alloc (;6;) (type 1) (param i32 i32) (result i32)
      (local i32)
      block ;; label = @1
        block ;; label = @2
          local.get 1
          i32.const 8
          i32.gt_u
          br_if 0 (;@2;)
          local.get 1
          local.get 0
          i32.le_u
          br_if 1 (;@1;)
        end
        local.get 1
        local.get 1
        local.get 0
        local.get 1
        i32.rem_u
        local.tee 2
        i32.sub
        i32.const 0
        local.get 2
        select
        local.get 0
        i32.add
        call $aligned_alloc
        return
      end
      local.get 0
      call $malloc
    )
    (func $__rdl_realloc (;7;) (type 2) (param i32 i32 i32 i32) (result i32)
      (local i32 i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 2
            i32.const 8
            i32.gt_u
            br_if 0 (;@3;)
            local.get 2
            local.get 3
            i32.le_u
            br_if 1 (;@2;)
          end
          i32.const 0
          local.set 4
          local.get 2
          local.get 2
          local.get 3
          local.get 2
          i32.rem_u
          local.tee 5
          i32.sub
          i32.const 0
          local.get 5
          select
          local.get 3
          i32.add
          call $aligned_alloc
          local.tee 2
          i32.eqz
          br_if 1 (;@1;)
          local.get 2
          local.get 0
          local.get 1
          local.get 3
          local.get 1
          local.get 3
          i32.lt_u
          select
          call $memcpy
          local.set 2
          local.get 0
          call $free
          local.get 2
          return
        end
        local.get 0
        local.get 3
        call $realloc
        local.set 4
      end
      local.get 4
    )
    (func $malloc (;8;) (type 3) (param i32) (result i32)
      local.get 0
      call $dlmalloc
    )
    (func $dlmalloc (;9;) (type 3) (param i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    block ;; label = @8
                      block ;; label = @9
                        block ;; label = @10
                          block ;; label = @11
                            block ;; label = @12
                              i32.const 0
                              i32.load offset=1048604
                              local.tee 2
                              br_if 0 (;@12;)
                              block ;; label = @13
                                i32.const 0
                                i32.load offset=1049052
                                local.tee 3
                                br_if 0 (;@13;)
                                i32.const 0
                                i64.const -1
                                i64.store offset=1049064 align=4
                                i32.const 0
                                i64.const 281474976776192
                                i64.store offset=1049056 align=4
                                i32.const 0
                                local.get 1
                                i32.const 8
                                i32.add
                                i32.const -16
                                i32.and
                                i32.const 1431655768
                                i32.xor
                                local.tee 3
                                i32.store offset=1049052
                                i32.const 0
                                i32.const 0
                                i32.store offset=1049072
                                i32.const 0
                                i32.const 0
                                i32.store offset=1049024
                              end
                              i32.const 1114112
                              i32.const 1049088
                              i32.lt_u
                              br_if 1 (;@11;)
                              i32.const 0
                              local.set 2
                              i32.const 1114112
                              i32.const 1049088
                              i32.sub
                              i32.const 89
                              i32.lt_u
                              br_if 0 (;@12;)
                              i32.const 0
                              local.set 4
                              i32.const 0
                              i32.const 1049088
                              i32.store offset=1049028
                              i32.const 0
                              i32.const 1049088
                              i32.store offset=1048596
                              i32.const 0
                              local.get 3
                              i32.store offset=1048616
                              i32.const 0
                              i32.const -1
                              i32.store offset=1048612
                              i32.const 0
                              i32.const 1114112
                              i32.const 1049088
                              i32.sub
                              i32.store offset=1049032
                              loop ;; label = @13
                                local.get 4
                                i32.const 1048640
                                i32.add
                                local.get 4
                                i32.const 1048628
                                i32.add
                                local.tee 3
                                i32.store
                                local.get 3
                                local.get 4
                                i32.const 1048620
                                i32.add
                                local.tee 5
                                i32.store
                                local.get 4
                                i32.const 1048632
                                i32.add
                                local.get 5
                                i32.store
                                local.get 4
                                i32.const 1048648
                                i32.add
                                local.get 4
                                i32.const 1048636
                                i32.add
                                local.tee 5
                                i32.store
                                local.get 5
                                local.get 3
                                i32.store
                                local.get 4
                                i32.const 1048656
                                i32.add
                                local.get 4
                                i32.const 1048644
                                i32.add
                                local.tee 3
                                i32.store
                                local.get 3
                                local.get 5
                                i32.store
                                local.get 4
                                i32.const 1048652
                                i32.add
                                local.get 3
                                i32.store
                                local.get 4
                                i32.const 32
                                i32.add
                                local.tee 4
                                i32.const 256
                                i32.ne
                                br_if 0 (;@13;)
                              end
                              i32.const 1049088
                              i32.const -8
                              i32.const 1049088
                              i32.sub
                              i32.const 15
                              i32.and
                              i32.const 0
                              i32.const 1049088
                              i32.const 8
                              i32.add
                              i32.const 15
                              i32.and
                              select
                              local.tee 4
                              i32.add
                              local.tee 2
                              i32.const 4
                              i32.add
                              i32.const 1114112
                              i32.const 1049088
                              i32.sub
                              i32.const -56
                              i32.add
                              local.tee 3
                              local.get 4
                              i32.sub
                              local.tee 4
                              i32.const 1
                              i32.or
                              i32.store
                              i32.const 0
                              i32.const 0
                              i32.load offset=1049068
                              i32.store offset=1048608
                              i32.const 0
                              local.get 4
                              i32.store offset=1048592
                              i32.const 0
                              local.get 2
                              i32.store offset=1048604
                              local.get 3
                              i32.const 1049088
                              i32.add
                              i32.const 4
                              i32.add
                              i32.const 56
                              i32.store
                            end
                            block ;; label = @12
                              block ;; label = @13
                                local.get 0
                                i32.const 236
                                i32.gt_u
                                br_if 0 (;@13;)
                                block ;; label = @14
                                  i32.const 0
                                  i32.load offset=1048580
                                  local.tee 6
                                  i32.const 16
                                  local.get 0
                                  i32.const 19
                                  i32.add
                                  i32.const -16
                                  i32.and
                                  local.get 0
                                  i32.const 11
                                  i32.lt_u
                                  select
                                  local.tee 7
                                  i32.const 3
                                  i32.shr_u
                                  local.tee 3
                                  i32.shr_u
                                  local.tee 4
                                  i32.const 3
                                  i32.and
                                  i32.eqz
                                  br_if 0 (;@14;)
                                  block ;; label = @15
                                    block ;; label = @16
                                      local.get 4
                                      i32.const 1
                                      i32.and
                                      local.get 3
                                      i32.or
                                      i32.const 1
                                      i32.xor
                                      local.tee 5
                                      i32.const 3
                                      i32.shl
                                      local.tee 3
                                      i32.const 1048620
                                      i32.add
                                      local.tee 4
                                      local.get 3
                                      i32.const 1048628
                                      i32.add
                                      i32.load
                                      local.tee 3
                                      i32.load offset=8
                                      local.tee 7
                                      i32.ne
                                      br_if 0 (;@16;)
                                      i32.const 0
                                      local.get 6
                                      i32.const -2
                                      local.get 5
                                      i32.rotl
                                      i32.and
                                      i32.store offset=1048580
                                      br 1 (;@15;)
                                    end
                                    local.get 4
                                    local.get 7
                                    i32.store offset=8
                                    local.get 7
                                    local.get 4
                                    i32.store offset=12
                                  end
                                  local.get 3
                                  i32.const 8
                                  i32.add
                                  local.set 4
                                  local.get 3
                                  local.get 5
                                  i32.const 3
                                  i32.shl
                                  local.tee 5
                                  i32.const 3
                                  i32.or
                                  i32.store offset=4
                                  local.get 3
                                  local.get 5
                                  i32.add
                                  local.tee 3
                                  local.get 3
                                  i32.load offset=4
                                  i32.const 1
                                  i32.or
                                  i32.store offset=4
                                  br 13 (;@1;)
                                end
                                local.get 7
                                i32.const 0
                                i32.load offset=1048588
                                local.tee 8
                                i32.le_u
                                br_if 1 (;@12;)
                                block ;; label = @14
                                  local.get 4
                                  i32.eqz
                                  br_if 0 (;@14;)
                                  block ;; label = @15
                                    block ;; label = @16
                                      local.get 4
                                      local.get 3
                                      i32.shl
                                      i32.const 2
                                      local.get 3
                                      i32.shl
                                      local.tee 4
                                      i32.const 0
                                      local.get 4
                                      i32.sub
                                      i32.or
                                      i32.and
                                      local.tee 4
                                      i32.const 0
                                      local.get 4
                                      i32.sub
                                      i32.and
                                      i32.ctz
                                      local.tee 3
                                      i32.const 3
                                      i32.shl
                                      local.tee 4
                                      i32.const 1048620
                                      i32.add
                                      local.tee 5
                                      local.get 4
                                      i32.const 1048628
                                      i32.add
                                      i32.load
                                      local.tee 4
                                      i32.load offset=8
                                      local.tee 0
                                      i32.ne
                                      br_if 0 (;@16;)
                                      i32.const 0
                                      local.get 6
                                      i32.const -2
                                      local.get 3
                                      i32.rotl
                                      i32.and
                                      local.tee 6
                                      i32.store offset=1048580
                                      br 1 (;@15;)
                                    end
                                    local.get 5
                                    local.get 0
                                    i32.store offset=8
                                    local.get 0
                                    local.get 5
                                    i32.store offset=12
                                  end
                                  local.get 4
                                  local.get 7
                                  i32.const 3
                                  i32.or
                                  i32.store offset=4
                                  local.get 4
                                  local.get 3
                                  i32.const 3
                                  i32.shl
                                  local.tee 3
                                  i32.add
                                  local.get 3
                                  local.get 7
                                  i32.sub
                                  local.tee 5
                                  i32.store
                                  local.get 4
                                  local.get 7
                                  i32.add
                                  local.tee 0
                                  local.get 5
                                  i32.const 1
                                  i32.or
                                  i32.store offset=4
                                  block ;; label = @15
                                    local.get 8
                                    i32.eqz
                                    br_if 0 (;@15;)
                                    local.get 8
                                    i32.const -8
                                    i32.and
                                    i32.const 1048620
                                    i32.add
                                    local.set 7
                                    i32.const 0
                                    i32.load offset=1048600
                                    local.set 3
                                    block ;; label = @16
                                      block ;; label = @17
                                        local.get 6
                                        i32.const 1
                                        local.get 8
                                        i32.const 3
                                        i32.shr_u
                                        i32.shl
                                        local.tee 9
                                        i32.and
                                        br_if 0 (;@17;)
                                        i32.const 0
                                        local.get 6
                                        local.get 9
                                        i32.or
                                        i32.store offset=1048580
                                        local.get 7
                                        local.set 9
                                        br 1 (;@16;)
                                      end
                                      local.get 7
                                      i32.load offset=8
                                      local.set 9
                                    end
                                    local.get 9
                                    local.get 3
                                    i32.store offset=12
                                    local.get 7
                                    local.get 3
                                    i32.store offset=8
                                    local.get 3
                                    local.get 7
                                    i32.store offset=12
                                    local.get 3
                                    local.get 9
                                    i32.store offset=8
                                  end
                                  local.get 4
                                  i32.const 8
                                  i32.add
                                  local.set 4
                                  i32.const 0
                                  local.get 0
                                  i32.store offset=1048600
                                  i32.const 0
                                  local.get 5
                                  i32.store offset=1048588
                                  br 13 (;@1;)
                                end
                                i32.const 0
                                i32.load offset=1048584
                                local.tee 10
                                i32.eqz
                                br_if 1 (;@12;)
                                local.get 10
                                i32.const 0
                                local.get 10
                                i32.sub
                                i32.and
                                i32.ctz
                                i32.const 2
                                i32.shl
                                i32.const 1048884
                                i32.add
                                i32.load
                                local.tee 0
                                i32.load offset=4
                                i32.const -8
                                i32.and
                                local.get 7
                                i32.sub
                                local.set 3
                                local.get 0
                                local.set 5
                                block ;; label = @14
                                  loop ;; label = @15
                                    block ;; label = @16
                                      local.get 5
                                      i32.load offset=16
                                      local.tee 4
                                      br_if 0 (;@16;)
                                      local.get 5
                                      i32.const 20
                                      i32.add
                                      i32.load
                                      local.tee 4
                                      i32.eqz
                                      br_if 2 (;@14;)
                                    end
                                    local.get 4
                                    i32.load offset=4
                                    i32.const -8
                                    i32.and
                                    local.get 7
                                    i32.sub
                                    local.tee 5
                                    local.get 3
                                    local.get 5
                                    local.get 3
                                    i32.lt_u
                                    local.tee 5
                                    select
                                    local.set 3
                                    local.get 4
                                    local.get 0
                                    local.get 5
                                    select
                                    local.set 0
                                    local.get 4
                                    local.set 5
                                    br 0 (;@15;)
                                  end
                                end
                                local.get 0
                                i32.load offset=24
                                local.set 11
                                block ;; label = @14
                                  local.get 0
                                  i32.load offset=12
                                  local.tee 9
                                  local.get 0
                                  i32.eq
                                  br_if 0 (;@14;)
                                  local.get 0
                                  i32.load offset=8
                                  local.tee 4
                                  i32.const 0
                                  i32.load offset=1048596
                                  i32.lt_u
                                  drop
                                  local.get 9
                                  local.get 4
                                  i32.store offset=8
                                  local.get 4
                                  local.get 9
                                  i32.store offset=12
                                  br 12 (;@2;)
                                end
                                block ;; label = @14
                                  local.get 0
                                  i32.const 20
                                  i32.add
                                  local.tee 5
                                  i32.load
                                  local.tee 4
                                  br_if 0 (;@14;)
                                  local.get 0
                                  i32.load offset=16
                                  local.tee 4
                                  i32.eqz
                                  br_if 4 (;@10;)
                                  local.get 0
                                  i32.const 16
                                  i32.add
                                  local.set 5
                                end
                                loop ;; label = @14
                                  local.get 5
                                  local.set 2
                                  local.get 4
                                  local.tee 9
                                  i32.const 20
                                  i32.add
                                  local.tee 5
                                  i32.load
                                  local.tee 4
                                  br_if 0 (;@14;)
                                  local.get 9
                                  i32.const 16
                                  i32.add
                                  local.set 5
                                  local.get 9
                                  i32.load offset=16
                                  local.tee 4
                                  br_if 0 (;@14;)
                                end
                                local.get 2
                                i32.const 0
                                i32.store
                                br 11 (;@2;)
                              end
                              i32.const -1
                              local.set 7
                              local.get 0
                              i32.const -65
                              i32.gt_u
                              br_if 0 (;@12;)
                              local.get 0
                              i32.const 19
                              i32.add
                              local.tee 4
                              i32.const -16
                              i32.and
                              local.set 7
                              i32.const 0
                              i32.load offset=1048584
                              local.tee 10
                              i32.eqz
                              br_if 0 (;@12;)
                              i32.const 0
                              local.set 8
                              block ;; label = @13
                                local.get 7
                                i32.const 256
                                i32.lt_u
                                br_if 0 (;@13;)
                                i32.const 31
                                local.set 8
                                local.get 7
                                i32.const 16777215
                                i32.gt_u
                                br_if 0 (;@13;)
                                local.get 7
                                i32.const 38
                                local.get 4
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
                                local.set 8
                              end
                              i32.const 0
                              local.get 7
                              i32.sub
                              local.set 3
                              block ;; label = @13
                                block ;; label = @14
                                  block ;; label = @15
                                    block ;; label = @16
                                      local.get 8
                                      i32.const 2
                                      i32.shl
                                      i32.const 1048884
                                      i32.add
                                      i32.load
                                      local.tee 5
                                      br_if 0 (;@16;)
                                      i32.const 0
                                      local.set 4
                                      i32.const 0
                                      local.set 9
                                      br 1 (;@15;)
                                    end
                                    i32.const 0
                                    local.set 4
                                    local.get 7
                                    i32.const 0
                                    i32.const 25
                                    local.get 8
                                    i32.const 1
                                    i32.shr_u
                                    i32.sub
                                    local.get 8
                                    i32.const 31
                                    i32.eq
                                    select
                                    i32.shl
                                    local.set 0
                                    i32.const 0
                                    local.set 9
                                    loop ;; label = @16
                                      block ;; label = @17
                                        local.get 5
                                        i32.load offset=4
                                        i32.const -8
                                        i32.and
                                        local.get 7
                                        i32.sub
                                        local.tee 6
                                        local.get 3
                                        i32.ge_u
                                        br_if 0 (;@17;)
                                        local.get 6
                                        local.set 3
                                        local.get 5
                                        local.set 9
                                        local.get 6
                                        br_if 0 (;@17;)
                                        i32.const 0
                                        local.set 3
                                        local.get 5
                                        local.set 9
                                        local.get 5
                                        local.set 4
                                        br 3 (;@14;)
                                      end
                                      local.get 4
                                      local.get 5
                                      i32.const 20
                                      i32.add
                                      i32.load
                                      local.tee 6
                                      local.get 6
                                      local.get 5
                                      local.get 0
                                      i32.const 29
                                      i32.shr_u
                                      i32.const 4
                                      i32.and
                                      i32.add
                                      i32.const 16
                                      i32.add
                                      i32.load
                                      local.tee 5
                                      i32.eq
                                      select
                                      local.get 4
                                      local.get 6
                                      select
                                      local.set 4
                                      local.get 0
                                      i32.const 1
                                      i32.shl
                                      local.set 0
                                      local.get 5
                                      br_if 0 (;@16;)
                                    end
                                  end
                                  block ;; label = @15
                                    local.get 4
                                    local.get 9
                                    i32.or
                                    br_if 0 (;@15;)
                                    i32.const 0
                                    local.set 9
                                    i32.const 2
                                    local.get 8
                                    i32.shl
                                    local.tee 4
                                    i32.const 0
                                    local.get 4
                                    i32.sub
                                    i32.or
                                    local.get 10
                                    i32.and
                                    local.tee 4
                                    i32.eqz
                                    br_if 3 (;@12;)
                                    local.get 4
                                    i32.const 0
                                    local.get 4
                                    i32.sub
                                    i32.and
                                    i32.ctz
                                    i32.const 2
                                    i32.shl
                                    i32.const 1048884
                                    i32.add
                                    i32.load
                                    local.set 4
                                  end
                                  local.get 4
                                  i32.eqz
                                  br_if 1 (;@13;)
                                end
                                loop ;; label = @14
                                  local.get 4
                                  i32.load offset=4
                                  i32.const -8
                                  i32.and
                                  local.get 7
                                  i32.sub
                                  local.tee 6
                                  local.get 3
                                  i32.lt_u
                                  local.set 0
                                  block ;; label = @15
                                    local.get 4
                                    i32.load offset=16
                                    local.tee 5
                                    br_if 0 (;@15;)
                                    local.get 4
                                    i32.const 20
                                    i32.add
                                    i32.load
                                    local.set 5
                                  end
                                  local.get 6
                                  local.get 3
                                  local.get 0
                                  select
                                  local.set 3
                                  local.get 4
                                  local.get 9
                                  local.get 0
                                  select
                                  local.set 9
                                  local.get 5
                                  local.set 4
                                  local.get 5
                                  br_if 0 (;@14;)
                                end
                              end
                              local.get 9
                              i32.eqz
                              br_if 0 (;@12;)
                              local.get 3
                              i32.const 0
                              i32.load offset=1048588
                              local.get 7
                              i32.sub
                              i32.ge_u
                              br_if 0 (;@12;)
                              local.get 9
                              i32.load offset=24
                              local.set 2
                              block ;; label = @13
                                local.get 9
                                i32.load offset=12
                                local.tee 0
                                local.get 9
                                i32.eq
                                br_if 0 (;@13;)
                                local.get 9
                                i32.load offset=8
                                local.tee 4
                                i32.const 0
                                i32.load offset=1048596
                                i32.lt_u
                                drop
                                local.get 0
                                local.get 4
                                i32.store offset=8
                                local.get 4
                                local.get 0
                                i32.store offset=12
                                br 10 (;@3;)
                              end
                              block ;; label = @13
                                local.get 9
                                i32.const 20
                                i32.add
                                local.tee 5
                                i32.load
                                local.tee 4
                                br_if 0 (;@13;)
                                local.get 9
                                i32.load offset=16
                                local.tee 4
                                i32.eqz
                                br_if 4 (;@9;)
                                local.get 9
                                i32.const 16
                                i32.add
                                local.set 5
                              end
                              loop ;; label = @13
                                local.get 5
                                local.set 6
                                local.get 4
                                local.tee 0
                                i32.const 20
                                i32.add
                                local.tee 5
                                i32.load
                                local.tee 4
                                br_if 0 (;@13;)
                                local.get 0
                                i32.const 16
                                i32.add
                                local.set 5
                                local.get 0
                                i32.load offset=16
                                local.tee 4
                                br_if 0 (;@13;)
                              end
                              local.get 6
                              i32.const 0
                              i32.store
                              br 9 (;@3;)
                            end
                            block ;; label = @12
                              i32.const 0
                              i32.load offset=1048588
                              local.tee 4
                              local.get 7
                              i32.lt_u
                              br_if 0 (;@12;)
                              i32.const 0
                              i32.load offset=1048600
                              local.set 3
                              block ;; label = @13
                                block ;; label = @14
                                  local.get 4
                                  local.get 7
                                  i32.sub
                                  local.tee 5
                                  i32.const 16
                                  i32.lt_u
                                  br_if 0 (;@14;)
                                  local.get 3
                                  local.get 7
                                  i32.add
                                  local.tee 0
                                  local.get 5
                                  i32.const 1
                                  i32.or
                                  i32.store offset=4
                                  local.get 3
                                  local.get 4
                                  i32.add
                                  local.get 5
                                  i32.store
                                  local.get 3
                                  local.get 7
                                  i32.const 3
                                  i32.or
                                  i32.store offset=4
                                  br 1 (;@13;)
                                end
                                local.get 3
                                local.get 4
                                i32.const 3
                                i32.or
                                i32.store offset=4
                                local.get 3
                                local.get 4
                                i32.add
                                local.tee 4
                                local.get 4
                                i32.load offset=4
                                i32.const 1
                                i32.or
                                i32.store offset=4
                                i32.const 0
                                local.set 0
                                i32.const 0
                                local.set 5
                              end
                              i32.const 0
                              local.get 5
                              i32.store offset=1048588
                              i32.const 0
                              local.get 0
                              i32.store offset=1048600
                              local.get 3
                              i32.const 8
                              i32.add
                              local.set 4
                              br 11 (;@1;)
                            end
                            block ;; label = @12
                              i32.const 0
                              i32.load offset=1048592
                              local.tee 5
                              local.get 7
                              i32.le_u
                              br_if 0 (;@12;)
                              local.get 2
                              local.get 7
                              i32.add
                              local.tee 4
                              local.get 5
                              local.get 7
                              i32.sub
                              local.tee 3
                              i32.const 1
                              i32.or
                              i32.store offset=4
                              i32.const 0
                              local.get 4
                              i32.store offset=1048604
                              i32.const 0
                              local.get 3
                              i32.store offset=1048592
                              local.get 2
                              local.get 7
                              i32.const 3
                              i32.or
                              i32.store offset=4
                              local.get 2
                              i32.const 8
                              i32.add
                              local.set 4
                              br 11 (;@1;)
                            end
                            block ;; label = @12
                              block ;; label = @13
                                i32.const 0
                                i32.load offset=1049052
                                i32.eqz
                                br_if 0 (;@13;)
                                i32.const 0
                                i32.load offset=1049060
                                local.set 3
                                br 1 (;@12;)
                              end
                              i32.const 0
                              i64.const -1
                              i64.store offset=1049064 align=4
                              i32.const 0
                              i64.const 281474976776192
                              i64.store offset=1049056 align=4
                              i32.const 0
                              local.get 1
                              i32.const 12
                              i32.add
                              i32.const -16
                              i32.and
                              i32.const 1431655768
                              i32.xor
                              i32.store offset=1049052
                              i32.const 0
                              i32.const 0
                              i32.store offset=1049072
                              i32.const 0
                              i32.const 0
                              i32.store offset=1049024
                              i32.const 65536
                              local.set 3
                            end
                            i32.const 0
                            local.set 4
                            block ;; label = @12
                              local.get 3
                              local.get 7
                              i32.const 71
                              i32.add
                              local.tee 8
                              i32.add
                              local.tee 0
                              i32.const 0
                              local.get 3
                              i32.sub
                              local.tee 6
                              i32.and
                              local.tee 9
                              local.get 7
                              i32.gt_u
                              br_if 0 (;@12;)
                              i32.const 0
                              i32.const 48
                              i32.store offset=1049076
                              br 11 (;@1;)
                            end
                            block ;; label = @12
                              i32.const 0
                              i32.load offset=1049020
                              local.tee 4
                              i32.eqz
                              br_if 0 (;@12;)
                              block ;; label = @13
                                i32.const 0
                                i32.load offset=1049012
                                local.tee 3
                                local.get 9
                                i32.add
                                local.tee 10
                                local.get 3
                                i32.le_u
                                br_if 0 (;@13;)
                                local.get 10
                                local.get 4
                                i32.le_u
                                br_if 1 (;@12;)
                              end
                              i32.const 0
                              local.set 4
                              i32.const 0
                              i32.const 48
                              i32.store offset=1049076
                              br 11 (;@1;)
                            end
                            i32.const 0
                            i32.load8_u offset=1049024
                            i32.const 4
                            i32.and
                            br_if 5 (;@6;)
                            block ;; label = @12
                              block ;; label = @13
                                block ;; label = @14
                                  local.get 2
                                  i32.eqz
                                  br_if 0 (;@14;)
                                  i32.const 1049028
                                  local.set 4
                                  loop ;; label = @15
                                    block ;; label = @16
                                      local.get 4
                                      i32.load
                                      local.tee 3
                                      local.get 2
                                      i32.gt_u
                                      br_if 0 (;@16;)
                                      local.get 3
                                      local.get 4
                                      i32.load offset=4
                                      i32.add
                                      local.get 2
                                      i32.gt_u
                                      br_if 3 (;@13;)
                                    end
                                    local.get 4
                                    i32.load offset=8
                                    local.tee 4
                                    br_if 0 (;@15;)
                                  end
                                end
                                i32.const 0
                                call $sbrk
                                local.tee 0
                                i32.const -1
                                i32.eq
                                br_if 6 (;@7;)
                                local.get 9
                                local.set 6
                                block ;; label = @14
                                  i32.const 0
                                  i32.load offset=1049056
                                  local.tee 4
                                  i32.const -1
                                  i32.add
                                  local.tee 3
                                  local.get 0
                                  i32.and
                                  i32.eqz
                                  br_if 0 (;@14;)
                                  local.get 9
                                  local.get 0
                                  i32.sub
                                  local.get 3
                                  local.get 0
                                  i32.add
                                  i32.const 0
                                  local.get 4
                                  i32.sub
                                  i32.and
                                  i32.add
                                  local.set 6
                                end
                                local.get 6
                                local.get 7
                                i32.le_u
                                br_if 6 (;@7;)
                                local.get 6
                                i32.const 2147483646
                                i32.gt_u
                                br_if 6 (;@7;)
                                block ;; label = @14
                                  i32.const 0
                                  i32.load offset=1049020
                                  local.tee 4
                                  i32.eqz
                                  br_if 0 (;@14;)
                                  i32.const 0
                                  i32.load offset=1049012
                                  local.tee 3
                                  local.get 6
                                  i32.add
                                  local.tee 5
                                  local.get 3
                                  i32.le_u
                                  br_if 7 (;@7;)
                                  local.get 5
                                  local.get 4
                                  i32.gt_u
                                  br_if 7 (;@7;)
                                end
                                local.get 6
                                call $sbrk
                                local.tee 4
                                local.get 0
                                i32.ne
                                br_if 1 (;@12;)
                                br 8 (;@5;)
                              end
                              local.get 0
                              local.get 5
                              i32.sub
                              local.get 6
                              i32.and
                              local.tee 6
                              i32.const 2147483646
                              i32.gt_u
                              br_if 5 (;@7;)
                              local.get 6
                              call $sbrk
                              local.tee 0
                              local.get 4
                              i32.load
                              local.get 4
                              i32.load offset=4
                              i32.add
                              i32.eq
                              br_if 4 (;@8;)
                              local.get 0
                              local.set 4
                            end
                            block ;; label = @12
                              local.get 4
                              i32.const -1
                              i32.eq
                              br_if 0 (;@12;)
                              local.get 7
                              i32.const 72
                              i32.add
                              local.get 6
                              i32.le_u
                              br_if 0 (;@12;)
                              block ;; label = @13
                                local.get 8
                                local.get 6
                                i32.sub
                                i32.const 0
                                i32.load offset=1049060
                                local.tee 3
                                i32.add
                                i32.const 0
                                local.get 3
                                i32.sub
                                i32.and
                                local.tee 3
                                i32.const 2147483646
                                i32.le_u
                                br_if 0 (;@13;)
                                local.get 4
                                local.set 0
                                br 8 (;@5;)
                              end
                              block ;; label = @13
                                local.get 3
                                call $sbrk
                                i32.const -1
                                i32.eq
                                br_if 0 (;@13;)
                                local.get 3
                                local.get 6
                                i32.add
                                local.set 6
                                local.get 4
                                local.set 0
                                br 8 (;@5;)
                              end
                              i32.const 0
                              local.get 6
                              i32.sub
                              call $sbrk
                              drop
                              br 5 (;@7;)
                            end
                            local.get 4
                            local.set 0
                            local.get 4
                            i32.const -1
                            i32.ne
                            br_if 6 (;@5;)
                            br 4 (;@7;)
                          end
                          unreachable
                          unreachable
                        end
                        i32.const 0
                        local.set 9
                        br 7 (;@2;)
                      end
                      i32.const 0
                      local.set 0
                      br 5 (;@3;)
                    end
                    local.get 0
                    i32.const -1
                    i32.ne
                    br_if 2 (;@5;)
                  end
                  i32.const 0
                  i32.const 0
                  i32.load offset=1049024
                  i32.const 4
                  i32.or
                  i32.store offset=1049024
                end
                local.get 9
                i32.const 2147483646
                i32.gt_u
                br_if 1 (;@4;)
                local.get 9
                call $sbrk
                local.set 0
                i32.const 0
                call $sbrk
                local.set 4
                local.get 0
                i32.const -1
                i32.eq
                br_if 1 (;@4;)
                local.get 4
                i32.const -1
                i32.eq
                br_if 1 (;@4;)
                local.get 0
                local.get 4
                i32.ge_u
                br_if 1 (;@4;)
                local.get 4
                local.get 0
                i32.sub
                local.tee 6
                local.get 7
                i32.const 56
                i32.add
                i32.le_u
                br_if 1 (;@4;)
              end
              i32.const 0
              i32.const 0
              i32.load offset=1049012
              local.get 6
              i32.add
              local.tee 4
              i32.store offset=1049012
              block ;; label = @5
                local.get 4
                i32.const 0
                i32.load offset=1049016
                i32.le_u
                br_if 0 (;@5;)
                i32.const 0
                local.get 4
                i32.store offset=1049016
              end
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    block ;; label = @8
                      i32.const 0
                      i32.load offset=1048604
                      local.tee 3
                      i32.eqz
                      br_if 0 (;@8;)
                      i32.const 1049028
                      local.set 4
                      loop ;; label = @9
                        local.get 0
                        local.get 4
                        i32.load
                        local.tee 5
                        local.get 4
                        i32.load offset=4
                        local.tee 9
                        i32.add
                        i32.eq
                        br_if 2 (;@7;)
                        local.get 4
                        i32.load offset=8
                        local.tee 4
                        br_if 0 (;@9;)
                        br 3 (;@6;)
                      end
                    end
                    block ;; label = @8
                      block ;; label = @9
                        i32.const 0
                        i32.load offset=1048596
                        local.tee 4
                        i32.eqz
                        br_if 0 (;@9;)
                        local.get 0
                        local.get 4
                        i32.ge_u
                        br_if 1 (;@8;)
                      end
                      i32.const 0
                      local.get 0
                      i32.store offset=1048596
                    end
                    i32.const 0
                    local.set 4
                    i32.const 0
                    local.get 6
                    i32.store offset=1049032
                    i32.const 0
                    local.get 0
                    i32.store offset=1049028
                    i32.const 0
                    i32.const -1
                    i32.store offset=1048612
                    i32.const 0
                    i32.const 0
                    i32.load offset=1049052
                    i32.store offset=1048616
                    i32.const 0
                    i32.const 0
                    i32.store offset=1049040
                    loop ;; label = @8
                      local.get 4
                      i32.const 1048640
                      i32.add
                      local.get 4
                      i32.const 1048628
                      i32.add
                      local.tee 3
                      i32.store
                      local.get 3
                      local.get 4
                      i32.const 1048620
                      i32.add
                      local.tee 5
                      i32.store
                      local.get 4
                      i32.const 1048632
                      i32.add
                      local.get 5
                      i32.store
                      local.get 4
                      i32.const 1048648
                      i32.add
                      local.get 4
                      i32.const 1048636
                      i32.add
                      local.tee 5
                      i32.store
                      local.get 5
                      local.get 3
                      i32.store
                      local.get 4
                      i32.const 1048656
                      i32.add
                      local.get 4
                      i32.const 1048644
                      i32.add
                      local.tee 3
                      i32.store
                      local.get 3
                      local.get 5
                      i32.store
                      local.get 4
                      i32.const 1048652
                      i32.add
                      local.get 3
                      i32.store
                      local.get 4
                      i32.const 32
                      i32.add
                      local.tee 4
                      i32.const 256
                      i32.ne
                      br_if 0 (;@8;)
                    end
                    local.get 0
                    i32.const -8
                    local.get 0
                    i32.sub
                    i32.const 15
                    i32.and
                    i32.const 0
                    local.get 0
                    i32.const 8
                    i32.add
                    i32.const 15
                    i32.and
                    select
                    local.tee 4
                    i32.add
                    local.tee 3
                    local.get 6
                    i32.const -56
                    i32.add
                    local.tee 5
                    local.get 4
                    i32.sub
                    local.tee 4
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    i32.const 0
                    i32.const 0
                    i32.load offset=1049068
                    i32.store offset=1048608
                    i32.const 0
                    local.get 4
                    i32.store offset=1048592
                    i32.const 0
                    local.get 3
                    i32.store offset=1048604
                    local.get 0
                    local.get 5
                    i32.add
                    i32.const 56
                    i32.store offset=4
                    br 2 (;@5;)
                  end
                  local.get 4
                  i32.load8_u offset=12
                  i32.const 8
                  i32.and
                  br_if 0 (;@6;)
                  local.get 3
                  local.get 5
                  i32.lt_u
                  br_if 0 (;@6;)
                  local.get 3
                  local.get 0
                  i32.ge_u
                  br_if 0 (;@6;)
                  local.get 3
                  i32.const -8
                  local.get 3
                  i32.sub
                  i32.const 15
                  i32.and
                  i32.const 0
                  local.get 3
                  i32.const 8
                  i32.add
                  i32.const 15
                  i32.and
                  select
                  local.tee 5
                  i32.add
                  local.tee 0
                  i32.const 0
                  i32.load offset=1048592
                  local.get 6
                  i32.add
                  local.tee 2
                  local.get 5
                  i32.sub
                  local.tee 5
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 4
                  local.get 9
                  local.get 6
                  i32.add
                  i32.store offset=4
                  i32.const 0
                  i32.const 0
                  i32.load offset=1049068
                  i32.store offset=1048608
                  i32.const 0
                  local.get 5
                  i32.store offset=1048592
                  i32.const 0
                  local.get 0
                  i32.store offset=1048604
                  local.get 3
                  local.get 2
                  i32.add
                  i32.const 56
                  i32.store offset=4
                  br 1 (;@5;)
                end
                block ;; label = @6
                  local.get 0
                  i32.const 0
                  i32.load offset=1048596
                  local.tee 9
                  i32.ge_u
                  br_if 0 (;@6;)
                  i32.const 0
                  local.get 0
                  i32.store offset=1048596
                  local.get 0
                  local.set 9
                end
                local.get 0
                local.get 6
                i32.add
                local.set 5
                i32.const 1049028
                local.set 4
                block ;; label = @6
                  block ;; label = @7
                    block ;; label = @8
                      block ;; label = @9
                        block ;; label = @10
                          block ;; label = @11
                            block ;; label = @12
                              loop ;; label = @13
                                local.get 4
                                i32.load
                                local.get 5
                                i32.eq
                                br_if 1 (;@12;)
                                local.get 4
                                i32.load offset=8
                                local.tee 4
                                br_if 0 (;@13;)
                                br 2 (;@11;)
                              end
                            end
                            local.get 4
                            i32.load8_u offset=12
                            i32.const 8
                            i32.and
                            i32.eqz
                            br_if 1 (;@10;)
                          end
                          i32.const 1049028
                          local.set 4
                          loop ;; label = @11
                            block ;; label = @12
                              local.get 4
                              i32.load
                              local.tee 5
                              local.get 3
                              i32.gt_u
                              br_if 0 (;@12;)
                              local.get 5
                              local.get 4
                              i32.load offset=4
                              i32.add
                              local.tee 5
                              local.get 3
                              i32.gt_u
                              br_if 3 (;@9;)
                            end
                            local.get 4
                            i32.load offset=8
                            local.set 4
                            br 0 (;@11;)
                          end
                        end
                        local.get 4
                        local.get 0
                        i32.store
                        local.get 4
                        local.get 4
                        i32.load offset=4
                        local.get 6
                        i32.add
                        i32.store offset=4
                        local.get 0
                        i32.const -8
                        local.get 0
                        i32.sub
                        i32.const 15
                        i32.and
                        i32.const 0
                        local.get 0
                        i32.const 8
                        i32.add
                        i32.const 15
                        i32.and
                        select
                        i32.add
                        local.tee 2
                        local.get 7
                        i32.const 3
                        i32.or
                        i32.store offset=4
                        local.get 5
                        i32.const -8
                        local.get 5
                        i32.sub
                        i32.const 15
                        i32.and
                        i32.const 0
                        local.get 5
                        i32.const 8
                        i32.add
                        i32.const 15
                        i32.and
                        select
                        i32.add
                        local.tee 6
                        local.get 2
                        local.get 7
                        i32.add
                        local.tee 7
                        i32.sub
                        local.set 4
                        block ;; label = @10
                          local.get 6
                          local.get 3
                          i32.ne
                          br_if 0 (;@10;)
                          i32.const 0
                          local.get 7
                          i32.store offset=1048604
                          i32.const 0
                          i32.const 0
                          i32.load offset=1048592
                          local.get 4
                          i32.add
                          local.tee 4
                          i32.store offset=1048592
                          local.get 7
                          local.get 4
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          br 3 (;@7;)
                        end
                        block ;; label = @10
                          local.get 6
                          i32.const 0
                          i32.load offset=1048600
                          i32.ne
                          br_if 0 (;@10;)
                          i32.const 0
                          local.get 7
                          i32.store offset=1048600
                          i32.const 0
                          i32.const 0
                          i32.load offset=1048588
                          local.get 4
                          i32.add
                          local.tee 4
                          i32.store offset=1048588
                          local.get 7
                          local.get 4
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          local.get 7
                          local.get 4
                          i32.add
                          local.get 4
                          i32.store
                          br 3 (;@7;)
                        end
                        block ;; label = @10
                          local.get 6
                          i32.load offset=4
                          local.tee 3
                          i32.const 3
                          i32.and
                          i32.const 1
                          i32.ne
                          br_if 0 (;@10;)
                          local.get 3
                          i32.const -8
                          i32.and
                          local.set 8
                          block ;; label = @11
                            block ;; label = @12
                              local.get 3
                              i32.const 255
                              i32.gt_u
                              br_if 0 (;@12;)
                              local.get 6
                              i32.load offset=8
                              local.tee 5
                              local.get 3
                              i32.const 3
                              i32.shr_u
                              local.tee 9
                              i32.const 3
                              i32.shl
                              i32.const 1048620
                              i32.add
                              local.tee 0
                              i32.eq
                              drop
                              block ;; label = @13
                                local.get 6
                                i32.load offset=12
                                local.tee 3
                                local.get 5
                                i32.ne
                                br_if 0 (;@13;)
                                i32.const 0
                                i32.const 0
                                i32.load offset=1048580
                                i32.const -2
                                local.get 9
                                i32.rotl
                                i32.and
                                i32.store offset=1048580
                                br 2 (;@11;)
                              end
                              local.get 3
                              local.get 0
                              i32.eq
                              drop
                              local.get 3
                              local.get 5
                              i32.store offset=8
                              local.get 5
                              local.get 3
                              i32.store offset=12
                              br 1 (;@11;)
                            end
                            local.get 6
                            i32.load offset=24
                            local.set 10
                            block ;; label = @12
                              block ;; label = @13
                                local.get 6
                                i32.load offset=12
                                local.tee 0
                                local.get 6
                                i32.eq
                                br_if 0 (;@13;)
                                local.get 6
                                i32.load offset=8
                                local.tee 3
                                local.get 9
                                i32.lt_u
                                drop
                                local.get 0
                                local.get 3
                                i32.store offset=8
                                local.get 3
                                local.get 0
                                i32.store offset=12
                                br 1 (;@12;)
                              end
                              block ;; label = @13
                                local.get 6
                                i32.const 20
                                i32.add
                                local.tee 3
                                i32.load
                                local.tee 5
                                br_if 0 (;@13;)
                                local.get 6
                                i32.const 16
                                i32.add
                                local.tee 3
                                i32.load
                                local.tee 5
                                br_if 0 (;@13;)
                                i32.const 0
                                local.set 0
                                br 1 (;@12;)
                              end
                              loop ;; label = @13
                                local.get 3
                                local.set 9
                                local.get 5
                                local.tee 0
                                i32.const 20
                                i32.add
                                local.tee 3
                                i32.load
                                local.tee 5
                                br_if 0 (;@13;)
                                local.get 0
                                i32.const 16
                                i32.add
                                local.set 3
                                local.get 0
                                i32.load offset=16
                                local.tee 5
                                br_if 0 (;@13;)
                              end
                              local.get 9
                              i32.const 0
                              i32.store
                            end
                            local.get 10
                            i32.eqz
                            br_if 0 (;@11;)
                            block ;; label = @12
                              block ;; label = @13
                                local.get 6
                                local.get 6
                                i32.load offset=28
                                local.tee 5
                                i32.const 2
                                i32.shl
                                i32.const 1048884
                                i32.add
                                local.tee 3
                                i32.load
                                i32.ne
                                br_if 0 (;@13;)
                                local.get 3
                                local.get 0
                                i32.store
                                local.get 0
                                br_if 1 (;@12;)
                                i32.const 0
                                i32.const 0
                                i32.load offset=1048584
                                i32.const -2
                                local.get 5
                                i32.rotl
                                i32.and
                                i32.store offset=1048584
                                br 2 (;@11;)
                              end
                              local.get 10
                              i32.const 16
                              i32.const 20
                              local.get 10
                              i32.load offset=16
                              local.get 6
                              i32.eq
                              select
                              i32.add
                              local.get 0
                              i32.store
                              local.get 0
                              i32.eqz
                              br_if 1 (;@11;)
                            end
                            local.get 0
                            local.get 10
                            i32.store offset=24
                            block ;; label = @12
                              local.get 6
                              i32.load offset=16
                              local.tee 3
                              i32.eqz
                              br_if 0 (;@12;)
                              local.get 0
                              local.get 3
                              i32.store offset=16
                              local.get 3
                              local.get 0
                              i32.store offset=24
                            end
                            local.get 6
                            i32.load offset=20
                            local.tee 3
                            i32.eqz
                            br_if 0 (;@11;)
                            local.get 0
                            i32.const 20
                            i32.add
                            local.get 3
                            i32.store
                            local.get 3
                            local.get 0
                            i32.store offset=24
                          end
                          local.get 8
                          local.get 4
                          i32.add
                          local.set 4
                          local.get 6
                          local.get 8
                          i32.add
                          local.tee 6
                          i32.load offset=4
                          local.set 3
                        end
                        local.get 6
                        local.get 3
                        i32.const -2
                        i32.and
                        i32.store offset=4
                        local.get 7
                        local.get 4
                        i32.add
                        local.get 4
                        i32.store
                        local.get 7
                        local.get 4
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        block ;; label = @10
                          local.get 4
                          i32.const 255
                          i32.gt_u
                          br_if 0 (;@10;)
                          local.get 4
                          i32.const -8
                          i32.and
                          i32.const 1048620
                          i32.add
                          local.set 3
                          block ;; label = @11
                            block ;; label = @12
                              i32.const 0
                              i32.load offset=1048580
                              local.tee 5
                              i32.const 1
                              local.get 4
                              i32.const 3
                              i32.shr_u
                              i32.shl
                              local.tee 4
                              i32.and
                              br_if 0 (;@12;)
                              i32.const 0
                              local.get 5
                              local.get 4
                              i32.or
                              i32.store offset=1048580
                              local.get 3
                              local.set 4
                              br 1 (;@11;)
                            end
                            local.get 3
                            i32.load offset=8
                            local.set 4
                          end
                          local.get 4
                          local.get 7
                          i32.store offset=12
                          local.get 3
                          local.get 7
                          i32.store offset=8
                          local.get 7
                          local.get 3
                          i32.store offset=12
                          local.get 7
                          local.get 4
                          i32.store offset=8
                          br 3 (;@7;)
                        end
                        i32.const 31
                        local.set 3
                        block ;; label = @10
                          local.get 4
                          i32.const 16777215
                          i32.gt_u
                          br_if 0 (;@10;)
                          local.get 4
                          i32.const 38
                          local.get 4
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
                        local.get 7
                        local.get 3
                        i32.store offset=28
                        local.get 7
                        i64.const 0
                        i64.store offset=16 align=4
                        local.get 3
                        i32.const 2
                        i32.shl
                        i32.const 1048884
                        i32.add
                        local.set 5
                        block ;; label = @10
                          i32.const 0
                          i32.load offset=1048584
                          local.tee 0
                          i32.const 1
                          local.get 3
                          i32.shl
                          local.tee 9
                          i32.and
                          br_if 0 (;@10;)
                          local.get 5
                          local.get 7
                          i32.store
                          i32.const 0
                          local.get 0
                          local.get 9
                          i32.or
                          i32.store offset=1048584
                          local.get 7
                          local.get 5
                          i32.store offset=24
                          local.get 7
                          local.get 7
                          i32.store offset=8
                          local.get 7
                          local.get 7
                          i32.store offset=12
                          br 3 (;@7;)
                        end
                        local.get 4
                        i32.const 0
                        i32.const 25
                        local.get 3
                        i32.const 1
                        i32.shr_u
                        i32.sub
                        local.get 3
                        i32.const 31
                        i32.eq
                        select
                        i32.shl
                        local.set 3
                        local.get 5
                        i32.load
                        local.set 0
                        loop ;; label = @10
                          local.get 0
                          local.tee 5
                          i32.load offset=4
                          i32.const -8
                          i32.and
                          local.get 4
                          i32.eq
                          br_if 2 (;@8;)
                          local.get 3
                          i32.const 29
                          i32.shr_u
                          local.set 0
                          local.get 3
                          i32.const 1
                          i32.shl
                          local.set 3
                          local.get 5
                          local.get 0
                          i32.const 4
                          i32.and
                          i32.add
                          i32.const 16
                          i32.add
                          local.tee 9
                          i32.load
                          local.tee 0
                          br_if 0 (;@10;)
                        end
                        local.get 9
                        local.get 7
                        i32.store
                        local.get 7
                        local.get 5
                        i32.store offset=24
                        local.get 7
                        local.get 7
                        i32.store offset=12
                        local.get 7
                        local.get 7
                        i32.store offset=8
                        br 2 (;@7;)
                      end
                      local.get 0
                      i32.const -8
                      local.get 0
                      i32.sub
                      i32.const 15
                      i32.and
                      i32.const 0
                      local.get 0
                      i32.const 8
                      i32.add
                      i32.const 15
                      i32.and
                      select
                      local.tee 4
                      i32.add
                      local.tee 2
                      local.get 6
                      i32.const -56
                      i32.add
                      local.tee 9
                      local.get 4
                      i32.sub
                      local.tee 4
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 0
                      local.get 9
                      i32.add
                      i32.const 56
                      i32.store offset=4
                      local.get 3
                      local.get 5
                      i32.const 55
                      local.get 5
                      i32.sub
                      i32.const 15
                      i32.and
                      i32.const 0
                      local.get 5
                      i32.const -55
                      i32.add
                      i32.const 15
                      i32.and
                      select
                      i32.add
                      i32.const -63
                      i32.add
                      local.tee 9
                      local.get 9
                      local.get 3
                      i32.const 16
                      i32.add
                      i32.lt_u
                      select
                      local.tee 9
                      i32.const 35
                      i32.store offset=4
                      i32.const 0
                      i32.const 0
                      i32.load offset=1049068
                      i32.store offset=1048608
                      i32.const 0
                      local.get 4
                      i32.store offset=1048592
                      i32.const 0
                      local.get 2
                      i32.store offset=1048604
                      local.get 9
                      i32.const 16
                      i32.add
                      i32.const 0
                      i64.load offset=1049036 align=4
                      i64.store align=4
                      local.get 9
                      i32.const 0
                      i64.load offset=1049028 align=4
                      i64.store offset=8 align=4
                      i32.const 0
                      local.get 9
                      i32.const 8
                      i32.add
                      i32.store offset=1049036
                      i32.const 0
                      local.get 6
                      i32.store offset=1049032
                      i32.const 0
                      local.get 0
                      i32.store offset=1049028
                      i32.const 0
                      i32.const 0
                      i32.store offset=1049040
                      local.get 9
                      i32.const 36
                      i32.add
                      local.set 4
                      loop ;; label = @9
                        local.get 4
                        i32.const 7
                        i32.store
                        local.get 4
                        i32.const 4
                        i32.add
                        local.tee 4
                        local.get 5
                        i32.lt_u
                        br_if 0 (;@9;)
                      end
                      local.get 9
                      local.get 3
                      i32.eq
                      br_if 3 (;@5;)
                      local.get 9
                      local.get 9
                      i32.load offset=4
                      i32.const -2
                      i32.and
                      i32.store offset=4
                      local.get 9
                      local.get 9
                      local.get 3
                      i32.sub
                      local.tee 0
                      i32.store
                      local.get 3
                      local.get 0
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      block ;; label = @9
                        local.get 0
                        i32.const 255
                        i32.gt_u
                        br_if 0 (;@9;)
                        local.get 0
                        i32.const -8
                        i32.and
                        i32.const 1048620
                        i32.add
                        local.set 4
                        block ;; label = @10
                          block ;; label = @11
                            i32.const 0
                            i32.load offset=1048580
                            local.tee 5
                            i32.const 1
                            local.get 0
                            i32.const 3
                            i32.shr_u
                            i32.shl
                            local.tee 0
                            i32.and
                            br_if 0 (;@11;)
                            i32.const 0
                            local.get 5
                            local.get 0
                            i32.or
                            i32.store offset=1048580
                            local.get 4
                            local.set 5
                            br 1 (;@10;)
                          end
                          local.get 4
                          i32.load offset=8
                          local.set 5
                        end
                        local.get 5
                        local.get 3
                        i32.store offset=12
                        local.get 4
                        local.get 3
                        i32.store offset=8
                        local.get 3
                        local.get 4
                        i32.store offset=12
                        local.get 3
                        local.get 5
                        i32.store offset=8
                        br 4 (;@5;)
                      end
                      i32.const 31
                      local.set 4
                      block ;; label = @9
                        local.get 0
                        i32.const 16777215
                        i32.gt_u
                        br_if 0 (;@9;)
                        local.get 0
                        i32.const 38
                        local.get 0
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
                        local.set 4
                      end
                      local.get 3
                      local.get 4
                      i32.store offset=28
                      local.get 3
                      i64.const 0
                      i64.store offset=16 align=4
                      local.get 4
                      i32.const 2
                      i32.shl
                      i32.const 1048884
                      i32.add
                      local.set 5
                      block ;; label = @9
                        i32.const 0
                        i32.load offset=1048584
                        local.tee 9
                        i32.const 1
                        local.get 4
                        i32.shl
                        local.tee 6
                        i32.and
                        br_if 0 (;@9;)
                        local.get 5
                        local.get 3
                        i32.store
                        i32.const 0
                        local.get 9
                        local.get 6
                        i32.or
                        i32.store offset=1048584
                        local.get 3
                        local.get 5
                        i32.store offset=24
                        local.get 3
                        local.get 3
                        i32.store offset=8
                        local.get 3
                        local.get 3
                        i32.store offset=12
                        br 4 (;@5;)
                      end
                      local.get 0
                      i32.const 0
                      i32.const 25
                      local.get 4
                      i32.const 1
                      i32.shr_u
                      i32.sub
                      local.get 4
                      i32.const 31
                      i32.eq
                      select
                      i32.shl
                      local.set 4
                      local.get 5
                      i32.load
                      local.set 9
                      loop ;; label = @9
                        local.get 9
                        local.tee 5
                        i32.load offset=4
                        i32.const -8
                        i32.and
                        local.get 0
                        i32.eq
                        br_if 3 (;@6;)
                        local.get 4
                        i32.const 29
                        i32.shr_u
                        local.set 9
                        local.get 4
                        i32.const 1
                        i32.shl
                        local.set 4
                        local.get 5
                        local.get 9
                        i32.const 4
                        i32.and
                        i32.add
                        i32.const 16
                        i32.add
                        local.tee 6
                        i32.load
                        local.tee 9
                        br_if 0 (;@9;)
                      end
                      local.get 6
                      local.get 3
                      i32.store
                      local.get 3
                      local.get 5
                      i32.store offset=24
                      local.get 3
                      local.get 3
                      i32.store offset=12
                      local.get 3
                      local.get 3
                      i32.store offset=8
                      br 3 (;@5;)
                    end
                    local.get 5
                    i32.load offset=8
                    local.tee 4
                    local.get 7
                    i32.store offset=12
                    local.get 5
                    local.get 7
                    i32.store offset=8
                    local.get 7
                    i32.const 0
                    i32.store offset=24
                    local.get 7
                    local.get 5
                    i32.store offset=12
                    local.get 7
                    local.get 4
                    i32.store offset=8
                  end
                  local.get 2
                  i32.const 8
                  i32.add
                  local.set 4
                  br 5 (;@1;)
                end
                local.get 5
                i32.load offset=8
                local.tee 4
                local.get 3
                i32.store offset=12
                local.get 5
                local.get 3
                i32.store offset=8
                local.get 3
                i32.const 0
                i32.store offset=24
                local.get 3
                local.get 5
                i32.store offset=12
                local.get 3
                local.get 4
                i32.store offset=8
              end
              i32.const 0
              i32.load offset=1048592
              local.tee 4
              local.get 7
              i32.le_u
              br_if 0 (;@4;)
              i32.const 0
              i32.load offset=1048604
              local.tee 3
              local.get 7
              i32.add
              local.tee 5
              local.get 4
              local.get 7
              i32.sub
              local.tee 4
              i32.const 1
              i32.or
              i32.store offset=4
              i32.const 0
              local.get 4
              i32.store offset=1048592
              i32.const 0
              local.get 5
              i32.store offset=1048604
              local.get 3
              local.get 7
              i32.const 3
              i32.or
              i32.store offset=4
              local.get 3
              i32.const 8
              i32.add
              local.set 4
              br 3 (;@1;)
            end
            i32.const 0
            local.set 4
            i32.const 0
            i32.const 48
            i32.store offset=1049076
            br 2 (;@1;)
          end
          block ;; label = @3
            local.get 2
            i32.eqz
            br_if 0 (;@3;)
            block ;; label = @4
              block ;; label = @5
                local.get 9
                local.get 9
                i32.load offset=28
                local.tee 5
                i32.const 2
                i32.shl
                i32.const 1048884
                i32.add
                local.tee 4
                i32.load
                i32.ne
                br_if 0 (;@5;)
                local.get 4
                local.get 0
                i32.store
                local.get 0
                br_if 1 (;@4;)
                i32.const 0
                local.get 10
                i32.const -2
                local.get 5
                i32.rotl
                i32.and
                local.tee 10
                i32.store offset=1048584
                br 2 (;@3;)
              end
              local.get 2
              i32.const 16
              i32.const 20
              local.get 2
              i32.load offset=16
              local.get 9
              i32.eq
              select
              i32.add
              local.get 0
              i32.store
              local.get 0
              i32.eqz
              br_if 1 (;@3;)
            end
            local.get 0
            local.get 2
            i32.store offset=24
            block ;; label = @4
              local.get 9
              i32.load offset=16
              local.tee 4
              i32.eqz
              br_if 0 (;@4;)
              local.get 0
              local.get 4
              i32.store offset=16
              local.get 4
              local.get 0
              i32.store offset=24
            end
            local.get 9
            i32.const 20
            i32.add
            i32.load
            local.tee 4
            i32.eqz
            br_if 0 (;@3;)
            local.get 0
            i32.const 20
            i32.add
            local.get 4
            i32.store
            local.get 4
            local.get 0
            i32.store offset=24
          end
          block ;; label = @3
            block ;; label = @4
              local.get 3
              i32.const 15
              i32.gt_u
              br_if 0 (;@4;)
              local.get 9
              local.get 3
              local.get 7
              i32.add
              local.tee 4
              i32.const 3
              i32.or
              i32.store offset=4
              local.get 9
              local.get 4
              i32.add
              local.tee 4
              local.get 4
              i32.load offset=4
              i32.const 1
              i32.or
              i32.store offset=4
              br 1 (;@3;)
            end
            local.get 9
            local.get 7
            i32.add
            local.tee 0
            local.get 3
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 9
            local.get 7
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 0
            local.get 3
            i32.add
            local.get 3
            i32.store
            block ;; label = @4
              local.get 3
              i32.const 255
              i32.gt_u
              br_if 0 (;@4;)
              local.get 3
              i32.const -8
              i32.and
              i32.const 1048620
              i32.add
              local.set 4
              block ;; label = @5
                block ;; label = @6
                  i32.const 0
                  i32.load offset=1048580
                  local.tee 5
                  i32.const 1
                  local.get 3
                  i32.const 3
                  i32.shr_u
                  i32.shl
                  local.tee 3
                  i32.and
                  br_if 0 (;@6;)
                  i32.const 0
                  local.get 5
                  local.get 3
                  i32.or
                  i32.store offset=1048580
                  local.get 4
                  local.set 3
                  br 1 (;@5;)
                end
                local.get 4
                i32.load offset=8
                local.set 3
              end
              local.get 3
              local.get 0
              i32.store offset=12
              local.get 4
              local.get 0
              i32.store offset=8
              local.get 0
              local.get 4
              i32.store offset=12
              local.get 0
              local.get 3
              i32.store offset=8
              br 1 (;@3;)
            end
            i32.const 31
            local.set 4
            block ;; label = @4
              local.get 3
              i32.const 16777215
              i32.gt_u
              br_if 0 (;@4;)
              local.get 3
              i32.const 38
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
              local.set 4
            end
            local.get 0
            local.get 4
            i32.store offset=28
            local.get 0
            i64.const 0
            i64.store offset=16 align=4
            local.get 4
            i32.const 2
            i32.shl
            i32.const 1048884
            i32.add
            local.set 5
            block ;; label = @4
              local.get 10
              i32.const 1
              local.get 4
              i32.shl
              local.tee 7
              i32.and
              br_if 0 (;@4;)
              local.get 5
              local.get 0
              i32.store
              i32.const 0
              local.get 10
              local.get 7
              i32.or
              i32.store offset=1048584
              local.get 0
              local.get 5
              i32.store offset=24
              local.get 0
              local.get 0
              i32.store offset=8
              local.get 0
              local.get 0
              i32.store offset=12
              br 1 (;@3;)
            end
            local.get 3
            i32.const 0
            i32.const 25
            local.get 4
            i32.const 1
            i32.shr_u
            i32.sub
            local.get 4
            i32.const 31
            i32.eq
            select
            i32.shl
            local.set 4
            local.get 5
            i32.load
            local.set 7
            block ;; label = @4
              loop ;; label = @5
                local.get 7
                local.tee 5
                i32.load offset=4
                i32.const -8
                i32.and
                local.get 3
                i32.eq
                br_if 1 (;@4;)
                local.get 4
                i32.const 29
                i32.shr_u
                local.set 7
                local.get 4
                i32.const 1
                i32.shl
                local.set 4
                local.get 5
                local.get 7
                i32.const 4
                i32.and
                i32.add
                i32.const 16
                i32.add
                local.tee 6
                i32.load
                local.tee 7
                br_if 0 (;@5;)
              end
              local.get 6
              local.get 0
              i32.store
              local.get 0
              local.get 5
              i32.store offset=24
              local.get 0
              local.get 0
              i32.store offset=12
              local.get 0
              local.get 0
              i32.store offset=8
              br 1 (;@3;)
            end
            local.get 5
            i32.load offset=8
            local.tee 4
            local.get 0
            i32.store offset=12
            local.get 5
            local.get 0
            i32.store offset=8
            local.get 0
            i32.const 0
            i32.store offset=24
            local.get 0
            local.get 5
            i32.store offset=12
            local.get 0
            local.get 4
            i32.store offset=8
          end
          local.get 9
          i32.const 8
          i32.add
          local.set 4
          br 1 (;@1;)
        end
        block ;; label = @2
          local.get 11
          i32.eqz
          br_if 0 (;@2;)
          block ;; label = @3
            block ;; label = @4
              local.get 0
              local.get 0
              i32.load offset=28
              local.tee 5
              i32.const 2
              i32.shl
              i32.const 1048884
              i32.add
              local.tee 4
              i32.load
              i32.ne
              br_if 0 (;@4;)
              local.get 4
              local.get 9
              i32.store
              local.get 9
              br_if 1 (;@3;)
              i32.const 0
              local.get 10
              i32.const -2
              local.get 5
              i32.rotl
              i32.and
              i32.store offset=1048584
              br 2 (;@2;)
            end
            local.get 11
            i32.const 16
            i32.const 20
            local.get 11
            i32.load offset=16
            local.get 0
            i32.eq
            select
            i32.add
            local.get 9
            i32.store
            local.get 9
            i32.eqz
            br_if 1 (;@2;)
          end
          local.get 9
          local.get 11
          i32.store offset=24
          block ;; label = @3
            local.get 0
            i32.load offset=16
            local.tee 4
            i32.eqz
            br_if 0 (;@3;)
            local.get 9
            local.get 4
            i32.store offset=16
            local.get 4
            local.get 9
            i32.store offset=24
          end
          local.get 0
          i32.const 20
          i32.add
          i32.load
          local.tee 4
          i32.eqz
          br_if 0 (;@2;)
          local.get 9
          i32.const 20
          i32.add
          local.get 4
          i32.store
          local.get 4
          local.get 9
          i32.store offset=24
        end
        block ;; label = @2
          block ;; label = @3
            local.get 3
            i32.const 15
            i32.gt_u
            br_if 0 (;@3;)
            local.get 0
            local.get 3
            local.get 7
            i32.add
            local.tee 4
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 0
            local.get 4
            i32.add
            local.tee 4
            local.get 4
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            br 1 (;@2;)
          end
          local.get 0
          local.get 7
          i32.add
          local.tee 5
          local.get 3
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 0
          local.get 7
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 5
          local.get 3
          i32.add
          local.get 3
          i32.store
          block ;; label = @3
            local.get 8
            i32.eqz
            br_if 0 (;@3;)
            local.get 8
            i32.const -8
            i32.and
            i32.const 1048620
            i32.add
            local.set 7
            i32.const 0
            i32.load offset=1048600
            local.set 4
            block ;; label = @4
              block ;; label = @5
                i32.const 1
                local.get 8
                i32.const 3
                i32.shr_u
                i32.shl
                local.tee 9
                local.get 6
                i32.and
                br_if 0 (;@5;)
                i32.const 0
                local.get 9
                local.get 6
                i32.or
                i32.store offset=1048580
                local.get 7
                local.set 9
                br 1 (;@4;)
              end
              local.get 7
              i32.load offset=8
              local.set 9
            end
            local.get 9
            local.get 4
            i32.store offset=12
            local.get 7
            local.get 4
            i32.store offset=8
            local.get 4
            local.get 7
            i32.store offset=12
            local.get 4
            local.get 9
            i32.store offset=8
          end
          i32.const 0
          local.get 5
          i32.store offset=1048600
          i32.const 0
          local.get 3
          i32.store offset=1048588
        end
        local.get 0
        i32.const 8
        i32.add
        local.set 4
      end
      local.get 1
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 4
    )
    (func $free (;10;) (type 4) (param i32)
      local.get 0
      call $dlfree
    )
    (func $dlfree (;11;) (type 4) (param i32)
      (local i32 i32 i32 i32 i32 i32 i32)
      block ;; label = @1
        local.get 0
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.const -8
        i32.add
        local.tee 1
        local.get 0
        i32.const -4
        i32.add
        i32.load
        local.tee 2
        i32.const -8
        i32.and
        local.tee 0
        i32.add
        local.set 3
        block ;; label = @2
          local.get 2
          i32.const 1
          i32.and
          br_if 0 (;@2;)
          local.get 2
          i32.const 3
          i32.and
          i32.eqz
          br_if 1 (;@1;)
          local.get 1
          local.get 1
          i32.load
          local.tee 2
          i32.sub
          local.tee 1
          i32.const 0
          i32.load offset=1048596
          local.tee 4
          i32.lt_u
          br_if 1 (;@1;)
          local.get 2
          local.get 0
          i32.add
          local.set 0
          block ;; label = @3
            local.get 1
            i32.const 0
            i32.load offset=1048600
            i32.eq
            br_if 0 (;@3;)
            block ;; label = @4
              local.get 2
              i32.const 255
              i32.gt_u
              br_if 0 (;@4;)
              local.get 1
              i32.load offset=8
              local.tee 4
              local.get 2
              i32.const 3
              i32.shr_u
              local.tee 5
              i32.const 3
              i32.shl
              i32.const 1048620
              i32.add
              local.tee 6
              i32.eq
              drop
              block ;; label = @5
                local.get 1
                i32.load offset=12
                local.tee 2
                local.get 4
                i32.ne
                br_if 0 (;@5;)
                i32.const 0
                i32.const 0
                i32.load offset=1048580
                i32.const -2
                local.get 5
                i32.rotl
                i32.and
                i32.store offset=1048580
                br 3 (;@2;)
              end
              local.get 2
              local.get 6
              i32.eq
              drop
              local.get 2
              local.get 4
              i32.store offset=8
              local.get 4
              local.get 2
              i32.store offset=12
              br 2 (;@2;)
            end
            local.get 1
            i32.load offset=24
            local.set 7
            block ;; label = @4
              block ;; label = @5
                local.get 1
                i32.load offset=12
                local.tee 6
                local.get 1
                i32.eq
                br_if 0 (;@5;)
                local.get 1
                i32.load offset=8
                local.tee 2
                local.get 4
                i32.lt_u
                drop
                local.get 6
                local.get 2
                i32.store offset=8
                local.get 2
                local.get 6
                i32.store offset=12
                br 1 (;@4;)
              end
              block ;; label = @5
                local.get 1
                i32.const 20
                i32.add
                local.tee 2
                i32.load
                local.tee 4
                br_if 0 (;@5;)
                local.get 1
                i32.const 16
                i32.add
                local.tee 2
                i32.load
                local.tee 4
                br_if 0 (;@5;)
                i32.const 0
                local.set 6
                br 1 (;@4;)
              end
              loop ;; label = @5
                local.get 2
                local.set 5
                local.get 4
                local.tee 6
                i32.const 20
                i32.add
                local.tee 2
                i32.load
                local.tee 4
                br_if 0 (;@5;)
                local.get 6
                i32.const 16
                i32.add
                local.set 2
                local.get 6
                i32.load offset=16
                local.tee 4
                br_if 0 (;@5;)
              end
              local.get 5
              i32.const 0
              i32.store
            end
            local.get 7
            i32.eqz
            br_if 1 (;@2;)
            block ;; label = @4
              block ;; label = @5
                local.get 1
                local.get 1
                i32.load offset=28
                local.tee 4
                i32.const 2
                i32.shl
                i32.const 1048884
                i32.add
                local.tee 2
                i32.load
                i32.ne
                br_if 0 (;@5;)
                local.get 2
                local.get 6
                i32.store
                local.get 6
                br_if 1 (;@4;)
                i32.const 0
                i32.const 0
                i32.load offset=1048584
                i32.const -2
                local.get 4
                i32.rotl
                i32.and
                i32.store offset=1048584
                br 3 (;@2;)
              end
              local.get 7
              i32.const 16
              i32.const 20
              local.get 7
              i32.load offset=16
              local.get 1
              i32.eq
              select
              i32.add
              local.get 6
              i32.store
              local.get 6
              i32.eqz
              br_if 2 (;@2;)
            end
            local.get 6
            local.get 7
            i32.store offset=24
            block ;; label = @4
              local.get 1
              i32.load offset=16
              local.tee 2
              i32.eqz
              br_if 0 (;@4;)
              local.get 6
              local.get 2
              i32.store offset=16
              local.get 2
              local.get 6
              i32.store offset=24
            end
            local.get 1
            i32.load offset=20
            local.tee 2
            i32.eqz
            br_if 1 (;@2;)
            local.get 6
            i32.const 20
            i32.add
            local.get 2
            i32.store
            local.get 2
            local.get 6
            i32.store offset=24
            br 1 (;@2;)
          end
          local.get 3
          i32.load offset=4
          local.tee 2
          i32.const 3
          i32.and
          i32.const 3
          i32.ne
          br_if 0 (;@2;)
          local.get 3
          local.get 2
          i32.const -2
          i32.and
          i32.store offset=4
          i32.const 0
          local.get 0
          i32.store offset=1048588
          local.get 1
          local.get 0
          i32.add
          local.get 0
          i32.store
          local.get 1
          local.get 0
          i32.const 1
          i32.or
          i32.store offset=4
          return
        end
        local.get 1
        local.get 3
        i32.ge_u
        br_if 0 (;@1;)
        local.get 3
        i32.load offset=4
        local.tee 2
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@1;)
        block ;; label = @2
          block ;; label = @3
            local.get 2
            i32.const 2
            i32.and
            br_if 0 (;@3;)
            block ;; label = @4
              local.get 3
              i32.const 0
              i32.load offset=1048604
              i32.ne
              br_if 0 (;@4;)
              i32.const 0
              local.get 1
              i32.store offset=1048604
              i32.const 0
              i32.const 0
              i32.load offset=1048592
              local.get 0
              i32.add
              local.tee 0
              i32.store offset=1048592
              local.get 1
              local.get 0
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 1
              i32.const 0
              i32.load offset=1048600
              i32.ne
              br_if 3 (;@1;)
              i32.const 0
              i32.const 0
              i32.store offset=1048588
              i32.const 0
              i32.const 0
              i32.store offset=1048600
              return
            end
            block ;; label = @4
              local.get 3
              i32.const 0
              i32.load offset=1048600
              i32.ne
              br_if 0 (;@4;)
              i32.const 0
              local.get 1
              i32.store offset=1048600
              i32.const 0
              i32.const 0
              i32.load offset=1048588
              local.get 0
              i32.add
              local.tee 0
              i32.store offset=1048588
              local.get 1
              local.get 0
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 1
              local.get 0
              i32.add
              local.get 0
              i32.store
              return
            end
            local.get 2
            i32.const -8
            i32.and
            local.get 0
            i32.add
            local.set 0
            block ;; label = @4
              block ;; label = @5
                local.get 2
                i32.const 255
                i32.gt_u
                br_if 0 (;@5;)
                local.get 3
                i32.load offset=8
                local.tee 4
                local.get 2
                i32.const 3
                i32.shr_u
                local.tee 5
                i32.const 3
                i32.shl
                i32.const 1048620
                i32.add
                local.tee 6
                i32.eq
                drop
                block ;; label = @6
                  local.get 3
                  i32.load offset=12
                  local.tee 2
                  local.get 4
                  i32.ne
                  br_if 0 (;@6;)
                  i32.const 0
                  i32.const 0
                  i32.load offset=1048580
                  i32.const -2
                  local.get 5
                  i32.rotl
                  i32.and
                  i32.store offset=1048580
                  br 2 (;@4;)
                end
                local.get 2
                local.get 6
                i32.eq
                drop
                local.get 2
                local.get 4
                i32.store offset=8
                local.get 4
                local.get 2
                i32.store offset=12
                br 1 (;@4;)
              end
              local.get 3
              i32.load offset=24
              local.set 7
              block ;; label = @5
                block ;; label = @6
                  local.get 3
                  i32.load offset=12
                  local.tee 6
                  local.get 3
                  i32.eq
                  br_if 0 (;@6;)
                  local.get 3
                  i32.load offset=8
                  local.tee 2
                  i32.const 0
                  i32.load offset=1048596
                  i32.lt_u
                  drop
                  local.get 6
                  local.get 2
                  i32.store offset=8
                  local.get 2
                  local.get 6
                  i32.store offset=12
                  br 1 (;@5;)
                end
                block ;; label = @6
                  local.get 3
                  i32.const 20
                  i32.add
                  local.tee 2
                  i32.load
                  local.tee 4
                  br_if 0 (;@6;)
                  local.get 3
                  i32.const 16
                  i32.add
                  local.tee 2
                  i32.load
                  local.tee 4
                  br_if 0 (;@6;)
                  i32.const 0
                  local.set 6
                  br 1 (;@5;)
                end
                loop ;; label = @6
                  local.get 2
                  local.set 5
                  local.get 4
                  local.tee 6
                  i32.const 20
                  i32.add
                  local.tee 2
                  i32.load
                  local.tee 4
                  br_if 0 (;@6;)
                  local.get 6
                  i32.const 16
                  i32.add
                  local.set 2
                  local.get 6
                  i32.load offset=16
                  local.tee 4
                  br_if 0 (;@6;)
                end
                local.get 5
                i32.const 0
                i32.store
              end
              local.get 7
              i32.eqz
              br_if 0 (;@4;)
              block ;; label = @5
                block ;; label = @6
                  local.get 3
                  local.get 3
                  i32.load offset=28
                  local.tee 4
                  i32.const 2
                  i32.shl
                  i32.const 1048884
                  i32.add
                  local.tee 2
                  i32.load
                  i32.ne
                  br_if 0 (;@6;)
                  local.get 2
                  local.get 6
                  i32.store
                  local.get 6
                  br_if 1 (;@5;)
                  i32.const 0
                  i32.const 0
                  i32.load offset=1048584
                  i32.const -2
                  local.get 4
                  i32.rotl
                  i32.and
                  i32.store offset=1048584
                  br 2 (;@4;)
                end
                local.get 7
                i32.const 16
                i32.const 20
                local.get 7
                i32.load offset=16
                local.get 3
                i32.eq
                select
                i32.add
                local.get 6
                i32.store
                local.get 6
                i32.eqz
                br_if 1 (;@4;)
              end
              local.get 6
              local.get 7
              i32.store offset=24
              block ;; label = @5
                local.get 3
                i32.load offset=16
                local.tee 2
                i32.eqz
                br_if 0 (;@5;)
                local.get 6
                local.get 2
                i32.store offset=16
                local.get 2
                local.get 6
                i32.store offset=24
              end
              local.get 3
              i32.load offset=20
              local.tee 2
              i32.eqz
              br_if 0 (;@4;)
              local.get 6
              i32.const 20
              i32.add
              local.get 2
              i32.store
              local.get 2
              local.get 6
              i32.store offset=24
            end
            local.get 1
            local.get 0
            i32.add
            local.get 0
            i32.store
            local.get 1
            local.get 0
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 1
            i32.const 0
            i32.load offset=1048600
            i32.ne
            br_if 1 (;@2;)
            i32.const 0
            local.get 0
            i32.store offset=1048588
            return
          end
          local.get 3
          local.get 2
          i32.const -2
          i32.and
          i32.store offset=4
          local.get 1
          local.get 0
          i32.add
          local.get 0
          i32.store
          local.get 1
          local.get 0
          i32.const 1
          i32.or
          i32.store offset=4
        end
        block ;; label = @2
          local.get 0
          i32.const 255
          i32.gt_u
          br_if 0 (;@2;)
          local.get 0
          i32.const -8
          i32.and
          i32.const 1048620
          i32.add
          local.set 2
          block ;; label = @3
            block ;; label = @4
              i32.const 0
              i32.load offset=1048580
              local.tee 4
              i32.const 1
              local.get 0
              i32.const 3
              i32.shr_u
              i32.shl
              local.tee 0
              i32.and
              br_if 0 (;@4;)
              i32.const 0
              local.get 4
              local.get 0
              i32.or
              i32.store offset=1048580
              local.get 2
              local.set 0
              br 1 (;@3;)
            end
            local.get 2
            i32.load offset=8
            local.set 0
          end
          local.get 0
          local.get 1
          i32.store offset=12
          local.get 2
          local.get 1
          i32.store offset=8
          local.get 1
          local.get 2
          i32.store offset=12
          local.get 1
          local.get 0
          i32.store offset=8
          return
        end
        i32.const 31
        local.set 2
        block ;; label = @2
          local.get 0
          i32.const 16777215
          i32.gt_u
          br_if 0 (;@2;)
          local.get 0
          i32.const 38
          local.get 0
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
        local.get 1
        local.get 2
        i32.store offset=28
        local.get 1
        i64.const 0
        i64.store offset=16 align=4
        local.get 2
        i32.const 2
        i32.shl
        i32.const 1048884
        i32.add
        local.set 4
        block ;; label = @2
          block ;; label = @3
            i32.const 0
            i32.load offset=1048584
            local.tee 6
            i32.const 1
            local.get 2
            i32.shl
            local.tee 3
            i32.and
            br_if 0 (;@3;)
            local.get 4
            local.get 1
            i32.store
            i32.const 0
            local.get 6
            local.get 3
            i32.or
            i32.store offset=1048584
            local.get 1
            local.get 4
            i32.store offset=24
            local.get 1
            local.get 1
            i32.store offset=8
            local.get 1
            local.get 1
            i32.store offset=12
            br 1 (;@2;)
          end
          local.get 0
          i32.const 0
          i32.const 25
          local.get 2
          i32.const 1
          i32.shr_u
          i32.sub
          local.get 2
          i32.const 31
          i32.eq
          select
          i32.shl
          local.set 2
          local.get 4
          i32.load
          local.set 6
          block ;; label = @3
            loop ;; label = @4
              local.get 6
              local.tee 4
              i32.load offset=4
              i32.const -8
              i32.and
              local.get 0
              i32.eq
              br_if 1 (;@3;)
              local.get 2
              i32.const 29
              i32.shr_u
              local.set 6
              local.get 2
              i32.const 1
              i32.shl
              local.set 2
              local.get 4
              local.get 6
              i32.const 4
              i32.and
              i32.add
              i32.const 16
              i32.add
              local.tee 3
              i32.load
              local.tee 6
              br_if 0 (;@4;)
            end
            local.get 3
            local.get 1
            i32.store
            local.get 1
            local.get 4
            i32.store offset=24
            local.get 1
            local.get 1
            i32.store offset=12
            local.get 1
            local.get 1
            i32.store offset=8
            br 1 (;@2;)
          end
          local.get 4
          i32.load offset=8
          local.tee 0
          local.get 1
          i32.store offset=12
          local.get 4
          local.get 1
          i32.store offset=8
          local.get 1
          i32.const 0
          i32.store offset=24
          local.get 1
          local.get 4
          i32.store offset=12
          local.get 1
          local.get 0
          i32.store offset=8
        end
        i32.const 0
        i32.const 0
        i32.load offset=1048612
        i32.const -1
        i32.add
        local.tee 1
        i32.const -1
        local.get 1
        select
        i32.store offset=1048612
      end
    )
    (func $realloc (;12;) (type 1) (param i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
      block ;; label = @1
        local.get 0
        br_if 0 (;@1;)
        local.get 1
        call $dlmalloc
        return
      end
      block ;; label = @1
        local.get 1
        i32.const -64
        i32.lt_u
        br_if 0 (;@1;)
        i32.const 0
        i32.const 48
        i32.store offset=1049076
        i32.const 0
        return
      end
      i32.const 16
      local.get 1
      i32.const 19
      i32.add
      i32.const -16
      i32.and
      local.get 1
      i32.const 11
      i32.lt_u
      select
      local.set 2
      local.get 0
      i32.const -4
      i32.add
      local.tee 3
      i32.load
      local.tee 4
      i32.const -8
      i32.and
      local.set 5
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 4
            i32.const 3
            i32.and
            br_if 0 (;@3;)
            local.get 2
            i32.const 256
            i32.lt_u
            br_if 1 (;@2;)
            local.get 5
            local.get 2
            i32.const 4
            i32.or
            i32.lt_u
            br_if 1 (;@2;)
            local.get 5
            local.get 2
            i32.sub
            i32.const 0
            i32.load offset=1049060
            i32.const 1
            i32.shl
            i32.le_u
            br_if 2 (;@1;)
            br 1 (;@2;)
          end
          local.get 0
          i32.const -8
          i32.add
          local.tee 6
          local.get 5
          i32.add
          local.set 7
          block ;; label = @3
            local.get 5
            local.get 2
            i32.lt_u
            br_if 0 (;@3;)
            local.get 5
            local.get 2
            i32.sub
            local.tee 1
            i32.const 16
            i32.lt_u
            br_if 2 (;@1;)
            local.get 3
            local.get 2
            local.get 4
            i32.const 1
            i32.and
            i32.or
            i32.const 2
            i32.or
            i32.store
            local.get 6
            local.get 2
            i32.add
            local.tee 2
            local.get 1
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 7
            local.get 7
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 2
            local.get 1
            call $dispose_chunk
            local.get 0
            return
          end
          block ;; label = @3
            local.get 7
            i32.const 0
            i32.load offset=1048604
            i32.ne
            br_if 0 (;@3;)
            i32.const 0
            i32.load offset=1048592
            local.get 5
            i32.add
            local.tee 5
            local.get 2
            i32.le_u
            br_if 1 (;@2;)
            local.get 3
            local.get 2
            local.get 4
            i32.const 1
            i32.and
            i32.or
            i32.const 2
            i32.or
            i32.store
            i32.const 0
            local.get 6
            local.get 2
            i32.add
            local.tee 1
            i32.store offset=1048604
            i32.const 0
            local.get 5
            local.get 2
            i32.sub
            local.tee 2
            i32.store offset=1048592
            local.get 1
            local.get 2
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 0
            return
          end
          block ;; label = @3
            local.get 7
            i32.const 0
            i32.load offset=1048600
            i32.ne
            br_if 0 (;@3;)
            i32.const 0
            i32.load offset=1048588
            local.get 5
            i32.add
            local.tee 5
            local.get 2
            i32.lt_u
            br_if 1 (;@2;)
            block ;; label = @4
              block ;; label = @5
                local.get 5
                local.get 2
                i32.sub
                local.tee 1
                i32.const 16
                i32.lt_u
                br_if 0 (;@5;)
                local.get 3
                local.get 2
                local.get 4
                i32.const 1
                i32.and
                i32.or
                i32.const 2
                i32.or
                i32.store
                local.get 6
                local.get 2
                i32.add
                local.tee 2
                local.get 1
                i32.const 1
                i32.or
                i32.store offset=4
                local.get 6
                local.get 5
                i32.add
                local.tee 5
                local.get 1
                i32.store
                local.get 5
                local.get 5
                i32.load offset=4
                i32.const -2
                i32.and
                i32.store offset=4
                br 1 (;@4;)
              end
              local.get 3
              local.get 4
              i32.const 1
              i32.and
              local.get 5
              i32.or
              i32.const 2
              i32.or
              i32.store
              local.get 6
              local.get 5
              i32.add
              local.tee 1
              local.get 1
              i32.load offset=4
              i32.const 1
              i32.or
              i32.store offset=4
              i32.const 0
              local.set 1
              i32.const 0
              local.set 2
            end
            i32.const 0
            local.get 2
            i32.store offset=1048600
            i32.const 0
            local.get 1
            i32.store offset=1048588
            local.get 0
            return
          end
          local.get 7
          i32.load offset=4
          local.tee 8
          i32.const 2
          i32.and
          br_if 0 (;@2;)
          local.get 8
          i32.const -8
          i32.and
          local.get 5
          i32.add
          local.tee 9
          local.get 2
          i32.lt_u
          br_if 0 (;@2;)
          local.get 9
          local.get 2
          i32.sub
          local.set 10
          block ;; label = @3
            block ;; label = @4
              local.get 8
              i32.const 255
              i32.gt_u
              br_if 0 (;@4;)
              local.get 7
              i32.load offset=8
              local.tee 1
              local.get 8
              i32.const 3
              i32.shr_u
              local.tee 11
              i32.const 3
              i32.shl
              i32.const 1048620
              i32.add
              local.tee 8
              i32.eq
              drop
              block ;; label = @5
                local.get 7
                i32.load offset=12
                local.tee 5
                local.get 1
                i32.ne
                br_if 0 (;@5;)
                i32.const 0
                i32.const 0
                i32.load offset=1048580
                i32.const -2
                local.get 11
                i32.rotl
                i32.and
                i32.store offset=1048580
                br 2 (;@3;)
              end
              local.get 5
              local.get 8
              i32.eq
              drop
              local.get 5
              local.get 1
              i32.store offset=8
              local.get 1
              local.get 5
              i32.store offset=12
              br 1 (;@3;)
            end
            local.get 7
            i32.load offset=24
            local.set 12
            block ;; label = @4
              block ;; label = @5
                local.get 7
                i32.load offset=12
                local.tee 8
                local.get 7
                i32.eq
                br_if 0 (;@5;)
                local.get 7
                i32.load offset=8
                local.tee 1
                i32.const 0
                i32.load offset=1048596
                i32.lt_u
                drop
                local.get 8
                local.get 1
                i32.store offset=8
                local.get 1
                local.get 8
                i32.store offset=12
                br 1 (;@4;)
              end
              block ;; label = @5
                local.get 7
                i32.const 20
                i32.add
                local.tee 1
                i32.load
                local.tee 5
                br_if 0 (;@5;)
                local.get 7
                i32.const 16
                i32.add
                local.tee 1
                i32.load
                local.tee 5
                br_if 0 (;@5;)
                i32.const 0
                local.set 8
                br 1 (;@4;)
              end
              loop ;; label = @5
                local.get 1
                local.set 11
                local.get 5
                local.tee 8
                i32.const 20
                i32.add
                local.tee 1
                i32.load
                local.tee 5
                br_if 0 (;@5;)
                local.get 8
                i32.const 16
                i32.add
                local.set 1
                local.get 8
                i32.load offset=16
                local.tee 5
                br_if 0 (;@5;)
              end
              local.get 11
              i32.const 0
              i32.store
            end
            local.get 12
            i32.eqz
            br_if 0 (;@3;)
            block ;; label = @4
              block ;; label = @5
                local.get 7
                local.get 7
                i32.load offset=28
                local.tee 5
                i32.const 2
                i32.shl
                i32.const 1048884
                i32.add
                local.tee 1
                i32.load
                i32.ne
                br_if 0 (;@5;)
                local.get 1
                local.get 8
                i32.store
                local.get 8
                br_if 1 (;@4;)
                i32.const 0
                i32.const 0
                i32.load offset=1048584
                i32.const -2
                local.get 5
                i32.rotl
                i32.and
                i32.store offset=1048584
                br 2 (;@3;)
              end
              local.get 12
              i32.const 16
              i32.const 20
              local.get 12
              i32.load offset=16
              local.get 7
              i32.eq
              select
              i32.add
              local.get 8
              i32.store
              local.get 8
              i32.eqz
              br_if 1 (;@3;)
            end
            local.get 8
            local.get 12
            i32.store offset=24
            block ;; label = @4
              local.get 7
              i32.load offset=16
              local.tee 1
              i32.eqz
              br_if 0 (;@4;)
              local.get 8
              local.get 1
              i32.store offset=16
              local.get 1
              local.get 8
              i32.store offset=24
            end
            local.get 7
            i32.load offset=20
            local.tee 1
            i32.eqz
            br_if 0 (;@3;)
            local.get 8
            i32.const 20
            i32.add
            local.get 1
            i32.store
            local.get 1
            local.get 8
            i32.store offset=24
          end
          block ;; label = @3
            local.get 10
            i32.const 15
            i32.gt_u
            br_if 0 (;@3;)
            local.get 3
            local.get 4
            i32.const 1
            i32.and
            local.get 9
            i32.or
            i32.const 2
            i32.or
            i32.store
            local.get 6
            local.get 9
            i32.add
            local.tee 1
            local.get 1
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 0
            return
          end
          local.get 3
          local.get 2
          local.get 4
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get 6
          local.get 2
          i32.add
          local.tee 1
          local.get 10
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 6
          local.get 9
          i32.add
          local.tee 2
          local.get 2
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 1
          local.get 10
          call $dispose_chunk
          local.get 0
          return
        end
        block ;; label = @2
          local.get 1
          call $dlmalloc
          local.tee 2
          br_if 0 (;@2;)
          i32.const 0
          return
        end
        local.get 2
        local.get 0
        i32.const -4
        i32.const -8
        local.get 3
        i32.load
        local.tee 5
        i32.const 3
        i32.and
        select
        local.get 5
        i32.const -8
        i32.and
        i32.add
        local.tee 5
        local.get 1
        local.get 5
        local.get 1
        i32.lt_u
        select
        call $memcpy
        local.set 1
        local.get 0
        call $dlfree
        local.get 1
        local.set 0
      end
      local.get 0
    )
    (func $dispose_chunk (;13;) (type 5) (param i32 i32)
      (local i32 i32 i32 i32 i32 i32)
      local.get 0
      local.get 1
      i32.add
      local.set 2
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.load offset=4
          local.tee 3
          i32.const 1
          i32.and
          br_if 0 (;@2;)
          local.get 3
          i32.const 3
          i32.and
          i32.eqz
          br_if 1 (;@1;)
          local.get 0
          i32.load
          local.tee 3
          local.get 1
          i32.add
          local.set 1
          block ;; label = @3
            block ;; label = @4
              local.get 0
              local.get 3
              i32.sub
              local.tee 0
              i32.const 0
              i32.load offset=1048600
              i32.eq
              br_if 0 (;@4;)
              block ;; label = @5
                local.get 3
                i32.const 255
                i32.gt_u
                br_if 0 (;@5;)
                local.get 0
                i32.load offset=8
                local.tee 4
                local.get 3
                i32.const 3
                i32.shr_u
                local.tee 5
                i32.const 3
                i32.shl
                i32.const 1048620
                i32.add
                local.tee 6
                i32.eq
                drop
                local.get 0
                i32.load offset=12
                local.tee 3
                local.get 4
                i32.ne
                br_if 2 (;@3;)
                i32.const 0
                i32.const 0
                i32.load offset=1048580
                i32.const -2
                local.get 5
                i32.rotl
                i32.and
                i32.store offset=1048580
                br 3 (;@2;)
              end
              local.get 0
              i32.load offset=24
              local.set 7
              block ;; label = @5
                block ;; label = @6
                  local.get 0
                  i32.load offset=12
                  local.tee 6
                  local.get 0
                  i32.eq
                  br_if 0 (;@6;)
                  local.get 0
                  i32.load offset=8
                  local.tee 3
                  i32.const 0
                  i32.load offset=1048596
                  i32.lt_u
                  drop
                  local.get 6
                  local.get 3
                  i32.store offset=8
                  local.get 3
                  local.get 6
                  i32.store offset=12
                  br 1 (;@5;)
                end
                block ;; label = @6
                  local.get 0
                  i32.const 20
                  i32.add
                  local.tee 3
                  i32.load
                  local.tee 4
                  br_if 0 (;@6;)
                  local.get 0
                  i32.const 16
                  i32.add
                  local.tee 3
                  i32.load
                  local.tee 4
                  br_if 0 (;@6;)
                  i32.const 0
                  local.set 6
                  br 1 (;@5;)
                end
                loop ;; label = @6
                  local.get 3
                  local.set 5
                  local.get 4
                  local.tee 6
                  i32.const 20
                  i32.add
                  local.tee 3
                  i32.load
                  local.tee 4
                  br_if 0 (;@6;)
                  local.get 6
                  i32.const 16
                  i32.add
                  local.set 3
                  local.get 6
                  i32.load offset=16
                  local.tee 4
                  br_if 0 (;@6;)
                end
                local.get 5
                i32.const 0
                i32.store
              end
              local.get 7
              i32.eqz
              br_if 2 (;@2;)
              block ;; label = @5
                block ;; label = @6
                  local.get 0
                  local.get 0
                  i32.load offset=28
                  local.tee 4
                  i32.const 2
                  i32.shl
                  i32.const 1048884
                  i32.add
                  local.tee 3
                  i32.load
                  i32.ne
                  br_if 0 (;@6;)
                  local.get 3
                  local.get 6
                  i32.store
                  local.get 6
                  br_if 1 (;@5;)
                  i32.const 0
                  i32.const 0
                  i32.load offset=1048584
                  i32.const -2
                  local.get 4
                  i32.rotl
                  i32.and
                  i32.store offset=1048584
                  br 4 (;@2;)
                end
                local.get 7
                i32.const 16
                i32.const 20
                local.get 7
                i32.load offset=16
                local.get 0
                i32.eq
                select
                i32.add
                local.get 6
                i32.store
                local.get 6
                i32.eqz
                br_if 3 (;@2;)
              end
              local.get 6
              local.get 7
              i32.store offset=24
              block ;; label = @5
                local.get 0
                i32.load offset=16
                local.tee 3
                i32.eqz
                br_if 0 (;@5;)
                local.get 6
                local.get 3
                i32.store offset=16
                local.get 3
                local.get 6
                i32.store offset=24
              end
              local.get 0
              i32.load offset=20
              local.tee 3
              i32.eqz
              br_if 2 (;@2;)
              local.get 6
              i32.const 20
              i32.add
              local.get 3
              i32.store
              local.get 3
              local.get 6
              i32.store offset=24
              br 2 (;@2;)
            end
            local.get 2
            i32.load offset=4
            local.tee 3
            i32.const 3
            i32.and
            i32.const 3
            i32.ne
            br_if 1 (;@2;)
            local.get 2
            local.get 3
            i32.const -2
            i32.and
            i32.store offset=4
            i32.const 0
            local.get 1
            i32.store offset=1048588
            local.get 2
            local.get 1
            i32.store
            local.get 0
            local.get 1
            i32.const 1
            i32.or
            i32.store offset=4
            return
          end
          local.get 3
          local.get 6
          i32.eq
          drop
          local.get 3
          local.get 4
          i32.store offset=8
          local.get 4
          local.get 3
          i32.store offset=12
        end
        block ;; label = @2
          block ;; label = @3
            local.get 2
            i32.load offset=4
            local.tee 3
            i32.const 2
            i32.and
            br_if 0 (;@3;)
            block ;; label = @4
              local.get 2
              i32.const 0
              i32.load offset=1048604
              i32.ne
              br_if 0 (;@4;)
              i32.const 0
              local.get 0
              i32.store offset=1048604
              i32.const 0
              i32.const 0
              i32.load offset=1048592
              local.get 1
              i32.add
              local.tee 1
              i32.store offset=1048592
              local.get 0
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              i32.const 0
              i32.load offset=1048600
              i32.ne
              br_if 3 (;@1;)
              i32.const 0
              i32.const 0
              i32.store offset=1048588
              i32.const 0
              i32.const 0
              i32.store offset=1048600
              return
            end
            block ;; label = @4
              local.get 2
              i32.const 0
              i32.load offset=1048600
              i32.ne
              br_if 0 (;@4;)
              i32.const 0
              local.get 0
              i32.store offset=1048600
              i32.const 0
              i32.const 0
              i32.load offset=1048588
              local.get 1
              i32.add
              local.tee 1
              i32.store offset=1048588
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
              return
            end
            local.get 3
            i32.const -8
            i32.and
            local.get 1
            i32.add
            local.set 1
            block ;; label = @4
              block ;; label = @5
                local.get 3
                i32.const 255
                i32.gt_u
                br_if 0 (;@5;)
                local.get 2
                i32.load offset=8
                local.tee 4
                local.get 3
                i32.const 3
                i32.shr_u
                local.tee 5
                i32.const 3
                i32.shl
                i32.const 1048620
                i32.add
                local.tee 6
                i32.eq
                drop
                block ;; label = @6
                  local.get 2
                  i32.load offset=12
                  local.tee 3
                  local.get 4
                  i32.ne
                  br_if 0 (;@6;)
                  i32.const 0
                  i32.const 0
                  i32.load offset=1048580
                  i32.const -2
                  local.get 5
                  i32.rotl
                  i32.and
                  i32.store offset=1048580
                  br 2 (;@4;)
                end
                local.get 3
                local.get 6
                i32.eq
                drop
                local.get 3
                local.get 4
                i32.store offset=8
                local.get 4
                local.get 3
                i32.store offset=12
                br 1 (;@4;)
              end
              local.get 2
              i32.load offset=24
              local.set 7
              block ;; label = @5
                block ;; label = @6
                  local.get 2
                  i32.load offset=12
                  local.tee 6
                  local.get 2
                  i32.eq
                  br_if 0 (;@6;)
                  local.get 2
                  i32.load offset=8
                  local.tee 3
                  i32.const 0
                  i32.load offset=1048596
                  i32.lt_u
                  drop
                  local.get 6
                  local.get 3
                  i32.store offset=8
                  local.get 3
                  local.get 6
                  i32.store offset=12
                  br 1 (;@5;)
                end
                block ;; label = @6
                  local.get 2
                  i32.const 20
                  i32.add
                  local.tee 4
                  i32.load
                  local.tee 3
                  br_if 0 (;@6;)
                  local.get 2
                  i32.const 16
                  i32.add
                  local.tee 4
                  i32.load
                  local.tee 3
                  br_if 0 (;@6;)
                  i32.const 0
                  local.set 6
                  br 1 (;@5;)
                end
                loop ;; label = @6
                  local.get 4
                  local.set 5
                  local.get 3
                  local.tee 6
                  i32.const 20
                  i32.add
                  local.tee 4
                  i32.load
                  local.tee 3
                  br_if 0 (;@6;)
                  local.get 6
                  i32.const 16
                  i32.add
                  local.set 4
                  local.get 6
                  i32.load offset=16
                  local.tee 3
                  br_if 0 (;@6;)
                end
                local.get 5
                i32.const 0
                i32.store
              end
              local.get 7
              i32.eqz
              br_if 0 (;@4;)
              block ;; label = @5
                block ;; label = @6
                  local.get 2
                  local.get 2
                  i32.load offset=28
                  local.tee 4
                  i32.const 2
                  i32.shl
                  i32.const 1048884
                  i32.add
                  local.tee 3
                  i32.load
                  i32.ne
                  br_if 0 (;@6;)
                  local.get 3
                  local.get 6
                  i32.store
                  local.get 6
                  br_if 1 (;@5;)
                  i32.const 0
                  i32.const 0
                  i32.load offset=1048584
                  i32.const -2
                  local.get 4
                  i32.rotl
                  i32.and
                  i32.store offset=1048584
                  br 2 (;@4;)
                end
                local.get 7
                i32.const 16
                i32.const 20
                local.get 7
                i32.load offset=16
                local.get 2
                i32.eq
                select
                i32.add
                local.get 6
                i32.store
                local.get 6
                i32.eqz
                br_if 1 (;@4;)
              end
              local.get 6
              local.get 7
              i32.store offset=24
              block ;; label = @5
                local.get 2
                i32.load offset=16
                local.tee 3
                i32.eqz
                br_if 0 (;@5;)
                local.get 6
                local.get 3
                i32.store offset=16
                local.get 3
                local.get 6
                i32.store offset=24
              end
              local.get 2
              i32.load offset=20
              local.tee 3
              i32.eqz
              br_if 0 (;@4;)
              local.get 6
              i32.const 20
              i32.add
              local.get 3
              i32.store
              local.get 3
              local.get 6
              i32.store offset=24
            end
            local.get 0
            local.get 1
            i32.add
            local.get 1
            i32.store
            local.get 0
            local.get 1
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 0
            i32.const 0
            i32.load offset=1048600
            i32.ne
            br_if 1 (;@2;)
            i32.const 0
            local.get 1
            i32.store offset=1048588
            return
          end
          local.get 2
          local.get 3
          i32.const -2
          i32.and
          i32.store offset=4
          local.get 0
          local.get 1
          i32.add
          local.get 1
          i32.store
          local.get 0
          local.get 1
          i32.const 1
          i32.or
          i32.store offset=4
        end
        block ;; label = @2
          local.get 1
          i32.const 255
          i32.gt_u
          br_if 0 (;@2;)
          local.get 1
          i32.const -8
          i32.and
          i32.const 1048620
          i32.add
          local.set 3
          block ;; label = @3
            block ;; label = @4
              i32.const 0
              i32.load offset=1048580
              local.tee 4
              i32.const 1
              local.get 1
              i32.const 3
              i32.shr_u
              i32.shl
              local.tee 1
              i32.and
              br_if 0 (;@4;)
              i32.const 0
              local.get 4
              local.get 1
              i32.or
              i32.store offset=1048580
              local.get 3
              local.set 1
              br 1 (;@3;)
            end
            local.get 3
            i32.load offset=8
            local.set 1
          end
          local.get 1
          local.get 0
          i32.store offset=12
          local.get 3
          local.get 0
          i32.store offset=8
          local.get 0
          local.get 3
          i32.store offset=12
          local.get 0
          local.get 1
          i32.store offset=8
          return
        end
        i32.const 31
        local.set 3
        block ;; label = @2
          local.get 1
          i32.const 16777215
          i32.gt_u
          br_if 0 (;@2;)
          local.get 1
          i32.const 38
          local.get 1
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
        local.get 0
        local.get 3
        i32.store offset=28
        local.get 0
        i64.const 0
        i64.store offset=16 align=4
        local.get 3
        i32.const 2
        i32.shl
        i32.const 1048884
        i32.add
        local.set 4
        block ;; label = @2
          i32.const 0
          i32.load offset=1048584
          local.tee 6
          i32.const 1
          local.get 3
          i32.shl
          local.tee 2
          i32.and
          br_if 0 (;@2;)
          local.get 4
          local.get 0
          i32.store
          i32.const 0
          local.get 6
          local.get 2
          i32.or
          i32.store offset=1048584
          local.get 0
          local.get 4
          i32.store offset=24
          local.get 0
          local.get 0
          i32.store offset=8
          local.get 0
          local.get 0
          i32.store offset=12
          return
        end
        local.get 1
        i32.const 0
        i32.const 25
        local.get 3
        i32.const 1
        i32.shr_u
        i32.sub
        local.get 3
        i32.const 31
        i32.eq
        select
        i32.shl
        local.set 3
        local.get 4
        i32.load
        local.set 6
        block ;; label = @2
          loop ;; label = @3
            local.get 6
            local.tee 4
            i32.load offset=4
            i32.const -8
            i32.and
            local.get 1
            i32.eq
            br_if 1 (;@2;)
            local.get 3
            i32.const 29
            i32.shr_u
            local.set 6
            local.get 3
            i32.const 1
            i32.shl
            local.set 3
            local.get 4
            local.get 6
            i32.const 4
            i32.and
            i32.add
            i32.const 16
            i32.add
            local.tee 2
            i32.load
            local.tee 6
            br_if 0 (;@3;)
          end
          local.get 2
          local.get 0
          i32.store
          local.get 0
          local.get 4
          i32.store offset=24
          local.get 0
          local.get 0
          i32.store offset=12
          local.get 0
          local.get 0
          i32.store offset=8
          return
        end
        local.get 4
        i32.load offset=8
        local.tee 1
        local.get 0
        i32.store offset=12
        local.get 4
        local.get 0
        i32.store offset=8
        local.get 0
        i32.const 0
        i32.store offset=24
        local.get 0
        local.get 4
        i32.store offset=12
        local.get 0
        local.get 1
        i32.store offset=8
      end
    )
    (func $internal_memalign (;14;) (type 1) (param i32 i32) (result i32)
      (local i32 i32 i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.const 16
          local.get 0
          i32.const 16
          i32.gt_u
          select
          local.tee 2
          local.get 2
          i32.const -1
          i32.add
          i32.and
          br_if 0 (;@2;)
          local.get 2
          local.set 0
          br 1 (;@1;)
        end
        i32.const 32
        local.set 3
        loop ;; label = @2
          local.get 3
          local.tee 0
          i32.const 1
          i32.shl
          local.set 3
          local.get 0
          local.get 2
          i32.lt_u
          br_if 0 (;@2;)
        end
      end
      block ;; label = @1
        i32.const -64
        local.get 0
        i32.sub
        local.get 1
        i32.gt_u
        br_if 0 (;@1;)
        i32.const 0
        i32.const 48
        i32.store offset=1049076
        i32.const 0
        return
      end
      block ;; label = @1
        local.get 0
        i32.const 16
        local.get 1
        i32.const 19
        i32.add
        i32.const -16
        i32.and
        local.get 1
        i32.const 11
        i32.lt_u
        select
        local.tee 1
        i32.add
        i32.const 12
        i32.add
        call $dlmalloc
        local.tee 3
        br_if 0 (;@1;)
        i32.const 0
        return
      end
      local.get 3
      i32.const -8
      i32.add
      local.set 2
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.const -1
          i32.add
          local.get 3
          i32.and
          br_if 0 (;@2;)
          local.get 2
          local.set 0
          br 1 (;@1;)
        end
        local.get 3
        i32.const -4
        i32.add
        local.tee 4
        i32.load
        local.tee 5
        i32.const -8
        i32.and
        local.get 3
        local.get 0
        i32.add
        i32.const -1
        i32.add
        i32.const 0
        local.get 0
        i32.sub
        i32.and
        i32.const -8
        i32.add
        local.tee 3
        i32.const 0
        local.get 0
        local.get 3
        local.get 2
        i32.sub
        i32.const 15
        i32.gt_u
        select
        i32.add
        local.tee 0
        local.get 2
        i32.sub
        local.tee 3
        i32.sub
        local.set 6
        block ;; label = @2
          local.get 5
          i32.const 3
          i32.and
          br_if 0 (;@2;)
          local.get 0
          local.get 6
          i32.store offset=4
          local.get 0
          local.get 2
          i32.load
          local.get 3
          i32.add
          i32.store
          br 1 (;@1;)
        end
        local.get 0
        local.get 6
        local.get 0
        i32.load offset=4
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store offset=4
        local.get 0
        local.get 6
        i32.add
        local.tee 6
        local.get 6
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 4
        local.get 3
        local.get 4
        i32.load
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store
        local.get 2
        local.get 3
        i32.add
        local.tee 6
        local.get 6
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 2
        local.get 3
        call $dispose_chunk
      end
      block ;; label = @1
        local.get 0
        i32.load offset=4
        local.tee 3
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@1;)
        local.get 3
        i32.const -8
        i32.and
        local.tee 2
        local.get 1
        i32.const 16
        i32.add
        i32.le_u
        br_if 0 (;@1;)
        local.get 0
        local.get 1
        local.get 3
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store offset=4
        local.get 0
        local.get 1
        i32.add
        local.tee 3
        local.get 2
        local.get 1
        i32.sub
        local.tee 1
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 0
        local.get 2
        i32.add
        local.tee 2
        local.get 2
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 3
        local.get 1
        call $dispose_chunk
      end
      local.get 0
      i32.const 8
      i32.add
    )
    (func $aligned_alloc (;15;) (type 1) (param i32 i32) (result i32)
      block ;; label = @1
        local.get 0
        i32.const 16
        i32.gt_u
        br_if 0 (;@1;)
        local.get 1
        call $dlmalloc
        return
      end
      local.get 0
      local.get 1
      call $internal_memalign
    )
    (func $abort (;16;) (type 0)
      unreachable
      unreachable
    )
    (func $sbrk (;17;) (type 3) (param i32) (result i32)
      block ;; label = @1
        local.get 0
        br_if 0 (;@1;)
        memory.size
        i32.const 16
        i32.shl
        return
      end
      block ;; label = @1
        local.get 0
        i32.const 65535
        i32.and
        br_if 0 (;@1;)
        local.get 0
        i32.const -1
        i32.le_s
        br_if 0 (;@1;)
        block ;; label = @2
          local.get 0
          i32.const 16
          i32.shr_u
          memory.grow
          local.tee 0
          i32.const -1
          i32.ne
          br_if 0 (;@2;)
          i32.const 0
          i32.const 48
          i32.store offset=1049076
          i32.const -1
          return
        end
        local.get 0
        i32.const 16
        i32.shl
        return
      end
      call $abort
      unreachable
    )
    (func $memcpy (;18;) (type 6) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 2
            i32.const 32
            i32.gt_u
            br_if 0 (;@3;)
            local.get 1
            i32.const 3
            i32.and
            i32.eqz
            br_if 1 (;@2;)
            local.get 2
            i32.eqz
            br_if 1 (;@2;)
            local.get 0
            local.get 1
            i32.load8_u
            i32.store8
            local.get 2
            i32.const -1
            i32.add
            local.set 3
            local.get 0
            i32.const 1
            i32.add
            local.set 4
            local.get 1
            i32.const 1
            i32.add
            local.tee 5
            i32.const 3
            i32.and
            i32.eqz
            br_if 2 (;@1;)
            local.get 3
            i32.eqz
            br_if 2 (;@1;)
            local.get 0
            local.get 1
            i32.load8_u offset=1
            i32.store8 offset=1
            local.get 2
            i32.const -2
            i32.add
            local.set 3
            local.get 0
            i32.const 2
            i32.add
            local.set 4
            local.get 1
            i32.const 2
            i32.add
            local.tee 5
            i32.const 3
            i32.and
            i32.eqz
            br_if 2 (;@1;)
            local.get 3
            i32.eqz
            br_if 2 (;@1;)
            local.get 0
            local.get 1
            i32.load8_u offset=2
            i32.store8 offset=2
            local.get 2
            i32.const -3
            i32.add
            local.set 3
            local.get 0
            i32.const 3
            i32.add
            local.set 4
            local.get 1
            i32.const 3
            i32.add
            local.tee 5
            i32.const 3
            i32.and
            i32.eqz
            br_if 2 (;@1;)
            local.get 3
            i32.eqz
            br_if 2 (;@1;)
            local.get 0
            local.get 1
            i32.load8_u offset=3
            i32.store8 offset=3
            local.get 2
            i32.const -4
            i32.add
            local.set 3
            local.get 0
            i32.const 4
            i32.add
            local.set 4
            local.get 1
            i32.const 4
            i32.add
            local.set 5
            br 2 (;@1;)
          end
          local.get 0
          local.get 1
          local.get 2
          memory.copy
          local.get 0
          return
        end
        local.get 2
        local.set 3
        local.get 0
        local.set 4
        local.get 1
        local.set 5
      end
      block ;; label = @1
        block ;; label = @2
          local.get 4
          i32.const 3
          i32.and
          local.tee 2
          br_if 0 (;@2;)
          block ;; label = @3
            block ;; label = @4
              local.get 3
              i32.const 16
              i32.ge_u
              br_if 0 (;@4;)
              local.get 3
              local.set 2
              br 1 (;@3;)
            end
            block ;; label = @4
              local.get 3
              i32.const -16
              i32.add
              local.tee 2
              i32.const 16
              i32.and
              br_if 0 (;@4;)
              local.get 4
              local.get 5
              i64.load align=4
              i64.store align=4
              local.get 4
              local.get 5
              i64.load offset=8 align=4
              i64.store offset=8 align=4
              local.get 4
              i32.const 16
              i32.add
              local.set 4
              local.get 5
              i32.const 16
              i32.add
              local.set 5
              local.get 2
              local.set 3
            end
            local.get 2
            i32.const 16
            i32.lt_u
            br_if 0 (;@3;)
            local.get 3
            local.set 2
            loop ;; label = @4
              local.get 4
              local.get 5
              i64.load align=4
              i64.store align=4
              local.get 4
              local.get 5
              i64.load offset=8 align=4
              i64.store offset=8 align=4
              local.get 4
              local.get 5
              i64.load offset=16 align=4
              i64.store offset=16 align=4
              local.get 4
              local.get 5
              i64.load offset=24 align=4
              i64.store offset=24 align=4
              local.get 4
              i32.const 32
              i32.add
              local.set 4
              local.get 5
              i32.const 32
              i32.add
              local.set 5
              local.get 2
              i32.const -32
              i32.add
              local.tee 2
              i32.const 15
              i32.gt_u
              br_if 0 (;@4;)
            end
          end
          block ;; label = @3
            local.get 2
            i32.const 8
            i32.lt_u
            br_if 0 (;@3;)
            local.get 4
            local.get 5
            i64.load align=4
            i64.store align=4
            local.get 5
            i32.const 8
            i32.add
            local.set 5
            local.get 4
            i32.const 8
            i32.add
            local.set 4
          end
          block ;; label = @3
            local.get 2
            i32.const 4
            i32.and
            i32.eqz
            br_if 0 (;@3;)
            local.get 4
            local.get 5
            i32.load
            i32.store
            local.get 5
            i32.const 4
            i32.add
            local.set 5
            local.get 4
            i32.const 4
            i32.add
            local.set 4
          end
          block ;; label = @3
            local.get 2
            i32.const 2
            i32.and
            i32.eqz
            br_if 0 (;@3;)
            local.get 4
            local.get 5
            i32.load16_u align=1
            i32.store16 align=1
            local.get 4
            i32.const 2
            i32.add
            local.set 4
            local.get 5
            i32.const 2
            i32.add
            local.set 5
          end
          local.get 2
          i32.const 1
          i32.and
          i32.eqz
          br_if 1 (;@1;)
          local.get 4
          local.get 5
          i32.load8_u
          i32.store8
          local.get 0
          return
        end
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  local.get 3
                  i32.const 32
                  i32.lt_u
                  br_if 0 (;@6;)
                  block ;; label = @7
                    block ;; label = @8
                      local.get 2
                      i32.const -1
                      i32.add
                      br_table 3 (;@5;) 0 (;@8;) 1 (;@7;) 7 (;@1;)
                    end
                    local.get 4
                    local.get 5
                    i32.load
                    i32.store16 align=1
                    local.get 4
                    local.get 5
                    i32.const 2
                    i32.add
                    i32.load align=2
                    i32.store offset=2
                    local.get 4
                    local.get 5
                    i32.const 6
                    i32.add
                    i64.load align=2
                    i64.store offset=6 align=4
                    local.get 4
                    i32.const 18
                    i32.add
                    local.set 2
                    local.get 5
                    i32.const 18
                    i32.add
                    local.set 1
                    i32.const 14
                    local.set 6
                    local.get 5
                    i32.const 14
                    i32.add
                    i32.load align=2
                    local.set 5
                    i32.const 14
                    local.set 3
                    br 3 (;@4;)
                  end
                  local.get 4
                  local.get 5
                  i32.load
                  i32.store8
                  local.get 4
                  local.get 5
                  i32.const 1
                  i32.add
                  i32.load align=1
                  i32.store offset=1
                  local.get 4
                  local.get 5
                  i32.const 5
                  i32.add
                  i64.load align=1
                  i64.store offset=5 align=4
                  local.get 4
                  i32.const 17
                  i32.add
                  local.set 2
                  local.get 5
                  i32.const 17
                  i32.add
                  local.set 1
                  i32.const 13
                  local.set 6
                  local.get 5
                  i32.const 13
                  i32.add
                  i32.load align=1
                  local.set 5
                  i32.const 15
                  local.set 3
                  br 2 (;@4;)
                end
                block ;; label = @6
                  block ;; label = @7
                    local.get 3
                    i32.const 16
                    i32.ge_u
                    br_if 0 (;@7;)
                    local.get 4
                    local.set 2
                    local.get 5
                    local.set 1
                    br 1 (;@6;)
                  end
                  local.get 4
                  local.get 5
                  i32.load8_u
                  i32.store8
                  local.get 4
                  local.get 5
                  i32.load offset=1 align=1
                  i32.store offset=1 align=1
                  local.get 4
                  local.get 5
                  i64.load offset=5 align=1
                  i64.store offset=5 align=1
                  local.get 4
                  local.get 5
                  i32.load16_u offset=13 align=1
                  i32.store16 offset=13 align=1
                  local.get 4
                  local.get 5
                  i32.load8_u offset=15
                  i32.store8 offset=15
                  local.get 4
                  i32.const 16
                  i32.add
                  local.set 2
                  local.get 5
                  i32.const 16
                  i32.add
                  local.set 1
                end
                local.get 3
                i32.const 8
                i32.and
                br_if 2 (;@3;)
                br 3 (;@2;)
              end
              local.get 4
              local.get 5
              i32.load
              local.tee 2
              i32.store8
              local.get 4
              local.get 2
              i32.const 16
              i32.shr_u
              i32.store8 offset=2
              local.get 4
              local.get 2
              i32.const 8
              i32.shr_u
              i32.store8 offset=1
              local.get 4
              local.get 5
              i32.const 3
              i32.add
              i32.load align=1
              i32.store offset=3
              local.get 4
              local.get 5
              i32.const 7
              i32.add
              i64.load align=1
              i64.store offset=7 align=4
              local.get 4
              i32.const 19
              i32.add
              local.set 2
              local.get 5
              i32.const 19
              i32.add
              local.set 1
              i32.const 15
              local.set 6
              local.get 5
              i32.const 15
              i32.add
              i32.load align=1
              local.set 5
              i32.const 13
              local.set 3
            end
            local.get 4
            local.get 6
            i32.add
            local.get 5
            i32.store
          end
          local.get 2
          local.get 1
          i64.load align=1
          i64.store align=1
          local.get 2
          i32.const 8
          i32.add
          local.set 2
          local.get 1
          i32.const 8
          i32.add
          local.set 1
        end
        block ;; label = @2
          local.get 3
          i32.const 4
          i32.and
          i32.eqz
          br_if 0 (;@2;)
          local.get 2
          local.get 1
          i32.load align=1
          i32.store align=1
          local.get 2
          i32.const 4
          i32.add
          local.set 2
          local.get 1
          i32.const 4
          i32.add
          local.set 1
        end
        block ;; label = @2
          local.get 3
          i32.const 2
          i32.and
          i32.eqz
          br_if 0 (;@2;)
          local.get 2
          local.get 1
          i32.load16_u align=1
          i32.store16 align=1
          local.get 2
          i32.const 2
          i32.add
          local.set 2
          local.get 1
          i32.const 2
          i32.add
          local.set 1
        end
        local.get 3
        i32.const 1
        i32.and
        i32.eqz
        br_if 0 (;@1;)
        local.get 2
        local.get 1
        i32.load8_u
        i32.store8
      end
      local.get 0
    )
    (table (;0;) 1 1 funcref)
    (memory (;0;) 17)
    (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
    (export "memory" (memory 0))
    (export "miden:add/add@1.0.0#add" (func $miden:add/add@1.0.0#add))
    (export "cabi_realloc" (func $cabi_realloc))
  )
  (core instance (;0;) (instantiate 0))
  (alias core export 0 "memory" (core memory (;0;)))
  (alias core export 0 "cabi_realloc" (core func (;0;)))
  (type (;0;) (func (param "a" u32) (param "b" u32) (result u32)))
  (alias core export 0 "miden:add/add@1.0.0#add" (core func (;1;)))
  (func (;0;) (type 0) (canon lift (core func 1)))
  (component (;0;)
    (type (;0;) (func (param "a" u32) (param "b" u32) (result u32)))
    (import "import-func-add" (func (;0;) (type 0)))
    (type (;1;) (func (param "a" u32) (param "b" u32) (result u32)))
    (export (;1;) "add" (func 0) (func (type 1)))
  )
  (instance (;0;) (instantiate 0
      (with "import-func-add" (func 0))
    )
  )
  (export (;1;) "miden:add/add@1.0.0" (instance 0))
)