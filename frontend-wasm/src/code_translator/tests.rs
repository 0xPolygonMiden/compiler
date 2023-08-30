use expect_test::expect;
use miden_ir::hir::write_instruction;

use crate::test_utils::test_diagnostics;
use crate::translate_module;
use crate::WasmTranslationConfig;

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
    let fref = module.get_funcref_by_name("test_wrapper").unwrap();
    let func = module.get_function(fref).unwrap();
    let entry_block = func.dfg.entry_block().unwrap();
    let entry_block_data = func.dfg.block_data(entry_block);
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
fn memory_grow() {
    check_op(
        r#"
            i32.const 1
            memory.grow
            drop
        "#,
        expect![[r#"
            v0 = const.int 1  : i32
            v1 = const.int 1048575  : i32
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
            v0 = const.int 1048575  : i32
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
            v0 = const.int 1024  : i32
            v1 = inttoptr v0  : *mut i8
            v2 = load v1  : i8
            v3 = zext v2  : i32
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
            v0 = const.int 1024  : i32
            v1 = inttoptr v0  : *mut i16
            v2 = load v1  : i16
            v3 = zext v2  : i32
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
            v0 = const.int 1024  : i32
            v1 = inttoptr v0  : *mut i8
            v2 = load v1  : i8
            v3 = sext v2  : i32
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
            v0 = const.int 1024  : i32
            v1 = inttoptr v0  : *mut i16
            v2 = load v1  : i16
            v3 = sext v2  : i32
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
            v0 = const.int 1024  : i32
            v1 = inttoptr v0  : *mut i8
            v2 = load v1  : i8
            v3 = zext v2  : i64
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
            v0 = const.int 1024  : i32
            v1 = inttoptr v0  : *mut i16
            v2 = load v1  : i16
            v3 = zext v2  : i64
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
            v0 = const.int 1024  : i32
            v1 = inttoptr v0  : *mut i8
            v2 = load v1  : i8
            v3 = sext v2  : i64
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
            v0 = const.int 1024  : i32
            v1 = inttoptr v0  : *mut i16
            v2 = load v1  : i16
            v3 = sext v2  : i64
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
            v0 = const.int 1024  : i32
            v1 = inttoptr v0  : *mut i32
            v2 = load v1  : i32
            v3 = sext v2  : i64
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
            v0 = const.int 1024  : i32
            v1 = inttoptr v0  : *mut i32
            v2 = load v1  : i32
            v3 = zext v2  : i64
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
            v0 = const.int 1024  : i32
            v1 = inttoptr v0  : *mut i32
            v2 = load v1  : i32
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
            v0 = const.int 1024  : i32
            v1 = inttoptr v0  : *mut i64
            v2 = load v1  : i64
        "#]],
    )
}

#[test]
fn f64_load() {
    check_op(
        r#"
            i32.const 1024
            f64.load
            drop
        "#,
        expect![[r#"
            v0 = const.int 1024  : i32
            v1 = inttoptr v0  : *mut f64
            v2 = load v1  : f64
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
            v0 = const.int 1024  : i32
            v1 = const.int 1  : i32
            v2 = inttoptr v0  : *mut i32
            store v2, v1
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
            v0 = const.int 1024  : i32
            v1 = const.int 1  : i64
            v2 = inttoptr v0  : *mut i64
            store v2, v1
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
            v0 = const.int 1024  : i32
            v1 = const.int 1  : i32
            v2 = trunc v1  : i8
            v3 = inttoptr v0  : *mut i8
            store v3, v2
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
            v0 = const.int 1024  : i32
            v1 = const.int 1  : i32
            v2 = trunc v1  : i16
            v3 = inttoptr v0  : *mut i16
            store v3, v2
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
            v0 = const.int 1024  : i32
            v1 = const.int 1  : i64
            v2 = trunc v1  : i32
            v3 = inttoptr v0  : *mut i32
            store v3, v2
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
            v0 = const.int 1  : i32
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
            v0 = const.int 1  : i64
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
            v0 = const.int 3  : i32
            v1 = const.int 1  : i32
            v2 = add v0, v1  : i32
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
            v0 = const.int 3  : i32
            v1 = const.int 1  : i32
            v2 = sub v0, v1  : i32
        "#]],
    )
}
