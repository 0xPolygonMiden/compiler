(module
  (type (;0;) (func (param i32)))
  (type (;1;) (func (param i32 i32) (result i32)))
  (type (;2;) (func (result i32)))
  (type (;3;) (func (param i32 i32)))
  (type (;4;) (func (param i32 i32 i32)))
  (func $rust_begin_unwind (;0;) (type 0) (param i32)
    loop ;; label = @1
      br 0 (;@1;)
    end
  )
  (func $div_s (;1;) (type 1) (param i32 i32) (result i32)
    block ;; label = @1
      block ;; label = @2
        local.get 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.const -2147483648
        i32.ne
        br_if 1 (;@1;)
        local.get 1
        i32.const -1
        i32.ne
        br_if 1 (;@1;)
        i32.const 1048704
        i32.const 31
        i32.const 1048648
        call $_ZN4core9panicking5panic17h62f53cc4db8dd7b3E
        unreachable
      end
      i32.const 1048672
      i32.const 25
      i32.const 1048648
      call $_ZN4core9panicking5panic17h62f53cc4db8dd7b3E
      unreachable
    end
    local.get 0
    local.get 1
    i32.div_s
  )
  (func $div_u (;2;) (type 1) (param i32 i32) (result i32)
    block ;; label = @1
      local.get 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.div_u
      return
    end
    i32.const 1048672
    i32.const 25
    i32.const 1048736
    call $_ZN4core9panicking5panic17h62f53cc4db8dd7b3E
    unreachable
  )
  (func $rem_s (;3;) (type 1) (param i32 i32) (result i32)
    block ;; label = @1
      block ;; label = @2
        local.get 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.const -2147483648
        i32.ne
        br_if 1 (;@1;)
        local.get 1
        i32.const -1
        i32.ne
        br_if 1 (;@1;)
        i32.const 1048832
        i32.const 48
        i32.const 1048752
        call $_ZN4core9panicking5panic17h62f53cc4db8dd7b3E
        unreachable
      end
      i32.const 1048768
      i32.const 57
      i32.const 1048752
      call $_ZN4core9panicking5panic17h62f53cc4db8dd7b3E
      unreachable
    end
    local.get 0
    local.get 1
    i32.rem_s
  )
  (func $rem_u (;4;) (type 1) (param i32 i32) (result i32)
    block ;; label = @1
      local.get 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.rem_u
      return
    end
    i32.const 1048768
    i32.const 57
    i32.const 1048880
    call $_ZN4core9panicking5panic17h62f53cc4db8dd7b3E
    unreachable
  )
  (func $shr_s (;5;) (type 1) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.shr_s
  )
  (func $shr_u (;6;) (type 1) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.shr_u
  )
  (func $__main (;7;) (type 2) (result i32)
    i32.const -8
    i32.const -4
    call $div_s
    i32.const -8
    i32.const -3
    call $rem_s
    i32.add
    i32.const -16
    i32.const 2
    call $shr_s
    i32.add
    i32.const 8
    i32.const 4
    call $div_u
    i32.add
    i32.const 8
    i32.const 3
    call $rem_u
    i32.add
    i32.const 16
    i32.const 2
    call $shr_u
    i32.add
  )
  (func $_ZN4core3ptr37drop_in_place$LT$core..fmt..Error$GT$17h282a1f10dc7e004dE (;8;) (type 0) (param i32))
  (func $_ZN4core9panicking9panic_fmt17h9f61a1f2faa523f9E (;9;) (type 3) (param i32 i32)
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
    i32.const 1048896
    i32.store offset=16
    local.get 2
    i32.const 1048896
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
  (func $_ZN4core9panicking5panic17h62f53cc4db8dd7b3E (;10;) (type 4) (param i32 i32 i32)
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
    i32.const 1048896
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
    call $_ZN4core9panicking9panic_fmt17h9f61a1f2faa523f9E
    unreachable
  )
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h29327df37c6e3023E (;11;) (type 3) (param i32 i32)
    local.get 0
    i64.const -1688046730280208939
    i64.store offset=8
    local.get 0
    i64.const -2518113060735759681
    i64.store
  )
  (table (;0;) 3 3 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (global (;1;) i32 i32.const 1048912)
  (global (;2;) i32 i32.const 1048912)
  (export "memory" (memory 0))
  (export "div_s" (func $div_s))
  (export "div_u" (func $div_u))
  (export "rem_s" (func $rem_s))
  (export "rem_u" (func $rem_u))
  (export "shr_s" (func $shr_s))
  (export "shr_u" (func $shr_u))
  (export "__main" (func $__main))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (elem (;0;) (i32.const 1) func $_ZN4core3ptr37drop_in_place$LT$core..fmt..Error$GT$17h282a1f10dc7e004dE $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h29327df37c6e3023E)
  (data $.rodata (;0;) (i32.const 1048576) "/tmp/6c3d0db843d22d28fff49dffc552879651b21c3f44039227473ff2d47441c4f3.rs\00\00\10\00H\00\00\00\0c\00\00\00\05\00\00\00\00\00\00\00\00\00\00\00attempt to divide by zero\00\00\00\00\00\00\00attempt to divide with overflow\00\00\00\10\00H\00\00\00\12\00\00\00\05\00\00\00\00\00\10\00H\00\00\00\18\00\00\00\05\00\00\00attempt to calculate the remainder with a divisor of zero\00\00\00\00\00\00\00attempt to calculate the remainder with overflow\00\00\10\00H\00\00\00\1e\00\00\00\05\00\00\00\01\00\00\00\00\00\00\00\01\00\00\00\02\00\00\00")
)