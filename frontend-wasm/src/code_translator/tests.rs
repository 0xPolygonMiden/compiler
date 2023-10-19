use expect_test::expect;
use miden_hir::write_instruction;
use miden_hir::Ident;

use crate::test_utils::test_diagnostics;
use crate::translate_module;
use crate::WasmTranslationConfig;

/// Compiles the given Wasm code to Miden IR and checks the IR generated.
fn check_ir(wat: &str, expected_ir: expect_test::Expect) {
    let wasm = wat::parse_str(wat).unwrap();
    let diagnostics = test_diagnostics();
    let module = translate_module(&wasm, &WasmTranslationConfig::default(), &diagnostics).unwrap();
    expected_ir.assert_eq(&module.to_string());
}

/// Check IR generated for a Wasm op(s).
/// Wrap Wasm ops in a function and check the IR generated for the entry block of that function.
fn check_op(wat_op: &str, expected_ir: expect_test::Expect) {
    let wat = format!(
        r#"
        (module
            (memory (;0;) 16384)
            (func $test_wrapper
                {wat_op}
            )
        )"#,
    );
    let wasm = wat::parse_str(wat).unwrap();
    let diagnostics = test_diagnostics();
    let module = translate_module(&wasm, &WasmTranslationConfig::default(), &diagnostics).unwrap();
    let func = module.function(Ident::from("test_wrapper")).unwrap();
    // let fref = module.get_funcref_by_name("test_wrapper").unwrap();
    // let func = module.get_function(fref).unwrap();
    let entry_block = func.dfg.entry_block();
    // let entry_block_data = func.dfg.block_data(entry_block);
    let entry_block_data = func.dfg.block(entry_block);
    let mut w = String::new();
    // print instructions up to the branch to the exit block
    for inst in entry_block_data
        .insts()
        .take_while(|inst| !func.dfg[*inst].opcode().is_branch())
    {
        write_instruction(&mut w, func, inst, 0).unwrap();
    }
    expected_ir.assert_eq(&w);
}

