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
  (import (interface "miden:base/types@1.0.0") (instance (;0;) (type 0)))
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
  (import (interface "miden:base/tx-kernel@1.0.0") (instance (;1;) (type 4)))
  (alias export 0 "asset" (type (;5;)))
  (type (;6;)
    (instance
      (alias outer 1 5 (type (;0;)))
      (export (;1;) "asset" (type (eq 0)))
      (type (;2;) (func (param "asset" 1)))
      (export (;0;) "receive-asset" (func (type 2)))
    )
  )
  (import (interface "miden:basic-wallet/basic-wallet@1.0.0") (instance (;2;) (type 6)))
  (alias export 0 "asset" (type (;7;)))
  (type (;8;)
    (instance
      (alias outer 1 7 (type (;0;)))
      (export (;1;) "asset" (type (eq 0)))
      (type (;2;) (func (param "asset" 1) (result bool)))
      (export (;0;) "some-asset-check" (func (type 2)))
    )
  )
  (import (interface "miden:basic-wallet-helpers/check-helpers@1.0.0") (instance (;3;) (type 8)))
  (type (;9;)
    (instance
      (export (;0;) "error" (type (sub resource)))
    )
  )
  (import (interface "wasi:io/error@0.2.0-rc-2023-11-10") (instance (;4;) (type 9)))
  (alias export 4 "error" (type (;10;)))
  (type (;11;)
    (instance
      (export (;0;) "output-stream" (type (sub resource)))
      (alias outer 1 10 (type (;1;)))
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
  (import (interface "wasi:io/streams@0.2.0-rc-2023-11-10") (instance (;5;) (type 11)))
  (alias export 5 "output-stream" (type (;12;)))
  (alias export 5 "error" (type (;13;)))
  (type (;14;)
    (instance
      (export (;0;) "descriptor" (type (sub resource)))
      (type (;1;) u64)
      (export (;2;) "filesize" (type (eq 1)))
      (alias outer 1 12 (type (;3;)))
      (export (;4;) "output-stream" (type (eq 3)))
      (type (;5;) (enum "access" "would-block" "already" "bad-descriptor" "busy" "deadlock" "quota" "exist" "file-too-large" "illegal-byte-sequence" "in-progress" "interrupted" "invalid" "io" "is-directory" "loop" "too-many-links" "message-size" "name-too-long" "no-device" "no-entry" "no-lock" "insufficient-memory" "insufficient-space" "not-directory" "not-empty" "not-recoverable" "unsupported" "no-tty" "no-such-device" "overflow" "not-permitted" "pipe" "read-only" "invalid-seek" "text-file-busy" "cross-device"))
      (export (;6;) "error-code" (type (eq 5)))
      (type (;7;) (enum "unknown" "block-device" "character-device" "directory" "fifo" "symbolic-link" "regular-file" "socket"))
      (export (;8;) "descriptor-type" (type (eq 7)))
      (type (;9;) (flags "symlink-follow"))
      (export (;10;) "path-flags" (type (eq 9)))
      (type (;11;) (flags "create" "directory" "exclusive" "truncate"))
      (export (;12;) "open-flags" (type (eq 11)))
      (type (;13;) (flags "read" "write" "file-integrity-sync" "data-integrity-sync" "requested-write-sync" "mutate-directory"))
      (export (;14;) "descriptor-flags" (type (eq 13)))
      (alias outer 1 13 (type (;15;)))
      (export (;16;) "error" (type (eq 15)))
      (export (;17;) "directory-entry-stream" (type (sub resource)))
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
      (type (;25;) (own 0))
      (type (;26;) (result 25 (error 6)))
      (type (;27;) (func (param "self" 18) (param "path-flags" 10) (param "path" string) (param "open-flags" 12) (param "flags" 14) (result 26)))
      (export (;3;) "[method]descriptor.open-at" (func (type 27)))
      (type (;28;) (borrow 16))
      (type (;29;) (option 6))
      (type (;30;) (func (param "err" 28) (result 29)))
      (export (;4;) "filesystem-error-code" (func (type 30)))
    )
  )
  (import (interface "wasi:filesystem/types@0.2.0-rc-2023-11-10") (instance (;6;) (type 14)))
  (alias export 6 "descriptor" (type (;15;)))
  (type (;16;)
    (instance
      (alias outer 1 15 (type (;0;)))
      (export (;1;) "descriptor" (type (eq 0)))
      (type (;2;) (own 1))
      (type (;3;) (tuple 2 string))
      (type (;4;) (list 3))
      (type (;5;) (func (result 4)))
      (export (;0;) "get-directories" (func (type 5)))
    )
  )
  (import (interface "wasi:filesystem/preopens@0.2.0-rc-2023-11-10") (instance (;7;) (type 16)))
  (type (;17;)
    (instance
      (export (;0;) "tcp-socket" (type (sub resource)))
    )
  )
  (import (interface "wasi:sockets/tcp@0.2.0-rc-2023-11-10") (instance (;8;) (type 17)))
  (type (;18;)
    (instance
      (type (;0;) (tuple string string))
      (type (;1;) (list 0))
      (type (;2;) (func (result 1)))
      (export (;0;) "get-environment" (func (type 2)))
    )
  )
  (import (interface "wasi:cli/environment@0.2.0-rc-2023-11-10") (instance (;9;) (type 18)))
  (type (;19;)
    (instance
      (type (;0;) (result))
      (type (;1;) (func (param "status" 0)))
      (export (;0;) "exit" (func (type 1)))
    )
  )
  (import (interface "wasi:cli/exit@0.2.0-rc-2023-11-10") (instance (;10;) (type 19)))
  (alias export 5 "input-stream" (type (;20;)))
  (type (;21;)
    (instance
      (alias outer 1 20 (type (;0;)))
      (export (;1;) "input-stream" (type (eq 0)))
      (type (;2;) (own 1))
      (type (;3;) (func (result 2)))
      (export (;0;) "get-stdin" (func (type 3)))
    )
  )
  (import (interface "wasi:cli/stdin@0.2.0-rc-2023-11-10") (instance (;11;) (type 21)))
  (alias export 5 "output-stream" (type (;22;)))
  (type (;23;)
    (instance
      (alias outer 1 22 (type (;0;)))
      (export (;1;) "output-stream" (type (eq 0)))
      (type (;2;) (own 1))
      (type (;3;) (func (result 2)))
      (export (;0;) "get-stdout" (func (type 3)))
    )
  )
  (import (interface "wasi:cli/stdout@0.2.0-rc-2023-11-10") (instance (;12;) (type 23)))
  (alias export 5 "output-stream" (type (;24;)))
  (type (;25;)
    (instance
      (alias outer 1 24 (type (;0;)))
      (export (;1;) "output-stream" (type (eq 0)))
      (type (;2;) (own 1))
      (type (;3;) (func (result 2)))
      (export (;0;) "get-stderr" (func (type 3)))
    )
  )
  (import (interface "wasi:cli/stderr@0.2.0-rc-2023-11-10") (instance (;13;) (type 25)))
  (type (;26;)
    (instance
      (export (;0;) "terminal-input" (type (sub resource)))
    )
  )
  (import (interface "wasi:cli/terminal-input@0.2.0-rc-2023-11-10") (instance (;14;) (type 26)))
  (type (;27;)
    (instance
      (export (;0;) "terminal-output" (type (sub resource)))
    )
  )
  (import (interface "wasi:cli/terminal-output@0.2.0-rc-2023-11-10") (instance (;15;) (type 27)))
  (alias export 14 "terminal-input" (type (;28;)))
  (type (;29;)
    (instance
      (alias outer 1 28 (type (;0;)))
      (export (;1;) "terminal-input" (type (eq 0)))
      (type (;2;) (own 1))
      (type (;3;) (option 2))
      (type (;4;) (func (result 3)))
      (export (;0;) "get-terminal-stdin" (func (type 4)))
    )
  )
  (import (interface "wasi:cli/terminal-stdin@0.2.0-rc-2023-11-10") (instance (;16;) (type 29)))
  (alias export 15 "terminal-output" (type (;30;)))
  (type (;31;)
    (instance
      (alias outer 1 30 (type (;0;)))
      (export (;1;) "terminal-output" (type (eq 0)))
      (type (;2;) (own 1))
      (type (;3;) (option 2))
      (type (;4;) (func (result 3)))
      (export (;0;) "get-terminal-stdout" (func (type 4)))
    )
  )
  (import (interface "wasi:cli/terminal-stdout@0.2.0-rc-2023-11-10") (instance (;17;) (type 31)))
  (alias export 15 "terminal-output" (type (;32;)))
  (type (;33;)
    (instance
      (alias outer 1 32 (type (;0;)))
      (export (;1;) "terminal-output" (type (eq 0)))
      (type (;2;) (own 1))
      (type (;3;) (option 2))
      (type (;4;) (func (result 3)))
      (export (;0;) "get-terminal-stderr" (func (type 4)))
    )
  )
  (import (interface "wasi:cli/terminal-stderr@0.2.0-rc-2023-11-10") (instance (;18;) (type 33)))
  (core module (;0;)
    (type $.rodata (;0;) (func))
    (type $.data (;1;) (func (param i32)))
    (type (;2;) (func (param i32 i32)))
    (type (;3;) (func (param i32) (result i32)))
    (type (;4;) (func (param i32 i32 i32)))
    (type (;5;) (func (param i32 i32 i32) (result i32)))
    (type (;6;) (func (param i32 i32) (result i32)))
    (type (;7;) (func (result i64)))
    (type (;8;) (func (param i32 i64 i64 i64 i64) (result i32)))
    (type (;9;) (func (param i32 i64 i64 i64 i64)))
    (type (;10;) (func (param i32 i32 i32 i32) (result i32)))
    (type (;11;) (func (param i32 i32 i32 i32 i32 i64 i64 i32 i32) (result i32)))
    (type (;12;) (func (param i32 i32 i32 i32)))
    (type (;13;) (func (result i32)))
    (type (;14;) (func (param i32 i32 i32 i32 i32)))
    (type (;15;) (func (param i32 i32 i32 i32 i32) (result i32)))
    (type (;16;) (func (param i32 i32 i32 i32 i32 i32 i64 i64 i32)))
    (type (;17;) (func (param i32 i32 i32 i32 i32 i32) (result i32)))
    (type (;18;) (func (param i32 i32 i32 i32 i32 i32 i32)))
    (type (;19;) (func (param i32 i32 i32 i32 i32 i32 i32) (result i32)))
    (type (;20;) (func (param i64 i32 i32) (result i32)))
    (import "miden:base/tx-kernel@1.0.0" "get-inputs" (func $basic_wallet_p2id_note::bindings::miden::base::tx_kernel::get_inputs::wit_import (;0;) (type $.data)))
    (import "miden:base/tx-kernel@1.0.0" "get-id" (func $basic_wallet_p2id_note::bindings::miden::base::tx_kernel::get_id::wit_import (;1;) (type 7)))
    (import "miden:base/tx-kernel@1.0.0" "get-assets" (func $basic_wallet_p2id_note::bindings::miden::base::tx_kernel::get_assets::wit_import (;2;) (type $.data)))
    (import "miden:basic-wallet-helpers/check-helpers@1.0.0" "some-asset-check" (func $basic_wallet_p2id_note::bindings::miden::basic_wallet_helpers::check_helpers::some_asset_check::wit_import (;3;) (type 8)))
    (import "miden:basic-wallet/basic-wallet@1.0.0" "receive-asset" (func $basic_wallet_p2id_note::bindings::miden::basic_wallet::basic_wallet::receive_asset::wit_import (;4;) (type 9)))
    (import "wasi_snapshot_preview1" "fd_write" (func $wasi::lib_generated::wasi_snapshot_preview1::fd_write (;5;) (type 10)))
    (import "wasi_snapshot_preview1" "path_open" (func $wasi::lib_generated::wasi_snapshot_preview1::path_open (;6;) (type 11)))
    (import "wasi_snapshot_preview1" "environ_get" (func $__imported_wasi_snapshot_preview1_environ_get (;7;) (type 6)))
    (import "wasi_snapshot_preview1" "environ_sizes_get" (func $__imported_wasi_snapshot_preview1_environ_sizes_get (;8;) (type 6)))
    (import "wasi_snapshot_preview1" "fd_close" (func $__imported_wasi_snapshot_preview1_fd_close (;9;) (type 3)))
    (import "wasi_snapshot_preview1" "fd_prestat_get" (func $__imported_wasi_snapshot_preview1_fd_prestat_get (;10;) (type 6)))
    (import "wasi_snapshot_preview1" "fd_prestat_dir_name" (func $__imported_wasi_snapshot_preview1_fd_prestat_dir_name (;11;) (type 5)))
    (import "wasi_snapshot_preview1" "proc_exit" (func $__imported_wasi_snapshot_preview1_proc_exit (;12;) (type $.data)))
    (func $__wasm_call_ctors (;13;) (type $.rodata))
    (func $_start (;14;) (type $.rodata)
      (local i32)
      block ;; label = @1
        block ;; label = @2
          i32.const 0
          i32.load offset=1056304
          br_if 0 (;@2;)
          i32.const 0
          i32.const 1
          i32.store offset=1056304
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
    (func $std::sys_common::backtrace::__rust_begin_short_backtrace (;15;) (type $.data) (param i32)
      local.get 0
      call_indirect (type $.rodata)
    )
    (func $std::rt::lang_start::{{closure}} (;16;) (type 3) (param i32) (result i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      local.get 0
      i32.load
      call $std::sys_common::backtrace::__rust_begin_short_backtrace
      local.get 1
      i32.const 0
      i32.store8 offset=15
      local.get 1
      i32.const 15
      i32.add
      call $std::sys::wasi::process::ExitCode::as_i32
      local.set 0
      local.get 1
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $<&T as core::fmt::Debug>::fmt (;17;) (type 6) (param i32 i32) (result i32)
      local.get 0
      i32.load
      local.set 0
      block ;; label = @1
        local.get 1
        call $core::fmt::Formatter::debug_lower_hex
        br_if 0 (;@1;)
        block ;; label = @2
          local.get 1
          call $core::fmt::Formatter::debug_upper_hex
          br_if 0 (;@2;)
          local.get 0
          local.get 1
          call $core::fmt::num::imp::<impl core::fmt::Display for u64>::fmt
          return
        end
        local.get 0
        local.get 1
        call $core::fmt::num::<impl core::fmt::UpperHex for i64>::fmt
        return
      end
      local.get 0
      local.get 1
      call $core::fmt::num::<impl core::fmt::LowerHex for i64>::fmt
    )
    (func $core::ops::function::FnOnce::call_once{{vtable.shim}} (;18;) (type 3) (param i32) (result i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      local.get 0
      i32.load
      call $std::sys_common::backtrace::__rust_begin_short_backtrace
      local.get 1
      i32.const 0
      i32.store8 offset=15
      local.get 1
      i32.const 15
      i32.add
      call $std::sys::wasi::process::ExitCode::as_i32
      local.set 0
      local.get 1
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $core::ptr::drop_in_place<&u64> (;19;) (type $.data) (param i32))
    (func $core::panicking::assert_failed (;20;) (type 4) (param i32 i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      local.get 1
      i32.store offset=12
      local.get 3
      local.get 0
      i32.store offset=8
      i32.const 0
      local.get 3
      i32.const 8
      i32.add
      i32.const 1048600
      local.get 3
      i32.const 12
      i32.add
      i32.const 1048600
      local.get 2
      i32.const 1048628
      call $core::panicking::assert_failed_inner
      unreachable
    )
    (func $alloc::raw_vec::finish_grow (;21;) (type 12) (param i32 i32 i32 i32)
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
                    i32.load8_u offset=1056309
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
                i32.load8_u offset=1056309
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
    (func $alloc::raw_vec::RawVec<T,A>::reserve_for_push (;22;) (type 2) (param i32 i32)
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
    (func $basic_wallet_p2id_note::main (;23;) (type $.rodata)
      (local i32 i64 i64 i32 i32 i32 i32 i32 i32 i64 i64 i64 i32)
      global.get $__stack_pointer
      i32.const 160
      i32.sub
      local.tee 0
      global.set $__stack_pointer
      local.get 0
      i32.const 24
      i32.add
      call $basic_wallet_p2id_note::bindings::miden::base::tx_kernel::get_inputs::wit_import
      local.get 0
      local.get 0
      i64.load offset=24
      local.tee 1
      i64.store offset=8
      local.get 0
      call $basic_wallet_p2id_note::bindings::miden::base::tx_kernel::get_id::wit_import
      local.tee 2
      i64.store offset=16
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 2
              local.get 1
              i64.ne
              br_if 0 (;@4;)
              local.get 0
              i32.const 152
              i32.add
              call $basic_wallet_p2id_note::bindings::miden::base::tx_kernel::get_assets::wit_import
              local.get 0
              i32.load offset=152
              local.set 3
              block ;; label = @5
                block ;; label = @6
                  local.get 0
                  i32.const 156
                  i32.add
                  i32.load
                  local.tee 4
                  i32.eqz
                  br_if 0 (;@6;)
                  local.get 4
                  i32.const 53687091
                  i32.gt_u
                  br_if 3 (;@3;)
                  local.get 4
                  i32.const 40
                  i32.mul
                  local.tee 5
                  i32.const -1
                  i32.le_s
                  br_if 3 (;@3;)
                  i32.const 8
                  local.set 6
                  local.get 5
                  i32.eqz
                  br_if 1 (;@5;)
                  i32.const 0
                  i32.load8_u offset=1056309
                  drop
                  local.get 5
                  i32.const 8
                  call $__rust_alloc
                  local.tee 6
                  br_if 1 (;@5;)
                  i32.const 8
                  local.get 5
                  call $alloc::alloc::handle_alloc_error
                  unreachable
                end
                local.get 3
                i32.const 0
                i32.const 8
                call $wit_bindgen::rt::dealloc
                br 4 (;@1;)
              end
              i32.const 0
              local.set 7
              local.get 0
              i32.const 0
              i32.store offset=32
              local.get 0
              local.get 4
              i32.store offset=28
              local.get 0
              local.get 6
              i32.store offset=24
              local.get 3
              local.set 8
              loop ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    local.get 8
                    i32.load8_u
                    br_if 0 (;@7;)
                    i64.const 0
                    local.set 1
                    br 1 (;@6;)
                  end
                  local.get 8
                  i32.const 32
                  i32.add
                  i64.load
                  local.set 9
                  local.get 8
                  i32.const 24
                  i32.add
                  i64.load
                  local.set 10
                  i64.const 1
                  local.set 1
                end
                local.get 8
                i32.const 8
                i32.add
                i64.load
                local.set 2
                local.get 8
                i32.const 16
                i32.add
                i64.load
                local.set 11
                block ;; label = @6
                  local.get 7
                  local.get 0
                  i32.load offset=28
                  i32.ne
                  br_if 0 (;@6;)
                  local.get 0
                  i32.const 24
                  i32.add
                  local.get 7
                  call $alloc::raw_vec::RawVec<T,A>::reserve_for_push
                  local.get 0
                  i32.load offset=24
                  local.set 6
                  local.get 0
                  i32.load offset=32
                  local.set 7
                end
                local.get 6
                local.get 7
                i32.const 40
                i32.mul
                i32.add
                local.tee 12
                local.get 9
                i64.store offset=32
                local.get 12
                local.get 10
                i64.store offset=24
                local.get 12
                local.get 11
                i64.store offset=16
                local.get 12
                local.get 2
                i64.store offset=8
                local.get 12
                local.get 1
                i64.store
                local.get 0
                local.get 7
                i32.const 1
                i32.add
                local.tee 7
                i32.store offset=32
                local.get 8
                i32.const 40
                i32.add
                local.set 8
                local.get 4
                i32.const -1
                i32.add
                local.tee 4
                br_if 0 (;@5;)
              end
              local.get 0
              i32.load offset=28
              local.set 6
              local.get 0
              i32.load offset=24
              local.set 4
              local.get 3
              local.get 5
              i32.const 8
              call $wit_bindgen::rt::dealloc
              local.get 7
              i32.eqz
              br_if 2 (;@2;)
              local.get 4
              local.get 7
              i32.const 40
              i32.mul
              i32.add
              local.set 12
              local.get 4
              local.set 8
              loop ;; label = @5
                local.get 8
                i64.load
                local.tee 1
                i64.const 2
                i64.eq
                br_if 3 (;@2;)
                block ;; label = @6
                  local.get 1
                  i64.const 0
                  i64.ne
                  local.tee 7
                  local.get 8
                  i64.load offset=8
                  local.tee 1
                  local.get 8
                  i64.load offset=16
                  local.tee 2
                  local.get 8
                  i64.load offset=24
                  i64.const 0
                  local.get 7
                  select
                  local.tee 11
                  local.get 8
                  i64.load offset=32
                  i64.const 0
                  local.get 7
                  select
                  local.tee 9
                  call $basic_wallet_p2id_note::bindings::miden::basic_wallet_helpers::check_helpers::some_asset_check::wit_import
                  call $wit_bindgen::rt::bool_lift
                  i32.eqz
                  br_if 0 (;@6;)
                  local.get 7
                  local.get 1
                  local.get 2
                  local.get 11
                  local.get 9
                  call $basic_wallet_p2id_note::bindings::miden::basic_wallet::basic_wallet::receive_asset::wit_import
                end
                local.get 8
                i32.const 40
                i32.add
                local.tee 8
                local.get 12
                i32.ne
                br_if 0 (;@5;)
                br 3 (;@2;)
              end
            end
            local.get 0
            i32.const 0
            i32.store offset=24
            local.get 0
            i32.const 16
            i32.add
            local.get 0
            i32.const 8
            i32.add
            local.get 0
            i32.const 24
            i32.add
            call $core::panicking::assert_failed
            unreachable
          end
          call $alloc::raw_vec::capacity_overflow
          unreachable
        end
        local.get 6
        i32.eqz
        br_if 0 (;@1;)
        local.get 4
        local.get 6
        i32.const 40
        i32.mul
        i32.const 8
        call $__rust_dealloc
      end
      local.get 0
      i32.const 160
      i32.add
      global.set $__stack_pointer
    )
    (func $__main_void (;24;) (type 13) (result i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 0
      global.set $__stack_pointer
      local.get 0
      i32.const 1
      i32.store offset=12
      local.get 0
      i32.const 12
      i32.add
      i32.const 1048576
      i32.const 0
      i32.const 0
      i32.const 0
      call $std::rt::lang_start_internal
      local.set 1
      local.get 0
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 1
    )
    (func $__rust_alloc (;25;) (type 6) (param i32 i32) (result i32)
      (local i32)
      local.get 0
      local.get 1
      call $__rdl_alloc
      local.set 2
      local.get 2
      return
    )
    (func $__rust_dealloc (;26;) (type 4) (param i32 i32 i32)
      local.get 0
      local.get 1
      local.get 2
      call $__rdl_dealloc
      return
    )
    (func $__rust_realloc (;27;) (type 10) (param i32 i32 i32 i32) (result i32)
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
    (func $__rust_alloc_error_handler (;28;) (type 2) (param i32 i32)
      local.get 0
      local.get 1
      call $__rg_oom
      return
    )
    (func $cabi_realloc (;29;) (type 10) (param i32 i32 i32 i32) (result i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 1
            br_if 0 (;@3;)
            local.get 3
            i32.eqz
            br_if 2 (;@1;)
            i32.const 0
            i32.load8_u offset=1056309
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
    (func $wit_bindgen::rt::dealloc (;30;) (type 4) (param i32 i32 i32)
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
    (func $wit_bindgen::rt::bool_lift (;31;) (type 3) (param i32) (result i32)
      local.get 0
      i32.const 255
      i32.and
      i32.const 0
      i32.ne
    )
    (func $<T as core::any::Any>::type_id (;32;) (type 2) (param i32 i32)
      local.get 0
      i64.const -3751304911407043677
      i64.store offset=8
      local.get 0
      i64.const 118126004786499436
      i64.store
    )
    (func $<T as core::any::Any>::type_id (;33;) (type 2) (param i32 i32)
      local.get 0
      i64.const -1151673474265811458
      i64.store offset=8
      local.get 0
      i64.const -6622677684352136008
      i64.store
    )
    (func $<T as core::any::Any>::type_id (;34;) (type 2) (param i32 i32)
      local.get 0
      i64.const -163230743173927068
      i64.store offset=8
      local.get 0
      i64.const -4493808902380553279
      i64.store
    )
    (func $<&T as core::fmt::Debug>::fmt (;35;) (type 6) (param i32 i32) (result i32)
      local.get 0
      i32.load
      local.get 0
      i32.load offset=4
      local.get 1
      call $<core::ffi::c_str::CStr as core::fmt::Debug>::fmt
    )
    (func $<&T as core::fmt::Debug>::fmt (;36;) (type 6) (param i32 i32) (result i32)
      local.get 0
      i32.load
      local.set 0
      block ;; label = @1
        local.get 1
        call $core::fmt::Formatter::debug_lower_hex
        br_if 0 (;@1;)
        block ;; label = @2
          local.get 1
          call $core::fmt::Formatter::debug_upper_hex
          br_if 0 (;@2;)
          local.get 0
          local.get 1
          call $core::fmt::num::imp::<impl core::fmt::Display for i32>::fmt
          return
        end
        local.get 0
        local.get 1
        call $core::fmt::num::<impl core::fmt::UpperHex for i32>::fmt
        return
      end
      local.get 0
      local.get 1
      call $core::fmt::num::<impl core::fmt::LowerHex for i32>::fmt
    )
    (func $<&T as core::fmt::Debug>::fmt (;37;) (type 6) (param i32 i32) (result i32)
      local.get 0
      i32.load
      i32.load
      local.get 1
      call $<alloc::ffi::c_str::NulError as core::fmt::Debug>::fmt
    )
    (func $<&T as core::fmt::Debug>::fmt (;38;) (type 6) (param i32 i32) (result i32)
      local.get 1
      i32.const 1048644
      i32.const 2
      call $core::fmt::Formatter::pad
    )
    (func $<&T as core::fmt::Debug>::fmt (;39;) (type 6) (param i32 i32) (result i32)
      local.get 0
      i32.load
      local.get 1
      call $<bool as core::fmt::Display>::fmt
    )
    (func $<&T as core::fmt::Display>::fmt (;40;) (type 6) (param i32 i32) (result i32)
      local.get 0
      i32.load
      local.get 0
      i32.load offset=4
      local.get 1
      call $<str as core::fmt::Display>::fmt
    )
    (func $<&T as core::fmt::Display>::fmt (;41;) (type 6) (param i32 i32) (result i32)
      local.get 0
      i32.load
      local.get 1
      call $<core::panic::location::Location as core::fmt::Display>::fmt
    )
    (func $core::fmt::Write::write_char (;42;) (type 6) (param i32 i32) (result i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 2
      i32.const 0
      i32.store offset=12
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
              i32.store8 offset=14
              local.get 2
              local.get 1
              i32.const 12
              i32.shr_u
              i32.const 224
              i32.or
              i32.store8 offset=12
              local.get 2
              local.get 1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              i32.const 3
              local.set 1
              br 3 (;@1;)
            end
            local.get 2
            local.get 1
            i32.store8 offset=12
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
          i32.store8 offset=13
          local.get 2
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 192
          i32.or
          i32.store8 offset=12
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
        i32.store8 offset=15
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=14
        local.get 2
        local.get 1
        i32.const 12
        i32.shr_u
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 18
        i32.shr_u
        i32.const 7
        i32.and
        i32.const 240
        i32.or
        i32.store8 offset=12
        i32.const 4
        local.set 1
      end
      local.get 0
      local.get 2
      i32.const 12
      i32.add
      local.get 1
      call $<std::io::Write::write_fmt::Adapter<T> as core::fmt::Write>::write_str
      local.set 1
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 1
    )
    (func $<std::io::Write::write_fmt::Adapter<T> as core::fmt::Write>::write_str (;43;) (type 5) (param i32 i32 i32) (result i32)
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
                    i32.const 1049708
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
          local.get 5
          local.get 2
          i32.const 1049720
          call $core::slice::index::slice_start_index_len_fail
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
          call_indirect (type $.data)
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
    (func $core::fmt::Write::write_char (;44;) (type 6) (param i32 i32) (result i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 2
      i32.const 0
      i32.store offset=12
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
              i32.store8 offset=14
              local.get 2
              local.get 1
              i32.const 12
              i32.shr_u
              i32.const 224
              i32.or
              i32.store8 offset=12
              local.get 2
              local.get 1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              i32.const 3
              local.set 1
              br 3 (;@1;)
            end
            local.get 2
            local.get 1
            i32.store8 offset=12
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
          i32.store8 offset=13
          local.get 2
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 192
          i32.or
          i32.store8 offset=12
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
        i32.store8 offset=15
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=14
        local.get 2
        local.get 1
        i32.const 12
        i32.shr_u
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 18
        i32.shr_u
        i32.const 7
        i32.and
        i32.const 240
        i32.or
        i32.store8 offset=12
        i32.const 4
        local.set 1
      end
      local.get 0
      local.get 2
      i32.const 12
      i32.add
      local.get 1
      call $<std::io::Write::write_fmt::Adapter<T> as core::fmt::Write>::write_str
      local.set 1
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 1
    )
    (func $<std::io::Write::write_fmt::Adapter<T> as core::fmt::Write>::write_str (;45;) (type 5) (param i32 i32 i32) (result i32)
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
        i32.load offset=8
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
              local.get 5
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
                    local.tee 6
                    br_if 1 (;@6;)
                    i32.const 2
                    local.set 2
                    i32.const 1049708
                    local.set 6
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
                  local.tee 6
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
                local.get 6
                i32.lt_u
                br_if 2 (;@3;)
                local.get 1
                local.get 6
                i32.add
                local.set 1
                local.get 2
                local.get 6
                i32.sub
                local.set 2
              end
              local.get 2
              br_if 0 (;@4;)
              br 3 (;@1;)
            end
          end
          local.get 6
          local.get 2
          i32.const 1049720
          call $core::slice::index::slice_start_index_len_fail
          unreachable
        end
        local.get 0
        i32.load offset=4
        local.set 5
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
          local.get 5
          i32.load
          local.tee 4
          local.get 5
          i32.const 4
          i32.add
          i32.load
          local.tee 1
          i32.load
          call_indirect (type $.data)
          block ;; label = @3
            local.get 1
            i32.load offset=4
            local.tee 7
            i32.eqz
            br_if 0 (;@3;)
            local.get 4
            local.get 7
            local.get 1
            i32.load offset=8
            call $__rust_dealloc
          end
          local.get 5
          i32.const 12
          i32.const 4
          call $__rust_dealloc
        end
        local.get 0
        local.get 6
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
    (func $core::fmt::Write::write_char (;46;) (type 6) (param i32 i32) (result i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 2
      i32.const 0
      i32.store offset=12
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
              i32.store8 offset=14
              local.get 2
              local.get 1
              i32.const 12
              i32.shr_u
              i32.const 224
              i32.or
              i32.store8 offset=12
              local.get 2
              local.get 1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              i32.const 3
              local.set 3
              br 3 (;@1;)
            end
            local.get 2
            local.get 1
            i32.store8 offset=12
            i32.const 1
            local.set 3
            br 2 (;@1;)
          end
          local.get 2
          local.get 1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=13
          local.get 2
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 192
          i32.or
          i32.store8 offset=12
          i32.const 2
          local.set 3
          br 1 (;@1;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=15
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=14
        local.get 2
        local.get 1
        i32.const 12
        i32.shr_u
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 18
        i32.shr_u
        i32.const 7
        i32.and
        i32.const 240
        i32.or
        i32.store8 offset=12
        i32.const 4
        local.set 3
      end
      block ;; label = @1
        local.get 0
        i32.load offset=8
        local.tee 1
        i32.load offset=4
        local.get 1
        i32.load offset=8
        local.tee 0
        i32.sub
        local.get 3
        i32.ge_u
        br_if 0 (;@1;)
        local.get 1
        local.get 0
        local.get 3
        call $alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle
        local.get 1
        i32.load offset=8
        local.set 0
      end
      local.get 1
      i32.load
      local.get 0
      i32.add
      local.get 2
      i32.const 12
      i32.add
      local.get 3
      call $memcpy
      drop
      local.get 1
      local.get 0
      local.get 3
      i32.add
      i32.store offset=8
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      i32.const 0
    )
    (func $alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle (;47;) (type 4) (param i32 i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          local.get 1
          local.get 2
          i32.add
          local.tee 2
          local.get 1
          i32.lt_u
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=4
          local.tee 1
          i32.const 1
          i32.shl
          local.tee 4
          local.get 2
          local.get 4
          local.get 2
          i32.gt_u
          select
          local.tee 2
          i32.const 8
          local.get 2
          i32.const 8
          i32.gt_u
          select
          local.tee 2
          i32.const -1
          i32.xor
          i32.const 31
          i32.shr_u
          local.set 4
          block ;; label = @3
            block ;; label = @4
              local.get 1
              i32.eqz
              br_if 0 (;@4;)
              local.get 3
              local.get 1
              i32.store offset=28
              local.get 3
              i32.const 1
              i32.store offset=24
              local.get 3
              local.get 0
              i32.load
              i32.store offset=20
              br 1 (;@3;)
            end
            local.get 3
            i32.const 0
            i32.store offset=24
          end
          local.get 3
          i32.const 8
          i32.add
          local.get 4
          local.get 2
          local.get 3
          i32.const 20
          i32.add
          call $alloc::raw_vec::finish_grow
          local.get 3
          i32.load offset=12
          local.set 1
          block ;; label = @3
            local.get 3
            i32.load offset=8
            br_if 0 (;@3;)
            local.get 0
            local.get 2
            i32.store offset=4
            local.get 0
            local.get 1
            i32.store
            br 2 (;@1;)
          end
          local.get 1
          i32.const -2147483647
          i32.eq
          br_if 1 (;@1;)
          local.get 1
          i32.eqz
          br_if 0 (;@2;)
          local.get 1
          local.get 3
          i32.const 16
          i32.add
          i32.load
          call $alloc::alloc::handle_alloc_error
          unreachable
        end
        call $alloc::raw_vec::capacity_overflow
        unreachable
      end
      local.get 3
      i32.const 32
      i32.add
      global.set $__stack_pointer
    )
    (func $core::fmt::Write::write_fmt (;48;) (type 6) (param i32 i32) (result i32)
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
      i32.const 1048648
      local.get 1
      call $core::fmt::write
      local.set 0
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $core::fmt::Write::write_fmt (;49;) (type 6) (param i32 i32) (result i32)
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
      i32.const 1048696
      local.get 1
      call $core::fmt::write
      local.set 0
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $core::fmt::Write::write_fmt (;50;) (type 6) (param i32 i32) (result i32)
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
      i32.const 1048720
      local.get 1
      call $core::fmt::write
      local.set 0
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $core::fmt::Arguments::new_v1 (;51;) (type 14) (param i32 i32 i32 i32 i32)
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
      i32.const 1048756
      i32.store offset=8
      local.get 5
      i32.const 1048764
      i32.store offset=16
      local.get 5
      i32.const 8
      i32.add
      i32.const 1048840
      call $core::panicking::panic_fmt
      unreachable
    )
    (func $std::panicking::panic_hook_with_disk_dump (;52;) (type 4) (param i32 i32 i32)
      (local i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 160
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      local.get 2
      i32.store offset=52
      local.get 3
      local.get 1
      i32.store offset=48
      i32.const 1
      local.set 4
      block ;; label = @1
        i32.const 0
        i32.load offset=1056392
        i32.const 1
        i32.gt_u
        br_if 0 (;@1;)
        call $std::panic::get_backtrace_style
        local.set 4
      end
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 0
              call $core::panic::panic_info::PanicInfo::location
              local.tee 5
              i32.eqz
              br_if 0 (;@4;)
              local.get 3
              local.get 5
              i32.store offset=56
              local.get 3
              i32.const 40
              i32.add
              local.get 0
              call $core::panic::panic_info::PanicInfo::payload
              local.get 3
              i32.const 24
              i32.add
              local.get 3
              i32.load offset=40
              local.tee 5
              local.get 3
              i32.load offset=44
              i32.load offset=12
              call_indirect (type 2)
              block ;; label = @5
                local.get 5
                i32.eqz
                br_if 0 (;@5;)
                local.get 3
                i64.load offset=24
                i64.const -4493808902380553279
                i64.xor
                local.get 3
                i32.const 24
                i32.add
                i32.const 8
                i32.add
                i64.load
                i64.const -163230743173927068
                i64.xor
                i64.or
                i64.eqz
                br_if 2 (;@3;)
              end
              local.get 3
              i32.const 16
              i32.add
              local.get 0
              call $core::panic::panic_info::PanicInfo::payload
              local.get 3
              local.get 3
              i32.load offset=16
              local.tee 5
              local.get 3
              i32.load offset=20
              i32.load offset=12
              call_indirect (type 2)
              i32.const 12
              local.set 0
              i32.const 1050728
              local.set 6
              local.get 5
              i32.eqz
              br_if 3 (;@1;)
              local.get 3
              i64.load
              i64.const -6622677684352136008
              i64.xor
              local.get 3
              i32.const 8
              i32.add
              i64.load
              i64.const -1151673474265811458
              i64.xor
              i64.or
              i64.const 0
              i64.ne
              br_if 3 (;@1;)
              local.get 5
              i32.const 8
              i32.add
              local.set 0
              br 2 (;@2;)
            end
            i32.const 1048892
            i32.const 43
            i32.const 1050712
            call $core::panicking::panic
            unreachable
          end
          local.get 5
          i32.const 4
          i32.add
          local.set 0
        end
        local.get 0
        i32.load
        local.set 0
        local.get 5
        i32.load
        local.set 6
      end
      local.get 3
      local.get 0
      i32.store offset=64
      local.get 3
      local.get 6
      i32.store offset=60
      local.get 3
      call $std::sys_common::thread_info::current_thread
      local.tee 5
      i32.store offset=68
      i32.const 9
      local.set 0
      i32.const 1050740
      local.set 6
      block ;; label = @1
        local.get 5
        i32.eqz
        br_if 0 (;@1;)
        local.get 5
        i32.const 16
        i32.add
        i32.load
        local.tee 7
        i32.eqz
        br_if 0 (;@1;)
        local.get 5
        i32.const 20
        i32.add
        i32.load
        i32.const -1
        i32.add
        local.set 0
        local.get 7
        local.set 6
      end
      local.get 3
      local.get 0
      i32.store offset=76
      local.get 3
      local.get 6
      i32.store offset=72
      local.get 3
      local.get 3
      i32.const 48
      i32.add
      i32.store offset=92
      local.get 3
      local.get 3
      i32.const 60
      i32.add
      i32.store offset=88
      local.get 3
      local.get 3
      i32.const 56
      i32.add
      i32.store offset=84
      local.get 3
      local.get 3
      i32.const 72
      i32.add
      i32.store offset=80
      block ;; label = @1
        local.get 1
        i32.eqz
        br_if 0 (;@1;)
        local.get 3
        i32.const 0
        i32.store16 offset=136
        local.get 3
        i32.const 1
        i32.store8 offset=138
        local.get 3
        i64.const 0
        i64.store offset=112
        local.get 3
        i64.const 0
        i64.store offset=96
        local.get 3
        i64.const 281479271677953
        i64.store offset=128
        local.get 3
        i32.const 144
        i32.add
        local.get 1
        local.get 2
        local.get 3
        i32.const 96
        i32.add
        call $std::sys::wasi::fs::File::open
        i32.const 4
        local.set 1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 3
              i32.load8_u offset=144
              i32.const 4
              i32.ne
              br_if 0 (;@4;)
              local.get 3
              i32.load offset=148
              local.set 0
              br 1 (;@3;)
            end
            local.get 3
            i32.load offset=148
            local.set 0
            block ;; label = @4
              local.get 3
              i32.load8_u offset=144
              local.tee 1
              i32.const 4
              i32.ne
              br_if 0 (;@4;)
              local.get 1
              local.set 1
              br 1 (;@3;)
            end
            block ;; label = @4
              local.get 1
              i32.const 4
              i32.eq
              br_if 0 (;@4;)
              local.get 1
              local.set 1
              br 2 (;@2;)
            end
            local.get 0
            call $close
            drop
            br 2 (;@1;)
          end
          local.get 3
          local.get 0
          i32.store offset=144
          local.get 3
          i32.const 80
          i32.add
          local.get 3
          i32.const 144
          i32.add
          i32.const 1050752
          i32.const 1
          call $std::panicking::panic_hook_with_disk_dump::{{closure}}
          local.get 3
          i32.load offset=144
          call $close
          drop
          local.get 1
          i32.const 4
          i32.eq
          br_if 1 (;@1;)
        end
        local.get 1
        i32.const 3
        i32.lt_u
        br_if 0 (;@1;)
        local.get 0
        i32.load
        local.tee 2
        local.get 0
        i32.const 4
        i32.add
        i32.load
        local.tee 1
        i32.load
        call_indirect (type $.data)
        block ;; label = @2
          local.get 1
          i32.load offset=4
          local.tee 6
          i32.eqz
          br_if 0 (;@2;)
          local.get 2
          local.get 6
          local.get 1
          i32.load offset=8
          call $__rust_dealloc
        end
        local.get 0
        i32.const 12
        i32.const 4
        call $__rust_dealloc
      end
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              i32.const 0
              i32.load8_u offset=1056311
              br_if 0 (;@4;)
              local.get 3
              i32.const 0
              i32.store offset=144
              br 1 (;@3;)
            end
            i32.const 0
            i32.const 1
            i32.store8 offset=1056311
            block ;; label = @4
              i32.const 0
              i32.load8_u offset=1056412
              br_if 0 (;@4;)
              i32.const 0
              i32.const 1
              i32.store8 offset=1056412
              i32.const 0
              i32.const 0
              i32.store offset=1056416
              local.get 3
              i32.const 0
              i32.store offset=144
              br 1 (;@3;)
            end
            local.get 3
            i32.const 0
            i32.load offset=1056416
            local.tee 1
            i32.store offset=144
            i32.const 0
            i32.const 0
            i32.store offset=1056416
            local.get 1
            br_if 1 (;@2;)
          end
          local.get 3
          i32.const 80
          i32.add
          local.get 3
          i32.const 159
          i32.add
          i32.const 1050832
          local.get 4
          call $std::panicking::panic_hook_with_disk_dump::{{closure}}
          i32.const 0
          local.set 1
          br 1 (;@1;)
        end
        local.get 1
        i32.load8_u offset=8
        local.set 5
        local.get 1
        i32.const 1
        i32.store8 offset=8
        local.get 3
        local.get 5
        i32.store8 offset=158
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                local.get 5
                br_if 0 (;@5;)
                i32.const 0
                i32.load offset=1056376
                i32.const 2147483647
                i32.and
                br_if 1 (;@4;)
                local.get 3
                i32.const 80
                i32.add
                local.get 1
                i32.const 12
                i32.add
                i32.const 1050792
                local.get 4
                call $std::panicking::panic_hook_with_disk_dump::{{closure}}
                local.get 1
                i32.const 9
                i32.add
                local.set 5
                br 2 (;@3;)
              end
              local.get 3
              i64.const 0
              i64.store offset=108 align=4
              local.get 3
              i32.const 1048764
              i32.store offset=104
              local.get 3
              i32.const 1
              i32.store offset=100
              local.get 3
              i32.const 1049976
              i32.store offset=96
              local.get 3
              i32.const 158
              i32.add
              local.get 3
              i32.const 96
              i32.add
              call $core::panicking::assert_failed
              unreachable
            end
            call $std::panicking::panic_count::is_zero_slow_path
            local.set 5
            local.get 3
            i32.const 80
            i32.add
            local.get 1
            i32.const 12
            i32.add
            i32.const 1050792
            local.get 4
            call $std::panicking::panic_hook_with_disk_dump::{{closure}}
            local.get 5
            i32.eqz
            br_if 1 (;@2;)
            local.get 1
            i32.const 9
            i32.add
            local.set 5
          end
          i32.const 0
          i32.load offset=1056376
          i32.const 2147483647
          i32.and
          i32.eqz
          br_if 0 (;@2;)
          call $std::panicking::panic_count::is_zero_slow_path
          br_if 0 (;@2;)
          local.get 5
          i32.const 1
          i32.store8
        end
        local.get 1
        i32.const 0
        i32.store8 offset=8
        i32.const 0
        i32.const 1
        i32.store8 offset=1056311
        block ;; label = @2
          block ;; label = @3
            i32.const 0
            i32.load8_u offset=1056412
            br_if 0 (;@3;)
            i32.const 0
            local.get 1
            i32.store offset=1056416
            i32.const 0
            i32.const 1
            i32.store8 offset=1056412
            br 1 (;@2;)
          end
          i32.const 0
          i32.load offset=1056416
          local.set 5
          i32.const 0
          local.get 1
          i32.store offset=1056416
          local.get 3
          local.get 5
          i32.store offset=96
          local.get 5
          i32.eqz
          br_if 0 (;@2;)
          local.get 5
          local.get 5
          i32.load
          local.tee 1
          i32.const -1
          i32.add
          i32.store
          local.get 1
          i32.const 1
          i32.ne
          br_if 0 (;@2;)
          local.get 3
          i32.const 96
          i32.add
          call $alloc::sync::Arc<T,A>::drop_slow
        end
        i32.const 1
        local.set 1
        local.get 3
        i32.load offset=68
        local.set 5
      end
      block ;; label = @1
        local.get 5
        i32.eqz
        br_if 0 (;@1;)
        local.get 5
        local.get 5
        i32.load
        local.tee 0
        i32.const -1
        i32.add
        i32.store
        local.get 0
        i32.const 1
        i32.ne
        br_if 0 (;@1;)
        local.get 3
        i32.const 68
        i32.add
        call $alloc::sync::Arc<T,A>::drop_slow
      end
      block ;; label = @1
        local.get 1
        i32.const -1
        i32.xor
        local.get 3
        i32.load offset=144
        local.tee 5
        i32.const 0
        i32.ne
        i32.and
        i32.eqz
        br_if 0 (;@1;)
        local.get 5
        local.get 5
        i32.load
        local.tee 1
        i32.const -1
        i32.add
        i32.store
        local.get 1
        i32.const 1
        i32.ne
        br_if 0 (;@1;)
        local.get 3
        i32.const 144
        i32.add
        call $alloc::sync::Arc<T,A>::drop_slow
      end
      local.get 3
      i32.const 160
      i32.add
      global.set $__stack_pointer
    )
    (func $core::ptr::drop_in_place<&mut std::io::Write::write_fmt::Adapter<alloc::vec::Vec<u8>>> (;53;) (type $.data) (param i32))
    (func $std::panicking::panic_count::is_zero_slow_path (;54;) (type 13) (result i32)
      i32.const 0
      i32.load offset=1056392
      i32.eqz
    )
    (func $core::ptr::drop_in_place<<alloc::boxed::Box<dyn core::error::Error+core::marker::Send+core::marker::Sync> as core::convert::From<alloc::string::String>>::from::StringError> (;55;) (type $.data) (param i32)
      (local i32)
      block ;; label = @1
        local.get 0
        i32.load offset=4
        local.tee 1
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load
        local.get 1
        i32.const 1
        call $__rust_dealloc
      end
    )
    (func $core::ptr::drop_in_place<()> (;56;) (type $.data) (param i32))
    (func $core::ptr::drop_in_place<std::fs::File> (;57;) (type $.data) (param i32)
      local.get 0
      i32.load
      call $close
      drop
    )
    (func $alloc::sync::Arc<T,A>::drop_slow (;58;) (type $.data) (param i32)
      (local i32 i32)
      block ;; label = @1
        local.get 0
        i32.load
        local.tee 0
        i32.const 16
        i32.add
        i32.load
        local.tee 1
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.const 20
        i32.add
        i32.load
        local.set 2
        local.get 1
        i32.const 0
        i32.store8
        local.get 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 1
        local.get 2
        i32.const 1
        call $__rust_dealloc
      end
      block ;; label = @1
        local.get 0
        i32.const -1
        i32.eq
        br_if 0 (;@1;)
        local.get 0
        local.get 0
        i32.load offset=4
        local.tee 1
        i32.const -1
        i32.add
        i32.store offset=4
        local.get 1
        i32.const 1
        i32.ne
        br_if 0 (;@1;)
        local.get 0
        i32.const 24
        i32.const 8
        call $__rust_dealloc
      end
    )
    (func $core::ptr::drop_in_place<alloc::string::String> (;59;) (type $.data) (param i32)
      (local i32)
      block ;; label = @1
        local.get 0
        i32.load offset=4
        local.tee 1
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load
        local.get 1
        i32.const 1
        call $__rust_dealloc
      end
    )
    (func $core::ptr::drop_in_place<alloc::vec::Vec<u8>> (;60;) (type $.data) (param i32)
      (local i32)
      block ;; label = @1
        local.get 0
        i32.load offset=4
        local.tee 1
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.load
        local.get 1
        i32.const 1
        call $__rust_dealloc
      end
    )
    (func $core::ptr::drop_in_place<std::panicking::begin_panic_handler::PanicPayload> (;61;) (type $.data) (param i32)
      (local i32)
      block ;; label = @1
        local.get 0
        i32.load offset=4
        local.tee 1
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.const 8
        i32.add
        i32.load
        local.tee 0
        i32.eqz
        br_if 0 (;@1;)
        local.get 1
        local.get 0
        i32.const 1
        call $__rust_dealloc
      end
    )
    (func $core::ptr::drop_in_place<std::io::Write::write_fmt::Adapter<std::fs::File>> (;62;) (type $.data) (param i32)
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
        call_indirect (type $.data)
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
    (func $core::ptr::drop_in_place<core::result::Result<(),std::io::error::Error>> (;63;) (type 2) (param i32 i32)
      (local i32 i32)
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.const 255
          i32.and
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
        call_indirect (type $.data)
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
    (func $core::error::Error::cause (;64;) (type 2) (param i32 i32)
      local.get 0
      i32.const 0
      i32.store
    )
    (func $core::error::Error::provide (;65;) (type 4) (param i32 i32 i32))
    (func $core::error::Error::type_id (;66;) (type 2) (param i32 i32)
      local.get 0
      i64.const -1279653969975287714
      i64.store offset=8
      local.get 0
      i64.const -1088588774072656362
      i64.store
    )
    (func $core::panicking::assert_failed (;67;) (type 2) (param i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 2
      i32.const 1050220
      i32.store offset=12
      local.get 2
      local.get 0
      i32.store offset=8
      i32.const 1
      local.get 2
      i32.const 8
      i32.add
      i32.const 1048860
      local.get 2
      i32.const 12
      i32.add
      i32.const 1048860
      local.get 1
      i32.const 1051460
      call $core::panicking::assert_failed_inner
      unreachable
    )
    (func $core::panicking::assert_failed (;68;) (type 2) (param i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 2
      i32.const 1048856
      i32.store offset=12
      local.get 2
      local.get 0
      i32.store offset=8
      i32.const 0
      local.get 2
      i32.const 8
      i32.add
      i32.const 1048876
      local.get 2
      i32.const 12
      i32.add
      i32.const 1048876
      local.get 1
      i32.const 1050040
      call $core::panicking::assert_failed_inner
      unreachable
    )
    (func $<&mut W as core::fmt::Write>::write_char (;69;) (type 6) (param i32 i32) (result i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 0
      i32.load
      local.set 0
      local.get 2
      i32.const 0
      i32.store offset=12
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
              i32.store8 offset=14
              local.get 2
              local.get 1
              i32.const 12
              i32.shr_u
              i32.const 224
              i32.or
              i32.store8 offset=12
              local.get 2
              local.get 1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              i32.const 3
              local.set 1
              br 3 (;@1;)
            end
            local.get 2
            local.get 1
            i32.store8 offset=12
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
          i32.store8 offset=13
          local.get 2
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 192
          i32.or
          i32.store8 offset=12
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
        i32.store8 offset=15
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=14
        local.get 2
        local.get 1
        i32.const 12
        i32.shr_u
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 18
        i32.shr_u
        i32.const 7
        i32.and
        i32.const 240
        i32.or
        i32.store8 offset=12
        i32.const 4
        local.set 1
      end
      local.get 0
      local.get 2
      i32.const 12
      i32.add
      local.get 1
      call $<std::io::Write::write_fmt::Adapter<T> as core::fmt::Write>::write_str
      local.set 1
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 1
    )
    (func $<&mut W as core::fmt::Write>::write_char (;70;) (type 6) (param i32 i32) (result i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 0
      i32.load
      local.set 0
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 1
              i32.const 128
              i32.lt_u
              br_if 0 (;@4;)
              local.get 2
              i32.const 0
              i32.store offset=12
              local.get 1
              i32.const 2048
              i32.lt_u
              br_if 1 (;@3;)
              block ;; label = @5
                local.get 1
                i32.const 65536
                i32.ge_u
                br_if 0 (;@5;)
                local.get 2
                local.get 1
                i32.const 63
                i32.and
                i32.const 128
                i32.or
                i32.store8 offset=14
                local.get 2
                local.get 1
                i32.const 12
                i32.shr_u
                i32.const 224
                i32.or
                i32.store8 offset=12
                local.get 2
                local.get 1
                i32.const 6
                i32.shr_u
                i32.const 63
                i32.and
                i32.const 128
                i32.or
                i32.store8 offset=13
                i32.const 3
                local.set 1
                br 3 (;@2;)
              end
              local.get 2
              local.get 1
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=15
              local.get 2
              local.get 1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=14
              local.get 2
              local.get 1
              i32.const 12
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              local.get 2
              local.get 1
              i32.const 18
              i32.shr_u
              i32.const 7
              i32.and
              i32.const 240
              i32.or
              i32.store8 offset=12
              i32.const 4
              local.set 1
              br 2 (;@2;)
            end
            block ;; label = @4
              local.get 0
              i32.load offset=8
              local.tee 3
              local.get 0
              i32.load offset=4
              i32.ne
              br_if 0 (;@4;)
              local.get 0
              local.get 3
              call $alloc::raw_vec::RawVec<T,A>::reserve_for_push
              local.get 0
              i32.load offset=8
              local.set 3
            end
            local.get 0
            local.get 3
            i32.const 1
            i32.add
            i32.store offset=8
            local.get 0
            i32.load
            local.get 3
            i32.add
            local.get 1
            i32.store8
            br 2 (;@1;)
          end
          local.get 2
          local.get 1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=13
          local.get 2
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 192
          i32.or
          i32.store8 offset=12
          i32.const 2
          local.set 1
        end
        block ;; label = @2
          local.get 0
          i32.load offset=4
          local.get 0
          i32.load offset=8
          local.tee 3
          i32.sub
          local.get 1
          i32.ge_u
          br_if 0 (;@2;)
          local.get 0
          local.get 3
          local.get 1
          call $alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle
          local.get 0
          i32.load offset=8
          local.set 3
        end
        local.get 0
        i32.load
        local.get 3
        i32.add
        local.get 2
        i32.const 12
        i32.add
        local.get 1
        call $memcpy
        drop
        local.get 0
        local.get 3
        local.get 1
        i32.add
        i32.store offset=8
      end
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      i32.const 0
    )
    (func $alloc::raw_vec::RawVec<T,A>::reserve_for_push (;71;) (type 2) (param i32 i32)
      (local i32 i32 i32)
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
          i32.const 8
          local.get 1
          i32.const 8
          i32.gt_u
          select
          local.tee 1
          i32.const -1
          i32.xor
          i32.const 31
          i32.shr_u
          local.set 4
          block ;; label = @3
            block ;; label = @4
              local.get 3
              i32.eqz
              br_if 0 (;@4;)
              local.get 2
              local.get 3
              i32.store offset=28
              local.get 2
              i32.const 1
              i32.store offset=24
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
          local.get 4
          local.get 1
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
    (func $<&mut W as core::fmt::Write>::write_char (;72;) (type 6) (param i32 i32) (result i32)
      local.get 0
      i32.load
      local.get 1
      call $core::fmt::Write::write_char
    )
    (func $<&mut W as core::fmt::Write>::write_char (;73;) (type 6) (param i32 i32) (result i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 0
      i32.load
      local.set 0
      local.get 2
      i32.const 0
      i32.store offset=12
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
              i32.store8 offset=14
              local.get 2
              local.get 1
              i32.const 12
              i32.shr_u
              i32.const 224
              i32.or
              i32.store8 offset=12
              local.get 2
              local.get 1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              i32.const 3
              local.set 1
              br 3 (;@1;)
            end
            local.get 2
            local.get 1
            i32.store8 offset=12
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
          i32.store8 offset=13
          local.get 2
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 192
          i32.or
          i32.store8 offset=12
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
        i32.store8 offset=15
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=14
        local.get 2
        local.get 1
        i32.const 12
        i32.shr_u
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 18
        i32.shr_u
        i32.const 7
        i32.and
        i32.const 240
        i32.or
        i32.store8 offset=12
        i32.const 4
        local.set 1
      end
      local.get 0
      local.get 2
      i32.const 12
      i32.add
      local.get 1
      call $<std::io::Write::write_fmt::Adapter<T> as core::fmt::Write>::write_str
      local.set 1
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 1
    )
    (func $<&mut W as core::fmt::Write>::write_fmt (;74;) (type 6) (param i32 i32) (result i32)
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
      i32.const 1048672
      local.get 1
      call $core::fmt::write
      local.set 0
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $<&mut W as core::fmt::Write>::write_fmt (;75;) (type 6) (param i32 i32) (result i32)
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
      i32.const 1048648
      local.get 1
      call $core::fmt::write
      local.set 0
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $<&mut W as core::fmt::Write>::write_fmt (;76;) (type 6) (param i32 i32) (result i32)
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
      i32.const 1048696
      local.get 1
      call $core::fmt::write
      local.set 0
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $<&mut W as core::fmt::Write>::write_fmt (;77;) (type 6) (param i32 i32) (result i32)
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
      i32.const 1048720
      local.get 1
      call $core::fmt::write
      local.set 0
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $<&mut W as core::fmt::Write>::write_str (;78;) (type 5) (param i32 i32 i32) (result i32)
      local.get 0
      i32.load
      local.get 1
      local.get 2
      call $<std::io::Write::write_fmt::Adapter<T> as core::fmt::Write>::write_str
    )
    (func $<&mut W as core::fmt::Write>::write_str (;79;) (type 5) (param i32 i32 i32) (result i32)
      (local i32)
      block ;; label = @1
        local.get 0
        i32.load
        i32.load offset=8
        local.tee 0
        i32.load offset=4
        local.get 0
        i32.load offset=8
        local.tee 3
        i32.sub
        local.get 2
        i32.ge_u
        br_if 0 (;@1;)
        local.get 0
        local.get 3
        local.get 2
        call $alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle
        local.get 0
        i32.load offset=8
        local.set 3
      end
      local.get 0
      i32.load
      local.get 3
      i32.add
      local.get 1
      local.get 2
      call $memcpy
      drop
      local.get 0
      local.get 3
      local.get 2
      i32.add
      i32.store offset=8
      i32.const 0
    )
    (func $<&mut W as core::fmt::Write>::write_str (;80;) (type 5) (param i32 i32 i32) (result i32)
      local.get 0
      i32.load
      local.get 1
      local.get 2
      call $<std::io::Write::write_fmt::Adapter<T> as core::fmt::Write>::write_str
    )
    (func $<&mut W as core::fmt::Write>::write_str (;81;) (type 5) (param i32 i32 i32) (result i32)
      (local i32)
      block ;; label = @1
        local.get 0
        i32.load
        local.tee 0
        i32.load offset=4
        local.get 0
        i32.load offset=8
        local.tee 3
        i32.sub
        local.get 2
        i32.ge_u
        br_if 0 (;@1;)
        local.get 0
        local.get 3
        local.get 2
        call $alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle
        local.get 0
        i32.load offset=8
        local.set 3
      end
      local.get 0
      i32.load
      local.get 3
      i32.add
      local.get 1
      local.get 2
      call $memcpy
      drop
      local.get 0
      local.get 3
      local.get 2
      i32.add
      i32.store offset=8
      i32.const 0
    )
    (func $alloc::sync::Arc<T,A>::drop_slow (;82;) (type $.data) (param i32)
      (local i32)
      block ;; label = @1
        local.get 0
        i32.load
        local.tee 0
        i32.const 16
        i32.add
        i32.load
        local.tee 1
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.const 12
        i32.add
        i32.load
        local.get 1
        i32.const 1
        call $__rust_dealloc
      end
      block ;; label = @1
        local.get 0
        i32.const -1
        i32.eq
        br_if 0 (;@1;)
        local.get 0
        local.get 0
        i32.load offset=4
        local.tee 1
        i32.const -1
        i32.add
        i32.store offset=4
        local.get 1
        i32.const 1
        i32.ne
        br_if 0 (;@1;)
        local.get 0
        i32.const 24
        i32.const 4
        call $__rust_dealloc
      end
    )
    (func $alloc::raw_vec::finish_grow (;83;) (type 12) (param i32 i32 i32 i32)
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
                    i32.load8_u offset=1056309
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
                i32.load8_u offset=1056309
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
    (func $<core::result::Result<T,E> as core::fmt::Debug>::fmt (;84;) (type 6) (param i32 i32) (result i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.load
          br_if 0 (;@2;)
          local.get 2
          local.get 0
          i32.store offset=8
          local.get 1
          i32.const 1049068
          i32.const 2
          local.get 2
          i32.const 8
          i32.add
          i32.const 1049072
          call $core::fmt::Formatter::debug_tuple_field1_finish
          local.set 0
          br 1 (;@1;)
        end
        local.get 2
        local.get 0
        i32.store offset=12
        local.get 1
        i32.const 1049088
        i32.const 3
        local.get 2
        i32.const 12
        i32.add
        i32.const 1049092
        call $core::fmt::Formatter::debug_tuple_field1_finish
        local.set 0
      end
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $std::sys::wasi::once::Once::call (;85;) (type $.data) (param i32)
      (local i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 32
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
                    i32.const 0
                    i32.load8_u offset=1056310
                    br_table 0 (;@7;) 2 (;@5;) 6 (;@1;) 1 (;@6;) 0 (;@7;)
                  end
                  i32.const 0
                  i32.const 2
                  i32.store8 offset=1056310
                  local.get 0
                  i32.load8_u
                  local.set 2
                  local.get 0
                  i32.const 0
                  i32.store8
                  local.get 2
                  i32.eqz
                  br_if 2 (;@4;)
                  local.get 1
                  i32.const 0
                  i32.store8 offset=31
                  block ;; label = @7
                    block ;; label = @8
                      i32.const 0
                      i32.load8_u offset=1056344
                      i32.const 3
                      i32.eq
                      br_if 0 (;@8;)
                      local.get 1
                      i32.const 31
                      i32.add
                      call $std::sync::once_lock::OnceLock<T>::initialize
                      local.get 1
                      i32.load8_u offset=31
                      br_if 1 (;@7;)
                    end
                    block ;; label = @8
                      block ;; label = @9
                        i32.const 0
                        i32.load offset=1056312
                        i32.const 1056408
                        i32.eq
                        br_if 0 (;@9;)
                        i32.const 0
                        i32.load8_u offset=1056340
                        local.set 2
                        i32.const 1
                        local.set 0
                        i32.const 0
                        i32.const 1
                        i32.store8 offset=1056340
                        local.get 2
                        br_if 2 (;@7;)
                        i32.const 0
                        i32.const 1056408
                        i32.store offset=1056312
                        br 1 (;@8;)
                      end
                      i32.const 0
                      i32.load offset=1056316
                      i32.const 1
                      i32.add
                      local.tee 0
                      i32.eqz
                      br_if 5 (;@3;)
                    end
                    i32.const 0
                    local.get 0
                    i32.store offset=1056316
                    i32.const 0
                    i32.load offset=1056320
                    br_if 5 (;@2;)
                    i32.const 0
                    i32.const -1
                    i32.store offset=1056320
                    block ;; label = @8
                      i32.const 0
                      i32.load8_u offset=1056336
                      br_if 0 (;@8;)
                      local.get 1
                      i32.const 4
                      i32.add
                      i32.const 1056324
                      call $std::io::buffered::bufwriter::BufWriter<W>::flush_buf
                      local.get 1
                      i32.load offset=8
                      local.set 2
                      block ;; label = @9
                        local.get 1
                        i32.load8_u offset=4
                        local.tee 0
                        i32.const 4
                        i32.gt_u
                        br_if 0 (;@9;)
                        local.get 0
                        i32.const 3
                        i32.ne
                        br_if 1 (;@8;)
                      end
                      local.get 2
                      i32.load
                      local.tee 3
                      local.get 2
                      i32.const 4
                      i32.add
                      i32.load
                      local.tee 0
                      i32.load
                      call_indirect (type $.data)
                      block ;; label = @9
                        local.get 0
                        i32.load offset=4
                        local.tee 4
                        i32.eqz
                        br_if 0 (;@9;)
                        local.get 3
                        local.get 4
                        local.get 0
                        i32.load offset=8
                        call $__rust_dealloc
                      end
                      local.get 2
                      i32.const 12
                      i32.const 4
                      call $__rust_dealloc
                    end
                    block ;; label = @8
                      i32.const 0
                      i32.load offset=1056328
                      local.tee 0
                      i32.eqz
                      br_if 0 (;@8;)
                      i32.const 0
                      i32.load offset=1056324
                      local.get 0
                      i32.const 1
                      call $__rust_dealloc
                    end
                    i32.const 0
                    i64.const 0
                    i64.store offset=1056328 align=4
                    i32.const 0
                    i32.const 1
                    i32.store offset=1056324
                    i32.const 0
                    i32.const 0
                    i32.load offset=1056320
                    i32.const 1
                    i32.add
                    i32.store offset=1056320
                    i32.const 0
                    i32.const 0
                    i32.load offset=1056316
                    i32.const -1
                    i32.add
                    local.tee 0
                    i32.store offset=1056316
                    i32.const 0
                    i32.const 0
                    i32.store8 offset=1056336
                    local.get 0
                    br_if 0 (;@7;)
                    i32.const 0
                    i32.const 0
                    i32.store8 offset=1056340
                    i32.const 0
                    i32.const 0
                    i32.store offset=1056312
                  end
                  i32.const 0
                  i32.const 3
                  i32.store8 offset=1056310
                end
                local.get 1
                i32.const 32
                i32.add
                global.set $__stack_pointer
                return
              end
              local.get 1
              i32.const 16
              i32.add
              i64.const 0
              i64.store align=4
              local.get 1
              i32.const 1
              i32.store offset=8
              local.get 1
              i32.const 1051764
              i32.store offset=4
              local.get 1
              i32.const 1048764
              i32.store offset=12
              local.get 1
              i32.const 4
              i32.add
              i32.const 1049212
              call $core::panicking::panic_fmt
              unreachable
            end
            i32.const 1048892
            i32.const 43
            i32.const 1050084
            call $core::panicking::panic
            unreachable
          end
          i32.const 1050132
          i32.const 38
          i32.const 1050204
          call $core::option::expect_failed
          unreachable
        end
        i32.const 1049404
        i32.const 16
        local.get 1
        i32.const 4
        i32.add
        i32.const 1049420
        i32.const 1049464
        call $core::result::unwrap_failed
        unreachable
      end
      local.get 1
      i32.const 16
      i32.add
      i64.const 0
      i64.store align=4
      local.get 1
      i32.const 1
      i32.store offset=8
      local.get 1
      i32.const 1051828
      i32.store offset=4
      local.get 1
      i32.const 1048764
      i32.store offset=12
      local.get 1
      i32.const 4
      i32.add
      i32.const 1049212
      call $core::panicking::panic_fmt
      unreachable
    )
    (func $std::rt::lang_start_internal (;86;) (type 15) (param i32 i32 i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i64 i64 i64)
      global.get $__stack_pointer
      i32.const 80
      i32.sub
      local.tee 5
      global.set $__stack_pointer
      local.get 5
      i32.const 16
      i32.add
      i32.const 1049108
      i32.const 4
      call $<&str as alloc::ffi::c_str::CString::new::SpecNewImpl>::spec_new_impl
      block ;; label = @1
        local.get 5
        i32.load offset=16
        br_if 0 (;@1;)
        local.get 5
        i32.const 16
        i32.add
        i32.const 8
        i32.add
        i32.load
        local.set 6
        local.get 5
        i32.load offset=20
        local.set 7
        local.get 5
        i32.const 8
        i32.add
        i32.const 8
        i32.const 16
        call $alloc::sync::arcinner_layout_for_value_layout
        local.get 5
        i32.load offset=8
        local.set 8
        block ;; label = @2
          block ;; label = @3
            local.get 5
            i32.load offset=12
            local.tee 9
            br_if 0 (;@3;)
            local.get 8
            local.set 10
            br 1 (;@2;)
          end
          i32.const 0
          i32.load8_u offset=1056309
          drop
          local.get 9
          local.get 8
          call $__rust_alloc
          local.set 10
        end
        block ;; label = @2
          block ;; label = @3
            local.get 10
            i32.eqz
            br_if 0 (;@3;)
            local.get 10
            i64.const 4294967297
            i64.store align=4
            local.get 10
            local.get 6
            i32.store offset=20
            local.get 10
            i32.const 16
            i32.add
            local.get 7
            i32.store
            i32.const 0
            i64.load offset=1056384
            local.set 11
            loop ;; label = @4
              local.get 11
              i64.const 1
              i64.add
              local.tee 12
              i64.eqz
              br_if 2 (;@2;)
              i32.const 0
              local.get 12
              i32.const 0
              i64.load offset=1056384
              local.tee 13
              local.get 13
              local.get 11
              i64.eq
              local.tee 8
              select
              i64.store offset=1056384
              local.get 13
              local.set 11
              local.get 8
              i32.eqz
              br_if 0 (;@4;)
            end
            local.get 10
            local.get 12
            i64.store offset=8
            block ;; label = @4
              local.get 5
              i32.load offset=16
              local.tee 8
              i32.eqz
              br_if 0 (;@4;)
              local.get 5
              i32.load offset=20
              local.tee 6
              i32.eqz
              br_if 0 (;@4;)
              local.get 8
              local.get 6
              i32.const 1
              call $__rust_dealloc
            end
            local.get 10
            call $std::sys_common::thread_info::set
            local.get 0
            local.get 1
            i32.const 20
            i32.add
            i32.load
            call_indirect (type 3)
            local.set 8
            block ;; label = @4
              i32.const 0
              i32.load8_u offset=1056310
              i32.const 3
              i32.eq
              br_if 0 (;@4;)
              local.get 5
              i32.const 1
              i32.store8 offset=44
              local.get 5
              i32.const 44
              i32.add
              call $std::sys::wasi::once::Once::call
            end
            local.get 5
            i32.const 80
            i32.add
            global.set $__stack_pointer
            local.get 8
            return
          end
          local.get 8
          local.get 9
          call $alloc::alloc::handle_alloc_error
          unreachable
        end
        call $std::thread::ThreadId::new::exhausted
        unreachable
      end
      local.get 5
      local.get 5
      i32.const 16
      i32.add
      i32.store offset=32
      local.get 5
      i32.const 6
      i32.store offset=72
      local.get 5
      local.get 5
      i32.const 32
      i32.add
      i32.store offset=68
      local.get 5
      i32.const 44
      i32.add
      i32.const 1049172
      i32.const 2
      local.get 5
      i32.const 68
      i32.add
      i32.const 1
      call $core::fmt::Arguments::new_v1
      local.get 5
      i32.const 36
      i32.add
      local.get 5
      i32.const 79
      i32.add
      local.get 5
      i32.const 44
      i32.add
      call $std::io::Write::write_fmt
      local.get 5
      i32.load8_u offset=36
      local.get 5
      i32.load offset=40
      call $core::ptr::drop_in_place<core::result::Result<(),std::io::error::Error>>
      call $std::sys::wasi::abort_internal
      unreachable
    )
    (func $std::sys_common::thread_info::set (;87;) (type $.data) (param i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          i32.const 0
          i32.load offset=1056400
          br_if 0 (;@2;)
          i32.const 0
          i32.const -1
          i32.store offset=1056400
          i32.const 0
          i32.load offset=1056404
          br_if 1 (;@1;)
          i32.const 0
          local.get 0
          i32.store offset=1056404
          i32.const 0
          i32.const 0
          i32.store offset=1056400
          local.get 1
          i32.const 48
          i32.add
          global.set $__stack_pointer
          return
        end
        i32.const 1049404
        i32.const 16
        local.get 1
        i32.const 44
        i32.add
        i32.const 1049420
        i32.const 1050544
        call $core::result::unwrap_failed
        unreachable
      end
      local.get 1
      i32.const 20
      i32.add
      i32.const 1050536
      i32.const 1
      local.get 1
      i32.const 44
      i32.add
      i32.const 0
      call $core::fmt::Arguments::new_v1
      local.get 1
      i32.const 12
      i32.add
      local.get 1
      i32.const 44
      i32.add
      local.get 1
      i32.const 20
      i32.add
      call $std::io::Write::write_fmt
      local.get 1
      i32.load8_u offset=12
      local.get 1
      i32.load offset=16
      call $core::ptr::drop_in_place<core::result::Result<(),std::io::error::Error>>
      call $std::sys::wasi::abort_internal
      unreachable
    )
    (func $std::thread::ThreadId::new::exhausted (;88;) (type $.rodata)
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
      i32.const 1049312
      i32.store offset=8
      local.get 0
      i32.const 1048764
      i32.store offset=16
      local.get 0
      i32.const 8
      i32.add
      i32.const 1049320
      call $core::panicking::panic_fmt
      unreachable
    )
    (func $std::io::Write::write_fmt (;89;) (type 4) (param i32 i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      i32.const 4
      i32.store8
      local.get 3
      local.get 1
      i32.store offset=8
      block ;; label = @1
        block ;; label = @2
          local.get 3
          i32.const 1049812
          local.get 2
          call $core::fmt::write
          i32.eqz
          br_if 0 (;@2;)
          block ;; label = @3
            local.get 3
            i32.load8_u
            i32.const 4
            i32.ne
            br_if 0 (;@3;)
            local.get 0
            i32.const 1049776
            i32.store offset=4
            local.get 0
            i32.const 2
            i32.store8
            br 2 (;@1;)
          end
          local.get 0
          local.get 3
          i64.load
          i64.store align=4
          br 1 (;@1;)
        end
        local.get 0
        i32.const 4
        i32.store8
        local.get 3
        i32.load offset=4
        local.set 1
        block ;; label = @2
          local.get 3
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
        call_indirect (type $.data)
        block ;; label = @2
          local.get 0
          i32.load offset=4
          local.tee 4
          i32.eqz
          br_if 0 (;@2;)
          local.get 2
          local.get 4
          local.get 0
          i32.load offset=8
          call $__rust_dealloc
        end
        local.get 1
        i32.const 12
        i32.const 4
        call $__rust_dealloc
      end
      local.get 3
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $std::sys::wasi::abort_internal (;90;) (type $.rodata)
      call $abort
      unreachable
    )
    (func $std::sys_common::thread_info::current_thread (;91;) (type 13) (result i32)
      (local i32 i32 i32 i32 i64 i64 i64)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 0
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              i32.const 0
              i32.load offset=1056400
              br_if 0 (;@4;)
              i32.const 0
              i32.const -1
              i32.store offset=1056400
              block ;; label = @5
                i32.const 0
                i32.load offset=1056404
                local.tee 1
                br_if 0 (;@5;)
                local.get 0
                i32.const 8
                i32.const 16
                call $alloc::sync::arcinner_layout_for_value_layout
                local.get 0
                i32.load
                local.set 2
                block ;; label = @6
                  block ;; label = @7
                    local.get 0
                    i32.load offset=4
                    local.tee 3
                    br_if 0 (;@7;)
                    local.get 2
                    local.set 1
                    br 1 (;@6;)
                  end
                  i32.const 0
                  i32.load8_u offset=1056309
                  drop
                  local.get 3
                  local.get 2
                  call $__rust_alloc
                  local.set 1
                end
                local.get 1
                i32.eqz
                br_if 3 (;@2;)
                local.get 1
                i64.const 4294967297
                i64.store align=4
                local.get 1
                i32.const 16
                i32.add
                i32.const 0
                i32.store
                i32.const 0
                i64.load offset=1056384
                local.set 4
                loop ;; label = @6
                  local.get 4
                  i64.const 1
                  i64.add
                  local.tee 5
                  i64.eqz
                  br_if 5 (;@1;)
                  i32.const 0
                  local.get 5
                  i32.const 0
                  i64.load offset=1056384
                  local.tee 6
                  local.get 6
                  local.get 4
                  i64.eq
                  local.tee 2
                  select
                  i64.store offset=1056384
                  local.get 6
                  local.set 4
                  local.get 2
                  i32.eqz
                  br_if 0 (;@6;)
                end
                i32.const 0
                local.get 1
                i32.store offset=1056404
                local.get 1
                local.get 5
                i64.store offset=8
              end
              local.get 1
              local.get 1
              i32.load
              local.tee 2
              i32.const 1
              i32.add
              i32.store
              local.get 2
              i32.const -1
              i32.gt_s
              br_if 1 (;@3;)
              unreachable
              unreachable
            end
            i32.const 1049404
            i32.const 16
            local.get 0
            i32.const 15
            i32.add
            i32.const 1049420
            i32.const 1050456
            call $core::result::unwrap_failed
            unreachable
          end
          i32.const 0
          i32.const 0
          i32.load offset=1056400
          i32.const 1
          i32.add
          i32.store offset=1056400
          local.get 0
          i32.const 16
          i32.add
          global.set $__stack_pointer
          local.get 1
          return
        end
        local.get 2
        local.get 3
        call $alloc::alloc::handle_alloc_error
        unreachable
      end
      call $std::thread::ThreadId::new::exhausted
      unreachable
    )
    (func $std::env::current_dir (;92;) (type $.data) (param i32)
      (local i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      i32.const 0
      i32.load8_u offset=1056309
      drop
      i32.const 512
      local.set 2
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              i32.const 512
              i32.const 1
              call $__rust_alloc
              local.tee 3
              i32.eqz
              br_if 0 (;@4;)
              local.get 1
              i32.const 512
              i32.store offset=8
              local.get 1
              local.get 3
              i32.store offset=4
              local.get 3
              i32.const 512
              call $getcwd
              br_if 1 (;@3;)
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    i32.const 0
                    i32.load offset=1056916
                    local.tee 2
                    i32.const 68
                    i32.ne
                    br_if 0 (;@7;)
                    i32.const 512
                    local.set 2
                    br 1 (;@6;)
                  end
                  local.get 0
                  i64.const 0
                  i64.store align=4
                  local.get 0
                  i32.const 8
                  i32.add
                  local.get 2
                  i32.store
                  i32.const 512
                  local.set 2
                  br 1 (;@5;)
                end
                loop ;; label = @6
                  local.get 1
                  local.get 2
                  i32.store offset=12
                  local.get 1
                  i32.const 4
                  i32.add
                  local.get 2
                  i32.const 1
                  call $alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle
                  local.get 1
                  i32.load offset=4
                  local.tee 3
                  local.get 1
                  i32.load offset=8
                  local.tee 2
                  call $getcwd
                  br_if 3 (;@3;)
                  i32.const 0
                  i32.load offset=1056916
                  local.tee 4
                  i32.const 68
                  i32.eq
                  br_if 0 (;@6;)
                end
                local.get 0
                i64.const 0
                i64.store align=4
                local.get 0
                i32.const 8
                i32.add
                local.get 4
                i32.store
                local.get 2
                i32.eqz
                br_if 3 (;@2;)
              end
              local.get 3
              local.get 2
              i32.const 1
              call $__rust_dealloc
              br 2 (;@2;)
            end
            i32.const 1
            i32.const 512
            call $alloc::alloc::handle_alloc_error
            unreachable
          end
          local.get 1
          local.get 3
          call $strlen
          local.tee 4
          i32.store offset=12
          block ;; label = @3
            local.get 2
            local.get 4
            i32.le_u
            br_if 0 (;@3;)
            block ;; label = @4
              block ;; label = @5
                local.get 4
                br_if 0 (;@5;)
                i32.const 1
                local.set 5
                local.get 3
                local.get 2
                i32.const 1
                call $__rust_dealloc
                br 1 (;@4;)
              end
              local.get 3
              local.get 2
              i32.const 1
              local.get 4
              call $__rust_realloc
              local.tee 5
              i32.eqz
              br_if 3 (;@1;)
            end
            local.get 1
            local.get 4
            i32.store offset=8
            local.get 1
            local.get 5
            i32.store offset=4
          end
          local.get 0
          local.get 1
          i64.load offset=4 align=4
          i64.store align=4
          local.get 0
          i32.const 8
          i32.add
          local.get 1
          i32.const 4
          i32.add
          i32.const 8
          i32.add
          i32.load
          i32.store
        end
        local.get 1
        i32.const 16
        i32.add
        global.set $__stack_pointer
        return
      end
      i32.const 1
      local.get 4
      call $alloc::alloc::handle_alloc_error
      unreachable
    )
    (func $std::env::_var_os (;93;) (type 4) (param i32 i32 i32)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 416
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 2
              i32.const 383
              i32.gt_u
              br_if 0 (;@4;)
              local.get 3
              i32.const 20
              i32.add
              local.get 1
              local.get 2
              call $memcpy
              drop
              local.get 3
              i32.const 20
              i32.add
              local.get 2
              i32.add
              i32.const 0
              i32.store8
              i32.const 1
              local.set 1
              local.get 3
              i32.const 404
              i32.add
              local.get 3
              i32.const 20
              i32.add
              local.get 2
              i32.const 1
              i32.add
              call $core::ffi::c_str::CStr::from_bytes_with_nul
              block ;; label = @5
                local.get 3
                i32.load offset=404
                br_if 0 (;@5;)
                block ;; label = @6
                  local.get 3
                  i32.load offset=408
                  call $getenv
                  local.tee 1
                  br_if 0 (;@6;)
                  i32.const 0
                  local.set 1
                  local.get 3
                  i32.const 0
                  i32.store offset=8
                  br 5 (;@1;)
                end
                block ;; label = @6
                  block ;; label = @7
                    local.get 1
                    call $strlen
                    local.tee 2
                    br_if 0 (;@7;)
                    i32.const 1
                    local.set 4
                    br 1 (;@6;)
                  end
                  local.get 2
                  i32.const -1
                  i32.le_s
                  br_if 3 (;@3;)
                  i32.const 0
                  i32.load8_u offset=1056309
                  drop
                  local.get 2
                  i32.const 1
                  call $__rust_alloc
                  local.tee 4
                  i32.eqz
                  br_if 4 (;@2;)
                end
                local.get 4
                local.get 1
                local.get 2
                call $memcpy
                local.set 1
                local.get 3
                i32.const 16
                i32.add
                local.get 2
                i32.store
                local.get 3
                i32.const 12
                i32.add
                local.get 2
                i32.store
                local.get 3
                local.get 1
                i32.store offset=8
                i32.const 0
                local.set 1
                br 4 (;@1;)
              end
              local.get 3
              i32.const 0
              i64.load offset=1050280
              i64.store offset=8 align=4
              br 3 (;@1;)
            end
            local.get 3
            i32.const 4
            i32.add
            local.get 1
            local.get 2
            call $std::sys::common::small_c_string::run_with_cstr_allocating
            local.get 3
            i32.load offset=4
            local.set 1
            br 2 (;@1;)
          end
          call $alloc::raw_vec::capacity_overflow
          unreachable
        end
        i32.const 1
        local.get 2
        call $alloc::alloc::handle_alloc_error
        unreachable
      end
      block ;; label = @1
        block ;; label = @2
          local.get 1
          br_if 0 (;@2;)
          local.get 0
          local.get 3
          i64.load offset=8 align=4
          i64.store align=4
          local.get 0
          i32.const 8
          i32.add
          local.get 3
          i32.const 16
          i32.add
          i32.load
          i32.store
          br 1 (;@1;)
        end
        block ;; label = @2
          local.get 3
          i32.load8_u offset=8
          i32.const 3
          i32.ne
          br_if 0 (;@2;)
          local.get 3
          i32.const 12
          i32.add
          i32.load
          local.tee 2
          i32.load
          local.tee 4
          local.get 2
          i32.const 4
          i32.add
          i32.load
          local.tee 1
          i32.load
          call_indirect (type $.data)
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
          local.get 2
          i32.const 12
          i32.const 4
          call $__rust_dealloc
        end
        local.get 0
        i32.const 0
        i32.store
      end
      local.get 3
      i32.const 416
      i32.add
      global.set $__stack_pointer
    )
    (func $std::sys::common::small_c_string::run_with_cstr_allocating (;94;) (type 4) (param i32 i32 i32)
      (local i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      local.get 1
      local.get 2
      call $<&str as alloc::ffi::c_str::CString::new::SpecNewImpl>::spec_new_impl
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 3
              i32.load
              local.tee 2
              br_if 0 (;@4;)
              local.get 3
              i32.const 8
              i32.add
              i32.load
              local.set 4
              block ;; label = @5
                block ;; label = @6
                  local.get 3
                  i32.load offset=4
                  local.tee 1
                  call $getenv
                  local.tee 5
                  br_if 0 (;@6;)
                  local.get 0
                  i32.const 0
                  i32.store offset=4
                  br 1 (;@5;)
                end
                block ;; label = @6
                  block ;; label = @7
                    local.get 5
                    call $strlen
                    local.tee 2
                    br_if 0 (;@7;)
                    i32.const 1
                    local.set 6
                    br 1 (;@6;)
                  end
                  local.get 2
                  i32.const -1
                  i32.le_s
                  br_if 3 (;@3;)
                  i32.const 0
                  i32.load8_u offset=1056309
                  drop
                  local.get 2
                  i32.const 1
                  call $__rust_alloc
                  local.tee 6
                  i32.eqz
                  br_if 4 (;@2;)
                end
                local.get 6
                local.get 5
                local.get 2
                call $memcpy
                local.set 5
                local.get 0
                i32.const 12
                i32.add
                local.get 2
                i32.store
                local.get 0
                i32.const 8
                i32.add
                local.get 2
                i32.store
                local.get 0
                local.get 5
                i32.store offset=4
              end
              local.get 1
              i32.const 0
              i32.store8
              local.get 0
              i32.const 0
              i32.store
              local.get 4
              i32.eqz
              br_if 3 (;@1;)
              local.get 1
              local.get 4
              i32.const 1
              call $__rust_dealloc
              br 3 (;@1;)
            end
            local.get 0
            i32.const 1
            i32.store
            local.get 0
            i32.const 0
            i64.load offset=1050280
            i64.store offset=4 align=4
            local.get 3
            i32.load offset=4
            local.tee 0
            i32.eqz
            br_if 2 (;@1;)
            local.get 2
            local.get 0
            i32.const 1
            call $__rust_dealloc
            br 2 (;@1;)
          end
          call $alloc::raw_vec::capacity_overflow
          unreachable
        end
        i32.const 1
        local.get 2
        call $alloc::alloc::handle_alloc_error
        unreachable
      end
      local.get 3
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $std::sys::wasi::fs::File::open (;95;) (type 12) (param i32 i32 i32 i32)
      (local i32 i64)
      global.get $__stack_pointer
      i32.const 416
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          local.get 2
          i32.const 383
          i32.gt_u
          br_if 0 (;@2;)
          local.get 4
          i32.const 20
          i32.add
          local.get 1
          local.get 2
          call $memcpy
          drop
          local.get 4
          i32.const 20
          i32.add
          local.get 2
          i32.add
          i32.const 0
          i32.store8
          local.get 4
          i32.const 404
          i32.add
          local.get 4
          i32.const 20
          i32.add
          local.get 2
          i32.const 1
          i32.add
          call $core::ffi::c_str::CStr::from_bytes_with_nul
          block ;; label = @3
            local.get 4
            i32.load offset=404
            br_if 0 (;@3;)
            local.get 4
            i32.const 4
            i32.add
            local.get 4
            i32.load offset=408
            local.get 4
            i32.const 412
            i32.add
            i32.load
            call $std::sys::wasi::fs::open_parent::{{closure}}
            br 2 (;@1;)
          end
          local.get 4
          i32.const -1
          i32.store offset=4
          local.get 4
          i32.const 0
          i64.load offset=1050280
          i64.store offset=8 align=4
          br 1 (;@1;)
        end
        local.get 4
        i32.const 4
        i32.add
        local.get 1
        local.get 2
        call $std::sys::common::small_c_string::run_with_cstr_allocating
      end
      block ;; label = @1
        block ;; label = @2
          local.get 4
          i32.load offset=4
          local.tee 2
          i32.const -1
          i32.eq
          br_if 0 (;@2;)
          local.get 0
          local.get 2
          local.get 4
          i64.load offset=8 align=4
          local.tee 5
          i32.wrap_i64
          local.tee 1
          local.get 4
          i32.load offset=16
          local.get 3
          call $std::sys::wasi::fs::open_at
          local.get 5
          i64.const 32
          i64.shr_u
          i32.wrap_i64
          local.tee 2
          i32.eqz
          br_if 1 (;@1;)
          local.get 1
          local.get 2
          i32.const 1
          call $__rust_dealloc
          br 1 (;@1;)
        end
        local.get 0
        local.get 4
        i64.load offset=8 align=4
        i64.store align=4
      end
      local.get 4
      i32.const 416
      i32.add
      global.set $__stack_pointer
    )
    (func $std::io::Write::write_all (;96;) (type 12) (param i32 i32 i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 3
            i32.eqz
            br_if 0 (;@3;)
            local.get 1
            i32.load
            local.set 5
            loop ;; label = @4
              local.get 4
              local.get 3
              i32.store offset=16
              local.get 4
              local.get 2
              i32.store offset=12
              local.get 4
              i32.const 20
              i32.add
              local.get 5
              local.get 4
              i32.const 12
              i32.add
              i32.const 1
              call $wasi::lib_generated::fd_write
              block ;; label = @5
                block ;; label = @6
                  local.get 4
                  i32.load16_u offset=20
                  br_if 0 (;@6;)
                  block ;; label = @7
                    local.get 4
                    i32.load offset=24
                    local.tee 1
                    br_if 0 (;@7;)
                    local.get 0
                    i32.const 1049708
                    i32.store offset=4
                    local.get 0
                    i32.const 2
                    i32.store8
                    br 6 (;@1;)
                  end
                  block ;; label = @7
                    local.get 3
                    local.get 1
                    i32.lt_u
                    br_if 0 (;@7;)
                    local.get 2
                    local.get 1
                    i32.add
                    local.set 2
                    local.get 3
                    local.get 1
                    i32.sub
                    local.set 3
                    br 2 (;@5;)
                  end
                  local.get 1
                  local.get 3
                  i32.const 1049720
                  call $core::slice::index::slice_start_index_len_fail
                  unreachable
                end
                local.get 4
                local.get 4
                i32.load16_u offset=22
                i32.store16 offset=30
                local.get 4
                i32.const 30
                i32.add
                call $wasi::lib_generated::Errno::raw
                i32.const 65535
                i32.and
                local.tee 1
                call $std::sys::wasi::decode_error_kind
                i32.const 255
                i32.and
                i32.const 35
                i32.ne
                br_if 3 (;@2;)
              end
              local.get 3
              br_if 0 (;@4;)
            end
          end
          local.get 0
          i32.const 4
          i32.store8
          br 1 (;@1;)
        end
        local.get 0
        local.get 1
        i32.store offset=4
        local.get 0
        i32.const 0
        i32.store
      end
      local.get 4
      i32.const 32
      i32.add
      global.set $__stack_pointer
    )
    (func $<std::fs::File as std::io::Write>::write (;97;) (type 12) (param i32 i32 i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      local.get 1
      i32.load
      local.set 1
      local.get 4
      local.get 3
      i32.store offset=16
      local.get 4
      local.get 2
      i32.store offset=12
      local.get 4
      i32.const 20
      i32.add
      local.get 1
      local.get 4
      i32.const 12
      i32.add
      i32.const 1
      call $wasi::lib_generated::fd_write
      block ;; label = @1
        block ;; label = @2
          local.get 4
          i32.load16_u offset=20
          br_if 0 (;@2;)
          local.get 0
          local.get 4
          i32.load offset=24
          i32.store offset=4
          local.get 0
          i32.const 4
          i32.store8
          br 1 (;@1;)
        end
        local.get 4
        local.get 4
        i32.load16_u offset=22
        i32.store16 offset=30
        local.get 0
        local.get 4
        i32.const 30
        i32.add
        call $wasi::lib_generated::Errno::raw
        i64.extend_i32_u
        i64.const 65535
        i64.and
        i64.const 32
        i64.shl
        i64.store align=4
      end
      local.get 4
      i32.const 32
      i32.add
      global.set $__stack_pointer
    )
    (func $<std::fs::File as std::io::Write>::write_vectored (;98;) (type 12) (param i32 i32 i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      local.get 4
      i32.const 4
      i32.add
      local.get 1
      i32.load
      local.get 2
      local.get 3
      call $wasi::lib_generated::fd_write
      block ;; label = @1
        block ;; label = @2
          local.get 4
          i32.load16_u offset=4
          br_if 0 (;@2;)
          local.get 0
          local.get 4
          i32.load offset=8
          i32.store offset=4
          local.get 0
          i32.const 4
          i32.store8
          br 1 (;@1;)
        end
        local.get 4
        local.get 4
        i32.load16_u offset=6
        i32.store16 offset=14
        local.get 0
        local.get 4
        i32.const 14
        i32.add
        call $wasi::lib_generated::Errno::raw
        i64.extend_i32_u
        i64.const 65535
        i64.and
        i64.const 32
        i64.shl
        i64.store align=4
      end
      local.get 4
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $std::sys::wasi::decode_error_kind (;99;) (type 3) (param i32) (result i32)
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
        i32.const 1051630
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 3
        local.set 1
        i32.const 1051632
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 1
        local.set 1
        i32.const 1051634
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 1051636
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 11
        local.set 1
        i32.const 1051638
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 7
        local.set 1
        i32.const 1051640
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 6
        local.set 1
        i32.const 1051642
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 9
        local.set 1
        i32.const 1051644
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 8
        local.set 1
        i32.const 1051646
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 0
        local.set 1
        i32.const 1051648
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 35
        local.set 1
        i32.const 1051650
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 20
        local.set 1
        i32.const 1051652
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 22
        local.set 1
        i32.const 1051654
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 12
        local.set 1
        i32.const 1051656
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 13
        local.set 1
        i32.const 1051658
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 36
        local.set 1
        i32.const 1051660
        call $wasi::lib_generated::Errno::raw
        i32.const 65535
        i32.and
        local.get 0
        i32.eq
        br_if 0 (;@1;)
        i32.const 38
        i32.const 40
        i32.const 1051662
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
    (func $std::io::buffered::bufwriter::BufWriter<W>::flush_buf (;100;) (type 2) (param i32 i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 1
            i32.load offset=8
            local.tee 3
            br_if 0 (;@3;)
            local.get 0
            i32.const 4
            i32.store8
            br 1 (;@2;)
          end
          local.get 1
          i32.load
          local.set 4
          i32.const 0
          local.set 5
          block ;; label = @3
            block ;; label = @4
              loop ;; label = @5
                local.get 2
                local.get 3
                local.get 5
                i32.sub
                local.tee 6
                i32.store offset=16
                local.get 2
                local.get 4
                local.get 5
                i32.add
                local.tee 7
                i32.store offset=12
                local.get 2
                i32.const 20
                i32.add
                i32.const 1
                local.get 2
                i32.const 12
                i32.add
                i32.const 1
                call $wasi::lib_generated::fd_write
                block ;; label = @6
                  block ;; label = @7
                    block ;; label = @8
                      block ;; label = @9
                        block ;; label = @10
                          local.get 2
                          i32.load16_u offset=20
                          br_if 0 (;@10;)
                          local.get 2
                          i32.load offset=24
                          local.set 8
                          br 1 (;@9;)
                        end
                        local.get 2
                        local.get 2
                        i32.load16_u offset=22
                        i32.store16 offset=30
                        local.get 6
                        local.set 8
                        local.get 2
                        i32.const 30
                        i32.add
                        call $wasi::lib_generated::Errno::raw
                        i32.const 65535
                        i32.and
                        local.tee 9
                        i32.const 1051628
                        call $wasi::lib_generated::Errno::raw
                        i32.const 65535
                        i32.and
                        i32.eq
                        br_if 0 (;@9;)
                        local.get 1
                        i32.const 0
                        i32.store8 offset=12
                        local.get 9
                        call $std::sys::wasi::decode_error_kind
                        i32.const 255
                        i32.and
                        i32.const 35
                        i32.eq
                        br_if 3 (;@6;)
                        local.get 0
                        local.get 9
                        i32.store offset=4
                        local.get 0
                        i32.const 0
                        i32.store
                        br 1 (;@8;)
                      end
                      local.get 1
                      i32.const 0
                      i32.store8 offset=12
                      local.get 8
                      br_if 1 (;@7;)
                      local.get 0
                      i32.const 1049384
                      i32.store offset=4
                      local.get 0
                      i32.const 2
                      i32.store8
                    end
                    local.get 5
                    i32.eqz
                    br_if 5 (;@2;)
                    local.get 3
                    local.get 5
                    i32.lt_u
                    br_if 3 (;@4;)
                    local.get 1
                    i32.const 0
                    i32.store offset=8
                    local.get 3
                    local.get 5
                    i32.eq
                    br_if 5 (;@2;)
                    local.get 4
                    local.get 7
                    local.get 6
                    call $memmove
                    drop
                    local.get 1
                    local.get 6
                    i32.store offset=8
                    br 5 (;@2;)
                  end
                  local.get 8
                  local.get 5
                  i32.add
                  local.set 5
                end
                local.get 5
                local.get 3
                i32.ge_u
                br_if 2 (;@3;)
                br 0 (;@5;)
              end
            end
            local.get 5
            local.get 3
            i32.const 1049052
            call $core::slice::index::slice_end_index_len_fail
            unreachable
          end
          local.get 0
          i32.const 4
          i32.store8
          local.get 5
          i32.eqz
          br_if 0 (;@2;)
          local.get 3
          local.get 5
          i32.lt_u
          br_if 1 (;@1;)
          local.get 1
          i32.const 0
          i32.store offset=8
        end
        local.get 2
        i32.const 32
        i32.add
        global.set $__stack_pointer
        return
      end
      local.get 5
      local.get 3
      i32.const 1049052
      call $core::slice::index::slice_end_index_len_fail
      unreachable
    )
    (func $std::io::impls::<impl std::io::Write for alloc::vec::Vec<u8,A>>::write (;101;) (type 12) (param i32 i32 i32 i32)
      (local i32)
      block ;; label = @1
        local.get 1
        i32.load offset=4
        local.get 1
        i32.load offset=8
        local.tee 4
        i32.sub
        local.get 3
        i32.ge_u
        br_if 0 (;@1;)
        local.get 1
        local.get 4
        local.get 3
        call $alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle
        local.get 1
        i32.load offset=8
        local.set 4
      end
      local.get 1
      i32.load
      local.get 4
      i32.add
      local.get 2
      local.get 3
      call $memcpy
      drop
      local.get 0
      local.get 3
      i32.store offset=4
      local.get 1
      local.get 4
      local.get 3
      i32.add
      i32.store offset=8
      local.get 0
      i32.const 4
      i32.store8
    )
    (func $std::io::impls::<impl std::io::Write for alloc::vec::Vec<u8,A>>::write_vectored (;102;) (type 12) (param i32 i32 i32 i32)
      (local i32 i32 i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          local.get 3
          br_if 0 (;@2;)
          i32.const 0
          local.set 4
          br 1 (;@1;)
        end
        local.get 3
        i32.const 3
        i32.and
        local.set 5
        block ;; label = @2
          block ;; label = @3
            local.get 3
            i32.const 4
            i32.ge_u
            br_if 0 (;@3;)
            i32.const 0
            local.set 4
            i32.const 0
            local.set 6
            br 1 (;@2;)
          end
          local.get 2
          i32.const 28
          i32.add
          local.set 7
          local.get 3
          i32.const -4
          i32.and
          local.set 8
          i32.const 0
          local.set 4
          i32.const 0
          local.set 6
          loop ;; label = @3
            local.get 7
            i32.load
            local.get 7
            i32.const -8
            i32.add
            i32.load
            local.get 7
            i32.const -16
            i32.add
            i32.load
            local.get 7
            i32.const -24
            i32.add
            i32.load
            local.get 4
            i32.add
            i32.add
            i32.add
            i32.add
            local.set 4
            local.get 7
            i32.const 32
            i32.add
            local.set 7
            local.get 8
            local.get 6
            i32.const 4
            i32.add
            local.tee 6
            i32.ne
            br_if 0 (;@3;)
          end
        end
        block ;; label = @2
          local.get 5
          i32.eqz
          br_if 0 (;@2;)
          local.get 6
          i32.const 3
          i32.shl
          local.get 2
          i32.add
          i32.const 4
          i32.add
          local.set 7
          loop ;; label = @3
            local.get 7
            i32.load
            local.get 4
            i32.add
            local.set 4
            local.get 7
            i32.const 8
            i32.add
            local.set 7
            local.get 5
            i32.const -1
            i32.add
            local.tee 5
            br_if 0 (;@3;)
          end
        end
        block ;; label = @2
          local.get 1
          i32.load offset=4
          local.get 1
          i32.load offset=8
          local.tee 7
          i32.sub
          local.get 4
          i32.ge_u
          br_if 0 (;@2;)
          local.get 1
          local.get 7
          local.get 4
          call $alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle
        end
        local.get 3
        i32.eqz
        br_if 0 (;@1;)
        local.get 2
        local.get 3
        i32.const 3
        i32.shl
        i32.add
        local.set 8
        local.get 1
        i32.load offset=8
        local.set 7
        loop ;; label = @2
          local.get 2
          i32.load
          local.set 6
          block ;; label = @3
            local.get 1
            i32.load offset=4
            local.get 7
            i32.sub
            local.get 2
            i32.const 4
            i32.add
            i32.load
            local.tee 5
            i32.ge_u
            br_if 0 (;@3;)
            local.get 1
            local.get 7
            local.get 5
            call $alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle
            local.get 1
            i32.load offset=8
            local.set 7
          end
          local.get 1
          i32.load
          local.get 7
          i32.add
          local.get 6
          local.get 5
          call $memcpy
          drop
          local.get 1
          local.get 7
          local.get 5
          i32.add
          local.tee 7
          i32.store offset=8
          local.get 2
          i32.const 8
          i32.add
          local.tee 2
          local.get 8
          i32.ne
          br_if 0 (;@2;)
        end
      end
      local.get 0
      i32.const 4
      i32.store8
      local.get 0
      local.get 4
      i32.store offset=4
    )
    (func $std::io::impls::<impl std::io::Write for alloc::vec::Vec<u8,A>>::is_write_vectored (;103;) (type 3) (param i32) (result i32)
      i32.const 1
    )
    (func $std::io::impls::<impl std::io::Write for alloc::vec::Vec<u8,A>>::write_all (;104;) (type 12) (param i32 i32 i32 i32)
      (local i32)
      block ;; label = @1
        local.get 1
        i32.load offset=4
        local.get 1
        i32.load offset=8
        local.tee 4
        i32.sub
        local.get 3
        i32.ge_u
        br_if 0 (;@1;)
        local.get 1
        local.get 4
        local.get 3
        call $alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle
        local.get 1
        i32.load offset=8
        local.set 4
      end
      local.get 1
      i32.load
      local.get 4
      i32.add
      local.get 2
      local.get 3
      call $memcpy
      drop
      local.get 0
      i32.const 4
      i32.store8
      local.get 1
      local.get 4
      local.get 3
      i32.add
      i32.store offset=8
    )
    (func $std::io::impls::<impl std::io::Write for alloc::vec::Vec<u8,A>>::flush (;105;) (type 2) (param i32 i32)
      local.get 0
      i32.const 4
      i32.store8
    )
    (func $std::io::Write::write_all_vectored (;106;) (type 12) (param i32 i32 i32 i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 3
            i32.eqz
            br_if 0 (;@3;)
            local.get 2
            i32.const 4
            i32.add
            local.set 5
            local.get 3
            i32.const 3
            i32.shl
            local.set 6
            local.get 3
            i32.const -1
            i32.add
            i32.const 536870911
            i32.and
            i32.const 1
            i32.add
            local.set 7
            i32.const 0
            local.set 8
            block ;; label = @4
              loop ;; label = @5
                local.get 5
                i32.load
                br_if 1 (;@4;)
                local.get 5
                i32.const 8
                i32.add
                local.set 5
                local.get 8
                i32.const 1
                i32.add
                local.set 8
                local.get 6
                i32.const -8
                i32.add
                local.tee 6
                br_if 0 (;@5;)
              end
              local.get 7
              local.set 8
            end
            block ;; label = @4
              local.get 3
              local.get 8
              i32.ge_u
              br_if 0 (;@4;)
              local.get 8
              local.get 3
              i32.const 1049508
              call $core::slice::index::slice_start_index_len_fail
              unreachable
            end
            local.get 3
            local.get 8
            i32.sub
            local.tee 9
            i32.eqz
            br_if 0 (;@3;)
            local.get 2
            local.get 8
            i32.const 3
            i32.shl
            i32.add
            local.set 10
            loop ;; label = @4
              local.get 4
              i32.const 8
              i32.add
              i32.const 2
              local.get 10
              local.get 9
              call $wasi::lib_generated::fd_write
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    local.get 4
                    i32.load16_u offset=8
                    br_if 0 (;@7;)
                    block ;; label = @8
                      local.get 4
                      i32.load offset=12
                      local.tee 7
                      br_if 0 (;@8;)
                      local.get 0
                      i32.const 1049708
                      i32.store offset=4
                      local.get 0
                      i32.const 2
                      i32.store8
                      br 7 (;@1;)
                    end
                    local.get 10
                    i32.const 4
                    i32.add
                    local.set 5
                    local.get 9
                    i32.const 3
                    i32.shl
                    local.set 2
                    local.get 9
                    i32.const -1
                    i32.add
                    i32.const 536870911
                    i32.and
                    i32.const 1
                    i32.add
                    local.set 11
                    i32.const 0
                    local.set 8
                    i32.const 0
                    local.set 6
                    loop ;; label = @8
                      local.get 5
                      i32.load
                      local.get 6
                      i32.add
                      local.tee 3
                      local.get 7
                      i32.gt_u
                      br_if 2 (;@6;)
                      local.get 5
                      i32.const 8
                      i32.add
                      local.set 5
                      local.get 8
                      i32.const 1
                      i32.add
                      local.set 8
                      local.get 3
                      local.set 6
                      local.get 2
                      i32.const -8
                      i32.add
                      local.tee 2
                      br_if 0 (;@8;)
                    end
                    local.get 3
                    local.set 6
                    local.get 11
                    local.set 8
                    br 1 (;@6;)
                  end
                  local.get 4
                  local.get 4
                  i32.load16_u offset=10
                  i32.store16 offset=6
                  local.get 4
                  i32.const 6
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
                  local.get 0
                  local.get 5
                  i32.store offset=4
                  local.get 0
                  i32.const 0
                  i32.store
                  br 5 (;@1;)
                end
                local.get 9
                local.get 8
                i32.lt_u
                br_if 3 (;@2;)
                local.get 9
                local.get 8
                i32.sub
                local.set 3
                local.get 10
                local.get 8
                i32.const 3
                i32.shl
                local.tee 2
                i32.add
                local.set 5
                block ;; label = @6
                  local.get 9
                  local.get 8
                  i32.ne
                  br_if 0 (;@6;)
                  local.get 5
                  local.set 10
                  local.get 3
                  local.set 9
                  local.get 7
                  local.get 6
                  i32.eq
                  br_if 1 (;@5;)
                  local.get 4
                  i32.const 20
                  i32.add
                  i64.const 0
                  i64.store align=4
                  local.get 4
                  i32.const 1
                  i32.store offset=12
                  local.get 4
                  i32.const 1049564
                  i32.store offset=8
                  local.get 4
                  i32.const 1048764
                  i32.store offset=16
                  local.get 4
                  i32.const 8
                  i32.add
                  i32.const 1049572
                  call $core::panicking::panic_fmt
                  unreachable
                end
                block ;; label = @6
                  local.get 10
                  local.get 2
                  i32.add
                  local.tee 2
                  i32.load offset=4
                  local.tee 9
                  local.get 7
                  local.get 6
                  i32.sub
                  local.tee 8
                  i32.lt_u
                  br_if 0 (;@6;)
                  local.get 2
                  i32.const 4
                  i32.add
                  local.get 9
                  local.get 8
                  i32.sub
                  i32.store
                  local.get 5
                  local.get 5
                  i32.load
                  local.get 8
                  i32.add
                  i32.store
                  local.get 5
                  local.set 10
                  local.get 3
                  local.set 9
                  br 1 (;@5;)
                end
                local.get 4
                i32.const 20
                i32.add
                i64.const 0
                i64.store align=4
                local.get 4
                i32.const 1
                i32.store offset=12
                local.get 4
                i32.const 1049624
                i32.store offset=8
                local.get 4
                i32.const 1048764
                i32.store offset=16
                local.get 4
                i32.const 8
                i32.add
                i32.const 1049664
                call $core::panicking::panic_fmt
                unreachable
              end
              local.get 9
              br_if 0 (;@4;)
            end
          end
          local.get 0
          i32.const 4
          i32.store8
          br 1 (;@1;)
        end
        local.get 8
        local.get 9
        i32.const 1049508
        call $core::slice::index::slice_start_index_len_fail
        unreachable
      end
      local.get 4
      i32.const 32
      i32.add
      global.set $__stack_pointer
    )
    (func $std::io::Write::write_all (;107;) (type 12) (param i32 i32 i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 3
            i32.eqz
            br_if 0 (;@3;)
            loop ;; label = @4
              local.get 4
              local.get 3
              i32.store offset=16
              local.get 4
              local.get 2
              i32.store offset=12
              local.get 4
              i32.const 20
              i32.add
              i32.const 2
              local.get 4
              i32.const 12
              i32.add
              i32.const 1
              call $wasi::lib_generated::fd_write
              block ;; label = @5
                block ;; label = @6
                  local.get 4
                  i32.load16_u offset=20
                  br_if 0 (;@6;)
                  block ;; label = @7
                    local.get 4
                    i32.load offset=24
                    local.tee 5
                    br_if 0 (;@7;)
                    local.get 0
                    i32.const 1049708
                    i32.store offset=4
                    local.get 0
                    i32.const 2
                    i32.store8
                    br 6 (;@1;)
                  end
                  block ;; label = @7
                    local.get 3
                    local.get 5
                    i32.lt_u
                    br_if 0 (;@7;)
                    local.get 2
                    local.get 5
                    i32.add
                    local.set 2
                    local.get 3
                    local.get 5
                    i32.sub
                    local.set 3
                    br 2 (;@5;)
                  end
                  local.get 5
                  local.get 3
                  i32.const 1049720
                  call $core::slice::index::slice_start_index_len_fail
                  unreachable
                end
                local.get 4
                local.get 4
                i32.load16_u offset=22
                i32.store16 offset=30
                local.get 4
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
                i32.ne
                br_if 3 (;@2;)
              end
              local.get 3
              br_if 0 (;@4;)
            end
          end
          local.get 0
          i32.const 4
          i32.store8
          br 1 (;@1;)
        end
        local.get 0
        local.get 5
        i32.store offset=4
        local.get 0
        i32.const 0
        i32.store
      end
      local.get 4
      i32.const 32
      i32.add
      global.set $__stack_pointer
    )
    (func $std::io::Write::write_all_vectored (;108;) (type 12) (param i32 i32 i32 i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 3
              i32.eqz
              br_if 0 (;@4;)
              local.get 2
              i32.const 4
              i32.add
              local.set 5
              local.get 3
              i32.const 3
              i32.shl
              local.set 6
              local.get 3
              i32.const -1
              i32.add
              i32.const 536870911
              i32.and
              i32.const 1
              i32.add
              local.set 7
              i32.const 0
              local.set 8
              block ;; label = @5
                loop ;; label = @6
                  local.get 5
                  i32.load
                  br_if 1 (;@5;)
                  local.get 5
                  i32.const 8
                  i32.add
                  local.set 5
                  local.get 8
                  i32.const 1
                  i32.add
                  local.set 8
                  local.get 6
                  i32.const -8
                  i32.add
                  local.tee 6
                  br_if 0 (;@6;)
                end
                local.get 7
                local.set 8
              end
              block ;; label = @5
                local.get 3
                local.get 8
                i32.ge_u
                br_if 0 (;@5;)
                local.get 8
                local.get 3
                i32.const 1049508
                call $core::slice::index::slice_start_index_len_fail
                unreachable
              end
              local.get 3
              local.get 8
              i32.sub
              local.tee 9
              i32.eqz
              br_if 0 (;@4;)
              local.get 2
              local.get 8
              i32.const 3
              i32.shl
              i32.add
              local.set 10
              loop ;; label = @5
                i32.const 0
                local.set 3
                i32.const 0
                local.set 6
                block ;; label = @6
                  local.get 9
                  i32.const -1
                  i32.add
                  local.tee 11
                  i32.const 3
                  i32.lt_u
                  br_if 0 (;@6;)
                  local.get 10
                  i32.const 28
                  i32.add
                  local.set 5
                  local.get 9
                  i32.const -4
                  i32.and
                  local.set 8
                  i32.const 0
                  local.set 3
                  i32.const 0
                  local.set 6
                  loop ;; label = @7
                    local.get 5
                    i32.load
                    local.get 5
                    i32.const -8
                    i32.add
                    i32.load
                    local.get 5
                    i32.const -16
                    i32.add
                    i32.load
                    local.get 5
                    i32.const -24
                    i32.add
                    i32.load
                    local.get 3
                    i32.add
                    i32.add
                    i32.add
                    i32.add
                    local.set 3
                    local.get 5
                    i32.const 32
                    i32.add
                    local.set 5
                    local.get 8
                    local.get 6
                    i32.const 4
                    i32.add
                    local.tee 6
                    i32.ne
                    br_if 0 (;@7;)
                  end
                end
                block ;; label = @6
                  local.get 9
                  i32.const 3
                  i32.and
                  local.tee 8
                  i32.eqz
                  br_if 0 (;@6;)
                  local.get 10
                  local.get 6
                  i32.const 3
                  i32.shl
                  i32.add
                  i32.const 4
                  i32.add
                  local.set 5
                  loop ;; label = @7
                    local.get 5
                    i32.load
                    local.get 3
                    i32.add
                    local.set 3
                    local.get 5
                    i32.const 8
                    i32.add
                    local.set 5
                    local.get 8
                    i32.const -1
                    i32.add
                    local.tee 8
                    br_if 0 (;@7;)
                  end
                end
                block ;; label = @6
                  local.get 1
                  i32.load offset=4
                  local.get 1
                  i32.load offset=8
                  local.tee 5
                  i32.sub
                  local.get 3
                  i32.ge_u
                  br_if 0 (;@6;)
                  local.get 1
                  local.get 5
                  local.get 3
                  call $alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle
                  local.get 1
                  i32.load offset=8
                  local.set 5
                end
                local.get 10
                local.get 9
                i32.const 3
                i32.shl
                local.tee 12
                i32.add
                local.set 7
                local.get 10
                local.set 8
                loop ;; label = @6
                  local.get 8
                  i32.load
                  local.set 2
                  block ;; label = @7
                    local.get 1
                    i32.load offset=4
                    local.get 5
                    i32.sub
                    local.get 8
                    i32.const 4
                    i32.add
                    i32.load
                    local.tee 6
                    i32.ge_u
                    br_if 0 (;@7;)
                    local.get 1
                    local.get 5
                    local.get 6
                    call $alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle
                    local.get 1
                    i32.load offset=8
                    local.set 5
                  end
                  local.get 1
                  i32.load
                  local.get 5
                  i32.add
                  local.get 2
                  local.get 6
                  call $memcpy
                  drop
                  local.get 1
                  local.get 5
                  local.get 6
                  i32.add
                  local.tee 5
                  i32.store offset=8
                  local.get 8
                  i32.const 8
                  i32.add
                  local.tee 8
                  local.get 7
                  i32.ne
                  br_if 0 (;@6;)
                end
                block ;; label = @6
                  local.get 3
                  br_if 0 (;@6;)
                  local.get 0
                  i32.const 1049708
                  i32.store offset=4
                  local.get 0
                  i32.const 2
                  i32.store8
                  br 5 (;@1;)
                end
                local.get 10
                i32.const 4
                i32.add
                local.set 5
                local.get 11
                i32.const 536870911
                i32.and
                i32.const 1
                i32.add
                local.set 7
                i32.const 0
                local.set 8
                i32.const 0
                local.set 6
                block ;; label = @6
                  loop ;; label = @7
                    local.get 5
                    i32.load
                    local.get 6
                    i32.add
                    local.tee 2
                    local.get 3
                    i32.gt_u
                    br_if 1 (;@6;)
                    local.get 5
                    i32.const 8
                    i32.add
                    local.set 5
                    local.get 8
                    i32.const 1
                    i32.add
                    local.set 8
                    local.get 2
                    local.set 6
                    local.get 12
                    i32.const -8
                    i32.add
                    local.tee 12
                    br_if 0 (;@7;)
                  end
                  local.get 2
                  local.set 6
                  local.get 7
                  local.set 8
                end
                local.get 9
                local.get 8
                i32.lt_u
                br_if 2 (;@3;)
                local.get 10
                local.get 8
                i32.const 3
                i32.shl
                local.tee 2
                i32.add
                local.set 5
                block ;; label = @6
                  block ;; label = @7
                    local.get 9
                    local.get 8
                    i32.ne
                    br_if 0 (;@7;)
                    local.get 3
                    local.get 6
                    i32.eq
                    br_if 1 (;@6;)
                    local.get 4
                    i32.const 20
                    i32.add
                    i64.const 0
                    i64.store align=4
                    local.get 4
                    i32.const 1
                    i32.store offset=12
                    local.get 4
                    i32.const 1049564
                    i32.store offset=8
                    local.get 4
                    i32.const 1048764
                    i32.store offset=16
                    local.get 4
                    i32.const 8
                    i32.add
                    i32.const 1049572
                    call $core::panicking::panic_fmt
                    unreachable
                  end
                  local.get 10
                  local.get 2
                  i32.add
                  local.tee 2
                  i32.load offset=4
                  local.tee 7
                  local.get 3
                  local.get 6
                  i32.sub
                  local.tee 6
                  i32.lt_u
                  br_if 4 (;@2;)
                  local.get 2
                  i32.const 4
                  i32.add
                  local.get 7
                  local.get 6
                  i32.sub
                  i32.store
                  local.get 5
                  local.get 5
                  i32.load
                  local.get 6
                  i32.add
                  i32.store
                end
                local.get 5
                local.set 10
                local.get 9
                local.get 8
                i32.sub
                local.tee 9
                br_if 0 (;@5;)
              end
            end
            local.get 0
            i32.const 4
            i32.store8
            br 2 (;@1;)
          end
          local.get 8
          local.get 9
          i32.const 1049508
          call $core::slice::index::slice_start_index_len_fail
          unreachable
        end
        local.get 4
        i32.const 20
        i32.add
        i64.const 0
        i64.store align=4
        local.get 4
        i32.const 1
        i32.store offset=12
        local.get 4
        i32.const 1049624
        i32.store offset=8
        local.get 4
        i32.const 1048764
        i32.store offset=16
        local.get 4
        i32.const 8
        i32.add
        i32.const 1049664
        call $core::panicking::panic_fmt
        unreachable
      end
      local.get 4
      i32.const 32
      i32.add
      global.set $__stack_pointer
    )
    (func $std::io::Write::write_all_vectored (;109;) (type 12) (param i32 i32 i32 i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 3
            i32.eqz
            br_if 0 (;@3;)
            local.get 2
            i32.const 4
            i32.add
            local.set 5
            local.get 3
            i32.const 3
            i32.shl
            local.set 6
            local.get 3
            i32.const -1
            i32.add
            i32.const 536870911
            i32.and
            i32.const 1
            i32.add
            local.set 7
            i32.const 0
            local.set 8
            block ;; label = @4
              loop ;; label = @5
                local.get 5
                i32.load
                br_if 1 (;@4;)
                local.get 5
                i32.const 8
                i32.add
                local.set 5
                local.get 8
                i32.const 1
                i32.add
                local.set 8
                local.get 6
                i32.const -8
                i32.add
                local.tee 6
                br_if 0 (;@5;)
              end
              local.get 7
              local.set 8
            end
            block ;; label = @4
              local.get 3
              local.get 8
              i32.ge_u
              br_if 0 (;@4;)
              local.get 8
              local.get 3
              i32.const 1049508
              call $core::slice::index::slice_start_index_len_fail
              unreachable
            end
            local.get 3
            local.get 8
            i32.sub
            local.tee 7
            i32.eqz
            br_if 0 (;@3;)
            local.get 2
            local.get 8
            i32.const 3
            i32.shl
            i32.add
            local.set 9
            local.get 1
            i32.load
            local.set 10
            loop ;; label = @4
              local.get 4
              i32.const 8
              i32.add
              local.get 10
              local.get 9
              local.get 7
              call $wasi::lib_generated::fd_write
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    local.get 4
                    i32.load16_u offset=8
                    br_if 0 (;@7;)
                    block ;; label = @8
                      local.get 4
                      i32.load offset=12
                      local.tee 1
                      br_if 0 (;@8;)
                      local.get 0
                      i32.const 1049708
                      i32.store offset=4
                      local.get 0
                      i32.const 2
                      i32.store8
                      br 7 (;@1;)
                    end
                    local.get 9
                    i32.const 4
                    i32.add
                    local.set 5
                    local.get 7
                    i32.const 3
                    i32.shl
                    local.set 2
                    local.get 7
                    i32.const -1
                    i32.add
                    i32.const 536870911
                    i32.and
                    i32.const 1
                    i32.add
                    local.set 11
                    i32.const 0
                    local.set 8
                    i32.const 0
                    local.set 6
                    loop ;; label = @8
                      local.get 5
                      i32.load
                      local.get 6
                      i32.add
                      local.tee 3
                      local.get 1
                      i32.gt_u
                      br_if 2 (;@6;)
                      local.get 5
                      i32.const 8
                      i32.add
                      local.set 5
                      local.get 8
                      i32.const 1
                      i32.add
                      local.set 8
                      local.get 3
                      local.set 6
                      local.get 2
                      i32.const -8
                      i32.add
                      local.tee 2
                      br_if 0 (;@8;)
                    end
                    local.get 3
                    local.set 6
                    local.get 11
                    local.set 8
                    br 1 (;@6;)
                  end
                  local.get 4
                  local.get 4
                  i32.load16_u offset=10
                  i32.store16 offset=6
                  local.get 4
                  i32.const 6
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
                  local.get 0
                  local.get 5
                  i32.store offset=4
                  local.get 0
                  i32.const 0
                  i32.store
                  br 5 (;@1;)
                end
                local.get 7
                local.get 8
                i32.lt_u
                br_if 3 (;@2;)
                local.get 7
                local.get 8
                i32.sub
                local.set 3
                local.get 9
                local.get 8
                i32.const 3
                i32.shl
                local.tee 2
                i32.add
                local.set 5
                block ;; label = @6
                  local.get 7
                  local.get 8
                  i32.ne
                  br_if 0 (;@6;)
                  local.get 5
                  local.set 9
                  local.get 3
                  local.set 7
                  local.get 1
                  local.get 6
                  i32.eq
                  br_if 1 (;@5;)
                  local.get 4
                  i32.const 20
                  i32.add
                  i64.const 0
                  i64.store align=4
                  local.get 4
                  i32.const 1
                  i32.store offset=12
                  local.get 4
                  i32.const 1049564
                  i32.store offset=8
                  local.get 4
                  i32.const 1048764
                  i32.store offset=16
                  local.get 4
                  i32.const 8
                  i32.add
                  i32.const 1049572
                  call $core::panicking::panic_fmt
                  unreachable
                end
                block ;; label = @6
                  local.get 9
                  local.get 2
                  i32.add
                  local.tee 2
                  i32.load offset=4
                  local.tee 7
                  local.get 1
                  local.get 6
                  i32.sub
                  local.tee 8
                  i32.lt_u
                  br_if 0 (;@6;)
                  local.get 2
                  i32.const 4
                  i32.add
                  local.get 7
                  local.get 8
                  i32.sub
                  i32.store
                  local.get 5
                  local.get 5
                  i32.load
                  local.get 8
                  i32.add
                  i32.store
                  local.get 5
                  local.set 9
                  local.get 3
                  local.set 7
                  br 1 (;@5;)
                end
                local.get 4
                i32.const 20
                i32.add
                i64.const 0
                i64.store align=4
                local.get 4
                i32.const 1
                i32.store offset=12
                local.get 4
                i32.const 1049624
                i32.store offset=8
                local.get 4
                i32.const 1048764
                i32.store offset=16
                local.get 4
                i32.const 8
                i32.add
                i32.const 1049664
                call $core::panicking::panic_fmt
                unreachable
              end
              local.get 7
              br_if 0 (;@4;)
            end
          end
          local.get 0
          i32.const 4
          i32.store8
          br 1 (;@1;)
        end
        local.get 8
        local.get 7
        i32.const 1049508
        call $core::slice::index::slice_start_index_len_fail
        unreachable
      end
      local.get 4
      i32.const 32
      i32.add
      global.set $__stack_pointer
    )
    (func $std::io::Write::write_fmt (;110;) (type 4) (param i32 i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      i32.const 4
      i32.store8
      local.get 3
      local.get 1
      i32.store offset=8
      block ;; label = @1
        block ;; label = @2
          local.get 3
          i32.const 1049736
          local.get 2
          call $core::fmt::write
          i32.eqz
          br_if 0 (;@2;)
          block ;; label = @3
            local.get 3
            i32.load8_u
            i32.const 4
            i32.ne
            br_if 0 (;@3;)
            local.get 0
            i32.const 1049776
            i32.store offset=4
            local.get 0
            i32.const 2
            i32.store8
            br 2 (;@1;)
          end
          local.get 0
          local.get 3
          i64.load
          i64.store align=4
          br 1 (;@1;)
        end
        local.get 0
        i32.const 4
        i32.store8
        local.get 3
        i32.load offset=4
        local.set 1
        block ;; label = @2
          local.get 3
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
        call_indirect (type $.data)
        block ;; label = @2
          local.get 0
          i32.load offset=4
          local.tee 4
          i32.eqz
          br_if 0 (;@2;)
          local.get 2
          local.get 4
          local.get 0
          i32.load offset=8
          call $__rust_dealloc
        end
        local.get 1
        i32.const 12
        i32.const 4
        call $__rust_dealloc
      end
      local.get 3
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $std::io::Write::write_fmt (;111;) (type 4) (param i32 i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      i32.const 4
      i32.store8
      local.get 3
      local.get 1
      i32.store offset=8
      block ;; label = @1
        block ;; label = @2
          local.get 3
          i32.const 1049788
          local.get 2
          call $core::fmt::write
          i32.eqz
          br_if 0 (;@2;)
          block ;; label = @3
            local.get 3
            i32.load8_u
            i32.const 4
            i32.ne
            br_if 0 (;@3;)
            local.get 0
            i32.const 1049776
            i32.store offset=4
            local.get 0
            i32.const 2
            i32.store8
            br 2 (;@1;)
          end
          local.get 0
          local.get 3
          i64.load
          i64.store align=4
          br 1 (;@1;)
        end
        local.get 0
        i32.const 4
        i32.store8
        local.get 3
        i32.load offset=4
        local.set 1
        block ;; label = @2
          local.get 3
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
        call_indirect (type $.data)
        block ;; label = @2
          local.get 0
          i32.load offset=4
          local.tee 4
          i32.eqz
          br_if 0 (;@2;)
          local.get 2
          local.get 4
          local.get 0
          i32.load offset=8
          call $__rust_dealloc
        end
        local.get 1
        i32.const 12
        i32.const 4
        call $__rust_dealloc
      end
      local.get 3
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $<std::io::Write::write_fmt::Adapter<T> as core::fmt::Write>::write_str (;112;) (type 5) (param i32 i32 i32) (result i32)
      (local i32)
      block ;; label = @1
        local.get 0
        i32.load offset=8
        local.tee 0
        i32.load offset=4
        local.get 0
        i32.load offset=8
        local.tee 3
        i32.sub
        local.get 2
        i32.ge_u
        br_if 0 (;@1;)
        local.get 0
        local.get 3
        local.get 2
        call $alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle
        local.get 0
        i32.load offset=8
        local.set 3
      end
      local.get 0
      i32.load
      local.get 3
      i32.add
      local.get 1
      local.get 2
      call $memcpy
      drop
      local.get 0
      local.get 3
      local.get 2
      i32.add
      i32.store offset=8
      i32.const 0
    )
    (func $std::panic::get_backtrace_style (;113;) (type 13) (result i32)
      (local i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 0
      global.set $__stack_pointer
      i32.const 0
      local.set 1
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                i32.const 0
                i32.load offset=1056348
                br_table 3 (;@2;) 4 (;@1;) 2 (;@3;) 1 (;@4;) 0 (;@5;)
              end
              i32.const 1048935
              i32.const 40
              i32.const 1049924
              call $core::panicking::panic
              unreachable
            end
            i32.const 2
            local.set 1
            br 2 (;@1;)
          end
          i32.const 1
          local.set 1
          br 1 (;@1;)
        end
        local.get 0
        i32.const 4
        i32.add
        i32.const 1049336
        i32.const 14
        call $std::env::_var_os
        block ;; label = @2
          block ;; label = @3
            local.get 0
            i32.load offset=4
            local.tee 2
            i32.eqz
            br_if 0 (;@3;)
            i32.const 0
            local.set 1
            local.get 0
            i32.load offset=8
            local.set 3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  local.get 0
                  i32.load offset=12
                  i32.const -1
                  i32.add
                  br_table 0 (;@6;) 2 (;@4;) 2 (;@4;) 1 (;@5;) 2 (;@4;)
                end
                local.get 2
                i32.load8_u
                i32.const 48
                i32.eq
                i32.const 1
                i32.shl
                local.set 1
                br 1 (;@4;)
              end
              local.get 2
              i32.const 1049940
              i32.const 4
              call $memcmp
              i32.eqz
              local.set 1
            end
            block ;; label = @4
              local.get 3
              i32.eqz
              br_if 0 (;@4;)
              local.get 2
              local.get 3
              i32.const 1
              call $__rust_dealloc
            end
            local.get 1
            i32.const 1
            i32.add
            local.set 2
            br 1 (;@2;)
          end
          i32.const 3
          local.set 2
          i32.const 2
          local.set 1
        end
        i32.const 0
        local.get 2
        i32.store offset=1056348
      end
      local.get 0
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 1
    )
    (func $<std::path::Display as core::fmt::Display>::fmt (;114;) (type 6) (param i32 i32) (result i32)
      local.get 0
      i32.load
      local.get 0
      i32.load offset=4
      local.get 1
      call $<std::sys::wasi::os_str::Slice as core::fmt::Display>::fmt
    )
    (func $<std::sys::wasi::os_str::Slice as core::fmt::Display>::fmt (;115;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 64
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          local.get 1
          br_if 0 (;@2;)
          i32.const 1048764
          i32.const 0
          local.get 2
          call $<str as core::fmt::Display>::fmt
          local.set 4
          br 1 (;@1;)
        end
        local.get 3
        i32.const 16
        i32.add
        local.get 0
        local.get 1
        call $core::str::lossy::Utf8Chunks::new
        local.get 3
        local.get 3
        i64.load offset=16
        i64.store offset=24 align=4
        local.get 3
        i32.const 32
        i32.add
        local.get 3
        i32.const 24
        i32.add
        call $<core::str::lossy::Utf8Chunks as core::iter::traits::iterator::Iterator>::next
        block ;; label = @2
          local.get 3
          i32.load offset=32
          i32.eqz
          br_if 0 (;@2;)
          local.get 3
          i32.const 48
          i32.add
          i32.const 8
          i32.add
          local.set 5
          loop ;; label = @3
            local.get 5
            local.get 3
            i32.const 32
            i32.add
            i32.const 8
            i32.add
            i64.load align=4
            i64.store
            local.get 3
            local.get 3
            i64.load offset=32 align=4
            i64.store offset=48
            local.get 3
            i32.const 8
            i32.add
            local.get 3
            i32.const 48
            i32.add
            call $core::str::lossy::Utf8Chunk::valid
            local.get 3
            i32.load offset=12
            local.set 1
            local.get 3
            i32.load offset=8
            local.set 0
            local.get 3
            local.get 3
            i32.const 48
            i32.add
            call $core::str::lossy::Utf8Chunk::invalid
            block ;; label = @4
              local.get 3
              i32.load offset=4
              br_if 0 (;@4;)
              local.get 0
              local.get 1
              local.get 2
              call $<str as core::fmt::Display>::fmt
              local.set 4
              br 3 (;@1;)
            end
            i32.const 1
            local.set 4
            local.get 2
            local.get 0
            local.get 1
            call $core::fmt::Formatter::write_str
            br_if 2 (;@1;)
            local.get 2
            i32.const 65533
            call $<core::fmt::Formatter as core::fmt::Write>::write_char
            br_if 2 (;@1;)
            local.get 3
            i32.const 32
            i32.add
            local.get 3
            i32.const 24
            i32.add
            call $<core::str::lossy::Utf8Chunks as core::iter::traits::iterator::Iterator>::next
            local.get 3
            i32.load offset=32
            br_if 0 (;@3;)
          end
        end
        i32.const 0
        local.set 4
      end
      local.get 3
      i32.const 64
      i32.add
      global.set $__stack_pointer
      local.get 4
    )
    (func $std::process::abort (;116;) (type $.rodata)
      call $std::sys::wasi::abort_internal
      unreachable
    )
    (func $std::sync::once_lock::OnceLock<T>::initialize (;117;) (type $.data) (param i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      block ;; label = @1
        i32.const 0
        i32.load8_u offset=1056344
        i32.const 3
        i32.eq
        br_if 0 (;@1;)
        local.get 1
        i32.const 1056312
        i32.store offset=4
        local.get 1
        local.get 0
        i32.store
        local.get 1
        local.get 1
        i32.const 15
        i32.add
        i32.store offset=8
        local.get 1
        call $std::sys::wasi::once::Once::call
      end
      local.get 1
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $std::sys::wasi::once::Once::call (;118;) (type $.data) (param i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              i32.const 0
              i32.load8_u offset=1056344
              local.tee 2
              i32.const 2
              i32.lt_u
              br_if 0 (;@4;)
              local.get 2
              i32.const -2
              i32.add
              br_table 3 (;@1;) 1 (;@3;) 3 (;@1;)
            end
            i32.const 0
            i32.const 2
            i32.store8 offset=1056344
            local.get 0
            i32.load
            local.set 2
            local.get 0
            i32.const 0
            i32.store
            local.get 2
            i32.eqz
            br_if 1 (;@2;)
            local.get 0
            i32.load offset=4
            local.set 0
            local.get 2
            i32.const 1
            i32.store8
            local.get 0
            i64.const 0
            i64.store offset=16 align=4
            local.get 0
            i64.const 4294967296
            i64.store offset=8 align=4
            local.get 0
            i64.const 0
            i64.store align=4
            local.get 0
            i32.const 24
            i32.add
            i32.const 0
            i32.store8
            local.get 0
            i32.const 0
            i32.store8 offset=28
            i32.const 0
            i32.const 3
            i32.store8 offset=1056344
          end
          local.get 1
          i32.const 32
          i32.add
          global.set $__stack_pointer
          return
        end
        i32.const 1048892
        i32.const 43
        i32.const 1050116
        call $core::panicking::panic
        unreachable
      end
      local.get 1
      i32.const 20
      i32.add
      i64.const 0
      i64.store align=4
      local.get 1
      i32.const 1
      i32.store offset=12
      local.get 1
      i32.const 1051828
      i32.store offset=8
      local.get 1
      i32.const 1048764
      i32.store offset=16
      local.get 1
      i32.const 8
      i32.add
      i32.const 1050100
      call $core::panicking::panic_fmt
      unreachable
    )
    (func $std::sys::common::small_c_string::run_with_cstr_allocating (;119;) (type 4) (param i32 i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      local.get 1
      local.get 2
      call $<&str as alloc::ffi::c_str::CString::new::SpecNewImpl>::spec_new_impl
      block ;; label = @1
        block ;; label = @2
          local.get 3
          i32.load
          local.tee 2
          br_if 0 (;@2;)
          local.get 0
          local.get 3
          i32.load offset=4
          local.tee 1
          local.get 3
          i32.const 8
          i32.add
          i32.load
          local.tee 2
          call $std::sys::wasi::fs::open_parent::{{closure}}
          local.get 1
          i32.const 0
          i32.store8
          local.get 2
          i32.eqz
          br_if 1 (;@1;)
          local.get 1
          local.get 2
          i32.const 1
          call $__rust_dealloc
          br 1 (;@1;)
        end
        local.get 0
        i32.const -1
        i32.store
        local.get 0
        i32.const 0
        i64.load offset=1050280
        i64.store offset=4 align=4
        local.get 3
        i32.load offset=4
        local.tee 0
        i32.eqz
        br_if 0 (;@1;)
        local.get 2
        local.get 0
        i32.const 1
        call $__rust_dealloc
      end
      local.get 3
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $std::sys::wasi::fs::open_parent::{{closure}} (;120;) (type 4) (param i32 i32 i32)
      (local i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 80
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      local.get 2
      i32.store offset=8
      local.get 3
      local.get 1
      i32.store offset=4
      i32.const 0
      i32.load8_u offset=1056309
      drop
      i32.const 512
      local.set 2
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  i32.const 512
                  i32.const 1
                  call $__rust_alloc
                  local.tee 4
                  i32.eqz
                  br_if 0 (;@6;)
                  local.get 3
                  i64.const 512
                  i64.store offset=16 align=4
                  local.get 3
                  local.get 4
                  i32.store offset=12
                  local.get 3
                  local.get 4
                  i32.store offset=24
                  local.get 3
                  i32.const 0
                  i32.store offset=28
                  local.get 1
                  local.get 3
                  i32.const 28
                  i32.add
                  local.get 3
                  i32.const 24
                  i32.add
                  i32.const 512
                  call $__wasilibc_find_relpath
                  local.tee 5
                  i32.const -1
                  i32.ne
                  br_if 2 (;@4;)
                  i32.const 512
                  local.set 2
                  i32.const 0
                  i32.load offset=1056916
                  i32.const 48
                  i32.ne
                  br_if 1 (;@5;)
                  i32.const 512
                  local.set 2
                  loop ;; label = @7
                    local.get 3
                    local.get 2
                    i32.store offset=20
                    local.get 3
                    i32.const 12
                    i32.add
                    local.get 2
                    i32.const 1
                    call $alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle
                    local.get 3
                    local.get 3
                    i32.load offset=12
                    local.tee 4
                    i32.store offset=24
                    local.get 3
                    i32.const 0
                    i32.store offset=28
                    local.get 1
                    local.get 3
                    i32.const 28
                    i32.add
                    local.get 3
                    i32.const 24
                    i32.add
                    local.get 3
                    i32.load offset=16
                    local.tee 2
                    call $__wasilibc_find_relpath
                    local.tee 5
                    i32.const -1
                    i32.ne
                    br_if 3 (;@4;)
                    i32.const 0
                    i32.load offset=1056916
                    i32.const 48
                    i32.ne
                    br_if 2 (;@5;)
                    br 0 (;@7;)
                  end
                end
                i32.const 1
                i32.const 512
                call $alloc::alloc::handle_alloc_error
                unreachable
              end
              local.get 3
              i32.const 32
              i32.add
              i32.const 12
              i32.add
              i64.const 1
              i64.store align=4
              local.get 3
              i32.const 2
              i32.store offset=36
              local.get 3
              i32.const 1051552
              i32.store offset=32
              local.get 3
              i32.const 7
              i32.store offset=60
              local.get 3
              local.get 3
              i32.const 56
              i32.add
              i32.store offset=40
              local.get 3
              local.get 3
              i32.const 4
              i32.add
              i32.store offset=56
              local.get 3
              i32.const 64
              i32.add
              local.get 3
              i32.const 32
              i32.add
              call $alloc::fmt::format::format_inner
              i32.const 0
              i32.load8_u offset=1056309
              drop
              block ;; label = @5
                block ;; label = @6
                  i32.const 12
                  i32.const 4
                  call $__rust_alloc
                  local.tee 5
                  i32.eqz
                  br_if 0 (;@6;)
                  local.get 5
                  local.get 3
                  i64.load offset=64
                  i64.store align=4
                  local.get 5
                  i32.const 8
                  i32.add
                  local.get 3
                  i32.const 64
                  i32.add
                  i32.const 8
                  i32.add
                  i32.load
                  i32.store
                  i32.const 0
                  i32.load8_u offset=1056309
                  drop
                  i32.const 12
                  i32.const 4
                  call $__rust_alloc
                  local.tee 1
                  i32.eqz
                  br_if 1 (;@5;)
                  local.get 1
                  i32.const 40
                  i32.store8 offset=8
                  local.get 1
                  i32.const 1051584
                  i32.store offset=4
                  local.get 1
                  local.get 5
                  i32.store
                  local.get 0
                  i32.const -1
                  i32.store
                  local.get 0
                  local.get 1
                  i64.extend_i32_u
                  i64.const 32
                  i64.shl
                  i64.const 3
                  i64.or
                  i64.store offset=4 align=4
                  br 3 (;@3;)
                end
                i32.const 4
                i32.const 12
                call $alloc::alloc::handle_alloc_error
                unreachable
              end
              i32.const 4
              i32.const 12
              call $alloc::alloc::handle_alloc_error
              unreachable
            end
            block ;; label = @4
              block ;; label = @5
                local.get 3
                i32.load offset=24
                local.tee 6
                call $strlen
                local.tee 1
                br_if 0 (;@5;)
                i32.const 1
                local.set 7
                br 1 (;@4;)
              end
              local.get 1
              i32.const -1
              i32.le_s
              br_if 2 (;@2;)
              i32.const 0
              i32.load8_u offset=1056309
              drop
              local.get 1
              i32.const 1
              call $__rust_alloc
              local.tee 7
              i32.eqz
              br_if 3 (;@1;)
            end
            local.get 7
            local.get 6
            local.get 1
            call $memcpy
            local.set 6
            local.get 0
            local.get 1
            i32.store offset=12
            local.get 0
            local.get 1
            i32.store offset=8
            local.get 0
            local.get 6
            i32.store offset=4
            local.get 0
            local.get 5
            i32.store
          end
          block ;; label = @3
            local.get 2
            i32.eqz
            br_if 0 (;@3;)
            local.get 4
            local.get 2
            i32.const 1
            call $__rust_dealloc
          end
          local.get 3
          i32.const 80
          i32.add
          global.set $__stack_pointer
          return
        end
        call $alloc::raw_vec::capacity_overflow
        unreachable
      end
      i32.const 1
      local.get 1
      call $alloc::alloc::handle_alloc_error
      unreachable
    )
    (func $std::sys_common::backtrace::print (;121;) (type 12) (param i32 i32 i32 i32)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      i32.const 0
      i32.load8_u offset=1056352
      local.set 5
      i32.const 1
      local.set 6
      i32.const 0
      i32.const 1
      i32.store8 offset=1056352
      local.get 4
      local.get 5
      i32.store8 offset=36
      block ;; label = @1
        local.get 5
        br_if 0 (;@1;)
        block ;; label = @2
          i32.const 0
          i32.load offset=1056376
          i32.const 2147483647
          i32.and
          i32.eqz
          br_if 0 (;@2;)
          call $std::panicking::panic_count::is_zero_slow_path
          local.set 6
        end
        local.get 2
        i32.const 36
        i32.add
        i32.load
        local.set 5
        local.get 4
        i32.const 24
        i32.add
        i64.const 1
        i64.store align=4
        local.get 4
        i32.const 1
        i32.store offset=16
        local.get 4
        i32.const 1049396
        i32.store offset=12
        local.get 4
        i32.const 8
        i32.store offset=40
        local.get 4
        local.get 3
        i32.store8 offset=47
        local.get 4
        local.get 4
        i32.const 36
        i32.add
        i32.store offset=20
        local.get 4
        local.get 4
        i32.const 47
        i32.add
        i32.store offset=36
        local.get 0
        local.get 1
        local.get 4
        i32.const 12
        i32.add
        local.get 5
        call_indirect (type 4)
        block ;; label = @2
          local.get 6
          i32.eqz
          br_if 0 (;@2;)
          i32.const 0
          i32.load offset=1056376
          i32.const 2147483647
          i32.and
          i32.eqz
          br_if 0 (;@2;)
          call $std::panicking::panic_count::is_zero_slow_path
          br_if 0 (;@2;)
          i32.const 0
          i32.const 1
          i32.store8 offset=1056353
        end
        i32.const 0
        i32.const 0
        i32.store8 offset=1056352
        local.get 4
        i32.const 48
        i32.add
        global.set $__stack_pointer
        return
      end
      local.get 4
      i64.const 0
      i64.store offset=24 align=4
      local.get 4
      i32.const 1048764
      i32.store offset=20
      local.get 4
      i32.const 1
      i32.store offset=16
      local.get 4
      i32.const 1049976
      i32.store offset=12
      local.get 4
      i32.const 36
      i32.add
      local.get 4
      i32.const 12
      i32.add
      call $core::panicking::assert_failed
      unreachable
    )
    (func $<std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt (;122;) (type 6) (param i32 i32) (result i32)
      (local i32 i32 i64 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 0
      i32.load8_u
      local.set 3
      local.get 2
      i32.const 8
      i32.add
      call $std::env::current_dir
      local.get 2
      i64.load offset=12 align=4
      local.set 4
      block ;; label = @1
        local.get 2
        i32.load offset=8
        local.tee 0
        br_if 0 (;@1;)
        local.get 4
        i64.const 255
        i64.and
        i64.const 3
        i64.ne
        br_if 0 (;@1;)
        local.get 4
        i64.const 32
        i64.shr_u
        i32.wrap_i64
        local.tee 5
        i32.load
        local.tee 6
        local.get 5
        i32.const 4
        i32.add
        i32.load
        local.tee 7
        i32.load
        call_indirect (type $.data)
        block ;; label = @2
          local.get 7
          i32.load offset=4
          local.tee 8
          i32.eqz
          br_if 0 (;@2;)
          local.get 6
          local.get 8
          local.get 7
          i32.load offset=8
          call $__rust_dealloc
        end
        local.get 5
        i32.const 12
        i32.const 4
        call $__rust_dealloc
      end
      local.get 2
      i32.const 20
      i32.add
      i64.const 0
      i64.store align=4
      i32.const 1
      local.set 5
      local.get 2
      i32.const 1
      i32.store offset=12
      local.get 2
      i32.const 1050308
      i32.store offset=8
      local.get 2
      i32.const 1048764
      i32.store offset=16
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 1
            local.get 2
            i32.const 8
            i32.add
            call $core::fmt::Formatter::write_fmt
            br_if 0 (;@3;)
            block ;; label = @4
              local.get 3
              i32.const 255
              i32.and
              br_if 0 (;@4;)
              local.get 2
              i32.const 20
              i32.add
              i64.const 0
              i64.store align=4
              local.get 2
              i32.const 1
              i32.store offset=12
              local.get 2
              i32.const 1050404
              i32.store offset=8
              local.get 2
              i32.const 1048764
              i32.store offset=16
              local.get 1
              local.get 2
              i32.const 8
              i32.add
              call $core::fmt::Formatter::write_fmt
              br_if 1 (;@3;)
            end
            i32.const 0
            local.set 5
            local.get 0
            i32.eqz
            br_if 2 (;@1;)
            br 1 (;@2;)
          end
          local.get 0
          i32.eqz
          br_if 1 (;@1;)
        end
        local.get 4
        i32.wrap_i64
        local.tee 1
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        local.get 1
        i32.const 1
        call $__rust_dealloc
      end
      local.get 2
      i32.const 32
      i32.add
      global.set $__stack_pointer
      local.get 5
    )
    (func $std::sys_common::backtrace::__rust_end_short_backtrace (;123;) (type $.data) (param i32)
      local.get 0
      call $std::panicking::begin_panic_handler::{{closure}}
      unreachable
    )
    (func $std::panicking::begin_panic_handler::{{closure}} (;124;) (type $.data) (param i32)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      local.get 0
      i32.load
      local.tee 2
      i32.const 12
      i32.add
      i32.load
      local.set 3
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 2
              i32.load offset=4
              br_table 0 (;@4;) 1 (;@3;) 3 (;@1;)
            end
            local.get 3
            br_if 2 (;@1;)
            i32.const 1048764
            local.set 2
            i32.const 0
            local.set 3
            br 1 (;@2;)
          end
          local.get 3
          br_if 1 (;@1;)
          local.get 2
          i32.load
          local.tee 2
          i32.load offset=4
          local.set 3
          local.get 2
          i32.load
          local.set 2
        end
        local.get 1
        local.get 3
        i32.store offset=4
        local.get 1
        local.get 2
        i32.store
        local.get 1
        i32.const 1051148
        local.get 0
        i32.load offset=4
        local.tee 2
        call $core::panic::panic_info::PanicInfo::message
        local.get 0
        i32.load offset=8
        local.get 2
        call $core::panic::panic_info::PanicInfo::can_unwind
        call $std::panicking::rust_panic_with_hook
        unreachable
      end
      local.get 1
      i32.const 0
      i32.store offset=4
      local.get 1
      local.get 2
      i32.store
      local.get 1
      i32.const 1051168
      local.get 0
      i32.load offset=4
      local.tee 2
      call $core::panic::panic_info::PanicInfo::message
      local.get 0
      i32.load offset=8
      local.get 2
      call $core::panic::panic_info::PanicInfo::can_unwind
      call $std::panicking::rust_panic_with_hook
      unreachable
    )
    (func $std::alloc::default_alloc_error_hook (;125;) (type 2) (param i32 i32)
      (local i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 64
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        i32.const 0
        i32.load8_u offset=1056308
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
        i32.const 1050596
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
        i32.const 1049812
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
            call_indirect (type $.data)
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
          call_indirect (type $.data)
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
      local.get 2
      i32.const 9
      i32.store offset=52
      local.get 2
      local.get 1
      i32.store offset=36
      local.get 2
      local.get 2
      i32.const 36
      i32.add
      i32.store offset=48
      local.get 2
      i32.const 12
      i32.add
      i32.const 1050628
      i32.const 2
      local.get 2
      i32.const 48
      i32.add
      i32.const 1
      call $core::fmt::Arguments::new_v1
      local.get 2
      i32.const 12
      i32.add
      i32.const 1050668
      call $core::panicking::panic_fmt
      unreachable
    )
    (func $__rdl_alloc (;126;) (type 6) (param i32 i32) (result i32)
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
    (func $__rdl_dealloc (;127;) (type 4) (param i32 i32 i32)
      local.get 0
      call $free
    )
    (func $__rdl_realloc (;128;) (type 10) (param i32 i32 i32 i32) (result i32)
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
    (func $std::panicking::panic_hook_with_disk_dump::{{closure}} (;129;) (type 12) (param i32 i32 i32 i32)
      (local i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 64
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      local.get 4
      i32.const 12
      i32.add
      i64.const 3
      i64.store align=4
      local.get 4
      i32.const 60
      i32.add
      i32.const 10
      i32.store
      local.get 4
      i32.const 40
      i32.add
      i32.const 12
      i32.add
      i32.const 11
      i32.store
      local.get 4
      i32.const 1050896
      i32.store
      local.get 4
      i32.const 10
      i32.store offset=44
      local.get 4
      local.get 0
      i32.load offset=8
      i32.store offset=56
      local.get 4
      local.get 0
      i32.load offset=4
      i32.store offset=48
      local.get 4
      local.get 0
      i32.load
      i32.store offset=40
      local.get 4
      local.get 4
      i32.const 40
      i32.add
      i32.store offset=8
      local.get 4
      i32.const 4
      i32.store offset=4
      local.get 4
      i32.const 32
      i32.add
      local.get 1
      local.get 4
      local.get 2
      i32.load offset=36
      local.tee 5
      call_indirect (type 4)
      local.get 4
      i32.load offset=36
      local.set 6
      block ;; label = @1
        block ;; label = @2
          local.get 4
          i32.load8_u offset=32
          local.tee 7
          i32.const 4
          i32.gt_u
          br_if 0 (;@2;)
          local.get 7
          i32.const 3
          i32.ne
          br_if 1 (;@1;)
        end
        local.get 6
        i32.load
        local.tee 8
        local.get 6
        i32.const 4
        i32.add
        i32.load
        local.tee 7
        i32.load
        call_indirect (type $.data)
        block ;; label = @2
          local.get 7
          i32.load offset=4
          local.tee 9
          i32.eqz
          br_if 0 (;@2;)
          local.get 8
          local.get 9
          local.get 7
          i32.load offset=8
          call $__rust_dealloc
        end
        local.get 6
        i32.const 12
        i32.const 4
        call $__rust_dealloc
      end
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 3
              i32.const 255
              i32.and
              br_table 0 (;@4;) 1 (;@3;) 2 (;@2;) 3 (;@1;) 0 (;@4;)
            end
            local.get 4
            i32.const 40
            i32.add
            local.get 1
            local.get 2
            i32.const 0
            call $std::sys_common::backtrace::print
            local.get 4
            i32.load offset=44
            local.set 1
            block ;; label = @4
              local.get 4
              i32.load8_u offset=40
              local.tee 0
              i32.const 4
              i32.gt_u
              br_if 0 (;@4;)
              local.get 0
              i32.const 3
              i32.ne
              br_if 3 (;@1;)
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
            call_indirect (type $.data)
            block ;; label = @4
              local.get 0
              i32.load offset=4
              local.tee 3
              i32.eqz
              br_if 0 (;@4;)
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
            br 2 (;@1;)
          end
          local.get 4
          i32.const 40
          i32.add
          local.get 1
          local.get 2
          i32.const 1
          call $std::sys_common::backtrace::print
          local.get 4
          i32.load offset=44
          local.set 1
          block ;; label = @3
            local.get 4
            i32.load8_u offset=40
            local.tee 0
            i32.const 4
            i32.gt_u
            br_if 0 (;@3;)
            local.get 0
            i32.const 3
            i32.ne
            br_if 2 (;@1;)
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
          call_indirect (type $.data)
          block ;; label = @3
            local.get 0
            i32.load offset=4
            local.tee 3
            i32.eqz
            br_if 0 (;@3;)
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
          br 1 (;@1;)
        end
        i32.const 0
        i32.load8_u offset=1056292
        local.set 2
        i32.const 0
        i32.const 0
        i32.store8 offset=1056292
        local.get 2
        i32.eqz
        br_if 0 (;@1;)
        block ;; label = @2
          local.get 0
          i32.load offset=12
          local.tee 0
          i32.load
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=4
          local.set 0
          local.get 4
          i32.const 52
          i32.add
          i64.const 1
          i64.store align=4
          local.get 4
          i32.const 2
          i32.store offset=44
          local.get 4
          i32.const 1050980
          i32.store offset=40
          local.get 4
          i32.const 12
          i32.store offset=36
          local.get 4
          local.get 0
          i32.store offset=4
          local.get 4
          local.get 2
          i32.store
          local.get 4
          local.get 4
          i32.const 32
          i32.add
          i32.store offset=48
          local.get 4
          local.get 4
          i32.store offset=32
          local.get 4
          i32.const 24
          i32.add
          local.get 1
          local.get 4
          i32.const 40
          i32.add
          local.get 5
          call_indirect (type 4)
          local.get 4
          i32.load offset=28
          local.set 1
          block ;; label = @3
            local.get 4
            i32.load8_u offset=24
            local.tee 0
            i32.const 4
            i32.gt_u
            br_if 0 (;@3;)
            local.get 0
            i32.const 3
            i32.ne
            br_if 2 (;@1;)
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
          call_indirect (type $.data)
          block ;; label = @3
            local.get 0
            i32.load offset=4
            local.tee 3
            i32.eqz
            br_if 0 (;@3;)
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
          br 1 (;@1;)
        end
        local.get 4
        i32.const 52
        i32.add
        i64.const 0
        i64.store align=4
        local.get 4
        i32.const 1
        i32.store offset=44
        local.get 4
        i32.const 1051076
        i32.store offset=40
        local.get 4
        i32.const 1048764
        i32.store offset=48
        local.get 4
        local.get 1
        local.get 4
        i32.const 40
        i32.add
        local.get 5
        call_indirect (type 4)
        local.get 4
        i32.load offset=4
        local.set 1
        block ;; label = @2
          local.get 4
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
        call_indirect (type $.data)
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
      local.get 4
      i32.const 64
      i32.add
      global.set $__stack_pointer
    )
    (func $rust_begin_unwind (;130;) (type $.data) (param i32)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          local.get 0
          call $core::panic::panic_info::PanicInfo::location
          local.tee 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          call $core::panic::panic_info::PanicInfo::message
          local.tee 3
          i32.eqz
          br_if 1 (;@1;)
          local.get 1
          local.get 2
          i32.store offset=12
          local.get 1
          local.get 0
          i32.store offset=8
          local.get 1
          local.get 3
          i32.store offset=4
          local.get 1
          i32.const 4
          i32.add
          call $std::sys_common::backtrace::__rust_end_short_backtrace
          unreachable
        end
        i32.const 1048892
        i32.const 43
        i32.const 1051084
        call $core::panicking::panic
        unreachable
      end
      i32.const 1048892
      i32.const 43
      i32.const 1051100
      call $core::panicking::panic
      unreachable
    )
    (func $<std::panicking::begin_panic_handler::PanicPayload as core::panic::BoxMeUp>::take_box (;131;) (type 2) (param i32 i32)
      (local i32 i32 i32 i32 i64)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 1
      i32.const 4
      i32.add
      local.set 3
      block ;; label = @1
        local.get 1
        i32.load offset=4
        br_if 0 (;@1;)
        local.get 1
        i32.load
        local.set 4
        local.get 2
        i32.const 32
        i32.add
        i32.const 8
        i32.add
        local.tee 5
        i32.const 0
        i32.store
        local.get 2
        i64.const 1
        i64.store offset=32 align=4
        local.get 2
        local.get 2
        i32.const 32
        i32.add
        i32.store offset=44
        local.get 2
        i32.const 44
        i32.add
        i32.const 1048672
        local.get 4
        call $core::fmt::write
        drop
        local.get 2
        i32.const 16
        i32.add
        i32.const 8
        i32.add
        local.get 5
        i32.load
        local.tee 4
        i32.store
        local.get 2
        local.get 2
        i64.load offset=32 align=4
        local.tee 6
        i64.store offset=16
        local.get 3
        i32.const 8
        i32.add
        local.get 4
        i32.store
        local.get 3
        local.get 6
        i64.store align=4
      end
      local.get 2
      i32.const 8
      i32.add
      local.tee 4
      local.get 3
      i32.const 8
      i32.add
      i32.load
      i32.store
      local.get 1
      i32.const 12
      i32.add
      i32.const 0
      i32.store
      local.get 3
      i64.load align=4
      local.set 6
      local.get 1
      i64.const 1
      i64.store offset=4 align=4
      i32.const 0
      i32.load8_u offset=1056309
      drop
      local.get 2
      local.get 6
      i64.store
      block ;; label = @1
        i32.const 12
        i32.const 4
        call $__rust_alloc
        local.tee 1
        br_if 0 (;@1;)
        i32.const 4
        i32.const 12
        call $alloc::alloc::handle_alloc_error
        unreachable
      end
      local.get 1
      local.get 2
      i64.load
      i64.store align=4
      local.get 1
      i32.const 8
      i32.add
      local.get 4
      i32.load
      i32.store
      local.get 0
      i32.const 1051116
      i32.store offset=4
      local.get 0
      local.get 1
      i32.store
      local.get 2
      i32.const 48
      i32.add
      global.set $__stack_pointer
    )
    (func $<std::panicking::begin_panic_handler::PanicPayload as core::panic::BoxMeUp>::get (;132;) (type 2) (param i32 i32)
      (local i32 i32 i32 i64)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 1
      i32.const 4
      i32.add
      local.set 3
      block ;; label = @1
        local.get 1
        i32.load offset=4
        br_if 0 (;@1;)
        local.get 1
        i32.load
        local.set 1
        local.get 2
        i32.const 16
        i32.add
        i32.const 8
        i32.add
        local.tee 4
        i32.const 0
        i32.store
        local.get 2
        i64.const 1
        i64.store offset=16 align=4
        local.get 2
        local.get 2
        i32.const 16
        i32.add
        i32.store offset=28
        local.get 2
        i32.const 28
        i32.add
        i32.const 1048672
        local.get 1
        call $core::fmt::write
        drop
        local.get 2
        i32.const 8
        i32.add
        local.get 4
        i32.load
        local.tee 1
        i32.store
        local.get 2
        local.get 2
        i64.load offset=16 align=4
        local.tee 5
        i64.store
        local.get 3
        i32.const 8
        i32.add
        local.get 1
        i32.store
        local.get 3
        local.get 5
        i64.store align=4
      end
      local.get 0
      i32.const 1051116
      i32.store offset=4
      local.get 0
      local.get 3
      i32.store
      local.get 2
      i32.const 32
      i32.add
      global.set $__stack_pointer
    )
    (func $<std::panicking::begin_panic_handler::StrPanicPayload as core::panic::BoxMeUp>::take_box (;133;) (type 2) (param i32 i32)
      (local i32 i32)
      i32.const 0
      i32.load8_u offset=1056309
      drop
      local.get 1
      i32.load offset=4
      local.set 2
      local.get 1
      i32.load
      local.set 3
      block ;; label = @1
        i32.const 8
        i32.const 4
        call $__rust_alloc
        local.tee 1
        br_if 0 (;@1;)
        i32.const 4
        i32.const 8
        call $alloc::alloc::handle_alloc_error
        unreachable
      end
      local.get 1
      local.get 2
      i32.store offset=4
      local.get 1
      local.get 3
      i32.store
      local.get 0
      i32.const 1051132
      i32.store offset=4
      local.get 0
      local.get 1
      i32.store
    )
    (func $<std::panicking::begin_panic_handler::StrPanicPayload as core::panic::BoxMeUp>::get (;134;) (type 2) (param i32 i32)
      local.get 0
      i32.const 1051132
      i32.store offset=4
      local.get 0
      local.get 1
      i32.store
    )
    (func $std::panicking::rust_panic_with_hook (;135;) (type 14) (param i32 i32 i32 i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 80
      i32.sub
      local.tee 5
      global.set $__stack_pointer
      i32.const 0
      i32.const 0
      i32.load offset=1056376
      local.tee 6
      i32.const 1
      i32.add
      i32.store offset=1056376
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    local.get 6
                    i32.const 0
                    i32.lt_s
                    br_if 0 (;@7;)
                    i32.const 0
                    i32.load8_u offset=1056396
                    br_if 1 (;@6;)
                    i32.const 0
                    i32.const 1
                    i32.store8 offset=1056396
                    i32.const 0
                    i32.const 0
                    i32.load offset=1056392
                    i32.const 1
                    i32.add
                    i32.store offset=1056392
                    local.get 5
                    local.get 2
                    i32.store offset=28
                    local.get 5
                    i32.const 1051188
                    i32.store offset=20
                    local.get 5
                    i32.const 1048764
                    i32.store offset=16
                    local.get 5
                    local.get 4
                    i32.store8 offset=32
                    local.get 5
                    local.get 3
                    i32.store offset=24
                    i32.const 0
                    i32.load offset=1056360
                    local.tee 6
                    i32.const -1
                    i32.le_s
                    br_if 5 (;@2;)
                    i32.const 0
                    local.get 6
                    i32.const 1
                    i32.add
                    i32.store offset=1056360
                    i32.const 0
                    i32.load offset=1056368
                    local.set 6
                    local.get 5
                    local.get 0
                    local.get 1
                    i32.load offset=16
                    call_indirect (type 2)
                    local.get 5
                    local.get 5
                    i64.load
                    i64.store offset=16 align=4
                    local.get 6
                    br_if 3 (;@4;)
                    local.get 5
                    i32.const 16
                    i32.add
                    i32.const 0
                    local.get 5
                    call $std::panicking::panic_hook_with_disk_dump
                    br 4 (;@3;)
                  end
                  local.get 5
                  local.get 2
                  i32.store offset=28
                  local.get 5
                  i32.const 1051188
                  i32.store offset=20
                  local.get 5
                  i32.const 1048764
                  i32.store offset=16
                  local.get 5
                  local.get 4
                  i32.store8 offset=32
                  local.get 5
                  local.get 3
                  i32.store offset=24
                  local.get 5
                  i32.const 52
                  i32.add
                  i64.const 1
                  i64.store align=4
                  local.get 5
                  i32.const 2
                  i32.store offset=44
                  local.get 5
                  i32.const 1051256
                  i32.store offset=40
                  local.get 5
                  i32.const 13
                  i32.store offset=12
                  local.get 5
                  local.get 5
                  i32.const 8
                  i32.add
                  i32.store offset=48
                  local.get 5
                  local.get 5
                  i32.const 16
                  i32.add
                  i32.store offset=8
                  local.get 5
                  i32.const 4
                  i32.store8 offset=64
                  local.get 5
                  local.get 5
                  i32.const 8
                  i32.add
                  i32.store offset=72
                  local.get 5
                  i32.const 64
                  i32.add
                  i32.const 1049812
                  local.get 5
                  i32.const 40
                  i32.add
                  call $core::fmt::write
                  local.set 4
                  local.get 5
                  i32.load8_u offset=64
                  local.set 6
                  block ;; label = @7
                    local.get 4
                    i32.eqz
                    br_if 0 (;@7;)
                    local.get 6
                    i32.const 4
                    i32.eq
                    br_if 2 (;@5;)
                    local.get 5
                    i32.load offset=68
                    local.set 6
                    block ;; label = @8
                      local.get 5
                      i32.load8_u offset=64
                      local.tee 5
                      i32.const 4
                      i32.gt_u
                      br_if 0 (;@8;)
                      local.get 5
                      i32.const 3
                      i32.ne
                      br_if 3 (;@5;)
                    end
                    local.get 6
                    i32.load
                    local.tee 4
                    local.get 6
                    i32.const 4
                    i32.add
                    i32.load
                    local.tee 5
                    i32.load
                    call_indirect (type $.data)
                    block ;; label = @8
                      local.get 5
                      i32.load offset=4
                      local.tee 3
                      i32.eqz
                      br_if 0 (;@8;)
                      local.get 4
                      local.get 3
                      local.get 5
                      i32.load offset=8
                      call $__rust_dealloc
                    end
                    local.get 6
                    i32.const 12
                    i32.const 4
                    call $__rust_dealloc
                    call $std::sys::wasi::abort_internal
                    unreachable
                  end
                  local.get 5
                  i32.load offset=68
                  local.set 5
                  block ;; label = @7
                    local.get 6
                    i32.const 4
                    i32.gt_u
                    br_if 0 (;@7;)
                    local.get 6
                    i32.const 3
                    i32.ne
                    br_if 2 (;@5;)
                  end
                  local.get 5
                  i32.load
                  local.tee 4
                  local.get 5
                  i32.const 4
                  i32.add
                  i32.load
                  local.tee 6
                  i32.load
                  call_indirect (type $.data)
                  block ;; label = @7
                    local.get 6
                    i32.load offset=4
                    local.tee 3
                    i32.eqz
                    br_if 0 (;@7;)
                    local.get 4
                    local.get 3
                    local.get 6
                    i32.load offset=8
                    call $__rust_dealloc
                  end
                  local.get 5
                  i32.const 12
                  i32.const 4
                  call $__rust_dealloc
                  call $std::sys::wasi::abort_internal
                  unreachable
                end
                local.get 5
                i32.const 52
                i32.add
                i64.const 0
                i64.store align=4
                local.get 5
                i32.const 1
                i32.store offset=44
                local.get 5
                i32.const 1051324
                i32.store offset=40
                local.get 5
                i32.const 1048764
                i32.store offset=48
                local.get 5
                i32.const 4
                i32.store8 offset=16
                local.get 5
                local.get 5
                i32.const 8
                i32.add
                i32.store offset=24
                local.get 5
                i32.const 16
                i32.add
                i32.const 1049812
                local.get 5
                i32.const 40
                i32.add
                call $core::fmt::write
                local.set 4
                local.get 5
                i32.load8_u offset=16
                local.set 6
                block ;; label = @6
                  local.get 4
                  i32.eqz
                  br_if 0 (;@6;)
                  local.get 6
                  i32.const 4
                  i32.eq
                  br_if 1 (;@5;)
                  local.get 5
                  i32.load offset=20
                  local.set 6
                  block ;; label = @7
                    local.get 5
                    i32.load8_u offset=16
                    local.tee 5
                    i32.const 4
                    i32.gt_u
                    br_if 0 (;@7;)
                    local.get 5
                    i32.const 3
                    i32.ne
                    br_if 2 (;@5;)
                  end
                  local.get 6
                  i32.load
                  local.tee 4
                  local.get 6
                  i32.const 4
                  i32.add
                  i32.load
                  local.tee 5
                  i32.load
                  call_indirect (type $.data)
                  block ;; label = @7
                    local.get 5
                    i32.load offset=4
                    local.tee 3
                    i32.eqz
                    br_if 0 (;@7;)
                    local.get 4
                    local.get 3
                    local.get 5
                    i32.load offset=8
                    call $__rust_dealloc
                  end
                  local.get 6
                  i32.const 12
                  i32.const 4
                  call $__rust_dealloc
                  call $std::sys::wasi::abort_internal
                  unreachable
                end
                local.get 5
                i32.load offset=20
                local.set 5
                block ;; label = @6
                  local.get 6
                  i32.const 4
                  i32.gt_u
                  br_if 0 (;@6;)
                  local.get 6
                  i32.const 3
                  i32.ne
                  br_if 1 (;@5;)
                end
                local.get 5
                i32.load
                local.tee 4
                local.get 5
                i32.const 4
                i32.add
                i32.load
                local.tee 6
                i32.load
                call_indirect (type $.data)
                block ;; label = @6
                  local.get 6
                  i32.load offset=4
                  local.tee 3
                  i32.eqz
                  br_if 0 (;@6;)
                  local.get 4
                  local.get 3
                  local.get 6
                  i32.load offset=8
                  call $__rust_dealloc
                end
                local.get 5
                i32.const 12
                i32.const 4
                call $__rust_dealloc
              end
              call $std::sys::wasi::abort_internal
              unreachable
            end
            i32.const 0
            i32.load offset=1056368
            local.get 5
            i32.const 16
            i32.add
            i32.const 0
            i32.load offset=1056372
            i32.load offset=20
            call_indirect (type 2)
          end
          i32.const 0
          i32.const 0
          i32.load offset=1056360
          i32.const -1
          i32.add
          i32.store offset=1056360
          i32.const 0
          i32.const 0
          i32.store8 offset=1056396
          local.get 4
          br_if 1 (;@1;)
          local.get 5
          i32.const 52
          i32.add
          i64.const 0
          i64.store align=4
          local.get 5
          i32.const 1
          i32.store offset=44
          local.get 5
          i32.const 1051380
          i32.store offset=40
          local.get 5
          i32.const 1048764
          i32.store offset=48
          local.get 5
          i32.const 64
          i32.add
          local.get 5
          i32.const 8
          i32.add
          local.get 5
          i32.const 40
          i32.add
          call $std::io::Write::write_fmt
          local.get 5
          i32.load8_u offset=64
          local.get 5
          i32.load offset=68
          call $core::ptr::drop_in_place<core::result::Result<(),std::io::error::Error>>
          call $std::sys::wasi::abort_internal
          unreachable
        end
        local.get 5
        i32.const 40
        i32.add
        i32.const 1051712
        i32.const 1
        local.get 5
        i32.const 8
        i32.add
        i32.const 0
        call $core::fmt::Arguments::new_v1
        local.get 5
        i32.const 64
        i32.add
        local.get 5
        i32.const 8
        i32.add
        local.get 5
        i32.const 40
        i32.add
        call $std::io::Write::write_fmt
        local.get 5
        i32.load8_u offset=64
        local.get 5
        i32.load offset=68
        call $core::ptr::drop_in_place<core::result::Result<(),std::io::error::Error>>
        call $std::sys::wasi::abort_internal
        unreachable
      end
      local.get 0
      local.get 1
      call $rust_panic
      unreachable
    )
    (func $rust_panic (;136;) (type 2) (param i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 2
      local.get 0
      local.get 1
      call $__rust_start_panic
      i32.store
      local.get 2
      i32.const 9
      i32.store offset=40
      local.get 2
      local.get 2
      i32.store offset=36
      local.get 2
      i32.const 12
      i32.add
      i32.const 1051444
      i32.const 2
      local.get 2
      i32.const 36
      i32.add
      i32.const 1
      call $core::fmt::Arguments::new_v1
      local.get 2
      i32.const 4
      i32.add
      local.get 2
      i32.const 47
      i32.add
      local.get 2
      i32.const 12
      i32.add
      call $std::io::Write::write_fmt
      local.get 2
      i32.load8_u offset=4
      local.get 2
      i32.load offset=8
      call $core::ptr::drop_in_place<core::result::Result<(),std::io::error::Error>>
      call $std::sys::wasi::abort_internal
      unreachable
    )
    (func $std::sys::wasi::fs::open_at (;137;) (type 14) (param i32 i32 i32 i32 i32)
      (local i32 i32 i32 i64 i64)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 5
      global.set $__stack_pointer
      local.get 4
      i32.load offset=32
      local.set 6
      local.get 5
      i32.const 24
      i32.add
      local.get 2
      local.get 3
      call $core::str::converts::from_utf8
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 5
            i32.load offset=24
            br_if 0 (;@3;)
            local.get 5
            i32.const 32
            i32.add
            i32.load
            local.set 3
            local.get 5
            i32.load offset=28
            local.set 2
            local.get 4
            i32.load16_u offset=38
            local.set 7
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  local.get 4
                  i64.load
                  i64.const 0
                  i64.ne
                  br_if 0 (;@6;)
                  i64.const 16386
                  i64.const 0
                  local.get 4
                  i32.load8_u offset=40
                  select
                  local.tee 8
                  i64.const 4194625
                  i64.or
                  local.get 8
                  local.get 4
                  i32.load8_u offset=41
                  local.get 4
                  i32.load8_u offset=42
                  i32.or
                  select
                  i64.const 262651580
                  i64.or
                  local.tee 9
                  local.set 8
                  local.get 4
                  i64.load offset=16
                  i64.eqz
                  i32.eqz
                  br_if 1 (;@5;)
                  br 2 (;@4;)
                end
                local.get 4
                i64.load offset=8
                local.tee 9
                local.set 8
                local.get 4
                i64.load offset=16
                i64.eqz
                br_if 1 (;@4;)
              end
              local.get 4
              i32.const 24
              i32.add
              i64.load
              local.set 8
            end
            local.get 5
            i32.const 12
            i32.add
            local.get 1
            local.get 6
            local.get 2
            local.get 3
            local.get 7
            local.get 9
            local.get 8
            local.get 4
            i32.load16_u offset=36
            call $wasi::lib_generated::path_open
            block ;; label = @4
              local.get 5
              i32.load16_u offset=12
              br_if 0 (;@4;)
              local.get 5
              local.get 5
              i32.load offset=16
              local.tee 4
              i32.store offset=20
              local.get 4
              i32.const -1
              i32.eq
              br_if 3 (;@1;)
              local.get 0
              i32.const 4
              i32.store8
              local.get 0
              local.get 4
              i32.store offset=4
              br 2 (;@2;)
            end
            local.get 5
            local.get 5
            i32.load16_u offset=14
            i32.store16 offset=24
            local.get 5
            i32.const 24
            i32.add
            call $wasi::lib_generated::Errno::raw
            local.set 4
            local.get 0
            i32.const 3
            i32.add
            i32.const 0
            i32.store8
            local.get 0
            i32.const 0
            i32.store16 offset=1 align=1
            local.get 0
            local.get 4
            i32.const 65535
            i32.and
            i32.store offset=4
            local.get 0
            i32.const 0
            i32.store8
            br 1 (;@2;)
          end
          local.get 0
          i32.const 1049856
          i32.store offset=4
          local.get 0
          i32.const 2
          i32.store
        end
        local.get 5
        i32.const 48
        i32.add
        global.set $__stack_pointer
        return
      end
      local.get 5
      i32.const 0
      i32.store offset=24
      local.get 5
      i32.const 20
      i32.add
      local.get 5
      i32.const 24
      i32.add
      call $core::panicking::assert_failed
      unreachable
    )
    (func $std::sys::wasi::process::ExitCode::as_i32 (;138;) (type 3) (param i32) (result i32)
      local.get 0
      i32.load8_u
    )
    (func $<std::sys::wasi::stdio::Stderr as std::io::Write>::write (;139;) (type 12) (param i32 i32 i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      local.get 4
      local.get 3
      i32.store offset=16
      local.get 4
      local.get 2
      i32.store offset=12
      local.get 4
      i32.const 20
      i32.add
      i32.const 2
      local.get 4
      i32.const 12
      i32.add
      i32.const 1
      call $wasi::lib_generated::fd_write
      block ;; label = @1
        block ;; label = @2
          local.get 4
          i32.load16_u offset=20
          br_if 0 (;@2;)
          local.get 0
          local.get 4
          i32.load offset=24
          i32.store offset=4
          local.get 0
          i32.const 4
          i32.store8
          br 1 (;@1;)
        end
        local.get 4
        local.get 4
        i32.load16_u offset=22
        i32.store16 offset=30
        local.get 0
        local.get 4
        i32.const 30
        i32.add
        call $wasi::lib_generated::Errno::raw
        i64.extend_i32_u
        i64.const 65535
        i64.and
        i64.const 32
        i64.shl
        i64.store align=4
      end
      local.get 4
      i32.const 32
      i32.add
      global.set $__stack_pointer
    )
    (func $<std::sys::wasi::stdio::Stderr as std::io::Write>::write_vectored (;140;) (type 12) (param i32 i32 i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      local.get 4
      i32.const 4
      i32.add
      i32.const 2
      local.get 2
      local.get 3
      call $wasi::lib_generated::fd_write
      block ;; label = @1
        block ;; label = @2
          local.get 4
          i32.load16_u offset=4
          br_if 0 (;@2;)
          local.get 0
          local.get 4
          i32.load offset=8
          i32.store offset=4
          local.get 0
          i32.const 4
          i32.store8
          br 1 (;@1;)
        end
        local.get 4
        local.get 4
        i32.load16_u offset=6
        i32.store16 offset=14
        local.get 0
        local.get 4
        i32.const 14
        i32.add
        call $wasi::lib_generated::Errno::raw
        i64.extend_i32_u
        i64.const 65535
        i64.and
        i64.const 32
        i64.shl
        i64.store align=4
      end
      local.get 4
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $<std::sys::wasi::stdio::Stderr as std::io::Write>::is_write_vectored (;141;) (type 3) (param i32) (result i32)
      i32.const 1
    )
    (func $<std::sys::wasi::stdio::Stderr as std::io::Write>::flush (;142;) (type 2) (param i32 i32)
      local.get 0
      i32.const 4
      i32.store8
    )
    (func $std::alloc::rust_oom (;143;) (type 2) (param i32 i32)
      (local i32)
      local.get 0
      local.get 1
      i32.const 0
      i32.load offset=1056356
      local.tee 2
      i32.const 14
      local.get 2
      select
      call_indirect (type 2)
      call $std::process::abort
      unreachable
    )
    (func $__rg_oom (;144;) (type 2) (param i32 i32)
      local.get 1
      local.get 0
      call $std::alloc::rust_oom
      unreachable
    )
    (func $__rust_start_panic (;145;) (type 6) (param i32 i32) (result i32)
      unreachable
      unreachable
    )
    (func $wasi::lib_generated::Errno::raw (;146;) (type 3) (param i32) (result i32)
      local.get 0
      i32.load16_u
    )
    (func $wasi::lib_generated::fd_write (;147;) (type 12) (param i32 i32 i32 i32)
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
    (func $wasi::lib_generated::path_open (;148;) (type 16) (param i32 i32 i32 i32 i32 i32 i64 i64 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 9
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          local.get 1
          local.get 2
          local.get 3
          local.get 4
          local.get 5
          i32.const 65535
          i32.and
          local.get 6
          local.get 7
          local.get 8
          i32.const 65535
          i32.and
          local.get 9
          i32.const 12
          i32.add
          call $wasi::lib_generated::wasi_snapshot_preview1::path_open
          local.tee 8
          br_if 0 (;@2;)
          local.get 0
          local.get 9
          i32.load offset=12
          i32.store offset=4
          i32.const 0
          local.set 8
          br 1 (;@1;)
        end
        local.get 0
        local.get 8
        i32.store16 offset=2
        i32.const 1
        local.set 8
      end
      local.get 0
      local.get 8
      i32.store16
      local.get 9
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $malloc (;149;) (type 3) (param i32) (result i32)
      local.get 0
      call $dlmalloc
    )
    (func $dlmalloc (;150;) (type 3) (param i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      block ;; label = @1
        i32.const 0
        i32.load offset=1056444
        local.tee 2
        br_if 0 (;@1;)
        block ;; label = @2
          block ;; label = @3
            i32.const 0
            i32.load offset=1056892
            local.tee 3
            i32.eqz
            br_if 0 (;@3;)
            i32.const 0
            i32.load offset=1056896
            local.set 4
            br 1 (;@2;)
          end
          i32.const 0
          i64.const -1
          i64.store offset=1056904 align=4
          i32.const 0
          i64.const 281474976776192
          i64.store offset=1056896 align=4
          i32.const 0
          local.get 1
          i32.const 8
          i32.add
          i32.const -16
          i32.and
          i32.const 1431655768
          i32.xor
          local.tee 3
          i32.store offset=1056892
          i32.const 0
          i32.const 0
          i32.store offset=1056912
          i32.const 0
          i32.const 0
          i32.store offset=1056864
          i32.const 65536
          local.set 4
        end
        i32.const 0
        local.set 2
        i32.const 1114112
        i32.const 1056960
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
        i32.const 1056960
        i32.sub
        local.tee 5
        i32.const 89
        i32.lt_u
        br_if 0 (;@1;)
        i32.const 0
        local.set 4
        i32.const 0
        local.get 5
        i32.store offset=1056872
        i32.const 0
        i32.const 1056960
        i32.store offset=1056868
        i32.const 0
        i32.const 1056960
        i32.store offset=1056436
        i32.const 0
        local.get 3
        i32.store offset=1056456
        i32.const 0
        i32.const -1
        i32.store offset=1056452
        loop ;; label = @2
          local.get 4
          i32.const 1056480
          i32.add
          local.get 4
          i32.const 1056468
          i32.add
          local.tee 3
          i32.store
          local.get 3
          local.get 4
          i32.const 1056460
          i32.add
          local.tee 6
          i32.store
          local.get 4
          i32.const 1056472
          i32.add
          local.get 6
          i32.store
          local.get 4
          i32.const 1056488
          i32.add
          local.get 4
          i32.const 1056476
          i32.add
          local.tee 6
          i32.store
          local.get 6
          local.get 3
          i32.store
          local.get 4
          i32.const 1056496
          i32.add
          local.get 4
          i32.const 1056484
          i32.add
          local.tee 3
          i32.store
          local.get 3
          local.get 6
          i32.store
          local.get 4
          i32.const 1056492
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
        i32.const 1056960
        i32.const -8
        i32.const 1056960
        i32.sub
        i32.const 15
        i32.and
        i32.const 0
        i32.const 1056960
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
        i32.load offset=1056908
        i32.store offset=1056448
        i32.const 0
        local.get 4
        i32.store offset=1056432
        i32.const 0
        local.get 2
        i32.store offset=1056444
        i32.const 1056960
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
                                i32.load offset=1056420
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
                                    i32.const 1056460
                                    i32.add
                                    local.tee 4
                                    local.get 3
                                    i32.const 1056468
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
                                    i32.store offset=1056420
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
                              i32.load offset=1056428
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
                                    i32.const 1056460
                                    i32.add
                                    local.tee 6
                                    local.get 4
                                    i32.const 1056468
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
                                    i32.store offset=1056420
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
                                  i32.const 1056460
                                  i32.add
                                  local.set 5
                                  i32.const 0
                                  i32.load offset=1056440
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
                                      i32.store offset=1056420
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
                                i32.store offset=1056440
                                i32.const 0
                                local.get 6
                                i32.store offset=1056428
                                br 12 (;@1;)
                              end
                              i32.const 0
                              i32.load offset=1056424
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
                              i32.const 1056724
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
                                i32.load offset=1056436
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
                            i32.load offset=1056424
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
                                    i32.const 1056724
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
                                  i32.const 1056724
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
                            i32.load offset=1056428
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
                              i32.load offset=1056436
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
                            i32.load offset=1056428
                            local.tee 4
                            local.get 5
                            i32.lt_u
                            br_if 0 (;@11;)
                            i32.const 0
                            i32.load offset=1056440
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
                            i32.store offset=1056428
                            i32.const 0
                            local.get 0
                            i32.store offset=1056440
                            local.get 3
                            i32.const 8
                            i32.add
                            local.set 4
                            br 10 (;@1;)
                          end
                          block ;; label = @11
                            i32.const 0
                            i32.load offset=1056432
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
                            i32.store offset=1056444
                            i32.const 0
                            local.get 3
                            i32.store offset=1056432
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
                              i32.load offset=1056892
                              i32.eqz
                              br_if 0 (;@12;)
                              i32.const 0
                              i32.load offset=1056900
                              local.set 3
                              br 1 (;@11;)
                            end
                            i32.const 0
                            i64.const -1
                            i64.store offset=1056904 align=4
                            i32.const 0
                            i64.const 281474976776192
                            i64.store offset=1056896 align=4
                            i32.const 0
                            local.get 1
                            i32.const 12
                            i32.add
                            i32.const -16
                            i32.and
                            i32.const 1431655768
                            i32.xor
                            i32.store offset=1056892
                            i32.const 0
                            i32.const 0
                            i32.store offset=1056912
                            i32.const 0
                            i32.const 0
                            i32.store offset=1056864
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
                            i32.store offset=1056916
                            br 10 (;@1;)
                          end
                          block ;; label = @11
                            i32.const 0
                            i32.load offset=1056860
                            local.tee 4
                            i32.eqz
                            br_if 0 (;@11;)
                            block ;; label = @12
                              i32.const 0
                              i32.load offset=1056852
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
                            i32.store offset=1056916
                            br 10 (;@1;)
                          end
                          i32.const 0
                          i32.load8_u offset=1056864
                          i32.const 4
                          i32.and
                          br_if 4 (;@6;)
                          block ;; label = @11
                            block ;; label = @12
                              block ;; label = @13
                                local.get 2
                                i32.eqz
                                br_if 0 (;@13;)
                                i32.const 1056868
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
                                i32.load offset=1056896
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
                                i32.load offset=1056860
                                local.tee 4
                                i32.eqz
                                br_if 0 (;@13;)
                                i32.const 0
                                i32.load offset=1056852
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
                              i32.load offset=1056900
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
                  i32.load offset=1056864
                  i32.const 4
                  i32.or
                  i32.store offset=1056864
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
              i32.load offset=1056852
              local.get 7
              i32.add
              local.tee 4
              i32.store offset=1056852
              block ;; label = @5
                local.get 4
                i32.const 0
                i32.load offset=1056856
                i32.le_u
                br_if 0 (;@5;)
                i32.const 0
                local.get 4
                i32.store offset=1056856
              end
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    block ;; label = @8
                      i32.const 0
                      i32.load offset=1056444
                      local.tee 3
                      i32.eqz
                      br_if 0 (;@8;)
                      i32.const 1056868
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
                        i32.load offset=1056436
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
                      i32.store offset=1056436
                    end
                    i32.const 0
                    local.set 4
                    i32.const 0
                    local.get 7
                    i32.store offset=1056872
                    i32.const 0
                    local.get 0
                    i32.store offset=1056868
                    i32.const 0
                    i32.const -1
                    i32.store offset=1056452
                    i32.const 0
                    i32.const 0
                    i32.load offset=1056892
                    i32.store offset=1056456
                    i32.const 0
                    i32.const 0
                    i32.store offset=1056880
                    loop ;; label = @8
                      local.get 4
                      i32.const 1056480
                      i32.add
                      local.get 4
                      i32.const 1056468
                      i32.add
                      local.tee 3
                      i32.store
                      local.get 3
                      local.get 4
                      i32.const 1056460
                      i32.add
                      local.tee 6
                      i32.store
                      local.get 4
                      i32.const 1056472
                      i32.add
                      local.get 6
                      i32.store
                      local.get 4
                      i32.const 1056488
                      i32.add
                      local.get 4
                      i32.const 1056476
                      i32.add
                      local.tee 6
                      i32.store
                      local.get 6
                      local.get 3
                      i32.store
                      local.get 4
                      i32.const 1056496
                      i32.add
                      local.get 4
                      i32.const 1056484
                      i32.add
                      local.tee 3
                      i32.store
                      local.get 3
                      local.get 6
                      i32.store
                      local.get 4
                      i32.const 1056492
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
                    i32.load offset=1056908
                    i32.store offset=1056448
                    i32.const 0
                    local.get 4
                    i32.store offset=1056432
                    i32.const 0
                    local.get 3
                    i32.store offset=1056444
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
                  i32.load offset=1056432
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
                  i32.load offset=1056908
                  i32.store offset=1056448
                  i32.const 0
                  local.get 6
                  i32.store offset=1056432
                  i32.const 0
                  local.get 0
                  i32.store offset=1056444
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
                  i32.load offset=1056436
                  local.tee 9
                  i32.ge_u
                  br_if 0 (;@6;)
                  i32.const 0
                  local.get 0
                  i32.store offset=1056436
                  local.get 0
                  local.set 9
                end
                local.get 0
                local.get 7
                i32.add
                local.set 6
                i32.const 1056868
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
                          i32.const 1056868
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
                          i32.store offset=1056444
                          i32.const 0
                          i32.const 0
                          i32.load offset=1056432
                          local.get 4
                          i32.add
                          local.tee 4
                          i32.store offset=1056432
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
                          i32.load offset=1056440
                          i32.ne
                          br_if 0 (;@10;)
                          i32.const 0
                          local.get 5
                          i32.store offset=1056440
                          i32.const 0
                          i32.const 0
                          i32.load offset=1056428
                          local.get 4
                          i32.add
                          local.tee 4
                          i32.store offset=1056428
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
                              i32.const 1056460
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
                                i32.load offset=1056420
                                i32.const -2
                                local.get 9
                                i32.rotl
                                i32.and
                                i32.store offset=1056420
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
                                i32.const 1056724
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
                                i32.load offset=1056424
                                i32.const -2
                                local.get 6
                                i32.rotl
                                i32.and
                                i32.store offset=1056424
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
                          i32.const 1056460
                          i32.add
                          local.set 3
                          block ;; label = @11
                            block ;; label = @12
                              i32.const 0
                              i32.load offset=1056420
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
                              i32.store offset=1056420
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
                        i32.const 1056724
                        i32.add
                        local.set 6
                        block ;; label = @10
                          i32.const 0
                          i32.load offset=1056424
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
                          i32.store offset=1056424
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
                      i32.load offset=1056908
                      i32.store offset=1056448
                      i32.const 0
                      local.get 4
                      i32.store offset=1056432
                      i32.const 0
                      local.get 2
                      i32.store offset=1056444
                      local.get 9
                      i32.const 16
                      i32.add
                      i32.const 0
                      i64.load offset=1056876 align=4
                      i64.store align=4
                      local.get 9
                      i32.const 0
                      i64.load offset=1056868 align=4
                      i64.store offset=8 align=4
                      i32.const 0
                      local.get 9
                      i32.const 8
                      i32.add
                      i32.store offset=1056876
                      i32.const 0
                      local.get 7
                      i32.store offset=1056872
                      i32.const 0
                      local.get 0
                      i32.store offset=1056868
                      i32.const 0
                      i32.const 0
                      i32.store offset=1056880
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
                        i32.const 1056460
                        i32.add
                        local.set 4
                        block ;; label = @10
                          block ;; label = @11
                            i32.const 0
                            i32.load offset=1056420
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
                            i32.store offset=1056420
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
                      i32.const 1056724
                      i32.add
                      local.set 6
                      block ;; label = @9
                        i32.const 0
                        i32.load offset=1056424
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
                        i32.store offset=1056424
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
              i32.load offset=1056432
              local.tee 4
              local.get 5
              i32.le_u
              br_if 0 (;@4;)
              i32.const 0
              i32.load offset=1056444
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
              i32.store offset=1056432
              i32.const 0
              local.get 6
              i32.store offset=1056444
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
            i32.store offset=1056916
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
                i32.const 1056724
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
                i32.store offset=1056424
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
              i32.const 1056460
              i32.add
              local.set 4
              block ;; label = @5
                block ;; label = @6
                  i32.const 0
                  i32.load offset=1056420
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
                  i32.store offset=1056420
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
            i32.const 1056724
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
              i32.store offset=1056424
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
              i32.const 1056724
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
              i32.store offset=1056424
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
            i32.const 1056460
            i32.add
            local.set 5
            i32.const 0
            i32.load offset=1056440
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
                i32.store offset=1056420
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
          i32.store offset=1056440
          i32.const 0
          local.get 3
          i32.store offset=1056428
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
    (func $free (;151;) (type $.data) (param i32)
      local.get 0
      call $dlfree
    )
    (func $dlfree (;152;) (type $.data) (param i32)
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
          i32.load offset=1056436
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
            i32.load offset=1056440
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
              i32.const 1056460
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
                i32.load offset=1056420
                i32.const -2
                local.get 5
                i32.rotl
                i32.and
                i32.store offset=1056420
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
                i32.const 1056724
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
                i32.load offset=1056424
                i32.const -2
                local.get 4
                i32.rotl
                i32.and
                i32.store offset=1056424
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
          i32.store offset=1056428
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
              i32.load offset=1056444
              i32.ne
              br_if 0 (;@4;)
              i32.const 0
              local.get 1
              i32.store offset=1056444
              i32.const 0
              i32.const 0
              i32.load offset=1056432
              local.get 0
              i32.add
              local.tee 0
              i32.store offset=1056432
              local.get 1
              local.get 0
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 1
              i32.const 0
              i32.load offset=1056440
              i32.ne
              br_if 3 (;@1;)
              i32.const 0
              i32.const 0
              i32.store offset=1056428
              i32.const 0
              i32.const 0
              i32.store offset=1056440
              return
            end
            block ;; label = @4
              local.get 3
              i32.const 0
              i32.load offset=1056440
              i32.ne
              br_if 0 (;@4;)
              i32.const 0
              local.get 1
              i32.store offset=1056440
              i32.const 0
              i32.const 0
              i32.load offset=1056428
              local.get 0
              i32.add
              local.tee 0
              i32.store offset=1056428
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
                i32.const 1056460
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
                  i32.load offset=1056420
                  i32.const -2
                  local.get 5
                  i32.rotl
                  i32.and
                  i32.store offset=1056420
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
                  i32.load offset=1056436
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
                  i32.const 1056724
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
                  i32.load offset=1056424
                  i32.const -2
                  local.get 4
                  i32.rotl
                  i32.and
                  i32.store offset=1056424
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
            i32.load offset=1056440
            i32.ne
            br_if 1 (;@2;)
            i32.const 0
            local.get 0
            i32.store offset=1056428
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
          i32.const 1056460
          i32.add
          local.set 2
          block ;; label = @3
            block ;; label = @4
              i32.const 0
              i32.load offset=1056420
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
              i32.store offset=1056420
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
        i32.const 1056724
        i32.add
        local.set 4
        block ;; label = @2
          block ;; label = @3
            i32.const 0
            i32.load offset=1056424
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
            i32.store offset=1056424
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
        i32.load offset=1056452
        i32.const -1
        i32.add
        local.tee 1
        i32.const -1
        local.get 1
        select
        i32.store offset=1056452
      end
    )
    (func $calloc (;153;) (type 6) (param i32 i32) (result i32)
      (local i32 i64)
      block ;; label = @1
        block ;; label = @2
          local.get 0
          br_if 0 (;@2;)
          i32.const 0
          local.set 2
          br 1 (;@1;)
        end
        local.get 0
        i64.extend_i32_u
        local.get 1
        i64.extend_i32_u
        i64.mul
        local.tee 3
        i32.wrap_i64
        local.set 2
        local.get 1
        local.get 0
        i32.or
        i32.const 65536
        i32.lt_u
        br_if 0 (;@1;)
        i32.const -1
        local.get 2
        local.get 3
        i64.const 32
        i64.shr_u
        i32.wrap_i64
        i32.const 0
        i32.ne
        select
        local.set 2
      end
      block ;; label = @1
        local.get 2
        call $dlmalloc
        local.tee 0
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.const -4
        i32.add
        i32.load8_u
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.const 0
        local.get 2
        call $memset
        drop
      end
      local.get 0
    )
    (func $realloc (;154;) (type 6) (param i32 i32) (result i32)
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
        i32.store offset=1056916
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
            i32.load offset=1056900
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
            i32.load offset=1056444
            i32.ne
            br_if 0 (;@3;)
            i32.const 0
            i32.load offset=1056432
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
            i32.store offset=1056444
            i32.const 0
            local.get 5
            local.get 2
            i32.sub
            local.tee 2
            i32.store offset=1056432
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
            i32.load offset=1056440
            i32.ne
            br_if 0 (;@3;)
            i32.const 0
            i32.load offset=1056428
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
            i32.store offset=1056440
            i32.const 0
            local.get 1
            i32.store offset=1056428
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
              i32.const 1056460
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
                i32.load offset=1056420
                i32.const -2
                local.get 11
                i32.rotl
                i32.and
                i32.store offset=1056420
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
                i32.load offset=1056436
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
                i32.const 1056724
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
                i32.load offset=1056424
                i32.const -2
                local.get 5
                i32.rotl
                i32.and
                i32.store offset=1056424
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
    (func $dispose_chunk (;155;) (type 2) (param i32 i32)
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
              i32.load offset=1056440
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
                i32.const 1056460
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
                i32.load offset=1056420
                i32.const -2
                local.get 5
                i32.rotl
                i32.and
                i32.store offset=1056420
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
                  i32.load offset=1056436
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
                  i32.const 1056724
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
                  i32.load offset=1056424
                  i32.const -2
                  local.get 4
                  i32.rotl
                  i32.and
                  i32.store offset=1056424
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
            i32.store offset=1056428
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
              i32.load offset=1056444
              i32.ne
              br_if 0 (;@4;)
              i32.const 0
              local.get 0
              i32.store offset=1056444
              i32.const 0
              i32.const 0
              i32.load offset=1056432
              local.get 1
              i32.add
              local.tee 1
              i32.store offset=1056432
              local.get 0
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              i32.const 0
              i32.load offset=1056440
              i32.ne
              br_if 3 (;@1;)
              i32.const 0
              i32.const 0
              i32.store offset=1056428
              i32.const 0
              i32.const 0
              i32.store offset=1056440
              return
            end
            block ;; label = @4
              local.get 2
              i32.const 0
              i32.load offset=1056440
              i32.ne
              br_if 0 (;@4;)
              i32.const 0
              local.get 0
              i32.store offset=1056440
              i32.const 0
              i32.const 0
              i32.load offset=1056428
              local.get 1
              i32.add
              local.tee 1
              i32.store offset=1056428
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
                i32.const 1056460
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
                  i32.load offset=1056420
                  i32.const -2
                  local.get 5
                  i32.rotl
                  i32.and
                  i32.store offset=1056420
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
                  i32.load offset=1056436
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
                  i32.const 1056724
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
                  i32.load offset=1056424
                  i32.const -2
                  local.get 4
                  i32.rotl
                  i32.and
                  i32.store offset=1056424
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
            i32.load offset=1056440
            i32.ne
            br_if 1 (;@2;)
            i32.const 0
            local.get 1
            i32.store offset=1056428
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
          i32.const 1056460
          i32.add
          local.set 3
          block ;; label = @3
            block ;; label = @4
              i32.const 0
              i32.load offset=1056420
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
              i32.store offset=1056420
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
        i32.const 1056724
        i32.add
        local.set 4
        block ;; label = @2
          i32.const 0
          i32.load offset=1056424
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
          i32.store offset=1056424
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
    (func $internal_memalign (;156;) (type 6) (param i32 i32) (result i32)
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
        i32.store offset=1056916
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
    (func $aligned_alloc (;157;) (type 6) (param i32 i32) (result i32)
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
    (func $close (;158;) (type 3) (param i32) (result i32)
      call $__wasilibc_populate_preopens
      block ;; label = @1
        local.get 0
        call $__wasi_fd_close
        local.tee 0
        br_if 0 (;@1;)
        i32.const 0
        return
      end
      i32.const 0
      local.get 0
      i32.store offset=1056916
      i32.const -1
    )
    (func $_Exit (;159;) (type $.data) (param i32)
      local.get 0
      call $__wasi_proc_exit
      unreachable
    )
    (func $__wasilibc_ensure_environ (;160;) (type $.rodata)
      block ;; label = @1
        i32.const 0
        i32.load offset=1056296
        i32.const -1
        i32.ne
        br_if 0 (;@1;)
        call $__wasilibc_initialize_environ
      end
    )
    (func $__wasilibc_initialize_environ (;161;) (type $.rodata)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 0
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.const 12
          i32.add
          local.get 0
          i32.const 8
          i32.add
          call $__wasi_environ_sizes_get
          br_if 0 (;@2;)
          block ;; label = @3
            local.get 0
            i32.load offset=12
            local.tee 1
            br_if 0 (;@3;)
            i32.const 1056920
            local.set 1
            br 2 (;@1;)
          end
          block ;; label = @3
            block ;; label = @4
              local.get 1
              i32.const 1
              i32.add
              local.tee 1
              i32.eqz
              br_if 0 (;@4;)
              local.get 0
              i32.load offset=8
              call $malloc
              local.tee 2
              i32.eqz
              br_if 0 (;@4;)
              local.get 1
              i32.const 4
              call $calloc
              local.tee 1
              br_if 1 (;@3;)
              local.get 2
              call $free
            end
            i32.const 70
            call $_Exit
            unreachable
          end
          local.get 1
          local.get 2
          call $__wasi_environ_get
          i32.eqz
          br_if 1 (;@1;)
          local.get 2
          call $free
          local.get 1
          call $free
        end
        i32.const 71
        call $_Exit
        unreachable
      end
      i32.const 0
      local.get 1
      i32.store offset=1056296
      local.get 0
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $__wasi_environ_get (;162;) (type 6) (param i32 i32) (result i32)
      local.get 0
      local.get 1
      call $__imported_wasi_snapshot_preview1_environ_get
      i32.const 65535
      i32.and
    )
    (func $__wasi_environ_sizes_get (;163;) (type 6) (param i32 i32) (result i32)
      local.get 0
      local.get 1
      call $__imported_wasi_snapshot_preview1_environ_sizes_get
      i32.const 65535
      i32.and
    )
    (func $__wasi_fd_close (;164;) (type 3) (param i32) (result i32)
      local.get 0
      call $__imported_wasi_snapshot_preview1_fd_close
      i32.const 65535
      i32.and
    )
    (func $__wasi_fd_prestat_get (;165;) (type 6) (param i32 i32) (result i32)
      local.get 0
      local.get 1
      call $__imported_wasi_snapshot_preview1_fd_prestat_get
      i32.const 65535
      i32.and
    )
    (func $__wasi_fd_prestat_dir_name (;166;) (type 5) (param i32 i32 i32) (result i32)
      local.get 0
      local.get 1
      local.get 2
      call $__imported_wasi_snapshot_preview1_fd_prestat_dir_name
      i32.const 65535
      i32.and
    )
    (func $__wasi_proc_exit (;167;) (type $.data) (param i32)
      local.get 0
      call $__imported_wasi_snapshot_preview1_proc_exit
      unreachable
    )
    (func $abort (;168;) (type $.rodata)
      unreachable
      unreachable
    )
    (func $__wasilibc_find_relpath_alloc (;169;) (type 15) (param i32 i32 i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 5
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  local.get 0
                  i32.load8_u
                  local.tee 6
                  i32.eqz
                  br_if 0 (;@6;)
                  block ;; label = @7
                    local.get 6
                    i32.const 47
                    i32.ne
                    br_if 0 (;@7;)
                    local.get 0
                    local.set 6
                    br 5 (;@2;)
                  end
                  local.get 0
                  i32.const 1051839
                  call $strcmp
                  i32.eqz
                  br_if 0 (;@6;)
                  local.get 0
                  i32.const 1051836
                  call $strcmp
                  br_if 1 (;@5;)
                end
                i32.const 0
                i32.load offset=1056300
                local.set 6
                br 1 (;@4;)
              end
              block ;; label = @5
                local.get 6
                i32.const 46
                i32.ne
                br_if 0 (;@5;)
                local.get 0
                local.get 0
                i32.load8_u offset=1
                i32.const 47
                i32.eq
                i32.const 1
                i32.shl
                i32.add
                local.set 0
              end
              i32.const 0
              i32.load offset=1056924
              local.set 7
              block ;; label = @5
                i32.const 0
                i32.load offset=1056300
                local.tee 6
                call $strlen
                local.tee 8
                local.get 0
                call $strlen
                i32.add
                local.get 8
                local.get 6
                i32.add
                i32.const -1
                i32.add
                i32.load8_u
                local.tee 9
                i32.const 47
                i32.ne
                local.tee 10
                i32.add
                i32.const 1
                i32.add
                local.tee 11
                i32.const 0
                i32.load offset=1056928
                i32.le_u
                br_if 0 (;@5;)
                local.get 7
                local.get 11
                call $realloc
                local.tee 7
                i32.eqz
                br_if 2 (;@3;)
                i32.const 0
                local.get 11
                i32.store offset=1056928
                i32.const 0
                local.get 7
                i32.store offset=1056924
                i32.const 0
                i32.load offset=1056300
                local.set 6
              end
              local.get 7
              local.get 6
              call $strcpy
              local.set 6
              block ;; label = @5
                local.get 9
                i32.const 47
                i32.eq
                br_if 0 (;@5;)
                local.get 6
                local.get 8
                i32.add
                i32.const 47
                i32.store16 align=1
              end
              local.get 6
              local.get 8
              i32.add
              local.get 10
              i32.add
              local.get 0
              call $strcpy
              drop
            end
            local.get 6
            br_if 1 (;@2;)
          end
          i32.const 0
          i32.const 48
          i32.store offset=1056916
          i32.const -1
          local.set 0
          br 1 (;@1;)
        end
        block ;; label = @2
          local.get 6
          local.get 1
          local.get 5
          i32.const 12
          i32.add
          call $__wasilibc_find_abspath
          local.tee 0
          i32.const -1
          i32.ne
          br_if 0 (;@2;)
          i32.const -1
          local.set 0
          br 1 (;@1;)
        end
        block ;; label = @2
          block ;; label = @3
            local.get 3
            i32.load
            local.get 5
            i32.load offset=12
            local.tee 6
            call $strlen
            i32.const 1
            i32.add
            local.tee 8
            i32.lt_u
            br_if 0 (;@3;)
            local.get 2
            i32.load
            local.set 1
            br 1 (;@2;)
          end
          block ;; label = @3
            local.get 4
            br_if 0 (;@3;)
            i32.const 0
            i32.const 68
            i32.store offset=1056916
            i32.const -1
            local.set 0
            br 2 (;@1;)
          end
          block ;; label = @3
            local.get 2
            i32.load
            local.get 8
            call $realloc
            local.tee 1
            br_if 0 (;@3;)
            i32.const 0
            i32.const 48
            i32.store offset=1056916
            i32.const -1
            local.set 0
            br 2 (;@1;)
          end
          local.get 3
          local.get 8
          i32.store
          local.get 2
          local.get 1
          i32.store
          local.get 5
          i32.load offset=12
          local.set 6
        end
        local.get 1
        local.get 6
        call $strcpy
        drop
      end
      local.get 5
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $getcwd (;170;) (type 6) (param i32 i32) (result i32)
      (local i32)
      i32.const 0
      i32.load offset=1056300
      local.set 2
      block ;; label = @1
        block ;; label = @2
          local.get 0
          br_if 0 (;@2;)
          local.get 2
          call $strdup
          local.tee 0
          br_if 1 (;@1;)
          i32.const 0
          i32.const 48
          i32.store offset=1056916
          i32.const 0
          return
        end
        block ;; label = @2
          local.get 2
          call $strlen
          i32.const 1
          i32.add
          local.get 1
          i32.gt_u
          br_if 0 (;@2;)
          local.get 0
          local.get 2
          call $strcpy
          return
        end
        i32.const 0
        local.set 0
        i32.const 0
        i32.const 68
        i32.store offset=1056916
      end
      local.get 0
    )
    (func $__wasilibc_populate_preopens (;171;) (type $.rodata)
      (local i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 0
      global.set $__stack_pointer
      block ;; label = @1
        i32.const 0
        i32.load8_u offset=1056940
        i32.const 1
        i32.and
        br_if 0 (;@1;)
        i32.const 0
        i32.load8_u offset=1056940
        i32.const 1
        i32.and
        br_if 0 (;@1;)
        i32.const 3
        local.set 1
        block ;; label = @2
          block ;; label = @3
            loop ;; label = @4
              block ;; label = @5
                local.get 1
                local.get 0
                i32.const 8
                i32.add
                call $__wasi_fd_prestat_get
                local.tee 2
                i32.eqz
                br_if 0 (;@5;)
                local.get 2
                i32.const 8
                i32.ne
                br_if 2 (;@3;)
                i32.const 0
                i32.const 1
                i32.store8 offset=1056940
                br 4 (;@1;)
              end
              block ;; label = @5
                local.get 0
                i32.load8_u offset=8
                br_if 0 (;@5;)
                local.get 0
                i32.load offset=12
                local.tee 3
                i32.const 1
                i32.add
                call $malloc
                local.tee 2
                i32.eqz
                br_if 3 (;@2;)
                local.get 1
                local.get 2
                local.get 3
                call $__wasi_fd_prestat_dir_name
                br_if 2 (;@3;)
                local.get 2
                local.get 0
                i32.load offset=12
                i32.add
                i32.const 0
                i32.store8
                local.get 1
                local.get 2
                call $internal_register_preopened_fd_unlocked
                br_if 3 (;@2;)
                local.get 2
                call $free
              end
              local.get 1
              i32.const 1
              i32.add
              local.set 1
              br 0 (;@4;)
            end
          end
          i32.const 71
          call $_Exit
          unreachable
        end
        i32.const 70
        call $_Exit
        unreachable
      end
      local.get 0
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $internal_register_preopened_fd_unlocked (;172;) (type 6) (param i32 i32) (result i32)
      (local i32 i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.const 2
          i32.add
          br_table 1 (;@1;) 1 (;@1;) 0 (;@2;)
        end
        local.get 1
        i32.eqz
        br_if 0 (;@1;)
        block ;; label = @2
          i32.const 0
          i32.load offset=1056932
          local.tee 2
          i32.const 0
          i32.load offset=1056944
          i32.ne
          br_if 0 (;@2;)
          i32.const 0
          i32.load offset=1056936
          local.set 3
          block ;; label = @3
            i32.const 8
            local.get 2
            i32.const 1
            i32.shl
            i32.const 4
            local.get 2
            select
            local.tee 4
            call $calloc
            local.tee 5
            br_if 0 (;@3;)
            i32.const -1
            return
          end
          local.get 5
          local.get 3
          local.get 2
          i32.const 3
          i32.shl
          call $memcpy
          local.set 5
          i32.const 0
          local.get 4
          i32.store offset=1056944
          i32.const 0
          local.get 5
          i32.store offset=1056936
          local.get 3
          call $free
        end
        block ;; label = @2
          loop ;; label = @3
            block ;; label = @4
              block ;; label = @5
                local.get 1
                local.tee 3
                i32.load8_u
                i32.const -46
                i32.add
                br_table 1 (;@4;) 0 (;@5;) 3 (;@2;)
              end
              local.get 3
              i32.const 1
              i32.add
              local.set 1
              br 1 (;@3;)
            end
            local.get 3
            i32.const 1
            i32.add
            local.set 1
            local.get 3
            i32.load8_u offset=1
            local.tee 4
            i32.eqz
            br_if 0 (;@3;)
            local.get 4
            i32.const 47
            i32.ne
            br_if 1 (;@2;)
            local.get 3
            i32.const 2
            i32.add
            local.set 1
            br 0 (;@3;)
          end
        end
        block ;; label = @2
          local.get 3
          call $strdup
          local.tee 3
          br_if 0 (;@2;)
          i32.const -1
          return
        end
        i32.const 0
        local.get 2
        i32.const 1
        i32.add
        i32.store offset=1056932
        i32.const 0
        i32.load offset=1056936
        local.get 2
        i32.const 3
        i32.shl
        i32.add
        local.tee 1
        local.get 0
        i32.store offset=4
        local.get 1
        local.get 3
        i32.store
        i32.const 0
        return
      end
      call $abort
      unreachable
    )
    (func $__wasilibc_find_relpath (;173;) (type 10) (param i32 i32 i32 i32) (result i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 4
      global.set $__stack_pointer
      local.get 4
      local.get 3
      i32.store offset=12
      block ;; label = @1
        block ;; label = @2
          i32.const 81
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          local.get 1
          local.get 2
          local.get 4
          i32.const 12
          i32.add
          i32.const 0
          call $__wasilibc_find_relpath_alloc
          local.set 3
          br 1 (;@1;)
        end
        local.get 0
        local.get 1
        local.get 2
        call $__wasilibc_find_abspath
        local.set 3
      end
      local.get 4
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 3
    )
    (func $__wasilibc_find_abspath (;174;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
      local.get 0
      i32.const -1
      i32.add
      local.set 3
      call $__wasilibc_populate_preopens
      loop ;; label = @1
        local.get 3
        i32.const 1
        i32.add
        local.tee 3
        i32.load8_u
        i32.const 47
        i32.eq
        br_if 0 (;@1;)
      end
      i32.const 0
      local.set 4
      block ;; label = @1
        block ;; label = @2
          i32.const 0
          i32.load offset=1056932
          local.tee 5
          i32.eqz
          br_if 0 (;@2;)
          i32.const 0
          i32.load offset=1056936
          local.set 6
          i32.const -1
          local.set 7
          loop ;; label = @3
            local.get 6
            local.get 5
            i32.const -1
            i32.add
            local.tee 5
            i32.const 3
            i32.shl
            i32.add
            local.tee 8
            i32.load
            local.tee 9
            call $strlen
            local.set 10
            block ;; label = @4
              block ;; label = @5
                local.get 7
                i32.const -1
                i32.eq
                br_if 0 (;@5;)
                local.get 10
                local.get 4
                i32.le_u
                br_if 1 (;@4;)
              end
              block ;; label = @5
                block ;; label = @6
                  local.get 10
                  br_if 0 (;@6;)
                  local.get 3
                  i32.load8_u
                  i32.const 255
                  i32.and
                  i32.const 47
                  i32.ne
                  br_if 1 (;@5;)
                end
                local.get 3
                local.get 9
                local.get 10
                call $memcmp
                br_if 1 (;@4;)
                local.get 9
                i32.const -1
                i32.add
                local.set 11
                local.get 10
                local.set 12
                block ;; label = @6
                  loop ;; label = @7
                    local.get 12
                    local.tee 0
                    i32.eqz
                    br_if 1 (;@6;)
                    local.get 0
                    i32.const -1
                    i32.add
                    local.set 12
                    local.get 11
                    local.get 0
                    i32.add
                    i32.load8_u
                    i32.const 47
                    i32.eq
                    br_if 0 (;@7;)
                  end
                end
                local.get 3
                local.get 0
                i32.add
                i32.load8_u
                local.tee 0
                i32.const 47
                i32.eq
                br_if 0 (;@5;)
                local.get 0
                br_if 1 (;@4;)
              end
              local.get 1
              local.get 9
              i32.store
              local.get 8
              i32.load offset=4
              local.set 7
              local.get 10
              local.set 4
            end
            local.get 5
            br_if 0 (;@3;)
          end
          local.get 7
          i32.const -1
          i32.ne
          br_if 1 (;@1;)
        end
        i32.const 0
        i32.const 44
        i32.store offset=1056916
        i32.const -1
        return
      end
      local.get 3
      local.get 4
      i32.add
      i32.const -1
      i32.add
      local.set 0
      loop ;; label = @1
        local.get 0
        i32.const 1
        i32.add
        local.tee 0
        i32.load8_u
        local.tee 3
        i32.const 47
        i32.eq
        br_if 0 (;@1;)
      end
      local.get 2
      local.get 0
      i32.const 1051839
      local.get 3
      select
      i32.store
      local.get 7
    )
    (func $sbrk (;175;) (type 3) (param i32) (result i32)
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
          i32.store offset=1056916
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
    (func $getenv (;176;) (type 3) (param i32) (result i32)
      (local i32 i32 i32 i32)
      call $__wasilibc_ensure_environ
      block ;; label = @1
        local.get 0
        i32.const 61
        call $__strchrnul
        local.tee 1
        local.get 0
        i32.ne
        br_if 0 (;@1;)
        i32.const 0
        return
      end
      i32.const 0
      local.set 2
      block ;; label = @1
        local.get 0
        local.get 1
        local.get 0
        i32.sub
        local.tee 3
        i32.add
        i32.load8_u
        br_if 0 (;@1;)
        i32.const 0
        i32.load offset=1056296
        local.tee 4
        i32.eqz
        br_if 0 (;@1;)
        local.get 4
        i32.load
        local.tee 1
        i32.eqz
        br_if 0 (;@1;)
        local.get 4
        i32.const 4
        i32.add
        local.set 4
        block ;; label = @2
          loop ;; label = @3
            block ;; label = @4
              local.get 0
              local.get 1
              local.get 3
              call $strncmp
              br_if 0 (;@4;)
              local.get 1
              local.get 3
              i32.add
              local.tee 1
              i32.load8_u
              i32.const 61
              i32.eq
              br_if 2 (;@2;)
            end
            local.get 4
            i32.load
            local.set 1
            local.get 4
            i32.const 4
            i32.add
            local.set 4
            local.get 1
            br_if 0 (;@3;)
            br 2 (;@1;)
          end
        end
        local.get 1
        i32.const 1
        i32.add
        local.set 2
      end
      local.get 2
    )
    (func $dummy (;177;) (type $.rodata))
    (func $__wasm_call_dtors (;178;) (type $.rodata)
      call $dummy
      call $dummy
    )
    (func $memcmp (;179;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32 i32)
      i32.const 0
      local.set 3
      block ;; label = @1
        local.get 2
        i32.eqz
        br_if 0 (;@1;)
        block ;; label = @2
          loop ;; label = @3
            local.get 0
            i32.load8_u
            local.tee 4
            local.get 1
            i32.load8_u
            local.tee 5
            i32.ne
            br_if 1 (;@2;)
            local.get 1
            i32.const 1
            i32.add
            local.set 1
            local.get 0
            i32.const 1
            i32.add
            local.set 0
            local.get 2
            i32.const -1
            i32.add
            local.tee 2
            br_if 0 (;@3;)
            br 2 (;@1;)
          end
        end
        local.get 4
        local.get 5
        i32.sub
        local.set 3
      end
      local.get 3
    )
    (func $memcpy (;180;) (type 5) (param i32 i32 i32) (result i32)
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
    (func $memmove (;181;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 2
            i32.const 33
            i32.ge_u
            br_if 0 (;@3;)
            local.get 0
            local.get 1
            i32.eq
            br_if 2 (;@1;)
            local.get 1
            local.get 0
            local.get 2
            i32.add
            local.tee 3
            i32.sub
            i32.const 0
            local.get 2
            i32.const 1
            i32.shl
            i32.sub
            i32.gt_u
            br_if 1 (;@2;)
          end
          local.get 0
          local.get 1
          local.get 2
          memory.copy
          br 1 (;@1;)
        end
        local.get 1
        local.get 0
        i32.xor
        i32.const 3
        i32.and
        local.set 4
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 0
              local.get 1
              i32.ge_u
              br_if 0 (;@4;)
              block ;; label = @5
                local.get 4
                i32.eqz
                br_if 0 (;@5;)
                local.get 2
                local.set 5
                local.get 0
                local.set 3
                br 3 (;@2;)
              end
              block ;; label = @5
                local.get 0
                i32.const 3
                i32.and
                br_if 0 (;@5;)
                local.get 2
                local.set 5
                local.get 0
                local.set 3
                br 2 (;@3;)
              end
              local.get 2
              i32.eqz
              br_if 3 (;@1;)
              local.get 0
              local.get 1
              i32.load8_u
              i32.store8
              local.get 2
              i32.const -1
              i32.add
              local.set 5
              block ;; label = @5
                local.get 0
                i32.const 1
                i32.add
                local.tee 3
                i32.const 3
                i32.and
                br_if 0 (;@5;)
                local.get 1
                i32.const 1
                i32.add
                local.set 1
                br 2 (;@3;)
              end
              local.get 5
              i32.eqz
              br_if 3 (;@1;)
              local.get 0
              local.get 1
              i32.load8_u offset=1
              i32.store8 offset=1
              local.get 2
              i32.const -2
              i32.add
              local.set 5
              block ;; label = @5
                local.get 0
                i32.const 2
                i32.add
                local.tee 3
                i32.const 3
                i32.and
                br_if 0 (;@5;)
                local.get 1
                i32.const 2
                i32.add
                local.set 1
                br 2 (;@3;)
              end
              local.get 5
              i32.eqz
              br_if 3 (;@1;)
              local.get 0
              local.get 1
              i32.load8_u offset=2
              i32.store8 offset=2
              local.get 2
              i32.const -3
              i32.add
              local.set 5
              block ;; label = @5
                local.get 0
                i32.const 3
                i32.add
                local.tee 3
                i32.const 3
                i32.and
                br_if 0 (;@5;)
                local.get 1
                i32.const 3
                i32.add
                local.set 1
                br 2 (;@3;)
              end
              local.get 5
              i32.eqz
              br_if 3 (;@1;)
              local.get 0
              local.get 1
              i32.load8_u offset=3
              i32.store8 offset=3
              local.get 0
              i32.const 4
              i32.add
              local.set 3
              local.get 1
              i32.const 4
              i32.add
              local.set 1
              local.get 2
              i32.const -4
              i32.add
              local.set 5
              br 1 (;@3;)
            end
            block ;; label = @4
              local.get 4
              br_if 0 (;@4;)
              block ;; label = @5
                local.get 3
                i32.const 3
                i32.and
                i32.eqz
                br_if 0 (;@5;)
                local.get 2
                i32.eqz
                br_if 4 (;@1;)
                local.get 0
                local.get 2
                i32.const -1
                i32.add
                local.tee 3
                i32.add
                local.tee 4
                local.get 1
                local.get 3
                i32.add
                i32.load8_u
                i32.store8
                block ;; label = @6
                  local.get 4
                  i32.const 3
                  i32.and
                  br_if 0 (;@6;)
                  local.get 3
                  local.set 2
                  br 1 (;@5;)
                end
                local.get 3
                i32.eqz
                br_if 4 (;@1;)
                local.get 0
                local.get 2
                i32.const -2
                i32.add
                local.tee 3
                i32.add
                local.tee 4
                local.get 1
                local.get 3
                i32.add
                i32.load8_u
                i32.store8
                block ;; label = @6
                  local.get 4
                  i32.const 3
                  i32.and
                  br_if 0 (;@6;)
                  local.get 3
                  local.set 2
                  br 1 (;@5;)
                end
                local.get 3
                i32.eqz
                br_if 4 (;@1;)
                local.get 0
                local.get 2
                i32.const -3
                i32.add
                local.tee 3
                i32.add
                local.tee 4
                local.get 1
                local.get 3
                i32.add
                i32.load8_u
                i32.store8
                block ;; label = @6
                  local.get 4
                  i32.const 3
                  i32.and
                  br_if 0 (;@6;)
                  local.get 3
                  local.set 2
                  br 1 (;@5;)
                end
                local.get 3
                i32.eqz
                br_if 4 (;@1;)
                local.get 0
                local.get 2
                i32.const -4
                i32.add
                local.tee 2
                i32.add
                local.get 1
                local.get 2
                i32.add
                i32.load8_u
                i32.store8
              end
              local.get 2
              i32.const 4
              i32.lt_u
              br_if 0 (;@4;)
              block ;; label = @5
                local.get 2
                i32.const -4
                i32.add
                local.tee 6
                i32.const 2
                i32.shr_u
                i32.const 1
                i32.add
                i32.const 3
                i32.and
                local.tee 3
                i32.eqz
                br_if 0 (;@5;)
                local.get 1
                i32.const -4
                i32.add
                local.set 4
                local.get 0
                i32.const -4
                i32.add
                local.set 5
                loop ;; label = @6
                  local.get 5
                  local.get 2
                  i32.add
                  local.get 4
                  local.get 2
                  i32.add
                  i32.load
                  i32.store
                  local.get 2
                  i32.const -4
                  i32.add
                  local.set 2
                  local.get 3
                  i32.const -1
                  i32.add
                  local.tee 3
                  br_if 0 (;@6;)
                end
              end
              local.get 6
              i32.const 12
              i32.lt_u
              br_if 0 (;@4;)
              local.get 1
              i32.const -16
              i32.add
              local.set 5
              local.get 0
              i32.const -16
              i32.add
              local.set 6
              loop ;; label = @5
                local.get 6
                local.get 2
                i32.add
                local.tee 3
                i32.const 12
                i32.add
                local.get 5
                local.get 2
                i32.add
                local.tee 4
                i32.const 12
                i32.add
                i32.load
                i32.store
                local.get 3
                i32.const 8
                i32.add
                local.get 4
                i32.const 8
                i32.add
                i32.load
                i32.store
                local.get 3
                i32.const 4
                i32.add
                local.get 4
                i32.const 4
                i32.add
                i32.load
                i32.store
                local.get 3
                local.get 4
                i32.load
                i32.store
                local.get 2
                i32.const -16
                i32.add
                local.tee 2
                i32.const 3
                i32.gt_u
                br_if 0 (;@5;)
              end
            end
            local.get 2
            i32.eqz
            br_if 2 (;@1;)
            local.get 2
            local.set 3
            block ;; label = @4
              local.get 2
              i32.const 3
              i32.and
              local.tee 4
              i32.eqz
              br_if 0 (;@4;)
              local.get 1
              i32.const -1
              i32.add
              local.set 5
              local.get 0
              i32.const -1
              i32.add
              local.set 6
              local.get 2
              local.set 3
              loop ;; label = @5
                local.get 6
                local.get 3
                i32.add
                local.get 5
                local.get 3
                i32.add
                i32.load8_u
                i32.store8
                local.get 3
                i32.const -1
                i32.add
                local.set 3
                local.get 4
                i32.const -1
                i32.add
                local.tee 4
                br_if 0 (;@5;)
              end
            end
            local.get 2
            i32.const 4
            i32.lt_u
            br_if 2 (;@1;)
            local.get 1
            i32.const -4
            i32.add
            local.set 4
            local.get 0
            i32.const -4
            i32.add
            local.set 5
            loop ;; label = @4
              local.get 5
              local.get 3
              i32.add
              local.tee 1
              i32.const 3
              i32.add
              local.get 4
              local.get 3
              i32.add
              local.tee 2
              i32.const 3
              i32.add
              i32.load8_u
              i32.store8
              local.get 1
              i32.const 2
              i32.add
              local.get 2
              i32.const 2
              i32.add
              i32.load8_u
              i32.store8
              local.get 1
              i32.const 1
              i32.add
              local.get 2
              i32.const 1
              i32.add
              i32.load8_u
              i32.store8
              local.get 1
              local.get 2
              i32.load8_u
              i32.store8
              local.get 3
              i32.const -4
              i32.add
              local.tee 3
              br_if 0 (;@4;)
              br 3 (;@1;)
            end
          end
          local.get 5
          i32.const 4
          i32.lt_u
          br_if 0 (;@2;)
          block ;; label = @3
            local.get 5
            i32.const -4
            i32.add
            local.tee 4
            i32.const 2
            i32.shr_u
            i32.const 1
            i32.add
            i32.const 7
            i32.and
            local.tee 2
            i32.eqz
            br_if 0 (;@3;)
            loop ;; label = @4
              local.get 3
              local.get 1
              i32.load
              i32.store
              local.get 1
              i32.const 4
              i32.add
              local.set 1
              local.get 3
              i32.const 4
              i32.add
              local.set 3
              local.get 5
              i32.const -4
              i32.add
              local.set 5
              local.get 2
              i32.const -1
              i32.add
              local.tee 2
              br_if 0 (;@4;)
            end
          end
          local.get 4
          i32.const 28
          i32.lt_u
          br_if 0 (;@2;)
          loop ;; label = @3
            local.get 3
            local.get 1
            i32.load
            i32.store
            local.get 3
            local.get 1
            i32.load offset=4
            i32.store offset=4
            local.get 3
            local.get 1
            i32.load offset=8
            i32.store offset=8
            local.get 3
            local.get 1
            i32.load offset=12
            i32.store offset=12
            local.get 3
            local.get 1
            i32.load offset=16
            i32.store offset=16
            local.get 3
            local.get 1
            i32.load offset=20
            i32.store offset=20
            local.get 3
            local.get 1
            i32.load offset=24
            i32.store offset=24
            local.get 3
            local.get 1
            i32.load offset=28
            i32.store offset=28
            local.get 1
            i32.const 32
            i32.add
            local.set 1
            local.get 3
            i32.const 32
            i32.add
            local.set 3
            local.get 5
            i32.const -32
            i32.add
            local.tee 5
            i32.const 3
            i32.gt_u
            br_if 0 (;@3;)
          end
        end
        local.get 5
        i32.eqz
        br_if 0 (;@1;)
        block ;; label = @2
          block ;; label = @3
            local.get 5
            i32.const 7
            i32.and
            local.tee 4
            br_if 0 (;@3;)
            local.get 5
            local.set 2
            br 1 (;@2;)
          end
          local.get 5
          local.set 2
          loop ;; label = @3
            local.get 3
            local.get 1
            i32.load8_u
            i32.store8
            local.get 2
            i32.const -1
            i32.add
            local.set 2
            local.get 3
            i32.const 1
            i32.add
            local.set 3
            local.get 1
            i32.const 1
            i32.add
            local.set 1
            local.get 4
            i32.const -1
            i32.add
            local.tee 4
            br_if 0 (;@3;)
          end
        end
        local.get 5
        i32.const 8
        i32.lt_u
        br_if 0 (;@1;)
        loop ;; label = @2
          local.get 3
          local.get 1
          i32.load8_u
          i32.store8
          local.get 3
          local.get 1
          i32.load8_u offset=1
          i32.store8 offset=1
          local.get 3
          local.get 1
          i32.load8_u offset=2
          i32.store8 offset=2
          local.get 3
          local.get 1
          i32.load8_u offset=3
          i32.store8 offset=3
          local.get 3
          local.get 1
          i32.load8_u offset=4
          i32.store8 offset=4
          local.get 3
          local.get 1
          i32.load8_u offset=5
          i32.store8 offset=5
          local.get 3
          local.get 1
          i32.load8_u offset=6
          i32.store8 offset=6
          local.get 3
          local.get 1
          i32.load8_u offset=7
          i32.store8 offset=7
          local.get 3
          i32.const 8
          i32.add
          local.set 3
          local.get 1
          i32.const 8
          i32.add
          local.set 1
          local.get 2
          i32.const -8
          i32.add
          local.tee 2
          br_if 0 (;@2;)
        end
      end
      local.get 0
    )
    (func $memset (;182;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i64)
      block ;; label = @1
        local.get 2
        i32.const 33
        i32.lt_u
        br_if 0 (;@1;)
        local.get 0
        local.get 1
        local.get 2
        memory.fill
        local.get 0
        return
      end
      block ;; label = @1
        local.get 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        local.get 1
        i32.store8
        local.get 2
        local.get 0
        i32.add
        local.tee 3
        i32.const -1
        i32.add
        local.get 1
        i32.store8
        local.get 2
        i32.const 3
        i32.lt_u
        br_if 0 (;@1;)
        local.get 0
        local.get 1
        i32.store8 offset=2
        local.get 0
        local.get 1
        i32.store8 offset=1
        local.get 3
        i32.const -3
        i32.add
        local.get 1
        i32.store8
        local.get 3
        i32.const -2
        i32.add
        local.get 1
        i32.store8
        local.get 2
        i32.const 7
        i32.lt_u
        br_if 0 (;@1;)
        local.get 0
        local.get 1
        i32.store8 offset=3
        local.get 3
        i32.const -4
        i32.add
        local.get 1
        i32.store8
        local.get 2
        i32.const 9
        i32.lt_u
        br_if 0 (;@1;)
        local.get 0
        i32.const 0
        local.get 0
        i32.sub
        i32.const 3
        i32.and
        local.tee 4
        i32.add
        local.tee 5
        local.get 1
        i32.const 255
        i32.and
        i32.const 16843009
        i32.mul
        local.tee 3
        i32.store
        local.get 5
        local.get 2
        local.get 4
        i32.sub
        i32.const -4
        i32.and
        local.tee 1
        i32.add
        local.tee 2
        i32.const -4
        i32.add
        local.get 3
        i32.store
        local.get 1
        i32.const 9
        i32.lt_u
        br_if 0 (;@1;)
        local.get 5
        local.get 3
        i32.store offset=8
        local.get 5
        local.get 3
        i32.store offset=4
        local.get 2
        i32.const -8
        i32.add
        local.get 3
        i32.store
        local.get 2
        i32.const -12
        i32.add
        local.get 3
        i32.store
        local.get 1
        i32.const 25
        i32.lt_u
        br_if 0 (;@1;)
        local.get 5
        local.get 3
        i32.store offset=24
        local.get 5
        local.get 3
        i32.store offset=20
        local.get 5
        local.get 3
        i32.store offset=16
        local.get 5
        local.get 3
        i32.store offset=12
        local.get 2
        i32.const -16
        i32.add
        local.get 3
        i32.store
        local.get 2
        i32.const -20
        i32.add
        local.get 3
        i32.store
        local.get 2
        i32.const -24
        i32.add
        local.get 3
        i32.store
        local.get 2
        i32.const -28
        i32.add
        local.get 3
        i32.store
        local.get 1
        local.get 5
        i32.const 4
        i32.and
        i32.const 24
        i32.or
        local.tee 2
        i32.sub
        local.tee 1
        i32.const 32
        i32.lt_u
        br_if 0 (;@1;)
        local.get 3
        i64.extend_i32_u
        i64.const 4294967297
        i64.mul
        local.set 6
        local.get 5
        local.get 2
        i32.add
        local.set 2
        loop ;; label = @2
          local.get 2
          local.get 6
          i64.store offset=24
          local.get 2
          local.get 6
          i64.store offset=16
          local.get 2
          local.get 6
          i64.store offset=8
          local.get 2
          local.get 6
          i64.store
          local.get 2
          i32.const 32
          i32.add
          local.set 2
          local.get 1
          i32.const -32
          i32.add
          local.tee 1
          i32.const 31
          i32.gt_u
          br_if 0 (;@2;)
        end
      end
      local.get 0
    )
    (func $__strchrnul (;183;) (type 6) (param i32 i32) (result i32)
      (local i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 1
              i32.const 255
              i32.and
              local.tee 2
              i32.eqz
              br_if 0 (;@4;)
              local.get 0
              i32.const 3
              i32.and
              i32.eqz
              br_if 2 (;@2;)
              block ;; label = @5
                local.get 0
                i32.load8_u
                local.tee 3
                br_if 0 (;@5;)
                local.get 0
                return
              end
              local.get 3
              local.get 1
              i32.const 255
              i32.and
              i32.ne
              br_if 1 (;@3;)
              local.get 0
              return
            end
            local.get 0
            local.get 0
            call $strlen
            i32.add
            return
          end
          block ;; label = @3
            local.get 0
            i32.const 1
            i32.add
            local.tee 3
            i32.const 3
            i32.and
            br_if 0 (;@3;)
            local.get 3
            local.set 0
            br 1 (;@2;)
          end
          local.get 3
          i32.load8_u
          local.tee 4
          i32.eqz
          br_if 1 (;@1;)
          local.get 4
          local.get 1
          i32.const 255
          i32.and
          i32.eq
          br_if 1 (;@1;)
          block ;; label = @3
            local.get 0
            i32.const 2
            i32.add
            local.tee 3
            i32.const 3
            i32.and
            br_if 0 (;@3;)
            local.get 3
            local.set 0
            br 1 (;@2;)
          end
          local.get 3
          i32.load8_u
          local.tee 4
          i32.eqz
          br_if 1 (;@1;)
          local.get 4
          local.get 1
          i32.const 255
          i32.and
          i32.eq
          br_if 1 (;@1;)
          block ;; label = @3
            local.get 0
            i32.const 3
            i32.add
            local.tee 3
            i32.const 3
            i32.and
            br_if 0 (;@3;)
            local.get 3
            local.set 0
            br 1 (;@2;)
          end
          local.get 3
          i32.load8_u
          local.tee 4
          i32.eqz
          br_if 1 (;@1;)
          local.get 4
          local.get 1
          i32.const 255
          i32.and
          i32.eq
          br_if 1 (;@1;)
          local.get 0
          i32.const 4
          i32.add
          local.set 0
        end
        block ;; label = @2
          local.get 0
          i32.load
          local.tee 3
          i32.const -1
          i32.xor
          local.get 3
          i32.const -16843009
          i32.add
          i32.and
          i32.const -2139062144
          i32.and
          br_if 0 (;@2;)
          local.get 2
          i32.const 16843009
          i32.mul
          local.set 2
          loop ;; label = @3
            local.get 3
            local.get 2
            i32.xor
            local.tee 3
            i32.const -1
            i32.xor
            local.get 3
            i32.const -16843009
            i32.add
            i32.and
            i32.const -2139062144
            i32.and
            br_if 1 (;@2;)
            local.get 0
            i32.const 4
            i32.add
            local.tee 0
            i32.load
            local.tee 3
            i32.const -1
            i32.xor
            local.get 3
            i32.const -16843009
            i32.add
            i32.and
            i32.const -2139062144
            i32.and
            i32.eqz
            br_if 0 (;@3;)
          end
        end
        local.get 0
        i32.const -1
        i32.add
        local.set 3
        loop ;; label = @2
          local.get 3
          i32.const 1
          i32.add
          local.tee 3
          i32.load8_u
          local.tee 0
          i32.eqz
          br_if 1 (;@1;)
          local.get 0
          local.get 1
          i32.const 255
          i32.and
          i32.ne
          br_if 0 (;@2;)
        end
      end
      local.get 3
    )
    (func $strcmp (;184;) (type 6) (param i32 i32) (result i32)
      (local i32 i32)
      local.get 1
      i32.load8_u
      local.set 2
      block ;; label = @1
        local.get 0
        i32.load8_u
        local.tee 3
        i32.eqz
        br_if 0 (;@1;)
        local.get 3
        local.get 2
        i32.const 255
        i32.and
        i32.ne
        br_if 0 (;@1;)
        local.get 0
        i32.const 1
        i32.add
        local.set 0
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        loop ;; label = @2
          local.get 1
          i32.load8_u
          local.set 2
          local.get 0
          i32.load8_u
          local.tee 3
          i32.eqz
          br_if 1 (;@1;)
          local.get 0
          i32.const 1
          i32.add
          local.set 0
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 3
          local.get 2
          i32.const 255
          i32.and
          i32.eq
          br_if 0 (;@2;)
        end
      end
      local.get 3
      local.get 2
      i32.const 255
      i32.and
      i32.sub
    )
    (func $__stpcpy (;185;) (type 6) (param i32 i32) (result i32)
      (local i32 i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 1
            local.get 0
            i32.xor
            i32.const 3
            i32.and
            i32.eqz
            br_if 0 (;@3;)
            local.get 1
            i32.load8_u
            local.set 2
            br 1 (;@2;)
          end
          block ;; label = @3
            local.get 1
            i32.const 3
            i32.and
            i32.eqz
            br_if 0 (;@3;)
            local.get 0
            local.get 1
            i32.load8_u
            local.tee 2
            i32.store8
            block ;; label = @4
              local.get 2
              br_if 0 (;@4;)
              local.get 0
              return
            end
            local.get 0
            i32.const 1
            i32.add
            local.set 2
            block ;; label = @4
              local.get 1
              i32.const 1
              i32.add
              local.tee 3
              i32.const 3
              i32.and
              br_if 0 (;@4;)
              local.get 2
              local.set 0
              local.get 3
              local.set 1
              br 1 (;@3;)
            end
            local.get 2
            local.get 3
            i32.load8_u
            local.tee 3
            i32.store8
            local.get 3
            i32.eqz
            br_if 2 (;@1;)
            local.get 0
            i32.const 2
            i32.add
            local.set 2
            block ;; label = @4
              local.get 1
              i32.const 2
              i32.add
              local.tee 3
              i32.const 3
              i32.and
              br_if 0 (;@4;)
              local.get 2
              local.set 0
              local.get 3
              local.set 1
              br 1 (;@3;)
            end
            local.get 2
            local.get 3
            i32.load8_u
            local.tee 3
            i32.store8
            local.get 3
            i32.eqz
            br_if 2 (;@1;)
            local.get 0
            i32.const 3
            i32.add
            local.set 2
            block ;; label = @4
              local.get 1
              i32.const 3
              i32.add
              local.tee 3
              i32.const 3
              i32.and
              br_if 0 (;@4;)
              local.get 2
              local.set 0
              local.get 3
              local.set 1
              br 1 (;@3;)
            end
            local.get 2
            local.get 3
            i32.load8_u
            local.tee 3
            i32.store8
            local.get 3
            i32.eqz
            br_if 2 (;@1;)
            local.get 0
            i32.const 4
            i32.add
            local.set 0
            local.get 1
            i32.const 4
            i32.add
            local.set 1
          end
          local.get 1
          i32.load
          local.tee 2
          i32.const -1
          i32.xor
          local.get 2
          i32.const -16843009
          i32.add
          i32.and
          i32.const -2139062144
          i32.and
          br_if 0 (;@2;)
          loop ;; label = @3
            local.get 0
            local.get 2
            i32.store
            local.get 0
            i32.const 4
            i32.add
            local.set 0
            local.get 1
            i32.const 4
            i32.add
            local.tee 1
            i32.load
            local.tee 2
            i32.const -1
            i32.xor
            local.get 2
            i32.const -16843009
            i32.add
            i32.and
            i32.const -2139062144
            i32.and
            i32.eqz
            br_if 0 (;@3;)
          end
        end
        local.get 0
        local.get 2
        i32.store8
        block ;; label = @2
          local.get 2
          i32.const 255
          i32.and
          br_if 0 (;@2;)
          local.get 0
          return
        end
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 0
        local.set 2
        loop ;; label = @2
          local.get 2
          local.get 1
          i32.load8_u
          local.tee 0
          i32.store8 offset=1
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 2
          i32.const 1
          i32.add
          local.set 2
          local.get 0
          br_if 0 (;@2;)
        end
      end
      local.get 2
    )
    (func $strcpy (;186;) (type 6) (param i32 i32) (result i32)
      local.get 0
      local.get 1
      call $__stpcpy
      drop
      local.get 0
    )
    (func $strdup (;187;) (type 3) (param i32) (result i32)
      (local i32 i32)
      block ;; label = @1
        local.get 0
        call $strlen
        i32.const 1
        i32.add
        local.tee 1
        call $malloc
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 2
        local.get 0
        local.get 1
        call $memcpy
        drop
      end
      local.get 2
    )
    (func $strlen (;188;) (type 3) (param i32) (result i32)
      (local i32 i32)
      local.get 0
      local.set 1
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          local.set 1
          local.get 0
          i32.load8_u
          i32.eqz
          br_if 1 (;@1;)
          local.get 0
          i32.const 1
          i32.add
          local.tee 1
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@2;)
          local.get 1
          i32.load8_u
          i32.eqz
          br_if 1 (;@1;)
          local.get 0
          i32.const 2
          i32.add
          local.tee 1
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@2;)
          local.get 1
          i32.load8_u
          i32.eqz
          br_if 1 (;@1;)
          local.get 0
          i32.const 3
          i32.add
          local.tee 1
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@2;)
          local.get 1
          i32.load8_u
          i32.eqz
          br_if 1 (;@1;)
          local.get 0
          i32.const 4
          i32.add
          local.set 1
        end
        local.get 1
        i32.const -5
        i32.add
        local.set 1
        loop ;; label = @2
          local.get 1
          i32.const 5
          i32.add
          local.set 2
          local.get 1
          i32.const 4
          i32.add
          local.set 1
          local.get 2
          i32.load
          local.tee 2
          i32.const -1
          i32.xor
          local.get 2
          i32.const -16843009
          i32.add
          i32.and
          i32.const -2139062144
          i32.and
          i32.eqz
          br_if 0 (;@2;)
        end
        loop ;; label = @2
          local.get 1
          i32.const 1
          i32.add
          local.tee 1
          i32.load8_u
          br_if 0 (;@2;)
        end
      end
      local.get 1
      local.get 0
      i32.sub
    )
    (func $strncmp (;189;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32 i32)
      block ;; label = @1
        local.get 2
        br_if 0 (;@1;)
        i32.const 0
        return
      end
      i32.const 0
      local.set 3
      block ;; label = @1
        local.get 0
        i32.load8_u
        local.tee 4
        i32.eqz
        br_if 0 (;@1;)
        local.get 0
        i32.const 1
        i32.add
        local.set 0
        local.get 2
        i32.const -1
        i32.add
        local.set 2
        loop ;; label = @2
          block ;; label = @3
            local.get 1
            i32.load8_u
            local.tee 5
            br_if 0 (;@3;)
            local.get 4
            local.set 3
            br 2 (;@1;)
          end
          block ;; label = @3
            local.get 2
            br_if 0 (;@3;)
            local.get 4
            local.set 3
            br 2 (;@1;)
          end
          block ;; label = @3
            local.get 4
            i32.const 255
            i32.and
            local.get 5
            i32.eq
            br_if 0 (;@3;)
            local.get 4
            local.set 3
            br 2 (;@1;)
          end
          local.get 2
          i32.const -1
          i32.add
          local.set 2
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 0
          i32.load8_u
          local.set 4
          local.get 0
          i32.const 1
          i32.add
          local.set 0
          local.get 4
          br_if 0 (;@2;)
        end
      end
      local.get 3
      i32.const 255
      i32.and
      local.get 1
      i32.load8_u
      i32.sub
    )
    (func $<&T as core::fmt::Debug>::fmt (;190;) (type 6) (param i32 i32) (result i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 0
      i32.load
      local.tee 0
      i32.load offset=8
      local.set 3
      local.get 0
      i32.load
      local.set 0
      local.get 2
      i32.const 4
      i32.add
      local.get 1
      call $core::fmt::Formatter::debug_list
      block ;; label = @1
        local.get 3
        i32.eqz
        br_if 0 (;@1;)
        loop ;; label = @2
          local.get 2
          local.get 0
          i32.store offset=12
          local.get 2
          i32.const 4
          i32.add
          local.get 2
          i32.const 12
          i32.add
          i32.const 1051868
          call $core::fmt::builders::DebugSet::entry
          drop
          local.get 0
          i32.const 1
          i32.add
          local.set 0
          local.get 3
          i32.const -1
          i32.add
          local.tee 3
          br_if 0 (;@2;)
        end
      end
      local.get 2
      i32.const 4
      i32.add
      call $core::fmt::builders::DebugList::finish
      local.set 0
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $<&T as core::fmt::Debug>::fmt (;191;) (type 6) (param i32 i32) (result i32)
      local.get 0
      i32.load
      local.set 0
      block ;; label = @1
        local.get 1
        call $core::fmt::Formatter::debug_lower_hex
        br_if 0 (;@1;)
        block ;; label = @2
          local.get 1
          call $core::fmt::Formatter::debug_upper_hex
          br_if 0 (;@2;)
          local.get 0
          local.get 1
          call $core::fmt::num::imp::<impl core::fmt::Display for u8>::fmt
          return
        end
        local.get 0
        local.get 1
        call $core::fmt::num::<impl core::fmt::UpperHex for i8>::fmt
        return
      end
      local.get 0
      local.get 1
      call $core::fmt::num::<impl core::fmt::LowerHex for i8>::fmt
    )
    (func $core::fmt::num::<impl core::fmt::Debug for usize>::fmt (;192;) (type 6) (param i32 i32) (result i32)
      block ;; label = @1
        local.get 1
        call $core::fmt::Formatter::debug_lower_hex
        br_if 0 (;@1;)
        block ;; label = @2
          local.get 1
          call $core::fmt::Formatter::debug_upper_hex
          br_if 0 (;@2;)
          local.get 0
          local.get 1
          call $core::fmt::num::imp::<impl core::fmt::Display for u32>::fmt
          return
        end
        local.get 0
        local.get 1
        call $core::fmt::num::<impl core::fmt::UpperHex for i32>::fmt
        return
      end
      local.get 0
      local.get 1
      call $core::fmt::num::<impl core::fmt::LowerHex for i32>::fmt
    )
    (func $core::ptr::drop_in_place<usize> (;193;) (type $.data) (param i32))
    (func $core::ptr::drop_in_place<core::fmt::Error> (;194;) (type $.data) (param i32))
    (func $<&mut W as core::fmt::Write>::write_char (;195;) (type 6) (param i32 i32) (result i32)
      local.get 0
      i32.load
      local.get 1
      call $alloc::string::String::push
      i32.const 0
    )
    (func $alloc::string::String::push (;196;) (type 2) (param i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 1
              i32.const 128
              i32.lt_u
              br_if 0 (;@4;)
              local.get 2
              i32.const 0
              i32.store offset=12
              local.get 1
              i32.const 2048
              i32.lt_u
              br_if 1 (;@3;)
              block ;; label = @5
                local.get 1
                i32.const 65536
                i32.ge_u
                br_if 0 (;@5;)
                local.get 2
                local.get 1
                i32.const 63
                i32.and
                i32.const 128
                i32.or
                i32.store8 offset=14
                local.get 2
                local.get 1
                i32.const 12
                i32.shr_u
                i32.const 224
                i32.or
                i32.store8 offset=12
                local.get 2
                local.get 1
                i32.const 6
                i32.shr_u
                i32.const 63
                i32.and
                i32.const 128
                i32.or
                i32.store8 offset=13
                i32.const 3
                local.set 1
                br 3 (;@2;)
              end
              local.get 2
              local.get 1
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=15
              local.get 2
              local.get 1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=14
              local.get 2
              local.get 1
              i32.const 12
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              local.get 2
              local.get 1
              i32.const 18
              i32.shr_u
              i32.const 7
              i32.and
              i32.const 240
              i32.or
              i32.store8 offset=12
              i32.const 4
              local.set 1
              br 2 (;@2;)
            end
            block ;; label = @4
              local.get 0
              i32.load offset=8
              local.tee 3
              local.get 0
              i32.load offset=4
              i32.ne
              br_if 0 (;@4;)
              local.get 0
              local.get 3
              call $alloc::raw_vec::RawVec<T,A>::reserve_for_push
              local.get 0
              i32.load offset=8
              local.set 3
            end
            local.get 0
            local.get 3
            i32.const 1
            i32.add
            i32.store offset=8
            local.get 0
            i32.load
            local.get 3
            i32.add
            local.get 1
            i32.store8
            br 2 (;@1;)
          end
          local.get 2
          local.get 1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=13
          local.get 2
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 192
          i32.or
          i32.store8 offset=12
          i32.const 2
          local.set 1
        end
        block ;; label = @2
          local.get 0
          i32.load offset=4
          local.get 0
          i32.load offset=8
          local.tee 3
          i32.sub
          local.get 1
          i32.ge_u
          br_if 0 (;@2;)
          local.get 0
          local.get 3
          local.get 1
          call $alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle
          local.get 0
          i32.load offset=8
          local.set 3
        end
        local.get 0
        i32.load
        local.get 3
        i32.add
        local.get 2
        i32.const 12
        i32.add
        local.get 1
        call $memcpy
        drop
        local.get 0
        local.get 3
        local.get 1
        i32.add
        i32.store offset=8
      end
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $<&mut W as core::fmt::Write>::write_fmt (;197;) (type 6) (param i32 i32) (result i32)
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
      i32.const 1051844
      local.get 1
      call $core::fmt::write
      local.set 0
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $<&mut W as core::fmt::Write>::write_str (;198;) (type 5) (param i32 i32 i32) (result i32)
      (local i32)
      block ;; label = @1
        local.get 0
        i32.load
        local.tee 0
        i32.load offset=4
        local.get 0
        i32.load offset=8
        local.tee 3
        i32.sub
        local.get 2
        i32.ge_u
        br_if 0 (;@1;)
        local.get 0
        local.get 3
        local.get 2
        call $alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle
        local.get 0
        i32.load offset=8
        local.set 3
      end
      local.get 0
      i32.load
      local.get 3
      i32.add
      local.get 1
      local.get 2
      call $memcpy
      drop
      local.get 0
      local.get 3
      local.get 2
      i32.add
      i32.store offset=8
      i32.const 0
    )
    (func $alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle (;199;) (type 4) (param i32 i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          local.get 1
          local.get 2
          i32.add
          local.tee 2
          local.get 1
          i32.lt_u
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=4
          local.tee 1
          i32.const 1
          i32.shl
          local.tee 4
          local.get 2
          local.get 4
          local.get 2
          i32.gt_u
          select
          local.tee 2
          i32.const 8
          local.get 2
          i32.const 8
          i32.gt_u
          select
          local.tee 2
          i32.const -1
          i32.xor
          i32.const 31
          i32.shr_u
          local.set 4
          block ;; label = @3
            block ;; label = @4
              local.get 1
              i32.eqz
              br_if 0 (;@4;)
              local.get 3
              local.get 1
              i32.store offset=28
              local.get 3
              i32.const 1
              i32.store offset=24
              local.get 3
              local.get 0
              i32.load
              i32.store offset=20
              br 1 (;@3;)
            end
            local.get 3
            i32.const 0
            i32.store offset=24
          end
          local.get 3
          i32.const 8
          i32.add
          local.get 4
          local.get 2
          local.get 3
          i32.const 20
          i32.add
          call $alloc::raw_vec::finish_grow
          local.get 3
          i32.load offset=12
          local.set 1
          block ;; label = @3
            local.get 3
            i32.load offset=8
            br_if 0 (;@3;)
            local.get 0
            local.get 2
            i32.store offset=4
            local.get 0
            local.get 1
            i32.store
            br 2 (;@1;)
          end
          local.get 1
          i32.const -2147483647
          i32.eq
          br_if 1 (;@1;)
          local.get 1
          i32.eqz
          br_if 0 (;@2;)
          local.get 1
          local.get 3
          i32.const 16
          i32.add
          i32.load
          call $alloc::alloc::handle_alloc_error
          unreachable
        end
        call $alloc::raw_vec::capacity_overflow
        unreachable
      end
      local.get 3
      i32.const 32
      i32.add
      global.set $__stack_pointer
    )
    (func $alloc::raw_vec::finish_grow (;200;) (type 12) (param i32 i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          local.get 1
          i32.eqz
          br_if 0 (;@2;)
          local.get 2
          i32.const -1
          i32.le_s
          br_if 1 (;@1;)
          block ;; label = @3
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
                    local.tee 1
                    br_if 0 (;@7;)
                    block ;; label = @8
                      local.get 2
                      br_if 0 (;@8;)
                      i32.const 1
                      local.set 1
                      br 4 (;@4;)
                    end
                    i32.const 0
                    i32.load8_u offset=1056309
                    drop
                    local.get 2
                    i32.const 1
                    call $__rust_alloc
                    local.set 1
                    br 2 (;@5;)
                  end
                  local.get 3
                  i32.load
                  local.get 1
                  i32.const 1
                  local.get 2
                  call $__rust_realloc
                  local.set 1
                  br 1 (;@5;)
                end
                block ;; label = @6
                  local.get 2
                  br_if 0 (;@6;)
                  i32.const 1
                  local.set 1
                  br 2 (;@4;)
                end
                i32.const 0
                i32.load8_u offset=1056309
                drop
                local.get 2
                i32.const 1
                call $__rust_alloc
                local.set 1
              end
              local.get 1
              i32.eqz
              br_if 1 (;@3;)
            end
            local.get 0
            local.get 1
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
          i32.const 1
          i32.store offset=4
          local.get 0
          i32.const 8
          i32.add
          local.get 2
          i32.store
          local.get 0
          i32.const 1
          i32.store
          return
        end
        local.get 0
        i32.const 0
        i32.store offset=4
        local.get 0
        i32.const 8
        i32.add
        local.get 2
        i32.store
        local.get 0
        i32.const 1
        i32.store
        return
      end
      local.get 0
      i32.const 0
      i32.store offset=4
      local.get 0
      i32.const 1
      i32.store
    )
    (func $alloc::alloc::handle_alloc_error (;201;) (type 2) (param i32 i32)
      local.get 0
      local.get 1
      call $alloc::alloc::handle_alloc_error::rt_error
      unreachable
    )
    (func $alloc::raw_vec::capacity_overflow (;202;) (type $.rodata)
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
      i32.const 1051932
      i32.store offset=8
      local.get 0
      i32.const 1051884
      i32.store offset=16
      local.get 0
      i32.const 8
      i32.add
      i32.const 1051940
      call $core::panicking::panic_fmt
      unreachable
    )
    (func $alloc::raw_vec::RawVec<T,A>::reserve_for_push (;203;) (type 2) (param i32 i32)
      (local i32 i32 i32)
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
          i32.const 8
          local.get 1
          i32.const 8
          i32.gt_u
          select
          local.tee 1
          i32.const -1
          i32.xor
          i32.const 31
          i32.shr_u
          local.set 4
          block ;; label = @3
            block ;; label = @4
              local.get 3
              i32.eqz
              br_if 0 (;@4;)
              local.get 2
              local.get 3
              i32.store offset=28
              local.get 2
              i32.const 1
              i32.store offset=24
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
          local.get 4
          local.get 1
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
    (func $alloc::alloc::handle_alloc_error::rt_error (;204;) (type 2) (param i32 i32)
      local.get 1
      local.get 0
      call $__rust_alloc_error_handler
      unreachable
    )
    (func $<<alloc::boxed::Box<dyn core::error::Error+core::marker::Send+core::marker::Sync> as core::convert::From<alloc::string::String>>::from::StringError as core::error::Error>::description (;205;) (type 2) (param i32 i32)
      local.get 0
      local.get 1
      i32.load offset=8
      i32.store offset=4
      local.get 0
      local.get 1
      i32.load
      i32.store
    )
    (func $<<alloc::boxed::Box<dyn core::error::Error+core::marker::Send+core::marker::Sync> as core::convert::From<alloc::string::String>>::from::StringError as core::fmt::Display>::fmt (;206;) (type 6) (param i32 i32) (result i32)
      local.get 0
      i32.load
      local.get 0
      i32.load offset=8
      local.get 1
      call $<str as core::fmt::Display>::fmt
    )
    (func $<<alloc::boxed::Box<dyn core::error::Error+core::marker::Send+core::marker::Sync> as core::convert::From<alloc::string::String>>::from::StringError as core::fmt::Debug>::fmt (;207;) (type 6) (param i32 i32) (result i32)
      local.get 0
      i32.load
      local.get 0
      i32.load offset=8
      local.get 1
      call $<str as core::fmt::Debug>::fmt
    )
    (func $<&str as alloc::ffi::c_str::CString::new::SpecNewImpl>::spec_new_impl (;208;) (type 4) (param i32 i32 i32)
      (local i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                local.get 2
                i32.const 1
                i32.add
                local.tee 4
                i32.eqz
                br_if 0 (;@5;)
                local.get 4
                i32.const -1
                i32.le_s
                br_if 1 (;@4;)
                i32.const 0
                i32.load8_u offset=1056309
                drop
                local.get 4
                i32.const 1
                call $__rust_alloc
                local.tee 5
                i32.eqz
                br_if 2 (;@3;)
                local.get 5
                local.get 1
                local.get 2
                call $memcpy
                local.set 6
                block ;; label = @6
                  local.get 2
                  i32.const 8
                  i32.lt_u
                  br_if 0 (;@6;)
                  local.get 3
                  i32.const 8
                  i32.add
                  i32.const 0
                  local.get 1
                  local.get 2
                  call $core::slice::memchr::memchr_aligned
                  local.get 3
                  i32.load offset=12
                  local.set 7
                  local.get 3
                  i32.load offset=8
                  local.set 8
                  br 5 (;@1;)
                end
                block ;; label = @6
                  local.get 2
                  br_if 0 (;@6;)
                  i32.const 0
                  local.set 7
                  i32.const 0
                  local.set 8
                  br 5 (;@1;)
                end
                block ;; label = @6
                  local.get 1
                  i32.load8_u
                  br_if 0 (;@6;)
                  i32.const 1
                  local.set 8
                  i32.const 0
                  local.set 7
                  br 5 (;@1;)
                end
                i32.const 1
                local.set 8
                local.get 2
                i32.const 1
                i32.eq
                br_if 3 (;@2;)
                block ;; label = @6
                  local.get 1
                  i32.load8_u offset=1
                  br_if 0 (;@6;)
                  i32.const 1
                  local.set 7
                  br 5 (;@1;)
                end
                i32.const 2
                local.set 7
                local.get 2
                i32.const 2
                i32.eq
                br_if 3 (;@2;)
                local.get 1
                i32.load8_u offset=2
                i32.eqz
                br_if 4 (;@1;)
                i32.const 3
                local.set 7
                local.get 2
                i32.const 3
                i32.eq
                br_if 3 (;@2;)
                local.get 1
                i32.load8_u offset=3
                i32.eqz
                br_if 4 (;@1;)
                i32.const 4
                local.set 7
                local.get 2
                i32.const 4
                i32.eq
                br_if 3 (;@2;)
                local.get 1
                i32.load8_u offset=4
                i32.eqz
                br_if 4 (;@1;)
                i32.const 5
                local.set 7
                local.get 2
                i32.const 5
                i32.eq
                br_if 3 (;@2;)
                local.get 1
                i32.load8_u offset=5
                i32.eqz
                br_if 4 (;@1;)
                local.get 2
                local.set 7
                i32.const 0
                local.set 8
                local.get 2
                i32.const 6
                i32.eq
                br_if 4 (;@1;)
                local.get 2
                i32.const 6
                local.get 1
                i32.load8_u offset=6
                local.tee 1
                select
                local.set 7
                local.get 1
                i32.eqz
                local.set 8
                br 4 (;@1;)
              end
              i32.const 1051956
              i32.const 43
              i32.const 1052032
              call $core::panicking::panic
              unreachable
            end
            call $alloc::raw_vec::capacity_overflow
            unreachable
          end
          i32.const 1
          local.get 4
          call $alloc::alloc::handle_alloc_error
          unreachable
        end
        local.get 2
        local.set 7
        i32.const 0
        local.set 8
      end
      block ;; label = @1
        block ;; label = @2
          local.get 8
          br_if 0 (;@2;)
          local.get 3
          local.get 2
          i32.store offset=28
          local.get 3
          local.get 4
          i32.store offset=24
          local.get 3
          local.get 6
          i32.store offset=20
          local.get 3
          local.get 3
          i32.const 20
          i32.add
          call $alloc::ffi::c_str::CString::_from_vec_unchecked
          local.get 0
          local.get 3
          i64.load
          i64.store offset=4 align=4
          i32.const 0
          local.set 5
          br 1 (;@1;)
        end
        local.get 0
        local.get 2
        i32.store offset=8
        local.get 0
        local.get 4
        i32.store offset=4
        local.get 0
        local.get 7
        i32.store offset=12
      end
      local.get 0
      local.get 5
      i32.store
      local.get 3
      i32.const 32
      i32.add
      global.set $__stack_pointer
    )
    (func $alloc::ffi::c_str::CString::_from_vec_unchecked (;209;) (type 2) (param i32 i32)
      (local i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                local.get 1
                i32.load offset=4
                local.tee 3
                local.get 1
                i32.load offset=8
                local.tee 4
                i32.ne
                br_if 0 (;@5;)
                local.get 4
                i32.const 1
                i32.add
                local.tee 3
                i32.eqz
                br_if 2 (;@3;)
                local.get 3
                i32.const -1
                i32.xor
                i32.const 31
                i32.shr_u
                local.set 5
                block ;; label = @6
                  block ;; label = @7
                    local.get 4
                    i32.eqz
                    br_if 0 (;@7;)
                    local.get 2
                    local.get 4
                    i32.store offset=28
                    local.get 2
                    i32.const 1
                    i32.store offset=24
                    local.get 2
                    local.get 1
                    i32.load
                    i32.store offset=20
                    br 1 (;@6;)
                  end
                  local.get 2
                  i32.const 0
                  i32.store offset=24
                end
                local.get 2
                i32.const 8
                i32.add
                local.get 5
                local.get 3
                local.get 2
                i32.const 20
                i32.add
                call $alloc::raw_vec::finish_grow
                local.get 2
                i32.load offset=12
                local.set 5
                local.get 2
                i32.load offset=8
                br_if 1 (;@4;)
                local.get 1
                local.get 3
                i32.store offset=4
                local.get 1
                local.get 5
                i32.store
              end
              local.get 4
              local.get 3
              i32.ne
              br_if 3 (;@1;)
              br 2 (;@2;)
            end
            local.get 5
            i32.const -2147483647
            i32.eq
            br_if 1 (;@2;)
            local.get 5
            i32.eqz
            br_if 0 (;@3;)
            local.get 5
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
        local.get 1
        local.get 4
        call $alloc::raw_vec::RawVec<T,A>::reserve_for_push
        local.get 1
        i32.load offset=4
        local.set 3
        local.get 1
        i32.load offset=8
        local.set 4
      end
      local.get 1
      local.get 4
      i32.const 1
      i32.add
      local.tee 5
      i32.store offset=8
      local.get 1
      i32.load
      local.tee 1
      local.get 4
      i32.add
      i32.const 0
      i32.store8
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 3
            local.get 5
            i32.gt_u
            br_if 0 (;@3;)
            local.get 1
            local.set 4
            br 1 (;@2;)
          end
          block ;; label = @3
            local.get 5
            br_if 0 (;@3;)
            i32.const 1
            local.set 4
            local.get 1
            local.get 3
            i32.const 1
            call $__rust_dealloc
            br 1 (;@2;)
          end
          local.get 1
          local.get 3
          i32.const 1
          local.get 5
          call $__rust_realloc
          local.tee 4
          i32.eqz
          br_if 1 (;@1;)
        end
        local.get 0
        local.get 5
        i32.store offset=4
        local.get 0
        local.get 4
        i32.store
        local.get 2
        i32.const 32
        i32.add
        global.set $__stack_pointer
        return
      end
      i32.const 1
      local.get 5
      call $alloc::alloc::handle_alloc_error
      unreachable
    )
    (func $alloc::fmt::format::format_inner (;210;) (type 2) (param i32 i32)
      (local i32 i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 32
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
                  i32.load offset=4
                  local.tee 3
                  i32.eqz
                  br_if 0 (;@6;)
                  local.get 1
                  i32.load
                  local.set 4
                  local.get 3
                  i32.const 3
                  i32.and
                  local.set 5
                  block ;; label = @7
                    block ;; label = @8
                      local.get 3
                      i32.const 4
                      i32.ge_u
                      br_if 0 (;@8;)
                      i32.const 0
                      local.set 3
                      i32.const 0
                      local.set 6
                      br 1 (;@7;)
                    end
                    local.get 4
                    i32.const 28
                    i32.add
                    local.set 7
                    local.get 3
                    i32.const -4
                    i32.and
                    local.set 8
                    i32.const 0
                    local.set 3
                    i32.const 0
                    local.set 6
                    loop ;; label = @8
                      local.get 7
                      i32.load
                      local.get 7
                      i32.const -8
                      i32.add
                      i32.load
                      local.get 7
                      i32.const -16
                      i32.add
                      i32.load
                      local.get 7
                      i32.const -24
                      i32.add
                      i32.load
                      local.get 3
                      i32.add
                      i32.add
                      i32.add
                      i32.add
                      local.set 3
                      local.get 7
                      i32.const 32
                      i32.add
                      local.set 7
                      local.get 8
                      local.get 6
                      i32.const 4
                      i32.add
                      local.tee 6
                      i32.ne
                      br_if 0 (;@8;)
                    end
                  end
                  block ;; label = @7
                    local.get 5
                    i32.eqz
                    br_if 0 (;@7;)
                    local.get 6
                    i32.const 3
                    i32.shl
                    local.get 4
                    i32.add
                    i32.const 4
                    i32.add
                    local.set 7
                    loop ;; label = @8
                      local.get 7
                      i32.load
                      local.get 3
                      i32.add
                      local.set 3
                      local.get 7
                      i32.const 8
                      i32.add
                      local.set 7
                      local.get 5
                      i32.const -1
                      i32.add
                      local.tee 5
                      br_if 0 (;@8;)
                    end
                  end
                  block ;; label = @7
                    local.get 1
                    i32.const 12
                    i32.add
                    i32.load
                    i32.eqz
                    br_if 0 (;@7;)
                    local.get 3
                    i32.const 0
                    i32.lt_s
                    br_if 1 (;@6;)
                    local.get 3
                    i32.const 16
                    i32.lt_u
                    local.get 4
                    i32.load offset=4
                    i32.eqz
                    i32.and
                    br_if 1 (;@6;)
                    local.get 3
                    i32.const 1
                    i32.shl
                    local.set 3
                  end
                  local.get 3
                  br_if 1 (;@5;)
                end
                i32.const 1
                local.set 7
                i32.const 0
                local.set 3
                br 1 (;@4;)
              end
              local.get 3
              i32.const -1
              i32.le_s
              br_if 1 (;@3;)
              i32.const 0
              i32.load8_u offset=1056309
              drop
              local.get 3
              i32.const 1
              call $__rust_alloc
              local.tee 7
              i32.eqz
              br_if 2 (;@2;)
            end
            local.get 2
            i32.const 0
            i32.store offset=20
            local.get 2
            local.get 3
            i32.store offset=16
            local.get 2
            local.get 7
            i32.store offset=12
            local.get 2
            local.get 2
            i32.const 12
            i32.add
            i32.store offset=24
            local.get 2
            i32.const 24
            i32.add
            i32.const 1051844
            local.get 1
            call $core::fmt::write
            i32.eqz
            br_if 2 (;@1;)
            i32.const 1052048
            i32.const 51
            local.get 2
            i32.const 31
            i32.add
            i32.const 1052100
            i32.const 1052140
            call $core::result::unwrap_failed
            unreachable
          end
          call $alloc::raw_vec::capacity_overflow
          unreachable
        end
        i32.const 1
        local.get 3
        call $alloc::alloc::handle_alloc_error
        unreachable
      end
      local.get 0
      local.get 2
      i64.load offset=12 align=4
      i64.store align=4
      local.get 0
      i32.const 8
      i32.add
      local.get 2
      i32.const 12
      i32.add
      i32.const 8
      i32.add
      i32.load
      i32.store
      local.get 2
      i32.const 32
      i32.add
      global.set $__stack_pointer
    )
    (func $alloc::sync::arcinner_layout_for_value_layout (;211;) (type 4) (param i32 i32 i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      block ;; label = @1
        local.get 1
        i32.const 7
        i32.add
        i32.const 0
        local.get 1
        i32.sub
        i32.and
        local.tee 4
        local.get 4
        i32.const -8
        i32.add
        i32.lt_u
        br_if 0 (;@1;)
        local.get 4
        local.get 2
        i32.add
        local.tee 2
        local.get 4
        i32.lt_u
        br_if 0 (;@1;)
        local.get 2
        i32.const -2147483648
        local.get 1
        i32.const 4
        local.get 1
        i32.const 4
        i32.gt_u
        select
        local.tee 1
        i32.sub
        i32.gt_u
        br_if 0 (;@1;)
        local.get 0
        local.get 1
        i32.store
        local.get 0
        local.get 1
        local.get 2
        i32.add
        i32.const -1
        i32.add
        i32.const 0
        local.get 1
        i32.sub
        i32.and
        i32.store offset=4
        local.get 3
        i32.const 16
        i32.add
        global.set $__stack_pointer
        return
      end
      i32.const 1052156
      i32.const 43
      local.get 3
      i32.const 15
      i32.add
      i32.const 1052200
      i32.const 1052244
      call $core::result::unwrap_failed
      unreachable
    )
    (func $<alloc::ffi::c_str::NulError as core::fmt::Debug>::fmt (;212;) (type 6) (param i32 i32) (result i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 2
      local.get 0
      i32.store offset=12
      local.get 1
      i32.const 1052260
      i32.const 8
      local.get 0
      i32.const 12
      i32.add
      i32.const 1052268
      local.get 2
      i32.const 12
      i32.add
      i32.const 1052284
      call $core::fmt::Formatter::debug_tuple_field2_finish
      local.set 0
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $core::ops::function::FnOnce::call_once (;213;) (type 6) (param i32 i32) (result i32)
      local.get 0
      i32.load
      drop
      loop (result i32) ;; label = @1
        br 0 (;@1;)
      end
    )
    (func $core::ptr::drop_in_place<&core::iter::adapters::copied::Copied<core::slice::iter::Iter<u8>>> (;214;) (type $.data) (param i32))
    (func $core::ptr::drop_in_place<core::fmt::Error> (;215;) (type $.data) (param i32))
    (func $core::panicking::panic_fmt (;216;) (type 2) (param i32 i32)
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
      i32.const 1052488
      i32.store offset=16
      local.get 2
      i32.const 1052300
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
    (func $core::slice::index::slice_start_index_len_fail (;217;) (type 4) (param i32 i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      local.get 0
      i32.store
      local.get 3
      local.get 1
      i32.store offset=4
      local.get 3
      i32.const 44
      i32.add
      i32.const 9
      i32.store
      local.get 3
      i32.const 9
      i32.store offset=36
      local.get 3
      local.get 3
      i32.const 4
      i32.add
      i32.store offset=40
      local.get 3
      local.get 3
      i32.store offset=32
      local.get 3
      i32.const 8
      i32.add
      i32.const 1053156
      i32.const 2
      local.get 3
      i32.const 32
      i32.add
      i32.const 2
      call $#func223<core::fmt::Arguments::new_v1>
      local.get 3
      i32.const 8
      i32.add
      local.get 2
      call $core::panicking::panic_fmt
      unreachable
    )
    (func $core::panicking::panic_bounds_check (;218;) (type 4) (param i32 i32 i32)
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
      i32.const 9
      i32.store
      local.get 3
      i32.const 9
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
      i32.const 1052556
      i32.const 2
      local.get 3
      i32.const 32
      i32.add
      i32.const 2
      call $#func223<core::fmt::Arguments::new_v1>
      local.get 3
      i32.const 8
      i32.add
      local.get 2
      call $core::panicking::panic_fmt
      unreachable
    )
    (func $core::slice::index::slice_end_index_len_fail (;219;) (type 4) (param i32 i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      local.get 0
      i32.store
      local.get 3
      local.get 1
      i32.store offset=4
      local.get 3
      i32.const 44
      i32.add
      i32.const 9
      i32.store
      local.get 3
      i32.const 9
      i32.store offset=36
      local.get 3
      local.get 3
      i32.const 4
      i32.add
      i32.store offset=40
      local.get 3
      local.get 3
      i32.store offset=32
      local.get 3
      i32.const 8
      i32.add
      i32.const 1053188
      i32.const 2
      local.get 3
      i32.const 32
      i32.add
      i32.const 2
      call $#func223<core::fmt::Arguments::new_v1>
      local.get 3
      i32.const 8
      i32.add
      local.get 2
      call $core::panicking::panic_fmt
      unreachable
    )
    (func $core::fmt::Formatter::pad (;220;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32)
      block ;; label = @1
        local.get 0
        i32.load
        local.tee 3
        local.get 0
        i32.load offset=8
        local.tee 4
        i32.or
        i32.eqz
        br_if 0 (;@1;)
        block ;; label = @2
          local.get 4
          i32.eqz
          br_if 0 (;@2;)
          local.get 1
          local.get 2
          i32.add
          local.set 5
          local.get 0
          i32.const 12
          i32.add
          i32.load
          i32.const 1
          i32.add
          local.set 6
          i32.const 0
          local.set 7
          local.get 1
          local.set 8
          block ;; label = @3
            loop ;; label = @4
              local.get 8
              local.set 4
              local.get 6
              i32.const -1
              i32.add
              local.tee 6
              i32.eqz
              br_if 1 (;@3;)
              local.get 4
              local.get 5
              i32.eq
              br_if 2 (;@2;)
              block ;; label = @5
                block ;; label = @6
                  local.get 4
                  i32.load8_s
                  local.tee 9
                  i32.const -1
                  i32.le_s
                  br_if 0 (;@6;)
                  local.get 4
                  i32.const 1
                  i32.add
                  local.set 8
                  local.get 9
                  i32.const 255
                  i32.and
                  local.set 9
                  br 1 (;@5;)
                end
                local.get 4
                i32.load8_u offset=1
                i32.const 63
                i32.and
                local.set 10
                local.get 9
                i32.const 31
                i32.and
                local.set 8
                block ;; label = @6
                  local.get 9
                  i32.const -33
                  i32.gt_u
                  br_if 0 (;@6;)
                  local.get 8
                  i32.const 6
                  i32.shl
                  local.get 10
                  i32.or
                  local.set 9
                  local.get 4
                  i32.const 2
                  i32.add
                  local.set 8
                  br 1 (;@5;)
                end
                local.get 10
                i32.const 6
                i32.shl
                local.get 4
                i32.load8_u offset=2
                i32.const 63
                i32.and
                i32.or
                local.set 10
                block ;; label = @6
                  local.get 9
                  i32.const -16
                  i32.ge_u
                  br_if 0 (;@6;)
                  local.get 10
                  local.get 8
                  i32.const 12
                  i32.shl
                  i32.or
                  local.set 9
                  local.get 4
                  i32.const 3
                  i32.add
                  local.set 8
                  br 1 (;@5;)
                end
                local.get 10
                i32.const 6
                i32.shl
                local.get 4
                i32.load8_u offset=3
                i32.const 63
                i32.and
                i32.or
                local.get 8
                i32.const 18
                i32.shl
                i32.const 1835008
                i32.and
                i32.or
                local.tee 9
                i32.const 1114112
                i32.eq
                br_if 3 (;@2;)
                local.get 4
                i32.const 4
                i32.add
                local.set 8
              end
              local.get 7
              local.get 4
              i32.sub
              local.get 8
              i32.add
              local.set 7
              local.get 9
              i32.const 1114112
              i32.ne
              br_if 0 (;@4;)
              br 2 (;@2;)
            end
          end
          local.get 4
          local.get 5
          i32.eq
          br_if 0 (;@2;)
          block ;; label = @3
            local.get 4
            i32.load8_s
            local.tee 8
            i32.const -1
            i32.gt_s
            br_if 0 (;@3;)
            local.get 8
            i32.const -32
            i32.lt_u
            br_if 0 (;@3;)
            local.get 8
            i32.const -16
            i32.lt_u
            br_if 0 (;@3;)
            local.get 4
            i32.load8_u offset=2
            i32.const 63
            i32.and
            i32.const 6
            i32.shl
            local.get 4
            i32.load8_u offset=1
            i32.const 63
            i32.and
            i32.const 12
            i32.shl
            i32.or
            local.get 4
            i32.load8_u offset=3
            i32.const 63
            i32.and
            i32.or
            local.get 8
            i32.const 255
            i32.and
            i32.const 18
            i32.shl
            i32.const 1835008
            i32.and
            i32.or
            i32.const 1114112
            i32.eq
            br_if 1 (;@2;)
          end
          block ;; label = @3
            block ;; label = @4
              local.get 7
              i32.eqz
              br_if 0 (;@4;)
              block ;; label = @5
                local.get 7
                local.get 2
                i32.lt_u
                br_if 0 (;@5;)
                i32.const 0
                local.set 4
                local.get 7
                local.get 2
                i32.eq
                br_if 1 (;@4;)
                br 2 (;@3;)
              end
              i32.const 0
              local.set 4
              local.get 1
              local.get 7
              i32.add
              i32.load8_s
              i32.const -64
              i32.lt_s
              br_if 1 (;@3;)
            end
            local.get 1
            local.set 4
          end
          local.get 7
          local.get 2
          local.get 4
          select
          local.set 2
          local.get 4
          local.get 1
          local.get 4
          select
          local.set 1
        end
        block ;; label = @2
          local.get 3
          br_if 0 (;@2;)
          local.get 0
          i32.load offset=20
          local.get 1
          local.get 2
          local.get 0
          i32.const 24
          i32.add
          i32.load
          i32.load offset=12
          call_indirect (type 5)
          return
        end
        local.get 0
        i32.load offset=4
        local.set 5
        block ;; label = @2
          block ;; label = @3
            local.get 2
            i32.const 16
            i32.lt_u
            br_if 0 (;@3;)
            local.get 1
            local.get 2
            call $core::str::count::do_count_chars
            local.set 4
            br 1 (;@2;)
          end
          block ;; label = @3
            local.get 2
            br_if 0 (;@3;)
            i32.const 0
            local.set 4
            br 1 (;@2;)
          end
          local.get 2
          i32.const 3
          i32.and
          local.set 6
          block ;; label = @3
            block ;; label = @4
              local.get 2
              i32.const 4
              i32.ge_u
              br_if 0 (;@4;)
              i32.const 0
              local.set 4
              i32.const 0
              local.set 9
              br 1 (;@3;)
            end
            local.get 2
            i32.const -4
            i32.and
            local.set 7
            i32.const 0
            local.set 4
            i32.const 0
            local.set 9
            loop ;; label = @4
              local.get 4
              local.get 1
              local.get 9
              i32.add
              local.tee 8
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.get 8
              i32.const 1
              i32.add
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.get 8
              i32.const 2
              i32.add
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.get 8
              i32.const 3
              i32.add
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.set 4
              local.get 7
              local.get 9
              i32.const 4
              i32.add
              local.tee 9
              i32.ne
              br_if 0 (;@4;)
            end
          end
          local.get 6
          i32.eqz
          br_if 0 (;@2;)
          local.get 1
          local.get 9
          i32.add
          local.set 8
          loop ;; label = @3
            local.get 4
            local.get 8
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.set 4
            local.get 8
            i32.const 1
            i32.add
            local.set 8
            local.get 6
            i32.const -1
            i32.add
            local.tee 6
            br_if 0 (;@3;)
          end
        end
        block ;; label = @2
          block ;; label = @3
            local.get 5
            local.get 4
            i32.le_u
            br_if 0 (;@3;)
            local.get 5
            local.get 4
            i32.sub
            local.set 7
            i32.const 0
            local.set 4
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  local.get 0
                  i32.load8_u offset=32
                  br_table 2 (;@4;) 0 (;@6;) 1 (;@5;) 2 (;@4;) 2 (;@4;)
                end
                local.get 7
                local.set 4
                i32.const 0
                local.set 7
                br 1 (;@4;)
              end
              local.get 7
              i32.const 1
              i32.shr_u
              local.set 4
              local.get 7
              i32.const 1
              i32.add
              i32.const 1
              i32.shr_u
              local.set 7
            end
            local.get 4
            i32.const 1
            i32.add
            local.set 4
            local.get 0
            i32.const 24
            i32.add
            i32.load
            local.set 8
            local.get 0
            i32.load offset=16
            local.set 6
            local.get 0
            i32.load offset=20
            local.set 9
            loop ;; label = @4
              local.get 4
              i32.const -1
              i32.add
              local.tee 4
              i32.eqz
              br_if 2 (;@2;)
              local.get 9
              local.get 6
              local.get 8
              i32.load offset=16
              call_indirect (type 6)
              i32.eqz
              br_if 0 (;@4;)
            end
            i32.const 1
            return
          end
          local.get 0
          i32.load offset=20
          local.get 1
          local.get 2
          local.get 0
          i32.const 24
          i32.add
          i32.load
          i32.load offset=12
          call_indirect (type 5)
          return
        end
        i32.const 1
        local.set 4
        block ;; label = @2
          local.get 9
          local.get 1
          local.get 2
          local.get 8
          i32.load offset=12
          call_indirect (type 5)
          br_if 0 (;@2;)
          i32.const 0
          local.set 4
          block ;; label = @3
            loop ;; label = @4
              block ;; label = @5
                local.get 7
                local.get 4
                i32.ne
                br_if 0 (;@5;)
                local.get 7
                local.set 4
                br 2 (;@3;)
              end
              local.get 4
              i32.const 1
              i32.add
              local.set 4
              local.get 9
              local.get 6
              local.get 8
              i32.load offset=16
              call_indirect (type 6)
              i32.eqz
              br_if 0 (;@4;)
            end
            local.get 4
            i32.const -1
            i32.add
            local.set 4
          end
          local.get 4
          local.get 7
          i32.lt_u
          local.set 4
        end
        local.get 4
        return
      end
      local.get 0
      i32.load offset=20
      local.get 1
      local.get 2
      local.get 0
      i32.const 24
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type 5)
    )
    (func $core::panicking::panic (;221;) (type 4) (param i32 i32 i32)
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
      i32.const 1052300
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
      call $core::panicking::panic_fmt
      unreachable
    )
    (func $core::fmt::num::imp::<impl core::fmt::Display for u32>::fmt (;222;) (type 6) (param i32 i32) (result i32)
      local.get 0
      i64.load32_u
      i32.const 1
      local.get 1
      call $core::fmt::num::imp::fmt_u64
    )
    (func $#func223<core::fmt::Arguments::new_v1> (@name "core::fmt::Arguments::new_v1") (;223;) (type 14) (param i32 i32 i32 i32 i32)
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
      i32.const 1052356
      i32.store offset=8
      local.get 5
      i32.const 1052300
      i32.store offset=16
      local.get 5
      i32.const 8
      i32.add
      i32.const 1053044
      call $core::panicking::panic_fmt
      unreachable
    )
    (func $core::fmt::num::<impl core::fmt::Debug for u32>::fmt (;224;) (type 6) (param i32 i32) (result i32)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 128
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                local.get 1
                i32.load offset=28
                local.tee 3
                i32.const 16
                i32.and
                br_if 0 (;@5;)
                local.get 3
                i32.const 32
                i32.and
                br_if 1 (;@4;)
                local.get 0
                i64.load32_u
                i32.const 1
                local.get 1
                call $core::fmt::num::imp::fmt_u64
                local.set 0
                br 2 (;@3;)
              end
              local.get 0
              i32.load
              local.set 0
              i32.const 0
              local.set 3
              loop ;; label = @5
                local.get 2
                local.get 3
                i32.add
                i32.const 127
                i32.add
                i32.const 48
                i32.const 87
                local.get 0
                i32.const 15
                i32.and
                local.tee 4
                i32.const 10
                i32.lt_u
                select
                local.get 4
                i32.add
                i32.store8
                local.get 3
                i32.const -1
                i32.add
                local.set 3
                local.get 0
                i32.const 16
                i32.lt_u
                local.set 4
                local.get 0
                i32.const 4
                i32.shr_u
                local.set 0
                local.get 4
                i32.eqz
                br_if 0 (;@5;)
              end
              local.get 3
              i32.const 128
              i32.add
              local.tee 0
              i32.const 128
              i32.gt_u
              br_if 2 (;@2;)
              local.get 1
              i32.const 1
              i32.const 1052774
              i32.const 2
              local.get 2
              local.get 3
              i32.add
              i32.const 128
              i32.add
              i32.const 0
              local.get 3
              i32.sub
              call $core::fmt::Formatter::pad_integral
              local.set 0
              br 1 (;@3;)
            end
            local.get 0
            i32.load
            local.set 0
            i32.const 0
            local.set 3
            loop ;; label = @4
              local.get 2
              local.get 3
              i32.add
              i32.const 127
              i32.add
              i32.const 48
              i32.const 55
              local.get 0
              i32.const 15
              i32.and
              local.tee 4
              i32.const 10
              i32.lt_u
              select
              local.get 4
              i32.add
              i32.store8
              local.get 3
              i32.const -1
              i32.add
              local.set 3
              local.get 0
              i32.const 16
              i32.lt_u
              local.set 4
              local.get 0
              i32.const 4
              i32.shr_u
              local.set 0
              local.get 4
              i32.eqz
              br_if 0 (;@4;)
            end
            local.get 3
            i32.const 128
            i32.add
            local.tee 0
            i32.const 128
            i32.gt_u
            br_if 2 (;@1;)
            local.get 1
            i32.const 1
            i32.const 1052774
            i32.const 2
            local.get 2
            local.get 3
            i32.add
            i32.const 128
            i32.add
            i32.const 0
            local.get 3
            i32.sub
            call $core::fmt::Formatter::pad_integral
            local.set 0
          end
          local.get 2
          i32.const 128
          i32.add
          global.set $__stack_pointer
          local.get 0
          return
        end
        local.get 0
        i32.const 128
        i32.const 1052804
        call $core::slice::index::slice_start_index_len_fail
        unreachable
      end
      local.get 0
      i32.const 128
      i32.const 1052804
      call $core::slice::index::slice_start_index_len_fail
      unreachable
    )
    (func $core::fmt::write (;225;) (type 5) (param i32 i32 i32) (result i32)
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
                  call_indirect (type 5)
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
                call_indirect (type 6)
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
                call_indirect (type 5)
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
                  i32.const 92
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
                  i32.const 92
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
              call_indirect (type 6)
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
            call_indirect (type 5)
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
    (func $core::fmt::Formatter::pad_integral (;226;) (type 17) (param i32 i32 i32 i32 i32 i32) (result i32)
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
          call_indirect (type 5)
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
          call_indirect (type 5)
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
              call_indirect (type 6)
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
          call_indirect (type 5)
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
            call_indirect (type 6)
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
        call_indirect (type 5)
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
          call_indirect (type 6)
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
    (func $<core::ops::range::Range<Idx> as core::fmt::Debug>::fmt (;227;) (type 6) (param i32 i32) (result i32)
      (local i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      i32.const 1
      local.set 3
      block ;; label = @1
        local.get 0
        local.get 1
        call $core::fmt::num::<impl core::fmt::Debug for u32>::fmt
        br_if 0 (;@1;)
        local.get 2
        i32.const 20
        i32.add
        i64.const 0
        i64.store align=4
        i32.const 1
        local.set 3
        local.get 2
        i32.const 1
        i32.store offset=12
        local.get 2
        i32.const 1052396
        i32.store offset=8
        local.get 2
        i32.const 1052300
        i32.store offset=16
        local.get 1
        i32.load offset=20
        local.get 1
        i32.const 24
        i32.add
        i32.load
        local.get 2
        i32.const 8
        i32.add
        call $core::fmt::write
        br_if 0 (;@1;)
        local.get 0
        i32.const 4
        i32.add
        local.get 1
        call $core::fmt::num::<impl core::fmt::Debug for u32>::fmt
        local.set 3
      end
      local.get 2
      i32.const 32
      i32.add
      global.set $__stack_pointer
      local.get 3
    )
    (func $<T as core::any::Any>::type_id (;228;) (type 2) (param i32 i32)
      local.get 0
      i64.const -3751304911407043677
      i64.store offset=8
      local.get 0
      i64.const 118126004786499436
      i64.store
    )
    (func $core::fmt::builders::DebugInner::entry (;229;) (type 4) (param i32 i32 i32)
      (local i32 i32 i32 i32 i64)
      global.get $__stack_pointer
      i32.const 64
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      i32.const 1
      local.set 4
      block ;; label = @1
        local.get 0
        i32.load8_u offset=4
        br_if 0 (;@1;)
        local.get 0
        i32.load8_u offset=5
        local.set 4
        block ;; label = @2
          block ;; label = @3
            local.get 0
            i32.load
            local.tee 5
            i32.load offset=28
            local.tee 6
            i32.const 4
            i32.and
            br_if 0 (;@3;)
            local.get 4
            i32.const 255
            i32.and
            i32.eqz
            br_if 1 (;@2;)
            i32.const 1
            local.set 4
            local.get 5
            i32.load offset=20
            i32.const 1052764
            i32.const 2
            local.get 5
            i32.const 24
            i32.add
            i32.load
            i32.load offset=12
            call_indirect (type 5)
            i32.eqz
            br_if 1 (;@2;)
            br 2 (;@1;)
          end
          block ;; label = @3
            local.get 4
            i32.const 255
            i32.and
            br_if 0 (;@3;)
            i32.const 1
            local.set 4
            local.get 5
            i32.load offset=20
            i32.const 1052772
            i32.const 1
            local.get 5
            i32.const 24
            i32.add
            i32.load
            i32.load offset=12
            call_indirect (type 5)
            br_if 2 (;@1;)
            local.get 5
            i32.load offset=28
            local.set 6
          end
          i32.const 1
          local.set 4
          local.get 3
          i32.const 1
          i32.store8 offset=27
          local.get 3
          i32.const 52
          i32.add
          i32.const 1052736
          i32.store
          local.get 3
          local.get 5
          i64.load offset=20 align=4
          i64.store offset=12 align=4
          local.get 3
          local.get 3
          i32.const 27
          i32.add
          i32.store offset=20
          local.get 3
          local.get 5
          i64.load offset=8 align=4
          i64.store offset=36 align=4
          local.get 5
          i64.load align=4
          local.set 7
          local.get 3
          local.get 6
          i32.store offset=56
          local.get 3
          local.get 5
          i32.load offset=16
          i32.store offset=44
          local.get 3
          local.get 5
          i32.load8_u offset=32
          i32.store8 offset=60
          local.get 3
          local.get 7
          i64.store offset=28 align=4
          local.get 3
          local.get 3
          i32.const 12
          i32.add
          i32.store offset=48
          local.get 1
          local.get 3
          i32.const 28
          i32.add
          local.get 2
          i32.load offset=12
          call_indirect (type 6)
          br_if 1 (;@1;)
          local.get 3
          i32.load offset=48
          i32.const 1052766
          i32.const 2
          local.get 3
          i32.load offset=52
          i32.load offset=12
          call_indirect (type 5)
          local.set 4
          br 1 (;@1;)
        end
        local.get 1
        local.get 5
        local.get 2
        i32.load offset=12
        call_indirect (type 6)
        local.set 4
      end
      local.get 0
      i32.const 1
      i32.store8 offset=5
      local.get 0
      local.get 4
      i32.store8 offset=4
      local.get 3
      i32.const 64
      i32.add
      global.set $__stack_pointer
    )
    (func $core::slice::index::slice_index_order_fail (;230;) (type 4) (param i32 i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      local.get 0
      i32.store
      local.get 3
      local.get 1
      i32.store offset=4
      local.get 3
      i32.const 44
      i32.add
      i32.const 9
      i32.store
      local.get 3
      i32.const 9
      i32.store offset=36
      local.get 3
      local.get 3
      i32.const 4
      i32.add
      i32.store offset=40
      local.get 3
      local.get 3
      i32.store offset=32
      local.get 3
      i32.const 8
      i32.add
      i32.const 1053240
      i32.const 2
      local.get 3
      i32.const 32
      i32.add
      i32.const 2
      call $#func223<core::fmt::Arguments::new_v1>
      local.get 3
      i32.const 8
      i32.add
      local.get 2
      call $core::panicking::panic_fmt
      unreachable
    )
    (func $<core::cell::BorrowMutError as core::fmt::Debug>::fmt (;231;) (type 6) (param i32 i32) (result i32)
      local.get 1
      i32.load offset=20
      i32.const 1052404
      i32.const 14
      local.get 1
      i32.const 24
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type 5)
    )
    (func $core::char::methods::<impl char>::escape_debug_ext (;232;) (type 4) (param i32 i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 3
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
                          local.get 1
                          br_table 5 (;@5;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 1 (;@9;) 3 (;@7;) 8 (;@2;) 8 (;@2;) 2 (;@8;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 6 (;@4;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 7 (;@3;) 0 (;@10;)
                        end
                        local.get 1
                        i32.const 92
                        i32.eq
                        br_if 3 (;@6;)
                        br 7 (;@2;)
                      end
                      local.get 0
                      i32.const 512
                      i32.store16 offset=10
                      local.get 0
                      i64.const 0
                      i64.store offset=2 align=2
                      local.get 0
                      i32.const 29788
                      i32.store16
                      br 7 (;@1;)
                    end
                    local.get 0
                    i32.const 512
                    i32.store16 offset=10
                    local.get 0
                    i64.const 0
                    i64.store offset=2 align=2
                    local.get 0
                    i32.const 29276
                    i32.store16
                    br 6 (;@1;)
                  end
                  local.get 0
                  i32.const 512
                  i32.store16 offset=10
                  local.get 0
                  i64.const 0
                  i64.store offset=2 align=2
                  local.get 0
                  i32.const 28252
                  i32.store16
                  br 5 (;@1;)
                end
                local.get 0
                i32.const 512
                i32.store16 offset=10
                local.get 0
                i64.const 0
                i64.store offset=2 align=2
                local.get 0
                i32.const 23644
                i32.store16
                br 4 (;@1;)
              end
              local.get 0
              i32.const 512
              i32.store16 offset=10
              local.get 0
              i64.const 0
              i64.store offset=2 align=2
              local.get 0
              i32.const 12380
              i32.store16
              br 3 (;@1;)
            end
            local.get 2
            i32.const 65536
            i32.and
            i32.eqz
            br_if 1 (;@2;)
            local.get 0
            i32.const 512
            i32.store16 offset=10
            local.get 0
            i64.const 0
            i64.store offset=2 align=2
            local.get 0
            i32.const 8796
            i32.store16
            br 2 (;@1;)
          end
          local.get 2
          i32.const 256
          i32.and
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          i32.const 512
          i32.store16 offset=10
          local.get 0
          i64.const 0
          i64.store offset=2 align=2
          local.get 0
          i32.const 10076
          i32.store16
          br 1 (;@1;)
        end
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  local.get 2
                  i32.const 1
                  i32.and
                  i32.eqz
                  br_if 0 (;@6;)
                  local.get 1
                  call $core::unicode::unicode_data::grapheme_extend::lookup
                  br_if 1 (;@5;)
                end
                local.get 1
                call $core::unicode::printable::is_printable
                i32.eqz
                br_if 1 (;@4;)
                local.get 0
                local.get 1
                i32.store offset=4
                local.get 0
                i32.const 128
                i32.store8
                br 4 (;@1;)
              end
              local.get 3
              i32.const 6
              i32.add
              i32.const 2
              i32.add
              i32.const 0
              i32.store8
              local.get 3
              i32.const 0
              i32.store16 offset=6
              local.get 3
              i32.const 125
              i32.store8 offset=15
              local.get 3
              local.get 1
              i32.const 15
              i32.and
              i32.const 1055332
              i32.add
              i32.load8_u
              i32.store8 offset=14
              local.get 3
              local.get 1
              i32.const 4
              i32.shr_u
              i32.const 15
              i32.and
              i32.const 1055332
              i32.add
              i32.load8_u
              i32.store8 offset=13
              local.get 3
              local.get 1
              i32.const 8
              i32.shr_u
              i32.const 15
              i32.and
              i32.const 1055332
              i32.add
              i32.load8_u
              i32.store8 offset=12
              local.get 3
              local.get 1
              i32.const 12
              i32.shr_u
              i32.const 15
              i32.and
              i32.const 1055332
              i32.add
              i32.load8_u
              i32.store8 offset=11
              local.get 3
              local.get 1
              i32.const 16
              i32.shr_u
              i32.const 15
              i32.and
              i32.const 1055332
              i32.add
              i32.load8_u
              i32.store8 offset=10
              local.get 3
              local.get 1
              i32.const 20
              i32.shr_u
              i32.const 15
              i32.and
              i32.const 1055332
              i32.add
              i32.load8_u
              i32.store8 offset=9
              local.get 1
              i32.const 1
              i32.or
              i32.clz
              i32.const 2
              i32.shr_u
              i32.const -2
              i32.add
              local.tee 1
              i32.const 11
              i32.ge_u
              br_if 1 (;@3;)
              local.get 3
              i32.const 6
              i32.add
              local.get 1
              i32.add
              local.tee 2
              i32.const 0
              i32.load16_u offset=1055392 align=1
              i32.store16 align=1
              local.get 2
              i32.const 2
              i32.add
              i32.const 0
              i32.load8_u offset=1055394
              i32.store8
              local.get 0
              local.get 3
              i64.load offset=6 align=2
              i64.store align=1
              local.get 0
              i32.const 8
              i32.add
              local.get 3
              i32.const 6
              i32.add
              i32.const 8
              i32.add
              i32.load16_u
              i32.store16 align=1
              local.get 0
              i32.const 10
              i32.store8 offset=11
              local.get 0
              local.get 1
              i32.store8 offset=10
              br 3 (;@1;)
            end
            local.get 3
            i32.const 6
            i32.add
            i32.const 2
            i32.add
            i32.const 0
            i32.store8
            local.get 3
            i32.const 0
            i32.store16 offset=6
            local.get 3
            i32.const 125
            i32.store8 offset=15
            local.get 3
            local.get 1
            i32.const 15
            i32.and
            i32.const 1055332
            i32.add
            i32.load8_u
            i32.store8 offset=14
            local.get 3
            local.get 1
            i32.const 4
            i32.shr_u
            i32.const 15
            i32.and
            i32.const 1055332
            i32.add
            i32.load8_u
            i32.store8 offset=13
            local.get 3
            local.get 1
            i32.const 8
            i32.shr_u
            i32.const 15
            i32.and
            i32.const 1055332
            i32.add
            i32.load8_u
            i32.store8 offset=12
            local.get 3
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 15
            i32.and
            i32.const 1055332
            i32.add
            i32.load8_u
            i32.store8 offset=11
            local.get 3
            local.get 1
            i32.const 16
            i32.shr_u
            i32.const 15
            i32.and
            i32.const 1055332
            i32.add
            i32.load8_u
            i32.store8 offset=10
            local.get 3
            local.get 1
            i32.const 20
            i32.shr_u
            i32.const 15
            i32.and
            i32.const 1055332
            i32.add
            i32.load8_u
            i32.store8 offset=9
            local.get 1
            i32.const 1
            i32.or
            i32.clz
            i32.const 2
            i32.shr_u
            i32.const -2
            i32.add
            local.tee 1
            i32.const 11
            i32.ge_u
            br_if 1 (;@2;)
            local.get 3
            i32.const 6
            i32.add
            local.get 1
            i32.add
            local.tee 2
            i32.const 0
            i32.load16_u offset=1055392 align=1
            i32.store16 align=1
            local.get 2
            i32.const 2
            i32.add
            i32.const 0
            i32.load8_u offset=1055394
            i32.store8
            local.get 0
            local.get 3
            i64.load offset=6 align=2
            i64.store align=1
            local.get 0
            i32.const 8
            i32.add
            local.get 3
            i32.const 6
            i32.add
            i32.const 8
            i32.add
            i32.load16_u
            i32.store16 align=1
            local.get 0
            i32.const 10
            i32.store8 offset=11
            local.get 0
            local.get 1
            i32.store8 offset=10
            br 2 (;@1;)
          end
          local.get 1
          i32.const 10
          i32.const 1055376
          call $core::slice::index::slice_start_index_len_fail
          unreachable
        end
        local.get 1
        i32.const 10
        i32.const 1055376
        call $core::slice::index::slice_start_index_len_fail
        unreachable
      end
      local.get 3
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $core::unicode::unicode_data::grapheme_extend::lookup (;233;) (type 3) (param i32) (result i32)
      (local i32 i32 i32 i32 i32)
      local.get 0
      i32.const 11
      i32.shl
      local.set 1
      i32.const 0
      local.set 2
      i32.const 33
      local.set 3
      i32.const 33
      local.set 4
      block ;; label = @1
        block ;; label = @2
          loop ;; label = @3
            block ;; label = @4
              block ;; label = @5
                i32.const -1
                local.get 3
                i32.const 1
                i32.shr_u
                local.get 2
                i32.add
                local.tee 5
                i32.const 2
                i32.shl
                i32.const 1055420
                i32.add
                i32.load
                i32.const 11
                i32.shl
                local.tee 3
                local.get 1
                i32.ne
                local.get 3
                local.get 1
                i32.lt_u
                select
                local.tee 3
                i32.const 1
                i32.ne
                br_if 0 (;@5;)
                local.get 5
                local.set 4
                br 1 (;@4;)
              end
              local.get 3
              i32.const 255
              i32.and
              i32.const 255
              i32.ne
              br_if 2 (;@2;)
              local.get 5
              i32.const 1
              i32.add
              local.set 2
            end
            local.get 4
            local.get 2
            i32.sub
            local.set 3
            local.get 4
            local.get 2
            i32.gt_u
            br_if 0 (;@3;)
            br 2 (;@1;)
          end
        end
        local.get 5
        i32.const 1
        i32.add
        local.set 2
      end
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 2
              i32.const 32
              i32.gt_u
              br_if 0 (;@4;)
              local.get 2
              i32.const 2
              i32.shl
              local.tee 1
              i32.const 1055420
              i32.add
              i32.load
              i32.const 21
              i32.shr_u
              local.set 4
              local.get 2
              i32.const 32
              i32.ne
              br_if 1 (;@3;)
              i32.const 31
              local.set 2
              i32.const 727
              local.set 5
              br 2 (;@2;)
            end
            local.get 2
            i32.const 33
            i32.const 1055300
            call $core::panicking::panic_bounds_check
            unreachable
          end
          local.get 1
          i32.const 1055424
          i32.add
          i32.load
          i32.const 21
          i32.shr_u
          local.set 5
          block ;; label = @3
            local.get 2
            br_if 0 (;@3;)
            i32.const 0
            local.set 2
            br 2 (;@1;)
          end
          local.get 2
          i32.const -1
          i32.add
          local.set 2
        end
        local.get 2
        i32.const 2
        i32.shl
        i32.const 1055420
        i32.add
        i32.load
        i32.const 2097151
        i32.and
        local.set 2
      end
      block ;; label = @1
        block ;; label = @2
          local.get 5
          local.get 4
          i32.const -1
          i32.xor
          i32.add
          i32.eqz
          br_if 0 (;@2;)
          local.get 0
          local.get 2
          i32.sub
          local.set 3
          local.get 4
          i32.const 727
          local.get 4
          i32.const 727
          i32.gt_u
          select
          local.set 1
          local.get 5
          i32.const -1
          i32.add
          local.set 5
          i32.const 0
          local.set 2
          loop ;; label = @3
            local.get 1
            local.get 4
            i32.eq
            br_if 2 (;@1;)
            local.get 2
            local.get 4
            i32.const 1055552
            i32.add
            i32.load8_u
            i32.add
            local.tee 2
            local.get 3
            i32.gt_u
            br_if 1 (;@2;)
            local.get 5
            local.get 4
            i32.const 1
            i32.add
            local.tee 4
            i32.ne
            br_if 0 (;@3;)
          end
          local.get 5
          local.set 4
        end
        local.get 4
        i32.const 1
        i32.and
        return
      end
      local.get 1
      i32.const 727
      i32.const 1055316
      call $core::panicking::panic_bounds_check
      unreachable
    )
    (func $core::unicode::printable::is_printable (;234;) (type 3) (param i32) (result i32)
      (local i32)
      block ;; label = @1
        local.get 0
        i32.const 32
        i32.ge_u
        br_if 0 (;@1;)
        i32.const 0
        return
      end
      i32.const 1
      local.set 1
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.const 127
          i32.lt_u
          br_if 0 (;@2;)
          local.get 0
          i32.const 65536
          i32.lt_u
          br_if 1 (;@1;)
          block ;; label = @3
            block ;; label = @4
              local.get 0
              i32.const 131072
              i32.lt_u
              br_if 0 (;@4;)
              block ;; label = @5
                local.get 0
                i32.const -205744
                i32.add
                i32.const 712016
                i32.ge_u
                br_if 0 (;@5;)
                i32.const 0
                return
              end
              block ;; label = @5
                local.get 0
                i32.const -201547
                i32.add
                i32.const 5
                i32.ge_u
                br_if 0 (;@5;)
                i32.const 0
                return
              end
              block ;; label = @5
                local.get 0
                i32.const -195102
                i32.add
                i32.const 1506
                i32.ge_u
                br_if 0 (;@5;)
                i32.const 0
                return
              end
              block ;; label = @5
                local.get 0
                i32.const -191457
                i32.add
                i32.const 3103
                i32.ge_u
                br_if 0 (;@5;)
                i32.const 0
                return
              end
              block ;; label = @5
                local.get 0
                i32.const -183970
                i32.add
                i32.const 14
                i32.ge_u
                br_if 0 (;@5;)
                i32.const 0
                return
              end
              block ;; label = @5
                local.get 0
                i32.const -2
                i32.and
                i32.const 178206
                i32.ne
                br_if 0 (;@5;)
                i32.const 0
                return
              end
              local.get 0
              i32.const -32
              i32.and
              i32.const 173792
              i32.ne
              br_if 1 (;@3;)
              i32.const 0
              return
            end
            local.get 0
            i32.const 1053856
            i32.const 44
            i32.const 1053944
            i32.const 196
            i32.const 1054140
            i32.const 450
            call $core::unicode::printable::check
            return
          end
          i32.const 0
          local.set 1
          local.get 0
          i32.const -177978
          i32.add
          i32.const 6
          i32.lt_u
          br_if 0 (;@2;)
          local.get 0
          i32.const -1114112
          i32.add
          i32.const -196112
          i32.lt_u
          local.set 1
        end
        local.get 1
        return
      end
      local.get 0
      i32.const 1054590
      i32.const 40
      i32.const 1054670
      i32.const 287
      i32.const 1054957
      i32.const 303
      call $core::unicode::printable::check
    )
    (func $<core::ffi::c_str::CStr as core::fmt::Debug>::fmt (;235;) (type 5) (param i32 i32 i32) (result i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 64
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      i32.const 24
      i32.add
      i64.const 1
      i64.store align=4
      local.get 3
      i32.const 2
      i32.store offset=16
      local.get 3
      i32.const 1052420
      i32.store offset=12
      local.get 3
      i32.const 93
      i32.store offset=40
      local.get 3
      i32.const 128
      i32.store8 offset=58
      local.get 3
      i32.const 128
      i32.store8 offset=52
      local.get 3
      local.get 0
      i32.store offset=44
      local.get 3
      local.get 1
      local.get 0
      i32.add
      i32.const -1
      i32.add
      i32.store offset=48
      local.get 2
      i32.const 24
      i32.add
      i32.load
      local.set 0
      local.get 3
      local.get 3
      i32.const 36
      i32.add
      i32.store offset=20
      local.get 3
      local.get 3
      i32.const 44
      i32.add
      i32.store offset=36
      local.get 2
      i32.load offset=20
      local.get 0
      local.get 3
      i32.const 12
      i32.add
      call $core::fmt::write
      local.set 0
      local.get 3
      i32.const 64
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $<core::slice::ascii::EscapeAscii as core::fmt::Display>::fmt (;236;) (type 6) (param i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      i32.const 128
      local.set 3
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.load8_u offset=8
          i32.const 128
          i32.ne
          br_if 0 (;@2;)
          i32.const 128
          local.set 4
          br 1 (;@1;)
        end
        local.get 0
        i32.load offset=8
        local.tee 4
        i32.const 8
        i32.shr_u
        local.set 5
        local.get 0
        i32.const 13
        i32.add
        i32.load8_u
        local.set 6
        local.get 0
        i32.const 12
        i32.add
        i32.load8_u
        local.set 7
      end
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.load8_u offset=14
          i32.const 128
          i32.ne
          br_if 0 (;@2;)
          br 1 (;@1;)
        end
        local.get 0
        i32.load offset=14 align=2
        local.tee 3
        i32.const 8
        i32.shr_u
        local.set 8
        local.get 0
        i32.const 19
        i32.add
        i32.load8_u
        local.set 9
        local.get 0
        i32.const 18
        i32.add
        i32.load8_u
        local.set 10
      end
      local.get 0
      i32.load offset=4
      local.set 11
      local.get 0
      i32.load
      local.set 12
      local.get 2
      local.get 8
      i32.store16 offset=27 align=1
      local.get 2
      i32.const 29
      i32.add
      local.get 8
      i32.const 16
      i32.shr_u
      i32.store8
      local.get 2
      local.get 5
      i32.store16 offset=21 align=1
      local.get 2
      i32.const 23
      i32.add
      local.get 5
      i32.const 16
      i32.shr_u
      i32.store8
      local.get 2
      local.get 9
      i32.store8 offset=31
      local.get 2
      local.get 10
      i32.store8 offset=30
      local.get 2
      local.get 3
      i32.store8 offset=26
      local.get 2
      local.get 6
      i32.store8 offset=25
      local.get 2
      local.get 4
      i32.store8 offset=20
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                local.get 4
                i32.const 255
                i32.and
                i32.const 128
                i32.eq
                br_if 0 (;@5;)
                local.get 2
                i32.const 20
                i32.add
                local.set 13
                local.get 7
                i32.const 255
                i32.and
                local.tee 0
                i32.const 4
                local.get 0
                i32.const 4
                i32.gt_u
                select
                local.set 7
                local.get 6
                i32.const 255
                i32.and
                local.tee 5
                local.get 0
                local.get 5
                local.get 0
                i32.gt_u
                select
                local.set 8
                local.get 1
                i32.const 24
                i32.add
                i32.load
                local.set 14
                local.get 1
                i32.load offset=20
                local.set 15
                loop ;; label = @6
                  local.get 8
                  local.get 0
                  i32.eq
                  br_if 1 (;@5;)
                  local.get 2
                  local.get 0
                  i32.const 1
                  i32.add
                  local.tee 5
                  i32.store8 offset=24
                  local.get 7
                  local.get 0
                  i32.eq
                  br_if 4 (;@2;)
                  local.get 13
                  local.get 0
                  i32.add
                  local.set 4
                  local.get 5
                  local.set 0
                  local.get 15
                  local.get 4
                  i32.load8_u
                  local.get 14
                  i32.load offset=16
                  call_indirect (type 6)
                  i32.eqz
                  br_if 0 (;@6;)
                end
                i32.const 1
                local.set 16
                local.get 5
                i32.const -1
                i32.add
                local.get 6
                i32.const 255
                i32.and
                i32.lt_u
                br_if 1 (;@4;)
              end
              block ;; label = @5
                local.get 12
                i32.eqz
                br_if 0 (;@5;)
                local.get 12
                local.get 11
                i32.eq
                br_if 0 (;@5;)
                local.get 1
                i32.load offset=20
                local.set 6
                local.get 1
                i32.const 24
                i32.add
                i32.load
                i32.load offset=16
                local.set 17
                loop ;; label = @6
                  i32.const 116
                  local.set 8
                  i32.const 92
                  local.set 0
                  i32.const 0
                  local.set 13
                  i32.const 0
                  local.set 14
                  i32.const 0
                  local.set 5
                  i32.const 1
                  local.set 7
                  i32.const 0
                  local.set 15
                  i32.const 2
                  local.set 4
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
                                        local.get 12
                                        i32.load8_u
                                        local.tee 16
                                        i32.const -9
                                        i32.add
                                        br_table 10 (;@7;) 3 (;@14;) 1 (;@16;) 1 (;@16;) 2 (;@15;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 6 (;@11;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 1 (;@16;) 5 (;@12;) 0 (;@17;)
                                      end
                                      local.get 16
                                      i32.const 92
                                      i32.eq
                                      br_if 3 (;@13;)
                                    end
                                    local.get 16
                                    i32.extend8_s
                                    i32.const 0
                                    i32.lt_s
                                    br_if 5 (;@10;)
                                    local.get 16
                                    i32.const 32
                                    i32.lt_u
                                    br_if 5 (;@10;)
                                    local.get 16
                                    i32.const 127
                                    i32.eq
                                    br_if 5 (;@10;)
                                    i32.const 1
                                    local.set 5
                                    i32.const 0
                                    local.set 8
                                    local.get 16
                                    local.set 0
                                    i32.const 0
                                    local.set 13
                                    i32.const 0
                                    local.set 14
                                    i32.const 0
                                    local.set 7
                                    i32.const 0
                                    local.set 15
                                    i32.const 1
                                    local.set 4
                                    br 8 (;@7;)
                                  end
                                  i32.const 0
                                  local.set 13
                                  i32.const 114
                                  local.set 8
                                  br 5 (;@9;)
                                end
                                i32.const 0
                                local.set 13
                                i32.const 110
                                local.set 8
                                br 4 (;@9;)
                              end
                              i32.const 0
                              local.set 13
                              i32.const 92
                              local.set 0
                              i32.const 92
                              local.set 8
                              br 4 (;@8;)
                            end
                            i32.const 0
                            local.set 13
                            i32.const 39
                            local.set 8
                            br 2 (;@9;)
                          end
                          i32.const 0
                          local.set 13
                          i32.const 34
                          local.set 8
                          br 1 (;@9;)
                        end
                        i32.const 4
                        local.set 4
                        local.get 16
                        i32.const 4
                        i32.shr_u
                        i32.const 1055332
                        i32.add
                        i32.load8_u
                        i32.const 16
                        i32.shl
                        local.set 13
                        local.get 16
                        i32.const 15
                        i32.and
                        i32.const 1055332
                        i32.add
                        i32.load8_u
                        i32.const 24
                        i32.shl
                        local.set 14
                        i32.const 1
                        local.set 15
                        i32.const 0
                        local.set 5
                        i32.const 120
                        local.set 8
                        i32.const 92
                        local.set 0
                        i32.const 0
                        local.set 7
                        br 2 (;@7;)
                      end
                      i32.const 92
                      local.set 0
                    end
                    i32.const 0
                    local.set 14
                    i32.const 0
                    local.set 5
                    i32.const 1
                    local.set 7
                    i32.const 0
                    local.set 15
                    i32.const 2
                    local.set 4
                  end
                  block ;; label = @7
                    local.get 6
                    local.get 0
                    local.get 17
                    call_indirect (type 6)
                    i32.eqz
                    br_if 0 (;@7;)
                    i32.const 1
                    local.set 16
                    br 3 (;@4;)
                  end
                  i32.const 1
                  local.set 16
                  i32.const 1
                  local.set 0
                  block ;; label = @7
                    local.get 5
                    br_if 0 (;@7;)
                    block ;; label = @8
                      local.get 7
                      local.get 6
                      local.get 8
                      local.get 17
                      call_indirect (type 6)
                      local.tee 0
                      i32.or
                      i32.eqz
                      br_if 0 (;@8;)
                      i32.const 1
                      i32.const 2
                      local.get 0
                      select
                      local.set 0
                      br 1 (;@7;)
                    end
                    i32.const 2
                    local.set 0
                    local.get 6
                    local.get 13
                    i32.const 16
                    i32.shr_u
                    local.get 17
                    call_indirect (type 6)
                    br_if 0 (;@7;)
                    local.get 15
                    local.get 6
                    local.get 14
                    i32.const 24
                    i32.shr_u
                    local.get 17
                    call_indirect (type 6)
                    local.tee 0
                    i32.or
                    i32.const 1
                    i32.ne
                    br_if 6 (;@1;)
                    i32.const 3
                    i32.const 4
                    local.get 0
                    select
                    local.set 0
                  end
                  local.get 0
                  local.get 4
                  i32.lt_u
                  br_if 2 (;@4;)
                  local.get 12
                  i32.const 1
                  i32.add
                  local.tee 12
                  local.get 11
                  i32.ne
                  br_if 0 (;@6;)
                end
              end
              block ;; label = @5
                local.get 3
                i32.const 255
                i32.and
                i32.const 128
                i32.eq
                br_if 0 (;@5;)
                local.get 2
                i32.const 26
                i32.add
                local.set 7
                local.get 10
                i32.const 255
                i32.and
                local.tee 0
                i32.const 4
                local.get 0
                i32.const 4
                i32.gt_u
                select
                local.set 8
                local.get 9
                i32.const 255
                i32.and
                local.tee 5
                local.get 0
                local.get 5
                local.get 0
                i32.gt_u
                select
                local.set 12
                local.get 1
                i32.const 24
                i32.add
                i32.load
                local.set 13
                local.get 1
                i32.load offset=20
                local.set 14
                loop ;; label = @6
                  local.get 12
                  local.get 0
                  i32.eq
                  br_if 1 (;@5;)
                  local.get 2
                  local.get 0
                  i32.const 1
                  i32.add
                  local.tee 5
                  i32.store8 offset=30
                  local.get 8
                  local.get 0
                  i32.eq
                  br_if 3 (;@3;)
                  local.get 7
                  local.get 0
                  i32.add
                  local.set 4
                  local.get 5
                  local.set 0
                  local.get 14
                  local.get 4
                  i32.load8_u
                  local.get 13
                  i32.load offset=16
                  call_indirect (type 6)
                  i32.eqz
                  br_if 0 (;@6;)
                end
                i32.const 1
                local.set 16
                local.get 5
                i32.const -1
                i32.add
                local.get 9
                i32.const 255
                i32.and
                i32.lt_u
                br_if 1 (;@4;)
              end
              i32.const 0
              local.set 16
            end
            local.get 2
            i32.const 32
            i32.add
            global.set $__stack_pointer
            local.get 16
            return
          end
          local.get 8
          i32.const 4
          i32.const 1055396
          call $core::panicking::panic_bounds_check
          unreachable
        end
        local.get 7
        i32.const 4
        i32.const 1055396
        call $core::panicking::panic_bounds_check
        unreachable
      end
      i32.const 4
      i32.const 4
      i32.const 1055396
      call $core::panicking::panic_bounds_check
      unreachable
    )
    (func $core::ffi::c_str::CStr::from_bytes_with_nul (;237;) (type 4) (param i32 i32 i32)
      (local i32 i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    local.get 2
                    i32.const 8
                    i32.lt_u
                    br_if 0 (;@7;)
                    local.get 1
                    i32.const 3
                    i32.add
                    i32.const -4
                    i32.and
                    local.tee 3
                    local.get 1
                    i32.eq
                    br_if 1 (;@6;)
                    local.get 3
                    local.get 1
                    i32.sub
                    local.tee 3
                    i32.eqz
                    br_if 1 (;@6;)
                    i32.const 0
                    local.set 4
                    loop ;; label = @8
                      local.get 1
                      local.get 4
                      i32.add
                      i32.load8_u
                      i32.eqz
                      br_if 5 (;@3;)
                      local.get 3
                      local.get 4
                      i32.const 1
                      i32.add
                      local.tee 4
                      i32.ne
                      br_if 0 (;@8;)
                    end
                    local.get 3
                    local.get 2
                    i32.const -8
                    i32.add
                    local.tee 5
                    i32.gt_u
                    br_if 3 (;@4;)
                    br 2 (;@5;)
                  end
                  local.get 2
                  i32.eqz
                  br_if 4 (;@2;)
                  block ;; label = @7
                    local.get 1
                    i32.load8_u
                    br_if 0 (;@7;)
                    i32.const 0
                    local.set 4
                    br 4 (;@3;)
                  end
                  i32.const 1
                  local.set 4
                  local.get 2
                  i32.const 1
                  i32.eq
                  br_if 4 (;@2;)
                  local.get 1
                  i32.load8_u offset=1
                  i32.eqz
                  br_if 3 (;@3;)
                  i32.const 2
                  local.set 4
                  local.get 2
                  i32.const 2
                  i32.eq
                  br_if 4 (;@2;)
                  local.get 1
                  i32.load8_u offset=2
                  i32.eqz
                  br_if 3 (;@3;)
                  i32.const 3
                  local.set 4
                  local.get 2
                  i32.const 3
                  i32.eq
                  br_if 4 (;@2;)
                  local.get 1
                  i32.load8_u offset=3
                  i32.eqz
                  br_if 3 (;@3;)
                  i32.const 4
                  local.set 4
                  local.get 2
                  i32.const 4
                  i32.eq
                  br_if 4 (;@2;)
                  local.get 1
                  i32.load8_u offset=4
                  i32.eqz
                  br_if 3 (;@3;)
                  i32.const 5
                  local.set 4
                  local.get 2
                  i32.const 5
                  i32.eq
                  br_if 4 (;@2;)
                  local.get 1
                  i32.load8_u offset=5
                  i32.eqz
                  br_if 3 (;@3;)
                  i32.const 6
                  local.set 4
                  local.get 2
                  i32.const 6
                  i32.eq
                  br_if 4 (;@2;)
                  local.get 1
                  i32.load8_u offset=6
                  i32.eqz
                  br_if 3 (;@3;)
                  br 4 (;@2;)
                end
                local.get 2
                i32.const -8
                i32.add
                local.set 5
                i32.const 0
                local.set 3
              end
              loop ;; label = @5
                local.get 1
                local.get 3
                i32.add
                local.tee 6
                i32.load
                local.tee 4
                i32.const -1
                i32.xor
                local.get 4
                i32.const -16843009
                i32.add
                i32.and
                i32.const -2139062144
                i32.and
                br_if 1 (;@4;)
                local.get 6
                i32.const 4
                i32.add
                i32.load
                local.tee 4
                i32.const -1
                i32.xor
                local.get 4
                i32.const -16843009
                i32.add
                i32.and
                i32.const -2139062144
                i32.and
                br_if 1 (;@4;)
                local.get 3
                i32.const 8
                i32.add
                local.tee 3
                local.get 5
                i32.le_u
                br_if 0 (;@5;)
              end
            end
            local.get 3
            local.get 2
            i32.eq
            br_if 1 (;@2;)
            loop ;; label = @4
              block ;; label = @5
                local.get 1
                local.get 3
                i32.add
                i32.load8_u
                br_if 0 (;@5;)
                local.get 3
                local.set 4
                br 2 (;@3;)
              end
              local.get 2
              local.get 3
              i32.const 1
              i32.add
              local.tee 3
              i32.ne
              br_if 0 (;@4;)
              br 2 (;@2;)
            end
          end
          local.get 4
          i32.const 1
          i32.add
          local.get 2
          i32.eq
          br_if 1 (;@1;)
          local.get 0
          i32.const 0
          i32.store offset=4
          local.get 0
          i32.const 8
          i32.add
          local.get 4
          i32.store
          local.get 0
          i32.const 1
          i32.store
          return
        end
        local.get 0
        i32.const 1
        i32.store offset=4
        local.get 0
        i32.const 1
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
      local.get 0
      i32.const 0
      i32.store
    )
    (func $core::str::converts::from_utf8 (;238;) (type 4) (param i32 i32 i32)
      (local i32 i32 i32 i32 i32 i64 i64 i32)
      block ;; label = @1
        local.get 2
        i32.eqz
        br_if 0 (;@1;)
        i32.const 0
        local.get 2
        i32.const -7
        i32.add
        local.tee 3
        local.get 3
        local.get 2
        i32.gt_u
        select
        local.set 4
        local.get 1
        i32.const 3
        i32.add
        i32.const -4
        i32.and
        local.get 1
        i32.sub
        local.set 5
        i32.const 0
        local.set 3
        loop ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                local.get 1
                local.get 3
                i32.add
                i32.load8_u
                local.tee 6
                i32.extend8_s
                local.tee 7
                i32.const 0
                i32.lt_s
                br_if 0 (;@5;)
                block ;; label = @6
                  local.get 5
                  local.get 3
                  i32.sub
                  i32.const 3
                  i32.and
                  br_if 0 (;@6;)
                  local.get 3
                  local.get 4
                  i32.ge_u
                  br_if 2 (;@4;)
                  loop ;; label = @7
                    local.get 1
                    local.get 3
                    i32.add
                    local.tee 6
                    i32.load
                    i32.const -2139062144
                    i32.and
                    br_if 3 (;@4;)
                    local.get 6
                    i32.const 4
                    i32.add
                    i32.load
                    i32.const -2139062144
                    i32.and
                    br_if 3 (;@4;)
                    local.get 3
                    i32.const 8
                    i32.add
                    local.tee 3
                    local.get 4
                    i32.ge_u
                    br_if 3 (;@4;)
                    br 0 (;@7;)
                  end
                end
                local.get 3
                i32.const 1
                i32.add
                local.set 3
                br 2 (;@3;)
              end
              i64.const 1099511627776
              local.set 8
              i64.const 4294967296
              local.set 9
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
                                      local.get 6
                                      i32.const 1053256
                                      i32.add
                                      i32.load8_u
                                      i32.const -2
                                      i32.add
                                      br_table 0 (;@16;) 1 (;@15;) 2 (;@14;) 10 (;@6;)
                                    end
                                    local.get 3
                                    i32.const 1
                                    i32.add
                                    local.tee 6
                                    local.get 2
                                    i32.lt_u
                                    br_if 2 (;@13;)
                                    i64.const 0
                                    local.set 8
                                    i64.const 0
                                    local.set 9
                                    br 9 (;@6;)
                                  end
                                  i64.const 0
                                  local.set 8
                                  local.get 3
                                  i32.const 1
                                  i32.add
                                  local.tee 10
                                  local.get 2
                                  i32.lt_u
                                  br_if 2 (;@12;)
                                  i64.const 0
                                  local.set 9
                                  br 8 (;@6;)
                                end
                                i64.const 0
                                local.set 8
                                local.get 3
                                i32.const 1
                                i32.add
                                local.tee 10
                                local.get 2
                                i32.lt_u
                                br_if 2 (;@11;)
                                i64.const 0
                                local.set 9
                                br 7 (;@6;)
                              end
                              i64.const 1099511627776
                              local.set 8
                              i64.const 4294967296
                              local.set 9
                              local.get 1
                              local.get 6
                              i32.add
                              i32.load8_s
                              i32.const -65
                              i32.gt_s
                              br_if 6 (;@6;)
                              br 7 (;@5;)
                            end
                            local.get 1
                            local.get 10
                            i32.add
                            i32.load8_s
                            local.set 10
                            block ;; label = @12
                              block ;; label = @13
                                block ;; label = @14
                                  local.get 6
                                  i32.const -224
                                  i32.add
                                  br_table 0 (;@14;) 2 (;@12;) 2 (;@12;) 2 (;@12;) 2 (;@12;) 2 (;@12;) 2 (;@12;) 2 (;@12;) 2 (;@12;) 2 (;@12;) 2 (;@12;) 2 (;@12;) 2 (;@12;) 1 (;@13;) 2 (;@12;)
                                end
                                local.get 10
                                i32.const -32
                                i32.and
                                i32.const -96
                                i32.eq
                                br_if 4 (;@9;)
                                br 3 (;@10;)
                              end
                              local.get 10
                              i32.const -97
                              i32.gt_s
                              br_if 2 (;@10;)
                              br 3 (;@9;)
                            end
                            block ;; label = @12
                              local.get 7
                              i32.const 31
                              i32.add
                              i32.const 255
                              i32.and
                              i32.const 12
                              i32.lt_u
                              br_if 0 (;@12;)
                              local.get 7
                              i32.const -2
                              i32.and
                              i32.const -18
                              i32.ne
                              br_if 2 (;@10;)
                              local.get 10
                              i32.const -64
                              i32.lt_s
                              br_if 3 (;@9;)
                              br 2 (;@10;)
                            end
                            local.get 10
                            i32.const -64
                            i32.lt_s
                            br_if 2 (;@9;)
                            br 1 (;@10;)
                          end
                          local.get 1
                          local.get 10
                          i32.add
                          i32.load8_s
                          local.set 10
                          block ;; label = @11
                            block ;; label = @12
                              block ;; label = @13
                                block ;; label = @14
                                  local.get 6
                                  i32.const -240
                                  i32.add
                                  br_table 1 (;@13;) 0 (;@14;) 0 (;@14;) 0 (;@14;) 2 (;@12;) 0 (;@14;)
                                end
                                local.get 7
                                i32.const 15
                                i32.add
                                i32.const 255
                                i32.and
                                i32.const 2
                                i32.gt_u
                                br_if 3 (;@10;)
                                local.get 10
                                i32.const -64
                                i32.ge_s
                                br_if 3 (;@10;)
                                br 2 (;@11;)
                              end
                              local.get 10
                              i32.const 112
                              i32.add
                              i32.const 255
                              i32.and
                              i32.const 48
                              i32.ge_u
                              br_if 2 (;@10;)
                              br 1 (;@11;)
                            end
                            local.get 10
                            i32.const -113
                            i32.gt_s
                            br_if 1 (;@10;)
                          end
                          block ;; label = @11
                            local.get 3
                            i32.const 2
                            i32.add
                            local.tee 6
                            local.get 2
                            i32.lt_u
                            br_if 0 (;@11;)
                            i64.const 0
                            local.set 9
                            br 5 (;@6;)
                          end
                          local.get 1
                          local.get 6
                          i32.add
                          i32.load8_s
                          i32.const -65
                          i32.gt_s
                          br_if 2 (;@8;)
                          i64.const 0
                          local.set 9
                          local.get 3
                          i32.const 3
                          i32.add
                          local.tee 6
                          local.get 2
                          i32.ge_u
                          br_if 4 (;@6;)
                          local.get 1
                          local.get 6
                          i32.add
                          i32.load8_s
                          i32.const -65
                          i32.le_s
                          br_if 5 (;@5;)
                          i64.const 3298534883328
                          local.set 8
                          br 3 (;@7;)
                        end
                        i64.const 1099511627776
                        local.set 8
                        br 2 (;@7;)
                      end
                      i64.const 0
                      local.set 9
                      local.get 3
                      i32.const 2
                      i32.add
                      local.tee 6
                      local.get 2
                      i32.ge_u
                      br_if 2 (;@6;)
                      local.get 1
                      local.get 6
                      i32.add
                      i32.load8_s
                      i32.const -65
                      i32.le_s
                      br_if 3 (;@5;)
                    end
                    i64.const 2199023255552
                    local.set 8
                  end
                  i64.const 4294967296
                  local.set 9
                end
                local.get 0
                local.get 8
                local.get 3
                i64.extend_i32_u
                i64.or
                local.get 9
                i64.or
                i64.store offset=4 align=4
                local.get 0
                i32.const 1
                i32.store
                return
              end
              local.get 6
              i32.const 1
              i32.add
              local.set 3
              br 1 (;@3;)
            end
            local.get 3
            local.get 2
            i32.ge_u
            br_if 0 (;@3;)
            loop ;; label = @4
              local.get 1
              local.get 3
              i32.add
              i32.load8_s
              i32.const 0
              i32.lt_s
              br_if 1 (;@3;)
              local.get 2
              local.get 3
              i32.const 1
              i32.add
              local.tee 3
              i32.ne
              br_if 0 (;@4;)
              br 3 (;@1;)
            end
          end
          local.get 3
          local.get 2
          i32.lt_u
          br_if 0 (;@2;)
        end
      end
      local.get 0
      local.get 1
      i32.store offset=4
      local.get 0
      i32.const 8
      i32.add
      local.get 2
      i32.store
      local.get 0
      i32.const 0
      i32.store
    )
    (func $core::fmt::num::imp::<impl core::fmt::Display for u8>::fmt (;239;) (type 6) (param i32 i32) (result i32)
      local.get 0
      i64.load8_u
      i32.const 1
      local.get 1
      call $core::fmt::num::imp::fmt_u64
    )
    (func $core::result::unwrap_failed (;240;) (type 14) (param i32 i32 i32 i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 64
      i32.sub
      local.tee 5
      global.set $__stack_pointer
      local.get 5
      local.get 1
      i32.store offset=12
      local.get 5
      local.get 0
      i32.store offset=8
      local.get 5
      local.get 3
      i32.store offset=20
      local.get 5
      local.get 2
      i32.store offset=16
      local.get 5
      i32.const 60
      i32.add
      i32.const 94
      i32.store
      local.get 5
      i32.const 95
      i32.store offset=52
      local.get 5
      local.get 5
      i32.const 16
      i32.add
      i32.store offset=56
      local.get 5
      local.get 5
      i32.const 8
      i32.add
      i32.store offset=48
      local.get 5
      i32.const 24
      i32.add
      i32.const 1052720
      i32.const 2
      local.get 5
      i32.const 48
      i32.add
      i32.const 2
      call $#func223<core::fmt::Arguments::new_v1>
      local.get 5
      i32.const 24
      i32.add
      local.get 4
      call $core::panicking::panic_fmt
      unreachable
    )
    (func $core::option::expect_failed (;241;) (type 4) (param i32 i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 3
      local.get 1
      i32.store offset=12
      local.get 3
      local.get 0
      i32.store offset=8
      local.get 3
      i32.const 95
      i32.store offset=44
      local.get 3
      local.get 3
      i32.const 8
      i32.add
      i32.store offset=40
      local.get 3
      i32.const 16
      i32.add
      i32.const 1052436
      i32.const 1
      local.get 3
      i32.const 40
      i32.add
      i32.const 1
      call $#func223<core::fmt::Arguments::new_v1>
      local.get 3
      i32.const 16
      i32.add
      local.get 2
      call $core::panicking::panic_fmt
      unreachable
    )
    (func $<&T as core::fmt::Display>::fmt (;242;) (type 6) (param i32 i32) (result i32)
      local.get 1
      local.get 0
      i32.load
      local.get 0
      i32.load offset=4
      call $core::fmt::Formatter::pad
    )
    (func $<core::panic::location::Location as core::fmt::Display>::fmt (;243;) (type 6) (param i32 i32) (result i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 2
      i32.const 44
      i32.add
      i32.const 9
      i32.store
      local.get 2
      i32.const 24
      i32.add
      i32.const 12
      i32.add
      i32.const 9
      i32.store
      local.get 2
      i32.const 12
      i32.add
      i64.const 3
      i64.store align=4
      local.get 2
      i32.const 3
      i32.store offset=4
      local.get 2
      i32.const 1052448
      i32.store
      local.get 2
      i32.const 95
      i32.store offset=28
      local.get 2
      local.get 0
      i32.store offset=24
      local.get 2
      local.get 0
      i32.const 12
      i32.add
      i32.store offset=40
      local.get 2
      local.get 0
      i32.const 8
      i32.add
      i32.store offset=32
      local.get 1
      i32.const 24
      i32.add
      i32.load
      local.set 0
      local.get 2
      local.get 2
      i32.const 24
      i32.add
      i32.store offset=8
      local.get 1
      i32.load offset=20
      local.get 0
      local.get 2
      call $core::fmt::write
      local.set 0
      local.get 2
      i32.const 48
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $core::panic::panic_info::PanicInfo::payload (;244;) (type 2) (param i32 i32)
      local.get 0
      local.get 1
      i64.load align=4
      i64.store
    )
    (func $core::panic::panic_info::PanicInfo::message (;245;) (type 3) (param i32) (result i32)
      local.get 0
      i32.load offset=12
    )
    (func $core::panic::panic_info::PanicInfo::location (;246;) (type 3) (param i32) (result i32)
      local.get 0
      i32.load offset=8
    )
    (func $core::panic::panic_info::PanicInfo::can_unwind (;247;) (type 3) (param i32) (result i32)
      local.get 0
      i32.load8_u offset=16
    )
    (func $<core::panic::panic_info::PanicInfo as core::fmt::Display>::fmt (;248;) (type 6) (param i32 i32) (result i32)
      (local i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 64
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      i32.const 1
      local.set 3
      block ;; label = @1
        local.get 1
        i32.load offset=20
        local.tee 4
        i32.const 1052472
        i32.const 12
        local.get 1
        i32.const 24
        i32.add
        i32.load
        local.tee 5
        i32.load offset=12
        local.tee 6
        call_indirect (type 5)
        br_if 0 (;@1;)
        local.get 0
        i32.load offset=8
        local.set 1
        local.get 2
        i32.const 16
        i32.add
        i32.const 12
        i32.add
        i64.const 3
        i64.store align=4
        local.get 2
        i32.const 60
        i32.add
        i32.const 9
        i32.store
        local.get 2
        i32.const 40
        i32.add
        i32.const 12
        i32.add
        i32.const 9
        i32.store
        local.get 2
        i32.const 3
        i32.store offset=20
        local.get 2
        i32.const 1052448
        i32.store offset=16
        local.get 2
        local.get 1
        i32.const 12
        i32.add
        i32.store offset=56
        local.get 2
        local.get 1
        i32.const 8
        i32.add
        i32.store offset=48
        local.get 2
        i32.const 95
        i32.store offset=44
        local.get 2
        local.get 1
        i32.store offset=40
        local.get 2
        local.get 2
        i32.const 40
        i32.add
        i32.store offset=24
        local.get 4
        local.get 5
        local.get 2
        i32.const 16
        i32.add
        call $core::fmt::write
        br_if 0 (;@1;)
        block ;; label = @2
          block ;; label = @3
            local.get 0
            i32.load offset=12
            local.tee 1
            i32.eqz
            br_if 0 (;@3;)
            local.get 4
            i32.const 1052484
            i32.const 2
            local.get 6
            call_indirect (type 5)
            br_if 2 (;@1;)
            local.get 2
            i32.const 40
            i32.add
            i32.const 16
            i32.add
            local.get 1
            i32.const 16
            i32.add
            i64.load align=4
            i64.store
            local.get 2
            i32.const 40
            i32.add
            i32.const 8
            i32.add
            local.get 1
            i32.const 8
            i32.add
            i64.load align=4
            i64.store
            local.get 2
            local.get 1
            i64.load align=4
            i64.store offset=40
            local.get 4
            local.get 5
            local.get 2
            i32.const 40
            i32.add
            call $core::fmt::write
            br_if 2 (;@1;)
            br 1 (;@2;)
          end
          local.get 2
          local.get 0
          i32.load
          local.tee 1
          local.get 0
          i32.load offset=4
          i32.load offset=12
          call_indirect (type 2)
          local.get 2
          i64.load
          i64.const -4493808902380553279
          i64.xor
          local.get 2
          i32.const 8
          i32.add
          i64.load
          i64.const -163230743173927068
          i64.xor
          i64.or
          i64.eqz
          i32.eqz
          br_if 0 (;@2;)
          local.get 4
          i32.const 1052484
          i32.const 2
          local.get 6
          call_indirect (type 5)
          br_if 1 (;@1;)
          local.get 4
          local.get 1
          i32.load
          local.get 1
          i32.load offset=4
          local.get 6
          call_indirect (type 5)
          br_if 1 (;@1;)
        end
        i32.const 0
        local.set 3
      end
      local.get 2
      i32.const 64
      i32.add
      global.set $__stack_pointer
      local.get 3
    )
    (func $core::fmt::num::<impl core::fmt::LowerHex for i32>::fmt (;249;) (type 6) (param i32 i32) (result i32)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 128
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 0
      i32.load
      local.set 0
      i32.const 0
      local.set 3
      loop ;; label = @1
        local.get 2
        local.get 3
        i32.add
        i32.const 127
        i32.add
        i32.const 48
        i32.const 87
        local.get 0
        i32.const 15
        i32.and
        local.tee 4
        i32.const 10
        i32.lt_u
        select
        local.get 4
        i32.add
        i32.store8
        local.get 3
        i32.const -1
        i32.add
        local.set 3
        local.get 0
        i32.const 16
        i32.lt_u
        local.set 4
        local.get 0
        i32.const 4
        i32.shr_u
        local.set 0
        local.get 4
        i32.eqz
        br_if 0 (;@1;)
      end
      block ;; label = @1
        local.get 3
        i32.const 128
        i32.add
        local.tee 0
        i32.const 128
        i32.le_u
        br_if 0 (;@1;)
        local.get 0
        i32.const 128
        i32.const 1052804
        call $core::slice::index::slice_start_index_len_fail
        unreachable
      end
      local.get 1
      i32.const 1
      i32.const 1052774
      i32.const 2
      local.get 2
      local.get 3
      i32.add
      i32.const 128
      i32.add
      i32.const 0
      local.get 3
      i32.sub
      call $core::fmt::Formatter::pad_integral
      local.set 0
      local.get 2
      i32.const 128
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $core::panicking::assert_failed_inner (;250;) (type 18) (param i32 i32 i32 i32 i32 i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 112
      i32.sub
      local.tee 7
      global.set $__stack_pointer
      local.get 7
      local.get 2
      i32.store offset=12
      local.get 7
      local.get 1
      i32.store offset=8
      local.get 7
      local.get 4
      i32.store offset=20
      local.get 7
      local.get 3
      i32.store offset=16
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 0
              i32.const 255
              i32.and
              br_table 0 (;@4;) 1 (;@3;) 2 (;@2;) 0 (;@4;)
            end
            local.get 7
            i32.const 1052572
            i32.store offset=24
            i32.const 2
            local.set 2
            br 2 (;@1;)
          end
          local.get 7
          i32.const 1052574
          i32.store offset=24
          i32.const 2
          local.set 2
          br 1 (;@1;)
        end
        local.get 7
        i32.const 1052576
        i32.store offset=24
        i32.const 7
        local.set 2
      end
      local.get 7
      local.get 2
      i32.store offset=28
      block ;; label = @1
        local.get 5
        i32.load
        br_if 0 (;@1;)
        local.get 7
        i32.const 76
        i32.add
        i32.const 94
        i32.store
        local.get 7
        i32.const 68
        i32.add
        i32.const 94
        i32.store
        local.get 7
        i32.const 95
        i32.store offset=60
        local.get 7
        local.get 7
        i32.const 16
        i32.add
        i32.store offset=72
        local.get 7
        local.get 7
        i32.const 8
        i32.add
        i32.store offset=64
        local.get 7
        local.get 7
        i32.const 24
        i32.add
        i32.store offset=56
        local.get 7
        i32.const 88
        i32.add
        i32.const 1052632
        i32.const 3
        local.get 7
        i32.const 56
        i32.add
        i32.const 3
        call $#func223<core::fmt::Arguments::new_v1>
        local.get 7
        i32.const 88
        i32.add
        local.get 6
        call $core::panicking::panic_fmt
        unreachable
      end
      local.get 7
      i32.const 32
      i32.add
      i32.const 16
      i32.add
      local.get 5
      i32.const 16
      i32.add
      i64.load align=4
      i64.store
      local.get 7
      i32.const 32
      i32.add
      i32.const 8
      i32.add
      local.get 5
      i32.const 8
      i32.add
      i64.load align=4
      i64.store
      local.get 7
      local.get 5
      i64.load align=4
      i64.store offset=32
      local.get 7
      i32.const 84
      i32.add
      i32.const 94
      i32.store
      local.get 7
      i32.const 76
      i32.add
      i32.const 94
      i32.store
      local.get 7
      i32.const 68
      i32.add
      i32.const 96
      i32.store
      local.get 7
      i32.const 95
      i32.store offset=60
      local.get 7
      local.get 7
      i32.const 16
      i32.add
      i32.store offset=80
      local.get 7
      local.get 7
      i32.const 8
      i32.add
      i32.store offset=72
      local.get 7
      local.get 7
      i32.const 32
      i32.add
      i32.store offset=64
      local.get 7
      local.get 7
      i32.const 24
      i32.add
      i32.store offset=56
      local.get 7
      i32.const 88
      i32.add
      i32.const 1052684
      i32.const 4
      local.get 7
      i32.const 56
      i32.add
      i32.const 4
      call $#func223<core::fmt::Arguments::new_v1>
      local.get 7
      i32.const 88
      i32.add
      local.get 6
      call $core::panicking::panic_fmt
      unreachable
    )
    (func $<&T as core::fmt::Debug>::fmt (;251;) (type 6) (param i32 i32) (result i32)
      local.get 0
      i32.load
      local.get 1
      local.get 0
      i32.load offset=4
      i32.load offset=12
      call_indirect (type 6)
    )
    (func $<core::fmt::Arguments as core::fmt::Display>::fmt (;252;) (type 6) (param i32 i32) (result i32)
      local.get 1
      i32.load offset=20
      local.get 1
      i32.const 24
      i32.add
      i32.load
      local.get 0
      call $core::fmt::write
    )
    (func $<core::fmt::builders::PadAdapter as core::fmt::Write>::write_str (;253;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
      local.get 0
      i32.load offset=4
      local.set 3
      local.get 0
      i32.load
      local.set 4
      local.get 0
      i32.load offset=8
      local.set 5
      i32.const 0
      local.set 6
      i32.const 0
      local.set 7
      i32.const 0
      local.set 8
      i32.const 0
      local.set 9
      block ;; label = @1
        loop ;; label = @2
          local.get 9
          i32.const 255
          i32.and
          br_if 1 (;@1;)
          block ;; label = @3
            block ;; label = @4
              local.get 8
              local.get 2
              i32.gt_u
              br_if 0 (;@4;)
              loop ;; label = @5
                local.get 1
                local.get 8
                i32.add
                local.set 10
                block ;; label = @6
                  block ;; label = @7
                    local.get 2
                    local.get 8
                    i32.sub
                    local.tee 11
                    i32.const 8
                    i32.lt_u
                    br_if 0 (;@7;)
                    block ;; label = @8
                      block ;; label = @9
                        block ;; label = @10
                          local.get 10
                          i32.const 3
                          i32.add
                          i32.const -4
                          i32.and
                          local.tee 0
                          local.get 10
                          i32.eq
                          br_if 0 (;@10;)
                          local.get 0
                          local.get 10
                          i32.sub
                          local.tee 12
                          i32.eqz
                          br_if 0 (;@10;)
                          i32.const 0
                          local.set 0
                          loop ;; label = @11
                            local.get 10
                            local.get 0
                            i32.add
                            i32.load8_u
                            i32.const 10
                            i32.eq
                            br_if 5 (;@6;)
                            local.get 12
                            local.get 0
                            i32.const 1
                            i32.add
                            local.tee 0
                            i32.ne
                            br_if 0 (;@11;)
                          end
                          local.get 12
                          local.get 11
                          i32.const -8
                          i32.add
                          local.tee 13
                          i32.le_u
                          br_if 1 (;@9;)
                          br 2 (;@8;)
                        end
                        local.get 11
                        i32.const -8
                        i32.add
                        local.set 13
                        i32.const 0
                        local.set 12
                      end
                      loop ;; label = @9
                        local.get 10
                        local.get 12
                        i32.add
                        local.tee 9
                        i32.load
                        local.tee 0
                        i32.const -1
                        i32.xor
                        local.get 0
                        i32.const 168430090
                        i32.xor
                        i32.const -16843009
                        i32.add
                        i32.and
                        i32.const -2139062144
                        i32.and
                        br_if 1 (;@8;)
                        local.get 9
                        i32.const 4
                        i32.add
                        i32.load
                        local.tee 0
                        i32.const -1
                        i32.xor
                        local.get 0
                        i32.const 168430090
                        i32.xor
                        i32.const -16843009
                        i32.add
                        i32.and
                        i32.const -2139062144
                        i32.and
                        br_if 1 (;@8;)
                        local.get 12
                        i32.const 8
                        i32.add
                        local.tee 12
                        local.get 13
                        i32.le_u
                        br_if 0 (;@9;)
                      end
                    end
                    block ;; label = @8
                      local.get 12
                      local.get 11
                      i32.ne
                      br_if 0 (;@8;)
                      local.get 2
                      local.set 8
                      br 4 (;@4;)
                    end
                    loop ;; label = @8
                      block ;; label = @9
                        local.get 10
                        local.get 12
                        i32.add
                        i32.load8_u
                        i32.const 10
                        i32.ne
                        br_if 0 (;@9;)
                        local.get 12
                        local.set 0
                        br 3 (;@6;)
                      end
                      local.get 11
                      local.get 12
                      i32.const 1
                      i32.add
                      local.tee 12
                      i32.ne
                      br_if 0 (;@8;)
                    end
                    local.get 2
                    local.set 8
                    br 3 (;@4;)
                  end
                  block ;; label = @7
                    local.get 2
                    local.get 8
                    i32.ne
                    br_if 0 (;@7;)
                    local.get 2
                    local.set 8
                    br 3 (;@4;)
                  end
                  i32.const 0
                  local.set 0
                  loop ;; label = @7
                    local.get 10
                    local.get 0
                    i32.add
                    i32.load8_u
                    i32.const 10
                    i32.eq
                    br_if 1 (;@6;)
                    local.get 11
                    local.get 0
                    i32.const 1
                    i32.add
                    local.tee 0
                    i32.ne
                    br_if 0 (;@7;)
                  end
                  local.get 2
                  local.set 8
                  br 2 (;@4;)
                end
                local.get 8
                local.get 0
                i32.add
                local.tee 0
                i32.const 1
                i32.add
                local.set 8
                block ;; label = @6
                  local.get 0
                  local.get 2
                  i32.ge_u
                  br_if 0 (;@6;)
                  local.get 1
                  local.get 0
                  i32.add
                  i32.load8_u
                  i32.const 10
                  i32.ne
                  br_if 0 (;@6;)
                  i32.const 0
                  local.set 9
                  local.get 8
                  local.set 13
                  local.get 8
                  local.set 0
                  br 3 (;@3;)
                end
                local.get 8
                local.get 2
                i32.le_u
                br_if 0 (;@5;)
              end
            end
            i32.const 1
            local.set 9
            local.get 7
            local.set 13
            local.get 2
            local.set 0
            local.get 7
            local.get 2
            i32.eq
            br_if 2 (;@1;)
          end
          block ;; label = @3
            block ;; label = @4
              local.get 5
              i32.load8_u
              i32.eqz
              br_if 0 (;@4;)
              local.get 4
              i32.const 1052760
              i32.const 4
              local.get 3
              i32.load offset=12
              call_indirect (type 5)
              br_if 1 (;@3;)
            end
            local.get 1
            local.get 7
            i32.add
            local.set 12
            local.get 0
            local.get 7
            i32.sub
            local.set 10
            i32.const 0
            local.set 11
            block ;; label = @4
              local.get 0
              local.get 7
              i32.eq
              br_if 0 (;@4;)
              local.get 10
              local.get 12
              i32.add
              i32.const -1
              i32.add
              i32.load8_u
              i32.const 10
              i32.eq
              local.set 11
            end
            local.get 5
            local.get 11
            i32.store8
            local.get 13
            local.set 7
            local.get 4
            local.get 12
            local.get 10
            local.get 3
            i32.load offset=12
            call_indirect (type 5)
            i32.eqz
            br_if 1 (;@2;)
          end
        end
        i32.const 1
        local.set 6
      end
      local.get 6
    )
    (func $core::fmt::builders::DebugTuple::field (;254;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i64)
      global.get $__stack_pointer
      i32.const 64
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      local.get 0
      i32.load
      local.set 4
      i32.const 1
      local.set 5
      block ;; label = @1
        local.get 0
        i32.load8_u offset=8
        br_if 0 (;@1;)
        block ;; label = @2
          local.get 0
          i32.load offset=4
          local.tee 6
          i32.load offset=28
          local.tee 7
          i32.const 4
          i32.and
          br_if 0 (;@2;)
          i32.const 1
          local.set 5
          local.get 6
          i32.load offset=20
          i32.const 1052764
          i32.const 1052768
          local.get 4
          select
          i32.const 2
          i32.const 1
          local.get 4
          select
          local.get 6
          i32.const 24
          i32.add
          i32.load
          i32.load offset=12
          call_indirect (type 5)
          br_if 1 (;@1;)
          local.get 1
          local.get 6
          local.get 2
          i32.load offset=12
          call_indirect (type 6)
          local.set 5
          br 1 (;@1;)
        end
        block ;; label = @2
          local.get 4
          br_if 0 (;@2;)
          i32.const 1
          local.set 5
          local.get 6
          i32.load offset=20
          i32.const 1052769
          i32.const 2
          local.get 6
          i32.const 24
          i32.add
          i32.load
          i32.load offset=12
          call_indirect (type 5)
          br_if 1 (;@1;)
          local.get 6
          i32.load offset=28
          local.set 7
        end
        i32.const 1
        local.set 5
        local.get 3
        i32.const 1
        i32.store8 offset=27
        local.get 3
        i32.const 52
        i32.add
        i32.const 1052736
        i32.store
        local.get 3
        local.get 6
        i64.load offset=20 align=4
        i64.store offset=12 align=4
        local.get 3
        local.get 3
        i32.const 27
        i32.add
        i32.store offset=20
        local.get 3
        local.get 6
        i64.load offset=8 align=4
        i64.store offset=36 align=4
        local.get 6
        i64.load align=4
        local.set 8
        local.get 3
        local.get 7
        i32.store offset=56
        local.get 3
        local.get 6
        i32.load offset=16
        i32.store offset=44
        local.get 3
        local.get 6
        i32.load8_u offset=32
        i32.store8 offset=60
        local.get 3
        local.get 8
        i64.store offset=28 align=4
        local.get 3
        local.get 3
        i32.const 12
        i32.add
        i32.store offset=48
        local.get 1
        local.get 3
        i32.const 28
        i32.add
        local.get 2
        i32.load offset=12
        call_indirect (type 6)
        br_if 0 (;@1;)
        local.get 3
        i32.load offset=48
        i32.const 1052766
        i32.const 2
        local.get 3
        i32.load offset=52
        i32.load offset=12
        call_indirect (type 5)
        local.set 5
      end
      local.get 0
      local.get 5
      i32.store8 offset=8
      local.get 0
      local.get 4
      i32.const 1
      i32.add
      i32.store
      local.get 3
      i32.const 64
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $core::fmt::builders::DebugSet::entry (;255;) (type 5) (param i32 i32 i32) (result i32)
      local.get 0
      local.get 1
      local.get 2
      call $core::fmt::builders::DebugInner::entry
      local.get 0
    )
    (func $core::fmt::builders::DebugList::finish (;256;) (type 3) (param i32) (result i32)
      (local i32)
      i32.const 1
      local.set 1
      block ;; label = @1
        local.get 0
        i32.load8_u offset=4
        br_if 0 (;@1;)
        local.get 0
        i32.load
        local.tee 0
        i32.load offset=20
        i32.const 1052773
        i32.const 1
        local.get 0
        i32.const 24
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 5)
        local.set 1
      end
      local.get 1
    )
    (func $core::fmt::Write::write_char (;257;) (type 6) (param i32 i32) (result i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 2
      i32.const 0
      i32.store offset=12
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
              i32.store8 offset=14
              local.get 2
              local.get 1
              i32.const 12
              i32.shr_u
              i32.const 224
              i32.or
              i32.store8 offset=12
              local.get 2
              local.get 1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              i32.const 3
              local.set 1
              br 3 (;@1;)
            end
            local.get 2
            local.get 1
            i32.store8 offset=12
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
          i32.store8 offset=13
          local.get 2
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 192
          i32.or
          i32.store8 offset=12
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
        i32.store8 offset=15
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=14
        local.get 2
        local.get 1
        i32.const 12
        i32.shr_u
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 18
        i32.shr_u
        i32.const 7
        i32.and
        i32.const 240
        i32.or
        i32.store8 offset=12
        i32.const 4
        local.set 1
      end
      local.get 0
      local.get 2
      i32.const 12
      i32.add
      local.get 1
      call $<core::fmt::builders::PadAdapter as core::fmt::Write>::write_str
      local.set 1
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 1
    )
    (func $core::fmt::Write::write_fmt (;258;) (type 6) (param i32 i32) (result i32)
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
      i32.const 1053020
      local.get 1
      call $core::fmt::write
      local.set 0
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $<&mut W as core::fmt::Write>::write_str (;259;) (type 5) (param i32 i32 i32) (result i32)
      local.get 0
      i32.load
      local.get 1
      local.get 2
      call $<core::fmt::builders::PadAdapter as core::fmt::Write>::write_str
    )
    (func $<&mut W as core::fmt::Write>::write_char (;260;) (type 6) (param i32 i32) (result i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 0
      i32.load
      local.set 0
      local.get 2
      i32.const 0
      i32.store offset=12
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
              i32.store8 offset=14
              local.get 2
              local.get 1
              i32.const 12
              i32.shr_u
              i32.const 224
              i32.or
              i32.store8 offset=12
              local.get 2
              local.get 1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              i32.const 3
              local.set 1
              br 3 (;@1;)
            end
            local.get 2
            local.get 1
            i32.store8 offset=12
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
          i32.store8 offset=13
          local.get 2
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 192
          i32.or
          i32.store8 offset=12
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
        i32.store8 offset=15
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=14
        local.get 2
        local.get 1
        i32.const 12
        i32.shr_u
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 18
        i32.shr_u
        i32.const 7
        i32.and
        i32.const 240
        i32.or
        i32.store8 offset=12
        i32.const 4
        local.set 1
      end
      local.get 0
      local.get 2
      i32.const 12
      i32.add
      local.get 1
      call $<core::fmt::builders::PadAdapter as core::fmt::Write>::write_str
      local.set 1
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 1
    )
    (func $<&mut W as core::fmt::Write>::write_fmt (;261;) (type 6) (param i32 i32) (result i32)
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
      i32.const 1053020
      local.get 1
      call $core::fmt::write
      local.set 0
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $core::str::count::do_count_chars (;262;) (type 6) (param i32 i32) (result i32)
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
    (func $core::fmt::Formatter::pad_integral::write_prefix (;263;) (type 15) (param i32 i32 i32 i32 i32) (result i32)
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
            call_indirect (type 6)
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
      call_indirect (type 5)
    )
    (func $core::fmt::Formatter::write_str (;264;) (type 5) (param i32 i32 i32) (result i32)
      local.get 0
      i32.load offset=20
      local.get 1
      local.get 2
      local.get 0
      i32.const 24
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type 5)
    )
    (func $core::fmt::Formatter::write_fmt (;265;) (type 6) (param i32 i32) (result i32)
      local.get 0
      i32.load offset=20
      local.get 0
      i32.const 24
      i32.add
      i32.load
      local.get 1
      call $core::fmt::write
    )
    (func $core::fmt::Formatter::debug_lower_hex (;266;) (type 3) (param i32) (result i32)
      local.get 0
      i32.load8_u offset=28
      i32.const 16
      i32.and
      i32.const 4
      i32.shr_u
    )
    (func $core::fmt::Formatter::debug_upper_hex (;267;) (type 3) (param i32) (result i32)
      local.get 0
      i32.load8_u offset=28
      i32.const 32
      i32.and
      i32.const 5
      i32.shr_u
    )
    (func $core::fmt::Formatter::debug_tuple_field1_finish (;268;) (type 15) (param i32 i32 i32 i32 i32) (result i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 5
      global.set $__stack_pointer
      local.get 5
      local.get 0
      i32.load offset=20
      local.get 1
      local.get 2
      local.get 0
      i32.const 24
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type 5)
      i32.store8 offset=12
      local.get 5
      local.get 0
      i32.store offset=8
      local.get 5
      local.get 2
      i32.eqz
      i32.store8 offset=13
      local.get 5
      i32.const 0
      i32.store offset=4
      local.get 5
      i32.const 4
      i32.add
      local.get 3
      local.get 4
      call $core::fmt::builders::DebugTuple::field
      local.set 0
      local.get 5
      i32.load8_u offset=12
      local.set 2
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.load
          local.tee 1
          br_if 0 (;@2;)
          local.get 2
          i32.const 255
          i32.and
          i32.const 0
          i32.ne
          local.set 0
          br 1 (;@1;)
        end
        i32.const 1
        local.set 0
        local.get 2
        i32.const 255
        i32.and
        br_if 0 (;@1;)
        local.get 5
        i32.load offset=8
        local.set 2
        block ;; label = @2
          local.get 1
          i32.const 1
          i32.ne
          br_if 0 (;@2;)
          local.get 5
          i32.load8_u offset=13
          i32.const 255
          i32.and
          i32.eqz
          br_if 0 (;@2;)
          local.get 2
          i32.load8_u offset=28
          i32.const 4
          i32.and
          br_if 0 (;@2;)
          i32.const 1
          local.set 0
          local.get 2
          i32.load offset=20
          i32.const 1052771
          i32.const 1
          local.get 2
          i32.const 24
          i32.add
          i32.load
          i32.load offset=12
          call_indirect (type 5)
          br_if 1 (;@1;)
        end
        local.get 2
        i32.load offset=20
        i32.const 1052343
        i32.const 1
        local.get 2
        i32.const 24
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 5)
        local.set 0
      end
      local.get 5
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $core::fmt::Formatter::debug_tuple_field2_finish (;269;) (type 19) (param i32 i32 i32 i32 i32 i32 i32) (result i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 7
      global.set $__stack_pointer
      local.get 7
      local.get 0
      i32.load offset=20
      local.get 1
      local.get 2
      local.get 0
      i32.const 24
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type 5)
      i32.store8 offset=12
      local.get 7
      local.get 0
      i32.store offset=8
      local.get 7
      local.get 2
      i32.eqz
      i32.store8 offset=13
      local.get 7
      i32.const 0
      i32.store offset=4
      local.get 7
      i32.const 4
      i32.add
      local.get 3
      local.get 4
      call $core::fmt::builders::DebugTuple::field
      local.get 5
      local.get 6
      call $core::fmt::builders::DebugTuple::field
      local.set 0
      local.get 7
      i32.load8_u offset=12
      local.set 2
      block ;; label = @1
        block ;; label = @2
          local.get 0
          i32.load
          local.tee 1
          br_if 0 (;@2;)
          local.get 2
          i32.const 255
          i32.and
          i32.const 0
          i32.ne
          local.set 0
          br 1 (;@1;)
        end
        i32.const 1
        local.set 0
        local.get 2
        i32.const 255
        i32.and
        br_if 0 (;@1;)
        local.get 7
        i32.load offset=8
        local.set 2
        block ;; label = @2
          local.get 1
          i32.const 1
          i32.ne
          br_if 0 (;@2;)
          local.get 7
          i32.load8_u offset=13
          i32.const 255
          i32.and
          i32.eqz
          br_if 0 (;@2;)
          local.get 2
          i32.load8_u offset=28
          i32.const 4
          i32.and
          br_if 0 (;@2;)
          i32.const 1
          local.set 0
          local.get 2
          i32.load offset=20
          i32.const 1052771
          i32.const 1
          local.get 2
          i32.const 24
          i32.add
          i32.load
          i32.load offset=12
          call_indirect (type 5)
          br_if 1 (;@1;)
        end
        local.get 2
        i32.load offset=20
        i32.const 1052343
        i32.const 1
        local.get 2
        i32.const 24
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 5)
        local.set 0
      end
      local.get 7
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $core::fmt::Formatter::debug_list (;270;) (type 2) (param i32 i32)
      (local i32)
      local.get 1
      i32.load offset=20
      i32.const 1052445
      i32.const 1
      local.get 1
      i32.const 24
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type 5)
      local.set 2
      local.get 0
      i32.const 0
      i32.store8 offset=5
      local.get 0
      local.get 2
      i32.store8 offset=4
      local.get 0
      local.get 1
      i32.store
    )
    (func $<core::fmt::Formatter as core::fmt::Write>::write_char (;271;) (type 6) (param i32 i32) (result i32)
      local.get 0
      i32.load offset=20
      local.get 1
      local.get 0
      i32.const 24
      i32.add
      i32.load
      i32.load offset=16
      call_indirect (type 6)
    )
    (func $<bool as core::fmt::Display>::fmt (;272;) (type 6) (param i32 i32) (result i32)
      block ;; label = @1
        local.get 0
        i32.load8_u
        br_if 0 (;@1;)
        local.get 1
        i32.const 1053060
        i32.const 5
        call $core::fmt::Formatter::pad
        return
      end
      local.get 1
      i32.const 1053065
      i32.const 4
      call $core::fmt::Formatter::pad
    )
    (func $<str as core::fmt::Debug>::fmt (;273;) (type 5) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i32)
      global.get $__stack_pointer
      i32.const 32
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      i32.const 1
      local.set 4
      block ;; label = @1
        block ;; label = @2
          local.get 2
          i32.load offset=20
          local.tee 5
          i32.const 34
          local.get 2
          i32.const 24
          i32.add
          i32.load
          local.tee 6
          i32.load offset=16
          local.tee 7
          call_indirect (type 6)
          br_if 0 (;@2;)
          block ;; label = @3
            block ;; label = @4
              local.get 1
              br_if 0 (;@4;)
              i32.const 0
              local.set 2
              i32.const 0
              local.set 1
              br 1 (;@3;)
            end
            local.get 0
            local.get 1
            i32.add
            local.set 8
            i32.const 0
            local.set 2
            local.get 0
            local.set 9
            i32.const 0
            local.set 10
            block ;; label = @4
              block ;; label = @5
                loop ;; label = @6
                  block ;; label = @7
                    block ;; label = @8
                      local.get 9
                      local.tee 11
                      i32.load8_s
                      local.tee 12
                      i32.const -1
                      i32.le_s
                      br_if 0 (;@8;)
                      local.get 11
                      i32.const 1
                      i32.add
                      local.set 9
                      local.get 12
                      i32.const 255
                      i32.and
                      local.set 13
                      br 1 (;@7;)
                    end
                    local.get 11
                    i32.load8_u offset=1
                    i32.const 63
                    i32.and
                    local.set 14
                    local.get 12
                    i32.const 31
                    i32.and
                    local.set 15
                    block ;; label = @8
                      local.get 12
                      i32.const -33
                      i32.gt_u
                      br_if 0 (;@8;)
                      local.get 15
                      i32.const 6
                      i32.shl
                      local.get 14
                      i32.or
                      local.set 13
                      local.get 11
                      i32.const 2
                      i32.add
                      local.set 9
                      br 1 (;@7;)
                    end
                    local.get 14
                    i32.const 6
                    i32.shl
                    local.get 11
                    i32.load8_u offset=2
                    i32.const 63
                    i32.and
                    i32.or
                    local.set 14
                    local.get 11
                    i32.const 3
                    i32.add
                    local.set 9
                    block ;; label = @8
                      local.get 12
                      i32.const -16
                      i32.ge_u
                      br_if 0 (;@8;)
                      local.get 14
                      local.get 15
                      i32.const 12
                      i32.shl
                      i32.or
                      local.set 13
                      br 1 (;@7;)
                    end
                    local.get 14
                    i32.const 6
                    i32.shl
                    local.get 9
                    i32.load8_u
                    i32.const 63
                    i32.and
                    i32.or
                    local.get 15
                    i32.const 18
                    i32.shl
                    i32.const 1835008
                    i32.and
                    i32.or
                    local.tee 13
                    i32.const 1114112
                    i32.eq
                    br_if 3 (;@4;)
                    local.get 11
                    i32.const 4
                    i32.add
                    local.set 9
                  end
                  local.get 3
                  i32.const 4
                  i32.add
                  local.get 13
                  i32.const 65537
                  call $core::char::methods::<impl char>::escape_debug_ext
                  block ;; label = @7
                    block ;; label = @8
                      local.get 3
                      i32.load8_u offset=4
                      i32.const 128
                      i32.eq
                      br_if 0 (;@8;)
                      local.get 3
                      i32.load8_u offset=15
                      local.get 3
                      i32.load8_u offset=14
                      i32.sub
                      i32.const 255
                      i32.and
                      i32.const 1
                      i32.eq
                      br_if 0 (;@8;)
                      local.get 10
                      local.get 2
                      i32.lt_u
                      br_if 3 (;@5;)
                      block ;; label = @9
                        local.get 2
                        i32.eqz
                        br_if 0 (;@9;)
                        block ;; label = @10
                          local.get 2
                          local.get 1
                          i32.lt_u
                          br_if 0 (;@10;)
                          local.get 2
                          local.get 1
                          i32.eq
                          br_if 1 (;@9;)
                          br 5 (;@5;)
                        end
                        local.get 0
                        local.get 2
                        i32.add
                        i32.load8_s
                        i32.const -64
                        i32.lt_s
                        br_if 4 (;@5;)
                      end
                      block ;; label = @9
                        local.get 10
                        i32.eqz
                        br_if 0 (;@9;)
                        block ;; label = @10
                          local.get 10
                          local.get 1
                          i32.lt_u
                          br_if 0 (;@10;)
                          local.get 10
                          local.get 1
                          i32.eq
                          br_if 1 (;@9;)
                          br 5 (;@5;)
                        end
                        local.get 0
                        local.get 10
                        i32.add
                        i32.load8_s
                        i32.const -65
                        i32.le_s
                        br_if 4 (;@5;)
                      end
                      block ;; label = @9
                        block ;; label = @10
                          local.get 5
                          local.get 0
                          local.get 2
                          i32.add
                          local.get 10
                          local.get 2
                          i32.sub
                          local.get 6
                          i32.load offset=12
                          call_indirect (type 5)
                          br_if 0 (;@10;)
                          local.get 3
                          i32.const 16
                          i32.add
                          i32.const 8
                          i32.add
                          local.tee 15
                          local.get 3
                          i32.const 4
                          i32.add
                          i32.const 8
                          i32.add
                          i32.load
                          i32.store
                          local.get 3
                          local.get 3
                          i64.load offset=4 align=4
                          local.tee 16
                          i64.store offset=16
                          block ;; label = @11
                            local.get 16
                            i32.wrap_i64
                            i32.const 255
                            i32.and
                            i32.const 128
                            i32.ne
                            br_if 0 (;@11;)
                            i32.const 128
                            local.set 14
                            loop ;; label = @12
                              block ;; label = @13
                                block ;; label = @14
                                  local.get 14
                                  i32.const 255
                                  i32.and
                                  i32.const 128
                                  i32.eq
                                  br_if 0 (;@14;)
                                  local.get 3
                                  i32.load8_u offset=26
                                  local.tee 12
                                  local.get 3
                                  i32.load8_u offset=27
                                  i32.ge_u
                                  br_if 5 (;@9;)
                                  local.get 3
                                  local.get 12
                                  i32.const 1
                                  i32.add
                                  i32.store8 offset=26
                                  local.get 12
                                  i32.const 10
                                  i32.ge_u
                                  br_if 7 (;@7;)
                                  local.get 3
                                  i32.const 16
                                  i32.add
                                  local.get 12
                                  i32.add
                                  i32.load8_u
                                  local.set 2
                                  br 1 (;@13;)
                                end
                                i32.const 0
                                local.set 14
                                local.get 15
                                i32.const 0
                                i32.store
                                local.get 3
                                i32.load offset=20
                                local.set 2
                                local.get 3
                                i64.const 0
                                i64.store offset=16
                              end
                              local.get 5
                              local.get 2
                              local.get 7
                              call_indirect (type 6)
                              i32.eqz
                              br_if 0 (;@12;)
                              br 2 (;@10;)
                            end
                          end
                          local.get 3
                          i32.load8_u offset=26
                          local.tee 2
                          i32.const 10
                          local.get 2
                          i32.const 10
                          i32.gt_u
                          select
                          local.set 12
                          local.get 3
                          i32.load8_u offset=27
                          local.tee 14
                          local.get 2
                          local.get 14
                          local.get 2
                          i32.gt_u
                          select
                          local.set 17
                          loop ;; label = @11
                            local.get 17
                            local.get 2
                            i32.eq
                            br_if 2 (;@9;)
                            local.get 3
                            local.get 2
                            i32.const 1
                            i32.add
                            local.tee 14
                            i32.store8 offset=26
                            local.get 12
                            local.get 2
                            i32.eq
                            br_if 4 (;@7;)
                            local.get 3
                            i32.const 16
                            i32.add
                            local.get 2
                            i32.add
                            local.set 15
                            local.get 14
                            local.set 2
                            local.get 5
                            local.get 15
                            i32.load8_u
                            local.get 7
                            call_indirect (type 6)
                            i32.eqz
                            br_if 0 (;@11;)
                          end
                        end
                        i32.const 1
                        local.set 4
                        br 7 (;@2;)
                      end
                      i32.const 1
                      local.set 2
                      block ;; label = @9
                        local.get 13
                        i32.const 128
                        i32.lt_u
                        br_if 0 (;@9;)
                        i32.const 2
                        local.set 2
                        local.get 13
                        i32.const 2048
                        i32.lt_u
                        br_if 0 (;@9;)
                        i32.const 3
                        i32.const 4
                        local.get 13
                        i32.const 65536
                        i32.lt_u
                        select
                        local.set 2
                      end
                      local.get 2
                      local.get 10
                      i32.add
                      local.set 2
                    end
                    local.get 10
                    local.get 11
                    i32.sub
                    local.get 9
                    i32.add
                    local.set 10
                    local.get 9
                    local.get 8
                    i32.ne
                    br_if 1 (;@6;)
                    br 3 (;@4;)
                  end
                end
                local.get 12
                i32.const 10
                i32.const 1055396
                call $core::panicking::panic_bounds_check
                unreachable
              end
              local.get 0
              local.get 1
              local.get 2
              local.get 10
              i32.const 1053088
              call $core::str::slice_error_fail
              unreachable
            end
            block ;; label = @4
              local.get 2
              br_if 0 (;@4;)
              i32.const 0
              local.set 2
              br 1 (;@3;)
            end
            block ;; label = @4
              block ;; label = @5
                local.get 1
                local.get 2
                i32.gt_u
                br_if 0 (;@5;)
                local.get 1
                local.get 2
                i32.eq
                br_if 1 (;@4;)
                br 4 (;@1;)
              end
              local.get 0
              local.get 2
              i32.add
              i32.load8_s
              i32.const -65
              i32.le_s
              br_if 3 (;@1;)
            end
            local.get 1
            local.get 2
            i32.sub
            local.set 1
          end
          local.get 5
          local.get 0
          local.get 2
          i32.add
          local.get 1
          local.get 6
          i32.load offset=12
          call_indirect (type 5)
          br_if 0 (;@2;)
          local.get 5
          i32.const 34
          local.get 7
          call_indirect (type 6)
          local.set 4
        end
        local.get 3
        i32.const 32
        i32.add
        global.set $__stack_pointer
        local.get 4
        return
      end
      local.get 0
      local.get 1
      local.get 2
      local.get 1
      i32.const 1053072
      call $core::str::slice_error_fail
      unreachable
    )
    (func $core::str::slice_error_fail (;274;) (type 14) (param i32 i32 i32 i32 i32)
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      local.get 4
      call $core::str::slice_error_fail_rt
      unreachable
    )
    (func $<str as core::fmt::Display>::fmt (;275;) (type 5) (param i32 i32 i32) (result i32)
      local.get 2
      local.get 0
      local.get 1
      call $core::fmt::Formatter::pad
    )
    (func $<char as core::fmt::Debug>::fmt (;276;) (type 6) (param i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      i32.const 1
      local.set 3
      block ;; label = @1
        block ;; label = @2
          local.get 1
          i32.load offset=20
          local.tee 4
          i32.const 39
          local.get 1
          i32.const 24
          i32.add
          i32.load
          i32.load offset=16
          local.tee 5
          call_indirect (type 6)
          br_if 0 (;@2;)
          local.get 2
          local.get 0
          i32.load
          i32.const 257
          call $core::char::methods::<impl char>::escape_debug_ext
          block ;; label = @3
            block ;; label = @4
              local.get 2
              i32.load8_u
              i32.const 128
              i32.ne
              br_if 0 (;@4;)
              local.get 2
              i32.const 8
              i32.add
              local.set 6
              i32.const 128
              local.set 7
              loop ;; label = @5
                block ;; label = @6
                  block ;; label = @7
                    local.get 7
                    i32.const 255
                    i32.and
                    i32.const 128
                    i32.eq
                    br_if 0 (;@7;)
                    local.get 2
                    i32.load8_u offset=10
                    local.tee 0
                    local.get 2
                    i32.load8_u offset=11
                    i32.ge_u
                    br_if 4 (;@3;)
                    local.get 2
                    local.get 0
                    i32.const 1
                    i32.add
                    i32.store8 offset=10
                    local.get 0
                    i32.const 10
                    i32.ge_u
                    br_if 6 (;@1;)
                    local.get 2
                    local.get 0
                    i32.add
                    i32.load8_u
                    local.set 1
                    br 1 (;@6;)
                  end
                  i32.const 0
                  local.set 7
                  local.get 6
                  i32.const 0
                  i32.store
                  local.get 2
                  i32.load offset=4
                  local.set 1
                  local.get 2
                  i64.const 0
                  i64.store
                end
                local.get 4
                local.get 1
                local.get 5
                call_indirect (type 6)
                i32.eqz
                br_if 0 (;@5;)
                br 3 (;@2;)
              end
            end
            local.get 2
            i32.load8_u offset=10
            local.tee 1
            i32.const 10
            local.get 1
            i32.const 10
            i32.gt_u
            select
            local.set 0
            local.get 2
            i32.load8_u offset=11
            local.tee 7
            local.get 1
            local.get 7
            local.get 1
            i32.gt_u
            select
            local.set 8
            loop ;; label = @4
              local.get 8
              local.get 1
              i32.eq
              br_if 1 (;@3;)
              local.get 2
              local.get 1
              i32.const 1
              i32.add
              local.tee 7
              i32.store8 offset=10
              local.get 0
              local.get 1
              i32.eq
              br_if 3 (;@1;)
              local.get 2
              local.get 1
              i32.add
              local.set 6
              local.get 7
              local.set 1
              local.get 4
              local.get 6
              i32.load8_u
              local.get 5
              call_indirect (type 6)
              i32.eqz
              br_if 0 (;@4;)
              br 2 (;@2;)
            end
          end
          local.get 4
          i32.const 39
          local.get 5
          call_indirect (type 6)
          local.set 3
        end
        local.get 2
        i32.const 16
        i32.add
        global.set $__stack_pointer
        local.get 3
        return
      end
      local.get 0
      i32.const 10
      i32.const 1055396
      call $core::panicking::panic_bounds_check
      unreachable
    )
    (func $core::slice::memchr::memchr_aligned (;277;) (type 12) (param i32 i32 i32 i32)
      (local i32 i32 i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 2
              i32.const 3
              i32.add
              i32.const -4
              i32.and
              local.tee 4
              local.get 2
              i32.eq
              br_if 0 (;@4;)
              local.get 4
              local.get 2
              i32.sub
              local.tee 4
              local.get 3
              local.get 4
              local.get 3
              i32.lt_u
              select
              local.tee 4
              i32.eqz
              br_if 0 (;@4;)
              i32.const 0
              local.set 5
              local.get 1
              i32.const 255
              i32.and
              local.set 6
              i32.const 1
              local.set 7
              loop ;; label = @5
                local.get 2
                local.get 5
                i32.add
                i32.load8_u
                local.get 6
                i32.eq
                br_if 4 (;@1;)
                local.get 4
                local.get 5
                i32.const 1
                i32.add
                local.tee 5
                i32.ne
                br_if 0 (;@5;)
              end
              local.get 4
              local.get 3
              i32.const -8
              i32.add
              local.tee 8
              i32.gt_u
              br_if 2 (;@2;)
              br 1 (;@3;)
            end
            local.get 3
            i32.const -8
            i32.add
            local.set 8
            i32.const 0
            local.set 4
          end
          local.get 1
          i32.const 255
          i32.and
          i32.const 16843009
          i32.mul
          local.set 5
          loop ;; label = @3
            local.get 2
            local.get 4
            i32.add
            local.tee 7
            i32.load
            local.get 5
            i32.xor
            local.tee 6
            i32.const -1
            i32.xor
            local.get 6
            i32.const -16843009
            i32.add
            i32.and
            i32.const -2139062144
            i32.and
            br_if 1 (;@2;)
            local.get 7
            i32.const 4
            i32.add
            i32.load
            local.get 5
            i32.xor
            local.tee 6
            i32.const -1
            i32.xor
            local.get 6
            i32.const -16843009
            i32.add
            i32.and
            i32.const -2139062144
            i32.and
            br_if 1 (;@2;)
            local.get 4
            i32.const 8
            i32.add
            local.tee 4
            local.get 8
            i32.le_u
            br_if 0 (;@3;)
          end
        end
        i32.const 0
        local.set 7
        block ;; label = @2
          local.get 4
          local.get 3
          i32.eq
          br_if 0 (;@2;)
          local.get 1
          i32.const 255
          i32.and
          local.set 5
          loop ;; label = @3
            block ;; label = @4
              local.get 2
              local.get 4
              i32.add
              i32.load8_u
              local.get 5
              i32.ne
              br_if 0 (;@4;)
              local.get 4
              local.set 5
              i32.const 1
              local.set 7
              br 3 (;@1;)
            end
            local.get 3
            local.get 4
            i32.const 1
            i32.add
            local.tee 4
            i32.ne
            br_if 0 (;@3;)
          end
        end
        local.get 3
        local.set 5
      end
      local.get 0
      local.get 5
      i32.store offset=4
      local.get 0
      local.get 7
      i32.store
    )
    (func $core::str::lossy::Utf8Chunk::valid (;278;) (type 2) (param i32 i32)
      local.get 0
      local.get 1
      i64.load align=4
      i64.store
    )
    (func $core::str::lossy::Utf8Chunk::invalid (;279;) (type 2) (param i32 i32)
      local.get 0
      local.get 1
      i64.load offset=8 align=4
      i64.store
    )
    (func $<core::str::lossy::Utf8Chunks as core::iter::traits::iterator::Iterator>::next (;280;) (type 2) (param i32 i32)
      (local i32 i32 i32 i32 i32 i32 i32)
      block ;; label = @1
        local.get 1
        i32.load offset=4
        local.tee 2
        i32.eqz
        br_if 0 (;@1;)
        local.get 1
        i32.load
        local.set 3
        i32.const 0
        local.set 4
        block ;; label = @2
          loop ;; label = @3
            local.get 4
            i32.const 1
            i32.add
            local.set 5
            block ;; label = @4
              block ;; label = @5
                local.get 3
                local.get 4
                i32.add
                i32.load8_u
                local.tee 6
                i32.extend8_s
                local.tee 7
                i32.const 0
                i32.lt_s
                br_if 0 (;@5;)
                local.get 5
                local.set 4
                br 1 (;@4;)
              end
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
                                    local.get 6
                                    i32.const 1053256
                                    i32.add
                                    i32.load8_u
                                    i32.const -2
                                    i32.add
                                    br_table 0 (;@15;) 1 (;@14;) 2 (;@13;) 13 (;@2;)
                                  end
                                  local.get 3
                                  local.get 5
                                  i32.add
                                  i32.const 1053512
                                  local.get 5
                                  local.get 2
                                  i32.lt_u
                                  select
                                  i32.load8_u
                                  i32.const 192
                                  i32.and
                                  i32.const 128
                                  i32.ne
                                  br_if 12 (;@2;)
                                  local.get 4
                                  i32.const 2
                                  i32.add
                                  local.set 4
                                  br 10 (;@4;)
                                end
                                local.get 3
                                local.get 5
                                i32.add
                                i32.const 1053512
                                local.get 5
                                local.get 2
                                i32.lt_u
                                select
                                i32.load8_s
                                local.set 8
                                local.get 6
                                i32.const -224
                                i32.add
                                br_table 1 (;@12;) 3 (;@10;) 3 (;@10;) 3 (;@10;) 3 (;@10;) 3 (;@10;) 3 (;@10;) 3 (;@10;) 3 (;@10;) 3 (;@10;) 3 (;@10;) 3 (;@10;) 3 (;@10;) 2 (;@11;) 3 (;@10;)
                              end
                              local.get 3
                              local.get 5
                              i32.add
                              i32.const 1053512
                              local.get 5
                              local.get 2
                              i32.lt_u
                              select
                              i32.load8_s
                              local.set 8
                              local.get 6
                              i32.const -240
                              i32.add
                              br_table 4 (;@8;) 3 (;@9;) 3 (;@9;) 3 (;@9;) 5 (;@7;) 3 (;@9;)
                            end
                            local.get 8
                            i32.const -32
                            i32.and
                            i32.const -96
                            i32.ne
                            br_if 9 (;@2;)
                            br 6 (;@5;)
                          end
                          local.get 8
                          i32.const -97
                          i32.gt_s
                          br_if 8 (;@2;)
                          br 5 (;@5;)
                        end
                        block ;; label = @10
                          local.get 7
                          i32.const 31
                          i32.add
                          i32.const 255
                          i32.and
                          i32.const 12
                          i32.lt_u
                          br_if 0 (;@10;)
                          local.get 7
                          i32.const -2
                          i32.and
                          i32.const -18
                          i32.ne
                          br_if 8 (;@2;)
                          local.get 8
                          i32.const -64
                          i32.ge_s
                          br_if 8 (;@2;)
                          br 5 (;@5;)
                        end
                        local.get 8
                        i32.const -64
                        i32.ge_s
                        br_if 7 (;@2;)
                        br 4 (;@5;)
                      end
                      local.get 7
                      i32.const 15
                      i32.add
                      i32.const 255
                      i32.and
                      i32.const 2
                      i32.gt_u
                      br_if 6 (;@2;)
                      local.get 8
                      i32.const -64
                      i32.ge_s
                      br_if 6 (;@2;)
                      br 2 (;@6;)
                    end
                    local.get 8
                    i32.const 112
                    i32.add
                    i32.const 255
                    i32.and
                    i32.const 48
                    i32.ge_u
                    br_if 5 (;@2;)
                    br 1 (;@6;)
                  end
                  local.get 8
                  i32.const -113
                  i32.gt_s
                  br_if 4 (;@2;)
                end
                local.get 3
                local.get 4
                i32.const 2
                i32.add
                local.tee 5
                i32.add
                i32.const 1053512
                local.get 5
                local.get 2
                i32.lt_u
                select
                i32.load8_u
                i32.const 192
                i32.and
                i32.const 128
                i32.ne
                br_if 3 (;@2;)
                local.get 3
                local.get 4
                i32.const 3
                i32.add
                local.tee 5
                i32.add
                i32.const 1053512
                local.get 5
                local.get 2
                i32.lt_u
                select
                i32.load8_u
                i32.const 192
                i32.and
                i32.const 128
                i32.ne
                br_if 3 (;@2;)
                local.get 4
                i32.const 4
                i32.add
                local.set 4
                br 1 (;@4;)
              end
              local.get 3
              local.get 4
              i32.const 2
              i32.add
              local.tee 5
              i32.add
              i32.const 1053512
              local.get 5
              local.get 2
              i32.lt_u
              select
              i32.load8_u
              i32.const 192
              i32.and
              i32.const 128
              i32.ne
              br_if 2 (;@2;)
              local.get 4
              i32.const 3
              i32.add
              local.set 4
            end
            local.get 4
            local.set 5
            local.get 4
            local.get 2
            i32.lt_u
            br_if 0 (;@3;)
          end
        end
        local.get 0
        local.get 4
        i32.store offset=4
        local.get 0
        local.get 3
        i32.store
        local.get 1
        local.get 2
        local.get 5
        i32.sub
        i32.store offset=4
        local.get 1
        local.get 3
        local.get 5
        i32.add
        i32.store
        local.get 0
        local.get 5
        local.get 4
        i32.sub
        i32.store offset=12
        local.get 0
        local.get 3
        local.get 4
        i32.add
        i32.store offset=8
        return
      end
      local.get 0
      i32.const 0
      i32.store
    )
    (func $core::fmt::num::<impl core::fmt::UpperHex for i8>::fmt (;281;) (type 6) (param i32 i32) (result i32)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 128
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 0
      i32.load8_u
      local.set 3
      i32.const 0
      local.set 0
      loop ;; label = @1
        local.get 2
        local.get 0
        i32.add
        i32.const 127
        i32.add
        i32.const 48
        i32.const 55
        local.get 3
        i32.const 15
        i32.and
        local.tee 4
        i32.const 10
        i32.lt_u
        select
        local.get 4
        i32.add
        i32.store8
        local.get 0
        i32.const -1
        i32.add
        local.set 0
        local.get 3
        i32.const 255
        i32.and
        local.tee 4
        i32.const 4
        i32.shr_u
        local.set 3
        local.get 4
        i32.const 16
        i32.ge_u
        br_if 0 (;@1;)
      end
      block ;; label = @1
        local.get 0
        i32.const 128
        i32.add
        local.tee 3
        i32.const 128
        i32.le_u
        br_if 0 (;@1;)
        local.get 3
        i32.const 128
        i32.const 1052804
        call $core::slice::index::slice_start_index_len_fail
        unreachable
      end
      local.get 1
      i32.const 1
      i32.const 1052774
      i32.const 2
      local.get 2
      local.get 0
      i32.add
      i32.const 128
      i32.add
      i32.const 0
      local.get 0
      i32.sub
      call $core::fmt::Formatter::pad_integral
      local.set 0
      local.get 2
      i32.const 128
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $core::str::lossy::Utf8Chunks::new (;282;) (type 4) (param i32 i32 i32)
      local.get 0
      local.get 2
      i32.store offset=4
      local.get 0
      local.get 1
      i32.store
    )
    (func $core::str::slice_error_fail_rt (;283;) (type 14) (param i32 i32 i32 i32 i32)
      (local i32 i32 i32 i32 i32)
      global.get $__stack_pointer
      i32.const 112
      i32.sub
      local.tee 5
      global.set $__stack_pointer
      local.get 5
      local.get 3
      i32.store offset=12
      local.get 5
      local.get 2
      i32.store offset=8
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 1
            i32.const 257
            i32.lt_u
            br_if 0 (;@3;)
            i32.const 256
            local.set 6
            block ;; label = @4
              local.get 0
              i32.load8_s offset=256
              i32.const -65
              i32.gt_s
              br_if 0 (;@4;)
              i32.const 255
              local.set 6
              local.get 0
              i32.load8_s offset=255
              i32.const -65
              i32.gt_s
              br_if 0 (;@4;)
              i32.const 254
              local.set 6
              local.get 0
              i32.load8_s offset=254
              i32.const -65
              i32.gt_s
              br_if 0 (;@4;)
              i32.const 253
              local.set 6
            end
            local.get 0
            local.get 6
            i32.add
            i32.load8_s
            i32.const -65
            i32.le_s
            br_if 1 (;@2;)
            local.get 5
            local.get 6
            i32.store offset=20
            local.get 5
            local.get 0
            i32.store offset=16
            i32.const 5
            local.set 6
            i32.const 1053513
            local.set 7
            br 2 (;@1;)
          end
          local.get 5
          local.get 1
          i32.store offset=20
          local.get 5
          local.get 0
          i32.store offset=16
          i32.const 0
          local.set 6
          i32.const 1052300
          local.set 7
          br 1 (;@1;)
        end
        local.get 0
        local.get 1
        i32.const 0
        local.get 6
        local.get 4
        call $core::str::slice_error_fail
        unreachable
      end
      local.get 5
      local.get 6
      i32.store offset=28
      local.get 5
      local.get 7
      i32.store offset=24
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                local.get 2
                local.get 1
                i32.gt_u
                local.tee 6
                br_if 0 (;@5;)
                local.get 3
                local.get 1
                i32.gt_u
                br_if 0 (;@5;)
                local.get 2
                local.get 3
                i32.gt_u
                br_if 2 (;@3;)
                block ;; label = @6
                  block ;; label = @7
                    local.get 2
                    i32.eqz
                    br_if 0 (;@7;)
                    local.get 2
                    local.get 1
                    i32.ge_u
                    br_if 0 (;@7;)
                    local.get 0
                    local.get 2
                    i32.add
                    i32.load8_s
                    i32.const -64
                    i32.lt_s
                    br_if 1 (;@6;)
                  end
                  local.get 3
                  local.set 2
                end
                local.get 5
                local.get 2
                i32.store offset=32
                local.get 1
                local.set 3
                block ;; label = @6
                  local.get 2
                  local.get 1
                  i32.ge_u
                  br_if 0 (;@6;)
                  i32.const 0
                  local.get 2
                  i32.const -3
                  i32.add
                  local.tee 3
                  local.get 3
                  local.get 2
                  i32.gt_u
                  select
                  local.tee 3
                  local.get 2
                  i32.const 1
                  i32.add
                  local.tee 6
                  i32.gt_u
                  br_if 2 (;@4;)
                  block ;; label = @7
                    local.get 3
                    local.get 6
                    i32.eq
                    br_if 0 (;@7;)
                    local.get 0
                    local.get 6
                    i32.add
                    local.get 0
                    local.get 3
                    i32.add
                    local.tee 8
                    i32.sub
                    local.set 6
                    block ;; label = @8
                      local.get 0
                      local.get 2
                      i32.add
                      local.tee 9
                      i32.load8_s
                      i32.const -65
                      i32.le_s
                      br_if 0 (;@8;)
                      local.get 6
                      i32.const -1
                      i32.add
                      local.set 7
                      br 1 (;@7;)
                    end
                    local.get 3
                    local.get 2
                    i32.eq
                    br_if 0 (;@7;)
                    block ;; label = @8
                      local.get 9
                      i32.const -1
                      i32.add
                      local.tee 2
                      i32.load8_s
                      i32.const -65
                      i32.le_s
                      br_if 0 (;@8;)
                      local.get 6
                      i32.const -2
                      i32.add
                      local.set 7
                      br 1 (;@7;)
                    end
                    local.get 8
                    local.get 2
                    i32.eq
                    br_if 0 (;@7;)
                    block ;; label = @8
                      local.get 9
                      i32.const -2
                      i32.add
                      local.tee 2
                      i32.load8_s
                      i32.const -65
                      i32.le_s
                      br_if 0 (;@8;)
                      local.get 6
                      i32.const -3
                      i32.add
                      local.set 7
                      br 1 (;@7;)
                    end
                    local.get 8
                    local.get 2
                    i32.eq
                    br_if 0 (;@7;)
                    block ;; label = @8
                      local.get 9
                      i32.const -3
                      i32.add
                      local.tee 2
                      i32.load8_s
                      i32.const -65
                      i32.le_s
                      br_if 0 (;@8;)
                      local.get 6
                      i32.const -4
                      i32.add
                      local.set 7
                      br 1 (;@7;)
                    end
                    local.get 8
                    local.get 2
                    i32.eq
                    br_if 0 (;@7;)
                    local.get 6
                    i32.const -5
                    i32.add
                    local.set 7
                  end
                  local.get 7
                  local.get 3
                  i32.add
                  local.set 3
                end
                local.get 3
                i32.eqz
                br_if 4 (;@1;)
                block ;; label = @6
                  block ;; label = @7
                    local.get 1
                    local.get 3
                    i32.gt_u
                    br_if 0 (;@7;)
                    local.get 1
                    local.get 3
                    i32.ne
                    br_if 1 (;@6;)
                    br 5 (;@2;)
                  end
                  local.get 0
                  local.get 3
                  i32.add
                  i32.load8_s
                  i32.const -65
                  i32.gt_s
                  br_if 4 (;@2;)
                end
                local.get 0
                local.get 1
                local.get 3
                local.get 1
                local.get 4
                call $core::str::slice_error_fail
                unreachable
              end
              local.get 5
              local.get 2
              local.get 3
              local.get 6
              select
              i32.store offset=40
              local.get 5
              i32.const 92
              i32.add
              i32.const 95
              i32.store
              local.get 5
              i32.const 84
              i32.add
              i32.const 95
              i32.store
              local.get 5
              i32.const 9
              i32.store offset=76
              local.get 5
              local.get 5
              i32.const 24
              i32.add
              i32.store offset=88
              local.get 5
              local.get 5
              i32.const 16
              i32.add
              i32.store offset=80
              local.get 5
              local.get 5
              i32.const 40
              i32.add
              i32.store offset=72
              local.get 5
              i32.const 48
              i32.add
              i32.const 1053716
              i32.const 3
              local.get 5
              i32.const 72
              i32.add
              i32.const 3
              call $#func223<core::fmt::Arguments::new_v1>
              local.get 5
              i32.const 48
              i32.add
              local.get 4
              call $core::panicking::panic_fmt
              unreachable
            end
            local.get 3
            local.get 6
            i32.const 1053768
            call $core::slice::index::slice_index_order_fail
            unreachable
          end
          local.get 5
          i32.const 100
          i32.add
          i32.const 95
          i32.store
          local.get 5
          i32.const 92
          i32.add
          i32.const 95
          i32.store
          local.get 5
          i32.const 84
          i32.add
          i32.const 9
          i32.store
          local.get 5
          i32.const 9
          i32.store offset=76
          local.get 5
          local.get 5
          i32.const 24
          i32.add
          i32.store offset=96
          local.get 5
          local.get 5
          i32.const 16
          i32.add
          i32.store offset=88
          local.get 5
          local.get 5
          i32.const 12
          i32.add
          i32.store offset=80
          local.get 5
          local.get 5
          i32.const 8
          i32.add
          i32.store offset=72
          local.get 5
          i32.const 48
          i32.add
          i32.const 1053660
          i32.const 4
          local.get 5
          i32.const 72
          i32.add
          i32.const 4
          call $#func223<core::fmt::Arguments::new_v1>
          local.get 5
          i32.const 48
          i32.add
          local.get 4
          call $core::panicking::panic_fmt
          unreachable
        end
        local.get 1
        local.get 3
        i32.sub
        local.set 1
      end
      block ;; label = @1
        local.get 1
        i32.eqz
        br_if 0 (;@1;)
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                local.get 0
                local.get 3
                i32.add
                local.tee 2
                i32.load8_s
                local.tee 1
                i32.const -1
                i32.gt_s
                br_if 0 (;@5;)
                local.get 2
                i32.load8_u offset=1
                i32.const 63
                i32.and
                local.set 0
                local.get 1
                i32.const 31
                i32.and
                local.set 6
                local.get 1
                i32.const -33
                i32.gt_u
                br_if 1 (;@4;)
                local.get 6
                i32.const 6
                i32.shl
                local.get 0
                i32.or
                local.set 2
                br 2 (;@3;)
              end
              local.get 5
              local.get 1
              i32.const 255
              i32.and
              i32.store offset=36
              i32.const 1
              local.set 1
              br 2 (;@2;)
            end
            local.get 0
            i32.const 6
            i32.shl
            local.get 2
            i32.load8_u offset=2
            i32.const 63
            i32.and
            i32.or
            local.set 0
            block ;; label = @4
              local.get 1
              i32.const -16
              i32.ge_u
              br_if 0 (;@4;)
              local.get 0
              local.get 6
              i32.const 12
              i32.shl
              i32.or
              local.set 2
              br 1 (;@3;)
            end
            local.get 0
            i32.const 6
            i32.shl
            local.get 2
            i32.load8_u offset=3
            i32.const 63
            i32.and
            i32.or
            local.get 6
            i32.const 18
            i32.shl
            i32.const 1835008
            i32.and
            i32.or
            local.tee 2
            i32.const 1114112
            i32.eq
            br_if 2 (;@1;)
          end
          local.get 5
          local.get 2
          i32.store offset=36
          i32.const 1
          local.set 1
          local.get 2
          i32.const 128
          i32.lt_u
          br_if 0 (;@2;)
          i32.const 2
          local.set 1
          local.get 2
          i32.const 2048
          i32.lt_u
          br_if 0 (;@2;)
          i32.const 3
          i32.const 4
          local.get 2
          i32.const 65536
          i32.lt_u
          select
          local.set 1
        end
        local.get 5
        local.get 3
        i32.store offset=40
        local.get 5
        local.get 1
        local.get 3
        i32.add
        i32.store offset=44
        local.get 5
        i32.const 108
        i32.add
        i32.const 95
        i32.store
        local.get 5
        i32.const 100
        i32.add
        i32.const 95
        i32.store
        local.get 5
        i32.const 92
        i32.add
        i32.const 97
        i32.store
        local.get 5
        i32.const 84
        i32.add
        i32.const 98
        i32.store
        local.get 5
        i32.const 9
        i32.store offset=76
        local.get 5
        local.get 5
        i32.const 24
        i32.add
        i32.store offset=104
        local.get 5
        local.get 5
        i32.const 16
        i32.add
        i32.store offset=96
        local.get 5
        local.get 5
        i32.const 40
        i32.add
        i32.store offset=88
        local.get 5
        local.get 5
        i32.const 36
        i32.add
        i32.store offset=80
        local.get 5
        local.get 5
        i32.const 32
        i32.add
        i32.store offset=72
        local.get 5
        i32.const 48
        i32.add
        i32.const 1053584
        i32.const 5
        local.get 5
        i32.const 72
        i32.add
        i32.const 5
        call $#func223<core::fmt::Arguments::new_v1>
        local.get 5
        i32.const 48
        i32.add
        local.get 4
        call $core::panicking::panic_fmt
        unreachable
      end
      i32.const 1052300
      i32.const 43
      local.get 4
      call $core::panicking::panic
      unreachable
    )
    (func $core::fmt::num::imp::<impl core::fmt::Display for u64>::fmt (;284;) (type 6) (param i32 i32) (result i32)
      local.get 0
      i64.load
      i32.const 1
      local.get 1
      call $core::fmt::num::imp::fmt_u64
    )
    (func $core::unicode::printable::check (;285;) (type 19) (param i32 i32 i32 i32 i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32)
      i32.const 1
      local.set 7
      block ;; label = @1
        block ;; label = @2
          local.get 2
          i32.eqz
          br_if 0 (;@2;)
          local.get 1
          local.get 2
          i32.const 1
          i32.shl
          i32.add
          local.set 8
          local.get 0
          i32.const 65280
          i32.and
          i32.const 8
          i32.shr_u
          local.set 9
          i32.const 0
          local.set 10
          local.get 0
          i32.const 255
          i32.and
          local.set 11
          loop ;; label = @3
            local.get 1
            i32.const 2
            i32.add
            local.set 12
            local.get 10
            local.get 1
            i32.load8_u offset=1
            local.tee 2
            i32.add
            local.set 13
            block ;; label = @4
              local.get 1
              i32.load8_u
              local.tee 1
              local.get 9
              i32.eq
              br_if 0 (;@4;)
              local.get 1
              local.get 9
              i32.gt_u
              br_if 2 (;@2;)
              local.get 13
              local.set 10
              local.get 12
              local.set 1
              local.get 12
              local.get 8
              i32.eq
              br_if 2 (;@2;)
              br 1 (;@3;)
            end
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  local.get 10
                  local.get 13
                  i32.gt_u
                  br_if 0 (;@6;)
                  local.get 13
                  local.get 4
                  i32.gt_u
                  br_if 1 (;@5;)
                  local.get 3
                  local.get 10
                  i32.add
                  local.set 1
                  loop ;; label = @7
                    local.get 2
                    i32.eqz
                    br_if 3 (;@4;)
                    local.get 2
                    i32.const -1
                    i32.add
                    local.set 2
                    local.get 1
                    i32.load8_u
                    local.set 10
                    local.get 1
                    i32.const 1
                    i32.add
                    local.set 1
                    local.get 10
                    local.get 11
                    i32.ne
                    br_if 0 (;@7;)
                  end
                  i32.const 0
                  local.set 7
                  br 5 (;@1;)
                end
                local.get 10
                local.get 13
                i32.const 1053840
                call $core::slice::index::slice_index_order_fail
                unreachable
              end
              local.get 13
              local.get 4
              i32.const 1053840
              call $core::slice::index::slice_end_index_len_fail
              unreachable
            end
            local.get 13
            local.set 10
            local.get 12
            local.set 1
            local.get 12
            local.get 8
            i32.ne
            br_if 0 (;@3;)
          end
        end
        local.get 6
        i32.eqz
        br_if 0 (;@1;)
        local.get 5
        local.get 6
        i32.add
        local.set 11
        local.get 0
        i32.const 65535
        i32.and
        local.set 1
        i32.const 1
        local.set 7
        loop ;; label = @2
          local.get 5
          i32.const 1
          i32.add
          local.set 10
          block ;; label = @3
            block ;; label = @4
              local.get 5
              i32.load8_u
              local.tee 2
              i32.extend8_s
              local.tee 13
              i32.const 0
              i32.lt_s
              br_if 0 (;@4;)
              local.get 10
              local.set 5
              br 1 (;@3;)
            end
            block ;; label = @4
              local.get 10
              local.get 11
              i32.eq
              br_if 0 (;@4;)
              local.get 13
              i32.const 127
              i32.and
              i32.const 8
              i32.shl
              local.get 5
              i32.load8_u offset=1
              i32.or
              local.set 2
              local.get 5
              i32.const 2
              i32.add
              local.set 5
              br 1 (;@3;)
            end
            i32.const 1052300
            i32.const 43
            i32.const 1053824
            call $core::panicking::panic
            unreachable
          end
          local.get 1
          local.get 2
          i32.sub
          local.tee 1
          i32.const 0
          i32.lt_s
          br_if 1 (;@1;)
          local.get 7
          i32.const 1
          i32.xor
          local.set 7
          local.get 5
          local.get 11
          i32.ne
          br_if 0 (;@2;)
        end
      end
      local.get 7
      i32.const 1
      i32.and
    )
    (func $core::fmt::num::<impl core::fmt::LowerHex for i8>::fmt (;286;) (type 6) (param i32 i32) (result i32)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 128
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 0
      i32.load8_u
      local.set 3
      i32.const 0
      local.set 0
      loop ;; label = @1
        local.get 2
        local.get 0
        i32.add
        i32.const 127
        i32.add
        i32.const 48
        i32.const 87
        local.get 3
        i32.const 15
        i32.and
        local.tee 4
        i32.const 10
        i32.lt_u
        select
        local.get 4
        i32.add
        i32.store8
        local.get 0
        i32.const -1
        i32.add
        local.set 0
        local.get 3
        i32.const 255
        i32.and
        local.tee 4
        i32.const 4
        i32.shr_u
        local.set 3
        local.get 4
        i32.const 16
        i32.ge_u
        br_if 0 (;@1;)
      end
      block ;; label = @1
        local.get 0
        i32.const 128
        i32.add
        local.tee 3
        i32.const 128
        i32.le_u
        br_if 0 (;@1;)
        local.get 3
        i32.const 128
        i32.const 1052804
        call $core::slice::index::slice_start_index_len_fail
        unreachable
      end
      local.get 1
      i32.const 1
      i32.const 1052774
      i32.const 2
      local.get 2
      local.get 0
      i32.add
      i32.const 128
      i32.add
      i32.const 0
      local.get 0
      i32.sub
      call $core::fmt::Formatter::pad_integral
      local.set 0
      local.get 2
      i32.const 128
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $core::fmt::num::imp::fmt_u64 (;287;) (type 20) (param i64 i32 i32) (result i32)
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
          i32.const 1052820
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
          i32.const 1052820
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
        i32.const 1052820
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
          i32.const 1052820
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
      i32.const 1052300
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
    (func $core::fmt::num::<impl core::fmt::UpperHex for i32>::fmt (;288;) (type 6) (param i32 i32) (result i32)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 128
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 0
      i32.load
      local.set 0
      i32.const 0
      local.set 3
      loop ;; label = @1
        local.get 2
        local.get 3
        i32.add
        i32.const 127
        i32.add
        i32.const 48
        i32.const 55
        local.get 0
        i32.const 15
        i32.and
        local.tee 4
        i32.const 10
        i32.lt_u
        select
        local.get 4
        i32.add
        i32.store8
        local.get 3
        i32.const -1
        i32.add
        local.set 3
        local.get 0
        i32.const 16
        i32.lt_u
        local.set 4
        local.get 0
        i32.const 4
        i32.shr_u
        local.set 0
        local.get 4
        i32.eqz
        br_if 0 (;@1;)
      end
      block ;; label = @1
        local.get 3
        i32.const 128
        i32.add
        local.tee 0
        i32.const 128
        i32.le_u
        br_if 0 (;@1;)
        local.get 0
        i32.const 128
        i32.const 1052804
        call $core::slice::index::slice_start_index_len_fail
        unreachable
      end
      local.get 1
      i32.const 1
      i32.const 1052774
      i32.const 2
      local.get 2
      local.get 3
      i32.add
      i32.const 128
      i32.add
      i32.const 0
      local.get 3
      i32.sub
      call $core::fmt::Formatter::pad_integral
      local.set 0
      local.get 2
      i32.const 128
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $core::fmt::num::<impl core::fmt::LowerHex for i64>::fmt (;289;) (type 6) (param i32 i32) (result i32)
      (local i32 i64 i32)
      global.get $__stack_pointer
      i32.const 128
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 0
      i64.load
      local.set 3
      i32.const 0
      local.set 0
      loop ;; label = @1
        local.get 2
        local.get 0
        i32.add
        i32.const 127
        i32.add
        i32.const 48
        i32.const 87
        local.get 3
        i32.wrap_i64
        i32.const 15
        i32.and
        local.tee 4
        i32.const 10
        i32.lt_u
        select
        local.get 4
        i32.add
        i32.store8
        local.get 0
        i32.const -1
        i32.add
        local.set 0
        local.get 3
        i64.const 16
        i64.lt_u
        local.set 4
        local.get 3
        i64.const 4
        i64.shr_u
        local.set 3
        local.get 4
        i32.eqz
        br_if 0 (;@1;)
      end
      block ;; label = @1
        local.get 0
        i32.const 128
        i32.add
        local.tee 4
        i32.const 128
        i32.le_u
        br_if 0 (;@1;)
        local.get 4
        i32.const 128
        i32.const 1052804
        call $core::slice::index::slice_start_index_len_fail
        unreachable
      end
      local.get 1
      i32.const 1
      i32.const 1052774
      i32.const 2
      local.get 2
      local.get 0
      i32.add
      i32.const 128
      i32.add
      i32.const 0
      local.get 0
      i32.sub
      call $core::fmt::Formatter::pad_integral
      local.set 0
      local.get 2
      i32.const 128
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $core::fmt::num::<impl core::fmt::UpperHex for i64>::fmt (;290;) (type 6) (param i32 i32) (result i32)
      (local i32 i64 i32)
      global.get $__stack_pointer
      i32.const 128
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      local.get 0
      i64.load
      local.set 3
      i32.const 0
      local.set 0
      loop ;; label = @1
        local.get 2
        local.get 0
        i32.add
        i32.const 127
        i32.add
        i32.const 48
        i32.const 55
        local.get 3
        i32.wrap_i64
        i32.const 15
        i32.and
        local.tee 4
        i32.const 10
        i32.lt_u
        select
        local.get 4
        i32.add
        i32.store8
        local.get 0
        i32.const -1
        i32.add
        local.set 0
        local.get 3
        i64.const 16
        i64.lt_u
        local.set 4
        local.get 3
        i64.const 4
        i64.shr_u
        local.set 3
        local.get 4
        i32.eqz
        br_if 0 (;@1;)
      end
      block ;; label = @1
        local.get 0
        i32.const 128
        i32.add
        local.tee 4
        i32.const 128
        i32.le_u
        br_if 0 (;@1;)
        local.get 4
        i32.const 128
        i32.const 1052804
        call $core::slice::index::slice_start_index_len_fail
        unreachable
      end
      local.get 1
      i32.const 1
      i32.const 1052774
      i32.const 2
      local.get 2
      local.get 0
      i32.add
      i32.const 128
      i32.add
      i32.const 0
      local.get 0
      i32.sub
      call $core::fmt::Formatter::pad_integral
      local.set 0
      local.get 2
      i32.const 128
      i32.add
      global.set $__stack_pointer
      local.get 0
    )
    (func $core::fmt::num::imp::<impl core::fmt::Display for i32>::fmt (;291;) (type 6) (param i32 i32) (result i32)
      local.get 0
      i32.load
      local.tee 0
      i64.extend_i32_u
      i64.const 0
      local.get 0
      i64.extend_i32_s
      i64.sub
      local.get 0
      i32.const -1
      i32.gt_s
      local.tee 0
      select
      local.get 0
      local.get 1
      call $core::fmt::num::imp::fmt_u64
    )
    (func $<core::fmt::Error as core::fmt::Debug>::fmt (;292;) (type 6) (param i32 i32) (result i32)
      local.get 1
      i32.load offset=20
      i32.const 1055412
      i32.const 5
      local.get 1
      i32.const 24
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type 5)
    )
    (func $<core::alloc::layout::LayoutError as core::fmt::Debug>::fmt (;293;) (type 6) (param i32 i32) (result i32)
      local.get 1
      i32.load offset=20
      i32.const 1056279
      i32.const 11
      local.get 1
      i32.const 24
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type 5)
    )
    (table (;0;) 108 108 funcref)
    (memory (;0;) 17)
    (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
    (export "memory" (memory 0))
    (export "_start" (func $_start))
    (export "__main_void" (func $__main_void))
    (export "cabi_realloc" (func $cabi_realloc))
    (elem (;0;) (i32.const 1) func $basic_wallet_p2id_note::main $core::ptr::drop_in_place<&u64> $core::ops::function::FnOnce::call_once{{vtable.shim}} $std::rt::lang_start::{{closure}} $<&T as core::fmt::Debug>::fmt $<core::result::Result<T,E> as core::fmt::Debug>::fmt $<&T as core::fmt::Debug>::fmt $<std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt $core::fmt::num::imp::<impl core::fmt::Display for u32>::fmt $<&T as core::fmt::Display>::fmt $<&T as core::fmt::Display>::fmt $<std::path::Display as core::fmt::Display>::fmt $<core::panic::panic_info::PanicInfo as core::fmt::Display>::fmt $std::alloc::default_alloc_error_hook $core::ptr::drop_in_place<&mut std::io::Write::write_fmt::Adapter<alloc::vec::Vec<u8>>> $<&mut W as core::fmt::Write>::write_str $<&mut W as core::fmt::Write>::write_char $<&mut W as core::fmt::Write>::write_fmt $<&mut W as core::fmt::Write>::write_str $<&mut W as core::fmt::Write>::write_char $<&mut W as core::fmt::Write>::write_fmt $<&mut W as core::fmt::Write>::write_str $<&mut W as core::fmt::Write>::write_char $<&mut W as core::fmt::Write>::write_fmt $<&mut W as core::fmt::Write>::write_str $<&mut W as core::fmt::Write>::write_char $<&mut W as core::fmt::Write>::write_fmt $<&T as core::fmt::Debug>::fmt $<&T as core::fmt::Debug>::fmt $<&T as core::fmt::Debug>::fmt $<&T as core::fmt::Debug>::fmt $core::ptr::drop_in_place<()> $<core::cell::BorrowMutError as core::fmt::Debug>::fmt $core::ptr::drop_in_place<std::io::Write::write_fmt::Adapter<std::fs::File>> $<std::io::Write::write_fmt::Adapter<T> as core::fmt::Write>::write_str $core::fmt::Write::write_char $core::fmt::Write::write_fmt $<std::io::Write::write_fmt::Adapter<T> as core::fmt::Write>::write_str $core::fmt::Write::write_char $core::fmt::Write::write_fmt $<std::io::Write::write_fmt::Adapter<T> as core::fmt::Write>::write_str $core::fmt::Write::write_char $core::fmt::Write::write_fmt $core::ptr::drop_in_place<std::fs::File> $<std::fs::File as std::io::Write>::write $<std::fs::File as std::io::Write>::write_vectored $std::io::impls::<impl std::io::Write for alloc::vec::Vec<u8,A>>::is_write_vectored $std::io::impls::<impl std::io::Write for alloc::vec::Vec<u8,A>>::flush $std::io::Write::write_all $std::io::Write::write_all_vectored $std::io::Write::write_fmt $core::ptr::drop_in_place<alloc::vec::Vec<u8>> $std::io::impls::<impl std::io::Write for alloc::vec::Vec<u8,A>>::write $std::io::impls::<impl std::io::Write for alloc::vec::Vec<u8,A>>::write_vectored $std::io::impls::<impl std::io::Write for alloc::vec::Vec<u8,A>>::write_all $std::io::Write::write_all_vectored $std::io::Write::write_fmt $<std::sys::wasi::stdio::Stderr as std::io::Write>::write $<std::sys::wasi::stdio::Stderr as std::io::Write>::write_vectored $<std::sys::wasi::stdio::Stderr as std::io::Write>::is_write_vectored $<std::sys::wasi::stdio::Stderr as std::io::Write>::flush $std::io::Write::write_all $std::io::Write::write_all_vectored $std::io::Write::write_fmt $core::ptr::drop_in_place<alloc::string::String> $<T as core::any::Any>::type_id $<T as core::any::Any>::type_id $<std::panicking::begin_panic_handler::StrPanicPayload as core::panic::BoxMeUp>::take_box $<std::panicking::begin_panic_handler::StrPanicPayload as core::panic::BoxMeUp>::get $core::ptr::drop_in_place<std::panicking::begin_panic_handler::PanicPayload> $<std::panicking::begin_panic_handler::PanicPayload as core::panic::BoxMeUp>::take_box $<std::panicking::begin_panic_handler::PanicPayload as core::panic::BoxMeUp>::get $<T as core::any::Any>::type_id $core::ptr::drop_in_place<<alloc::boxed::Box<dyn core::error::Error+core::marker::Send+core::marker::Sync> as core::convert::From<alloc::string::String>>::from::StringError> $<<alloc::boxed::Box<dyn core::error::Error+core::marker::Send+core::marker::Sync> as core::convert::From<alloc::string::String>>::from::StringError as core::fmt::Display>::fmt $<<alloc::boxed::Box<dyn core::error::Error+core::marker::Send+core::marker::Sync> as core::convert::From<alloc::string::String>>::from::StringError as core::fmt::Debug>::fmt $core::error::Error::cause $core::error::Error::type_id $<<alloc::boxed::Box<dyn core::error::Error+core::marker::Send+core::marker::Sync> as core::convert::From<alloc::string::String>>::from::StringError as core::error::Error>::description $core::error::Error::provide $__wasilibc_find_relpath_alloc $core::ptr::drop_in_place<usize> $<&mut W as core::fmt::Write>::write_str $<&mut W as core::fmt::Write>::write_char $<&mut W as core::fmt::Write>::write_fmt $<&T as core::fmt::Debug>::fmt $core::ptr::drop_in_place<core::fmt::Error> $<core::fmt::Error as core::fmt::Debug>::fmt $<core::alloc::layout::LayoutError as core::fmt::Debug>::fmt $core::fmt::num::<impl core::fmt::Debug for usize>::fmt $<&T as core::fmt::Debug>::fmt $core::ops::function::FnOnce::call_once $<core::slice::ascii::EscapeAscii as core::fmt::Display>::fmt $<&T as core::fmt::Debug>::fmt $<&T as core::fmt::Display>::fmt $<core::fmt::Arguments as core::fmt::Display>::fmt $<core::ops::range::Range<Idx> as core::fmt::Debug>::fmt $<char as core::fmt::Debug>::fmt $core::ptr::drop_in_place<core::fmt::Error> $<T as core::any::Any>::type_id $core::ptr::drop_in_place<&core::iter::adapters::copied::Copied<core::slice::iter::Iter<u8>>> $<core::fmt::builders::PadAdapter as core::fmt::Write>::write_str $core::fmt::Write::write_char $core::fmt::Write::write_fmt $<&mut W as core::fmt::Write>::write_str $<&mut W as core::fmt::Write>::write_char $<&mut W as core::fmt::Write>::write_fmt)
    (data (;0;) (i32.const 1048576) "\02\00\00\00\04\00\00\00\04\00\00\00\03\00\00\00\04\00\00\00\04\00\00\00\02\00\00\00\04\00\00\00\04\00\00\00\05\00\00\00src/main.rs\00(\00\10\00\0b\00\00\00\0d\00\00\00\05\00\00\00()\00\00\0f\00\00\00\04\00\00\00\04\00\00\00\10\00\00\00\11\00\00\00\12\00\00\00\0f\00\00\00\04\00\00\00\04\00\00\00\13\00\00\00\14\00\00\00\15\00\00\00\0f\00\00\00\04\00\00\00\04\00\00\00\16\00\00\00\17\00\00\00\18\00\00\00\0f\00\00\00\04\00\00\00\04\00\00\00\19\00\00\00\1a\00\00\00\1b\00\00\00invalid args\a8\00\10\00\0c\00\00\00/rustc/c469197b19d53a6c45378568f73c00986b20a5a5/library/core/src/fmt/mod.rs\00\bc\00\10\00K\00\00\005\01\00\00\0d\00\00\00\00\00\00\00\0f\00\00\00\04\00\00\00\04\00\00\00\1c\00\00\00\0f\00\00\00\04\00\00\00\04\00\00\00\1d\00\00\00called `Option::unwrap()` on a `None` valueinternal error: entered unreachable code/rustc/c469197b19d53a6c45378568f73c00986b20a5a5/library/alloc/src/vec/mod.rs\00\8f\01\10\00L\00\00\00\c9\07\00\00$\00\00\00Ok\00\00\0f\00\00\00\04\00\00\00\04\00\00\00\1e\00\00\00Err\00\0f\00\00\00\04\00\00\00\04\00\00\00\1f\00\00\00mainfatal runtime error: unwrap failed: CString::new(\22main\22) = \0a\18\02\10\00;\00\00\00S\02\10\00\01\00\00\00library/std/src/rt.rs\00\00\00d\02\10\00\15\00\00\00r\00\00\00\0d\00\00\00library/std/src/thread/mod.rsfailed to generate unique thread ID: bitspace exhausted\a9\02\10\007\00\00\00\8c\02\10\00\1d\00\00\00J\04\00\00\0d\00\00\00RUST_BACKTRACEfailed to write the buffered data\00\06\03\10\00!\00\00\00\17\00\00\00\bc\00\10\00\00\00\00\00already borrowed \00\00\00\00\00\00\00\01\00\00\00!\00\00\00library/std/src/io/stdio.rs\00\5c\03\10\00\1b\00\00\00}\02\00\00\13\00\00\00library/std/src/io/mod.rs\00\00\00\88\03\10\00\19\00\00\00C\05\00\00 \00\00\00advancing io slices beyond their length\00\b4\03\10\00'\00\00\00\88\03\10\00\19\00\00\00E\05\00\00\0d\00\00\00advancing IoSlice beyond its length\00\f4\03\10\00#\00\00\00library/std/src/sys/wasi/io.rs\00\00 \04\10\00\1e\00\00\00\17\00\00\00\0d\00\00\00failed to write whole bufferP\04\10\00\1c\00\00\00\17\00\00\00\88\03\10\00\19\00\00\00-\06\00\00$\00\00\00\22\00\00\00\0c\00\00\00\04\00\00\00#\00\00\00$\00\00\00%\00\00\00formatter error\00\a0\04\10\00\0f\00\00\00(\00\00\00\22\00\00\00\0c\00\00\00\04\00\00\00&\00\00\00'\00\00\00(\00\00\00\22\00\00\00\0c\00\00\00\04\00\00\00)\00\00\00*\00\00\00+\00\00\00input must be utf-8\00\ec\04\10\00\13\00\00\00(\00\00\00library/std/src/os/fd/owned.rslibrary/std/src/panic.rs\00\00*\05\10\00\18\00\00\00\f5\00\00\00\12\00\00\00fullcannot recursively acquire mutexX\05\10\00 \00\00\00library/std/src/sys/wasi/../unsupported/locks/mutex.rs\00\00\80\05\10\006\00\00\00\14\00\00\00\09\00\00\00library/std/src/sync/once.rs\c8\05\10\00\1c\00\00\00\95\00\00\002\00\00\00\c8\05\10\00\1c\00\00\00\d0\00\00\00\14\00\00\00\c8\05\10\00\1c\00\00\00\d0\00\00\001\00\00\00lock count overflow in reentrant mutexlibrary/std/src/sync/remutex.rs\00\00\00:\06\10\00\1f\00\00\00\91\00\00\00\0e\00\00\00\ff\ff\ff\fffile name contained an unexpected NUL byte\00\00p\06\10\00*\00\00\00\14\00\00\00\02\00\00\00\9c\06\10\00stack backtrace:\0a\00\00\00\b0\06\10\00\11\00\00\00note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.\0a\cc\06\10\00X\00\00\00library/std/src/sys_common/thread_info.rs\00\00\00,\07\10\00)\00\00\00\15\00\00\003\00\00\00fatal runtime error: assertion failed: thread_info.is_none()\0a\00\00\00h\07\10\00=\00\00\00,\07\10\00)\00\00\00*\00\00\00+\00\00\00memory allocation of  bytes failed\0a\00\c0\07\10\00\15\00\00\00\d5\07\10\00\0e\00\00\00 bytes failed\00\00\00\c0\07\10\00\15\00\00\00\f4\07\10\00\0d\00\00\00library/std/src/alloc.rs\14\08\10\00\18\00\00\00T\01\00\00\09\00\00\00library/std/src/panicking.rs<\08\10\00\1c\00\00\00\00\01\00\00$\00\00\00Box<dyn Any><unnamed>\00\00\00,\00\00\00\04\00\00\00\04\00\00\00-\00\00\00.\00\00\00/\00\00\000\00\00\001\00\00\002\00\00\003\00\00\004\00\00\00\0c\00\00\00\04\00\00\005\00\00\006\00\00\00/\00\00\000\00\00\007\00\00\008\00\00\009\00\00\00 \00\00\00\00\00\00\00\01\00\00\00:\00\00\00;\00\00\00<\00\00\00=\00\00\00>\00\00\00?\00\00\00@\00\00\00thread '' panicked at :\0a\f8\08\10\00\08\00\00\00\00\09\10\00\0e\00\00\00\0e\09\10\00\02\00\00\00S\02\10\00\01\00\00\00note: a backtrace for this error was stored at ``\0a\00\000\09\10\000\00\00\00`\09\10\00\02\00\00\00note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\0a\00\00t\09\10\00N\00\00\00<\08\10\00\1c\00\00\00g\02\00\00\1f\00\00\00<\08\10\00\1c\00\00\00h\02\00\00\1e\00\00\00A\00\00\00\0c\00\00\00\04\00\00\00B\00\00\00\0f\00\00\00\08\00\00\00\04\00\00\00C\00\00\00\0f\00\00\00\08\00\00\00\04\00\00\00D\00\00\00E\00\00\00F\00\00\00\10\00\00\00\04\00\00\00G\00\00\00H\00\00\00 \00\00\00\00\00\00\00\01\00\00\00I\00\00\00\0apanicked after panic::always_abort(), aborting.\0a\00\00\00\bc\00\10\00\00\00\00\00D\0a\10\001\00\00\00thread panicked while processing panic. aborting.\0a\00\00\88\0a\10\002\00\00\00thread caused non-unwinding panic. aborting.\0a\00\00\00\c4\0a\10\00-\00\00\00fatal runtime error: failed to initiate panic, error \00\00\00\fc\0a\10\005\00\00\00S\02\10\00\01\00\00\00\0c\05\10\00\1e\00\00\00\a1\00\00\00\09\00\00\00failed to find a pre-opened file descriptor through which  could be opened\00\00T\0b\10\00:\00\00\00\8e\0b\10\00\10\00\00\00J\00\00\00\0c\00\00\00\04\00\00\00K\00\00\00J\00\00\00\0c\00\00\00\04\00\00\00L\00\00\00K\00\00\00\b0\0b\10\00M\00\00\00N\00\00\00O\00\00\00M\00\00\00P\00\00\00\08\00\0e\00\0f\00?\00\02\00@\005\00\0d\00\04\00\03\00,\00\1b\00\1c\00I\00\14\00\06\004\000\00fatal runtime error: rwlock locked for writing\0a\00\10\0c\10\00/\00\00\00Once instance has previously been poisoned\00\00H\0c\10\00*\00\00\00one-time initialization may not be performed recursively|\0c\10\008\00\00\00./\00.\00\00\00\00R\00\00\00\04\00\00\00\04\00\00\00S\00\00\00T\00\00\00U\00\00\00R\00\00\00\04\00\00\00\04\00\00\00V\00\00\00library/alloc/src/raw_vec.rscapacity overflow\00\00\00\08\0d\10\00\11\00\00\00\ec\0c\10\00\1c\00\00\00\16\02\00\00\05\00\00\00called `Option::unwrap()` on a `None` valuelibrary/alloc/src/ffi/c_str.rs\00\00\00_\0d\10\00\1e\00\00\00\1b\01\00\007\00\00\00a formatting trait implementation returned an error\00W\00\00\00\00\00\00\00\01\00\00\00X\00\00\00library/alloc/src/fmt.rs\d4\0d\10\00\18\00\00\00b\02\00\00 \00\00\00called `Result::unwrap()` on an `Err` value\00W\00\00\00\00\00\00\00\01\00\00\00Y\00\00\00library/alloc/src/sync.rs\00\00\008\0e\10\00\19\00\00\00n\01\00\002\00\00\00NulErrorR\00\00\00\04\00\00\00\04\00\00\00Z\00\00\00R\00\00\00\04\00\00\00\04\00\00\00[\00\00\00called `Option::unwrap()` on a `None` value)invalid args\b8\0e\10\00\0c\00\00\00library/core/src/fmt/mod.rs..\00\00\00\e7\0e\10\00\02\00\00\00BorrowMutError\22\00\02\0f\10\00\01\00\00\00\02\0f\10\00\01\00\00\00\8c\0e\10\00\00\00\00\00:[\00\00\8c\0e\10\00\00\00\00\00\1c\0f\10\00\01\00\00\00\1c\0f\10\00\01\00\00\00panicked at :\0a\00\00c\00\00\00\00\00\00\00\01\00\00\00d\00\00\00index out of bounds: the len is  but the index is \00\00X\0f\10\00 \00\00\00x\0f\10\00\12\00\00\00==!=matchesassertion `left  right` failed\0a  left: \0a right: \00\a7\0f\10\00\10\00\00\00\b7\0f\10\00\17\00\00\00\ce\0f\10\00\09\00\00\00 right` failed: \0a  left: \00\00\00\a7\0f\10\00\10\00\00\00\f0\0f\10\00\10\00\00\00\00\10\10\00\09\00\00\00\ce\0f\10\00\09\00\00\00: \00\00\8c\0e\10\00\00\00\00\00,\10\10\00\02\00\00\00e\00\00\00\0c\00\00\00\04\00\00\00f\00\00\00g\00\00\00h\00\00\00    , ,\0a((\0a,\0a]0xlibrary/core/src/fmt/num.rs\00h\10\10\00\1b\00\00\00i\00\00\00\17\00\00\0000010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899e\00\00\00\04\00\00\00\04\00\00\00i\00\00\00j\00\00\00k\00\00\00\cc\0e\10\00\1b\00\00\005\01\00\00\0d\00\00\00falsetrue\00\00\00\cc\0e\10\00\1b\00\00\00\1b\09\00\00\1a\00\00\00\cc\0e\10\00\1b\00\00\00\14\09\00\00\22\00\00\00range start index  out of range for slice of length \b0\11\10\00\12\00\00\00\c2\11\10\00\22\00\00\00range end index \f4\11\10\00\10\00\00\00\c2\11\10\00\22\00\00\00slice index starts at  but ends at \00\14\12\10\00\16\00\00\00*\12\10\00\0d\00\00\00\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\03\03\03\03\03\03\03\03\03\03\03\03\03\03\03\03\04\04\04\04\04\00\00\00\00\00\00\00\00\00\00\00\00[...]byte index  is not a char boundary; it is inside  (bytes ) of ``\00\00N\13\10\00\0b\00\00\00Y\13\10\00&\00\00\00\7f\13\10\00\08\00\00\00\87\13\10\00\06\00\00\00\8d\13\10\00\01\00\00\00begin <= end ( <= ) when slicing `\00\00\b8\13\10\00\0e\00\00\00\c6\13\10\00\04\00\00\00\ca\13\10\00\10\00\00\00\8d\13\10\00\01\00\00\00 is out of bounds of `\00\00N\13\10\00\0b\00\00\00\fc\13\10\00\16\00\00\00\8d\13\10\00\01\00\00\00library/core/src/str/mod.rs\00,\14\10\00\1b\00\00\00\03\01\00\00,\00\00\00library/core/src/unicode/printable.rs\00\00\00X\14\10\00%\00\00\00\1a\00\00\006\00\00\00X\14\10\00%\00\00\00\0a\00\00\00+\00\00\00\00\06\01\01\03\01\04\02\05\07\07\02\08\08\09\02\0a\05\0b\02\0e\04\10\01\11\02\12\05\13\11\14\01\15\02\17\02\19\0d\1c\05\1d\08\1f\01$\01j\04k\02\af\03\b1\02\bc\02\cf\02\d1\02\d4\0c\d5\09\d6\02\d7\02\da\01\e0\05\e1\02\e7\04\e8\02\ee \f0\04\f8\02\fa\03\fb\01\0c';>NO\8f\9e\9e\9f{\8b\93\96\a2\b2\ba\86\b1\06\07\096=>V\f3\d0\d1\04\14\1867VW\7f\aa\ae\af\bd5\e0\12\87\89\8e\9e\04\0d\0e\11\12)14:EFIJNOde\5c\b6\b7\1b\1c\07\08\0a\0b\14\1769:\a8\a9\d8\d9\097\90\91\a8\07\0a;>fi\8f\92\11o_\bf\ee\efZb\f4\fc\ffST\9a\9b./'(U\9d\a0\a1\a3\a4\a7\a8\ad\ba\bc\c4\06\0b\0c\15\1d:?EQ\a6\a7\cc\cd\a0\07\19\1a\22%>?\e7\ec\ef\ff\c5\c6\04 #%&(38:HJLPSUVXZ\5c^`cefksx}\7f\8a\a4\aa\af\b0\c0\d0\ae\afno\be\93^\22{\05\03\04-\03f\03\01/.\80\82\1d\031\0f\1c\04$\09\1e\05+\05D\04\0e*\80\aa\06$\04$\04(\084\0bNC\817\09\16\0a\08\18;E9\03c\08\090\16\05!\03\1b\05\01@8\04K\05/\04\0a\07\09\07@ '\04\0c\096\03:\05\1a\07\04\0c\07PI73\0d3\07.\08\0a\81&RK+\08*\16\1a&\1c\14\17\09N\04$\09D\0d\19\07\0a\06H\08'\09u\0bB>*\06;\05\0a\06Q\06\01\05\10\03\05\80\8bb\1eH\08\0a\80\a6^\22E\0b\0a\06\0d\13:\06\0a6,\04\17\80\b9<dS\0cH\09\0aFE\1bH\08S\0dI\07\0a\80\f6F\0a\1d\03GI7\03\0e\08\0a\069\07\0a\816\19\07;\03\1cV\01\0f2\0d\83\9bfu\0b\80\c4\8aLc\0d\840\10\16\8f\aa\82G\a1\b9\829\07*\04\5c\06&\0aF\0a(\05\13\82\b0[eK\049\07\11@\05\0b\02\0e\97\f8\08\84\d6*\09\a2\e7\813\0f\01\1d\06\0e\04\08\81\8c\89\04k\05\0d\03\09\07\10\92`G\09t<\80\f6\0as\08p\15Fz\14\0c\14\0cW\09\19\80\87\81G\03\85B\0f\15\84P\1f\06\06\80\d5+\05>!\01p-\03\1a\04\02\81@\1f\11:\05\01\81\d0*\82\e6\80\f7)L\04\0a\04\02\83\11DL=\80\c2<\06\01\04U\05\1b4\02\81\0e,\04d\0cV\0a\80\ae8\1d\0d,\04\09\07\02\0e\06\80\9a\83\d8\04\11\03\0d\03w\04_\06\0c\04\01\0f\0c\048\08\0a\06(\08\22N\81T\0c\1d\03\09\076\08\0e\04\09\07\09\07\80\cb%\0a\84\06\00\01\03\05\05\06\06\02\07\06\08\07\09\11\0a\1c\0b\19\0c\1a\0d\10\0e\0c\0f\04\10\03\12\12\13\09\16\01\17\04\18\01\19\03\1a\07\1b\01\1c\02\1f\16 \03+\03-\0b.\010\031\022\01\a7\02\a9\02\aa\04\ab\08\fa\02\fb\05\fd\02\fe\03\ff\09\adxy\8b\8d\a20WX\8b\8c\90\1c\dd\0e\0fKL\fb\fc./?\5c]_\e2\84\8d\8e\91\92\a9\b1\ba\bb\c5\c6\c9\ca\de\e4\e5\ff\00\04\11\12)147:;=IJ]\84\8e\92\a9\b1\b4\ba\bb\c6\ca\ce\cf\e4\e5\00\04\0d\0e\11\12)14:;EFIJ^de\84\91\9b\9d\c9\ce\cf\0d\11):;EIW[\5c^_de\8d\91\a9\b4\ba\bb\c5\c9\df\e4\e5\f0\0d\11EIde\80\84\b2\bc\be\bf\d5\d7\f0\f1\83\85\8b\a4\a6\be\bf\c5\c7\cf\da\dbH\98\bd\cd\c6\ce\cfINOWY^_\89\8e\8f\b1\b6\b7\bf\c1\c6\c7\d7\11\16\17[\5c\f6\f7\fe\ff\80mq\de\df\0e\1fno\1c\1d_}~\ae\af\7f\bb\bc\16\17\1e\1fFGNOXZ\5c^~\7f\b5\c5\d4\d5\dc\f0\f1\f5rs\8ftu\96&./\a7\af\b7\bf\c7\cf\d7\df\9a@\97\980\8f\1f\d2\d4\ce\ffNOZ[\07\08\0f\10'/\ee\efno7=?BE\90\91Sgu\c8\c9\d0\d1\d8\d9\e7\fe\ff\00 _\22\82\df\04\82D\08\1b\04\06\11\81\ac\0e\80\ab\05\1f\09\81\1b\03\19\08\01\04/\044\04\07\03\01\07\06\07\11\0aP\0f\12\07U\07\03\04\1c\0a\09\03\08\03\07\03\02\03\03\03\0c\04\05\03\0b\06\01\0e\15\05N\07\1b\07W\07\02\06\17\0cP\04C\03-\03\01\04\11\06\0f\0c:\04\1d%_ m\04j%\80\c8\05\82\b0\03\1a\06\82\fd\03Y\07\16\09\18\09\14\0c\14\0cj\06\0a\06\1a\06Y\07+\05F\0a,\04\0c\04\01\031\0b,\04\1a\06\0b\03\80\ac\06\0a\06/1M\03\80\a4\08<\03\0f\03<\078\08+\05\82\ff\11\18\08/\11-\03!\0f!\0f\80\8c\04\82\97\19\0b\15\88\94\05/\05;\07\02\0e\18\09\80\be\22t\0c\80\d6\1a\0c\05\80\ff\05\80\df\0c\f2\9d\037\09\81\5c\14\80\b8\08\80\cb\05\0a\18;\03\0a\068\08F\08\0c\06t\0b\1e\03Z\04Y\09\80\83\18\1c\0a\16\09L\04\80\8a\06\ab\a4\0c\17\041\a1\04\81\da&\07\0c\05\05\80\a6\10\81\f5\07\01 *\06L\04\80\8d\04\80\be\03\1b\03\0f\0dlibrary/core/src/unicode/unicode_data.rs\1c\1a\10\00(\00\00\00P\00\00\00(\00\00\00\1c\1a\10\00(\00\00\00\5c\00\00\00\16\00\00\000123456789abcdeflibrary/core/src/escape.rs\00\00t\1a\10\00\1a\00\00\004\00\00\00\0b\00\00\00\5cu{\00t\1a\10\00\1a\00\00\00b\00\00\00#\00\00\00Error\00\00\00\00\03\00\00\83\04 \00\91\05`\00]\13\a0\00\12\17 \1f\0c `\1f\ef,\a0+*0 ,o\a6\e0,\02\a8`-\1e\fb`.\00\fe 6\9e\ff`6\fd\01\e16\01\0a!7$\0d\e17\ab\0ea9/\18\a190\1caH\f3\1e\a1L@4aP\f0j\a1QOo!R\9d\bc\a1R\00\cfaSe\d1\a1S\00\da!T\00\e0\e1U\ae\e2aW\ec\e4!Y\d0\e8\a1Y \00\eeY\f0\01\7fZ\00p\00\07\00-\01\01\01\02\01\02\01\01H\0b0\15\10\01e\07\02\06\02\02\01\04#\01\1e\1b[\0b:\09\09\01\18\04\01\09\01\03\01\05+\03<\08*\18\01 7\01\01\01\04\08\04\01\03\07\0a\02\1d\01:\01\01\01\02\04\08\01\09\01\0a\02\1a\01\02\029\01\04\02\04\02\02\03\03\01\1e\02\03\01\0b\029\01\04\05\01\02\04\01\14\02\16\06\01\01:\01\01\02\01\04\08\01\07\03\0a\02\1e\01;\01\01\01\0c\01\09\01(\01\03\017\01\01\03\05\03\01\04\07\02\0b\02\1d\01:\01\02\01\02\01\03\01\05\02\07\02\0b\02\1c\029\02\01\01\02\04\08\01\09\01\0a\02\1d\01H\01\04\01\02\03\01\01\08\01Q\01\02\07\0c\08b\01\02\09\0b\07I\02\1b\01\01\01\01\017\0e\01\05\01\02\05\0b\01$\09\01f\04\01\06\01\02\02\02\19\02\04\03\10\04\0d\01\02\02\06\01\0f\01\00\03\00\03\1d\02\1e\02\1e\02@\02\01\07\08\01\02\0b\09\01-\03\01\01u\02\22\01v\03\04\02\09\01\06\03\db\02\02\01:\01\01\07\01\01\01\01\02\08\06\0a\02\010\1f1\040\07\01\01\05\01(\09\0c\02 \04\02\02\01\038\01\01\02\03\01\01\03:\08\02\02\98\03\01\0d\01\07\04\01\06\01\03\02\c6@\00\01\c3!\00\03\8d\01` \00\06i\02\00\04\01\0a \02P\02\00\01\03\01\04\01\19\02\05\01\97\02\1a\12\0d\01&\08\19\0b.\030\01\02\04\02\02'\01C\06\02\02\02\02\0c\01\08\01/\013\01\01\03\02\02\05\02\01\01*\02\08\01\ee\01\02\01\04\01\00\01\00\10\10\10\00\02\00\01\e2\01\95\05\00\03\01\02\05\04(\03\04\01\a5\02\00\04\00\02P\03F\0b1\04{\016\0f)\01\02\02\0a\031\04\02\02\07\01=\03$\05\01\08>\01\0c\024\09\0a\04\02\01_\03\02\01\01\02\06\01\02\01\9d\01\03\08\15\029\02\01\01\01\01\16\01\0e\07\03\05\c3\08\02\03\01\01\17\01Q\01\02\06\01\01\02\01\01\02\01\02\eb\01\02\04\06\02\01\02\1b\02U\08\02\01\01\02j\01\01\01\02\06\01\01e\03\02\04\01\05\00\09\01\02\f5\01\0a\02\01\01\04\01\90\04\02\02\04\01 \0a(\06\02\04\08\01\09\06\02\03.\0d\01\02\00\07\01\06\01\01R\16\02\07\01\02\01\02z\06\03\01\01\02\01\07\01\01H\02\03\01\01\01\00\02\0b\024\05\05\01\01\01\00\01\06\0f\00\05;\07\00\01?\04Q\01\00\02\00.\02\17\00\01\01\03\04\05\08\08\02\07\1e\04\94\03\007\042\08\01\0e\01\16\05\01\0f\00\07\01\11\02\07\01\02\01\05d\01\a0\07\00\01=\04\00\04\00\07m\07\00`\80\f0\00LayoutError")
    (data (;1;) (i32.const 1056292) "\01\00\00\00\ff\ff\ff\ff\bd\0c\10\00")
  )
  (core module (;1;)
    (type (;0;) (func))
    (type (;1;) (func (param i32)))
    (type (;2;) (func (param i32 i32)))
    (type (;3;) (func (param i32 i64 i32)))
    (type (;4;) (func (param i32 i32 i32 i32)))
    (type (;5;) (func (param i32 i32 i32 i32 i32 i32 i32)))
    (type (;6;) (func (param i32) (result i32)))
    (type (;7;) (func (param i32 i32 i32)))
    (type (;8;) (func (param i32 i32 i32 i32) (result i32)))
    (type (;9;) (func (param i32 i32 i32 i32 i32)))
    (type (;10;) (func (result i32)))
    (type (;11;) (func (param i32 i32 i32) (result i32)))
    (type (;12;) (func (param i32 i32) (result i32)))
    (type (;13;) (func (param i32 i32 i32 i32 i32 i64 i64 i32 i32) (result i32)))
    (type (;14;) (func))
    (import "env" "memory" (memory (;0;) 0))
    (import "__main_module__" "_start" (func $wasi_snapshot_preview1::run::_start (;0;) (type 0)))
    (import "wasi:filesystem/preopens@0.2.0-rc-2023-11-10" "get-directories" (func $wasi_snapshot_preview1::descriptors::Descriptors::new::get_preopens_import (;1;) (type 1)))
    (import "wasi:filesystem/types@0.2.0-rc-2023-11-10" "[resource-drop]directory-entry-stream" (func $<wasi_snapshot_preview1::bindings::wasi::filesystem::types::DirectoryEntryStream as wit_bindgen::WasmResource>::drop::drop (;2;) (type 1)))
    (import "wasi:filesystem/types@0.2.0-rc-2023-11-10" "[method]descriptor.get-type" (func $wasi_snapshot_preview1::bindings::wasi::filesystem::types::Descriptor::get_type::wit_import (;3;) (type 2)))
    (import "wasi:filesystem/types@0.2.0-rc-2023-11-10" "filesystem-error-code" (func $wasi_snapshot_preview1::bindings::wasi::filesystem::types::filesystem_error_code::wit_import (;4;) (type 2)))
    (import "wasi:io/error@0.2.0-rc-2023-11-10" "[resource-drop]error" (func $<wasi_snapshot_preview1::bindings::wasi::io::error::Error as wit_bindgen::WasmResource>::drop::drop (;5;) (type 1)))
    (import "wasi:io/streams@0.2.0-rc-2023-11-10" "[resource-drop]input-stream" (func $<wasi_snapshot_preview1::bindings::wasi::io::streams::InputStream as wit_bindgen::WasmResource>::drop::drop (;6;) (type 1)))
    (import "wasi:io/streams@0.2.0-rc-2023-11-10" "[resource-drop]output-stream" (func $<wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream as wit_bindgen::WasmResource>::drop::drop (;7;) (type 1)))
    (import "wasi:filesystem/types@0.2.0-rc-2023-11-10" "[resource-drop]descriptor" (func $<wasi_snapshot_preview1::bindings::wasi::filesystem::types::Descriptor as wit_bindgen::WasmResource>::drop::drop (;8;) (type 1)))
    (import "__main_module__" "cabi_realloc" (func $wasi_snapshot_preview1::State::new::cabi_realloc (;9;) (type 8)))
    (import "wasi:cli/environment@0.2.0-rc-2023-11-10" "get-environment" (func $wasi_snapshot_preview1::State::get_environment::get_environment_import (;10;) (type 1)))
    (import "wasi:filesystem/types@0.2.0-rc-2023-11-10" "[method]descriptor.write-via-stream" (func $wasi_snapshot_preview1::bindings::wasi::filesystem::types::Descriptor::write_via_stream::wit_import (;11;) (type 3)))
    (import "wasi:filesystem/types@0.2.0-rc-2023-11-10" "[method]descriptor.append-via-stream" (func $wasi_snapshot_preview1::bindings::wasi::filesystem::types::Descriptor::append_via_stream::wit_import (;12;) (type 2)))
    (import "wasi:filesystem/types@0.2.0-rc-2023-11-10" "[method]descriptor.open-at" (func $wasi_snapshot_preview1::bindings::wasi::filesystem::types::Descriptor::open_at::wit_import (;13;) (type 5)))
    (import "wasi:cli/terminal-output@0.2.0-rc-2023-11-10" "[resource-drop]terminal-output" (func $<wasi_snapshot_preview1::bindings::wasi::cli::terminal_output::TerminalOutput as wit_bindgen::WasmResource>::drop::drop (;14;) (type 1)))
    (import "wasi:sockets/tcp@0.2.0-rc-2023-11-10" "[resource-drop]tcp-socket" (func $<wasi_snapshot_preview1::bindings::wasi::sockets::tcp::TcpSocket as wit_bindgen::WasmResource>::drop::drop (;15;) (type 1)))
    (import "wasi:cli/terminal-input@0.2.0-rc-2023-11-10" "[resource-drop]terminal-input" (func $<wasi_snapshot_preview1::bindings::wasi::cli::terminal_input::TerminalInput as wit_bindgen::WasmResource>::drop::drop (;16;) (type 1)))
    (import "wasi:cli/stderr@0.2.0-rc-2023-11-10" "get-stderr" (func $wasi_snapshot_preview1::bindings::wasi::cli::stderr::get_stderr::wit_import (;17;) (type 10)))
    (import "wasi:cli/exit@0.2.0-rc-2023-11-10" "exit" (func $wasi_snapshot_preview1::bindings::wasi::cli::exit::exit::wit_import (;18;) (type 1)))
    (import "wasi:cli/stdin@0.2.0-rc-2023-11-10" "get-stdin" (func $wasi_snapshot_preview1::bindings::wasi::cli::stdin::get_stdin::wit_import (;19;) (type 10)))
    (import "wasi:cli/stdout@0.2.0-rc-2023-11-10" "get-stdout" (func $wasi_snapshot_preview1::bindings::wasi::cli::stdout::get_stdout::wit_import (;20;) (type 10)))
    (import "wasi:cli/terminal-stdin@0.2.0-rc-2023-11-10" "get-terminal-stdin" (func $wasi_snapshot_preview1::bindings::wasi::cli::terminal_stdin::get_terminal_stdin::wit_import (;21;) (type 1)))
    (import "wasi:cli/terminal-stdout@0.2.0-rc-2023-11-10" "get-terminal-stdout" (func $wasi_snapshot_preview1::bindings::wasi::cli::terminal_stdout::get_terminal_stdout::wit_import (;22;) (type 1)))
    (import "wasi:cli/terminal-stderr@0.2.0-rc-2023-11-10" "get-terminal-stderr" (func $wasi_snapshot_preview1::bindings::wasi::cli::terminal_stderr::get_terminal_stderr::wit_import (;23;) (type 1)))
    (import "wasi:io/streams@0.2.0-rc-2023-11-10" "[method]output-stream.check-write" (func $wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream::check_write::wit_import (;24;) (type 2)))
    (import "wasi:io/streams@0.2.0-rc-2023-11-10" "[method]output-stream.write" (func $wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream::write::wit_import (;25;) (type 4)))
    (import "wasi:io/streams@0.2.0-rc-2023-11-10" "[method]output-stream.blocking-write-and-flush" (func $wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream::blocking_write_and_flush::wit_import (;26;) (type 4)))
    (import "wasi:io/streams@0.2.0-rc-2023-11-10" "[method]output-stream.blocking-flush" (func $wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream::blocking_flush::wit_import (;27;) (type 2)))
    (func $wasi:cli/run@0.2.0-rc-2023-11-10#run (;28;) (type 10) (result i32)
      call $allocate_stack
      call $wasi_snapshot_preview1::run::_start
      i32.const 0
    )
    (func $cabi_import_realloc (;29;) (type 8) (param i32 i32 i32 i32) (result i32)
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
              i32.const 113
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
            i32.const 2404
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
          i32.const 2405
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
        i32.const 219
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
      i32.const 226
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
    (func $wasi_snapshot_preview1::State::ptr (;30;) (type 10) (result i32)
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
    (func $wasi_snapshot_preview1::BumpArena::alloc (;31;) (type 11) (param i32 i32 i32) (result i32)
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
      i32.const 143
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
    (func $wasi_snapshot_preview1::ImportAlloc::with_arena (;32;) (type 7) (param i32 i32 i32)
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
          i32.const 205
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
        i32.const 198
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
      call $wasi_snapshot_preview1::descriptors::Descriptors::new::get_preopens_import
      local.get 0
      i32.const 0
      i32.store offset=8
      local.get 3
      i32.const 48
      i32.add
      global.set $__stack_pointer
    )
    (func $cabi_export_realloc (;33;) (type 8) (param i32 i32 i32 i32) (result i32)
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
          i32.const 249
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
        i32.const 2404
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
      i32.const 2405
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
    (func $environ_get (;34;) (type 12) (param i32 i32) (result i32)
      (local i32 i32 i32 i32)
      call $allocate_stack
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          call $wasi_snapshot_preview1::State::ptr
          local.tee 3
          i32.load
          i32.const 560490357
          i32.ne
          br_if 0 (;@2;)
          local.get 3
          i32.load offset=65532
          i32.const 560490357
          i32.ne
          br_if 1 (;@1;)
          local.get 2
          local.get 3
          call $wasi_snapshot_preview1::State::get_environment
          block ;; label = @3
            local.get 2
            i32.load offset=4
            local.tee 4
            i32.eqz
            br_if 0 (;@3;)
            local.get 2
            i32.load
            local.tee 3
            local.get 4
            i32.const 4
            i32.shl
            i32.add
            local.set 5
            loop ;; label = @4
              local.get 0
              local.get 1
              i32.store
              local.get 1
              local.get 3
              i32.load
              local.get 3
              i32.const 4
              i32.add
              local.tee 4
              i32.load
              call $memcpy
              local.get 4
              i32.load
              i32.add
              local.tee 1
              i32.const 61
              i32.store8
              local.get 1
              i32.const 1
              i32.add
              local.get 3
              i32.const 8
              i32.add
              i32.load
              local.get 3
              i32.const 12
              i32.add
              local.tee 1
              i32.load
              call $memcpy
              local.get 1
              i32.load
              i32.add
              local.tee 1
              i32.const 0
              i32.store8
              local.get 1
              i32.const 1
              i32.add
              local.set 1
              local.get 0
              i32.const 4
              i32.add
              local.set 0
              local.get 3
              i32.const 16
              i32.add
              local.tee 3
              local.get 5
              i32.ne
              br_if 0 (;@4;)
            end
          end
          local.get 2
          i32.const 48
          i32.add
          global.set $__stack_pointer
          i32.const 0
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
        i32.const 2404
        call $wasi_snapshot_preview1::macros::eprint_u32
        local.get 2
        i32.const 8250
        i32.store16 offset=11 align=1
        local.get 2
        i32.const 11
        i32.add
        i32.const 2
        call $wasi_snapshot_preview1::macros::print
        local.get 2
        i32.const 10
        i32.store8 offset=27
        local.get 2
        i64.const 7234307576302018670
        i64.store offset=19 align=1
        local.get 2
        i64.const 8028075845441778529
        i64.store offset=11 align=1
        local.get 2
        i32.const 11
        i32.add
        i32.const 17
        call $wasi_snapshot_preview1::macros::print
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
      i32.const 2405
      call $wasi_snapshot_preview1::macros::eprint_u32
      local.get 2
      i32.const 8250
      i32.store16 offset=11 align=1
      local.get 2
      i32.const 11
      i32.add
      i32.const 2
      call $wasi_snapshot_preview1::macros::print
      local.get 2
      i32.const 10
      i32.store8 offset=27
      local.get 2
      i64.const 7234307576302018670
      i64.store offset=19 align=1
      local.get 2
      i64.const 8028075845441778529
      i64.store offset=11 align=1
      local.get 2
      i32.const 11
      i32.add
      i32.const 17
      call $wasi_snapshot_preview1::macros::print
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
    (func $wasi_snapshot_preview1::State::get_environment (;35;) (type 2) (param i32 i32)
      (local i32 i32 i32)
      global.get $__stack_pointer
      i32.const 64
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              local.get 1
              i32.load offset=65212
              local.tee 3
              i32.eqz
              br_if 0 (;@4;)
              local.get 1
              i32.load offset=65216
              local.set 4
              br 1 (;@3;)
            end
            local.get 2
            i64.const 0
            i64.store offset=16 align=4
            local.get 1
            i32.load offset=4
            br_if 1 (;@2;)
            local.get 1
            i32.const 12
            i32.add
            local.tee 3
            i32.load
            local.set 4
            local.get 3
            local.get 1
            i32.const 10288
            i32.add
            i32.store
            local.get 4
            br_if 2 (;@1;)
            local.get 2
            i32.const 16
            i32.add
            call $wasi_snapshot_preview1::State::get_environment::get_environment_import
            local.get 1
            i32.const 0
            i32.store offset=12
            local.get 1
            local.get 2
            i32.load offset=20
            local.tee 4
            i32.store offset=65216
            local.get 1
            local.get 2
            i32.load offset=16
            local.tee 3
            i32.store offset=65212
          end
          local.get 2
          i32.const 8
          i32.add
          local.get 3
          local.get 4
          call $<core::option::Option<T> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap
          local.get 2
          i32.load offset=12
          local.set 1
          local.get 0
          local.get 2
          i32.load offset=8
          i32.store
          local.get 0
          local.get 1
          i32.store offset=4
          local.get 2
          i32.const 64
          i32.add
          global.set $__stack_pointer
          return
        end
        local.get 2
        i32.const 32
        i32.store8 offset=63
        local.get 2
        i32.const 1701734764
        i32.store offset=59 align=1
        local.get 2
        i64.const 2338042707334751329
        i64.store offset=51 align=1
        local.get 2
        i64.const 2338600898263348341
        i64.store offset=43 align=1
        local.get 2
        i64.const 7162263158133189730
        i64.store offset=35 align=1
        local.get 2
        i64.const 7018969289221893749
        i64.store offset=27 align=1
        local.get 2
        i32.const 27
        i32.add
        i32.const 37
        call $wasi_snapshot_preview1::macros::print
        i32.const 198
        call $wasi_snapshot_preview1::macros::eprint_u32
        local.get 2
        i32.const 8250
        i32.store16 offset=27 align=1
        local.get 2
        i32.const 27
        i32.add
        i32.const 2
        call $wasi_snapshot_preview1::macros::print
        local.get 2
        i32.const 174417007
        i32.store offset=35 align=1
        local.get 2
        i64.const 7863410729224140130
        i64.store offset=27 align=1
        local.get 2
        i32.const 27
        i32.add
        i32.const 12
        call $wasi_snapshot_preview1::macros::print
        local.get 2
        i32.const 10
        i32.store8 offset=27
        local.get 2
        i32.const 27
        i32.add
        i32.const 1
        call $wasi_snapshot_preview1::macros::print
        unreachable
        unreachable
      end
      local.get 2
      i32.const 32
      i32.store8 offset=63
      local.get 2
      i32.const 1701734764
      i32.store offset=59 align=1
      local.get 2
      i64.const 2338042707334751329
      i64.store offset=51 align=1
      local.get 2
      i64.const 2338600898263348341
      i64.store offset=43 align=1
      local.get 2
      i64.const 7162263158133189730
      i64.store offset=35 align=1
      local.get 2
      i64.const 7018969289221893749
      i64.store offset=27 align=1
      local.get 2
      i32.const 27
      i32.add
      i32.const 37
      call $wasi_snapshot_preview1::macros::print
      i32.const 205
      call $wasi_snapshot_preview1::macros::eprint_u32
      local.get 2
      i32.const 8250
      i32.store16 offset=27 align=1
      local.get 2
      i32.const 27
      i32.add
      i32.const 2
      call $wasi_snapshot_preview1::macros::print
      local.get 2
      i64.const 748000395109933170
      i64.store offset=43 align=1
      local.get 2
      i64.const 7307218417350680677
      i64.store offset=35 align=1
      local.get 2
      i64.const 8390050488160450159
      i64.store offset=27 align=1
      local.get 2
      i32.const 27
      i32.add
      i32.const 24
      call $wasi_snapshot_preview1::macros::print
      local.get 2
      i32.const 10
      i32.store8 offset=27
      local.get 2
      i32.const 27
      i32.add
      i32.const 1
      call $wasi_snapshot_preview1::macros::print
      unreachable
      unreachable
    )
    (func $environ_sizes_get (;36;) (type 12) (param i32 i32) (result i32)
      (local i32 i32 i32 i32)
      call $allocate_stack
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              call $get_allocation_state
              i32.const -2
              i32.add
              i32.const -3
              i32.and
              i32.eqz
              br_if 0 (;@4;)
              i32.const 0
              local.set 3
              local.get 0
              i32.const 0
              i32.store
              br 1 (;@3;)
            end
            call $wasi_snapshot_preview1::State::ptr
            local.tee 3
            i32.load
            i32.const 560490357
            i32.ne
            br_if 1 (;@2;)
            local.get 3
            i32.load offset=65532
            i32.const 560490357
            i32.ne
            br_if 2 (;@1;)
            local.get 2
            local.get 3
            call $wasi_snapshot_preview1::State::get_environment
            local.get 2
            i32.load
            local.set 4
            local.get 0
            local.get 2
            i32.load offset=4
            local.tee 3
            i32.store
            block ;; label = @4
              local.get 3
              br_if 0 (;@4;)
              i32.const 0
              local.set 3
              br 1 (;@3;)
            end
            local.get 3
            i32.const 4
            i32.shl
            local.set 5
            local.get 4
            i32.const 12
            i32.add
            local.set 0
            i32.const 0
            local.set 3
            loop ;; label = @4
              local.get 3
              local.get 0
              i32.const -8
              i32.add
              i32.load
              i32.add
              local.get 0
              i32.load
              i32.add
              i32.const 2
              i32.add
              local.set 3
              local.get 0
              i32.const 16
              i32.add
              local.set 0
              local.get 5
              i32.const -16
              i32.add
              local.tee 5
              br_if 0 (;@4;)
            end
          end
          local.get 1
          local.get 3
          i32.store
          local.get 2
          i32.const 48
          i32.add
          global.set $__stack_pointer
          i32.const 0
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
        i32.const 2404
        call $wasi_snapshot_preview1::macros::eprint_u32
        local.get 2
        i32.const 8250
        i32.store16 offset=11 align=1
        local.get 2
        i32.const 11
        i32.add
        i32.const 2
        call $wasi_snapshot_preview1::macros::print
        local.get 2
        i32.const 10
        i32.store8 offset=27
        local.get 2
        i64.const 7234307576302018670
        i64.store offset=19 align=1
        local.get 2
        i64.const 8028075845441778529
        i64.store offset=11 align=1
        local.get 2
        i32.const 11
        i32.add
        i32.const 17
        call $wasi_snapshot_preview1::macros::print
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
      i32.const 2405
      call $wasi_snapshot_preview1::macros::eprint_u32
      local.get 2
      i32.const 8250
      i32.store16 offset=11 align=1
      local.get 2
      i32.const 11
      i32.add
      i32.const 2
      call $wasi_snapshot_preview1::macros::print
      local.get 2
      i32.const 10
      i32.store8 offset=27
      local.get 2
      i64.const 7234307576302018670
      i64.store offset=19 align=1
      local.get 2
      i64.const 8028075845441778529
      i64.store offset=11 align=1
      local.get 2
      i32.const 11
      i32.add
      i32.const 17
      call $wasi_snapshot_preview1::macros::print
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
    (func $wasi_snapshot_preview1::State::descriptors (;37;) (type 2) (param i32 i32)
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
        i32.const 2493
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
      i32.const 2497
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
    (func $fd_close (;38;) (type 6) (param i32) (result i32)
      (local i32 i32 i32 i32)
      call $allocate_stack
      global.get $__stack_pointer
      i32.const 64
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          call $wasi_snapshot_preview1::State::ptr
          local.tee 2
          i32.load
          i32.const 560490357
          i32.ne
          br_if 0 (;@2;)
          local.get 2
          i32.load offset=65532
          i32.const 560490357
          i32.ne
          br_if 1 (;@1;)
          block ;; label = @3
            local.get 2
            i32.const 65520
            i32.add
            i32.load
            local.get 0
            i32.ne
            br_if 0 (;@3;)
            local.get 2
            i32.const 65480
            i32.add
            local.tee 3
            i32.load
            local.set 4
            local.get 3
            i32.const 0
            i32.store
            local.get 4
            i32.eqz
            br_if 0 (;@3;)
            local.get 2
            i32.const 65484
            i32.add
            i32.load
            call $<wasi_snapshot_preview1::bindings::wasi::filesystem::types::DirectoryEntryStream as wit_bindgen::WasmResource>::drop::drop
          end
          local.get 1
          i32.const 16
          i32.add
          local.get 2
          call $wasi_snapshot_preview1::State::descriptors_mut
          local.get 1
          i32.load offset=20
          local.set 2
          local.get 1
          i32.const 8
          i32.add
          local.get 1
          i32.load offset=16
          local.get 0
          call $wasi_snapshot_preview1::descriptors::Descriptors::close
          local.get 1
          i32.load16_u offset=10
          local.set 0
          local.get 1
          i32.load16_u offset=8
          local.set 3
          local.get 2
          local.get 2
          i32.load
          i32.const 1
          i32.add
          i32.store
          local.get 1
          i32.const 64
          i32.add
          global.set $__stack_pointer
          local.get 0
          i32.const 0
          local.get 3
          select
          i32.const 65535
          i32.and
          return
        end
        local.get 1
        i32.const 32
        i32.store8 offset=63
        local.get 1
        i32.const 1701734764
        i32.store offset=59 align=1
        local.get 1
        i64.const 2338042707334751329
        i64.store offset=51 align=1
        local.get 1
        i64.const 2338600898263348341
        i64.store offset=43 align=1
        local.get 1
        i64.const 7162263158133189730
        i64.store offset=35 align=1
        local.get 1
        i64.const 7018969289221893749
        i64.store offset=27 align=1
        local.get 1
        i32.const 27
        i32.add
        i32.const 37
        call $wasi_snapshot_preview1::macros::print
        i32.const 2404
        call $wasi_snapshot_preview1::macros::eprint_u32
        local.get 1
        i32.const 8250
        i32.store16 offset=27 align=1
        local.get 1
        i32.const 27
        i32.add
        i32.const 2
        call $wasi_snapshot_preview1::macros::print
        local.get 1
        i32.const 10
        i32.store8 offset=43
        local.get 1
        i64.const 7234307576302018670
        i64.store offset=35 align=1
        local.get 1
        i64.const 8028075845441778529
        i64.store offset=27 align=1
        local.get 1
        i32.const 27
        i32.add
        i32.const 17
        call $wasi_snapshot_preview1::macros::print
        local.get 1
        i32.const 10
        i32.store8 offset=27
        local.get 1
        i32.const 27
        i32.add
        i32.const 1
        call $wasi_snapshot_preview1::macros::print
        unreachable
        unreachable
      end
      local.get 1
      i32.const 32
      i32.store8 offset=63
      local.get 1
      i32.const 1701734764
      i32.store offset=59 align=1
      local.get 1
      i64.const 2338042707334751329
      i64.store offset=51 align=1
      local.get 1
      i64.const 2338600898263348341
      i64.store offset=43 align=1
      local.get 1
      i64.const 7162263158133189730
      i64.store offset=35 align=1
      local.get 1
      i64.const 7018969289221893749
      i64.store offset=27 align=1
      local.get 1
      i32.const 27
      i32.add
      i32.const 37
      call $wasi_snapshot_preview1::macros::print
      i32.const 2405
      call $wasi_snapshot_preview1::macros::eprint_u32
      local.get 1
      i32.const 8250
      i32.store16 offset=27 align=1
      local.get 1
      i32.const 27
      i32.add
      i32.const 2
      call $wasi_snapshot_preview1::macros::print
      local.get 1
      i32.const 10
      i32.store8 offset=43
      local.get 1
      i64.const 7234307576302018670
      i64.store offset=35 align=1
      local.get 1
      i64.const 8028075845441778529
      i64.store offset=27 align=1
      local.get 1
      i32.const 27
      i32.add
      i32.const 17
      call $wasi_snapshot_preview1::macros::print
      local.get 1
      i32.const 10
      i32.store8 offset=27
      local.get 1
      i32.const 27
      i32.add
      i32.const 1
      call $wasi_snapshot_preview1::macros::print
      unreachable
      unreachable
    )
    (func $wasi_snapshot_preview1::State::descriptors_mut (;39;) (type 2) (param i32 i32)
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
        i32.const 2505
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
      i32.const 2509
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
    (func $fd_prestat_get (;40;) (type 12) (param i32 i32) (result i32)
      (local i32 i32 i32)
      call $allocate_stack
      global.get $__stack_pointer
      i32.const 64
      i32.sub
      local.tee 2
      global.set $__stack_pointer
      i32.const 8
      local.set 3
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            call $get_allocation_state
            i32.const -2
            i32.add
            i32.const -3
            i32.and
            br_if 0 (;@3;)
            call $wasi_snapshot_preview1::State::ptr
            local.tee 3
            i32.load
            i32.const 560490357
            i32.ne
            br_if 1 (;@2;)
            local.get 3
            i32.load offset=65532
            i32.const 560490357
            i32.ne
            br_if 2 (;@1;)
            local.get 2
            i32.const 16
            i32.add
            local.get 3
            call $wasi_snapshot_preview1::State::descriptors
            local.get 2
            i32.load offset=20
            local.set 4
            local.get 2
            i32.const 8
            i32.add
            local.get 2
            i32.load offset=16
            local.tee 3
            i32.load offset=6156
            local.get 3
            i32.const 6160
            i32.add
            i32.load
            call $<core::option::Option<T> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap
            i32.const 8
            local.set 3
            block ;; label = @4
              local.get 0
              i32.const 3
              i32.lt_u
              br_if 0 (;@4;)
              local.get 2
              i32.load offset=8
              local.get 0
              i32.const -3
              i32.add
              local.tee 0
              i32.const 12
              i32.mul
              i32.add
              i32.const 0
              local.get 0
              local.get 2
              i32.load offset=12
              i32.lt_u
              select
              local.tee 0
              i32.eqz
              br_if 0 (;@4;)
              local.get 1
              local.get 0
              i32.const 8
              i32.add
              i64.load32_u
              i64.const 32
              i64.shl
              i64.store align=4
              i32.const 0
              local.set 3
            end
            local.get 4
            local.get 4
            i32.load
            i32.const 1
            i32.add
            i32.store
          end
          local.get 2
          i32.const 64
          i32.add
          global.set $__stack_pointer
          local.get 3
          return
        end
        local.get 2
        i32.const 32
        i32.store8 offset=63
        local.get 2
        i32.const 1701734764
        i32.store offset=59 align=1
        local.get 2
        i64.const 2338042707334751329
        i64.store offset=51 align=1
        local.get 2
        i64.const 2338600898263348341
        i64.store offset=43 align=1
        local.get 2
        i64.const 7162263158133189730
        i64.store offset=35 align=1
        local.get 2
        i64.const 7018969289221893749
        i64.store offset=27 align=1
        local.get 2
        i32.const 27
        i32.add
        i32.const 37
        call $wasi_snapshot_preview1::macros::print
        i32.const 2404
        call $wasi_snapshot_preview1::macros::eprint_u32
        local.get 2
        i32.const 8250
        i32.store16 offset=27 align=1
        local.get 2
        i32.const 27
        i32.add
        i32.const 2
        call $wasi_snapshot_preview1::macros::print
        local.get 2
        i32.const 10
        i32.store8 offset=43
        local.get 2
        i64.const 7234307576302018670
        i64.store offset=35 align=1
        local.get 2
        i64.const 8028075845441778529
        i64.store offset=27 align=1
        local.get 2
        i32.const 27
        i32.add
        i32.const 17
        call $wasi_snapshot_preview1::macros::print
        local.get 2
        i32.const 10
        i32.store8 offset=27
        local.get 2
        i32.const 27
        i32.add
        i32.const 1
        call $wasi_snapshot_preview1::macros::print
        unreachable
        unreachable
      end
      local.get 2
      i32.const 32
      i32.store8 offset=63
      local.get 2
      i32.const 1701734764
      i32.store offset=59 align=1
      local.get 2
      i64.const 2338042707334751329
      i64.store offset=51 align=1
      local.get 2
      i64.const 2338600898263348341
      i64.store offset=43 align=1
      local.get 2
      i64.const 7162263158133189730
      i64.store offset=35 align=1
      local.get 2
      i64.const 7018969289221893749
      i64.store offset=27 align=1
      local.get 2
      i32.const 27
      i32.add
      i32.const 37
      call $wasi_snapshot_preview1::macros::print
      i32.const 2405
      call $wasi_snapshot_preview1::macros::eprint_u32
      local.get 2
      i32.const 8250
      i32.store16 offset=27 align=1
      local.get 2
      i32.const 27
      i32.add
      i32.const 2
      call $wasi_snapshot_preview1::macros::print
      local.get 2
      i32.const 10
      i32.store8 offset=43
      local.get 2
      i64.const 7234307576302018670
      i64.store offset=35 align=1
      local.get 2
      i64.const 8028075845441778529
      i64.store offset=27 align=1
      local.get 2
      i32.const 27
      i32.add
      i32.const 17
      call $wasi_snapshot_preview1::macros::print
      local.get 2
      i32.const 10
      i32.store8 offset=27
      local.get 2
      i32.const 27
      i32.add
      i32.const 1
      call $wasi_snapshot_preview1::macros::print
      unreachable
      unreachable
    )
    (func $fd_prestat_dir_name (;41;) (type 11) (param i32 i32 i32) (result i32)
      (local i32 i32 i32 i32)
      call $allocate_stack
      global.get $__stack_pointer
      i32.const 64
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          call $wasi_snapshot_preview1::State::ptr
          local.tee 4
          i32.load
          i32.const 560490357
          i32.ne
          br_if 0 (;@2;)
          local.get 4
          i32.load offset=65532
          i32.const 560490357
          i32.ne
          br_if 1 (;@1;)
          local.get 3
          i32.const 16
          i32.add
          local.get 4
          call $wasi_snapshot_preview1::State::descriptors
          local.get 3
          i32.load offset=20
          local.set 4
          local.get 3
          i32.const 8
          i32.add
          local.get 3
          i32.load offset=16
          local.tee 5
          i32.load offset=6156
          local.get 5
          i32.const 6160
          i32.add
          i32.load
          call $<core::option::Option<T> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap
          i32.const 54
          local.set 5
          block ;; label = @3
            local.get 0
            i32.const 3
            i32.lt_u
            br_if 0 (;@3;)
            local.get 3
            i32.load offset=8
            local.get 0
            i32.const -3
            i32.add
            local.tee 0
            i32.const 12
            i32.mul
            i32.add
            i32.const 0
            local.get 0
            local.get 3
            i32.load offset=12
            i32.lt_u
            select
            local.tee 0
            i32.eqz
            br_if 0 (;@3;)
            i32.const 37
            local.set 5
            local.get 0
            i32.const 8
            i32.add
            i32.load
            local.tee 6
            local.get 2
            i32.gt_u
            br_if 0 (;@3;)
            local.get 1
            local.get 0
            i32.load offset=4
            local.get 6
            call $memcpy
            drop
            i32.const 0
            local.set 5
          end
          local.get 4
          local.get 4
          i32.load
          i32.const 1
          i32.add
          i32.store
          local.get 3
          i32.const 64
          i32.add
          global.set $__stack_pointer
          local.get 5
          return
        end
        local.get 3
        i32.const 32
        i32.store8 offset=63
        local.get 3
        i32.const 1701734764
        i32.store offset=59 align=1
        local.get 3
        i64.const 2338042707334751329
        i64.store offset=51 align=1
        local.get 3
        i64.const 2338600898263348341
        i64.store offset=43 align=1
        local.get 3
        i64.const 7162263158133189730
        i64.store offset=35 align=1
        local.get 3
        i64.const 7018969289221893749
        i64.store offset=27 align=1
        local.get 3
        i32.const 27
        i32.add
        i32.const 37
        call $wasi_snapshot_preview1::macros::print
        i32.const 2404
        call $wasi_snapshot_preview1::macros::eprint_u32
        local.get 3
        i32.const 8250
        i32.store16 offset=27 align=1
        local.get 3
        i32.const 27
        i32.add
        i32.const 2
        call $wasi_snapshot_preview1::macros::print
        local.get 3
        i32.const 10
        i32.store8 offset=43
        local.get 3
        i64.const 7234307576302018670
        i64.store offset=35 align=1
        local.get 3
        i64.const 8028075845441778529
        i64.store offset=27 align=1
        local.get 3
        i32.const 27
        i32.add
        i32.const 17
        call $wasi_snapshot_preview1::macros::print
        local.get 3
        i32.const 10
        i32.store8 offset=27
        local.get 3
        i32.const 27
        i32.add
        i32.const 1
        call $wasi_snapshot_preview1::macros::print
        unreachable
        unreachable
      end
      local.get 3
      i32.const 32
      i32.store8 offset=63
      local.get 3
      i32.const 1701734764
      i32.store offset=59 align=1
      local.get 3
      i64.const 2338042707334751329
      i64.store offset=51 align=1
      local.get 3
      i64.const 2338600898263348341
      i64.store offset=43 align=1
      local.get 3
      i64.const 7162263158133189730
      i64.store offset=35 align=1
      local.get 3
      i64.const 7018969289221893749
      i64.store offset=27 align=1
      local.get 3
      i32.const 27
      i32.add
      i32.const 37
      call $wasi_snapshot_preview1::macros::print
      i32.const 2405
      call $wasi_snapshot_preview1::macros::eprint_u32
      local.get 3
      i32.const 8250
      i32.store16 offset=27 align=1
      local.get 3
      i32.const 27
      i32.add
      i32.const 2
      call $wasi_snapshot_preview1::macros::print
      local.get 3
      i32.const 10
      i32.store8 offset=43
      local.get 3
      i64.const 7234307576302018670
      i64.store offset=35 align=1
      local.get 3
      i64.const 8028075845441778529
      i64.store offset=27 align=1
      local.get 3
      i32.const 27
      i32.add
      i32.const 17
      call $wasi_snapshot_preview1::macros::print
      local.get 3
      i32.const 10
      i32.store8 offset=27
      local.get 3
      i32.const 27
      i32.add
      i32.const 1
      call $wasi_snapshot_preview1::macros::print
      unreachable
      unreachable
    )
    (func $wasi_snapshot_preview1::stream_error_to_errno (;42;) (type 6) (param i32) (result i32)
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
    (func $fd_write (;43;) (type 8) (param i32 i32 i32 i32) (result i32)
      (local i32 i32 i32 i32 i32)
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
                block ;; label = @6
                  call $get_allocation_state
                  i32.const -2
                  i32.add
                  i32.const -3
                  i32.and
                  br_if 0 (;@6;)
                  block ;; label = @7
                    block ;; label = @8
                      local.get 2
                      i32.eqz
                      br_if 0 (;@8;)
                      loop ;; label = @9
                        local.get 1
                        i32.const 4
                        i32.add
                        i32.load
                        local.tee 5
                        br_if 2 (;@7;)
                        local.get 1
                        i32.const 8
                        i32.add
                        local.set 1
                        local.get 2
                        i32.const -1
                        i32.add
                        local.tee 2
                        br_if 0 (;@9;)
                      end
                    end
                    i32.const 0
                    local.set 1
                    local.get 3
                    i32.const 0
                    i32.store
                    br 6 (;@1;)
                  end
                  local.get 1
                  i32.load
                  local.set 6
                  call $wasi_snapshot_preview1::State::ptr
                  local.tee 1
                  i32.load
                  i32.const 560490357
                  i32.ne
                  br_if 1 (;@5;)
                  local.get 1
                  i32.load offset=65532
                  i32.const 560490357
                  i32.ne
                  br_if 2 (;@4;)
                  local.get 4
                  local.get 1
                  call $wasi_snapshot_preview1::State::descriptors
                  local.get 4
                  i32.load
                  local.tee 7
                  i32.load16_u offset=6144
                  local.set 8
                  local.get 4
                  i32.load offset=4
                  local.set 2
                  i32.const 8
                  local.set 1
                  i32.const 0
                  local.get 0
                  call $<core::result::Result<T,E> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap
                  local.tee 0
                  local.get 8
                  i32.ge_u
                  br_if 4 (;@2;)
                  local.get 7
                  local.get 0
                  i32.const 48
                  i32.mul
                  i32.add
                  local.tee 0
                  i32.load
                  i32.eqz
                  br_if 4 (;@2;)
                  local.get 4
                  i32.const 8
                  i32.add
                  local.get 0
                  i32.const 8
                  i32.add
                  call $wasi_snapshot_preview1::descriptors::Streams::get_write_stream
                  block ;; label = @7
                    local.get 4
                    i32.load16_u offset=8
                    br_if 0 (;@7;)
                    local.get 4
                    i32.load offset=12
                    local.set 1
                    block ;; label = @8
                      block ;; label = @9
                        local.get 0
                        i32.const 41
                        i32.add
                        i32.load8_u
                        local.tee 7
                        i32.const -2
                        i32.add
                        i32.const 255
                        i32.and
                        local.tee 8
                        i32.const 2
                        i32.gt_u
                        br_if 0 (;@9;)
                        local.get 8
                        i32.const 1
                        i32.ne
                        br_if 1 (;@8;)
                      end
                      local.get 4
                      i32.const 8
                      i32.add
                      local.get 7
                      i32.const 255
                      i32.and
                      i32.const 0
                      i32.ne
                      local.get 1
                      local.get 6
                      local.get 5
                      call $wasi_snapshot_preview1::BlockingMode::write
                      local.get 4
                      i32.load16_u offset=8
                      br_if 1 (;@7;)
                      br 5 (;@3;)
                    end
                    local.get 4
                    i32.const 8
                    i32.add
                    i32.const 1
                    local.get 1
                    local.get 6
                    local.get 5
                    call $wasi_snapshot_preview1::BlockingMode::write
                    local.get 4
                    i32.load16_u offset=8
                    i32.eqz
                    br_if 4 (;@3;)
                  end
                  local.get 4
                  i32.load16_u offset=10
                  local.set 1
                  br 4 (;@2;)
                end
                local.get 3
                i32.const 0
                i32.store
                i32.const 29
                local.set 1
                br 4 (;@1;)
              end
              local.get 4
              i32.const 32
              i32.store8 offset=44
              local.get 4
              i32.const 1701734764
              i32.store offset=40 align=1
              local.get 4
              i64.const 2338042707334751329
              i64.store offset=32 align=1
              local.get 4
              i64.const 2338600898263348341
              i64.store offset=24 align=1
              local.get 4
              i64.const 7162263158133189730
              i64.store offset=16 align=1
              local.get 4
              i64.const 7018969289221893749
              i64.store offset=8 align=1
              local.get 4
              i32.const 8
              i32.add
              i32.const 37
              call $wasi_snapshot_preview1::macros::print
              i32.const 2404
              call $wasi_snapshot_preview1::macros::eprint_u32
              local.get 4
              i32.const 8250
              i32.store16 offset=8 align=1
              local.get 4
              i32.const 8
              i32.add
              i32.const 2
              call $wasi_snapshot_preview1::macros::print
              local.get 4
              i32.const 10
              i32.store8 offset=24
              local.get 4
              i64.const 7234307576302018670
              i64.store offset=16 align=1
              local.get 4
              i64.const 8028075845441778529
              i64.store offset=8 align=1
              local.get 4
              i32.const 8
              i32.add
              i32.const 17
              call $wasi_snapshot_preview1::macros::print
              local.get 4
              i32.const 10
              i32.store8 offset=8
              local.get 4
              i32.const 8
              i32.add
              i32.const 1
              call $wasi_snapshot_preview1::macros::print
              unreachable
              unreachable
            end
            local.get 4
            i32.const 32
            i32.store8 offset=44
            local.get 4
            i32.const 1701734764
            i32.store offset=40 align=1
            local.get 4
            i64.const 2338042707334751329
            i64.store offset=32 align=1
            local.get 4
            i64.const 2338600898263348341
            i64.store offset=24 align=1
            local.get 4
            i64.const 7162263158133189730
            i64.store offset=16 align=1
            local.get 4
            i64.const 7018969289221893749
            i64.store offset=8 align=1
            local.get 4
            i32.const 8
            i32.add
            i32.const 37
            call $wasi_snapshot_preview1::macros::print
            i32.const 2405
            call $wasi_snapshot_preview1::macros::eprint_u32
            local.get 4
            i32.const 8250
            i32.store16 offset=8 align=1
            local.get 4
            i32.const 8
            i32.add
            i32.const 2
            call $wasi_snapshot_preview1::macros::print
            local.get 4
            i32.const 10
            i32.store8 offset=24
            local.get 4
            i64.const 7234307576302018670
            i64.store offset=16 align=1
            local.get 4
            i64.const 8028075845441778529
            i64.store offset=8 align=1
            local.get 4
            i32.const 8
            i32.add
            i32.const 17
            call $wasi_snapshot_preview1::macros::print
            local.get 4
            i32.const 10
            i32.store8 offset=8
            local.get 4
            i32.const 8
            i32.add
            i32.const 1
            call $wasi_snapshot_preview1::macros::print
            unreachable
            unreachable
          end
          local.get 4
          i32.load offset=12
          local.set 1
          block ;; label = @3
            block ;; label = @4
              local.get 0
              i32.load8_u offset=41
              i32.const -2
              i32.add
              i32.const 255
              i32.and
              local.tee 5
              i32.const 2
              i32.gt_u
              br_if 0 (;@4;)
              local.get 5
              i32.const 1
              i32.ne
              br_if 1 (;@3;)
            end
            local.get 0
            i32.const 40
            i32.add
            i32.load8_u
            br_if 0 (;@3;)
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
          end
          local.get 3
          local.get 1
          i32.store
          i32.const 0
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
      i32.const 48
      i32.add
      global.set $__stack_pointer
      local.get 1
      i32.const 65535
      i32.and
    )
    (func $wasi_snapshot_preview1::BlockingMode::write (;44;) (type 9) (param i32 i32 i32 i32 i32)
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
    (func $path_open (;45;) (type 13) (param i32 i32 i32 i32 i32 i64 i64 i32 i32) (result i32)
      (local i32 i32 i32)
      call $allocate_stack
      global.get $__stack_pointer
      i32.const 80
      i32.sub
      local.tee 9
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                call $wasi_snapshot_preview1::State::ptr
                local.tee 10
                i32.load
                i32.const 560490357
                i32.ne
                br_if 0 (;@5;)
                local.get 10
                i32.load offset=65532
                i32.const 560490357
                i32.ne
                br_if 1 (;@4;)
                local.get 9
                i32.const 8
                i32.add
                local.get 10
                call $wasi_snapshot_preview1::State::descriptors
                local.get 9
                i32.load offset=12
                local.set 11
                local.get 9
                i32.const 72
                i32.add
                local.get 9
                i32.load offset=8
                local.get 0
                call $wasi_snapshot_preview1::descriptors::Descriptors::get_dir
                block ;; label = @6
                  block ;; label = @7
                    local.get 9
                    i32.load16_u offset=72
                    br_if 0 (;@7;)
                    local.get 9
                    i32.const 16
                    i32.add
                    local.get 9
                    i32.load offset=76
                    local.get 1
                    i32.const 1
                    i32.and
                    local.get 2
                    local.get 3
                    local.get 4
                    i32.const 15
                    i32.and
                    local.get 5
                    i32.wrap_i64
                    local.tee 0
                    i32.const 5
                    i32.shr_u
                    i32.const 2
                    i32.and
                    local.get 0
                    i32.const 1
                    i32.shr_u
                    i32.const 1
                    i32.and
                    i32.or
                    local.get 7
                    i32.const 2
                    i32.shr_u
                    i32.const 4
                    i32.and
                    i32.or
                    local.get 7
                    i32.const 2
                    i32.shl
                    i32.const 8
                    i32.and
                    i32.or
                    local.get 7
                    i32.const 1
                    i32.shl
                    i32.const 16
                    i32.and
                    i32.or
                    call $wasi_snapshot_preview1::bindings::wasi::filesystem::types::Descriptor::open_at
                    local.get 9
                    i32.load8_u offset=16
                    br_if 4 (;@3;)
                    local.get 9
                    i32.load offset=20
                    local.set 4
                    local.get 11
                    local.get 11
                    i32.load
                    i32.const 1
                    i32.add
                    i32.store
                    local.get 4
                    local.get 9
                    i32.const 72
                    i32.add
                    call $wasi_snapshot_preview1::bindings::wasi::filesystem::types::Descriptor::get_type::wit_import
                    local.get 9
                    i32.load8_u offset=73
                    local.set 11
                    block ;; label = @8
                      local.get 9
                      i32.load8_u offset=72
                      br_if 0 (;@8;)
                      local.get 9
                      i32.const 64
                      i32.add
                      local.get 7
                      i32.const 1
                      i32.and
                      i32.store8
                      local.get 9
                      i32.const 56
                      i32.add
                      i64.const 0
                      i64.store
                      local.get 9
                      i32.const 52
                      i32.add
                      local.get 11
                      i32.store8
                      local.get 9
                      i32.const 48
                      i32.add
                      local.get 4
                      i32.store
                      i32.const 0
                      local.set 0
                      local.get 9
                      i32.const 40
                      i32.add
                      i32.const 0
                      i32.store
                      local.get 9
                      i32.const 65
                      i32.add
                      local.get 7
                      i32.const 4
                      i32.and
                      i32.eqz
                      i32.store8
                      local.get 9
                      i32.const 0
                      i32.store offset=32
                      local.get 9
                      i32.const 1
                      i32.store offset=24
                      local.get 9
                      local.get 10
                      call $wasi_snapshot_preview1::State::descriptors_mut
                      local.get 9
                      i32.load offset=4
                      local.set 10
                      local.get 9
                      i32.const 72
                      i32.add
                      local.get 9
                      i32.load
                      local.get 9
                      i32.const 24
                      i32.add
                      call $wasi_snapshot_preview1::descriptors::Descriptors::open
                      local.get 9
                      i32.load16_u offset=72
                      i32.eqz
                      br_if 2 (;@6;)
                      local.get 9
                      i32.load16_u offset=74
                      local.set 0
                      local.get 10
                      local.get 10
                      i32.load
                      i32.const 1
                      i32.add
                      i32.store
                      br 7 (;@1;)
                    end
                    local.get 11
                    call $wasi_snapshot_preview1::<impl core::convert::From<wasi_snapshot_preview1::bindings::wasi::filesystem::types::ErrorCode> for wasi::lib_generated::Errno>::from
                    local.set 0
                    local.get 4
                    call $<wasi_snapshot_preview1::bindings::wasi::filesystem::types::Descriptor as wit_bindgen::WasmResource>::drop::drop
                    br 6 (;@1;)
                  end
                  local.get 9
                  i32.load16_u offset=74
                  local.set 0
                  br 4 (;@2;)
                end
                local.get 9
                i32.load offset=76
                local.set 11
                local.get 10
                local.get 10
                i32.load
                i32.const 1
                i32.add
                i32.store
                local.get 8
                local.get 11
                i32.store
                br 4 (;@1;)
              end
              local.get 9
              i32.const 32
              i32.store8 offset=60
              local.get 9
              i32.const 1701734764
              i32.store offset=56 align=1
              local.get 9
              i64.const 2338042707334751329
              i64.store offset=48 align=1
              local.get 9
              i64.const 2338600898263348341
              i64.store offset=40 align=1
              local.get 9
              i64.const 7162263158133189730
              i64.store offset=32 align=1
              local.get 9
              i64.const 7018969289221893749
              i64.store offset=24 align=1
              local.get 9
              i32.const 24
              i32.add
              i32.const 37
              call $wasi_snapshot_preview1::macros::print
              i32.const 2404
              call $wasi_snapshot_preview1::macros::eprint_u32
              local.get 9
              i32.const 8250
              i32.store16 offset=24 align=1
              local.get 9
              i32.const 24
              i32.add
              i32.const 2
              call $wasi_snapshot_preview1::macros::print
              local.get 9
              i32.const 10
              i32.store8 offset=40
              local.get 9
              i64.const 7234307576302018670
              i64.store offset=32 align=1
              local.get 9
              i64.const 8028075845441778529
              i64.store offset=24 align=1
              local.get 9
              i32.const 24
              i32.add
              i32.const 17
              call $wasi_snapshot_preview1::macros::print
              local.get 9
              i32.const 10
              i32.store8 offset=24
              local.get 9
              i32.const 24
              i32.add
              i32.const 1
              call $wasi_snapshot_preview1::macros::print
              unreachable
              unreachable
            end
            local.get 9
            i32.const 32
            i32.store8 offset=60
            local.get 9
            i32.const 1701734764
            i32.store offset=56 align=1
            local.get 9
            i64.const 2338042707334751329
            i64.store offset=48 align=1
            local.get 9
            i64.const 2338600898263348341
            i64.store offset=40 align=1
            local.get 9
            i64.const 7162263158133189730
            i64.store offset=32 align=1
            local.get 9
            i64.const 7018969289221893749
            i64.store offset=24 align=1
            local.get 9
            i32.const 24
            i32.add
            i32.const 37
            call $wasi_snapshot_preview1::macros::print
            i32.const 2405
            call $wasi_snapshot_preview1::macros::eprint_u32
            local.get 9
            i32.const 8250
            i32.store16 offset=24 align=1
            local.get 9
            i32.const 24
            i32.add
            i32.const 2
            call $wasi_snapshot_preview1::macros::print
            local.get 9
            i32.const 10
            i32.store8 offset=40
            local.get 9
            i64.const 7234307576302018670
            i64.store offset=32 align=1
            local.get 9
            i64.const 8028075845441778529
            i64.store offset=24 align=1
            local.get 9
            i32.const 24
            i32.add
            i32.const 17
            call $wasi_snapshot_preview1::macros::print
            local.get 9
            i32.const 10
            i32.store8 offset=24
            local.get 9
            i32.const 24
            i32.add
            i32.const 1
            call $wasi_snapshot_preview1::macros::print
            unreachable
            unreachable
          end
          local.get 9
          i32.load8_u offset=17
          call $wasi_snapshot_preview1::<impl core::convert::From<wasi_snapshot_preview1::bindings::wasi::filesystem::types::ErrorCode> for wasi::lib_generated::Errno>::from
          local.set 0
        end
        local.get 11
        local.get 11
        i32.load
        i32.const 1
        i32.add
        i32.store
      end
      local.get 9
      i32.const 80
      i32.add
      global.set $__stack_pointer
      local.get 0
      i32.const 65535
      i32.and
    )
    (func $proc_exit (;46;) (type 1) (param i32)
      (local i32)
      call $allocate_stack
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      local.get 0
      i32.const 0
      i32.ne
      call $wasi_snapshot_preview1::bindings::wasi::cli::exit::exit
      local.get 1
      i32.const 32
      i32.store8 offset=46
      local.get 1
      i32.const 1701734764
      i32.store offset=42 align=1
      local.get 1
      i64.const 2338042707334751329
      i64.store offset=34 align=1
      local.get 1
      i64.const 2338600898263348341
      i64.store offset=26 align=1
      local.get 1
      i64.const 7162263158133189730
      i64.store offset=18 align=1
      local.get 1
      i64.const 7018969289221893749
      i64.store offset=10 align=1
      local.get 1
      i32.const 10
      i32.add
      i32.const 37
      call $wasi_snapshot_preview1::macros::print
      i32.const 1938
      call $wasi_snapshot_preview1::macros::eprint_u32
      local.get 1
      i32.const 8250
      i32.store16 offset=10 align=1
      local.get 1
      i32.const 10
      i32.add
      i32.const 2
      call $wasi_snapshot_preview1::macros::print
      local.get 1
      i32.const 2593
      i32.store16 offset=46 align=1
      local.get 1
      i32.const 1953069157
      i32.store offset=42 align=1
      local.get 1
      i64.const 2338537461596644384
      i64.store offset=34 align=1
      local.get 1
      i64.const 7957695015159098981
      i64.store offset=26 align=1
      local.get 1
      i64.const 7882825952909664372
      i64.store offset=18 align=1
      local.get 1
      i64.const 7599935561254793064
      i64.store offset=10 align=1
      local.get 1
      i32.const 10
      i32.add
      i32.const 38
      call $wasi_snapshot_preview1::macros::print
      local.get 1
      i32.const 10
      i32.store8 offset=10
      local.get 1
      i32.const 10
      i32.add
      i32.const 1
      call $wasi_snapshot_preview1::macros::print
      unreachable
      unreachable
    )
    (func $wasi_snapshot_preview1::State::new (;47;) (type 10) (result i32)
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
      i32.const 2436
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
    (func $wasi_snapshot_preview1::bindings::wasi::filesystem::types::Descriptor::open_at (;48;) (type 5) (param i32 i32 i32 i32 i32 i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      local.tee 7
      global.set $__stack_pointer
      local.get 1
      i32.load
      local.get 2
      i32.const 255
      i32.and
      local.get 3
      local.get 4
      local.get 5
      i32.const 255
      i32.and
      local.get 6
      i32.const 255
      i32.and
      local.get 7
      i32.const 8
      i32.add
      call $wasi_snapshot_preview1::bindings::wasi::filesystem::types::Descriptor::open_at::wit_import
      block ;; label = @1
        block ;; label = @2
          local.get 7
          i32.load8_u offset=8
          local.tee 6
          br_if 0 (;@2;)
          local.get 0
          local.get 7
          i32.const 12
          i32.add
          i32.load
          i32.store offset=4
          br 1 (;@1;)
        end
        local.get 0
        local.get 7
        i32.const 12
        i32.add
        i32.load8_u
        i32.store8 offset=1
      end
      local.get 0
      local.get 6
      i32.store8
      local.get 7
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $wasi_snapshot_preview1::macros::print (;49;) (type 2) (param i32 i32)
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
    (func $wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream::blocking_write_and_flush (;50;) (type 4) (param i32 i32 i32 i32)
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
    (func $wasi_snapshot_preview1::macros::eprint_u32 (;51;) (type 1) (param i32)
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
    (func $wasi_snapshot_preview1::macros::eprint_u32::eprint_u32_impl (;52;) (type 1) (param i32)
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
    (func $<core::option::Option<T> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap (;53;) (type 7) (param i32 i32 i32)
      (local i32)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      block ;; label = @1
        local.get 1
        br_if 0 (;@1;)
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
        i32.const 83
        call $wasi_snapshot_preview1::macros::eprint_u32
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
      local.get 0
      local.get 2
      i32.store offset=4
      local.get 0
      local.get 1
      i32.store
      local.get 3
      i32.const 48
      i32.add
      global.set $__stack_pointer
    )
    (func $<core::option::Option<T> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap (;54;) (type 6) (param i32) (result i32)
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
        i32.const 83
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
    (func $<core::option::Option<T> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap (;55;) (type 12) (param i32 i32) (result i32)
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
        i32.const 83
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
    (func $<core::result::Result<T,E> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap (;56;) (type 2) (param i32 i32)
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
        i32.const 92
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
    (func $<core::result::Result<T,E> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap (;57;) (type 12) (param i32 i32) (result i32)
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
      i32.const 92
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
    (func $<core::result::Result<T,E> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap (;58;) (type 12) (param i32 i32) (result i32)
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
      i32.const 92
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
    (func $<core::result::Result<T,E> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap (;59;) (type 6) (param i32) (result i32)
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
      i32.const 92
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
    (func $<core::result::Result<T,E> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap (;60;) (type 6) (param i32) (result i32)
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
      i32.const 92
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
    (func $wasi_snapshot_preview1::<impl core::convert::From<wasi_snapshot_preview1::bindings::wasi::filesystem::types::ErrorCode> for wasi::lib_generated::Errno>::from (;61;) (type 6) (param i32) (result i32)
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
    (func $wasi_snapshot_preview1::bindings::wasi::cli::exit::exit (;62;) (type 1) (param i32)
      local.get 0
      call $wasi_snapshot_preview1::bindings::wasi::cli::exit::exit::wit_import
    )
    (func $wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream::check_write (;63;) (type 2) (param i32 i32)
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
    (func $wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream::write (;64;) (type 4) (param i32 i32 i32 i32)
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
    (func $wasi_snapshot_preview1::bindings::wasi::io::streams::OutputStream::blocking_flush (;65;) (type 2) (param i32 i32)
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
    (func $core::ptr::drop_in_place<wasi_snapshot_preview1::descriptors::Descriptor> (;66;) (type 1) (param i32)
      (local i32)
      block ;; label = @1
        local.get 0
        i32.load
        i32.eqz
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
        block ;; label = @2
          block ;; label = @3
            local.get 0
            i32.const 41
            i32.add
            i32.load8_u
            i32.const -2
            i32.add
            local.tee 1
            i32.const 1
            local.get 1
            i32.const 255
            i32.and
            i32.const 3
            i32.lt_u
            select
            i32.const 255
            i32.and
            br_table 2 (;@1;) 1 (;@2;) 0 (;@3;)
          end
          local.get 0
          i32.load offset=24
          call $<wasi_snapshot_preview1::bindings::wasi::sockets::tcp::TcpSocket as wit_bindgen::WasmResource>::drop::drop
          return
        end
        local.get 0
        i32.load offset=24
        call $<wasi_snapshot_preview1::bindings::wasi::filesystem::types::Descriptor as wit_bindgen::WasmResource>::drop::drop
      end
    )
    (func $wasi_snapshot_preview1::descriptors::Streams::get_write_stream (;67;) (type 2) (param i32 i32)
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
                    block ;; label = @8
                      local.get 1
                      i32.const 33
                      i32.add
                      i32.load8_u
                      i32.const -2
                      i32.add
                      i32.const 255
                      i32.and
                      local.tee 4
                      i32.const 2
                      i32.gt_u
                      br_if 0 (;@8;)
                      i32.const 1
                      local.set 5
                      local.get 4
                      i32.const 1
                      i32.ne
                      br_if 1 (;@7;)
                    end
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
                  br 5 (;@1;)
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
              local.set 5
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
        local.set 5
      end
      local.get 0
      local.get 5
      i32.store16
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
    )
    (func $wasi_snapshot_preview1::descriptors::Descriptors::new (;68;) (type 7) (param i32 i32 i32)
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
      local.get 3
      i32.const 0
      i32.store16 offset=6152
      local.get 3
      i32.const 6192
      i32.add
      call $wasi_snapshot_preview1::bindings::wasi::cli::terminal_stdin::get_terminal_stdin::wit_import
      block ;; label = @1
        local.get 3
        i32.load8_u offset=6192
        local.tee 4
        i32.eqz
        br_if 0 (;@1;)
        local.get 3
        i32.const 6196
        i32.add
        i32.load
        call $<wasi_snapshot_preview1::bindings::wasi::cli::terminal_input::TerminalInput as wit_bindgen::WasmResource>::drop::drop
      end
      local.get 3
      i32.const 6192
      i32.add
      call $wasi_snapshot_preview1::bindings::wasi::cli::terminal_stdout::get_terminal_stdout::wit_import
      block ;; label = @1
        local.get 3
        i32.load8_u offset=6192
        local.tee 5
        i32.eqz
        br_if 0 (;@1;)
        local.get 3
        i32.const 6196
        i32.add
        i32.load
        call $<wasi_snapshot_preview1::bindings::wasi::cli::terminal_output::TerminalOutput as wit_bindgen::WasmResource>::drop::drop
      end
      local.get 3
      i32.const 6192
      i32.add
      call $wasi_snapshot_preview1::bindings::wasi::cli::terminal_stderr::get_terminal_stderr::wit_import
      block ;; label = @1
        local.get 3
        i32.load8_u offset=6192
        local.tee 6
        i32.eqz
        br_if 0 (;@1;)
        local.get 3
        i32.const 6196
        i32.add
        i32.load
        call $<wasi_snapshot_preview1::bindings::wasi::cli::terminal_output::TerminalOutput as wit_bindgen::WasmResource>::drop::drop
      end
      call $wasi_snapshot_preview1::bindings::wasi::cli::stdin::get_stdin::wit_import
      local.set 7
      local.get 3
      i32.const 2
      i32.store8 offset=49
      local.get 3
      local.get 4
      i32.eqz
      i32.store8 offset=32
      local.get 3
      i32.const 0
      i32.store offset=24
      local.get 3
      local.get 7
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
      local.get 5
      i32.eqz
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
      local.set 7
      local.get 3
      i32.const 128
      i32.add
      local.get 6
      i32.eqz
      i32.store8
      local.get 3
      i32.const 124
      i32.add
      local.get 7
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
      local.set 8
      block ;; label = @1
        local.get 3
        i32.load offset=6180
        local.tee 9
        i32.eqz
        br_if 0 (;@1;)
        local.get 9
        i32.const 12
        i32.mul
        local.set 1
        local.get 3
        i32.const 6192
        i32.add
        i32.const 1
        i32.or
        local.set 7
        local.get 8
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
          local.get 7
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
      local.get 9
      i32.store
      local.get 3
      local.get 8
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
    (func $wasi_snapshot_preview1::descriptors::Descriptors::open (;69;) (type 7) (param i32 i32 i32)
      (local i32 i32 i32 i64)
      global.get $__stack_pointer
      i32.const 64
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                local.get 1
                i32.load offset=6148
                br_if 0 (;@5;)
                local.get 1
                i32.load16_u offset=6144
                local.tee 4
                i32.const 128
                i32.lt_u
                br_if 1 (;@4;)
                local.get 0
                i32.const 48
                i32.store16 offset=2
                local.get 2
                call $core::ptr::drop_in_place<wasi_snapshot_preview1::descriptors::Descriptor>
                i32.const 1
                local.set 1
                br 3 (;@2;)
              end
              block ;; label = @5
                block ;; label = @6
                  local.get 1
                  i32.const 6152
                  i32.add
                  i32.load
                  local.tee 5
                  local.get 1
                  i32.load16_u offset=6144
                  i32.lt_u
                  br_if 0 (;@6;)
                  local.get 3
                  i32.const 8
                  i32.store16 offset=14
                  i32.const 1
                  local.set 4
                  br 1 (;@5;)
                end
                local.get 3
                local.get 1
                local.get 5
                i32.const 48
                i32.mul
                i32.add
                i32.store offset=16
                i32.const 0
                local.set 4
              end
              local.get 3
              local.get 4
              i32.store16 offset=12
              local.get 3
              i32.const 12
              i32.add
              call $<core::result::Result<T,E> as wasi_snapshot_preview1::TrappingUnwrap<T>>::trapping_unwrap
              local.tee 4
              i32.load
              br_if 3 (;@1;)
              local.get 4
              i64.load offset=8
              local.set 6
              local.get 4
              call $core::ptr::drop_in_place<wasi_snapshot_preview1::descriptors::Descriptor>
              local.get 4
              local.get 2
              i32.const 48
              call $memcpy
              drop
              local.get 0
              local.get 5
              i32.store offset=4
              local.get 1
              local.get 6
              i64.store offset=6148 align=4
              br 1 (;@3;)
            end
            local.get 1
            local.get 4
            i32.const 48
            i32.mul
            i32.add
            local.get 2
            i32.const 48
            call $memcpy
            drop
            local.get 0
            local.get 4
            i32.store offset=4
            local.get 1
            local.get 4
            i32.const 1
            i32.add
            i32.store16 offset=6144
          end
          i32.const 0
          local.set 1
        end
        local.get 0
        local.get 1
        i32.store16
        local.get 3
        i32.const 64
        i32.add
        global.set $__stack_pointer
        return
      end
      local.get 3
      i32.const 32
      i32.store8 offset=48
      local.get 3
      i32.const 1701734764
      i32.store offset=44 align=1
      local.get 3
      i64.const 2338042707334751329
      i64.store offset=36 align=1
      local.get 3
      i64.const 2338600898263348341
      i64.store offset=28 align=1
      local.get 3
      i64.const 7162263158133189730
      i64.store offset=20 align=1
      local.get 3
      i64.const 7018969289221893749
      i64.store offset=12 align=1
      local.get 3
      i32.const 12
      i32.add
      i32.const 37
      call $wasi_snapshot_preview1::macros::print
      i32.const 267
      call $wasi_snapshot_preview1::macros::eprint_u32
      local.get 3
      i32.const 8250
      i32.store16 offset=12 align=1
      local.get 3
      i32.const 12
      i32.add
      i32.const 2
      call $wasi_snapshot_preview1::macros::print
      local.get 3
      i32.const 10
      i32.store8 offset=62
      local.get 3
      i32.const 29295
      i32.store16 offset=60 align=1
      local.get 3
      i64.const 8390322045806929252
      i64.store offset=52 align=1
      local.get 3
      i64.const 2334102053349778208
      i64.store offset=44 align=1
      local.get 3
      i64.const 6998716365485077614
      i64.store offset=36 align=1
      local.get 3
      i64.const 7597414381092301164
      i64.store offset=28 align=1
      local.get 3
      i64.const 7306371753431426412
      i64.store offset=20 align=1
      local.get 3
      i64.const 7091326027899628905
      i64.store offset=12 align=1
      local.get 3
      i32.const 12
      i32.add
      i32.const 51
      call $wasi_snapshot_preview1::macros::print
      local.get 3
      i32.const 10
      i32.store8 offset=12
      local.get 3
      i32.const 12
      i32.add
      i32.const 1
      call $wasi_snapshot_preview1::macros::print
      unreachable
      unreachable
    )
    (func $wasi_snapshot_preview1::descriptors::Descriptors::close (;70;) (type 7) (param i32 i32 i32)
      (local i32 i32 i32 i32 i32 i64)
      global.get $__stack_pointer
      i32.const 48
      i32.sub
      local.tee 3
      global.set $__stack_pointer
      i32.const 1
      local.set 4
      i32.const 8
      local.set 5
      block ;; label = @1
        local.get 1
        i32.load16_u offset=6144
        local.get 2
        i32.le_u
        br_if 0 (;@1;)
        local.get 1
        local.get 2
        i32.const 48
        i32.mul
        i32.add
        local.tee 6
        i32.load
        local.tee 7
        i32.eqz
        br_if 0 (;@1;)
        local.get 6
        i32.load16_u offset=4
        local.set 5
        local.get 1
        i64.load offset=6148 align=4
        local.set 8
        local.get 3
        i32.const 6
        i32.or
        local.get 6
        i32.const 6
        i32.add
        i32.const 42
        call $memcpy
        drop
        local.get 6
        local.get 8
        i64.store offset=8
        i32.const 0
        local.set 4
        local.get 6
        i32.const 0
        i32.store
        local.get 1
        i32.const 1
        i32.store offset=6148
        local.get 1
        i32.const 6152
        i32.add
        local.get 2
        i32.store
        local.get 3
        local.get 5
        i32.store16 offset=4
        local.get 3
        local.get 7
        i32.store
        local.get 3
        call $core::ptr::drop_in_place<wasi_snapshot_preview1::descriptors::Descriptor>
      end
      local.get 0
      local.get 5
      i32.store16 offset=2
      local.get 0
      local.get 4
      i32.store16
      local.get 3
      i32.const 48
      i32.add
      global.set $__stack_pointer
    )
    (func $wasi_snapshot_preview1::descriptors::Descriptors::get_dir (;71;) (type 7) (param i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            local.get 1
            i32.load16_u offset=6144
            local.get 2
            i32.le_u
            br_if 0 (;@3;)
            block ;; label = @4
              block ;; label = @5
                local.get 1
                local.get 2
                i32.const 48
                i32.mul
                i32.add
                local.tee 1
                i32.load
                i32.eqz
                br_if 0 (;@5;)
                local.get 1
                i32.const 41
                i32.add
                i32.load8_u
                i32.const -2
                i32.add
                i32.const 255
                i32.and
                local.tee 2
                i32.const 2
                i32.gt_u
                br_if 1 (;@4;)
                local.get 2
                i32.const 1
                i32.eq
                br_if 1 (;@4;)
              end
              local.get 0
              i32.const 8
              i32.store16 offset=2
              br 2 (;@2;)
            end
            block ;; label = @4
              local.get 1
              i32.const 28
              i32.add
              i32.load8_u
              i32.const 3
              i32.ne
              br_if 0 (;@4;)
              local.get 0
              local.get 1
              i32.const 24
              i32.add
              i32.store offset=4
              i32.const 0
              local.set 1
              br 3 (;@1;)
            end
            local.get 0
            i32.const 54
            i32.store16 offset=2
            br 1 (;@2;)
          end
          local.get 0
          i32.const 8
          i32.store16 offset=2
        end
        i32.const 1
        local.set 1
      end
      local.get 0
      local.get 1
      i32.store16
    )
    (func $get_state_ptr (;72;) (type 10) (result i32)
      global.get $internal_state_ptr
    )
    (func $set_state_ptr (;73;) (type 1) (param i32)
      local.get 0
      global.set $internal_state_ptr
    )
    (func $get_allocation_state (;74;) (type 10) (result i32)
      global.get $allocation_state
    )
    (func $set_allocation_state (;75;) (type 1) (param i32)
      local.get 0
      global.set $allocation_state
    )
    (func $compiler_builtins::mem::memcpy (;76;) (type 11) (param i32 i32 i32) (result i32)
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
    (func $memcpy (;77;) (type 11) (param i32 i32 i32) (result i32)
      local.get 0
      local.get 1
      local.get 2
      call $compiler_builtins::mem::memcpy
    )
    (func $allocate_stack (;78;) (type 14)
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
    (export "wasi:cli/run@0.2.0-rc-2023-11-10#run" (func $wasi:cli/run@0.2.0-rc-2023-11-10#run))
    (export "fd_prestat_get" (func $fd_prestat_get))
    (export "cabi_import_realloc" (func $cabi_import_realloc))
    (export "cabi_export_realloc" (func $cabi_export_realloc))
    (export "fd_close" (func $fd_close))
    (export "fd_prestat_dir_name" (func $fd_prestat_dir_name))
    (export "environ_get" (func $environ_get))
    (export "environ_sizes_get" (func $environ_sizes_get))
    (export "fd_write" (func $fd_write))
    (export "proc_exit" (func $proc_exit))
    (export "path_open" (func $path_open))
  )
  (core module (;2;)
    (type (;0;) (func (param i32)))
    (type (;1;) (func (param i32 i64 i32)))
    (type (;2;) (func (param i32 i32)))
    (type (;3;) (func (param i32 i32 i32 i32 i32 i32 i32)))
    (type (;4;) (func (param i32 i32 i32 i32)))
    (type (;5;) (func (param i32 i32 i32 i32) (result i32)))
    (type (;6;) (func (param i32 i32 i32 i32 i32 i64 i64 i32 i32) (result i32)))
    (type (;7;) (func (param i32 i32) (result i32)))
    (type (;8;) (func (param i32) (result i32)))
    (type (;9;) (func (param i32 i32 i32) (result i32)))
    (type (;10;) (func (param i32)))
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
    (func $#func6<indirect-wasi:filesystem/types@0.2.0-rc-2023-11-10-_method_descriptor.open-at> (@name "indirect-wasi:filesystem/types@0.2.0-rc-2023-11-10-[method]descriptor.open-at") (;6;) (type 3) (param i32 i32 i32 i32 i32 i32 i32)
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      local.get 4
      local.get 5
      local.get 6
      i32.const 6
      call_indirect (type 3)
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
    (func $#func9<indirect-wasi:io/streams@0.2.0-rc-2023-11-10-_method_output-stream.write> (@name "indirect-wasi:io/streams@0.2.0-rc-2023-11-10-[method]output-stream.write") (;9;) (type 4) (param i32 i32 i32 i32)
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      i32.const 9
      call_indirect (type 4)
    )
    (func $#func10<indirect-wasi:io/streams@0.2.0-rc-2023-11-10-_method_output-stream.blocking-write-and-flush> (@name "indirect-wasi:io/streams@0.2.0-rc-2023-11-10-[method]output-stream.blocking-write-and-flush") (;10;) (type 4) (param i32 i32 i32 i32)
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      i32.const 10
      call_indirect (type 4)
    )
    (func $#func11<indirect-wasi:io/streams@0.2.0-rc-2023-11-10-_method_output-stream.blocking-flush> (@name "indirect-wasi:io/streams@0.2.0-rc-2023-11-10-[method]output-stream.blocking-flush") (;11;) (type 2) (param i32 i32)
      local.get 0
      local.get 1
      i32.const 11
      call_indirect (type 2)
    )
    (func $indirect-wasi:cli/environment@0.2.0-rc-2023-11-10-get-environment (;12;) (type 0) (param i32)
      local.get 0
      i32.const 12
      call_indirect (type 0)
    )
    (func $indirect-wasi:cli/terminal-stdin@0.2.0-rc-2023-11-10-get-terminal-stdin (;13;) (type 0) (param i32)
      local.get 0
      i32.const 13
      call_indirect (type 0)
    )
    (func $indirect-wasi:cli/terminal-stdout@0.2.0-rc-2023-11-10-get-terminal-stdout (;14;) (type 0) (param i32)
      local.get 0
      i32.const 14
      call_indirect (type 0)
    )
    (func $indirect-wasi:cli/terminal-stderr@0.2.0-rc-2023-11-10-get-terminal-stderr (;15;) (type 0) (param i32)
      local.get 0
      i32.const 15
      call_indirect (type 0)
    )
    (func $adapt-wasi_snapshot_preview1-fd_write (;16;) (type 5) (param i32 i32 i32 i32) (result i32)
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      i32.const 16
      call_indirect (type 5)
    )
    (func $adapt-wasi_snapshot_preview1-path_open (;17;) (type 6) (param i32 i32 i32 i32 i32 i64 i64 i32 i32) (result i32)
      local.get 0
      local.get 1
      local.get 2
      local.get 3
      local.get 4
      local.get 5
      local.get 6
      local.get 7
      local.get 8
      i32.const 17
      call_indirect (type 6)
    )
    (func $adapt-wasi_snapshot_preview1-environ_get (;18;) (type 7) (param i32 i32) (result i32)
      local.get 0
      local.get 1
      i32.const 18
      call_indirect (type 7)
    )
    (func $adapt-wasi_snapshot_preview1-environ_sizes_get (;19;) (type 7) (param i32 i32) (result i32)
      local.get 0
      local.get 1
      i32.const 19
      call_indirect (type 7)
    )
    (func $adapt-wasi_snapshot_preview1-fd_close (;20;) (type 8) (param i32) (result i32)
      local.get 0
      i32.const 20
      call_indirect (type 8)
    )
    (func $adapt-wasi_snapshot_preview1-fd_prestat_get (;21;) (type 7) (param i32 i32) (result i32)
      local.get 0
      local.get 1
      i32.const 21
      call_indirect (type 7)
    )
    (func $adapt-wasi_snapshot_preview1-fd_prestat_dir_name (;22;) (type 9) (param i32 i32 i32) (result i32)
      local.get 0
      local.get 1
      local.get 2
      i32.const 22
      call_indirect (type 9)
    )
    (func $adapt-wasi_snapshot_preview1-proc_exit (;23;) (type 10) (param i32)
      local.get 0
      i32.const 23
      call_indirect (type 10)
    )
    (table (;0;) 24 24 funcref)
    (export "0" (func $indirect-miden:base/tx-kernel@1.0.0-get-inputs))
    (export "1" (func $indirect-miden:base/tx-kernel@1.0.0-get-assets))
    (export "2" (func $indirect-wasi:filesystem/preopens@0.2.0-rc-2023-11-10-get-directories))
    (export "3" (func $#func3<indirect-wasi:filesystem/types@0.2.0-rc-2023-11-10-_method_descriptor.write-via-stream>))
    (export "4" (func $#func4<indirect-wasi:filesystem/types@0.2.0-rc-2023-11-10-_method_descriptor.append-via-stream>))
    (export "5" (func $#func5<indirect-wasi:filesystem/types@0.2.0-rc-2023-11-10-_method_descriptor.get-type>))
    (export "6" (func $#func6<indirect-wasi:filesystem/types@0.2.0-rc-2023-11-10-_method_descriptor.open-at>))
    (export "7" (func $indirect-wasi:filesystem/types@0.2.0-rc-2023-11-10-filesystem-error-code))
    (export "8" (func $#func8<indirect-wasi:io/streams@0.2.0-rc-2023-11-10-_method_output-stream.check-write>))
    (export "9" (func $#func9<indirect-wasi:io/streams@0.2.0-rc-2023-11-10-_method_output-stream.write>))
    (export "10" (func $#func10<indirect-wasi:io/streams@0.2.0-rc-2023-11-10-_method_output-stream.blocking-write-and-flush>))
    (export "11" (func $#func11<indirect-wasi:io/streams@0.2.0-rc-2023-11-10-_method_output-stream.blocking-flush>))
    (export "12" (func $indirect-wasi:cli/environment@0.2.0-rc-2023-11-10-get-environment))
    (export "13" (func $indirect-wasi:cli/terminal-stdin@0.2.0-rc-2023-11-10-get-terminal-stdin))
    (export "14" (func $indirect-wasi:cli/terminal-stdout@0.2.0-rc-2023-11-10-get-terminal-stdout))
    (export "15" (func $indirect-wasi:cli/terminal-stderr@0.2.0-rc-2023-11-10-get-terminal-stderr))
    (export "16" (func $adapt-wasi_snapshot_preview1-fd_write))
    (export "17" (func $adapt-wasi_snapshot_preview1-path_open))
    (export "18" (func $adapt-wasi_snapshot_preview1-environ_get))
    (export "19" (func $adapt-wasi_snapshot_preview1-environ_sizes_get))
    (export "20" (func $adapt-wasi_snapshot_preview1-fd_close))
    (export "21" (func $adapt-wasi_snapshot_preview1-fd_prestat_get))
    (export "22" (func $adapt-wasi_snapshot_preview1-fd_prestat_dir_name))
    (export "23" (func $adapt-wasi_snapshot_preview1-proc_exit))
    (export "$imports" (table 0))
  )
  (core module (;3;)
    (type (;0;) (func (param i32)))
    (type (;1;) (func (param i32 i64 i32)))
    (type (;2;) (func (param i32 i32)))
    (type (;3;) (func (param i32 i32 i32 i32 i32 i32 i32)))
    (type (;4;) (func (param i32 i32 i32 i32)))
    (type (;5;) (func (param i32 i32 i32 i32) (result i32)))
    (type (;6;) (func (param i32 i32 i32 i32 i32 i64 i64 i32 i32) (result i32)))
    (type (;7;) (func (param i32 i32) (result i32)))
    (type (;8;) (func (param i32) (result i32)))
    (type (;9;) (func (param i32 i32 i32) (result i32)))
    (type (;10;) (func (param i32)))
    (import "" "0" (func (;0;) (type 0)))
    (import "" "1" (func (;1;) (type 0)))
    (import "" "2" (func (;2;) (type 0)))
    (import "" "3" (func (;3;) (type 1)))
    (import "" "4" (func (;4;) (type 2)))
    (import "" "5" (func (;5;) (type 2)))
    (import "" "6" (func (;6;) (type 3)))
    (import "" "7" (func (;7;) (type 2)))
    (import "" "8" (func (;8;) (type 2)))
    (import "" "9" (func (;9;) (type 4)))
    (import "" "10" (func (;10;) (type 4)))
    (import "" "11" (func (;11;) (type 2)))
    (import "" "12" (func (;12;) (type 0)))
    (import "" "13" (func (;13;) (type 0)))
    (import "" "14" (func (;14;) (type 0)))
    (import "" "15" (func (;15;) (type 0)))
    (import "" "16" (func (;16;) (type 5)))
    (import "" "17" (func (;17;) (type 6)))
    (import "" "18" (func (;18;) (type 7)))
    (import "" "19" (func (;19;) (type 7)))
    (import "" "20" (func (;20;) (type 8)))
    (import "" "21" (func (;21;) (type 7)))
    (import "" "22" (func (;22;) (type 9)))
    (import "" "23" (func (;23;) (type 10)))
    (import "" "$imports" (table (;0;) 24 24 funcref))
    (elem (;0;) (i32.const 0) func 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23)
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
  (alias export 3 "some-asset-check" (func (;1;)))
  (core func (;3;) (canon lower (func 1)))
  (core instance (;2;)
    (export "some-asset-check" (func 3))
  )
  (alias export 2 "receive-asset" (func (;2;)))
  (core func (;4;) (canon lower (func 2)))
  (core instance (;3;)
    (export "receive-asset" (func 4))
  )
  (alias core export 0 "16" (core func (;5;)))
  (alias core export 0 "17" (core func (;6;)))
  (alias core export 0 "18" (core func (;7;)))
  (alias core export 0 "19" (core func (;8;)))
  (alias core export 0 "20" (core func (;9;)))
  (alias core export 0 "21" (core func (;10;)))
  (alias core export 0 "22" (core func (;11;)))
  (alias core export 0 "23" (core func (;12;)))
  (core instance (;4;)
    (export "fd_write" (func 5))
    (export "path_open" (func 6))
    (export "environ_get" (func 7))
    (export "environ_sizes_get" (func 8))
    (export "fd_close" (func 9))
    (export "fd_prestat_get" (func 10))
    (export "fd_prestat_dir_name" (func 11))
    (export "proc_exit" (func 12))
  )
  (core instance (;5;) (instantiate 0
      (with "miden:base/tx-kernel@1.0.0" (instance 1))
      (with "miden:basic-wallet-helpers/check-helpers@1.0.0" (instance 2))
      (with "miden:basic-wallet/basic-wallet@1.0.0" (instance 3))
      (with "wasi_snapshot_preview1" (instance 4))
    )
  )
  (alias core export 5 "memory" (core memory (;0;)))
  (alias core export 5 "cabi_realloc" (core func (;13;)))
  (alias core export 5 "_start" (core func (;14;)))
  (alias core export 5 "cabi_realloc" (core func (;15;)))
  (core instance (;6;)
    (export "_start" (func 14))
    (export "cabi_realloc" (func 15))
  )
  (core instance (;7;)
    (export "memory" (memory 0))
  )
  (alias core export 0 "2" (core func (;16;)))
  (core instance (;8;)
    (export "get-directories" (func 16))
  )
  (alias export 6 "descriptor" (type (;34;)))
  (core func (;17;) (canon resource.drop 34))
  (alias export 6 "directory-entry-stream" (type (;35;)))
  (core func (;18;) (canon resource.drop 35))
  (alias core export 0 "3" (core func (;19;)))
  (alias core export 0 "4" (core func (;20;)))
  (alias core export 0 "5" (core func (;21;)))
  (alias core export 0 "6" (core func (;22;)))
  (alias core export 0 "7" (core func (;23;)))
  (core instance (;9;)
    (export "[resource-drop]descriptor" (func 17))
    (export "[resource-drop]directory-entry-stream" (func 18))
    (export "[method]descriptor.write-via-stream" (func 19))
    (export "[method]descriptor.append-via-stream" (func 20))
    (export "[method]descriptor.get-type" (func 21))
    (export "[method]descriptor.open-at" (func 22))
    (export "filesystem-error-code" (func 23))
  )
  (alias export 4 "error" (type (;36;)))
  (core func (;24;) (canon resource.drop 36))
  (core instance (;10;)
    (export "[resource-drop]error" (func 24))
  )
  (alias export 5 "input-stream" (type (;37;)))
  (core func (;25;) (canon resource.drop 37))
  (alias export 5 "output-stream" (type (;38;)))
  (core func (;26;) (canon resource.drop 38))
  (alias core export 0 "8" (core func (;27;)))
  (alias core export 0 "9" (core func (;28;)))
  (alias core export 0 "10" (core func (;29;)))
  (alias core export 0 "11" (core func (;30;)))
  (core instance (;11;)
    (export "[resource-drop]input-stream" (func 25))
    (export "[resource-drop]output-stream" (func 26))
    (export "[method]output-stream.check-write" (func 27))
    (export "[method]output-stream.write" (func 28))
    (export "[method]output-stream.blocking-write-and-flush" (func 29))
    (export "[method]output-stream.blocking-flush" (func 30))
  )
  (alias core export 0 "12" (core func (;31;)))
  (core instance (;12;)
    (export "get-environment" (func 31))
  )
  (alias export 15 "terminal-output" (type (;39;)))
  (core func (;32;) (canon resource.drop 39))
  (core instance (;13;)
    (export "[resource-drop]terminal-output" (func 32))
  )
  (alias export 8 "tcp-socket" (type (;40;)))
  (core func (;33;) (canon resource.drop 40))
  (core instance (;14;)
    (export "[resource-drop]tcp-socket" (func 33))
  )
  (alias export 14 "terminal-input" (type (;41;)))
  (core func (;34;) (canon resource.drop 41))
  (core instance (;15;)
    (export "[resource-drop]terminal-input" (func 34))
  )
  (alias export 13 "get-stderr" (func (;3;)))
  (core func (;35;) (canon lower (func 3)))
  (core instance (;16;)
    (export "get-stderr" (func 35))
  )
  (alias export 10 "exit" (func (;4;)))
  (core func (;36;) (canon lower (func 4)))
  (core instance (;17;)
    (export "exit" (func 36))
  )
  (alias export 11 "get-stdin" (func (;5;)))
  (core func (;37;) (canon lower (func 5)))
  (core instance (;18;)
    (export "get-stdin" (func 37))
  )
  (alias export 12 "get-stdout" (func (;6;)))
  (core func (;38;) (canon lower (func 6)))
  (core instance (;19;)
    (export "get-stdout" (func 38))
  )
  (alias core export 0 "13" (core func (;39;)))
  (core instance (;20;)
    (export "get-terminal-stdin" (func 39))
  )
  (alias core export 0 "14" (core func (;40;)))
  (core instance (;21;)
    (export "get-terminal-stdout" (func 40))
  )
  (alias core export 0 "15" (core func (;41;)))
  (core instance (;22;)
    (export "get-terminal-stderr" (func 41))
  )
  (core instance (;23;) (instantiate 1
      (with "__main_module__" (instance 6))
      (with "env" (instance 7))
      (with "wasi:filesystem/preopens@0.2.0-rc-2023-11-10" (instance 8))
      (with "wasi:filesystem/types@0.2.0-rc-2023-11-10" (instance 9))
      (with "wasi:io/error@0.2.0-rc-2023-11-10" (instance 10))
      (with "wasi:io/streams@0.2.0-rc-2023-11-10" (instance 11))
      (with "wasi:cli/environment@0.2.0-rc-2023-11-10" (instance 12))
      (with "wasi:cli/terminal-output@0.2.0-rc-2023-11-10" (instance 13))
      (with "wasi:sockets/tcp@0.2.0-rc-2023-11-10" (instance 14))
      (with "wasi:cli/terminal-input@0.2.0-rc-2023-11-10" (instance 15))
      (with "wasi:cli/stderr@0.2.0-rc-2023-11-10" (instance 16))
      (with "wasi:cli/exit@0.2.0-rc-2023-11-10" (instance 17))
      (with "wasi:cli/stdin@0.2.0-rc-2023-11-10" (instance 18))
      (with "wasi:cli/stdout@0.2.0-rc-2023-11-10" (instance 19))
      (with "wasi:cli/terminal-stdin@0.2.0-rc-2023-11-10" (instance 20))
      (with "wasi:cli/terminal-stdout@0.2.0-rc-2023-11-10" (instance 21))
      (with "wasi:cli/terminal-stderr@0.2.0-rc-2023-11-10" (instance 22))
    )
  )
  (alias core export 23 "cabi_export_realloc" (core func (;42;)))
  (alias core export 23 "cabi_import_realloc" (core func (;43;)))
  (alias core export 0 "$imports" (core table (;0;)))
  (alias export 1 "get-inputs" (func (;7;)))
  (core func (;44;) (canon lower (func 7) (memory 0)))
  (alias export 1 "get-assets" (func (;8;)))
  (core func (;45;) (canon lower (func 8) (memory 0) (realloc 13)))
  (alias export 7 "get-directories" (func (;9;)))
  (core func (;46;) (canon lower (func 9) (memory 0) (realloc 43) string-encoding=utf8))
  (alias export 6 "[method]descriptor.write-via-stream" (func (;10;)))
  (core func (;47;) (canon lower (func 10) (memory 0)))
  (alias export 6 "[method]descriptor.append-via-stream" (func (;11;)))
  (core func (;48;) (canon lower (func 11) (memory 0)))
  (alias export 6 "[method]descriptor.get-type" (func (;12;)))
  (core func (;49;) (canon lower (func 12) (memory 0)))
  (alias export 6 "[method]descriptor.open-at" (func (;13;)))
  (core func (;50;) (canon lower (func 13) (memory 0) string-encoding=utf8))
  (alias export 6 "filesystem-error-code" (func (;14;)))
  (core func (;51;) (canon lower (func 14) (memory 0)))
  (alias export 5 "[method]output-stream.check-write" (func (;15;)))
  (core func (;52;) (canon lower (func 15) (memory 0)))
  (alias export 5 "[method]output-stream.write" (func (;16;)))
  (core func (;53;) (canon lower (func 16) (memory 0)))
  (alias export 5 "[method]output-stream.blocking-write-and-flush" (func (;17;)))
  (core func (;54;) (canon lower (func 17) (memory 0)))
  (alias export 5 "[method]output-stream.blocking-flush" (func (;18;)))
  (core func (;55;) (canon lower (func 18) (memory 0)))
  (alias export 9 "get-environment" (func (;19;)))
  (core func (;56;) (canon lower (func 19) (memory 0) (realloc 43) string-encoding=utf8))
  (alias export 16 "get-terminal-stdin" (func (;20;)))
  (core func (;57;) (canon lower (func 20) (memory 0)))
  (alias export 17 "get-terminal-stdout" (func (;21;)))
  (core func (;58;) (canon lower (func 21) (memory 0)))
  (alias export 18 "get-terminal-stderr" (func (;22;)))
  (core func (;59;) (canon lower (func 22) (memory 0)))
  (alias core export 23 "fd_write" (core func (;60;)))
  (alias core export 23 "path_open" (core func (;61;)))
  (alias core export 23 "environ_get" (core func (;62;)))
  (alias core export 23 "environ_sizes_get" (core func (;63;)))
  (alias core export 23 "fd_close" (core func (;64;)))
  (alias core export 23 "fd_prestat_get" (core func (;65;)))
  (alias core export 23 "fd_prestat_dir_name" (core func (;66;)))
  (alias core export 23 "proc_exit" (core func (;67;)))
  (core instance (;24;)
    (export "$imports" (table 0))
    (export "0" (func 44))
    (export "1" (func 45))
    (export "2" (func 46))
    (export "3" (func 47))
    (export "4" (func 48))
    (export "5" (func 49))
    (export "6" (func 50))
    (export "7" (func 51))
    (export "8" (func 52))
    (export "9" (func 53))
    (export "10" (func 54))
    (export "11" (func 55))
    (export "12" (func 56))
    (export "13" (func 57))
    (export "14" (func 58))
    (export "15" (func 59))
    (export "16" (func 60))
    (export "17" (func 61))
    (export "18" (func 62))
    (export "19" (func 63))
    (export "20" (func 64))
    (export "21" (func 65))
    (export "22" (func 66))
    (export "23" (func 67))
  )
  (core instance (;25;) (instantiate 3
      (with "" (instance 24))
    )
  )
  (type (;42;) (result))
  (type (;43;) (func (result 42)))
  (alias core export 23 "wasi:cli/run@0.2.0-rc-2023-11-10#run" (core func (;68;)))
  (func (;23;) (type 43) (canon lift (core func 68)))
  (component (;0;)
    (type (;0;) (result))
    (type (;1;) (func (result 0)))
    (import "import-func-run" (func (;0;) (type 1)))
    (type (;2;) (result))
    (type (;3;) (func (result 2)))
    (export (;1;) "run" (func 0) (func (type 3)))
  )
  (instance (;19;) (instantiate 0
      (with "import-func-run" (func 23))
    )
  )
  (export (;20;) (interface "wasi:cli/run@0.2.0-rc-2023-11-10") (instance 19))
)