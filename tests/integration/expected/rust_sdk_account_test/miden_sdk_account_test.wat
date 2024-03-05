(module
  (type (;0;) (func (param i32) (result i32)))
  (type (;1;) (func))
  (type (;2;) (func (param i32 i32 i32)))
  (type (;3;) (func (param i32 i32 i32 i32)))
  (import "env" "miden_sdk_tx_kernel_get_inputs_mast_0x000000000000000000" (func $miden_sdk_tx_kernel_get_inputs_mast_0x000000000000000000 (;0;) (type 0)))
  (func $note_script (;1;) (type 1)
    (local i32 i32 i32 i32 i32 i64)
    global.get $__stack_pointer
    i32.const 2048
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    block ;; label = @1
      local.get 0
      call $miden_sdk_tx_kernel_get_inputs_mast_0x000000000000000000
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const -1
      i32.add
      i32.const 536870911
      i32.and
      local.tee 2
      i32.const 1
      i32.add
      local.tee 3
      i32.const 7
      i32.and
      local.set 4
      block ;; label = @2
        block ;; label = @3
          local.get 2
          i32.const 7
          i32.ge_u
          br_if 0 (;@3;)
          i64.const 0
          local.set 5
          local.get 0
          local.set 2
          br 1 (;@2;)
        end
        local.get 3
        i32.const 1073741816
        i32.and
        local.set 3
        i64.const 0
        local.set 5
        local.get 0
        local.set 2
        loop ;; label = @3
          local.get 2
          i32.const 56
          i32.add
          i64.load
          local.get 2
          i32.const 48
          i32.add
          i64.load
          local.get 2
          i32.const 40
          i32.add
          i64.load
          local.get 2
          i32.const 32
          i32.add
          i64.load
          local.get 2
          i32.const 24
          i32.add
          i64.load
          local.get 2
          i32.const 16
          i32.add
          i64.load
          local.get 2
          i32.const 8
          i32.add
          i64.load
          local.get 2
          i64.load
          local.get 5
          i64.add
          i64.add
          i64.add
          i64.add
          i64.add
          i64.add
          i64.add
          i64.add
          local.set 5
          local.get 2
          i32.const 64
          i32.add
          local.set 2
          local.get 3
          i32.const -8
          i32.add
          local.tee 3
          br_if 0 (;@3;)
        end
      end
      block ;; label = @2
        local.get 4
        i32.eqz
        br_if 0 (;@2;)
        loop ;; label = @3
          local.get 2
          i64.load
          local.get 5
          i64.add
          local.set 5
          local.get 2
          i32.const 8
          i32.add
          local.set 2
          local.get 4
          i32.const -1
          i32.add
          local.tee 4
          br_if 0 (;@3;)
        end
      end
      local.get 0
      local.get 1
      i32.const 3
      i32.shl
      i32.const 8
      call $__rust_dealloc
      local.get 5
      i64.const 42
      i64.ne
      br_if 0 (;@1;)
      local.get 0
      i32.const 2048
      i32.add
      global.set $__stack_pointer
      return
    end
    unreachable
    unreachable
  )
  (func $__rust_dealloc (;2;) (type 2) (param i32 i32 i32)
    i32.const 1048576
    local.get 0
    local.get 2
    local.get 1
    call $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::dealloc
  )
  (func $<wee_alloc::WeeAlloc as core::alloc::global::GlobalAlloc>::dealloc (;3;) (type 3) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      local.get 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load
      local.set 4
      local.get 1
      i32.const 0
      i32.store
      local.get 1
      i32.const -8
      i32.add
      local.tee 3
      local.get 3
      i32.load
      local.tee 5
      i32.const -2
      i32.and
      local.tee 6
      i32.store
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 3
              i32.const 4
              i32.add
              local.tee 7
              i32.load
              i32.const -4
              i32.and
              local.tee 8
              i32.eqz
              br_if 0 (;@5;)
              local.get 8
              i32.load
              local.tee 9
              i32.const 1
              i32.and
              br_if 0 (;@5;)
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    local.get 5
                    i32.const -4
                    i32.and
                    local.tee 10
                    br_if 0 (;@8;)
                    local.get 8
                    local.set 1
                    br 1 (;@7;)
                  end
                  local.get 8
                  local.set 1
                  local.get 5
                  i32.const 2
                  i32.and
                  br_if 0 (;@7;)
                  local.get 10
                  local.get 10
                  i32.load offset=4
                  i32.const 3
                  i32.and
                  local.get 8
                  i32.or
                  i32.store offset=4
                  local.get 3
                  i32.load
                  local.set 6
                  local.get 7
                  i32.load
                  local.tee 5
                  i32.const -4
                  i32.and
                  local.tee 1
                  i32.eqz
                  br_if 1 (;@6;)
                  local.get 1
                  i32.load
                  local.set 9
                end
                local.get 1
                local.get 6
                i32.const -4
                i32.and
                local.get 9
                i32.const 3
                i32.and
                i32.or
                i32.store
                local.get 7
                i32.load
                local.set 5
                local.get 3
                i32.load
                local.set 6
              end
              local.get 7
              local.get 5
              i32.const 3
              i32.and
              i32.store
              local.get 3
              local.get 6
              i32.const 3
              i32.and
              i32.store
              local.get 6
              i32.const 2
              i32.and
              i32.eqz
              br_if 1 (;@4;)
              local.get 8
              local.get 8
              i32.load
              i32.const 2
              i32.or
              i32.store
              br 1 (;@4;)
            end
            local.get 5
            i32.const -4
            i32.and
            local.tee 8
            i32.eqz
            br_if 1 (;@3;)
            local.get 5
            i32.const 2
            i32.and
            br_if 1 (;@3;)
            local.get 8
            i32.load8_u
            i32.const 1
            i32.and
            br_if 1 (;@3;)
            local.get 1
            local.get 8
            i32.load offset=8
            i32.const -4
            i32.and
            i32.store
            local.get 8
            local.get 3
            i32.const 1
            i32.or
            i32.store offset=8
          end
          local.get 4
          local.set 3
          br 1 (;@2;)
        end
        local.get 1
        local.get 4
        i32.store
      end
      local.get 0
      local.get 3
      i32.store
    end
  )
  (table (;0;) 1 1 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (export "memory" (memory 0))
  (export "note_script" (func $note_script))
)