#[test]
fn module() {
    check_ir(
        r#"
        (module
            (func $main
                i32.const 0
                drop
            )
        )
    "#,
        expect![[r#"
            module noname

            pub fn main() {
            block0:
            {
                v0 = const.i32 0  : i32
                br block1
            }

            block1:
            {
                ret
            }
            }
        "#]],
    );
}

#[test]
fn locals() {
    check_ir(
        r#"
        (module
            (func $main (local i32)
                i32.const 1
                local.set 0
                local.get 0
                drop
            )
        )
    "#,
        expect![[r#"
            module noname

            pub fn main() {
            block0:
            {
                v0 = const.i32 0  : i32
                v1 = const.i32 1  : i32
                br block1
            }

            block1:
            {
                ret
            }
            }
        "#]],
    );
}

#[test]
fn locals_inter_block() {
    check_ir(
        r#"
        (module
            (func $main (result i32) (local i32)
                block
                    i32.const 3
                    local.set 0
                end
                block
                    local.get 0
                    i32.const 5
                    i32.add
                    local.set 0
                end
                i32.const 7
                local.get 0
                i32.add
            )
        )
    "#,
        expect![[r#"
            module noname

            pub fn main() -> i32 {
            block0:
            {
                v1 = const.i32 0  : i32
                v2 = const.i32 3  : i32
                br block2
            }

            block1(v0: i32):
            {
                ret (v0)
            }

            block2:
            {
                v3 = const.i32 5  : i32
                v4 = add v2, v3  : i32
                br block3
            }

            block3:
            {
                v5 = const.i32 7  : i32
                v6 = add v5, v4  : i32
                br block1(v6)
            }
            }
        "#]],
    );
}

#[test]
fn func_call() {
    check_ir(
        r#"
        (module
            (func $add (param i32 i32) (result i32)
                local.get 0
                local.get 1
                i32.add
            )
            (func $main (result i32)
                i32.const 3
                i32.const 5
                call $add
            )
        )
    "#,
        expect![[r#"
            module noname

            pub fn add(i32, i32) -> i32 {
            block0(v0: i32, v1: i32):
            {
                v3 = add v0, v1  : i32
                br block1(v3)
            }

            block1(v2: i32):
            {
                ret (v2)
            }
            }

            pub fn main() -> i32 {
            block0:
            {
                v1 = const.i32 3  : i32
                v2 = const.i32 5  : i32
                v3 = call noname::add(v1, v2)  : i32
                br block1(v3)
            }

            block1(v0: i32):
            {
                ret (v0)
            }
            }
        "#]],
    );
}

#[test]
fn br() {
    check_ir(
        r#"
        (module
            (func $main (result i32) (local i32)
                block
                    i32.const 3
                    local.set 0
                    br 0
                end
                local.get 0
            )
        )
    "#,
        expect![[r#"
            module noname

            pub fn main() -> i32 {
            block0:
            {
                v1 = const.i32 0  : i32
                v2 = const.i32 3  : i32
                br block2
            }

            block1(v0: i32):
            {
                ret (v0)
            }

            block2:
            {
                br block1(v2)
            }
            }
        "#]],
    );
}

#[test]
fn loop_br_if() {
    // sum the decreasing numbers from 2 to 0, i.e. 2 + 1 + 0, then exit the loop
    check_ir(
        r#"
        (module
            (func $main (result i32) (local i32 i32)
                i32.const 2
                local.set 0
                loop
                    local.get 0
                    local.get 1
                    i32.add
                    local.set 1
                    local.get 0
                    i32.const 1
                    i32.sub
                    local.tee 0
                    br_if 0
                end
                local.get 1
            )
        )
    "#,
        expect![[r#"
            module noname

            pub fn main() -> i32 {
            block0:
            {
                v1 = const.i32 0  : i32
                v2 = const.i32 2  : i32
                br block2(v2, v1)
            }

            block1(v0: i32):
            {
                ret (v0)
            }

            block2(v3: i32, v4: i32):
            {
                v5 = add v3, v4  : i32
                v6 = const.i32 1  : i32
                v7 = sub v3, v6  : i32
                v8 = neq v7, 0  : i1
                condbr v8, block2(v7, v5), block4
            }

            block3:
            {
                br block1(v5)
            }

            block4:
            {
                br block3
            }
            }
        "#]],
    );
}

#[test]
fn if_then_else() {
    check_ir(
        r#"
        (module
            (func $main (result i32)
                i32.const 2
                if (result i32)
                    i32.const 3
                else
                    i32.const 5
                end
            )
        )
    "#,
        expect![[r#"
            module noname

            pub fn main() -> i32 {
            block0:
            {
                v1 = const.i32 2  : i32
                v2 = neq v1, 0  : i1
                condbr v2, block2, block4
            }

            block1(v0: i32):
            {
                ret (v0)
            }

            block2:
            {
                v4 = const.i32 3  : i32
                br block3(v4)
            }

            block3(v3: i32):
            {
                br block1(v3)
            }

            block4:
            {
                v5 = const.i32 5  : i32
                br block3(v5)
            }
            }
        "#]],
    );
}

#[test]
fn global_var() {
    check_ir(
        r#"
        (module
            (global $MyGlobalVal (mut i32) i32.const 42)
            (func $main
                global.get $MyGlobalVal
                i32.const 9
                i32.add
                global.set $MyGlobalVal
            )
        )
    "#,
        expect![[r#"
            module noname
            global external MyGlobalVal : i32 = 0x0000002a { id = gvar0 };


            pub fn main() {
            block0:
            {
                v0 = global.load (@MyGlobalVal) as *mut i8  : i32
                v1 = const.i32 9  : i32
                v2 = add v0, v1  : i32
                v3 = global.symbol @MyGlobalVal  : *mut i32
                store v3, v2
                br block1
            }

            block1:
            {
                ret
            }
            }
        "#]],
    );
}

#[test]
fn memory_grow() {
    check_op(
        r#"
            i32.const 1
            memory.grow
            drop
        "#,
        expect![[r#"
            v0 = const.i32 1  : i32
            v1 = cast v0  : u32
            v2 = memory.grow v1  : i32
        "#]],
    )
}

#[test]
fn memory_size() {
    check_op(
        r#"
            memory.size
            drop
        "#,
        expect![[r#"
            v0 = const.i32 1048575  : i32
        "#]],
    )
}

#[test]
fn i32_load8_u() {
    check_op(
        r#"
            i32.const 1024
            i32.load8_u
            drop
        "#,
        expect![[r#"
            v0 = const.i32 1024  : i32
            v1 = cast v0  : u32
            v2 = inttoptr v1  : *mut u8
            v3 = load v2  : u8
            v4 = zext v3  : i32
        "#]],
    )
}

#[test]
fn i32_load16_u() {
    check_op(
        r#"
            i32.const 1024
            i32.load16_u
            drop
        "#,
        expect![[r#"
            v0 = const.i32 1024  : i32
            v1 = cast v0  : u32
            v2 = inttoptr v1  : *mut u16
            v3 = load v2  : u16
            v4 = zext v3  : i32
        "#]],
    )
}

#[test]
fn i32_load8_s() {
    check_op(
        r#"
            i32.const 1024
            i32.load8_s
            drop
        "#,
        expect![[r#"
            v0 = const.i32 1024  : i32
            v1 = cast v0  : u32
            v2 = inttoptr v1  : *mut i8
            v3 = load v2  : i8
            v4 = sext v3  : i32
        "#]],
    )
}

#[test]
fn i32_load16_s() {
    check_op(
        r#"
            i32.const 1024
            i32.load16_s
            drop
        "#,
        expect![[r#"
            v0 = const.i32 1024  : i32
            v1 = cast v0  : u32
            v2 = inttoptr v1  : *mut i16
            v3 = load v2  : i16
            v4 = sext v3  : i32
        "#]],
    )
}

#[test]
fn i64_load8_u() {
    check_op(
        r#"
            i32.const 1024
            i64.load8_u
            drop
        "#,
        expect![[r#"
            v0 = const.i32 1024  : i32
            v1 = cast v0  : u32
            v2 = inttoptr v1  : *mut u8
            v3 = load v2  : u8
            v4 = zext v3  : i64
        "#]],
    )
}

#[test]
fn i64_load16_u() {
    check_op(
        r#"
            i32.const 1024
            i64.load16_u
            drop
        "#,
        expect![[r#"
            v0 = const.i32 1024  : i32
            v1 = cast v0  : u32
            v2 = inttoptr v1  : *mut u16
            v3 = load v2  : u16
            v4 = zext v3  : i64
        "#]],
    )
}

#[test]
fn i64_load8_s() {
    check_op(
        r#"
            i32.const 1024
            i64.load8_s
            drop
        "#,
        expect![[r#"
            v0 = const.i32 1024  : i32
            v1 = cast v0  : u32
            v2 = inttoptr v1  : *mut i8
            v3 = load v2  : i8
            v4 = sext v3  : i64
        "#]],
    )
}

#[test]
fn i64_load16_s() {
    check_op(
        r#"
            i32.const 1024
            i64.load16_s
            drop
        "#,
        expect![[r#"
            v0 = const.i32 1024  : i32
            v1 = cast v0  : u32
            v2 = inttoptr v1  : *mut i16
            v3 = load v2  : i16
            v4 = sext v3  : i64
        "#]],
    )
}

#[test]
fn i64_load32_s() {
    check_op(
        r#"
            i32.const 1024
            i64.load32_s
            drop
        "#,
        expect![[r#"
            v0 = const.i32 1024  : i32
            v1 = cast v0  : u32
            v2 = inttoptr v1  : *mut i32
            v3 = load v2  : i32
            v4 = sext v3  : i64
        "#]],
    )
}

#[test]
fn i64_load32_u() {
    check_op(
        r#"
            i32.const 1024
            i64.load32_u
            drop
        "#,
        expect![[r#"
            v0 = const.i32 1024  : i32
            v1 = cast v0  : u32
            v2 = inttoptr v1  : *mut u32
            v3 = load v2  : u32
            v4 = zext v3  : i64
        "#]],
    )
}

#[test]
fn i32_load() {
    check_op(
        r#"
            i32.const 1024
            i32.load
            drop
        "#,
        expect![[r#"
            v0 = const.i32 1024  : i32
            v1 = cast v0  : u32
            v2 = inttoptr v1  : *mut i32
            v3 = load v2  : i32
        "#]],
    )
}

#[test]
fn i64_load() {
    check_op(
        r#"
            i32.const 1024
            i64.load
            drop
        "#,
        expect![[r#"
            v0 = const.i32 1024  : i32
            v1 = cast v0  : u32
            v2 = inttoptr v1  : *mut i64
            v3 = load v2  : i64
        "#]],
    )
}

#[test]
fn i32_store() {
    check_op(
        r#"
            i32.const 1024
            i32.const 1
            i32.store
        "#,
        expect![[r#"
            v0 = const.i32 1024  : i32
            v1 = const.i32 1  : i32
            v2 = cast v0  : u32
            v3 = inttoptr v2  : *mut i32
            store v3, v1
        "#]],
    )
}

#[test]
fn i64_store() {
    check_op(
        r#"
            i32.const 1024
            i64.const 1
            i64.store
        "#,
        expect![[r#"
            v0 = const.i32 1024  : i32
            v1 = const.i64 1  : i64
            v2 = cast v0  : u32
            v3 = inttoptr v2  : *mut i64
            store v3, v1
        "#]],
    )
}

#[test]
fn i32_store8() {
    check_op(
        r#"
            i32.const 1024
            i32.const 1
            i32.store8
        "#,
        expect![[r#"
            v0 = const.i32 1024  : i32
            v1 = const.i32 1  : i32
            v2 = trunc v1  : u8
            v3 = cast v0  : u32
            v4 = inttoptr v3  : *mut u8
            store v4, v2
        "#]],
    )
}

#[test]
fn i32_store16() {
    check_op(
        r#"
            i32.const 1024
            i32.const 1
            i32.store16
        "#,
        expect![[r#"
            v0 = const.i32 1024  : i32
            v1 = const.i32 1  : i32
            v2 = trunc v1  : u16
            v3 = cast v0  : u32
            v4 = inttoptr v3  : *mut u16
            store v4, v2
        "#]],
    )
}

#[test]
fn i64_store32() {
    check_op(
        r#"
            i32.const 1024
            i64.const 1
            i64.store32
        "#,
        expect![[r#"
            v0 = const.i32 1024  : i32
            v1 = const.i64 1  : i64
            v2 = trunc v1  : u32
            v3 = cast v0  : u32
            v4 = inttoptr v3  : *mut u32
            store v4, v2
        "#]],
    )
}

#[test]
fn i32_const() {
    check_op(
        r#"
            i32.const 1
            drop
        "#,
        expect![[r#"
            v0 = const.i32 1  : i32
        "#]],
    )
}

#[test]
fn i64_const() {
    check_op(
        r#"
            i64.const 1
            drop
        "#,
        expect![[r#"
            v0 = const.i64 1  : i64
        "#]],
    )
}

#[test]
fn i32_popcnt() {
    check_op(
        r#"
            i32.const 1
            i32.popcnt
            drop
        "#,
        expect![[r#"
            v0 = const.i32 1  : i32
            v1 = popcnt v0  : i32
        "#]],
    )
}

#[test]
fn i64_extend_i32_s() {
    check_op(
        r#"
            i32.const 1
            i64.extend_i32_s
            drop
        "#,
        expect![[r#"
            v0 = const.i32 1  : i32
            v1 = sext v0  : i64
        "#]],
    )
}

#[test]
fn i64_extend_i32_u() {
    check_op(
        r#"
            i32.const 1
            i64.extend_i32_u
            drop
        "#,
        expect![[r#"
            v0 = const.i32 1  : i32
            v1 = zext v0  : i64
        "#]],
    )
}

#[test]
fn i32_wrap_i64() {
    check_op(
        r#"
            i64.const 1
            i32.wrap_i64
            drop
        "#,
        expect![[r#"
            v0 = const.i64 1  : i64
            v1 = trunc v0  : i32
        "#]],
    )
}

#[test]
fn i32_add() {
    check_op(
        r#"
            i32.const 3
            i32.const 1
            i32.add
            drop
        "#,
        expect![[r#"
            v0 = const.i32 3  : i32
            v1 = const.i32 1  : i32
            v2 = add v0, v1  : i32
        "#]],
    )
}

#[test]
fn i64_add() {
    check_op(
        r#"
            i64.const 3
            i64.const 1
            i64.add
            drop
        "#,
        expect![[r#"
            v0 = const.i64 3  : i64
            v1 = const.i64 1  : i64
            v2 = add v0, v1  : i64
        "#]],
    )
}

#[test]
fn i32_and() {
    check_op(
        r#"
            i32.const 2
            i32.const 1
            i32.and
            drop
        "#,
        expect![[r#"
            v0 = const.i32 2  : i32
            v1 = const.i32 1  : i32
            v2 = band v0, v1  : i32
        "#]],
    )
}

#[test]
fn i64_and() {
    check_op(
        r#"
            i64.const 2
            i64.const 1
            i64.and
            drop
        "#,
        expect![[r#"
            v0 = const.i64 2  : i64
            v1 = const.i64 1  : i64
            v2 = band v0, v1  : i64
        "#]],
    )
}

#[test]
fn i32_or() {
    check_op(
        r#"
            i32.const 2
            i32.const 1
            i32.or
            drop
        "#,
        expect![[r#"
            v0 = const.i32 2  : i32
            v1 = const.i32 1  : i32
            v2 = bor v0, v1  : i32
        "#]],
    )
}

#[test]
fn i64_or() {
    check_op(
        r#"
            i64.const 2
            i64.const 1
            i64.or
            drop
        "#,
        expect![[r#"
            v0 = const.i64 2  : i64
            v1 = const.i64 1  : i64
            v2 = bor v0, v1  : i64
        "#]],
    )
}

#[test]
fn i32_sub() {
    check_op(
        r#"
            i32.const 3
            i32.const 1
            i32.sub
            drop
        "#,
        expect![[r#"
            v0 = const.i32 3  : i32
            v1 = const.i32 1  : i32
            v2 = sub v0, v1  : i32
        "#]],
    )
}

#[test]
fn i64_sub() {
    check_op(
        r#"
            i64.const 3
            i64.const 1
            i64.sub
            drop
        "#,
        expect![[r#"
            v0 = const.i64 3  : i64
            v1 = const.i64 1  : i64
            v2 = sub v0, v1  : i64
        "#]],
    )
}

#[test]
fn i32_xor() {
    check_op(
        r#"
            i32.const 2
            i32.const 1
            i32.xor
            drop
        "#,
        expect![[r#"
            v0 = const.i32 2  : i32
            v1 = const.i32 1  : i32
            v2 = bxor v0, v1  : i32
        "#]],
    )
}

#[test]
fn i64_xor() {
    check_op(
        r#"
            i64.const 2
            i64.const 1
            i64.xor
            drop
        "#,
        expect![[r#"
            v0 = const.i64 2  : i64
            v1 = const.i64 1  : i64
            v2 = bxor v0, v1  : i64
        "#]],
    )
}

#[test]
fn i32_shl() {
    check_op(
        r#"
            i32.const 2
            i32.const 1
            i32.shl
            drop
        "#,
        expect![[r#"
            v0 = const.i32 2  : i32
            v1 = const.i32 1  : i32
            v2 = shl v0, v1  : i32
        "#]],
    )
}

#[test]
fn i64_shl() {
    check_op(
        r#"
            i64.const 2
            i64.const 1
            i64.shl
            drop
        "#,
        expect![[r#"
            v0 = const.i64 2  : i64
            v1 = const.i64 1  : i64
            v2 = shl v0, v1  : i64
        "#]],
    )
}

#[test]
fn i32_shr_u() {
    check_op(
        r#"
            i32.const 2
            i32.const 1
            i32.shr_u
            drop
        "#,
        expect![[r#"
            v0 = const.i32 2  : i32
            v1 = const.i32 1  : i32
            v2 = cast v0  : u32
            v3 = cast v1  : u32
            v4 = shr v2, v3  : u32
            v5 = cast v4  : i32
        "#]],
    )
}

#[test]
fn i64_shr_u() {
    check_op(
        r#"
            i64.const 2
            i64.const 1
            i64.shr_u
            drop
        "#,
        expect![[r#"
            v0 = const.i64 2  : i64
            v1 = const.i64 1  : i64
            v2 = cast v0  : u64
            v3 = cast v1  : u64
            v4 = shr v2, v3  : u64
            v5 = cast v4  : i64
        "#]],
    )
}

#[test]
fn i32_shr_s() {
    check_op(
        r#"
            i32.const 2
            i32.const 1
            i32.shr_s
            drop
        "#,
        expect![[r#"
            v0 = const.i32 2  : i32
            v1 = const.i32 1  : i32
            v2 = shr v0, v1  : i32
        "#]],
    )
}

#[test]
fn i64_shr_s() {
    check_op(
        r#"
            i64.const 2
            i64.const 1
            i64.shr_s
            drop
        "#,
        expect![[r#"
            v0 = const.i64 2  : i64
            v1 = const.i64 1  : i64
            v2 = shr v0, v1  : i64
        "#]],
    )
}

#[test]
fn i32_rotl() {
    check_op(
        r#"
            i32.const 2
            i32.const 1
            i32.rotl
            drop
        "#,
        expect![[r#"
            v0 = const.i32 2  : i32
            v1 = const.i32 1  : i32
            v2 = shl v0, v1  : i32
        "#]],
    )
}

#[test]
fn i64_rotl() {
    check_op(
        r#"
            i64.const 2
            i64.const 1
            i64.rotl
            drop
        "#,
        expect![[r#"
            v0 = const.i64 2  : i64
            v1 = const.i64 1  : i64
            v2 = shl v0, v1  : i64
        "#]],
    )
}

#[test]
fn i32_rotr() {
    check_op(
        r#"
            i32.const 2
            i32.const 1
            i32.rotr
            drop
        "#,
        expect![[r#"
            v0 = const.i32 2  : i32
            v1 = const.i32 1  : i32
            v2 = shr v0, v1  : i32
        "#]],
    )
}

#[test]
fn i64_rotr() {
    check_op(
        r#"
            i64.const 2
            i64.const 1
            i64.rotr
            drop
        "#,
        expect![[r#"
            v0 = const.i64 2  : i64
            v1 = const.i64 1  : i64
            v2 = shr v0, v1  : i64
        "#]],
    )
}

#[test]
fn i32_mul() {
    check_op(
        r#"
            i32.const 2
            i32.const 1
            i32.mul
            drop
        "#,
        expect![[r#"
            v0 = const.i32 2  : i32
            v1 = const.i32 1  : i32
            v2 = mul v0, v1  : i32
        "#]],
    )
}

#[test]
fn i64_mul() {
    check_op(
        r#"
            i64.const 2
            i64.const 1
            i64.mul
            drop
        "#,
        expect![[r#"
            v0 = const.i64 2  : i64
            v1 = const.i64 1  : i64
            v2 = mul v0, v1  : i64
        "#]],
    )
}

#[test]
fn i32_div_u() {
    check_op(
        r#"
            i32.const 2
            i32.const 1
            i32.div_u
            drop
        "#,
        expect![[r#"
            v0 = const.i32 2  : i32
            v1 = const.i32 1  : i32
            v2 = cast v0  : u32
            v3 = cast v1  : u32
            v4 = div v2, v3  : u32
            v5 = cast v4  : i32
        "#]],
    )
}

#[test]
fn i64_div_u() {
    check_op(
        r#"
            i64.const 2
            i64.const 1
            i64.div_u
            drop
        "#,
        expect![[r#"
            v0 = const.i64 2  : i64
            v1 = const.i64 1  : i64
            v2 = cast v0  : u64
            v3 = cast v1  : u64
            v4 = div v2, v3  : u64
            v5 = cast v4  : i64
        "#]],
    )
}

#[test]
fn i32_div_s() {
    check_op(
        r#"
            i32.const 2
            i32.const 1
            i32.div_s
            drop
        "#,
        expect![[r#"
            v0 = const.i32 2  : i32
            v1 = const.i32 1  : i32
            v2 = div v0, v1  : i32
        "#]],
    )
}

#[test]
fn i64_div_s() {
    check_op(
        r#"
            i64.const 2
            i64.const 1
            i64.div_s
            drop
        "#,
        expect![[r#"
            v0 = const.i64 2  : i64
            v1 = const.i64 1  : i64
            v2 = div v0, v1  : i64
        "#]],
    )
}

#[test]
fn i32_rem_u() {
    check_op(
        r#"
            i32.const 2
            i32.const 1
            i32.rem_u
            drop
        "#,
        expect![[r#"
            v0 = const.i32 2  : i32
            v1 = const.i32 1  : i32
            v2 = cast v0  : u32
            v3 = cast v1  : u32
            v4 = mod v2, v3  : u32
            v5 = cast v4  : i32
        "#]],
    )
}

#[test]
fn i64_rem_u() {
    check_op(
        r#"
            i64.const 2
            i64.const 1
            i64.rem_u
            drop
        "#,
        expect![[r#"
            v0 = const.i64 2  : i64
            v1 = const.i64 1  : i64
            v2 = cast v0  : u64
            v3 = cast v1  : u64
            v4 = mod v2, v3  : u64
            v5 = cast v4  : i64
        "#]],
    )
}

#[test]
fn i32_rem_s() {
    check_op(
        r#"
            i32.const 2
            i32.const 1
            i32.rem_s
            drop
        "#,
        expect![[r#"
            v0 = const.i32 2  : i32
            v1 = const.i32 1  : i32
            v2 = mod v0, v1  : i32
        "#]],
    )
}

#[test]
fn i64_rem_s() {
    check_op(
        r#"
            i64.const 2
            i64.const 1
            i64.rem_s
            drop
        "#,
        expect![[r#"
            v0 = const.i64 2  : i64
            v1 = const.i64 1  : i64
            v2 = mod v0, v1  : i64
        "#]],
    )
}

#[test]
fn i32_lt_u() {
    check_op(
        r#"
            i32.const 2
            i32.const 1
            i32.lt_u
            drop
        "#,
        expect![[r#"
            v0 = const.i32 2  : i32
            v1 = const.i32 1  : i32
            v2 = cast v0  : u32
            v3 = cast v1  : u32
            v4 = lt v2, v3  : i1
            v5 = cast v4  : i32
        "#]],
    )
}

#[test]
fn i64_lt_u() {
    check_op(
        r#"
            i64.const 2
            i64.const 1
            i64.lt_u
            drop
        "#,
        expect![[r#"
            v0 = const.i64 2  : i64
            v1 = const.i64 1  : i64
            v2 = cast v0  : u64
            v3 = cast v1  : u64
            v4 = lt v2, v3  : i1
            v5 = cast v4  : i32
        "#]],
    )
}

#[test]
fn i32_lt_s() {
    check_op(
        r#"
            i32.const 2
            i32.const 1
            i32.lt_s
            drop
        "#,
        expect![[r#"
            v0 = const.i32 2  : i32
            v1 = const.i32 1  : i32
            v2 = lt v0, v1  : i1
            v3 = cast v2  : i32
        "#]],
    )
}

#[test]
fn i64_lt_s() {
    check_op(
        r#"
            i64.const 2
            i64.const 1
            i64.lt_s
            drop
        "#,
        expect![[r#"
            v0 = const.i64 2  : i64
            v1 = const.i64 1  : i64
            v2 = lt v0, v1  : i1
            v3 = cast v2  : i32
        "#]],
    )
}

#[test]
fn i32_le_u() {
    check_op(
        r#"
            i32.const 2
            i32.const 1
            i32.le_u
            drop
        "#,
        expect![[r#"
            v0 = const.i32 2  : i32
            v1 = const.i32 1  : i32
            v2 = cast v0  : u32
            v3 = cast v1  : u32
            v4 = lte v2, v3  : i1
            v5 = cast v4  : i32
        "#]],
    )
}

#[test]
fn i64_le_u() {
    check_op(
        r#"
            i64.const 2
            i64.const 1
            i64.le_u
            drop
        "#,
        expect![[r#"
            v0 = const.i64 2  : i64
            v1 = const.i64 1  : i64
            v2 = cast v0  : u64
            v3 = cast v1  : u64
            v4 = lte v2, v3  : i1
            v5 = cast v4  : i32
        "#]],
    )
}

#[test]
fn i32_le_s() {
    check_op(
        r#"
            i32.const 2
            i32.const 1
            i32.le_s
            drop
        "#,
        expect![[r#"
            v0 = const.i32 2  : i32
            v1 = const.i32 1  : i32
            v2 = lte v0, v1  : i1
            v3 = cast v2  : i32
        "#]],
    )
}

#[test]
fn i64_le_s() {
    check_op(
        r#"
            i64.const 2
            i64.const 1
            i64.le_s
            drop
        "#,
        expect![[r#"
            v0 = const.i64 2  : i64
            v1 = const.i64 1  : i64
            v2 = lte v0, v1  : i1
            v3 = cast v2  : i32
        "#]],
    )
}

#[test]
fn i32_gt_u() {
    check_op(
        r#"
            i32.const 2
            i32.const 1
            i32.gt_u
            drop
        "#,
        expect![[r#"
            v0 = const.i32 2  : i32
            v1 = const.i32 1  : i32
            v2 = cast v0  : u32
            v3 = cast v1  : u32
            v4 = gt v2, v3  : i1
            v5 = cast v4  : i32
        "#]],
    )
}

#[test]
fn i64_gt_u() {
    check_op(
        r#"
            i64.const 2
            i64.const 1
            i64.gt_u
            drop
        "#,
        expect![[r#"
            v0 = const.i64 2  : i64
            v1 = const.i64 1  : i64
            v2 = cast v0  : u64
            v3 = cast v1  : u64
            v4 = gt v2, v3  : i1
            v5 = cast v4  : i32
        "#]],
    )
}

#[test]
fn i32_gt_s() {
    check_op(
        r#"
            i32.const 2
            i32.const 1
            i32.gt_s
            drop
        "#,
        expect![[r#"
            v0 = const.i32 2  : i32
            v1 = const.i32 1  : i32
            v2 = gt v0, v1  : i1
            v3 = cast v2  : i32
        "#]],
    )
}

#[test]
fn i64_gt_s() {
    check_op(
        r#"
            i64.const 2
            i64.const 1
            i64.gt_s
            drop
        "#,
        expect![[r#"
            v0 = const.i64 2  : i64
            v1 = const.i64 1  : i64
            v2 = gt v0, v1  : i1
            v3 = cast v2  : i32
        "#]],
    )
}

#[test]
fn i32_ge_u() {
    check_op(
        r#"
            i32.const 2
            i32.const 1
            i32.ge_u
            drop
        "#,
        expect![[r#"
            v0 = const.i32 2  : i32
            v1 = const.i32 1  : i32
            v2 = cast v0  : u32
            v3 = cast v1  : u32
            v4 = gte v2, v3  : i1
            v5 = cast v4  : i32
        "#]],
    )
}

#[test]
fn i64_ge_u() {
    check_op(
        r#"
            i64.const 2
            i64.const 1
            i64.ge_u
            drop
        "#,
        expect![[r#"
            v0 = const.i64 2  : i64
            v1 = const.i64 1  : i64
            v2 = cast v0  : u64
            v3 = cast v1  : u64
            v4 = gte v2, v3  : i1
            v5 = cast v4  : i32
        "#]],
    )
}

#[test]
fn i32_ge_s() {
    check_op(
        r#"
            i32.const 2
            i32.const 1
            i32.ge_s
            drop
        "#,
        expect![[r#"
            v0 = const.i32 2  : i32
            v1 = const.i32 1  : i32
            v2 = gte v0, v1  : i1
            v3 = cast v2  : i32
        "#]],
    )
}

#[test]
fn i64_ge_s() {
    check_op(
        r#"
            i64.const 2
            i64.const 1
            i64.ge_s
            drop
        "#,
        expect![[r#"
            v0 = const.i64 2  : i64
            v1 = const.i64 1  : i64
            v2 = gte v0, v1  : i1
            v3 = cast v2  : i32
        "#]],
    )
}

#[test]
fn i32_eqz() {
    check_op(
        r#"
            i32.const 2
            i32.eqz
            drop
        "#,
        expect![[r#"
            v0 = const.i32 2  : i32
            v1 = eq v0, 0  : i1
            v2 = cast v1  : i32
        "#]],
    )
}

#[test]
fn i64_eqz() {
    check_op(
        r#"
            i64.const 2
            i64.eqz
            drop
        "#,
        expect![[r#"
            v0 = const.i64 2  : i64
            v1 = eq v0, 0  : i1
            v2 = cast v1  : i32
        "#]],
    )
}

#[test]
fn i32_eq() {
    check_op(
        r#"
            i32.const 2
            i32.const 1
            i32.eq
            drop
        "#,
        expect![[r#"
            v0 = const.i32 2  : i32
            v1 = const.i32 1  : i32
            v2 = eq v0, v1  : i1
            v3 = cast v2  : i32
        "#]],
    )
}

#[test]
fn i64_eq() {
    check_op(
        r#"
            i64.const 2
            i64.const 1
            i64.eq
            drop
        "#,
        expect![[r#"
            v0 = const.i64 2  : i64
            v1 = const.i64 1  : i64
            v2 = eq v0, v1  : i1
            v3 = cast v2  : i32
        "#]],
    )
}

#[test]
fn i32_ne() {
    check_op(
        r#"
            i32.const 2
            i32.const 1
            i32.ne
            drop
        "#,
        expect![[r#"
            v0 = const.i32 2  : i32
            v1 = const.i32 1  : i32
            v2 = neq v0, v1  : i1
            v3 = cast v2  : i32
        "#]],
    )
}

#[test]
fn i64_ne() {
    check_op(
        r#"
            i64.const 2
            i64.const 1
            i64.ne
            drop
        "#,
        expect![[r#"
            v0 = const.i64 2  : i64
            v1 = const.i64 1  : i64
            v2 = neq v0, v1  : i1
            v3 = cast v2  : i32
        "#]],
    )
}

#[test]
fn select_i32() {
    check_op(
        r#"
            i64.const 3
            i64.const 7
            i32.const 42
            select
            drop
        "#,
        expect![[r#"
            v0 = const.i64 3  : i64
            v1 = const.i64 7  : i64
            v2 = const.i32 42  : i32
            v3 = neq v2, 0  : i1
            v4 = select v3, v0, v1  : i64
        "#]],
    )
}
