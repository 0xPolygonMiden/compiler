use expect_test::expect;
use miden_hir::write_instruction;
use miden_hir::Ident;

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
    let func = module.function(Ident::from_str("test_wrapper")).unwrap();
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
fn memory_grow() {
    check_op(
        r#"
            i32.const 1
            memory.grow
            drop
        "#,
        expect![[r#"
            v0 = const.i32 1  : i32
            v1 = const.i32 1048575  : i32
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
            v0 = const.i32 1024  : i32
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
            v0 = const.i32 1024  : i32
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
            v0 = const.i32 1024  : i32
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
            v0 = const.i32 1024  : i32
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
            v0 = const.i32 1024  : i32
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
            v0 = const.i32 1024  : i32
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
            v0 = const.i32 1024  : i32
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
            v0 = const.i32 1024  : i32
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
            v0 = const.i32 1024  : i32
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
            v0 = const.i32 1024  : i32
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
            v0 = const.i32 1024  : i32
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
            v0 = const.i32 1024  : i32
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
            v0 = const.i32 1024  : i32
            v1 = const.i32 1  : i32
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
            v0 = const.i32 1024  : i32
            v1 = const.i64 1  : i64
            v2 = inttoptr v0  : *mut i64
            store v2, v1
        "#]],
    )
}

#[test]
fn f64_store() {
    check_op(
        r#"
            i32.const 1024
            f64.const 1.9
            f64.store
        "#,
        expect![[r#"
            v0 = const.i32 1024  : i32
            v1 = const.f64 1.9  : f64
            v2 = inttoptr v0  : *mut f64
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
            v0 = const.i32 1024  : i32
            v1 = const.i32 1  : i32
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
            v0 = const.i32 1024  : i32
            v1 = const.i32 1  : i32
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
            v0 = const.i32 1024  : i32
            v1 = const.i64 1  : i64
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
fn f64_const() {
    check_op(
        r#"
            f64.const 1.9
            drop
        "#,
        expect![[r#"
            v0 = const.f64 1.9  : f64
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
            v2 = shr v0, v1  : i32
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

#[ignore = "not implemented"]
#[test]
fn f64_add() {
    check_op(
        r#"
            f64.const 2.5
            f64.const 1.9
            f64.add
            drop
        "#,
        expect![[r#"
            v0 = const.float 2.5  : f64
            v1 = const.float 1.9  : f64
            v2 = add v0, v1  : f64
        "#]],
    )
}

#[ignore = "not implemented"]
#[test]
fn f64_sub() {
    check_op(
        r#"
            f64.const 2.5
            f64.const 1.9
            f64.sub
            drop
        "#,
        expect![[r#"
            v0 = const.float 2.5  : f64
            v1 = const.float 1.9  : f64
            v2 = sub v0, v1  : f64
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

#[ignore = "not implemented"]
#[test]
fn f64_mul() {
    check_op(
        r#"
            f64.const 2.5
            f64.const 1.9
            f64.mul
            drop
        "#,
        expect![[r#"
            v0 = const.float 2.5  : f64
            v1 = const.float 1.9  : f64
            v2 = mul v0, v1  : f64
        "#]],
    )
}

#[ignore = "not implemented"]
#[test]
fn f64_div() {
    check_op(
        r#"
            f64.const 2.5
            f64.const 1.9
            f64.div
            drop
        "#,
        expect![[r#"
            v0 = const.float 2.5  : f64
            v1 = const.float 1.9  : f64
            v2 = div v0, v1  : f64
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
            v2 = div v0, v1  : i32
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
            v2 = mod v0, v1  : i32
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
            v2 = mod v0, v1  : i64
        "#]],
    )
}

#[ignore = "not implemented"]
#[test]
fn f64_min() {
    check_op(
        r#"
            f64.const 2.5
            f64.const 1.9
            f64.min
            drop
        "#,
        expect![[r#"
            v0 = const.float 2.5  : f64
            v1 = const.float 1.9  : f64
            v2 = min v0, v1  : f64
        "#]],
    )
}

#[ignore = "not implemented"]
#[test]
fn f64_max() {
    check_op(
        r#"
            f64.const 2.5
            f64.const 1.9
            f64.max
            drop
        "#,
        expect![[r#"
            v0 = const.float 2.5  : f64
            v1 = const.float 1.9  : f64
            v2 = max v0, v1  : f64
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
            v2 = lt v0, v1  : i1
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
            v2 = lt v0, v1  : i1
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
            v2 = lte v0, v1  : i1
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
            v2 = lte v0, v1  : i1
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
            v2 = gt v0, v1  : i1
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
            v2 = gt v0, v1  : i1
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
            v2 = gte v0, v1  : i1
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
            v2 = gte v0, v1  : i1
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
            v1 = const.i32 0  : i32
            v2 = eq v0, v1  : i1
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
            v1 = const.i64 0  : i64
            v2 = eq v0, v1  : i1
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
        "#]],
    )
}

#[ignore = "not implemented"]
#[test]
fn f64_eq() {
    check_op(
        r#"
            f64.const 2.5
            f64.const 1.9
            f64.eq
            drop
        "#,
        expect![[r#"
            v0 = const.f64 2.5  : f64
            v1 = const.f64 1.9  : f64
            v2 = eq v0, v1  : i1
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
        "#]],
    )
}

#[ignore = "not implemented"]
#[test]
fn f64_ne() {
    check_op(
        r#"
            f64.const 2.5
            f64.const 1.9
            f64.ne
            drop
        "#,
        expect![[r#"
            v0 = const.f64 2.5  : f64
            v1 = const.f64 1.9  : f64
            v2 = neq v0, v1  : i1
        "#]],
    )
}

#[ignore = "not implemented"]
#[test]
fn f64_gt() {
    check_op(
        r#"
            f64.const 2.5
            f64.const 1.9
            f64.gt
            drop
        "#,
        expect![[r#"
            v0 = const.float 2.5  : f64
            v1 = const.float 1.9  : f64
            v2 = gt v0, v1  : f64
        "#]],
    )
}

#[ignore = "not implemented"]
#[test]
fn f64_ge() {
    check_op(
        r#"
            f64.const 2.5
            f64.const 1.9
            f64.ge
            drop
        "#,
        expect![[r#"
            v0 = const.float 2.5  : f64
            v1 = const.float 1.9  : f64
            v2 = gte v0, v1  : f64
        "#]],
    )
}

#[ignore = "not implemented"]
#[test]
fn f64_le() {
    check_op(
        r#"
            f64.const 2.5
            f64.const 1.9
            f64.le
            drop
        "#,
        expect![[r#"
            v0 = const.float 2.5  : f64
            v1 = const.float 1.9  : f64
            v2 = lte v0, v1  : f64
        "#]],
    )
}

#[ignore = "not implemented"]
#[test]
fn f64_lt() {
    check_op(
        r#"
            f64.const 2.5
            f64.const 1.9
            f64.lt
            drop
        "#,
        expect![[r#"
            v0 = const.float 2.5  : f64
            v1 = const.float 1.9  : f64
            v2 = lt v0, v1  : f64
        "#]],
    )
}
