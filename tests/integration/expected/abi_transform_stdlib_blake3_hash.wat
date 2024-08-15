(module $abi_transform_stdlib_blake3_hash.wasm
  (type (;0;) (func (param i32 i32 i32 i32 i32 i32 i32 i32 i32)))
  (type (;1;) (func (param i32 i32)))
  (import "std::crypto::hashes::blake3" "hash_1to1<0x0000000000000000000000000000000000000000000000000000000000000000>" (func $miden_stdlib_sys::stdlib::crypto::hashes::extern_blake3_hash_1to1 (;0;) (type 0)))
  (func $entrypoint (;1;) (type 1) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 1
    i32.load align=1
    local.get 1
    i32.load offset=4 align=1
    local.get 1
    i32.load offset=8 align=1
    local.get 1
    i32.load offset=12 align=1
    local.get 1
    i32.load offset=16 align=1
    local.get 1
    i32.load offset=20 align=1
    local.get 1
    i32.load offset=24 align=1
    local.get 1
    i32.load offset=28 align=1
    local.get 2
    call $miden_stdlib_sys::stdlib::crypto::hashes::extern_blake3_hash_1to1
    local.get 0
    i32.const 24
    i32.add
    local.get 2
    i32.const 24
    i32.add
    i64.load align=1
    i64.store align=1
    local.get 0
    i32.const 16
    i32.add
    local.get 2
    i32.const 16
    i32.add
    i64.load align=1
    i64.store align=1
    local.get 0
    i32.const 8
    i32.add
    local.get 2
    i32.const 8
    i32.add
    i64.load align=1
    i64.store align=1
    local.get 0
    local.get 2
    i64.load align=1
    i64.store align=1
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (table (;0;) 1 1 funcref)
  (memory (;0;) 16)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (export "memory" (memory 0))
  (export "entrypoint" (func $entrypoint))
)