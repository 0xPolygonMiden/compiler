(component
  (type (;0;)
    (instance
      (type (;0;) u64)
      (export (;1;) "felt" (type (eq 0)))
      (export (;2;) "account-id" (type (eq 1)))
      (type (;3;) (tuple 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1))
      (export (;4;) "note-inputs" (type (eq 3)))
      (type (;5;) (record (field "asset" 2) (field "amount" u64)))
      (export (;6;) "fungible-asset" (type (eq 5)))
      (type (;7;) (tuple 1 1 1 1))
      (export (;8;) "word" (type (eq 7)))
      (export (;9;) "non-fungible-asset" (type (eq 8)))
      (type (;10;) (variant (case "fungible" 6) (case "non-fungible" 9)))
      (export (;11;) "asset" (type (eq 10)))
    )
  )
  (import "miden:base/types@1.0.0" (instance (;0;) (type 0)))
  (alias export 0 "account-id" (type (;1;)))
  (alias export 0 "note-inputs" (type (;2;)))
  (alias export 0 "asset" (type (;3;)))
  (type (;4;)
    (instance
      (alias outer 1 1 (type (;0;)))
      (export (;1;) "account-id" (type (eq 0)))
      (alias outer 1 2 (type (;2;)))
      (export (;3;) "note-inputs" (type (eq 2)))
      (alias outer 1 3 (type (;4;)))
      (export (;5;) "asset" (type (eq 4)))
      (type (;6;) (func (result 1)))
      (export (;0;) "get-id" (func (type 6)))
      (type (;7;) (func (result 3)))
      (export (;1;) "get-inputs" (func (type 7)))
      (type (;8;) (list 5))
      (type (;9;) (func (result 8)))
      (export (;2;) "get-assets" (func (type 9)))
    )
  )
  (import "miden:base/tx-kernel@1.0.0" (instance (;1;) (type 4)))
  (alias export 0 "asset" (type (;5;)))
  (type (;6;)
    (instance
      (alias outer 1 5 (type (;0;)))
      (export (;1;) "asset" (type (eq 0)))
      (type (;2;) (func (param "asset" 1)))
      (export (;0;) "receive-asset" (func (type 2)))
    )
  )
  (import "miden:basic-wallet/basic-wallet@1.0.0" (instance (;2;) (type 6)))
  (type (;7;)
    (instance
      (export (;0;) "error" (type (sub resource)))
    )
  )
  (import "wasi:io/error@0.2.0-rc-2023-11-10" (instance (;3;) (type 7)))
  (alias export 3 "error" (type (;8;)))
  (type (;9;)
    (instance
      (export (;0;) "output-stream" (type (sub resource)))
      (alias outer 1 8 (type (;1;)))
      (export (;2;) "error" (type (eq 1)))
      (type (;3;) (own 2))
      (type (;4;) (variant (case "last-operation-failed" 3) (case "closed")))
      (export (;5;) "stream-error" (type (eq 4)))
      (export (;6;) "input-stream" (type (sub resource)))
      (type (;7;) (borrow 0))
      (type (;8;) (result u64 (error 5)))
      (type (;9;) (func (param "self" 7) (result 8)))
      (export (;0;) "[method]output-stream.check-write" (func (type 9)))
      (type (;10;) (list u8))
      (type (;11;) (result (error 5)))
      (type (;12;) (func (param "self" 7) (param "contents" 10) (result 11)))
      (export (;1;) "[method]output-stream.write" (func (type 12)))
      (export (;2;) "[method]output-stream.blocking-write-and-flush" (func (type 12)))
      (type (;13;) (func (param "self" 7) (result 11)))
      (export (;3;) "[method]output-stream.blocking-flush" (func (type 13)))
    )
  )
  (import "wasi:io/streams@0.2.0-rc-2023-11-10" (instance (;4;) (type 9)))
  (alias export 4 "input-stream" (type (;10;)))
  (type (;11;)
    (instance
      (alias outer 1 10 (type (;0;)))
      (export (;1;) "input-stream" (type (eq 0)))
      (type (;2;) (own 1))
      (type (;3;) (func (result 2)))
      (export (;0;) "get-stdin" (func (type 3)))
    )
  )
  (import "wasi:cli/stdin@0.2.0-rc-2023-12-05" (instance (;5;) (type 11)))
  (alias export 4 "output-stream" (type (;12;)))
  (type (;13;)
    (instance
      (alias outer 1 12 (type (;0;)))
      (export (;1;) "output-stream" (type (eq 0)))
      (type (;2;) (own 1))
      (type (;3;) (func (result 2)))
      (export (;0;) "get-stdout" (func (type 3)))
    )
  )
  (import "wasi:cli/stdout@0.2.0-rc-2023-12-05" (instance (;6;) (type 13)))
  (alias export 4 "output-stream" (type (;14;)))
  (type (;15;)
    (instance
      (alias outer 1 14 (type (;0;)))
      (export (;1;) "output-stream" (type (eq 0)))
      (type (;2;) (own 1))
      (type (;3;) (func (result 2)))
      (export (;0;) "get-stderr" (func (type 3)))
    )
  )
  (import "wasi:cli/stderr@0.2.0-rc-2023-12-05" (instance (;7;) (type 15)))
  (type (;16;)
    (instance
      (type (;0;) (record (field "seconds" u64) (field "nanoseconds" u32)))
      (export (;1;) "datetime" (type (eq 0)))
    )
  )
  (import "wasi:clocks/wall-clock@0.2.0-rc-2023-11-10" (instance (;8;) (type 16)))
  (alias export 4 "output-stream" (type (;17;)))
  (alias export 8 "datetime" (type (;18;)))
  (alias export 4 "error" (type (;19;)))
  (type (;20;)
    (instance
      (export (;0;) "descriptor" (type (sub resource)))
      (type (;1;) u64)
      (export (;2;) "filesize" (type (eq 1)))
      (alias outer 1 17 (type (;3;)))
      (export (;4;) "output-stream" (type (eq 3)))
      (type (;5;) (enum "access" "would-block" "already" "bad-descriptor" "busy" "deadlock" "quota" "exist" "file-too-large" "illegal-byte-sequence" "in-progress" "interrupted" "invalid" "io" "is-directory" "loop" "too-many-links" "message-size" "name-too-long" "no-device" "no-entry" "no-lock" "insufficient-memory" "insufficient-space" "not-directory" "not-empty" "not-recoverable" "unsupported" "no-tty" "no-such-device" "overflow" "not-permitted" "pipe" "read-only" "invalid-seek" "text-file-busy" "cross-device"))
      (export (;6;) "error-code" (type (eq 5)))
      (type (;7;) (enum "unknown" "block-device" "character-device" "directory" "fifo" "symbolic-link" "regular-file" "socket"))
      (export (;8;) "descriptor-type" (type (eq 7)))
      (type (;9;) u64)
      (export (;10;) "link-count" (type (eq 9)))
      (alias outer 1 18 (type (;11;)))
      (export (;12;) "datetime" (type (eq 11)))
      (type (;13;) (option 12))
      (type (;14;) (record (field "type" 8) (field "link-count" 10) (field "size" 2) (field "data-access-timestamp" 13) (field "data-modification-timestamp" 13) (field "status-change-timestamp" 13)))
      (export (;15;) "descriptor-stat" (type (eq 14)))
      (alias outer 1 19 (type (;16;)))
      (export (;17;) "error" (type (eq 16)))
      (type (;18;) (borrow 0))
      (type (;19;) (own 4))
      (type (;20;) (result 19 (error 6)))
      (type (;21;) (func (param "self" 18) (param "offset" 2) (result 20)))
      (export (;0;) "[method]descriptor.write-via-stream" (func (type 21)))
      (type (;22;) (func (param "self" 18) (result 20)))
      (export (;1;) "[method]descriptor.append-via-stream" (func (type 22)))
      (type (;23;) (result 8 (error 6)))
      (type (;24;) (func (param "self" 18) (result 23)))
      (export (;2;) "[method]descriptor.get-type" (func (type 24)))
      (type (;25;) (result 15 (error 6)))
      (type (;26;) (func (param "self" 18) (result 25)))
      (export (;3;) "[method]descriptor.stat" (func (type 26)))
      (type (;27;) (borrow 17))
      (type (;28;) (option 6))
      (type (;29;) (func (param "err" 27) (result 28)))
      (export (;4;) "filesystem-error-code" (func (type 29)))
    )
  )
  (import "wasi:filesystem/types@0.2.0-rc-2023-11-10" (instance (;9;) (type 20)))
  (alias export 9 "descriptor" (type (;21;)))
  (type (;22;)
    (instance
      (alias outer 1 21 (type (;0;)))
      (export (;1;) "descriptor" (type (eq 0)))
      (type (;2;) (own 1))
      (type (;3;) (tuple 2 string))
      (type (;4;) (list 3))
      (type (;5;) (func (result 4)))
      (export (;0;) "get-directories" (func (type 5)))
    )
  )
  (import "wasi:filesystem/preopens@0.2.0-rc-2023-11-10" (instance (;10;) (type 22)))
  (core module (;0;)
    (type $.rodata (;0;) (func (param i32)))
    (type (;1;) (func (param i32 i32)))
    (type (;2;) (func (param i32 i32 i32) (result i32)))
    (type (;3;) (func (param i32 i32) (result i32)))
    (type (;4;) (func (result i64)))
    (type (;5;) (func (param i32 i64 i64 i64 i64)))
    (type (;6;) (func (param i32 i32 i32 i32) (result i32)))
    (type (;7;) (func))
    (type (;8;) (func (param i32 i32 i32 i32)))
    (type (;9;) (func (param i32 i32 i32)))
    (type (;10;) (func (param i32) (result i32)))
    (type (;11;) (func (param i32 i32 i32 i32 i32 i32) (result i32)))
    (type (;12;) (func (param i32 i32 i32 i32 i32) (result i32)))
    (type (;13;) (func (param i64 i32 i32) (result i32)))
    (import "miden:base/tx-kernel@1.0.0" "get-inputs" (func $basic_wallet_p2id_note::bindings::miden::base::tx_kernel::get_inputs::wit_import (;0;) (type $.rodata)))
    (import "miden:base/tx-kernel@1.0.0" "get-id" (func $basic_wallet_p2id_note::bindings::miden::base::tx_kernel::get_id::wit_import (;1;) (type 4)))
    (import "miden:base/tx-kernel@1.0.0" "get-assets" (func $basic_wallet_p2id_note::bindings::miden::base::tx_kernel::get_assets::wit_import (;2;) (type $.rodata)))
    (import "miden:basic-wallet/basic-wallet@1.0.0" "receive-asset" (func $basic_wallet_p2id_note::bindings::miden::basic_wallet::basic_wallet::receive_asset::wit_import (;3;) (type 5)))
    (import "wasi_snapshot_preview1" "fd_write" (func $wasi::lib_generated::wasi_snapshot_preview1::fd_write (;4;) (type 6)))
    (func $__wasm_call_ctors (;5;) (type 7))
    (func $alloc::raw_vec::finish_grow (;6;) (type 8) (param i32 i32 i32 i32)
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
                    i32.load8_u offset=1048953
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
                i32.load8_u offset=1048953
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
    (func $alloc::raw_vec::RawVec<T,A>::reserve_for_push (;7;) (type 1) (param i32 i32)
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
          i32.const 40
          i32.mul
          local.set 4
          local.get 1
          i32.const 53687092
          i32.lt_u
          i32.const 3
          i32.shl
          local.set 5
          block ;; label = @3
            block ;; label = @4
              local.get 3
              i32.eqz
              br_if 0 (;@4;)
              local.get 2
              i32.const 8
              i32.store offset=24
              local.get 2
              local.get 3
              i32.const 40
              i32.mul
              i32.store offset=28
              local.get 2
              local.get 0
              i32.load
              i32.store offset=20
              br 1 (;@3;)
            end
            local.get 2
            i32.const 0
            i32.store offset=24
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
          local.get 3
          i32.eqz
          br_if 0 (;@2;)
          local.get 3
          local.get 2
          i32.const 16
          i32.add
          i32.load
          call $alloc::alloc::handle_alloc_error
          unreachable
        end
        call $alloc::raw_vec::capacity_overflow
        unreachable
      end
      local.get 2
      i32.const 32
      i32.add
      global.set $__stack_pointer
    )
    (func $miden:base/note@1.0.0#note-script (;8;) (type 7)
      (local i32 i32 i32 i32 i32 i32 i32 i64 i64 i64 i64 i64 i32)
      global.get $__stack_pointer
      i32.const 144
      i32.sub
      local.tee 0
      global.set $__stack_pointer
      call $wit_bindgen::rt::run_ctors_once
      local.get 0
      i32.const 8
      i32.add
      call $basic_wallet_p2id_note::bindings::miden::base::tx_kernel::get_inputs::wit_import
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 0
              i64.load offset=8
              call $basic_wallet_p2id_note::bindings::miden::base::tx_kernel::get_id::wit_import
              i64.ne
              br_if 0 (;@4;)
              local.get 0
              i32.const 136
              i32.add
              call $basic_wallet_p2id_note::bindings::miden::base::tx_kernel::get_assets::wit_import
              local.get 0
              i32.load offset=136
              local.set 1
              block ;; label = @5
                block ;; label = @6
                  local.get 0
                  i32.const 140
                  i32.add
                  i32.load
                  local.tee 2
                  i32.eqz
                  br_if 0 (;@6;)
                  local.get 2
                  i32.const 53687091
                  i32.gt_u
                  br_if 1 (;@5;)
                  local.get 2
                  i32.const 40
                  i32.mul
                  local.tee 3
                  i32.const -1
                  i32.le_s
                  br_if 1 (;@5;)
                  i32.const 8
                  local.set 4
                  block ;; label = @7
                    local.get 3
                    i32.eqz
                    br_if 0 (;@7;)
                    i32.const 0
                    i32.load8_u offset=1048953
                    drop
                    local.get 3
                    i32.const 8
                    call $__rust_alloc
                    local.tee 4
                    i32.eqz
                    br_if 4 (;@3;)
                  end
                  i32.const 0
                  local.set 5
                  local.get 0
                  i32.const 0
                  i32.store offset=16
                  local.get 0
                  local.get 2
                  i32.store offset=12
                  local.get 0
                  local.get 4
                  i32.store offset=8
                  local.get 1
                  local.set 6
                  loop ;; label = @7
                    block ;; label = @8
                      block ;; label = @9
                        local.get 6
                        i32.load8_u
                        br_if 0 (;@9;)
                        i64.const 0
                        local.set 7
                        br 1 (;@8;)
                      end
                      local.get 6
                      i32.const 32
                      i32.add
                      i64.load
                      local.set 8
                      local.get 6
                      i32.const 24
                      i32.add
                      i64.load
                      local.set 9
                      i64.const 1
                      local.set 7
                    end
                    local.get 6
                    i32.const 8
                    i32.add
                    i64.load
                    local.set 10
                    local.get 6
                    i32.const 16
                    i32.add
                    i64.load
                    local.set 11
                    block ;; label = @8
                      local.get 5
                      local.get 0
                      i32.load offset=12
                      i32.ne
                      br_if 0 (;@8;)
                      local.get 0
                      i32.const 8
                      i32.add
                      local.get 5
                      call $alloc::raw_vec::RawVec<T,A>::reserve_for_push
                      local.get 0
                      i32.load offset=8
                      local.set 4
                      local.get 0
                      i32.load offset=16
                      local.set 5
                    end
                    local.get 4
                    local.get 5
                    i32.const 40
                    i32.mul
                    i32.add
                    local.tee 12
                    local.get 8
                    i64.store offset=32
                    local.get 12
                    local.get 9
                    i64.store offset=24
                    local.get 12
                    local.get 11
                    i64.store offset=16
                    local.get 12
                    local.get 10
                    i64.store offset=8
                    local.get 12
                    local.get 7
                    i64.store
                    local.get 0
                    local.get 5
                    i32.const 1
                    i32.add
                    local.tee 5
                    i32.store offset=16
                    local.get 6
                    i32.const 40
                    i32.add
                    local.set 6
                    local.get 2
                    i32.const -1
                    i32.add
                    local.tee 2
                    br_if 0 (;@7;)
                  end
                  local.get 0
                  i32.load offset=12
                  local.set 4
                  local.get 0
                  i32.load offset=8
                  local.set 2
                  local.get 1
                  local.get 3
                  i32.const 8
                  call $wit_bindgen::rt::dealloc
                  local.get 5
                  i32.eqz
                  br_if 4 (;@2;)
                  local.get 2
                  local.get 5
                  i32.const 40
                  i32.mul
                  i32.add
                  local.set 12
                  local.get 2
                  local.set 6
                  loop ;; label = @7
                    local.get 6
                    i64.load
                    local.tee 7
                    i64.const 2
                    i64.eq
                    br_if 5 (;@2;)
                    local.get 7
                    i64.const 0
                    i64.ne
                    local.tee 5
                    local.get 6
                    i64.load offset=8
                    local.get 6
                    i64.load offset=16
                    local.get 6
                    i64.load offset=24
                    i64.const 0
                    local.get 5
                    select
                    local.get 6
                    i64.load offset=32
                    i64.const 0
                    local.get 5
                    select
                    call $basic_wallet_p2id_note::bindings::miden::basic_wallet::basic_wallet::receive_asset::wit_import
                    local.get 6
                    i32.const 40
                    i32.add
                    local.tee 6
                    local.get 12
                    i32.ne
                    br_if 0 (;@7;)
                    br 5 (;@2;)
                  end
                end
                local.get 1
                i32.const 0
                i32.const 8
                call $wit_bindgen::rt::dealloc
                br 4 (;@1;)
              end
              call $alloc::raw_vec::capacity_overflow
              unreachable
            end
            unreachable
            unreachable
          end
          i32.const 8
          local.get 3
          call $alloc::alloc::handle_alloc_error
          unreachable
        end
        local.get 4
        i32.eqz
        br_if 0 (;@1;)
        local.get 2
        local.get 4
        i32.const 40
        i32.mul
        i32.const 8
        call $__rust_dealloc
      end
      local.get 0
      i32.const 144
      i32.add
      global.set $__stack_pointer
    )
    (func $__rust_alloc (;9;) (type 3) (param i32 i32) (result i32)
      (local i32)
      local.get 0
      local.get 1
      call $__rdl_alloc
      local.set 2
      local.get 2
      return
    )
    (func $__rust_dealloc (;10;) (type 9) (param i32 i32 i32)
      local.get 0
      local.get 1
      local.get 2
      call $__rdl_dealloc
      return
    )
    (func $__rust_realloc (;11;) (type 6) (param i32 i32 i32 i32) (result i32)
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
    (func $__rust_alloc_error_handler (;12;) (type 1) (param i32 i32)
      local.get 0
      local.get 1
      call $__rg_oom
      return
    )
    (func $wit_bindgen::rt::run_ctors_once (;13;) (type 7)
      block ;; label = @1
        i32.const 0
        i32.load8_u offset=1048954
        br_if 0 (;@1;)
        call $__wasm_call_ctors
        i32.const 0
        i32.const 1
        i32.store8 offset=1048954
      end
    )
    (func $cabi_realloc (;14;) (type 6) (param i32 i32 i32 i32) (result i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 1
            br_if 0 (;@3;)
            local.get 3
            i32.eqz
            br_if 2 (;@1;)
            i32.const 0
            i32.load8_u offset=1048953
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
    (func $wit_bindgen::rt::dealloc (;15;) (type 9) (param i32 i32 i32)
      block ;; label = @1
        local.get 1
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        local.get 1
        local.get 2
        call $__rust_dealloc
      end
    )
    (func $core::ptr::drop_in_place<std::io::Write::write_fmt::Adapter<std::sys::wasi::stdio::Stderr>> (;16;) (type $.rodata) (param i32)
      (local i32 i32 i32)
      local.get 0
      i32.load offset=4
      local.set 1
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.load8_u
          local.tee 0
          i32.const 4
          i32.gt_u
          br_if 0 (;@2;)
          local.get 0
          i32.const 3
          i32.ne
          br_if 1 (;@1;)
        end
        local.get 1
        i32.load
        local.tee 2
        local.get 1
        i32.const 4
        i32.add
        i32.load
        local.tee 0
        i32.load
        call_indirect (type $.rodata)
        block ;; label = @2
          local.get 0
          i32.load offset=4
          local.tee 3
          i32.eqz
          br_if 0 (;@2;)
          local.get 2
          local.get 3
          local.get 0
          i32.load offset=8
          call $__rust_dealloc
        end
        local.get 1
        i32.const 12
        i32.const 4
        call $__rust_dealloc
      end
    )
    (func $core::fmt::Write::write_char (;17;) (type 3) (param i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 2
      i32.const 0
      i32.store offset=8
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 1
              i32.const 128
              i32.lt_u
              br_if 0 (;@4;)
              local.get 1
              i32.const 2048
              i32.lt_u
              br_if 1 (;@3;)
              local.get 1
              i32.const 65536
              i32.ge_u
              br_if 2 (;@2;)
              local.get 2
              local.get 1
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=10
              local.get 2
              local.get 1
              i32.const 12
              i32.shr_u
              i32.const 224
              i32.or
              i32.store8 offset=8
              local.get 2
              local.get 1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=9
              i32.const 3
              local.set 1
              br 3 (;@1;)
            end
            local.get 2
            local.get 1
            i32.store8 offset=8
            i32.const 1
            local.set 1
            br 2 (;@1;)
          end
          local.get 2
          local.get 1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=9
          local.get 2
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 192
          i32.or
          i32.store8 offset=8
          i32.const 2
          local.set 1
          br 1 (;@1;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=11
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=10
        local.get 2
        local.get 1
        i32.const 12
        i32.shr_u
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=9
        local.get 2
        local.get 1
        i32.const 18
        i32.shr_u
        i32.const 7
        i32.and
        i32.const 240
        i32.or
        i32.store8 offset=8
        i32.const 4
        local.set 1
      end
      local.get 2
      i32.const 8
      i32.add
      local.set 3
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            loop ;; label = @4
              local.get 2
              local.get 1
              i32.store offset=16
              local.get 2
              local.get 3
              i32.store offset=12
              local.get 2
              i32.const 20
              i32.add
              i32.const 2
              local.get 2
              i32.const 12
              i32.add
              i32.const 1
              call $wasi::lib_generated::fd_write
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    local.get 2
                    i32.load16_u offset=20
                    br_if 0 (;@7;)
                    local.get 2
                    i32.load offset=24
                    local.tee 4
                    br_if 1 (;@6;)
                    i32.const 2
                    local.set 1
                    i32.const 1048604
                    local.set 4
                    br 5 (;@2;)
                  end
                  local.get 2
                  local.get 2
                  i32.load16_u offset=22
                  i32.store16 offset=30
                  local.get 2
                  i32.const 30
                  i32.add
                  call $wasi::lib_generated::Errno::raw
                  i32.const 65535
                  i32.and
                  local.tee 4
                  call $std::sys::wasi::decode_error_kind
                  i32.const 255
                  i32.and
                  i32.const 35
                  i32.eq
                  br_if 1 (;@5;)
                  i32.const 0
                  local.set 1
                  br 4 (;@2;)
                end
                local.get 1
                local.get 4
                i32.lt_u
                br_if 2 (;@3;)
                local.get 3
                local.get 4
                i32.add
                local.set 3
                local.get 1
                local.get 4
                i32.sub
                local.set 1
              end
              local.get 1
              br_if 0 (;@4;)
            end
            i32.const 0
            local.set 1
            br 2 (;@1;)
          end
          unreachable
          unreachable
        end
        local.get 0
        i32.load offset=4
        local.set 5
        block ;; label = @2
          block ;; label = @3
            local.get 0
            i32.load8_u
            local.tee 3
            i32.const 4
            i32.gt_u
            br_if 0 (;@3;)
            local.get 3
            i32.const 3
            i32.ne
            br_if 1 (;@2;)
          end
          local.get 5
          i32.load
          local.tee 6
          local.get 5
          i32.const 4
          i32.add
          i32.load
          local.tee 3
          i32.load
          call_indirect (type $.rodata)
          block ;; label = @3
            local.get 3
            i32.load offset=4
            local.tee 7
            i32.eqz
            br_if 0 (;@3;)
            local.get 6
            local.get 7
            local.get 3
            i32.load offset=8
            call $__rust_dealloc
          end
          local.get 5
          i32.const 12
          i32.const 4
          call $__rust_dealloc
        end
        local.get 0
        local.get 4
        i32.store offset=4
        local.get 0
        local.get 1
        i32.store
        i32.const 1
        local.set 1
      end
      local.get 2
      i32.const 32
      i32.add
      global.set $__stack_pointer
      local.get 1
    )
    (func $core::fmt::Write::write_fmt (;18;) (type 3) (param i32 i32) (result i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 2
      local.get 0
      i32.store offset=12
      local.get 2
      i32.const 12
      i32.add
      i32.const 1048640
      local.get 1
      call $core::fmt::write
      local.set 0
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $core::ptr::drop_in_place<&mut std::io::Write::write_fmt::Adapter<alloc::vec::Vec<u8>>> (;19;) (type $.rodata) (param i32))
    (func $<&mut W as core::fmt::Write>::write_char (;20;) (type 3) (param i32 i32) (result i32)
      local.get 0
      i32.load
      local.get 1
      call $core::fmt::Write::write_char
    )
    (func $<&mut W as core::fmt::Write>::write_fmt (;21;) (type 3) (param i32 i32) (result i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 2
      local.get 0
      i32.load
      i32.store offset=12
      local.get 2
      i32.const 12
      i32.add
      i32.const 1048640
      local.get 1
      call $core::fmt::write
      local.set 0
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $<&mut W as core::fmt::Write>::write_str (;22;) (type 2) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      i32.const 0
      local.set 4
      block ;; label = @1
        local.get 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load
        local.set 5
        block ;; label = @2
          block ;; label = @3
            loop ;; label = @4
              local.get 3
              local.get 2
              i32.store offset=16
              local.get 3
              local.get 1
              i32.store offset=12
              local.get 3
              i32.const 20
              i32.add
              i32.const 2
              local.get 3
              i32.const 12
              i32.add
              i32.const 1
              call $wasi::lib_generated::fd_write
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    local.get 3
                    i32.load16_u offset=20
                    br_if 0 (;@7;)
                    local.get 3
                    i32.load offset=24
                    local.tee 0
                    br_if 1 (;@6;)
                    i32.const 2
                    local.set 2
                    i32.const 1048604
                    local.set 0
                    br 5 (;@2;)
                  end
                  local.get 3
                  local.get 3
                  i32.load16_u offset=22
                  i32.store16 offset=30
                  local.get 3
                  i32.const 30
                  i32.add
                  call $wasi::lib_generated::Errno::raw
                  i32.const 65535
                  i32.and
                  local.tee 0
                  call $std::sys::wasi::decode_error_kind
                  i32.const 255
                  i32.and
                  i32.const 35
                  i32.eq
                  br_if 1 (;@5;)
                  i32.const 0
                  local.set 2
                  br 4 (;@2;)
                end
                local.get 2
                local.get 0
                i32.lt_u
                br_if 2 (;@3;)
                local.get 1
                local.get 0
                i32.add
                local.set 1
                local.get 2
                local.get 0
                i32.sub
                local.set 2
              end
              local.get 2
              br_if 0 (;@4;)
              br 3 (;@1;)
            end
          end
          unreachable
          unreachable
        end
        local.get 5
        i32.load offset=4
        local.set 4
        block ;; label = @2
          block ;; label = @3
            local.get 5
            i32.load8_u
            local.tee 1
            i32.const 4
            i32.gt_u
            br_if 0 (;@3;)
            local.get 1
            i32.const 3
            i32.ne
            br_if 1 (;@2;)
          end
          local.get 4
          i32.load
          local.tee 6
          local.get 4
          i32.const 4
          i32.add
          i32.load
          local.tee 1
          i32.load
          call_indirect (type $.rodata)
          block ;; label = @3
            local.get 1
            i32.load offset=4
            local.tee 7
            i32.eqz
            br_if 0 (;@3;)
            local.get 6
            local.get 7
            local.get 1
            i32.load offset=8
            call $__rust_dealloc
          end
          local.get 4
          i32.const 12
          i32.const 4
          call $__rust_dealloc
        end
        local.get 5
        local.get 0
        i32.store offset=4
        local.get 5
        local.get 2
        i32.store
        i32.const 1
        local.set 4
      end
      local.get 3
      i32.const 32
      i32.add
      global.set $__stack_pointer
      local.get 4
    )
    (func $<std::io::Write::write_fmt::Adapter<T> as core::fmt::Write>::write_str (;23;) (type 2) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      i32.const 0
      local.set 4
      block ;; label = @1
        local.get 2
        i32.eqz
        br_if 0 (;@1;)
        block ;; label = @2
          block ;; label = @3
            loop ;; label = @4
              local.get 3
              local.get 2
              i32.store offset=16
              local.get 3
              local.get 1
              i32.store offset=12
              local.get 3
              i32.const 20
              i32.add
              i32.const 2
              local.get 3
              i32.const 12
              i32.add
              i32.const 1
              call $wasi::lib_generated::fd_write
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    local.get 3
                    i32.load16_u offset=20
                    br_if 0 (;@7;)
                    local.get 3
                    i32.load offset=24
                    local.tee 5
                    br_if 1 (;@6;)
                    i32.const 2
                    local.set 2
                    i32.const 1048604
                    local.set 5
                    br 5 (;@2;)
                  end
                  local.get 3
                  local.get 3
                  i32.load16_u offset=22
                  i32.store16 offset=30
                  local.get 3
                  i32.const 30
                  i32.add
                  call $wasi::lib_generated::Errno::raw
                  i32.const 65535
                  i32.and
                  local.tee 5
                  call $std::sys::wasi::decode_error_kind
                  i32.const 255
                  i32.and
                  i32.const 35
                  i32.eq
                  br_if 1 (;@5;)
                  i32.const 0
                  local.set 2
                  br 4 (;@2;)
                end
                local.get 2
                local.get 5
                i32.lt_u
                br_if 2 (;@3;)
                local.get 1
                local.get 5
                i32.add
                local.set 1
                local.get 2
                local.get 5
                i32.sub
                local.set 2
              end
              local.get 2
              br_if 0 (;@4;)
              br 3 (;@1;)
            end
          end
          unreachable
          unreachable
        end
        local.get 0
        i32.load offset=4
        local.set 4
        block ;; label = @2
          block ;; label = @3
            local.get 0
            i32.load8_u
            local.tee 1
            i32.const 4
            i32.gt_u
            br_if 0 (;@3;)
            local.get 1
            i32.const 3
            i32.ne
            br_if 1 (;@2;)
          end
          local.get 4
          i32.load
          local.tee 6
          local.get 4
          i32.const 4
          i32.add
          i32.load
          local.tee 1
          i32.load
          call_indirect (type $.rodata)
          block ;; label = @3
            local.get 1
            i32.load offset=4
            local.tee 7
            i32.eqz
            br_if 0 (;@3;)
            local.get 6
            local.get 7
            local.get 1
            i32.load offset=8
            call $__rust_dealloc
          end
          local.get 4
          i32.const 12
          i32.const 4
          call $__rust_dealloc
        end
        local.get 0
        local.get 5
        i32.store offset=4
        local.get 0
        local.get 2
        i32.store
        i32.const 1
        local.set 4
      end
      local.get 3
      i32.const 32
      i32.add
      global.set $__stack_pointer
      local.get 4
    )
    (func $__rdl_alloc (;24;) (type 3) (param i32 i32) (result i32)
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
        local.get 0
        call $aligned_alloc
        return
      end
      local.get 0
      call $malloc
    )
    (func $__rdl_dealloc (;25;) (type 9) (param i32 i32 i32)
      local.get 0
      call $free
    )
    (func $__rdl_realloc (;26;) (type 6) (param i32 i32 i32 i32) (result i32)
      block ;; label = @1
        block ;; label = @2
          local.get 2
          i32.const 8
          i32.gt_u
          br_if 0 (;@2;)
          local.get 2
          local.get 3
          i32.le_u
          br_if 1 (;@1;)
        end
        block ;; label = @2
          local.get 2
          local.get 3
          call $aligned_alloc
          local.tee 2
          br_if 0 (;@2;)
          i32.const 0
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
        local.set 3
        local.get 0
        call $free
        local.get 3
        return
      end
      local.get 0
      local.get 3
      call $realloc
    )
    (func $std::alloc::default_alloc_error_hook (;27;) (type 1) (param i32 i32)
      (local i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 64
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        i32.const 0
        i32.load8_u offset=1048952
        br_if 0 (;@1;)
        local.get 2
        i32.const 24
        i32.add
        i64.const 1
        i64.store align=4
        local.get 2
        i32.const 2
        i32.store offset=16
        local.get 2
        i32.const 1048700
        i32.store offset=12
        local.get 2
        i32.const 9
        i32.store offset=40
        local.get 2
        local.get 1
        i32.store offset=44
        local.get 2
        local.get 2
        i32.const 36
        i32.add
        i32.store offset=20
        local.get 2
        local.get 2
        i32.const 44
        i32.add
        i32.store offset=36
        local.get 2
        i32.const 4
        i32.store8 offset=48
        local.get 2
        local.get 2
        i32.const 63
        i32.add
        i32.store offset=56
        local.get 2
        i32.const 48
        i32.add
        i32.const 1048616
        local.get 2
        i32.const 12
        i32.add
        call $core::fmt::write
        local.set 3
        local.get 2
        i32.load8_u offset=48
        local.set 1
        block ;; label = @2
          block ;; label = @3
            local.get 3
            i32.eqz
            br_if 0 (;@3;)
            local.get 1
            i32.const 4
            i32.eq
            br_if 1 (;@2;)
            local.get 2
            i32.load offset=52
            local.set 3
            block ;; label = @4
              local.get 2
              i32.load8_u offset=48
              local.tee 1
              i32.const 4
              i32.gt_u
              br_if 0 (;@4;)
              local.get 1
              i32.const 3
              i32.ne
              br_if 2 (;@2;)
            end
            local.get 3
            i32.load
            local.tee 4
            local.get 3
            i32.const 4
            i32.add
            i32.load
            local.tee 1
            i32.load
            call_indirect (type $.rodata)
            block ;; label = @4
              local.get 1
              i32.load offset=4
              local.tee 5
              i32.eqz
              br_if 0 (;@4;)
              local.get 4
              local.get 5
              local.get 1
              i32.load offset=8
              call $__rust_dealloc
            end
            local.get 3
            i32.const 12
            i32.const 4
            call $__rust_dealloc
            br 1 (;@2;)
          end
          local.get 2
          i32.load offset=52
          local.set 3
          block ;; label = @3
            local.get 1
            i32.const 4
            i32.gt_u
            br_if 0 (;@3;)
            local.get 1
            i32.const 3
            i32.ne
            br_if 1 (;@2;)
          end
          local.get 3
          i32.load
          local.tee 4
          local.get 3
          i32.const 4
          i32.add
          i32.load
          local.tee 1
          i32.load
          call_indirect (type $.rodata)
          block ;; label = @3
            local.get 1
            i32.load offset=4
            local.tee 5
            i32.eqz
            br_if 0 (;@3;)
            local.get 4
            local.get 5
            local.get 1
            i32.load offset=8
            call $__rust_dealloc
          end
          local.get 3
          i32.const 12
          i32.const 4
          call $__rust_dealloc
        end
        local.get 2
        i32.const 64
        i32.add
        global.set $__stack_pointer
        return
      end
      unreachable
      unreachable
    )
    (func $std::alloc::rust_oom (;28;) (type 1) (param i32 i32)
      (local i32)
      local.get 0
      local.get 1
      i32.const 0
      i32.load offset=1048956
      local.tee 2
      i32.const 10
      local.get 2
      select
      call_indirect (type 1)
      call $std::process::abort
      unreachable
    )
    (func $__rg_oom (;29;) (type 1) (param i32 i32)
      local.get 1
      local.get 0
      call $std::alloc::rust_oom
      unreachable
    )
    (func $std::sys::wasi::decode_error_kind (;30;) (type 10) (param i32) (result i32)
      (local i32)
      i32.const 40
      local.set 1
      block ;; label = @1
        local.get 0
        i32.const 65535
        i32.gt_u
        br_if 0 (;@1;)
        i32.const 2
        local.set 1
        i32.const 1048716
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 3
        local.set 1
        i32.const 1048718
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 1
        local.set 1
        i32.const 1048720
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 1048722
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 11
        local.set 1
        i32.const 1048724
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 7
        local.set 1
        i32.const 1048726
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 6
        local.set 1
        i32.const 1048728
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 9
        local.set 1
        i32.const 1048730
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 8
        local.set 1
        i32.const 1048732
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 0
        local.set 1
        i32.const 1048734
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 35
        local.set 1
        i32.const 1048736
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 20
        local.set 1
        i32.const 1048738
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 22
        local.set 1
        i32.const 1048740
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 12
        local.set 1
        i32.const 1048742
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 13
        local.set 1
        i32.const 1048744
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 36
        local.set 1
        i32.const 1048746
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 38
        i32.const 40
        i32.const 1048748
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        select
        local.set 1
      end
      local.get 1
    )
    (func $std::sys::wasi::abort_internal (;31;) (type 7)
      call $abort
      unreachable
    )
    (func $std::process::abort (;32;) (type 7)
      call $std::sys::wasi::abort_internal
      unreachable
    )
    (func $wasi::lib_generated::Errno::raw (;33;) (type 10) (param i32) (result i32)
      local.get 0
      i32.load16_u
    )
    (func $wasi::lib_generated::fd_write (;34;) (type 8) (param i32 i32 i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          local.get 1
          local.get 2
          local.get 3
          local.get 4
          i32.const 12
          i32.add
          call $wasi::lib_generated::wasi_snapshot_preview1::fd_write
          local.tee 3
          br_if 0 (;@2;)
          local.get 0
          local.get 4
          i32.load offset=12
          i32.store offset=4
          i32.const 0
          local.set 3
          br 1 (;@1;)
        end
        local.get 0
        local.get 3
        i32.store16 offset=2
        i32.const 1
        local.set 3
      end
      local.get 0
      local.get 3
      i32.store16
      local.get 4
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $malloc (;35;) (type 10) (param i32) (result i32)
      local.get 0
      call $dlmalloc
    )
    (func $dlmalloc (;36;) (type 10) (param i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      block ;; label = @1
        i32.const 0
        i32.load offset=1048984
        local.tee 2
        br_if 0 (;@1;)
        block ;; label = @2
          block ;; label = @3
            i32.const 0
            i32.load offset=1049432
            local.tee 3
            i32.eqz
            br_if 0 (;@3;)
            i32.const 0
            i32.load offset=1049436
            local.set 4
            br 1 (;@2;)
          end
          i32.const 0
          i64.const -1
          i64.store offset=1049444 align=4
          i32.const 0
          i64.const 281474976776192
          i64.store offset=1049436 align=4
          i32.const 0
          local.get 1
          i32.const 8
          i32.add
          i32.const -16
          i32.and
          i32.const 1431655768
          i32.xor
          local.tee 3
          i32.store offset=1049432
          i32.const 0
          i32.const 0
          i32.store offset=1049452
          i32.const 0
          i32.const 0
          i32.store offset=1049404
          i32.const 65536
          local.set 4
        end
        i32.const 0
        local.set 2
        i32.const 1114112
        i32.const 1049472
        local.get 4
        i32.add
        i32.const -1
        i32.add
        i32.const 0
        local.get 4
        i32.sub
        i32.and
        i32.const 1114112
        select
        i32.const 1049472
        i32.sub
        local.tee 5
        i32.const 89
        i32.lt_u
        br_if 0 (;@1;)
        i32.const 0
        local.set 4
        i32.const 0
        local.get 5
        i32.store offset=1049412
        i32.const 0
        i32.const 1049472
        i32.store offset=1049408
        i32.const 0
        i32.const 1049472
        i32.store offset=1048976
        i32.const 0
        local.get 3
        i32.store offset=1048996
        i32.const 0
        i32.const -1
        i32.store offset=1048992
        loop ;; label = @2
          local.get 4
          i32.const 1049020
          i32.add
          local.get 4
          i32.const 1049008
          i32.add
          local.tee 3
          i32.store
          local.get 3
          local.get 4
          i32.const 1049000
          i32.add
          local.tee 6
          i32.store
          local.get 4
          i32.const 1049012
          i32.add
          local.get 6
          i32.store
          local.get 4
          i32.const 1049028
          i32.add
          local.get 4
          i32.const 1049016
          i32.add
          local.tee 6
          i32.store
          local.get 6
          local.get 3
          i32.store
          local.get 4
          i32.const 1049036
          i32.add
          local.get 4
          i32.const 1049024
          i32.add
          local.tee 3
          i32.store
          local.get 3
          local.get 6
          i32.store
          local.get 4
          i32.const 1049032
          i32.add
          local.get 3
          i32.store
          local.get 4
          i32.const 32
          i32.add
          local.tee 4
          i32.const 256
          i32.ne
          br_if 0 (;@2;)
        end
        i32.const 1049472
        i32.const -8
        i32.const 1049472
        i32.sub
        i32.const 15
        i32.and
        i32.const 0
        i32.const 1049472
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
        local.get 5
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
        i32.load offset=1049448
        i32.store offset=1048988
        i32.const 0
        local.get 4
        i32.store offset=1048972
        i32.const 0
        local.get 2
        i32.store offset=1048984
        i32.const 1049472
        local.get 3
        i32.add
        i32.const 56
        i32.store offset=4
      end
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
                              local.get 0
                              i32.const 236
                              i32.gt_u
                              br_if 0 (;@12;)
                              block ;; label = @13
                                i32.const 0
                                i32.load offset=1048960
                                local.tee 7
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
                                local.tee 5
                                i32.const 3
                                i32.shr_u
                                local.tee 3
                                i32.shr_u
                                local.tee 4
                                i32.const 3
                                i32.and
                                i32.eqz
                                br_if 0 (;@13;)
                                block ;; label = @14
                                  block ;; label = @15
                                    local.get 4
                                    i32.const 1
                                    i32.and
                                    local.get 3
                                    i32.or
                                    i32.const 1
                                    i32.xor
                                    local.tee 6
                                    i32.const 3
                                    i32.shl
                                    local.tee 3
                                    i32.const 1049000
                                    i32.add
                                    local.tee 4
                                    local.get 3
                                    i32.const 1049008
                                    i32.add
                                    i32.load
                                    local.tee 3
                                    i32.load offset=8
                                    local.tee 5
                                    i32.ne
                                    br_if 0 (;@15;)
                                    i32.const 0
                                    local.get 7
                                    i32.const -2
                                    local.get 6
                                    i32.rotl
                                    i32.and
                                    i32.store offset=1048960
                                    br 1 (;@14;)
                                  end
                                  local.get 4
                                  local.get 5
                                  i32.store offset=8
                                  local.get 5
                                  local.get 4
                                  i32.store offset=12
                                end
                                local.get 3
                                i32.const 8
                                i32.add
                                local.set 4
                                local.get 3
                                local.get 6
                                i32.const 3
                                i32.shl
                                local.tee 6
                                i32.const 3
                                i32.or
                                i32.store offset=4
                                local.get 3
                                local.get 6
                                i32.add
                                local.tee 3
                                local.get 3
                                i32.load offset=4
                                i32.const 1
                                i32.or
                                i32.store offset=4
                                br 12 (;@1;)
                              end
                              local.get 5
                              i32.const 0
                              i32.load offset=1048968
                              local.tee 8
                              i32.le_u
                              br_if 1 (;@11;)
                              block ;; label = @13
                                local.get 4
                                i32.eqz
                                br_if 0 (;@13;)
                                block ;; label = @14
                                  block ;; label = @15
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
                                    i32.const 1049000
                                    i32.add
                                    local.tee 6
                                    local.get 4
                                    i32.const 1049008
                                    i32.add
                                    i32.load
                                    local.tee 4
                                    i32.load offset=8
                                    local.tee 0
                                    i32.ne
                                    br_if 0 (;@15;)
                                    i32.const 0
                                    local.get 7
                                    i32.const -2
                                    local.get 3
                                    i32.rotl
                                    i32.and
                                    local.tee 7
                                    i32.store offset=1048960
                                    br 1 (;@14;)
                                  end
                                  local.get 6
                                  local.get 0
                                  i32.store offset=8
                                  local.get 0
                                  local.get 6
                                  i32.store offset=12
                                end
                                local.get 4
                                local.get 5
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
                                local.get 5
                                i32.sub
                                local.tee 6
                                i32.store
                                local.get 4
                                local.get 5
                                i32.add
                                local.tee 0
                                local.get 6
                                i32.const 1
                                i32.or
                                i32.store offset=4
                                block ;; label = @14
                                  local.get 8
                                  i32.eqz
                                  br_if 0 (;@14;)
                                  local.get 8
                                  i32.const -8
                                  i32.and
                                  i32.const 1049000
                                  i32.add
                                  local.set 5
                                  i32.const 0
                                  i32.load offset=1048980
                                  local.set 3
                                  block ;; label = @15
                                    block ;; label = @16
                                      local.get 7
                                      i32.const 1
                                      local.get 8
                                      i32.const 3
                                      i32.shr_u
                                      i32.shl
                                      local.tee 9
                                      i32.and
                                      br_if 0 (;@16;)
                                      i32.const 0
                                      local.get 7
                                      local.get 9
                                      i32.or
                                      i32.store offset=1048960
                                      local.get 5
                                      local.set 9
                                      br 1 (;@15;)
                                    end
                                    local.get 5
                                    i32.load offset=8
                                    local.set 9
                                  end
                                  local.get 9
                                  local.get 3
                                  i32.store offset=12
                                  local.get 5
                                  local.get 3
                                  i32.store offset=8
                                  local.get 3
                                  local.get 5
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
                                i32.store offset=1048980
                                i32.const 0
                                local.get 6
                                i32.store offset=1048968
                                br 12 (;@1;)
                              end
                              i32.const 0
                              i32.load offset=1048964
                              local.tee 10
                              i32.eqz
                              br_if 1 (;@11;)
                              local.get 10
                              i32.const 0
                              local.get 10
                              i32.sub
                              i32.and
                              i32.ctz
                              i32.const 2
                              i32.shl
                              i32.const 1049264
                              i32.add
                              i32.load
                              local.tee 0
                              i32.load offset=4
                              i32.const -8
                              i32.and
                              local.get 5
                              i32.sub
                              local.set 3
                              local.get 0
                              local.set 6
                              block ;; label = @13
                                loop ;; label = @14
                                  block ;; label = @15
                                    local.get 6
                                    i32.load offset=16
                                    local.tee 4
                                    br_if 0 (;@15;)
                                    local.get 6
                                    i32.const 20
                                    i32.add
                                    i32.load
                                    local.tee 4
                                    i32.eqz
                                    br_if 2 (;@13;)
                                  end
                                  local.get 4
                                  i32.load offset=4
                                  i32.const -8
                                  i32.and
                                  local.get 5
                                  i32.sub
                                  local.tee 6
                                  local.get 3
                                  local.get 6
                                  local.get 3
                                  i32.lt_u
                                  local.tee 6
                                  select
                                  local.set 3
                                  local.get 4
                                  local.get 0
                                  local.get 6
                                  select
                                  local.set 0
                                  local.get 4
                                  local.set 6
                                  br 0 (;@14;)
                                end
                              end
                              local.get 0
                              i32.load offset=24
                              local.set 11
                              block ;; label = @13
                                local.get 0
                                i32.load offset=12
                                local.tee 9
                                local.get 0
                                i32.eq
                                br_if 0 (;@13;)
                                local.get 0
                                i32.load offset=8
                                local.tee 4
                                i32.const 0
                                i32.load offset=1048976
                                i32.lt_u
                                drop
                                local.get 9
                                local.get 4
                                i32.store offset=8
                                local.get 4
                                local.get 9
                                i32.store offset=12
                                br 11 (;@2;)
                              end
                              block ;; label = @13
                                local.get 0
                                i32.const 20
                                i32.add
                                local.tee 6
                                i32.load
                                local.tee 4
                                br_if 0 (;@13;)
                                local.get 0
                                i32.load offset=16
                                local.tee 4
                                i32.eqz
                                br_if 3 (;@10;)
                                local.get 0
                                i32.const 16
                                i32.add
                                local.set 6
                              end
                              loop ;; label = @13
                                local.get 6
                                local.set 2
                                local.get 4
                                local.tee 9
                                i32.const 20
                                i32.add
                                local.tee 6
                                i32.load
                                local.tee 4
                                br_if 0 (;@13;)
                                local.get 9
                                i32.const 16
                                i32.add
                                local.set 6
                                local.get 9
                                i32.load offset=16
                                local.tee 4
                                br_if 0 (;@13;)
                              end
                              local.get 2
                              i32.const 0
                              i32.store
                              br 10 (;@2;)
                            end
                            i32.const -1
                            local.set 5
                            local.get 0
                            i32.const -65
                            i32.gt_u
                            br_if 0 (;@11;)
                            local.get 0
                            i32.const 19
                            i32.add
                            local.tee 4
                            i32.const -16
                            i32.and
                            local.set 5
                            i32.const 0
                            i32.load offset=1048964
                            local.tee 10
                            i32.eqz
                            br_if 0 (;@11;)
                            i32.const 0
                            local.set 8
                            block ;; label = @12
                              local.get 5
                              i32.const 256
                              i32.lt_u
                              br_if 0 (;@12;)
                              i32.const 31
                              local.set 8
                              local.get 5
                              i32.const 16777215
                              i32.gt_u
                              br_if 0 (;@12;)
                              local.get 5
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
                            local.get 5
                            i32.sub
                            local.set 3
                            block ;; label = @12
                              block ;; label = @13
                                block ;; label = @14
                                  block ;; label = @15
                                    local.get 8
                                    i32.const 2
                                    i32.shl
                                    i32.const 1049264
                                    i32.add
                                    i32.load
                                    local.tee 6
                                    br_if 0 (;@15;)
                                    i32.const 0
                                    local.set 4
                                    i32.const 0
                                    local.set 9
                                    br 1 (;@14;)
                                  end
                                  i32.const 0
                                  local.set 4
                                  local.get 5
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
                                  loop ;; label = @15
                                    block ;; label = @16
                                      local.get 6
                                      i32.load offset=4
                                      i32.const -8
                                      i32.and
                                      local.get 5
                                      i32.sub
                                      local.tee 7
                                      local.get 3
                                      i32.ge_u
                                      br_if 0 (;@16;)
                                      local.get 7
                                      local.set 3
                                      local.get 6
                                      local.set 9
                                      local.get 7
                                      br_if 0 (;@16;)
                                      i32.const 0
                                      local.set 3
                                      local.get 6
                                      local.set 9
                                      local.get 6
                                      local.set 4
                                      br 3 (;@13;)
                                    end
                                    local.get 4
                                    local.get 6
                                    i32.const 20
                                    i32.add
                                    i32.load
                                    local.tee 7
                                    local.get 7
                                    local.get 6
                                    local.get 0
                                    i32.const 29
                                    i32.shr_u
                                    i32.const 4
                                    i32.and
                                    i32.add
                                    i32.const 16
                                    i32.add
                                    i32.load
                                    local.tee 6
                                    i32.eq
                                    select
                                    local.get 4
                                    local.get 7
                                    select
                                    local.set 4
                                    local.get 0
                                    i32.const 1
                                    i32.shl
                                    local.set 0
                                    local.get 6
                                    br_if 0 (;@15;)
                                  end
                                end
                                block ;; label = @14
                                  local.get 4
                                  local.get 9
                                  i32.or
                                  br_if 0 (;@14;)
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
                                  br_if 3 (;@11;)
                                  local.get 4
                                  i32.const 0
                                  local.get 4
                                  i32.sub
                                  i32.and
                                  i32.ctz
                                  i32.const 2
                                  i32.shl
                                  i32.const 1049264
                                  i32.add
                                  i32.load
                                  local.set 4
                                end
                                local.get 4
                                i32.eqz
                                br_if 1 (;@12;)
                              end
                              loop ;; label = @13
                                local.get 4
                                i32.load offset=4
                                i32.const -8
                                i32.and
                                local.get 5
                                i32.sub
                                local.tee 7
                                local.get 3
                                i32.lt_u
                                local.set 0
                                block ;; label = @14
                                  local.get 4
                                  i32.load offset=16
                                  local.tee 6
                                  br_if 0 (;@14;)
                                  local.get 4
                                  i32.const 20
                                  i32.add
                                  i32.load
                                  local.set 6
                                end
                                local.get 7
                                local.get 3
                                local.get 0
                                select
                                local.set 3
                                local.get 4
                                local.get 9
                                local.get 0
                                select
                                local.set 9
                                local.get 6
                                local.set 4
                                local.get 6
                                br_if 0 (;@13;)
                              end
                            end
                            local.get 9
                            i32.eqz
                            br_if 0 (;@11;)
                            local.get 3
                            i32.const 0
                            i32.load offset=1048968
                            local.get 5
                            i32.sub
                            i32.ge_u
                            br_if 0 (;@11;)
                            local.get 9
                            i32.load offset=24
                            local.set 2
                            block ;; label = @12
                              local.get 9
                              i32.load offset=12
                              local.tee 0
                              local.get 9
                              i32.eq
                              br_if 0 (;@12;)
                              local.get 9
                              i32.load offset=8
                              local.tee 4
                              i32.const 0
                              i32.load offset=1048976
                              i32.lt_u
                              drop
                              local.get 0
                              local.get 4
                              i32.store offset=8
                              local.get 4
                              local.get 0
                              i32.store offset=12
                              br 9 (;@3;)
                            end
                            block ;; label = @12
                              local.get 9
                              i32.const 20
                              i32.add
                              local.tee 6
                              i32.load
                              local.tee 4
                              br_if 0 (;@12;)
                              local.get 9
                              i32.load offset=16
                              local.tee 4
                              i32.eqz
                              br_if 3 (;@9;)
                              local.get 9
                              i32.const 16
                              i32.add
                              local.set 6
                            end
                            loop ;; label = @12
                              local.get 6
                              local.set 7
                              local.get 4
                              local.tee 0
                              i32.const 20
                              i32.add
                              local.tee 6
                              i32.load
                              local.tee 4
                              br_if 0 (;@12;)
                              local.get 0
                              i32.const 16
                              i32.add
                              local.set 6
                              local.get 0
                              i32.load offset=16
                              local.tee 4
                              br_if 0 (;@12;)
                            end
                            local.get 7
                            i32.const 0
                            i32.store
                            br 8 (;@3;)
                          end
                          block ;; label = @11
                            i32.const 0
                            i32.load offset=1048968
                            local.tee 4
                            local.get 5
                            i32.lt_u
                            br_if 0 (;@11;)
                            i32.const 0
                            i32.load offset=1048980
                            local.set 3
                            block ;; label = @12
                              block ;; label = @13
                                local.get 4
                                local.get 5
                                i32.sub
                                local.tee 6
                                i32.const 16
                                i32.lt_u
                                br_if 0 (;@13;)
                                local.get 3
                                local.get 5
                                i32.add
                                local.tee 0
                                local.get 6
                                i32.const 1
                                i32.or
                                i32.store offset=4
                                local.get 3
                                local.get 4
                                i32.add
                                local.get 6
                                i32.store
                                local.get 3
                                local.get 5
                                i32.const 3
                                i32.or
                                i32.store offset=4
                                br 1 (;@12;)
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
                              local.set 6
                            end
                            i32.const 0
                            local.get 6
                            i32.store offset=1048968
                            i32.const 0
                            local.get 0
                            i32.store offset=1048980
                            local.get 3
                            i32.const 8
                            i32.add
                            local.set 4
                            br 10 (;@1;)
                          end
                          block ;; label = @11
                            i32.const 0
                            i32.load offset=1048972
                            local.tee 6
                            local.get 5
                            i32.le_u
                            br_if 0 (;@11;)
                            local.get 2
                            local.get 5
                            i32.add
                            local.tee 4
                            local.get 6
                            local.get 5
                            i32.sub
                            local.tee 3
                            i32.const 1
                            i32.or
                            i32.store offset=4
                            i32.const 0
                            local.get 4
                            i32.store offset=1048984
                            i32.const 0
                            local.get 3
                            i32.store offset=1048972
                            local.get 2
                            local.get 5
                            i32.const 3
                            i32.or
                            i32.store offset=4
                            local.get 2
                            i32.const 8
                            i32.add
                            local.set 4
                            br 10 (;@1;)
                          end
                          block ;; label = @11
                            block ;; label = @12
                              i32.const 0
                              i32.load offset=1049432
                              i32.eqz
                              br_if 0 (;@12;)
                              i32.const 0
                              i32.load offset=1049440
                              local.set 3
                              br 1 (;@11;)
                            end
                            i32.const 0
                            i64.const -1
                            i64.store offset=1049444 align=4
                            i32.const 0
                            i64.const 281474976776192
                            i64.store offset=1049436 align=4
                            i32.const 0
                            local.get 1
                            i32.const 12
                            i32.add
                            i32.const -16
                            i32.and
                            i32.const 1431655768
                            i32.xor
                            i32.store offset=1049432
                            i32.const 0
                            i32.const 0
                            i32.store offset=1049452
                            i32.const 0
                            i32.const 0
                            i32.store offset=1049404
                            i32.const 65536
                            local.set 3
                          end
                          i32.const 0
                          local.set 4
                          block ;; label = @11
                            local.get 3
                            local.get 5
                            i32.const 71
                            i32.add
                            local.tee 8
                            i32.add
                            local.tee 0
                            i32.const 0
                            local.get 3
                            i32.sub
                            local.tee 7
                            i32.and
                            local.tee 9
                            local.get 5
                            i32.gt_u
                            br_if 0 (;@11;)
                            i32.const 0
                            i32.const 48
                            i32.store offset=1049456
                            br 10 (;@1;)
                          end
                          block ;; label = @11
                            i32.const 0
                            i32.load offset=1049400
                            local.tee 4
                            i32.eqz
                            br_if 0 (;@11;)
                            block ;; label = @12
                              i32.const 0
                              i32.load offset=1049392
                              local.tee 3
                              local.get 9
                              i32.add
                              local.tee 10
                              local.get 3
                              i32.le_u
                              br_if 0 (;@12;)
                              local.get 10
                              local.get 4
                              i32.le_u
                              br_if 1 (;@11;)
                            end
                            i32.const 0
                            local.set 4
                            i32.const 0
                            i32.const 48
                            i32.store offset=1049456
                            br 10 (;@1;)
                          end
                          i32.const 0
                          i32.load8_u offset=1049404
                          i32.const 4
                          i32.and
                          br_if 4 (;@6;)
                          block ;; label = @11
                            block ;; label = @12
                              block ;; label = @13
                                local.get 2
                                i32.eqz
                                br_if 0 (;@13;)
                                i32.const 1049408
                                local.set 4
                                loop ;; label = @14
                                  block ;; label = @15
                                    local.get 4
                                    i32.load
                                    local.tee 3
                                    local.get 2
                                    i32.gt_u
                                    br_if 0 (;@15;)
                                    local.get 3
                                    local.get 4
                                    i32.load offset=4
                                    i32.add
                                    local.get 2
                                    i32.gt_u
                                    br_if 3 (;@12;)
                                  end
                                  local.get 4
                                  i32.load offset=8
                                  local.tee 4
                                  br_if 0 (;@14;)
                                end
                              end
                              i32.const 0
                              call $sbrk
                              local.tee 0
                              i32.const -1
                              i32.eq
                              br_if 5 (;@7;)
                              local.get 9
                              local.set 7
                              block ;; label = @13
                                i32.const 0
                                i32.load offset=1049436
                                local.tee 4
                                i32.const -1
                                i32.add
                                local.tee 3
                                local.get 0
                                i32.and
                                i32.eqz
                                br_if 0 (;@13;)
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
                                local.set 7
                              end
                              local.get 7
                              local.get 5
                              i32.le_u
                              br_if 5 (;@7;)
                              local.get 7
                              i32.const 2147483646
                              i32.gt_u
                              br_if 5 (;@7;)
                              block ;; label = @13
                                i32.const 0
                                i32.load offset=1049400
                                local.tee 4
                                i32.eqz
                                br_if 0 (;@13;)
                                i32.const 0
                                i32.load offset=1049392
                                local.tee 3
                                local.get 7
                                i32.add
                                local.tee 6
                                local.get 3
                                i32.le_u
                                br_if 6 (;@7;)
                                local.get 6
                                local.get 4
                                i32.gt_u
                                br_if 6 (;@7;)
                              end
                              local.get 7
                              call $sbrk
                              local.tee 4
                              local.get 0
                              i32.ne
                              br_if 1 (;@11;)
                              br 7 (;@5;)
                            end
                            local.get 0
                            local.get 6
                            i32.sub
                            local.get 7
                            i32.and
                            local.tee 7
                            i32.const 2147483646
                            i32.gt_u
                            br_if 4 (;@7;)
                            local.get 7
                            call $sbrk
                            local.tee 0
                            local.get 4
                            i32.load
                            local.get 4
                            i32.load offset=4
                            i32.add
                            i32.eq
                            br_if 3 (;@8;)
                            local.get 0
                            local.set 4
                          end
                          block ;; label = @11
                            local.get 4
                            i32.const -1
                            i32.eq
                            br_if 0 (;@11;)
                            local.get 5
                            i32.const 72
                            i32.add
                            local.get 7
                            i32.le_u
                            br_if 0 (;@11;)
                            block ;; label = @12
                              local.get 8
                              local.get 7
                              i32.sub
                              i32.const 0
                              i32.load offset=1049440
                              local.tee 3
                              i32.add
                              i32.const 0
                              local.get 3
                              i32.sub
                              i32.and
                              local.tee 3
                              i32.const 2147483646
                              i32.le_u
                              br_if 0 (;@12;)
                              local.get 4
                              local.set 0
                              br 7 (;@5;)
                            end
                            block ;; label = @12
                              local.get 3
                              call $sbrk
                              i32.const -1
                              i32.eq
                              br_if 0 (;@12;)
                              local.get 3
                              local.get 7
                              i32.add
                              local.set 7
                              local.get 4
                              local.set 0
                              br 7 (;@5;)
                            end
                            i32.const 0
                            local.get 7
                            i32.sub
                            call $sbrk
                            drop
                            br 4 (;@7;)
                          end
                          local.get 4
                          local.set 0
                          local.get 4
                          i32.const -1
                          i32.ne
                          br_if 5 (;@5;)
                          br 3 (;@7;)
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
                  i32.load offset=1049404
                  i32.const 4
                  i32.or
                  i32.store offset=1049404
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
                local.tee 7
                local.get 5
                i32.const 56
                i32.add
                i32.le_u
                br_if 1 (;@4;)
              end
              i32.const 0
              i32.const 0
              i32.load offset=1049392
              local.get 7
              i32.add
              local.tee 4
              i32.store offset=1049392
              block ;; label = @5
                local.get 4
                i32.const 0
                i32.load offset=1049396
                i32.le_u
                br_if 0 (;@5;)
                i32.const 0
                local.get 4
                i32.store offset=1049396
              end
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    block ;; label = @8
                      i32.const 0
                      i32.load offset=1048984
                      local.tee 3
                      i32.eqz
                      br_if 0 (;@8;)
                      i32.const 1049408
                      local.set 4
                      loop ;; label = @9
                        local.get 0
                        local.get 4
                        i32.load
                        local.tee 6
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
                        i32.load offset=1048976
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
                      i32.store offset=1048976
                    end
                    i32.const 0
                    local.set 4
                    i32.const 0
                    local.get 7
                    i32.store offset=1049412
                    i32.const 0
                    local.get 0
                    i32.store offset=1049408
                    i32.const 0
                    i32.const -1
                    i32.store offset=1048992
                    i32.const 0
                    i32.const 0
                    i32.load offset=1049432
                    i32.store offset=1048996
                    i32.const 0
                    i32.const 0
                    i32.store offset=1049420
                    loop ;; label = @8
                      local.get 4
                      i32.const 1049020
                      i32.add
                      local.get 4
                      i32.const 1049008
                      i32.add
                      local.tee 3
                      i32.store
                      local.get 3
                      local.get 4
                      i32.const 1049000
                      i32.add
                      local.tee 6
                      i32.store
                      local.get 4
                      i32.const 1049012
                      i32.add
                      local.get 6
                      i32.store
                      local.get 4
                      i32.const 1049028
                      i32.add
                      local.get 4
                      i32.const 1049016
                      i32.add
                      local.tee 6
                      i32.store
                      local.get 6
                      local.get 3
                      i32.store
                      local.get 4
                      i32.const 1049036
                      i32.add
                      local.get 4
                      i32.const 1049024
                      i32.add
                      local.tee 3
                      i32.store
                      local.get 3
                      local.get 6
                      i32.store
                      local.get 4
                      i32.const 1049032
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
                    local.get 7
                    i32.const -56
                    i32.add
                    local.tee 6
                    local.get 4
                    i32.sub
                    local.tee 4
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    i32.const 0
                    i32.const 0
                    i32.load offset=1049448
                    i32.store offset=1048988
                    i32.const 0
                    local.get 4
                    i32.store offset=1048972
                    i32.const 0
                    local.get 3
                    i32.store offset=1048984
                    local.get 0
                    local.get 6
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
                  local.get 6
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
                  local.tee 6
                  i32.add
                  local.tee 0
                  i32.const 0
                  i32.load offset=1048972
                  local.get 7
                  i32.add
                  local.tee 2
                  local.get 6
                  i32.sub
                  local.tee 6
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 4
                  local.get 9
                  local.get 7
                  i32.add
                  i32.store offset=4
                  i32.const 0
                  i32.const 0
                  i32.load offset=1049448
                  i32.store offset=1048988
                  i32.const 0
                  local.get 6
                  i32.store offset=1048972
                  i32.const 0
                  local.get 0
                  i32.store offset=1048984
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
                  i32.load offset=1048976
                  local.tee 9
                  i32.ge_u
                  br_if 0 (;@6;)
                  i32.const 0
                  local.get 0
                  i32.store offset=1048976
                  local.get 0
                  local.set 9
                end
                local.get 0
                local.get 7
                i32.add
                local.set 6
                i32.const 1049408
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
                                local.get 6
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
                          i32.const 1049408
                          local.set 4
                          loop ;; label = @11
                            block ;; label = @12
                              local.get 4
                              i32.load
                              local.tee 6
                              local.get 3
                              i32.gt_u
                              br_if 0 (;@12;)
                              local.get 6
                              local.get 4
                              i32.load offset=4
                              i32.add
                              local.tee 6
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
                        local.get 7
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
                        local.get 5
                        i32.const 3
                        i32.or
                        i32.store offset=4
                        local.get 6
                        i32.const -8
                        local.get 6
                        i32.sub
                        i32.const 15
                        i32.and
                        i32.const 0
                        local.get 6
                        i32.const 8
                        i32.add
                        i32.const 15
                        i32.and
                        select
                        i32.add
                        local.tee 7
                        local.get 2
                        local.get 5
                        i32.add
                        local.tee 5
                        i32.sub
                        local.set 4
                        block ;; label = @10
                          local.get 7
                          local.get 3
                          i32.ne
                          br_if 0 (;@10;)
                          i32.const 0
                          local.get 5
                          i32.store offset=1048984
                          i32.const 0
                          i32.const 0
                          i32.load offset=1048972
                          local.get 4
                          i32.add
                          local.tee 4
                          i32.store offset=1048972
                          local.get 5
                          local.get 4
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          br 3 (;@7;)
                        end
                        block ;; label = @10
                          local.get 7
                          i32.const 0
                          i32.load offset=1048980
                          i32.ne
                          br_if 0 (;@10;)
                          i32.const 0
                          local.get 5
                          i32.store offset=1048980
                          i32.const 0
                          i32.const 0
                          i32.load offset=1048968
                          local.get 4
                          i32.add
                          local.tee 4
                          i32.store offset=1048968
                          local.get 5
                          local.get 4
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          local.get 5
                          local.get 4
                          i32.add
                          local.get 4
                          i32.store
                          br 3 (;@7;)
                        end
                        block ;; label = @10
                          local.get 7
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
                              local.get 7
                              i32.load offset=8
                              local.tee 6
                              local.get 3
                              i32.const 3
                              i32.shr_u
                              local.tee 9
                              i32.const 3
                              i32.shl
                              i32.const 1049000
                              i32.add
                              local.tee 0
                              i32.eq
                              drop
                              block ;; label = @13
                                local.get 7
                                i32.load offset=12
                                local.tee 3
                                local.get 6
                                i32.ne
                                br_if 0 (;@13;)
                                i32.const 0
                                i32.const 0
                                i32.load offset=1048960
                                i32.const -2
                                local.get 9
                                i32.rotl
                                i32.and
                                i32.store offset=1048960
                                br 2 (;@11;)
                              end
                              local.get 3
                              local.get 0
                              i32.eq
                              drop
                              local.get 3
                              local.get 6
                              i32.store offset=8
                              local.get 6
                              local.get 3
                              i32.store offset=12
                              br 1 (;@11;)
                            end
                            local.get 7
                            i32.load offset=24
                            local.set 10
                            block ;; label = @12
                              block ;; label = @13
                                local.get 7
                                i32.load offset=12
                                local.tee 0
                                local.get 7
                                i32.eq
                                br_if 0 (;@13;)
                                local.get 7
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
                                local.get 7
                                i32.const 20
                                i32.add
                                local.tee 3
                                i32.load
                                local.tee 6
                                br_if 0 (;@13;)
                                local.get 7
                                i32.const 16
                                i32.add
                                local.tee 3
                                i32.load
                                local.tee 6
                                br_if 0 (;@13;)
                                i32.const 0
                                local.set 0
                                br 1 (;@12;)
                              end
                              loop ;; label = @13
                                local.get 3
                                local.set 9
                                local.get 6
                                local.tee 0
                                i32.const 20
                                i32.add
                                local.tee 3
                                i32.load
                                local.tee 6
                                br_if 0 (;@13;)
                                local.get 0
                                i32.const 16
                                i32.add
                                local.set 3
                                local.get 0
                                i32.load offset=16
                                local.tee 6
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
                                local.get 7
                                local.get 7
                                i32.load offset=28
                                local.tee 6
                                i32.const 2
                                i32.shl
                                i32.const 1049264
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
                                i32.load offset=1048964
                                i32.const -2
                                local.get 6
                                i32.rotl
                                i32.and
                                i32.store offset=1048964
                                br 2 (;@11;)
                              end
                              local.get 10
                              i32.const 16
                              i32.const 20
                              local.get 10
                              i32.load offset=16
                              local.get 7
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
                              local.get 7
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
                            local.get 7
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
                        local.get 5
                        local.get 4
                        i32.add
                        local.get 4
                        i32.store
                        local.get 5
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
                          i32.const 1049000
                          i32.add
                          local.set 3
                          block ;; label = @11
                            block ;; label = @12
                              i32.const 0
                              i32.load offset=1048960
                              local.tee 6
                              i32.const 1
                              local.get 4
                              i32.const 3
                              i32.shr_u
                              i32.shl
                              local.tee 4
                              i32.and
                              br_if 0 (;@12;)
                              i32.const 0
                              local.get 6
                              local.get 4
                              i32.or
                              i32.store offset=1048960
                              local.get 3
                              local.set 4
                              br 1 (;@11;)
                            end
                            local.get 3
                            i32.load offset=8
                            local.set 4
                          end
                          local.get 4
                          local.get 5
                          i32.store offset=12
                          local.get 3
                          local.get 5
                          i32.store offset=8
                          local.get 5
                          local.get 3
                          i32.store offset=12
                          local.get 5
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
                        local.get 5
                        local.get 3
                        i32.store offset=28
                        local.get 5
                        i64.const 0
                        i64.store offset=16 align=4
                        local.get 3
                        i32.const 2
                        i32.shl
                        i32.const 1049264
                        i32.add
                        local.set 6
                        block ;; label = @10
                          i32.const 0
                          i32.load offset=1048964
                          local.tee 0
                          i32.const 1
                          local.get 3
                          i32.shl
                          local.tee 9
                          i32.and
                          br_if 0 (;@10;)
                          local.get 6
                          local.get 5
                          i32.store
                          i32.const 0
                          local.get 0
                          local.get 9
                          i32.or
                          i32.store offset=1048964
                          local.get 5
                          local.get 6
                          i32.store offset=24
                          local.get 5
                          local.get 5
                          i32.store offset=8
                          local.get 5
                          local.get 5
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
                        local.get 6
                        i32.load
                        local.set 0
                        loop ;; label = @10
                          local.get 0
                          local.tee 6
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
                          local.get 6
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
                        local.get 5
                        i32.store
                        local.get 5
                        local.get 6
                        i32.store offset=24
                        local.get 5
                        local.get 5
                        i32.store offset=12
                        local.get 5
                        local.get 5
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
                      local.get 7
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
                      local.get 6
                      i32.const 55
                      local.get 6
                      i32.sub
                      i32.const 15
                      i32.and
                      i32.const 0
                      local.get 6
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
                      i32.load offset=1049448
                      i32.store offset=1048988
                      i32.const 0
                      local.get 4
                      i32.store offset=1048972
                      i32.const 0
                      local.get 2
                      i32.store offset=1048984
                      local.get 9
                      i32.const 16
                      i32.add
                      i32.const 0
                      i64.load offset=1049416 align=4
                      i64.store align=4
                      local.get 9
                      i32.const 0
                      i64.load offset=1049408 align=4
                      i64.store offset=8 align=4
                      i32.const 0
                      local.get 9
                      i32.const 8
                      i32.add
                      i32.store offset=1049416
                      i32.const 0
                      local.get 7
                      i32.store offset=1049412
                      i32.const 0
                      local.get 0
                      i32.store offset=1049408
                      i32.const 0
                      i32.const 0
                      i32.store offset=1049420
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
                        local.get 6
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
                        i32.const 1049000
                        i32.add
                        local.set 4
                        block ;; label = @10
                          block ;; label = @11
                            i32.const 0
                            i32.load offset=1048960
                            local.tee 6
                            i32.const 1
                            local.get 0
                            i32.const 3
                            i32.shr_u
                            i32.shl
                            local.tee 0
                            i32.and
                            br_if 0 (;@11;)
                            i32.const 0
                            local.get 6
                            local.get 0
                            i32.or
                            i32.store offset=1048960
                            local.get 4
                            local.set 6
                            br 1 (;@10;)
                          end
                          local.get 4
                          i32.load offset=8
                          local.set 6
                        end
                        local.get 6
                        local.get 3
                        i32.store offset=12
                        local.get 4
                        local.get 3
                        i32.store offset=8
                        local.get 3
                        local.get 4
                        i32.store offset=12
                        local.get 3
                        local.get 6
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
                      i32.const 1049264
                      i32.add
                      local.set 6
                      block ;; label = @9
                        i32.const 0
                        i32.load offset=1048964
                        local.tee 9
                        i32.const 1
                        local.get 4
                        i32.shl
                        local.tee 7
                        i32.and
                        br_if 0 (;@9;)
                        local.get 6
                        local.get 3
                        i32.store
                        i32.const 0
                        local.get 9
                        local.get 7
                        i32.or
                        i32.store offset=1048964
                        local.get 3
                        local.get 6
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
                      local.get 6
                      i32.load
                      local.set 9
                      loop ;; label = @9
                        local.get 9
                        local.tee 6
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
                        local.get 6
                        local.get 9
                        i32.const 4
                        i32.and
                        i32.add
                        i32.const 16
                        i32.add
                        local.tee 7
                        i32.load
                        local.tee 9
                        br_if 0 (;@9;)
                      end
                      local.get 7
                      local.get 3
                      i32.store
                      local.get 3
                      local.get 6
                      i32.store offset=24
                      local.get 3
                      local.get 3
                      i32.store offset=12
                      local.get 3
                      local.get 3
                      i32.store offset=8
                      br 3 (;@5;)
                    end
                    local.get 6
                    i32.load offset=8
                    local.tee 4
                    local.get 5
                    i32.store offset=12
                    local.get 6
                    local.get 5
                    i32.store offset=8
                    local.get 5
                    i32.const 0
                    i32.store offset=24
                    local.get 5
                    local.get 6
                    i32.store offset=12
                    local.get 5
                    local.get 4
                    i32.store offset=8
                  end
                  local.get 2
                  i32.const 8
                  i32.add
                  local.set 4
                  br 5 (;@1;)
                end
                local.get 6
                i32.load offset=8
                local.tee 4
                local.get 3
                i32.store offset=12
                local.get 6
                local.get 3
                i32.store offset=8
                local.get 3
                i32.const 0
                i32.store offset=24
                local.get 3
                local.get 6
                i32.store offset=12
                local.get 3
                local.get 4
                i32.store offset=8
              end
              i32.const 0
              i32.load offset=1048972
              local.tee 4
              local.get 5
              i32.le_u
              br_if 0 (;@4;)
              i32.const 0
              i32.load offset=1048984
              local.tee 3
              local.get 5
              i32.add
              local.tee 6
              local.get 4
              local.get 5
              i32.sub
              local.tee 4
              i32.const 1
              i32.or
              i32.store offset=4
              i32.const 0
              local.get 4
              i32.store offset=1048972
              i32.const 0
              local.get 6
              i32.store offset=1048984
              local.get 3
              local.get 5
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
            i32.store offset=1049456
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
                local.tee 6
                i32.const 2
                i32.shl
                i32.const 1049264
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
                local.get 6
                i32.rotl
                i32.and
                local.tee 10
                i32.store offset=1048964
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
              local.get 5
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
            local.get 5
            i32.add
            local.tee 0
            local.get 3
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 9
            local.get 5
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
              i32.const 1049000
              i32.add
              local.set 4
              block ;; label = @5
                block ;; label = @6
                  i32.const 0
                  i32.load offset=1048960
                  local.tee 6
                  i32.const 1
                  local.get 3
                  i32.const 3
                  i32.shr_u
                  i32.shl
                  local.tee 3
                  i32.and
                  br_if 0 (;@6;)
                  i32.const 0
                  local.get 6
                  local.get 3
                  i32.or
                  i32.store offset=1048960
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
            i32.const 1049264
            i32.add
            local.set 6
            block ;; label = @4
              local.get 10
              i32.const 1
              local.get 4
              i32.shl
              local.tee 5
              i32.and
              br_if 0 (;@4;)
              local.get 6
              local.get 0
              i32.store
              i32.const 0
              local.get 10
              local.get 5
              i32.or
              i32.store offset=1048964
              local.get 0
              local.get 6
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
            local.get 6
            i32.load
            local.set 5
            block ;; label = @4
              loop ;; label = @5
                local.get 5
                local.tee 6
                i32.load offset=4
                i32.const -8
                i32.and
                local.get 3
                i32.eq
                br_if 1 (;@4;)
                local.get 4
                i32.const 29
                i32.shr_u
                local.set 5
                local.get 4
                i32.const 1
                i32.shl
                local.set 4
                local.get 6
                local.get 5
                i32.const 4
                i32.and
                i32.add
                i32.const 16
                i32.add
                local.tee 7
                i32.load
                local.tee 5
                br_if 0 (;@5;)
              end
              local.get 7
              local.get 0
              i32.store
              local.get 0
              local.get 6
              i32.store offset=24
              local.get 0
              local.get 0
              i32.store offset=12
              local.get 0
              local.get 0
              i32.store offset=8
              br 1 (;@3;)
            end
            local.get 6
            i32.load offset=8
            local.tee 4
            local.get 0
            i32.store offset=12
            local.get 6
            local.get 0
            i32.store offset=8
            local.get 0
            i32.const 0
            i32.store offset=24
            local.get 0
            local.get 6
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
              local.tee 6
              i32.const 2
              i32.shl
              i32.const 1049264
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
              local.get 6
              i32.rotl
              i32.and
              i32.store offset=1048964
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
            local.get 5
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
          local.get 5
          i32.add
          local.tee 6
          local.get 3
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 0
          local.get 5
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 6
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
            i32.const 1049000
            i32.add
            local.set 5
            i32.const 0
            i32.load offset=1048980
            local.set 4
            block ;; label = @4
              block ;; label = @5
                i32.const 1
                local.get 8
                i32.const 3
                i32.shr_u
                i32.shl
                local.tee 9
                local.get 7
                i32.and
                br_if 0 (;@5;)
                i32.const 0
                local.get 9
                local.get 7
                i32.or
                i32.store offset=1048960
                local.get 5
                local.set 9
                br 1 (;@4;)
              end
              local.get 5
              i32.load offset=8
              local.set 9
            end
            local.get 9
            local.get 4
            i32.store offset=12
            local.get 5
            local.get 4
            i32.store offset=8
            local.get 4
            local.get 5
            i32.store offset=12
            local.get 4
            local.get 9
            i32.store offset=8
          end
          i32.const 0
          local.get 6
          i32.store offset=1048980
          i32.const 0
          local.get 3
          i32.store offset=1048968
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
    (func $free (;37;) (type $.rodata) (param i32)
      local.get 0
      call $dlfree
    )
    (func $dlfree (;38;) (type $.rodata) (param i32)
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
          i32.load offset=1048976
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
            i32.load offset=1048980
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
              i32.const 1049000
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
                i32.load offset=1048960
                i32.const -2
                local.get 5
                i32.rotl
                i32.and
                i32.store offset=1048960
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
                i32.const 1049264
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
                i32.load offset=1048964
                i32.const -2
                local.get 4
                i32.rotl
                i32.and
                i32.store offset=1048964
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
          i32.store offset=1048968
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
              i32.load offset=1048984
              i32.ne
              br_if 0 (;@4;)
              i32.const 0
              local.get 1
              i32.store offset=1048984
              i32.const 0
              i32.const 0
              i32.load offset=1048972
              local.get 0
              i32.add
              local.tee 0
              i32.store offset=1048972
              local.get 1
              local.get 0
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 1
              i32.const 0
              i32.load offset=1048980
              i32.ne
              br_if 3 (;@1;)
              i32.const 0
              i32.const 0
              i32.store offset=1048968
              i32.const 0
              i32.const 0
              i32.store offset=1048980
              return
            end
            block ;; label = @4
              local.get 3
              i32.const 0
              i32.load offset=1048980
              i32.ne
              br_if 0 (;@4;)
              i32.const 0
              local.get 1
              i32.store offset=1048980
              i32.const 0
              i32.const 0
              i32.load offset=1048968
              local.get 0
              i32.add
              local.tee 0
              i32.store offset=1048968
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
                i32.const 1049000
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
                  i32.load offset=1048960
                  i32.const -2
                  local.get 5
                  i32.rotl
                  i32.and
                  i32.store offset=1048960
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
                  i32.load offset=1048976
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
                  i32.const 1049264
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
                  i32.load offset=1048964
                  i32.const -2
                  local.get 4
                  i32.rotl
                  i32.and
                  i32.store offset=1048964
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
            i32.load offset=1048980
            i32.ne
            br_if 1 (;@2;)
            i32.const 0
            local.get 0
            i32.store offset=1048968
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
          i32.const 1049000
          i32.add
          local.set 2
          block ;; label = @3
            block ;; label = @4
              i32.const 0
              i32.load offset=1048960
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
              i32.store offset=1048960
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
        i32.const 1049264
        i32.add
        local.set 4
        block ;; label = @2
          block ;; label = @3
            i32.const 0
            i32.load offset=1048964
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
            i32.store offset=1048964
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
        i32.load offset=1048992
        i32.const -1
        i32.add
        local.tee 1
        i32.const -1
        local.get 1
        select
        i32.store offset=1048992
      end
    )
    (func $realloc (;39;) (type 3) (param i32 i32) (result i32)
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
        i32.store offset=1049456
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
            i32.load offset=1049440
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
            i32.load offset=1048984
            i32.ne
            br_if 0 (;@3;)
            i32.const 0
            i32.load offset=1048972
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
            i32.store offset=1048984
            i32.const 0
            local.get 5
            local.get 2
            i32.sub
            local.tee 2
            i32.store offset=1048972
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
            i32.load offset=1048980
            i32.ne
            br_if 0 (;@3;)
            i32.const 0
            i32.load offset=1048968
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
            i32.store offset=1048980
            i32.const 0
            local.get 1
            i32.store offset=1048968
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
              i32.const 1049000
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
                i32.load offset=1048960
                i32.const -2
                local.get 11
                i32.rotl
                i32.and
                i32.store offset=1048960
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
                i32.load offset=1048976
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
                i32.const 1049264
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
                i32.load offset=1048964
                i32.const -2
                local.get 5
                i32.rotl
                i32.and
                i32.store offset=1048964
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
    (func $dispose_chunk (;40;) (type 1) (param i32 i32)
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
              i32.load offset=1048980
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
                i32.const 1049000
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
                i32.load offset=1048960
                i32.const -2
                local.get 5
                i32.rotl
                i32.and
                i32.store offset=1048960
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
                  i32.load offset=1048976
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
                  i32.const 1049264
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
                  i32.load offset=1048964
                  i32.const -2
                  local.get 4
                  i32.rotl
                  i32.and
                  i32.store offset=1048964
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
            i32.store offset=1048968
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
              i32.load offset=1048984
              i32.ne
              br_if 0 (;@4;)
              i32.const 0
              local.get 0
              i32.store offset=1048984
              i32.const 0
              i32.const 0
              i32.load offset=1048972
              local.get 1
              i32.add
              local.tee 1
              i32.store offset=1048972
              local.get 0
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              i32.const 0
              i32.load offset=1048980
              i32.ne
              br_if 3 (;@1;)
              i32.const 0
              i32.const 0
              i32.store offset=1048968
              i32.const 0
              i32.const 0
              i32.store offset=1048980
              return
            end
            block ;; label = @4
              local.get 2
              i32.const 0
              i32.load offset=1048980
              i32.ne
              br_if 0 (;@4;)
              i32.const 0
              local.get 0
              i32.store offset=1048980
              i32.const 0
              i32.const 0
              i32.load offset=1048968
              local.get 1
              i32.add
              local.tee 1
              i32.store offset=1048968
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
                i32.const 1049000
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
                  i32.load offset=1048960
                  i32.const -2
                  local.get 5
                  i32.rotl
                  i32.and
                  i32.store offset=1048960
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
                  i32.load offset=1048976
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
                  i32.const 1049264
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
                  i32.load offset=1048964
                  i32.const -2
                  local.get 4
                  i32.rotl
                  i32.and
                  i32.store offset=1048964
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
            i32.load offset=1048980
            i32.ne
            br_if 1 (;@2;)
            i32.const 0
            local.get 1
            i32.store offset=1048968
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
          i32.const 1049000
          i32.add
          local.set 3
          block ;; label = @3
            block ;; label = @4
              i32.const 0
              i32.load offset=1048960
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
              i32.store offset=1048960
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
        i32.const 1049264
        i32.add
        local.set 4
        block ;; label = @2
          i32.const 0
          i32.load offset=1048964
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
          i32.store offset=1048964
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
    (func $internal_memalign (;41;) (type 3) (param i32 i32) (result i32)
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
        i32.store offset=1049456
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
    (func $aligned_alloc (;42;) (type 3) (param i32 i32) (result i32)
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
    (func $abort (;43;) (type 7)
      unreachable
      unreachable
    )
    (func $sbrk (;44;) (type 10) (param i32) (result i32)
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
          i32.store offset=1049456
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
    (func $memcpy (;45;) (type 2) (param i32 i32 i32) (result i32)
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
    (func $alloc::raw_vec::capacity_overflow (;46;) (type 7)
      unreachable
      unreachable
    )
    (func $alloc::alloc::handle_alloc_error (;47;) (type 1) (param i32 i32)
      local.get 0
      local.get 1
      call $alloc::alloc::handle_alloc_error::rt_error
      unreachable
    )
    (func $alloc::alloc::handle_alloc_error::rt_error (;48;) (type 1) (param i32 i32)
      local.get 1
      local.get 0
      call $__rust_alloc_error_handler
      unreachable
    )
    (func $core::fmt::write (;49;) (type 2) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      i32.const 36
      i32.add
      local.get 1
      i32.store
      local.get 3
      i32.const 3
      i32.store8 offset=44
      local.get 3
      i32.const 32
      i32.store offset=28
      i32.const 0
      local.set 4
      local.get 3
      i32.const 0
      i32.store offset=40
      local.get 3
      local.get 0
      i32.store offset=32
      local.get 3
      i32.const 0
      i32.store offset=20
      local.get 3
      i32.const 0
      i32.store offset=12
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 2
              i32.load offset=16
              local.tee 5
              br_if 0 (;@4;)
              local.get 2
              i32.const 12
              i32.add
              i32.load
              local.tee 0
              i32.eqz
              br_if 1 (;@3;)
              local.get 2
              i32.load offset=8
              local.set 1
              local.get 0
              i32.const 3
              i32.shl
              local.set 6
              local.get 0
              i32.const -1
              i32.add
              i32.const 536870911
              i32.and
              i32.const 1
              i32.add
              local.set 4
              local.get 2
              i32.load
              local.set 0
              loop ;; label = @5
                block ;; label = @6
                  local.get 0
                  i32.const 4
                  i32.add
                  i32.load
                  local.tee 7
                  i32.eqz
                  br_if 0 (;@6;)
                  local.get 3
                  i32.load offset=32
                  local.get 0
                  i32.load
                  local.get 7
                  local.get 3
                  i32.load offset=36
                  i32.load offset=12
                  call_indirect (type 2)
                  br_if 4 (;@2;)
                end
                local.get 1
                i32.load
                local.get 3
                i32.const 12
                i32.add
                local.get 1
                i32.const 4
                i32.add
                i32.load
                call_indirect (type 3)
                br_if 3 (;@2;)
                local.get 1
                i32.const 8
                i32.add
                local.set 1
                local.get 0
                i32.const 8
                i32.add
                local.set 0
                local.get 6
                i32.const -8
                i32.add
                local.tee 6
                br_if 0 (;@5;)
                br 2 (;@3;)
              end
            end
            local.get 2
            i32.const 20
            i32.add
            i32.load
            local.tee 1
            i32.eqz
            br_if 0 (;@3;)
            local.get 1
            i32.const 5
            i32.shl
            local.set 8
            local.get 1
            i32.const -1
            i32.add
            i32.const 134217727
            i32.and
            i32.const 1
            i32.add
            local.set 4
            local.get 2
            i32.load offset=8
            local.set 9
            local.get 2
            i32.load
            local.set 0
            i32.const 0
            local.set 6
            loop ;; label = @4
              block ;; label = @5
                local.get 0
                i32.const 4
                i32.add
                i32.load
                local.tee 1
                i32.eqz
                br_if 0 (;@5;)
                local.get 3
                i32.load offset=32
                local.get 0
                i32.load
                local.get 1
                local.get 3
                i32.load offset=36
                i32.load offset=12
                call_indirect (type 2)
                br_if 3 (;@2;)
              end
              local.get 3
              local.get 5
              local.get 6
              i32.add
              local.tee 1
              i32.const 16
              i32.add
              i32.load
              i32.store offset=28
              local.get 3
              local.get 1
              i32.const 28
              i32.add
              i32.load8_u
              i32.store8 offset=44
              local.get 3
              local.get 1
              i32.const 24
              i32.add
              i32.load
              i32.store offset=40
              local.get 1
              i32.const 12
              i32.add
              i32.load
              local.set 10
              i32.const 0
              local.set 11
              i32.const 0
              local.set 7
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    local.get 1
                    i32.const 8
                    i32.add
                    i32.load
                    br_table 1 (;@6;) 0 (;@7;) 2 (;@5;) 1 (;@6;)
                  end
                  local.get 10
                  i32.const 3
                  i32.shl
                  local.set 12
                  i32.const 0
                  local.set 7
                  local.get 9
                  local.get 12
                  i32.add
                  local.tee 12
                  i32.load offset=4
                  i32.const 11
                  i32.ne
                  br_if 1 (;@5;)
                  local.get 12
                  i32.load
                  i32.load
                  local.set 10
                end
                i32.const 1
                local.set 7
              end
              local.get 3
              local.get 10
              i32.store offset=16
              local.get 3
              local.get 7
              i32.store offset=12
              local.get 1
              i32.const 4
              i32.add
              i32.load
              local.set 7
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    local.get 1
                    i32.load
                    br_table 1 (;@6;) 0 (;@7;) 2 (;@5;) 1 (;@6;)
                  end
                  local.get 7
                  i32.const 3
                  i32.shl
                  local.set 10
                  local.get 9
                  local.get 10
                  i32.add
                  local.tee 10
                  i32.load offset=4
                  i32.const 11
                  i32.ne
                  br_if 1 (;@5;)
                  local.get 10
                  i32.load
                  i32.load
                  local.set 7
                end
                i32.const 1
                local.set 11
              end
              local.get 3
              local.get 7
              i32.store offset=24
              local.get 3
              local.get 11
              i32.store offset=20
              local.get 9
              local.get 1
              i32.const 20
              i32.add
              i32.load
              i32.const 3
              i32.shl
              i32.add
              local.tee 1
              i32.load
              local.get 3
              i32.const 12
              i32.add
              local.get 1
              i32.load offset=4
              call_indirect (type 3)
              br_if 2 (;@2;)
              local.get 0
              i32.const 8
              i32.add
              local.set 0
              local.get 8
              local.get 6
              i32.const 32
              i32.add
              local.tee 6
              i32.ne
              br_if 0 (;@4;)
            end
          end
          block ;; label = @3
            local.get 4
            local.get 2
            i32.load offset=4
            i32.ge_u
            br_if 0 (;@3;)
            local.get 3
            i32.load offset=32
            local.get 2
            i32.load
            local.get 4
            i32.const 3
            i32.shl
            i32.add
            local.tee 1
            i32.load
            local.get 1
            i32.load offset=4
            local.get 3
            i32.load offset=36
            i32.load offset=12
            call_indirect (type 2)
            br_if 1 (;@2;)
          end
          i32.const 0
          local.set 1
          br 1 (;@1;)
        end
        i32.const 1
        local.set 1
      end
      local.get 3
      i32.const 48
      i32.add
      global.set $__stack_pointer
      local.get 1
    )
    (func $core::fmt::Formatter::pad_integral (;50;) (type 11) (param i32 i32 i32 i32 i32 i32) (result i32)
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
          call_indirect (type 2)
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
          call_indirect (type 2)
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
              call_indirect (type 3)
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
          call_indirect (type 2)
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
            call_indirect (type 3)
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
        call_indirect (type 2)
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
          call_indirect (type 3)
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
    (func $core::fmt::Formatter::pad_integral::write_prefix (;51;) (type 12) (param i32 i32 i32 i32 i32) (result i32)
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
            call_indirect (type 3)
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
      call_indirect (type 2)
    )
    (func $core::fmt::num::imp::fmt_u64 (;52;) (type 13) (param i64 i32 i32) (result i32)
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
          i32.const 1048750
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
          i32.const 1048750
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
        i32.const 1048750
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
          i32.const 1048750
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
      i32.const 1048952
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
    (func $core::fmt::num::imp::<impl core::fmt::Display for u32>::fmt (;53;) (type 3) (param i32 i32) (result i32)
      local.get 0
      i64.load32_u
      i32.const 1
      local.get 1
      call $core::fmt::num::imp::fmt_u64
    )
    (func $core::str::count::do_count_chars (;54;) (type 3) (param i32 i32) (result i32)
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
    (func $core::ops::function::FnOnce::call_once (;55;) (type 3) (param i32 i32) (result i32)
      local.get 0
      i32.load
      drop
      loop (result i32) ;; label = @1
        br 0 (;@1;)
      end
    )
    (table (;0;) 12 12 funcref)
    (memory (;0;) 17)
    (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
    (export "memory" (memory 0))
    (export "miden:base/note@1.0.0#note-script" (func $miden:base/note@1.0.0#note-script))
    (export "cabi_realloc" (func $cabi_realloc))
    (elem (;0;) (i32.const 1) func $core::ptr::drop_in_place<std::io::Write::write_fmt::Adapter<std::sys::wasi::stdio::Stderr>> $<std::io::Write::write_fmt::Adapter<T> as core::fmt::Write>::write_str $core::fmt::Write::write_char $core::fmt::Write::write_fmt $core::ptr::drop_in_place<&mut std::io::Write::write_fmt::Adapter<alloc::vec::Vec<u8>>> $<&mut W as core::fmt::Write>::write_str $<&mut W as core::fmt::Write>::write_char $<&mut W as core::fmt::Write>::write_fmt $core::fmt::num::imp::<impl core::fmt::Display for u32>::fmt $std::alloc::default_alloc_error_hook $core::ops::function::FnOnce::call_once)
    (data (;0;) (i32.const 1048576) "failed to write whole buffer\00\00\10\00\1c\00\00\00\17\00\00\00\01\00\00\00\0c\00\00\00\04\00\00\00\02\00\00\00\03\00\00\00\04\00\00\00\05\00\00\00\04\00\00\00\04\00\00\00\06\00\00\00\07\00\00\00\08\00\00\00memory allocation of  bytes failed\0a\00X\00\10\00\15\00\00\00m\00\10\00\0e\00\00\00\0e\00\0f\00?\00\02\00@\005\00\0d\00\04\00\03\00,\00\1b\00\1c\00I\00\14\00\06\004\000\0000010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899\00\00")
  )
  (core module (;1;)
    (type (;0;) (func (param i32)))
    (type (;1;) (func (param i32 i32)))
    (type (;2;) (func (param i32 i64 i32)))
    (type (;3;) (func (param i32 i32 i32 i32)))
    (type (;4;) (func (param i32) (result i32)))
    (type (;5;) (func (param i32 i32 i32)))
    (type (;6;) (func (param i32 i32 i32 i32) (result i32)))
    (type (;7;) (func (param i32 i32 i32 i32 i32)))
    (type (;8;) (func (result i32)))
    (type (;9;) (func (param i32 i32 i32) (result i32)))
    (type (;10;) (func (param i32 i32) (result i32)))
    (type (;11;) (func))
    (import "env" "memory" (memory (;0;) 0))
    (import "wasi:filesystem/preopens@0.2.0-rc-2023-11-10" "get-directories" (func $wasi_snapshot_preview1::descriptors::Descriptors::open_preopens::get_preopens_import (;0;) (type 0)))
    (import "wasi:filesystem/types@0.2.0-rc-2023-11-10" "[method]descriptor.get-type" (func $wasi_snapshot_preview1::bindings::wasi::filesystem::types::Descriptor::get_type::wit_import (;1;) (type 1)))
    (import "wasi:filesystem/types@0.2.0-rc-2023-11-10" "filesystem-error-code" (func $wasi_snapshot_preview1::bindings::wasi::filesystem::types::filesystem_error_code::wit_import (;2;) (type 1)))
    (import "wasi:io/error@0.2.0-rc-2023-11-10" "[resource-drop]error" (func $<wasi_snapshot_preview1::bindings::wasi::io::error::Error as wit_bindgen::WasmResource>::drop::drop (;3;) (type 0)))
    (import "wasi:io/streams@0.2.0-rc-2023-11-10" "[resource-drop]input-stream" (func $<wasi_snapshot_preview1::bindings::wasi::io::streams::InputStream as wit_bindgen::WasmResource>::drop::drop (;4;) (type 0)))
    (import "wasi:io/streams@0.2.0-rc-2023-11-10" "[resource-drop]output-stream" (func $<wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream as wit_bindgen::WasmResource>::drop::drop (;5;) (type 0)))
    (import "wasi:filesystem/types@0.2.0-rc-2023-11-10" "[resource-drop]descriptor" (func $<wasi_snapshot_preview1::bindings::wasi::filesystem::types::Descriptor as wit_bindgen::WasmResource>::drop::drop (;6;) (type 0)))
    (import "__main_module__" "cabi_realloc" (func $wasi_snapshot_preview1::State::new::cabi_realloc (;7;) (type 6)))
    (import "wasi:filesystem/types@0.2.0-rc-2023-11-10" "[method]descriptor.write-via-stream" (func $wasi_snapshot_preview1::bindings::wasi::filesystem::types::Descriptor::write_via_stream::wit_import (;8;) (type 2)))
    (import "wasi:filesystem/types@0.2.0-rc-2023-11-10" "[method]descriptor.append-via-stream" (func $wasi_snapshot_preview1::bindings::wasi::filesystem::types::Descriptor::append_via_stream::wit_import (;9;) (type 1)))
    (import "wasi:filesystem/types@0.2.0-rc-2023-11-10" "[method]descriptor.stat" (func $wasi_snapshot_preview1::bindings::wasi::filesystem::types::Descriptor::stat::wit_import (;10;) (type 1)))
    (import "wasi:cli/stderr@0.2.0-rc-2023-12-05" "get-stderr" (func $wasi_snapshot_preview1::bindings::wasi::cli::stderr::get_stderr::wit_import (;11;) (type 8)))
    (import "wasi:cli/stdin@0.2.0-rc-2023-12-05" "get-stdin" (func $wasi_snapshot_preview1::bindings::wasi::cli::stdin::get_stdin::wit_import (;12;) (type 8)))
    (import "wasi:cli/stdout@0.2.0-rc-2023-12-05" "get-stdout" (func $wasi_snapshot_preview1::bindings::wasi::cli::stdout::get_stdout::wit_import (;13;) (type 8)))
    (import "wasi:io/streams@0.2.0-rc-2023-11-10" "[method]output-stream.check-write" (func $wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream::check_write::wit_import (;14;) (type 1)))
    (import "wasi:io/streams@0.2.0-rc-2023-11-10" "[method]output-stream.write" (func $wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream::write::wit_import (;15;) (type 3)))
    (import "wasi:io/streams@0.2.0-rc-2023-11-10" "[method]output-stream.blocking-write-and-flush" (func $wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream::blocking_write_and_flush::wit_import (;16;) (type 3)))
    (import "wasi:io/streams@0.2.0-rc-2023-11-10" "[method]output-stream.blocking-flush" (func $wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream::blocking_flush::wit_import (;17;) (type 1)))
    (func $wasi_snapshot_preview1::State::ptr (;18;) (type 8) (result i32)
      (local i32)
      block ;; label = @1
        call $get_state_ptr
        local.tee 0
        br_if 0 (;@1;)
        call $wasi_snapshot_preview1::State::new
        local.tee 0
        call $set_state_ptr
      end
      local.get 0
    )
    (func $cabi_import_realloc (;19;) (type 6) (param i32 i32 i32 i32) (result i32)
      (local i32)
      call $allocate_stack
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                local.get 0
                br_if 0 (;@5;)
                local.get 1
                br_if 0 (;@5;)
                call $wasi_snapshot_preview1::State::ptr
                local.tee 0
                i32.load
                i32.const 560490357
                i32.ne
                br_if 1 (;@4;)
                local.get 0
                i32.load offset=65532
                i32.const 560490357
                i32.ne
                br_if 2 (;@3;)
                block ;; label = @6
                  block ;; label = @7
                    local.get 0
                    i32.const 12
                    i32.add
                    i32.load
                    local.tee 1
                    i32.eqz
                    br_if 0 (;@7;)
                    local.get 1
                    local.get 2
                    local.get 3
                    call $wasi_snapshot_preview1::BumpArena::alloc
                    local.set 2
                    br 1 (;@6;)
                  end
                  local.get 0
                  i32.load offset=4
                  local.tee 1
                  i32.eqz
                  br_if 4 (;@2;)
                  local.get 2
                  local.get 1
                  i32.add
                  i32.const -1
                  i32.add
                  i32.const 0
                  local.get 2
                  i32.sub
                  i32.and
                  local.tee 2
                  local.get 3
                  i32.add
                  local.tee 3
                  local.get 2
                  i32.ge_u
                  local.get 3
                  call $<core::option::Option<T> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap
                  local.get 1
                  local.get 0
                  i32.const 8
                  i32.add
                  i32.load
                  i32.add
                  local.tee 3
                  local.get 1
                  i32.ge_u
                  local.get 3
                  call $<core::option::Option<T> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap
                  i32.gt_u
                  br_if 5 (;@1;)
                  local.get 0
                  i32.const 0
                  i32.store offset=4
                end
                local.get 4
                i32.const 48
                i32.add
                global.set $__stack_pointer
                local.get 2
                return
              end
              local.get 4
              i32.const 32
              i32.store8 offset=47
              local.get 4
              i32.const 1701734764
              i32.store offset=43 align=1
              local.get 4
              i64.const 2338042707334751329
              i64.store offset=35 align=1
              local.get 4
              i64.const 2338600898263348341
              i64.store offset=27 align=1
              local.get 4
              i64.const 7162263158133189730
              i64.store offset=19 align=1
              local.get 4
              i64.const 7018969289221893749
              i64.store offset=11 align=1
              local.get 4
              i32.const 11
              i32.add
              i32.const 37
              call $wasi_snapshot_preview1::macros::print
              i32.const 184
              call $wasi_snapshot_preview1::macros::eprint_u32
              local.get 4
              i32.const 10
              i32.store8 offset=11
              local.get 4
              i32.const 11
              i32.add
              i32.const 1
              call $wasi_snapshot_preview1::macros::print
              unreachable
              unreachable
            end
            local.get 4
            i32.const 32
            i32.store8 offset=47
            local.get 4
            i32.const 1701734764
            i32.store offset=43 align=1
            local.get 4
            i64.const 2338042707334751329
            i64.store offset=35 align=1
            local.get 4
            i64.const 2338600898263348341
            i64.store offset=27 align=1
            local.get 4
            i64.const 7162263158133189730
            i64.store offset=19 align=1
            local.get 4
            i64.const 7018969289221893749
            i64.store offset=11 align=1
            local.get 4
            i32.const 11
            i32.add
            i32.const 37
            call $wasi_snapshot_preview1::macros::print
            i32.const 2552
            call $wasi_snapshot_preview1::macros::eprint_u32
            local.get 4
            i32.const 8250
            i32.store16 offset=11 align=1
            local.get 4
            i32.const 11
            i32.add
            i32.const 2
            call $wasi_snapshot_preview1::macros::print
            local.get 4
            i32.const 10
            i32.store8 offset=27
            local.get 4
            i64.const 7234307576302018670
            i64.store offset=19 align=1
            local.get 4
            i64.const 8028075845441778529
            i64.store offset=11 align=1
            local.get 4
            i32.const 11
            i32.add
            i32.const 17
            call $wasi_snapshot_preview1::macros::print
            local.get 4
            i32.const 10
            i32.store8 offset=11
            local.get 4
            i32.const 11
            i32.add
            i32.const 1
            call $wasi_snapshot_preview1::macros::print
            unreachable
            unreachable
          end
          local.get 4
          i32.const 32
          i32.store8 offset=47
          local.get 4
          i32.const 1701734764
          i32.store offset=43 align=1
          local.get 4
          i64.const 2338042707334751329
          i64.store offset=35 align=1
          local.get 4
          i64.const 2338600898263348341
          i64.store offset=27 align=1
          local.get 4
          i64.const 7162263158133189730
          i64.store offset=19 align=1
          local.get 4
          i64.const 7018969289221893749
          i64.store offset=11 align=1
          local.get 4
          i32.const 11
          i32.add
          i32.const 37
          call $wasi_snapshot_preview1::macros::print
          i32.const 2553
          call $wasi_snapshot_preview1::macros::eprint_u32
          local.get 4
          i32.const 8250
          i32.store16 offset=11 align=1
          local.get 4
          i32.const 11
          i32.add
          i32.const 2
          call $wasi_snapshot_preview1::macros::print
          local.get 4
          i32.const 10
          i32.store8 offset=27
          local.get 4
          i64.const 7234307576302018670
          i64.store offset=19 align=1
          local.get 4
          i64.const 8028075845441778529
          i64.store offset=11 align=1
          local.get 4
          i32.const 11
          i32.add
          i32.const 17
          call $wasi_snapshot_preview1::macros::print
          local.get 4
          i32.const 10
          i32.store8 offset=11
          local.get 4
          i32.const 11
          i32.add
          i32.const 1
          call $wasi_snapshot_preview1::macros::print
          unreachable
          unreachable
        end
        local.get 4
        i32.const 32
        i32.store8 offset=47
        local.get 4
        i32.const 1701734764
        i32.store offset=43 align=1
        local.get 4
        i64.const 2338042707334751329
        i64.store offset=35 align=1
        local.get 4
        i64.const 2338600898263348341
        i64.store offset=27 align=1
        local.get 4
        i64.const 7162263158133189730
        i64.store offset=19 align=1
        local.get 4
        i64.const 7018969289221893749
        i64.store offset=11 align=1
        local.get 4
        i32.const 11
        i32.add
        i32.const 37
        call $wasi_snapshot_preview1::macros::print
        i32.const 290
        call $wasi_snapshot_preview1::macros::eprint_u32
        local.get 4
        i32.const 8250
        i32.store16 offset=11 align=1
        local.get 4
        i32.const 11
        i32.add
        i32.const 2
        call $wasi_snapshot_preview1::macros::print
        local.get 4
        i32.const 10
        i32.store8 offset=47
        local.get 4
        i32.const 1684370293
        i32.store offset=43 align=1
        local.get 4
        i64.const 2340011850872286305
        i64.store offset=35 align=1
        local.get 4
        i64.const 2338053340533122404
        i64.store offset=27 align=1
        local.get 4
        i64.const 7599383958532420719
        i64.store offset=19 align=1
        local.get 4
        i64.const 7935468323262068066
        i64.store offset=11 align=1
        local.get 4
        i32.const 11
        i32.add
        i32.const 37
        call $wasi_snapshot_preview1::macros::print
        local.get 4
        i32.const 10
        i32.store8 offset=11
        local.get 4
        i32.const 11
        i32.add
        i32.const 1
        call $wasi_snapshot_preview1::macros::print
        unreachable
        unreachable
      end
      local.get 4
      i32.const 32
      i32.store8 offset=47
      local.get 4
      i32.const 1701734764
      i32.store offset=43 align=1
      local.get 4
      i64.const 2338042707334751329
      i64.store offset=35 align=1
      local.get 4
      i64.const 2338600898263348341
      i64.store offset=27 align=1
      local.get 4
      i64.const 7162263158133189730
      i64.store offset=19 align=1
      local.get 4
      i64.const 7018969289221893749
      i64.store offset=11 align=1
      local.get 4
      i32.const 11
      i32.add
      i32.const 37
      call $wasi_snapshot_preview1::macros::print
      i32.const 297
      call $wasi_snapshot_preview1::macros::eprint_u32
      local.get 4
      i32.const 8250
      i32.store16 offset=11 align=1
      local.get 4
      i32.const 11
      i32.add
      i32.const 2
      call $wasi_snapshot_preview1::macros::print
      local.get 4
      i32.const 2681
      i32.store16 offset=23 align=1
      local.get 4
      i32.const 1919905125
      i32.store offset=19 align=1
      local.get 4
      i64.const 7863397576860792175
      i64.store offset=11 align=1
      local.get 4
      i32.const 11
      i32.add
      i32.const 14
      call $wasi_snapshot_preview1::macros::print
      local.get 4
      i32.const 10
      i32.store8 offset=11
      local.get 4
      i32.const 11
      i32.add
      i32.const 1
      call $wasi_snapshot_preview1::macros::print
      unreachable
      unreachable
    )
    (func $wasi_snapshot_preview1::BumpArena::alloc (;20;) (type 9) (param i32 i32 i32) (result i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      block ;; label = @1
        local.get 0
        local.get 1
        i32.add
        local.get 0
        i32.load offset=54912
        i32.add
        i32.const -1
        i32.add
        i32.const 0
        local.get 1
        i32.sub
        i32.and
        local.tee 1
        local.get 0
        i32.sub
        local.get 2
        i32.add
        local.tee 2
        i32.const 54912
        i32.gt_u
        br_if 0 (;@1;)
        local.get 0
        local.get 2
        i32.store offset=54912
        local.get 3
        i32.const 48
        i32.add
        global.set $__stack_pointer
        local.get 1
        return
      end
      local.get 3
      i32.const 32
      i32.store8 offset=47
      local.get 3
      i32.const 1701734764
      i32.store offset=43 align=1
      local.get 3
      i64.const 2338042707334751329
      i64.store offset=35 align=1
      local.get 3
      i64.const 2338600898263348341
      i64.store offset=27 align=1
      local.get 3
      i64.const 7162263158133189730
      i64.store offset=19 align=1
      local.get 3
      i64.const 7018969289221893749
      i64.store offset=11 align=1
      local.get 3
      i32.const 11
      i32.add
      i32.const 37
      call $wasi_snapshot_preview1::macros::print
      i32.const 214
      call $wasi_snapshot_preview1::macros::eprint_u32
      local.get 3
      i32.const 8250
      i32.store16 offset=11 align=1
      local.get 3
      i32.const 11
      i32.add
      i32.const 2
      call $wasi_snapshot_preview1::macros::print
      local.get 3
      i32.const 2681
      i32.store16 offset=23 align=1
      local.get 3
      i32.const 1919905125
      i32.store offset=19 align=1
      local.get 3
      i64.const 7863397576860792175
      i64.store offset=11 align=1
      local.get 3
      i32.const 11
      i32.add
      i32.const 14
      call $wasi_snapshot_preview1::macros::print
      local.get 3
      i32.const 10
      i32.store8 offset=11
      local.get 3
      i32.const 11
      i32.add
      i32.const 1
      call $wasi_snapshot_preview1::macros::print
      unreachable
      unreachable
    )
    (func $wasi_snapshot_preview1::ImportAlloc::with_arena (;21;) (type 5) (param i32 i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.load
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=8
          local.set 4
          local.get 0
          local.get 1
          i32.store offset=8
          local.get 4
          i32.eqz
          br_if 1 (;@1;)
          local.get 3
          i32.const 32
          i32.store8 offset=47
          local.get 3
          i32.const 1701734764
          i32.store offset=43 align=1
          local.get 3
          i64.const 2338042707334751329
          i64.store offset=35 align=1
          local.get 3
          i64.const 2338600898263348341
          i64.store offset=27 align=1
          local.get 3
          i64.const 7162263158133189730
          i64.store offset=19 align=1
          local.get 3
          i64.const 7018969289221893749
          i64.store offset=11 align=1
          local.get 3
          i32.const 11
          i32.add
          i32.const 37
          call $wasi_snapshot_preview1::macros::print
          i32.const 276
          call $wasi_snapshot_preview1::macros::eprint_u32
          local.get 3
          i32.const 8250
          i32.store16 offset=11 align=1
          local.get 3
          i32.const 11
          i32.add
          i32.const 2
          call $wasi_snapshot_preview1::macros::print
          local.get 3
          i64.const 748000395109933170
          i64.store offset=27 align=1
          local.get 3
          i64.const 7307218417350680677
          i64.store offset=19 align=1
          local.get 3
          i64.const 8390050488160450159
          i64.store offset=11 align=1
          local.get 3
          i32.const 11
          i32.add
          i32.const 24
          call $wasi_snapshot_preview1::macros::print
          local.get 3
          i32.const 10
          i32.store8 offset=11
          local.get 3
          i32.const 11
          i32.add
          i32.const 1
          call $wasi_snapshot_preview1::macros::print
          unreachable
          unreachable
        end
        local.get 3
        i32.const 32
        i32.store8 offset=47
        local.get 3
        i32.const 1701734764
        i32.store offset=43 align=1
        local.get 3
        i64.const 2338042707334751329
        i64.store offset=35 align=1
        local.get 3
        i64.const 2338600898263348341
        i64.store offset=27 align=1
        local.get 3
        i64.const 7162263158133189730
        i64.store offset=19 align=1
        local.get 3
        i64.const 7018969289221893749
        i64.store offset=11 align=1
        local.get 3
        i32.const 11
        i32.add
        i32.const 37
        call $wasi_snapshot_preview1::macros::print
        i32.const 269
        call $wasi_snapshot_preview1::macros::eprint_u32
        local.get 3
        i32.const 8250
        i32.store16 offset=11 align=1
        local.get 3
        i32.const 11
        i32.add
        i32.const 2
        call $wasi_snapshot_preview1::macros::print
        local.get 3
        i32.const 174417007
        i32.store offset=19 align=1
        local.get 3
        i64.const 7863410729224140130
        i64.store offset=11 align=1
        local.get 3
        i32.const 11
        i32.add
        i32.const 12
        call $wasi_snapshot_preview1::macros::print
        local.get 3
        i32.const 10
        i32.store8 offset=11
        local.get 3
        i32.const 11
        i32.add
        i32.const 1
        call $wasi_snapshot_preview1::macros::print
        unreachable
        unreachable
      end
      local.get 2
      call $wasi_snapshot_preview1::descriptors::Descriptors::open_preopens::get_preopens_import
      local.get 0
      i32.const 0
      i32.store offset=8
      local.get 3
      i32.const 48
      i32.add
      global.set $__stack_pointer
    )
    (func $cabi_export_realloc (;22;) (type 6) (param i32 i32 i32 i32) (result i32)
      (local i32)
      call $allocate_stack
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 0
            br_if 0 (;@3;)
            local.get 1
            br_if 0 (;@3;)
            call $wasi_snapshot_preview1::State::ptr
            local.tee 0
            i32.load
            i32.const 560490357
            i32.ne
            br_if 1 (;@2;)
            local.get 0
            i32.load offset=65532
            i32.const 560490357
            i32.ne
            br_if 2 (;@1;)
            local.get 0
            i32.const 10288
            i32.add
            local.get 2
            local.get 3
            call $wasi_snapshot_preview1::BumpArena::alloc
            local.set 0
            local.get 4
            i32.const 48
            i32.add
            global.set $__stack_pointer
            local.get 0
            return
          end
          local.get 4
          i32.const 32
          i32.store8 offset=47
          local.get 4
          i32.const 1701734764
          i32.store offset=43 align=1
          local.get 4
          i64.const 2338042707334751329
          i64.store offset=35 align=1
          local.get 4
          i64.const 2338600898263348341
          i64.store offset=27 align=1
          local.get 4
          i64.const 7162263158133189730
          i64.store offset=19 align=1
          local.get 4
          i64.const 7018969289221893749
          i64.store offset=11 align=1
          local.get 4
          i32.const 11
          i32.add
          i32.const 37
          call $wasi_snapshot_preview1::macros::print
          i32.const 320
          call $wasi_snapshot_preview1::macros::eprint_u32
          local.get 4
          i32.const 10
          i32.store8 offset=11
          local.get 4
          i32.const 11
          i32.add
          i32.const 1
          call $wasi_snapshot_preview1::macros::print
          unreachable
          unreachable
        end
        local.get 4
        i32.const 32
        i32.store8 offset=47
        local.get 4
        i32.const 1701734764
        i32.store offset=43 align=1
        local.get 4
        i64.const 2338042707334751329
        i64.store offset=35 align=1
        local.get 4
        i64.const 2338600898263348341
        i64.store offset=27 align=1
        local.get 4
        i64.const 7162263158133189730
        i64.store offset=19 align=1
        local.get 4
        i64.const 7018969289221893749
        i64.store offset=11 align=1
        local.get 4
        i32.const 11
        i32.add
        i32.const 37
        call $wasi_snapshot_preview1::macros::print
        i32.const 2552
        call $wasi_snapshot_preview1::macros::eprint_u32
        local.get 4
        i32.const 8250
        i32.store16 offset=11 align=1
        local.get 4
        i32.const 11
        i32.add
        i32.const 2
        call $wasi_snapshot_preview1::macros::print
        local.get 4
        i32.const 10
        i32.store8 offset=27
        local.get 4
        i64.const 7234307576302018670
        i64.store offset=19 align=1
        local.get 4
        i64.const 8028075845441778529
        i64.store offset=11 align=1
        local.get 4
        i32.const 11
        i32.add
        i32.const 17
        call $wasi_snapshot_preview1::macros::print
        local.get 4
        i32.const 10
        i32.store8 offset=11
        local.get 4
        i32.const 11
        i32.add
        i32.const 1
        call $wasi_snapshot_preview1::macros::print
        unreachable
        unreachable
      end
      local.get 4
      i32.const 32
      i32.store8 offset=47
      local.get 4
      i32.const 1701734764
      i32.store offset=43 align=1
      local.get 4
      i64.const 2338042707334751329
      i64.store offset=35 align=1
      local.get 4
      i64.const 2338600898263348341
      i64.store offset=27 align=1
      local.get 4
      i64.const 7162263158133189730
      i64.store offset=19 align=1
      local.get 4
      i64.const 7018969289221893749
      i64.store offset=11 align=1
      local.get 4
      i32.const 11
      i32.add
      i32.const 37
      call $wasi_snapshot_preview1::macros::print
      i32.const 2553
      call $wasi_snapshot_preview1::macros::eprint_u32
      local.get 4
      i32.const 8250
      i32.store16 offset=11 align=1
      local.get 4
      i32.const 11
      i32.add
      i32.const 2
      call $wasi_snapshot_preview1::macros::print
      local.get 4
      i32.const 10
      i32.store8 offset=27
      local.get 4
      i64.const 7234307576302018670
      i64.store offset=19 align=1
      local.get 4
      i64.const 8028075845441778529
      i64.store offset=11 align=1
      local.get 4
      i32.const 11
      i32.add
      i32.const 17
      call $wasi_snapshot_preview1::macros::print
      local.get 4
      i32.const 10
      i32.store8 offset=11
      local.get 4
      i32.const 11
      i32.add
      i32.const 1
      call $wasi_snapshot_preview1::macros::print
      unreachable
      unreachable
    )
    (func $wasi_snapshot_preview1::State::descriptors (;23;) (type 1) (param i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 6176
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          local.get 1
          i32.load offset=16
          br_if 0 (;@2;)
          local.get 1
          i32.const -1
          i32.store offset=16
          local.get 1
          i32.const 24
          i32.add
          local.set 3
          block ;; label = @3
            local.get 1
            i32.const 6172
            i32.add
            i32.load
            i32.const 2
            i32.ne
            br_if 0 (;@3;)
            local.get 2
            i32.const 8
            i32.add
            local.get 1
            i32.const 4
            i32.add
            local.get 1
            i32.const 10288
            i32.add
            call $wasi_snapshot_preview1::descriptors::Descriptors::new
            local.get 3
            local.get 2
            i32.const 8
            i32.add
            i32.const 6168
            call $memcpy
            drop
            local.get 1
            i32.load offset=6172
            i32.const 2
            i32.eq
            br_if 2 (;@1;)
          end
          local.get 0
          local.get 1
          i32.const 16
          i32.add
          i32.store offset=4
          local.get 0
          local.get 3
          i32.store
          local.get 2
          i32.const 6176
          i32.add
          global.set $__stack_pointer
          return
        end
        local.get 2
        i32.const 32
        i32.store8 offset=44
        local.get 2
        i32.const 1701734764
        i32.store offset=40 align=1
        local.get 2
        i64.const 2338042707334751329
        i64.store offset=32 align=1
        local.get 2
        i64.const 2338600898263348341
        i64.store offset=24 align=1
        local.get 2
        i64.const 7162263158133189730
        i64.store offset=16 align=1
        local.get 2
        i64.const 7018969289221893749
        i64.store offset=8 align=1
        local.get 2
        i32.const 8
        i32.add
        i32.const 37
        call $wasi_snapshot_preview1::macros::print
        i32.const 2646
        call $wasi_snapshot_preview1::macros::eprint_u32
        local.get 2
        i32.const 10
        i32.store8 offset=8
        local.get 2
        i32.const 8
        i32.add
        i32.const 1
        call $wasi_snapshot_preview1::macros::print
        unreachable
        unreachable
      end
      local.get 2
      i32.const 32
      i32.store8 offset=44
      local.get 2
      i32.const 1701734764
      i32.store offset=40 align=1
      local.get 2
      i64.const 2338042707334751329
      i64.store offset=32 align=1
      local.get 2
      i64.const 2338600898263348341
      i64.store offset=24 align=1
      local.get 2
      i64.const 7162263158133189730
      i64.store offset=16 align=1
      local.get 2
      i64.const 7018969289221893749
      i64.store offset=8 align=1
      local.get 2
      i32.const 8
      i32.add
      i32.const 37
      call $wasi_snapshot_preview1::macros::print
      i32.const 2650
      call $wasi_snapshot_preview1::macros::eprint_u32
      local.get 2
      i32.const 10
      i32.store8 offset=8
      local.get 2
      i32.const 8
      i32.add
      i32.const 1
      call $wasi_snapshot_preview1::macros::print
      unreachable
      unreachable
    )
    (func $wasi_snapshot_preview1::stream_error_to_errno (;24;) (type 4) (param i32) (result i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      local.get 0
      local.get 1
      i32.const 14
      i32.add
      call $wasi_snapshot_preview1::bindings::wasi::filesystem::types::filesystem_error_code::wit_import
      block ;; label = @1
        block ;; label = @2
          local.get 1
          i32.load8_u offset=14
          br_if 0 (;@2;)
          i32.const 29
          local.set 2
          br 1 (;@1;)
        end
        local.get 1
        i32.load8_u offset=15
        call $wasi_snapshot_preview1::<impl core::convert::From<wasi_snapshot_preview1::bindings::wasi::filesystem::types::ErrorCode> for wasi::lib_generated::Errno>::from
        local.set 2
      end
      local.get 0
      call $<wasi_snapshot_preview1::bindings::wasi::io::error::Error as wit_bindgen::WasmResource>::drop::drop
      local.get 1
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 2
    )
    (func $fd_write (;25;) (type 6) (param i32 i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32)
      call $allocate_stack
      global.get $__stack_pointer
      i32.const 112
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            call $get_allocation_state
            i32.const -2
            i32.add
            i32.const -3
            i32.and
            br_if 0 (;@3;)
            block ;; label = @4
              local.get 2
              i32.eqz
              br_if 0 (;@4;)
              loop ;; label = @5
                local.get 1
                i32.const 4
                i32.add
                i32.load
                local.tee 5
                br_if 3 (;@2;)
                local.get 1
                i32.const 8
                i32.add
                local.set 1
                local.get 2
                i32.const -1
                i32.add
                local.tee 2
                br_if 0 (;@5;)
              end
            end
            i32.const 0
            local.set 1
            local.get 3
            i32.const 0
            i32.store
            br 2 (;@1;)
          end
          local.get 3
          i32.const 0
          i32.store
          i32.const 29
          local.set 1
          br 1 (;@1;)
        end
        local.get 1
        i32.load
        local.set 6
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                call $wasi_snapshot_preview1::State::ptr
                local.tee 1
                i32.load
                i32.const 560490357
                i32.ne
                br_if 0 (;@5;)
                local.get 1
                i32.load offset=65532
                i32.const 560490357
                i32.ne
                br_if 1 (;@4;)
                local.get 4
                i32.const 8
                i32.add
                local.get 1
                call $wasi_snapshot_preview1::State::descriptors
                local.get 4
                i32.load offset=8
                local.tee 7
                i32.load16_u offset=6144
                local.set 8
                local.get 4
                i32.load offset=12
                local.set 2
                i32.const 8
                local.set 1
                i32.const 0
                local.get 0
                call $<core::result::Result<T,E> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap
                local.tee 0
                local.get 8
                i32.ge_u
                br_if 3 (;@2;)
                local.get 7
                local.get 0
                i32.const 48
                i32.mul
                i32.add
                local.tee 0
                i32.load
                i32.const 1
                i32.ne
                br_if 3 (;@2;)
                local.get 4
                i32.const 16
                i32.add
                local.get 0
                i32.const 8
                i32.add
                call $wasi_snapshot_preview1::descriptors::Streams::get_write_stream
                block ;; label = @6
                  local.get 4
                  i32.load16_u offset=16
                  br_if 0 (;@6;)
                  local.get 4
                  i32.load offset=20
                  local.set 1
                  block ;; label = @7
                    local.get 0
                    i32.const 41
                    i32.add
                    i32.load8_u
                    local.tee 8
                    i32.const 2
                    i32.eq
                    br_if 0 (;@7;)
                    local.get 4
                    i32.const 16
                    i32.add
                    local.get 8
                    i32.const 0
                    i32.ne
                    local.get 1
                    local.get 6
                    local.get 5
                    call $wasi_snapshot_preview1::BlockingMode::write
                    local.get 4
                    i32.load16_u offset=16
                    br_if 1 (;@6;)
                    br 4 (;@3;)
                  end
                  local.get 4
                  i32.const 16
                  i32.add
                  i32.const 1
                  local.get 1
                  local.get 6
                  local.get 5
                  call $wasi_snapshot_preview1::BlockingMode::write
                  local.get 4
                  i32.load16_u offset=16
                  i32.eqz
                  br_if 3 (;@3;)
                end
                local.get 4
                i32.load16_u offset=18
                local.set 1
                br 3 (;@2;)
              end
              local.get 4
              i32.const 32
              i32.store8 offset=52
              local.get 4
              i32.const 1701734764
              i32.store offset=48 align=1
              local.get 4
              i64.const 2338042707334751329
              i64.store offset=40 align=1
              local.get 4
              i64.const 2338600898263348341
              i64.store offset=32 align=1
              local.get 4
              i64.const 7162263158133189730
              i64.store offset=24 align=1
              local.get 4
              i64.const 7018969289221893749
              i64.store offset=16 align=1
              local.get 4
              i32.const 16
              i32.add
              i32.const 37
              call $wasi_snapshot_preview1::macros::print
              i32.const 2552
              call $wasi_snapshot_preview1::macros::eprint_u32
              local.get 4
              i32.const 8250
              i32.store16 offset=16 align=1
              local.get 4
              i32.const 16
              i32.add
              i32.const 2
              call $wasi_snapshot_preview1::macros::print
              local.get 4
              i32.const 10
              i32.store8 offset=32
              local.get 4
              i64.const 7234307576302018670
              i64.store offset=24 align=1
              local.get 4
              i64.const 8028075845441778529
              i64.store offset=16 align=1
              local.get 4
              i32.const 16
              i32.add
              i32.const 17
              call $wasi_snapshot_preview1::macros::print
              local.get 4
              i32.const 10
              i32.store8 offset=16
              local.get 4
              i32.const 16
              i32.add
              i32.const 1
              call $wasi_snapshot_preview1::macros::print
              unreachable
              unreachable
            end
            local.get 4
            i32.const 32
            i32.store8 offset=52
            local.get 4
            i32.const 1701734764
            i32.store offset=48 align=1
            local.get 4
            i64.const 2338042707334751329
            i64.store offset=40 align=1
            local.get 4
            i64.const 2338600898263348341
            i64.store offset=32 align=1
            local.get 4
            i64.const 7162263158133189730
            i64.store offset=24 align=1
            local.get 4
            i64.const 7018969289221893749
            i64.store offset=16 align=1
            local.get 4
            i32.const 16
            i32.add
            i32.const 37
            call $wasi_snapshot_preview1::macros::print
            i32.const 2553
            call $wasi_snapshot_preview1::macros::eprint_u32
            local.get 4
            i32.const 8250
            i32.store16 offset=16 align=1
            local.get 4
            i32.const 16
            i32.add
            i32.const 2
            call $wasi_snapshot_preview1::macros::print
            local.get 4
            i32.const 10
            i32.store8 offset=32
            local.get 4
            i64.const 7234307576302018670
            i64.store offset=24 align=1
            local.get 4
            i64.const 8028075845441778529
            i64.store offset=16 align=1
            local.get 4
            i32.const 16
            i32.add
            i32.const 17
            call $wasi_snapshot_preview1::macros::print
            local.get 4
            i32.const 10
            i32.store8 offset=16
            local.get 4
            i32.const 16
            i32.add
            i32.const 1
            call $wasi_snapshot_preview1::macros::print
            unreachable
            unreachable
          end
          local.get 4
          i32.load offset=20
          local.set 1
          block ;; label = @3
            block ;; label = @4
              local.get 0
              i32.load8_u offset=41
              i32.const 2
              i32.eq
              br_if 0 (;@4;)
              block ;; label = @5
                local.get 0
                i32.const 40
                i32.add
                i32.load8_u
                br_if 0 (;@5;)
                local.get 0
                i32.const 32
                i32.add
                local.tee 5
                local.get 5
                i64.load
                local.get 1
                i64.extend_i32_u
                i64.add
                i64.store
                br 1 (;@4;)
              end
              local.get 4
              i32.const 16
              i32.add
              local.get 0
              i32.const 24
              i32.add
              call $wasi_snapshot_preview1::bindings::wasi::filesystem::types::Descriptor::stat
              local.get 4
              i64.load offset=88
              i64.const 2
              i64.eq
              br_if 1 (;@3;)
              local.get 0
              i32.const 32
              i32.add
              local.get 4
              i64.load offset=32
              i64.store
            end
            local.get 3
            local.get 1
            i32.store
            i32.const 0
            local.set 1
            br 1 (;@2;)
          end
          local.get 4
          i32.load8_u offset=16
          call $wasi_snapshot_preview1::<impl core::convert::From<wasi_snapshot_preview1::bindings::wasi::filesystem::types::ErrorCode> for wasi::lib_generated::Errno>::from
          local.set 1
        end
        local.get 2
        local.get 2
        i32.load
        i32.const 1
        i32.add
        i32.store
      end
      local.get 4
      i32.const 112
      i32.add
      global.set $__stack_pointer
      local.get 1
      i32.const 65535
      i32.and
    )
    (func $wasi_snapshot_preview1::BlockingMode::write (;26;) (type 7) (param i32 i32 i32 i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 5
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    local.get 1
                    i32.eqz
                    br_if 0 (;@7;)
                    local.get 4
                    local.set 1
                    loop ;; label = @8
                      local.get 1
                      i32.eqz
                      br_if 2 (;@6;)
                      local.get 5
                      i32.const 8
                      i32.add
                      local.get 2
                      local.get 3
                      local.get 1
                      i32.const 4096
                      local.get 1
                      i32.const 4096
                      i32.lt_u
                      select
                      local.tee 6
                      call $wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream::blocking_write_and_flush
                      local.get 1
                      local.get 6
                      i32.sub
                      local.set 1
                      local.get 3
                      local.get 6
                      i32.add
                      local.set 3
                      local.get 5
                      i32.load offset=8
                      local.tee 6
                      i32.const 2
                      i32.eq
                      br_if 0 (;@8;)
                    end
                    local.get 6
                    br_table 2 (;@5;) 3 (;@4;) 2 (;@5;)
                  end
                  local.get 5
                  i32.const 32
                  i32.add
                  local.get 2
                  call $wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream::check_write
                  block ;; label = @7
                    block ;; label = @8
                      local.get 5
                      i32.load offset=32
                      br_if 0 (;@8;)
                      local.get 5
                      i32.load offset=40
                      local.set 1
                      br 1 (;@7;)
                    end
                    i32.const 0
                    local.set 1
                    local.get 5
                    i32.load offset=36
                    i32.eqz
                    br_if 5 (;@2;)
                  end
                  block ;; label = @7
                    local.get 4
                    local.get 1
                    local.get 4
                    local.get 1
                    i32.lt_u
                    select
                    local.tee 1
                    br_if 0 (;@7;)
                    local.get 0
                    i32.const 0
                    i32.store16
                    local.get 0
                    i32.const 0
                    i32.store offset=4
                    br 6 (;@1;)
                  end
                  local.get 5
                  i32.const 24
                  i32.add
                  local.get 2
                  local.get 3
                  local.get 1
                  call $wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream::write
                  block ;; label = @7
                    block ;; label = @8
                      block ;; label = @9
                        block ;; label = @10
                          local.get 5
                          i32.load offset=24
                          br_table 1 (;@9;) 2 (;@8;) 0 (;@10;) 1 (;@9;)
                        end
                        local.get 5
                        i32.const 16
                        i32.add
                        local.get 2
                        call $wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream::blocking_flush
                        block ;; label = @10
                          block ;; label = @11
                            block ;; label = @12
                              block ;; label = @13
                                local.get 5
                                i32.load offset=16
                                br_table 1 (;@12;) 2 (;@11;) 0 (;@13;) 1 (;@12;)
                              end
                              local.get 0
                              i32.const 0
                              i32.store16
                              local.get 0
                              local.get 1
                              i32.store offset=4
                              br 11 (;@1;)
                            end
                            local.get 0
                            local.get 5
                            i32.load offset=20
                            call $wasi_snapshot_preview1::stream_error_to_errno
                            i32.store16 offset=2
                            i32.const 1
                            local.set 1
                            br 1 (;@10;)
                          end
                          i32.const 0
                          local.set 1
                          local.get 0
                          i32.const 0
                          i32.store offset=4
                        end
                        local.get 0
                        local.get 1
                        i32.store16
                        br 8 (;@1;)
                      end
                      local.get 0
                      local.get 5
                      i32.load offset=28
                      call $wasi_snapshot_preview1::stream_error_to_errno
                      i32.store16 offset=2
                      i32.const 1
                      local.set 1
                      br 1 (;@7;)
                    end
                    i32.const 0
                    local.set 1
                    local.get 0
                    i32.const 0
                    i32.store offset=4
                  end
                  local.get 0
                  local.get 1
                  i32.store16
                  br 5 (;@1;)
                end
                local.get 0
                i32.const 0
                i32.store16
                local.get 0
                local.get 4
                i32.store offset=4
                br 4 (;@1;)
              end
              local.get 5
              i32.load offset=12
              call $wasi_snapshot_preview1::stream_error_to_errno
              local.set 1
              br 1 (;@3;)
            end
            i32.const 29
            local.set 1
          end
          local.get 0
          i32.const 1
          i32.store16
          local.get 0
          local.get 1
          i32.store16 offset=2
          br 1 (;@1;)
        end
        local.get 5
        i32.const 40
        i32.add
        i32.load
        call $wasi_snapshot_preview1::stream_error_to_errno
        local.set 1
        local.get 0
        i32.const 1
        i32.store16
        local.get 0
        local.get 1
        i32.store16 offset=2
      end
      local.get 5
      i32.const 48
      i32.add
      global.set $__stack_pointer
    )
    (func $wasi_snapshot_preview1::State::new (;27;) (type 8) (result i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 0
      global.set $__stack_pointer
      block ;; label = @1
        call $get_allocation_state
        i32.const 2
        i32.ne
        br_if 0 (;@1;)
        i32.const 3
        call $set_allocation_state
        i32.const 0
        i32.const 0
        i32.const 8
        i32.const 65536
        call $wasi_snapshot_preview1::State::new::cabi_realloc
        local.set 1
        i32.const 4
        call $set_allocation_state
        local.get 1
        i64.const 0
        i64.store offset=4 align=4
        local.get 1
        i32.const 560490357
        i32.store
        local.get 1
        i32.const 12
        i32.add
        i64.const 0
        i64.store align=4
        local.get 1
        i64.const 0
        i64.store offset=65488
        local.get 1
        i32.const 0
        i32.store offset=65480
        local.get 1
        i32.const 0
        i32.store offset=65212
        local.get 1
        i64.const 0
        i64.store offset=65200
        local.get 1
        i32.const 2
        i32.store offset=6172
        local.get 1
        i32.const 65496
        i32.add
        i64.const 0
        i64.store
        local.get 1
        i32.const 65504
        i32.add
        i64.const 0
        i64.store
        local.get 1
        i32.const 65509
        i32.add
        i64.const 0
        i64.store align=1
        local.get 1
        i32.const 560490357
        i32.store offset=65532
        local.get 1
        i32.const 11822
        i32.store16 offset=65528
        local.get 1
        i32.const 0
        i32.store offset=65520
        local.get 0
        i32.const 48
        i32.add
        global.set $__stack_pointer
        local.get 1
        return
      end
      local.get 0
      i32.const 32
      i32.store8 offset=47
      local.get 0
      i32.const 1701734764
      i32.store offset=43 align=1
      local.get 0
      i64.const 2338042707334751329
      i64.store offset=35 align=1
      local.get 0
      i64.const 2338600898263348341
      i64.store offset=27 align=1
      local.get 0
      i64.const 7162263158133189730
      i64.store offset=19 align=1
      local.get 0
      i64.const 7018969289221893749
      i64.store offset=11 align=1
      local.get 0
      i32.const 11
      i32.add
      i32.const 37
      call $wasi_snapshot_preview1::macros::print
      i32.const 2584
      call $wasi_snapshot_preview1::macros::eprint_u32
      local.get 0
      i32.const 8250
      i32.store16 offset=11 align=1
      local.get 0
      i32.const 11
      i32.add
      i32.const 2
      call $wasi_snapshot_preview1::macros::print
      local.get 0
      i32.const 10
      i32.store8 offset=27
      local.get 0
      i64.const 7234307576302018670
      i64.store offset=19 align=1
      local.get 0
      i64.const 8028075845441778529
      i64.store offset=11 align=1
      local.get 0
      i32.const 11
      i32.add
      i32.const 17
      call $wasi_snapshot_preview1::macros::print
      local.get 0
      i32.const 10
      i32.store8 offset=11
      local.get 0
      i32.const 11
      i32.add
      i32.const 1
      call $wasi_snapshot_preview1::macros::print
      unreachable
      unreachable
    )
    (func $wasi_snapshot_preview1::bindings::wasi::filesystem::types::Descriptor::stat (;28;) (type 1) (param i32 i32)
      (local i32 i32 i32 i64 i64 i32 i64 i32 i32 i64 i64 i64 i64 i64)
      global.get $__stack_pointer
      i32.const 112
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 1
      i32.load
      local.get 2
      i32.const 8
      i32.add
      call $wasi_snapshot_preview1::bindings::wasi::filesystem::types::Descriptor::stat::wit_import
      local.get 2
      i32.const 16
      i32.add
      i32.load8_u
      local.set 1
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 2
              i32.load8_u offset=8
              br_if 0 (;@4;)
              local.get 2
              i32.const 88
              i32.add
              local.set 3
              local.get 2
              i32.const 64
              i32.add
              i32.load8_u
              local.set 4
              i64.const 0
              local.set 5
              local.get 2
              i32.const 40
              i32.add
              i32.load8_u
              br_if 1 (;@3;)
              i64.const 0
              local.set 6
              br 2 (;@2;)
            end
            local.get 0
            i64.const 2
            i64.store offset=72
            br 2 (;@1;)
          end
          local.get 2
          i32.const 56
          i32.add
          i32.load
          local.set 7
          local.get 2
          i32.const 48
          i32.add
          i64.load
          local.set 8
          i64.const 1
          local.set 6
        end
        local.get 2
        i32.const 32
        i32.add
        local.set 9
        local.get 2
        i32.const 24
        i32.add
        local.set 10
        local.get 3
        i32.load8_u
        local.set 3
        block ;; label = @2
          block ;; label = @3
            local.get 4
            i32.const 255
            i32.and
            br_if 0 (;@3;)
            br 1 (;@2;)
          end
          local.get 2
          i32.const 80
          i32.add
          i32.load
          local.set 4
          local.get 2
          i32.const 72
          i32.add
          i64.load
          local.set 11
          i64.const 1
          local.set 5
        end
        local.get 9
        i64.load
        local.set 12
        local.get 10
        i64.load
        local.set 13
        block ;; label = @2
          block ;; label = @3
            local.get 3
            i32.const 255
            i32.and
            br_if 0 (;@3;)
            i64.const 0
            local.set 14
            br 1 (;@2;)
          end
          local.get 2
          i32.const 104
          i32.add
          i32.load
          local.set 3
          local.get 2
          i32.const 96
          i32.add
          i64.load
          local.set 15
          i64.const 1
          local.set 14
        end
        local.get 0
        local.get 3
        i32.store offset=88
        local.get 0
        local.get 15
        i64.store offset=80
        local.get 0
        local.get 14
        i64.store offset=72
        local.get 0
        local.get 4
        i32.store offset=64
        local.get 0
        local.get 11
        i64.store offset=56
        local.get 0
        local.get 5
        i64.store offset=48
        local.get 0
        local.get 7
        i32.store offset=40
        local.get 0
        local.get 8
        i64.store offset=32
        local.get 0
        local.get 6
        i64.store offset=24
        local.get 0
        local.get 12
        i64.store offset=16
        local.get 0
        local.get 13
        i64.store offset=8
      end
      local.get 0
      local.get 1
      i32.store8
      local.get 2
      i32.const 112
      i32.add
      global.set $__stack_pointer
    )
    (func $wasi_snapshot_preview1::macros::print (;29;) (type 1) (param i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 2
      call $wasi_snapshot_preview1::bindings::wasi::cli::stderr::get_stderr::wit_import
      local.tee 3
      i32.store offset=12
      local.get 2
      local.get 2
      i32.const 12
      i32.add
      local.get 0
      local.get 1
      call $wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream::blocking_write_and_flush
      block ;; label = @1
        local.get 2
        i32.load
        br_if 0 (;@1;)
        local.get 2
        i32.load offset=4
        call $<wasi_snapshot_preview1::bindings::wasi::io::error::Error as wit_bindgen::WasmResource>::drop::drop
      end
      local.get 3
      call $<wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream as wit_bindgen::WasmResource>::drop::drop
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream::blocking_write_and_flush (;30;) (type 3) (param i32 i32 i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      local.get 1
      i32.load
      local.get 2
      local.get 3
      local.get 4
      i32.const 4
      i32.add
      call $wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream::blocking_write_and_flush::wit_import
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 4
              i32.load8_u offset=4
              br_if 0 (;@4;)
              i32.const 2
              local.set 3
              br 1 (;@3;)
            end
            local.get 4
            i32.const 8
            i32.add
            i32.load8_u
            i32.eqz
            br_if 1 (;@2;)
            i32.const 1
            local.set 3
          end
          br 1 (;@1;)
        end
        local.get 4
        i32.const 12
        i32.add
        i32.load
        local.set 1
        i32.const 0
        local.set 3
      end
      local.get 0
      local.get 1
      i32.store offset=4
      local.get 0
      local.get 3
      i32.store
      local.get 4
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $wasi_snapshot_preview1::macros::eprint_u32 (;31;) (type 0) (param i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          local.get 0
          br_if 0 (;@2;)
          local.get 1
          i32.const 48
          i32.store8 offset=15
          local.get 1
          i32.const 15
          i32.add
          i32.const 1
          call $wasi_snapshot_preview1::macros::print
          br 1 (;@1;)
        end
        local.get 0
        call $wasi_snapshot_preview1::macros::eprint_u32::eprint_u32_impl
      end
      local.get 1
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $wasi_snapshot_preview1::macros::eprint_u32::eprint_u32_impl (;32;) (type 0) (param i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      block ;; label = @1
        local.get 0
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.const 10
        i32.div_u
        local.tee 2
        call $wasi_snapshot_preview1::macros::eprint_u32::eprint_u32_impl
        local.get 1
        local.get 0
        local.get 2
        i32.const 10
        i32.mul
        i32.sub
        i32.const 48
        i32.or
        i32.store8 offset=15
        local.get 1
        i32.const 15
        i32.add
        i32.const 1
        call $wasi_snapshot_preview1::macros::print
      end
      local.get 1
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $<core::option::Option<T> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap (;33;) (type 4) (param i32) (result i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      block ;; label = @1
        local.get 0
        br_if 0 (;@1;)
        local.get 1
        i32.const 32
        i32.store8 offset=47
        local.get 1
        i32.const 1701734764
        i32.store offset=43 align=1
        local.get 1
        i64.const 2338042707334751329
        i64.store offset=35 align=1
        local.get 1
        i64.const 2338600898263348341
        i64.store offset=27 align=1
        local.get 1
        i64.const 7162263158133189730
        i64.store offset=19 align=1
        local.get 1
        i64.const 7018969289221893749
        i64.store offset=11 align=1
        local.get 1
        i32.const 11
        i32.add
        i32.const 37
        call $wasi_snapshot_preview1::macros::print
        i32.const 134
        call $wasi_snapshot_preview1::macros::eprint_u32
        local.get 1
        i32.const 10
        i32.store8 offset=11
        local.get 1
        i32.const 11
        i32.add
        i32.const 1
        call $wasi_snapshot_preview1::macros::print
        unreachable
        unreachable
      end
      local.get 1
      i32.const 48
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $<core::option::Option<T> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap (;34;) (type 10) (param i32 i32) (result i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        local.get 0
        br_if 0 (;@1;)
        local.get 2
        i32.const 32
        i32.store8 offset=47
        local.get 2
        i32.const 1701734764
        i32.store offset=43 align=1
        local.get 2
        i64.const 2338042707334751329
        i64.store offset=35 align=1
        local.get 2
        i64.const 2338600898263348341
        i64.store offset=27 align=1
        local.get 2
        i64.const 7162263158133189730
        i64.store offset=19 align=1
        local.get 2
        i64.const 7018969289221893749
        i64.store offset=11 align=1
        local.get 2
        i32.const 11
        i32.add
        i32.const 37
        call $wasi_snapshot_preview1::macros::print
        i32.const 134
        call $wasi_snapshot_preview1::macros::eprint_u32
        local.get 2
        i32.const 10
        i32.store8 offset=11
        local.get 2
        i32.const 11
        i32.add
        i32.const 1
        call $wasi_snapshot_preview1::macros::print
        unreachable
        unreachable
      end
      local.get 2
      i32.const 48
      i32.add
      global.set $__stack_pointer
      local.get 1
    )
    (func $<core::result::Result<T,E> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap (;35;) (type 10) (param i32 i32) (result i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        local.get 0
        br_if 0 (;@1;)
        local.get 2
        i32.const 48
        i32.add
        global.set $__stack_pointer
        local.get 1
        return
      end
      local.get 2
      i32.const 32
      i32.store8 offset=47
      local.get 2
      i32.const 1701734764
      i32.store offset=43 align=1
      local.get 2
      i64.const 2338042707334751329
      i64.store offset=35 align=1
      local.get 2
      i64.const 2338600898263348341
      i64.store offset=27 align=1
      local.get 2
      i64.const 7162263158133189730
      i64.store offset=19 align=1
      local.get 2
      i64.const 7018969289221893749
      i64.store offset=11 align=1
      local.get 2
      i32.const 11
      i32.add
      i32.const 37
      call $wasi_snapshot_preview1::macros::print
      i32.const 143
      call $wasi_snapshot_preview1::macros::eprint_u32
      local.get 2
      i32.const 10
      i32.store8 offset=11
      local.get 2
      i32.const 11
      i32.add
      i32.const 1
      call $wasi_snapshot_preview1::macros::print
      unreachable
      unreachable
    )
    (func $<core::result::Result<T,E> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap (;36;) (type 10) (param i32 i32) (result i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        local.get 0
        br_if 0 (;@1;)
        local.get 2
        i32.const 48
        i32.add
        global.set $__stack_pointer
        local.get 1
        return
      end
      local.get 2
      i32.const 32
      i32.store8 offset=47
      local.get 2
      i32.const 1701734764
      i32.store offset=43 align=1
      local.get 2
      i64.const 2338042707334751329
      i64.store offset=35 align=1
      local.get 2
      i64.const 2338600898263348341
      i64.store offset=27 align=1
      local.get 2
      i64.const 7162263158133189730
      i64.store offset=19 align=1
      local.get 2
      i64.const 7018969289221893749
      i64.store offset=11 align=1
      local.get 2
      i32.const 11
      i32.add
      i32.const 37
      call $wasi_snapshot_preview1::macros::print
      i32.const 143
      call $wasi_snapshot_preview1::macros::eprint_u32
      local.get 2
      i32.const 10
      i32.store8 offset=11
      local.get 2
      i32.const 11
      i32.add
      i32.const 1
      call $wasi_snapshot_preview1::macros::print
      unreachable
      unreachable
    )
    (func $<core::result::Result<T,E> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap (;37;) (type 1) (param i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        local.get 0
        i32.eqz
        br_if 0 (;@1;)
        local.get 2
        i32.const 32
        i32.store8 offset=47
        local.get 2
        i32.const 1701734764
        i32.store offset=43 align=1
        local.get 2
        i64.const 2338042707334751329
        i64.store offset=35 align=1
        local.get 2
        i64.const 2338600898263348341
        i64.store offset=27 align=1
        local.get 2
        i64.const 7162263158133189730
        i64.store offset=19 align=1
        local.get 2
        i64.const 7018969289221893749
        i64.store offset=11 align=1
        local.get 2
        i32.const 11
        i32.add
        i32.const 37
        call $wasi_snapshot_preview1::macros::print
        i32.const 143
        call $wasi_snapshot_preview1::macros::eprint_u32
        local.get 2
        i32.const 10
        i32.store8 offset=11
        local.get 2
        i32.const 11
        i32.add
        i32.const 1
        call $wasi_snapshot_preview1::macros::print
        unreachable
        unreachable
      end
      local.get 2
      i32.const 48
      i32.add
      global.set $__stack_pointer
    )
    (func $<core::result::Result<T,E> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap (;38;) (type 4) (param i32) (result i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      block ;; label = @1
        local.get 0
        i32.load16_u
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=4
        local.set 0
        local.get 1
        i32.const 48
        i32.add
        global.set $__stack_pointer
        local.get 0
        return
      end
      local.get 1
      i32.const 32
      i32.store8 offset=47
      local.get 1
      i32.const 1701734764
      i32.store offset=43 align=1
      local.get 1
      i64.const 2338042707334751329
      i64.store offset=35 align=1
      local.get 1
      i64.const 2338600898263348341
      i64.store offset=27 align=1
      local.get 1
      i64.const 7162263158133189730
      i64.store offset=19 align=1
      local.get 1
      i64.const 7018969289221893749
      i64.store offset=11 align=1
      local.get 1
      i32.const 11
      i32.add
      i32.const 37
      call $wasi_snapshot_preview1::macros::print
      i32.const 143
      call $wasi_snapshot_preview1::macros::eprint_u32
      local.get 1
      i32.const 10
      i32.store8 offset=11
      local.get 1
      i32.const 11
      i32.add
      i32.const 1
      call $wasi_snapshot_preview1::macros::print
      unreachable
      unreachable
    )
    (func $wasi_snapshot_preview1::<impl core::convert::From<wasi_snapshot_preview1::bindings::wasi::filesystem::types::ErrorCode> for wasi::lib_generated::Errno>::from (;39;) (type 4) (param i32) (result i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.set 1
      i32.const 6
      local.set 2
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
                              block ;; label = @13
                                block ;; label = @14
                                  block ;; label = @15
                                    block ;; label = @16
                                      block ;; label = @17
                                        block ;; label = @18
                                          block ;; label = @19
                                            block ;; label = @20
                                              block ;; label = @21
                                                block ;; label = @22
                                                  block ;; label = @23
                                                    block ;; label = @24
                                                      block ;; label = @25
                                                        block ;; label = @26
                                                          block ;; label = @27
                                                            block ;; label = @28
                                                              block ;; label = @29
                                                                block ;; label = @30
                                                                  block ;; label = @31
                                                                    block ;; label = @32
                                                                      block ;; label = @33
                                                                        block ;; label = @34
                                                                          block ;; label = @35
                                                                            block ;; label = @36
                                                                              block ;; label = @37
                                                                                local.get 0
                                                                                i32.const 255
                                                                                i32.and
                                                                                br_table 0 (;@37;) 36 (;@1;) 1 (;@36;) 2 (;@35;) 3 (;@34;) 4 (;@33;) 5 (;@32;) 6 (;@31;) 7 (;@30;) 8 (;@29;) 9 (;@28;) 10 (;@27;) 11 (;@26;) 12 (;@25;) 13 (;@24;) 14 (;@23;) 15 (;@22;) 16 (;@21;) 17 (;@20;) 18 (;@19;) 19 (;@18;) 20 (;@17;) 21 (;@16;) 22 (;@15;) 23 (;@14;) 24 (;@13;) 25 (;@12;) 26 (;@11;) 27 (;@10;) 28 (;@9;) 29 (;@8;) 30 (;@7;) 31 (;@6;) 32 (;@5;) 33 (;@4;) 34 (;@3;) 35 (;@2;) 0 (;@37;)
                                                                              end
                                                                              local.get 1
                                                                              i32.const 2
                                                                              i32.store16 offset=14
                                                                              local.get 1
                                                                              i32.const 14
                                                                              i32.add
                                                                              local.set 0
                                                                              local.get 1
                                                                              i32.load16_u offset=14
                                                                              return
                                                                            end
                                                                            i32.const 7
                                                                            return
                                                                          end
                                                                          i32.const 8
                                                                          return
                                                                        end
                                                                        i32.const 10
                                                                        return
                                                                      end
                                                                      i32.const 16
                                                                      return
                                                                    end
                                                                    i32.const 19
                                                                    return
                                                                  end
                                                                  i32.const 20
                                                                  return
                                                                end
                                                                i32.const 22
                                                                return
                                                              end
                                                              i32.const 25
                                                              return
                                                            end
                                                            i32.const 26
                                                            return
                                                          end
                                                          i32.const 27
                                                          return
                                                        end
                                                        i32.const 28
                                                        return
                                                      end
                                                      i32.const 29
                                                      return
                                                    end
                                                    i32.const 31
                                                    return
                                                  end
                                                  i32.const 32
                                                  return
                                                end
                                                i32.const 34
                                                return
                                              end
                                              i32.const 35
                                              return
                                            end
                                            i32.const 37
                                            return
                                          end
                                          i32.const 43
                                          return
                                        end
                                        i32.const 44
                                        return
                                      end
                                      i32.const 46
                                      return
                                    end
                                    i32.const 48
                                    return
                                  end
                                  i32.const 51
                                  return
                                end
                                i32.const 54
                                return
                              end
                              i32.const 55
                              return
                            end
                            i32.const 56
                            return
                          end
                          i32.const 58
                          return
                        end
                        i32.const 59
                        return
                      end
                      i32.const 60
                      return
                    end
                    i32.const 61
                    return
                  end
                  i32.const 63
                  return
                end
                i32.const 64
                return
              end
              i32.const 69
              return
            end
            i32.const 70
            return
          end
          i32.const 74
          return
        end
        i32.const 75
        local.set 2
      end
      local.get 2
    )
    (func $wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream::check_write (;40;) (type 1) (param i32 i32)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 1
      i32.load
      local.get 2
      call $wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream::check_write::wit_import
      block ;; label = @1
        block ;; label = @2
          local.get 2
          i32.load8_u
          br_if 0 (;@2;)
          local.get 0
          local.get 2
          i32.const 8
          i32.add
          i64.load
          i64.store offset=8
          i32.const 0
          local.set 1
          br 1 (;@1;)
        end
        i32.const 1
        local.set 1
        i32.const 1
        local.set 3
        block ;; label = @2
          local.get 2
          i32.const 8
          i32.add
          i32.load8_u
          br_if 0 (;@2;)
          local.get 2
          i32.const 12
          i32.add
          i32.load
          local.set 4
          i32.const 0
          local.set 3
        end
        local.get 0
        local.get 3
        i32.store offset=4
        local.get 0
        i32.const 8
        i32.add
        local.get 4
        i32.store
      end
      local.get 0
      local.get 1
      i32.store
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream::write (;41;) (type 3) (param i32 i32 i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      local.get 1
      i32.load
      local.get 2
      local.get 3
      local.get 4
      i32.const 4
      i32.add
      call $wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream::write::wit_import
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 4
              i32.load8_u offset=4
              br_if 0 (;@4;)
              i32.const 2
              local.set 3
              br 1 (;@3;)
            end
            local.get 4
            i32.const 8
            i32.add
            i32.load8_u
            i32.eqz
            br_if 1 (;@2;)
            i32.const 1
            local.set 3
          end
          br 1 (;@1;)
        end
        local.get 4
        i32.const 12
        i32.add
        i32.load
        local.set 1
        i32.const 0
        local.set 3
      end
      local.get 0
      local.get 1
      i32.store offset=4
      local.get 0
      local.get 3
      i32.store
      local.get 4
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream::blocking_flush (;42;) (type 1) (param i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 1
      i32.load
      local.get 2
      i32.const 4
      i32.add
      call $wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream::blocking_flush::wit_import
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 2
              i32.load8_u offset=4
              br_if 0 (;@4;)
              i32.const 2
              local.set 3
              br 1 (;@3;)
            end
            local.get 2
            i32.const 8
            i32.add
            i32.load8_u
            i32.eqz
            br_if 1 (;@2;)
            i32.const 1
            local.set 3
          end
          br 1 (;@1;)
        end
        local.get 2
        i32.const 12
        i32.add
        i32.load
        local.set 1
        i32.const 0
        local.set 3
      end
      local.get 0
      local.get 1
      i32.store offset=4
      local.get 0
      local.get 3
      i32.store
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $core::ptr::drop_in_place<wasi_snapshot_preview1::descriptors::Descriptor> (;43;) (type 0) (param i32)
      block ;; label = @1
        local.get 0
        i32.load
        i32.const 1
        i32.ne
        br_if 0 (;@1;)
        block ;; label = @2
          local.get 0
          i32.load offset=8
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.const 12
          i32.add
          i32.load
          call $<wasi_snapshot_preview1::bindings::wasi::io::streams::InputStream as wit_bindgen::WasmResource>::drop::drop
        end
        block ;; label = @2
          local.get 0
          i32.const 16
          i32.add
          i32.load
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.const 20
          i32.add
          i32.load
          call $<wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream as wit_bindgen::WasmResource>::drop::drop
        end
        local.get 0
        i32.const 41
        i32.add
        i32.load8_u
        i32.const 2
        i32.eq
        br_if 0 (;@1;)
        local.get 0
        i32.const 24
        i32.add
        i32.load
        call $<wasi_snapshot_preview1::bindings::wasi::filesystem::types::Descriptor as wit_bindgen::WasmResource>::drop::drop
      end
    )
    (func $wasi_snapshot_preview1::descriptors::Streams::get_write_stream (;44;) (type 1) (param i32 i32)
      (local i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 1
      i32.const 12
      i32.add
      local.set 3
      block ;; label = @1
        block ;; label = @2
          local.get 1
          i32.load offset=8
          br_if 0 (;@2;)
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    local.get 1
                    i32.const 33
                    i32.add
                    i32.load8_u
                    i32.const 2
                    i32.eq
                    br_if 0 (;@7;)
                    block ;; label = @8
                      local.get 1
                      i32.const 20
                      i32.add
                      i32.load8_u
                      i32.const 3
                      i32.ne
                      br_if 0 (;@8;)
                      local.get 0
                      i32.const 8
                      i32.store16 offset=2
                      br 3 (;@5;)
                    end
                    block ;; label = @8
                      local.get 1
                      i32.const 32
                      i32.add
                      i32.load8_u
                      br_if 0 (;@8;)
                      local.get 1
                      i32.load offset=16
                      local.get 1
                      i32.const 24
                      i32.add
                      i64.load
                      local.get 2
                      i32.const 8
                      i32.add
                      call $wasi_snapshot_preview1::bindings::wasi::filesystem::types::Descriptor::write_via_stream::wit_import
                      local.get 2
                      i32.load8_u offset=8
                      br_if 2 (;@6;)
                      local.get 2
                      i32.const 12
                      i32.add
                      i32.load
                      local.set 4
                      br 5 (;@3;)
                    end
                    local.get 1
                    i32.load offset=16
                    local.get 2
                    i32.const 8
                    i32.add
                    call $wasi_snapshot_preview1::bindings::wasi::filesystem::types::Descriptor::append_via_stream::wit_import
                    local.get 2
                    i32.load8_u offset=8
                    i32.eqz
                    br_if 3 (;@4;)
                    local.get 0
                    local.get 2
                    i32.const 12
                    i32.add
                    i32.load8_u
                    call $wasi_snapshot_preview1::<impl core::convert::From<wasi_snapshot_preview1::bindings::wasi::filesystem::types::ErrorCode> for wasi::lib_generated::Errno>::from
                    i32.store16 offset=2
                    br 2 (;@5;)
                  end
                  local.get 0
                  i32.const 8
                  i32.store16 offset=2
                  br 1 (;@5;)
                end
                local.get 0
                local.get 2
                i32.const 12
                i32.add
                i32.load8_u
                call $wasi_snapshot_preview1::<impl core::convert::From<wasi_snapshot_preview1::bindings::wasi::filesystem::types::ErrorCode> for wasi::lib_generated::Errno>::from
                i32.store16 offset=2
              end
              i32.const 1
              local.set 1
              br 3 (;@1;)
            end
            local.get 2
            i32.const 12
            i32.add
            i32.load
            local.set 4
          end
          i32.const 1
          local.set 5
          block ;; label = @3
            local.get 1
            i32.load offset=8
            br_if 0 (;@3;)
            local.get 1
            local.get 4
            i32.store offset=12
            local.get 1
            i32.const 1
            i32.store offset=8
            i32.const 0
            local.set 5
          end
          local.get 5
          local.get 4
          call $<core::result::Result<T,E> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap
          local.get 3
          i32.const 0
          local.get 1
          i32.load offset=8
          select
          call $<core::option::Option<T> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap
          local.set 3
        end
        local.get 0
        local.get 3
        i32.store offset=4
        i32.const 0
        local.set 1
      end
      local.get 0
      local.get 1
      i32.store16
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $wasi_snapshot_preview1::descriptors::Descriptors::new (;45;) (type 5) (param i32 i32 i32)
      (local i32 i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 6240
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      i32.const 0
      i32.store offset=6164
      local.get 3
      i32.const 0
      i32.store offset=6156
      call $wasi_snapshot_preview1::bindings::wasi::cli::stdin::get_stdin::wit_import
      local.set 4
      local.get 3
      i32.const 2
      i32.store8 offset=49
      local.get 3
      i32.const 0
      i32.store8 offset=32
      local.get 3
      i32.const 0
      i32.store offset=24
      local.get 3
      local.get 4
      i32.store offset=20
      local.get 3
      i32.const 1
      i32.store offset=16
      local.get 3
      i32.const 1
      i32.store offset=8
      local.get 3
      i32.const 0
      i32.store offset=6196
      local.get 3
      i32.const 0
      i32.store16 offset=6192
      local.get 3
      i32.const 6192
      i32.add
      call $<core::result::Result<T,E> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap
      drop
      call $wasi_snapshot_preview1::bindings::wasi::cli::stdout::get_stdout::wit_import
      local.set 4
      local.get 3
      i32.const 80
      i32.add
      i32.const 1
      i32.store8
      local.get 3
      i32.const 76
      i32.add
      local.get 4
      i32.store
      local.get 3
      i32.const 72
      i32.add
      i32.const 1
      i32.store
      local.get 3
      i32.const 64
      i32.add
      i32.const 0
      i32.store
      local.get 3
      i32.const 89
      i32.add
      local.get 3
      i32.const 6200
      i32.add
      local.tee 4
      i64.load align=1
      i64.store align=1
      local.get 3
      i32.const 102
      i32.add
      local.get 3
      i32.const 6180
      i32.add
      local.tee 5
      i32.load16_u
      i32.store16
      local.get 3
      i32.const 1
      i32.store offset=56
      local.get 3
      i32.const 2
      i32.store8 offset=97
      local.get 3
      i32.const 1
      i32.store offset=6188
      local.get 3
      i32.const 0
      i32.store16 offset=6184
      local.get 3
      local.get 3
      i64.load offset=6192 align=1
      i64.store offset=81 align=1
      local.get 3
      local.get 3
      i32.load offset=6176 align=2
      i32.store offset=98 align=2
      local.get 3
      i32.const 6184
      i32.add
      call $<core::result::Result<T,E> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap
      drop
      call $wasi_snapshot_preview1::bindings::wasi::cli::stderr::get_stderr::wit_import
      local.set 6
      local.get 3
      i32.const 128
      i32.add
      i32.const 2
      i32.store8
      local.get 3
      i32.const 124
      i32.add
      local.get 6
      i32.store
      local.get 3
      i32.const 120
      i32.add
      i32.const 1
      i32.store
      local.get 3
      i32.const 112
      i32.add
      i32.const 0
      i32.store
      local.get 3
      i32.const 137
      i32.add
      local.get 4
      i64.load align=1
      i64.store align=1
      local.get 3
      i32.const 150
      i32.add
      local.get 5
      i32.load16_u
      i32.store16
      local.get 3
      i32.const 1
      i32.store offset=104
      local.get 3
      i32.const 2
      i32.store8 offset=145
      i32.const 3
      local.set 4
      local.get 3
      i32.const 3
      i32.store16 offset=6152
      local.get 3
      i32.const 2
      i32.store offset=6188
      local.get 3
      i32.const 0
      i32.store16 offset=6184
      local.get 3
      local.get 3
      i64.load offset=6192 align=1
      i64.store offset=129 align=1
      local.get 3
      local.get 3
      i32.load offset=6176 align=2
      i32.store offset=146 align=2
      local.get 3
      i32.const 6184
      i32.add
      call $<core::result::Result<T,E> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap
      drop
      local.get 3
      i64.const 0
      i64.store offset=6176 align=4
      local.get 1
      local.get 2
      local.get 3
      i32.const 6176
      i32.add
      call $wasi_snapshot_preview1::ImportAlloc::with_arena
      local.get 3
      i32.load offset=6176
      local.set 7
      block ;; label = @1
        local.get 3
        i32.load offset=6180
        local.tee 8
        i32.eqz
        br_if 0 (;@1;)
        local.get 8
        i32.const 12
        i32.mul
        local.set 1
        local.get 3
        i32.const 6192
        i32.add
        i32.const 1
        i32.or
        local.set 9
        local.get 7
        local.set 2
        loop ;; label = @2
          local.get 2
          i32.load
          local.tee 5
          local.get 3
          i32.const 6192
          i32.add
          call $wasi_snapshot_preview1::bindings::wasi::filesystem::types::Descriptor::get_type::wit_import
          local.get 3
          i32.load8_u offset=6192
          i32.const 0
          i32.ne
          local.get 9
          i32.load8_u
          call $<core::result::Result<T,E> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap
          local.set 6
          local.get 3
          i32.const 256
          i32.store16 offset=6232
          local.get 3
          i64.const 0
          i64.store offset=6224
          local.get 3
          local.get 6
          i32.store8 offset=6220
          local.get 3
          local.get 5
          i32.store offset=6216
          local.get 3
          i32.const 0
          i32.store offset=6208
          local.get 3
          i32.const 0
          i32.store offset=6200
          local.get 3
          i32.const 1
          i32.store offset=6192
          block ;; label = @3
            block ;; label = @4
              local.get 4
              i32.const 65535
              i32.and
              local.tee 5
              i32.const 128
              i32.lt_u
              br_if 0 (;@4;)
              local.get 3
              i32.const 48
              i32.store16 offset=6186
              local.get 3
              i32.const 6192
              i32.add
              call $core::ptr::drop_in_place<wasi_snapshot_preview1::descriptors::Descriptor>
              i32.const 1
              local.set 5
              br 1 (;@3;)
            end
            local.get 3
            i32.const 8
            i32.add
            local.get 5
            i32.const 48
            i32.mul
            i32.add
            local.get 3
            i32.const 6192
            i32.add
            i32.const 48
            call $memcpy
            drop
            local.get 3
            local.get 5
            i32.store offset=6188
            local.get 3
            local.get 4
            i32.const 1
            i32.add
            local.tee 4
            i32.store16 offset=6152
            i32.const 0
            local.set 5
          end
          local.get 2
          i32.const 12
          i32.add
          local.set 2
          local.get 3
          local.get 5
          i32.store16 offset=6184
          local.get 3
          i32.const 6184
          i32.add
          call $<core::result::Result<T,E> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap
          drop
          local.get 1
          i32.const -12
          i32.add
          local.tee 1
          br_if 0 (;@2;)
        end
      end
      local.get 3
      i32.const 6168
      i32.add
      local.get 8
      i32.store
      local.get 3
      local.get 7
      i32.store offset=6164
      local.get 0
      local.get 3
      i32.const 8
      i32.add
      i32.const 6168
      call $memcpy
      drop
      local.get 3
      i32.const 6240
      i32.add
      global.set $__stack_pointer
    )
    (func $get_state_ptr (;46;) (type 8) (result i32)
      global.get $internal_state_ptr
    )
    (func $set_state_ptr (;47;) (type 0) (param i32)
      local.get 0
      global.set $internal_state_ptr
    )
    (func $get_allocation_state (;48;) (type 8) (result i32)
      global.get $allocation_state
    )
    (func $set_allocation_state (;49;) (type 0) (param i32)
      local.get 0
      global.set $allocation_state
    )
    (func $compiler_builtins::mem::memcpy (;50;) (type 9) (param i32 i32 i32) (result i32)
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
    (func $memcpy (;51;) (type 9) (param i32 i32 i32) (result i32)
      local.get 0
      local.get 1
      local.get 2
      call $compiler_builtins::mem::memcpy
    )
    (func $allocate_stack (;52;) (type 11)
      global.get $allocation_state
      i32.const 0
      i32.eq
      if ;; label = @1
        i32.const 1
        global.set $allocation_state
        i32.const 0
        i32.const 0
        i32.const 8
        i32.const 65536
        call $wasi_snapshot_preview1::State::new::cabi_realloc
        i32.const 65536
        i32.add
        global.set $__stack_pointer
        i32.const 2
        global.set $allocation_state
      end
    )
    (global $__stack_pointer (;0;) (mut i32) i32.const 0)
    (global $internal_state_ptr (;1;) (mut i32) i32.const 0)
    (global $allocation_state (;2;) (mut i32) i32.const 0)
    (export "cabi_import_realloc" (func $cabi_import_realloc))
    (export "cabi_export_realloc" (func $cabi_export_realloc))
    (export "fd_write" (func $fd_write))
  )
  (core module (;2;)
    (type (;0;) (func (param i32)))
    (type (;1;) (func (param i32 i64 i32)))
    (type (;2;) (func (param i32 i32)))
    (type (;3;) (func (param i32 i32 i32 i32)))
    (type (;4;) (func (param i32 i32 i32 i32) (result i32)))
    (func $indirect-miden:base/tx-kernel@1.0.0-get-inputs (;0;) (type 0) (param i32)
      local.get 0
      i32.const 0
      call_indirect (type 0)
    )
    (func $indirect-miden:base/tx-kernel@1.0.0-get-assets (;1;) (type 0) (param i32)
      local.get 0
      i32.const 1
      call_indirect (type 0)
    )
    (func $indirect-wasi:filesystem/preopens@0.2.0-rc-2023-11-10-get-directories (;2;) (type 0) (param i32)
      local.get 0
      i32.const 2
      call_indirect (type 0)
    )
    (func $#func3<indirect-wasi:filesystem/types@0.2.0-rc-2023-11-10-_method_descriptor.write-via-stream> (@name "indirect-wasi:filesystem/types@0.2.0-rc-2023-11-10-[method]descriptor.write-via-stream") (;3;) (type 1) (param i32 i64 i32)
      local.get 0
      local.get 1
      local.get 2
      i32.const 3
      call_indirect (type 1)
    )
    (func $#func4<indirect-wasi:filesystem/types@0.2.0-rc-2023-11-10-_method_descriptor.append-via-stream> (@name "indirect-wasi:filesystem/types@0.2.0-rc-2023-11-10-[method]descriptor.append-via-stream") (;4;) (type 2) (param i32 i32)
      local.get 0
      local.get 1
      i32.const 4
      call_indirect (type 2)
    )
    (func $#func5<indirect-wasi:filesystem/types@0.2.0-rc-2023-11-10-_method_descriptor.get-type> (@name "indirect-wasi:filesystem/types@0.2.0-rc-2023-11-10-[method]descriptor.get-type") (;5;) (type 2) (param i32 i32)
      local.get 0
      local.get 1
      i32.const 5
      call_indirect (type 2)
    )
    (func $#func6<indirect-wasi:filesystem/types@0.2.0-rc-2023-11-10-_method_descriptor.stat> (@name "indirect-wasi:filesystem/types@0.2.0-rc-2023-11-10-[method]descriptor.stat") (;6;) (type 2) (param i32 i32)
      local.get 0
      local.get 1
      i32.const 6
      call_indirect (type 2)
    )
    (func $indirect-wasi:filesystem/types@0.2.0-rc-2023-11-10-filesystem-error-code (;7;) (type 2) (param i32 i32)
      local.get 0
      local.get 1
      i32.const 7
      call_indirect (type 2)
    )
    (func $#func8<indirect-wasi:io/streams@0.2.0-rc-2023-11-10-_method_output-stream.check-write> (@name "indirect-wasi:io/streams@0.2.0-rc-2023-11-10-[method]output-stream.check-write") (;8;) (type 2) (param i32 i32)
      local.get 0
      local.get 1
      i32.const 8
      call_indirect (type 2)
    )
    (func $#func9<indirect-wasi:io/streams@0.2.0-rc-2023-11-10-_method_output-stream.write> (@name "indirect-wasi:io/streams@0.2.0-rc-2023-11-10-[method]output-stream.write") (;9;) (type 3) (param i32 i32 i32 i32)
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      i32.const 9
      call_indirect (type 3)
    )
    (func $#func10<indirect-wasi:io/streams@0.2.0-rc-2023-11-10-_method_output-stream.blocking-write-and-flush> (@name "indirect-wasi:io/streams@0.2.0-rc-2023-11-10-[method]output-stream.blocking-write-and-flush") (;10;) (type 3) (param i32 i32 i32 i32)
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      i32.const 10
      call_indirect (type 3)
    )
    (func $#func11<indirect-wasi:io/streams@0.2.0-rc-2023-11-10-_method_output-stream.blocking-flush> (@name "indirect-wasi:io/streams@0.2.0-rc-2023-11-10-[method]output-stream.blocking-flush") (;11;) (type 2) (param i32 i32)
      local.get 0
      local.get 1
      i32.const 11
      call_indirect (type 2)
    )
    (func $adapt-wasi_snapshot_preview1-fd_write (;12;) (type 4) (param i32 i32 i32 i32) (result i32)
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      i32.const 12
      call_indirect (type 4)
    )
    (table (;0;) 13 13 funcref)
    (export "0" (func $indirect-miden:base/tx-kernel@1.0.0-get-inputs))
    (export "1" (func $indirect-miden:base/tx-kernel@1.0.0-get-assets))
    (export "2" (func $indirect-wasi:filesystem/preopens@0.2.0-rc-2023-11-10-get-directories))
    (export "3" (func $#func3<indirect-wasi:filesystem/types@0.2.0-rc-2023-11-10-_method_descriptor.write-via-stream>))
    (export "4" (func $#func4<indirect-wasi:filesystem/types@0.2.0-rc-2023-11-10-_method_descriptor.append-via-stream>))
    (export "5" (func $#func5<indirect-wasi:filesystem/types@0.2.0-rc-2023-11-10-_method_descriptor.get-type>))
    (export "6" (func $#func6<indirect-wasi:filesystem/types@0.2.0-rc-2023-11-10-_method_descriptor.stat>))
    (export "7" (func $indirect-wasi:filesystem/types@0.2.0-rc-2023-11-10-filesystem-error-code))
    (export "8" (func $#func8<indirect-wasi:io/streams@0.2.0-rc-2023-11-10-_method_output-stream.check-write>))
    (export "9" (func $#func9<indirect-wasi:io/streams@0.2.0-rc-2023-11-10-_method_output-stream.write>))
    (export "10" (func $#func10<indirect-wasi:io/streams@0.2.0-rc-2023-11-10-_method_output-stream.blocking-write-and-flush>))
    (export "11" (func $#func11<indirect-wasi:io/streams@0.2.0-rc-2023-11-10-_method_output-stream.blocking-flush>))
    (export "12" (func $adapt-wasi_snapshot_preview1-fd_write))
    (export "$imports" (table 0))
  )
  (core module (;3;)
    (type (;0;) (func (param i32)))
    (type (;1;) (func (param i32 i64 i32)))
    (type (;2;) (func (param i32 i32)))
    (type (;3;) (func (param i32 i32 i32 i32)))
    (type (;4;) (func (param i32 i32 i32 i32) (result i32)))
    (import "" "0" (func (;0;) (type 0)))
    (import "" "1" (func (;1;) (type 0)))
    (import "" "2" (func (;2;) (type 0)))
    (import "" "3" (func (;3;) (type 1)))
    (import "" "4" (func (;4;) (type 2)))
    (import "" "5" (func (;5;) (type 2)))
    (import "" "6" (func (;6;) (type 2)))
    (import "" "7" (func (;7;) (type 2)))
    (import "" "8" (func (;8;) (type 2)))
    (import "" "9" (func (;9;) (type 3)))
    (import "" "10" (func (;10;) (type 3)))
    (import "" "11" (func (;11;) (type 2)))
    (import "" "12" (func (;12;) (type 4)))
    (import "" "$imports" (table (;0;) 13 13 funcref))
    (elem (;0;) (i32.const 0) func 0 1 2 3 4 5 6 7 8 9 10 11 12)
  )
  (core instance (;0;) (instantiate 2))
  (alias export 1 "get-id" (func (;0;)))
  (core func (;0;) (canon lower (func 0)))
  (alias core export 0 "0" (core func (;1;)))
  (alias core export 0 "1" (core func (;2;)))
  (core instance (;1;)
    (export "get-id" (func 0))
    (export "get-inputs" (func 1))
    (export "get-assets" (func 2))
  )
  (alias export 2 "receive-asset" (func (;1;)))
  (core func (;3;) (canon lower (func 1)))
  (core instance (;2;)
    (export "receive-asset" (func 3))
  )
  (alias core export 0 "12" (core func (;4;)))
  (core instance (;3;)
    (export "fd_write" (func 4))
  )
  (core instance (;4;) (instantiate 0
      (with "miden:base/tx-kernel@1.0.0" (instance 1))
      (with "miden:basic-wallet/basic-wallet@1.0.0" (instance 2))
      (with "wasi_snapshot_preview1" (instance 3))
    )
  )
  (alias core export 4 "memory" (core memory (;0;)))
  (alias core export 4 "cabi_realloc" (core func (;5;)))
  (alias core export 4 "cabi_realloc" (core func (;6;)))
  (core instance (;5;)
    (export "cabi_realloc" (func 6))
  )
  (core instance (;6;)
    (export "memory" (memory 0))
  )
  (alias core export 0 "2" (core func (;7;)))
  (core instance (;7;)
    (export "get-directories" (func 7))
  )
  (alias export 9 "descriptor" (type (;23;)))
  (core func (;8;) (canon resource.drop 23))
  (alias core export 0 "3" (core func (;9;)))
  (alias core export 0 "4" (core func (;10;)))
  (alias core export 0 "5" (core func (;11;)))
  (alias core export 0 "6" (core func (;12;)))
  (alias core export 0 "7" (core func (;13;)))
  (core instance (;8;)
    (export "[resource-drop]descriptor" (func 8))
    (export "[method]descriptor.write-via-stream" (func 9))
    (export "[method]descriptor.append-via-stream" (func 10))
    (export "[method]descriptor.get-type" (func 11))
    (export "[method]descriptor.stat" (func 12))
    (export "filesystem-error-code" (func 13))
  )
  (alias export 3 "error" (type (;24;)))
  (core func (;14;) (canon resource.drop 24))
  (core instance (;9;)
    (export "[resource-drop]error" (func 14))
  )
  (alias export 4 "input-stream" (type (;25;)))
  (core func (;15;) (canon resource.drop 25))
  (alias export 4 "output-stream" (type (;26;)))
  (core func (;16;) (canon resource.drop 26))
  (alias core export 0 "8" (core func (;17;)))
  (alias core export 0 "9" (core func (;18;)))
  (alias core export 0 "10" (core func (;19;)))
  (alias core export 0 "11" (core func (;20;)))
  (core instance (;10;)
    (export "[resource-drop]input-stream" (func 15))
    (export "[resource-drop]output-stream" (func 16))
    (export "[method]output-stream.check-write" (func 17))
    (export "[method]output-stream.write" (func 18))
    (export "[method]output-stream.blocking-write-and-flush" (func 19))
    (export "[method]output-stream.blocking-flush" (func 20))
  )
  (alias export 7 "get-stderr" (func (;2;)))
  (core func (;21;) (canon lower (func 2)))
  (core instance (;11;)
    (export "get-stderr" (func 21))
  )
  (alias export 5 "get-stdin" (func (;3;)))
  (core func (;22;) (canon lower (func 3)))
  (core instance (;12;)
    (export "get-stdin" (func 22))
  )
  (alias export 6 "get-stdout" (func (;4;)))
  (core func (;23;) (canon lower (func 4)))
  (core instance (;13;)
    (export "get-stdout" (func 23))
  )
  (core instance (;14;) (instantiate 1
      (with "__main_module__" (instance 5))
      (with "env" (instance 6))
      (with "wasi:filesystem/preopens@0.2.0-rc-2023-11-10" (instance 7))
      (with "wasi:filesystem/types@0.2.0-rc-2023-11-10" (instance 8))
      (with "wasi:io/error@0.2.0-rc-2023-11-10" (instance 9))
      (with "wasi:io/streams@0.2.0-rc-2023-11-10" (instance 10))
      (with "wasi:cli/stderr@0.2.0-rc-2023-12-05" (instance 11))
      (with "wasi:cli/stdin@0.2.0-rc-2023-12-05" (instance 12))
      (with "wasi:cli/stdout@0.2.0-rc-2023-12-05" (instance 13))
    )
  )
  (alias core export 14 "cabi_export_realloc" (core func (;24;)))
  (alias core export 14 "cabi_import_realloc" (core func (;25;)))
  (alias core export 0 "$imports" (core table (;0;)))
  (alias export 1 "get-inputs" (func (;5;)))
  (core func (;26;) (canon lower (func 5) (memory 0)))
  (alias export 1 "get-assets" (func (;6;)))
  (core func (;27;) (canon lower (func 6) (memory 0) (realloc 5)))
  (alias export 10 "get-directories" (func (;7;)))
  (core func (;28;) (canon lower (func 7) (memory 0) (realloc 25) string-encoding=utf8))
  (alias export 9 "[method]descriptor.write-via-stream" (func (;8;)))
  (core func (;29;) (canon lower (func 8) (memory 0)))
  (alias export 9 "[method]descriptor.append-via-stream" (func (;9;)))
  (core func (;30;) (canon lower (func 9) (memory 0)))
  (alias export 9 "[method]descriptor.get-type" (func (;10;)))
  (core func (;31;) (canon lower (func 10) (memory 0)))
  (alias export 9 "[method]descriptor.stat" (func (;11;)))
  (core func (;32;) (canon lower (func 11) (memory 0)))
  (alias export 9 "filesystem-error-code" (func (;12;)))
  (core func (;33;) (canon lower (func 12) (memory 0)))
  (alias export 4 "[method]output-stream.check-write" (func (;13;)))
  (core func (;34;) (canon lower (func 13) (memory 0)))
  (alias export 4 "[method]output-stream.write" (func (;14;)))
  (core func (;35;) (canon lower (func 14) (memory 0)))
  (alias export 4 "[method]output-stream.blocking-write-and-flush" (func (;15;)))
  (core func (;36;) (canon lower (func 15) (memory 0)))
  (alias export 4 "[method]output-stream.blocking-flush" (func (;16;)))
  (core func (;37;) (canon lower (func 16) (memory 0)))
  (alias core export 14 "fd_write" (core func (;38;)))
  (core instance (;15;)
    (export "$imports" (table 0))
    (export "0" (func 26))
    (export "1" (func 27))
    (export "2" (func 28))
    (export "3" (func 29))
    (export "4" (func 30))
    (export "5" (func 31))
    (export "6" (func 32))
    (export "7" (func 33))
    (export "8" (func 34))
    (export "9" (func 35))
    (export "10" (func 36))
    (export "11" (func 37))
    (export "12" (func 38))
  )
  (core instance (;16;) (instantiate 3
      (with "" (instance 15))
    )
  )
  (type (;27;) (func))
  (alias core export 4 "miden:base/note@1.0.0#note-script" (core func (;39;)))
  (func (;17;) (type 27) (canon lift (core func 39)))
  (component (;0;)
    (type (;0;) (func))
    (import "import-func-note-script" (func (;0;) (type 0)))
    (type (;1;) (func))
    (export (;1;) "note-script" (func 0) (func (type 1)))
  )
  (instance (;11;) (instantiate 0
      (with "import-func-note-script" (func 17))
    )
  )
  (export (;12;) "miden:base/note@1.0.0" (instance 11))
)