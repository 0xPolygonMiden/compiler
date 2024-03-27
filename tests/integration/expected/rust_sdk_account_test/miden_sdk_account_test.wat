(module $miden_sdk_account_test.wasm
  (type (;0;) (func (result f64)))
  (type (;1;) (func (param i64) (result f64)))
  (type (;2;) (func (param f64 f64) (result f64)))
  (import "miden:tx_kernel/account" "get_id<0x0000000000000000000000000000000000000000000000000000000000000000>" (func $miden_sdk_tx_kernel::extern_account_get_id (;0;) (type 0)))
  (import "miden:types/felt" "from_u64_unchecked" (func $miden_sdk_types::extern_felt_from_u64_unchecked (;1;) (type 1)))
  (import "miden:types/felt" "add" (func $miden_sdk_types::extern_felt_add (;2;) (type 2)))
  (func $get_wallet_magic_number (;3;) (type 0) (result f64)
    (local f64)
    call $miden_sdk_tx_kernel::extern_account_get_id
    local.set 0
    i64.const 42
    call $miden_sdk_types::extern_felt_from_u64_unchecked
    local.get 0
    call $miden_sdk_types::extern_felt_add
  )
  (table (;0;) 1 1 funcref)
  (memory (;0;) 16)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (export "memory" (memory 0))
  (export "get_wallet_magic_number" (func $get_wallet_magic_number))
)