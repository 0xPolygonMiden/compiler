use std::sync::Arc;

use expect_test::expect_file;
use miden_core::Felt;
use proptest::{
    prelude::*,
    test_runner::{TestError, TestRunner},
};

use crate::{execute_emulator, execute_vm, felt_conversion::TestFelt, CompilerTest};

macro_rules! test_bin_op {
    ($name:ident, $op:tt, $op_ty:tt, $res_ty:tt, $a_range:expr, $b_range:expr) => {
        concat_idents::concat_idents!(test_name = $name, _, $op_ty {
            #[test]
            fn test_name() {
                let op_str = stringify!($op);
                let op_ty_str = stringify!($op_ty);
                let res_ty_str = stringify!($res_ty);
                let main_fn = format!("(a: {op_ty_str}, b: {op_ty_str}) -> {res_ty_str} {{ a {op_str} b }}");
                let mut test = CompilerTest::rust_fn_body(&main_fn);
                // Test expected compilation artifacts
                let artifact_name = format!("{}_{}", stringify!($name), stringify!($op_ty));
                test.expect_wasm(expect_file![format!("../../expected/{artifact_name}.wat")]);
                test.expect_ir(expect_file![format!("../../expected/{artifact_name}.hir")]);
                test.expect_masm(expect_file![format!("../../expected/{artifact_name}.masm")]);
                let ir_masm = test.ir_masm_program();
                let vm_program = test.vm_masm_program();

                // Run the Rust and compiled MASM code against a bunch of random inputs and compare the results
                let res = TestRunner::default()
                    .run(&($a_range, $b_range), move |(a, b)| {
                        dbg!(a, b);
                        let rs_out = a $op b;
                        dbg!(&rs_out);
                        let args = [TestFelt::from(a).0, TestFelt::from(b).0];
                        run_masm_vs_rust(rs_out, &vm_program, ir_masm.clone(), &args)
                    });
                match res {
                    Err(TestError::Fail(_, value)) => {
                        panic!("Found minimal(shrinked) failing case: {:?}", value);
                    },
                    Ok(_) => (),
                    _ => panic!("Unexpected test result: {:?}", res),
    }
            }
        });
    };
}

macro_rules! test_unary_op {
    ($name:ident, $op:tt, $op_ty:tt, $range:expr) => {
        concat_idents::concat_idents!(test_name = $name, _, $op_ty {
            #[test]
            fn test_name() {
                let op_str = stringify!($op);
                let op_ty_str = stringify!($op_ty);
                let res_ty_str = stringify!($op_ty);
                let main_fn = format!("(a: {op_ty_str}) -> {res_ty_str} {{ {op_str}a }}");
                let mut test = CompilerTest::rust_fn_body(&main_fn);
                // Test expected compilation artifacts
                let artifact_name = format!("{}_{}", stringify!($name), stringify!($op_ty));
                test.expect_wasm(expect_file![format!("../../expected/{artifact_name}.wat")]);
                test.expect_ir(expect_file![format!("../../expected/{artifact_name}.hir")]);
                test.expect_masm(expect_file![format!("../../expected/{artifact_name}.masm")]);
                let ir_masm = test.ir_masm_program();
                let vm_program = test.vm_masm_program();

                // Run the Rust and compiled MASM code against a bunch of random inputs and compare the results
                let res = TestRunner::default()
                    .run(&($range), move |a| {
                        let rs_out = $op a;
                        dbg!(&rs_out);
                        let args = [TestFelt::from(a).0];
                        run_masm_vs_rust(rs_out, &vm_program, ir_masm.clone(), &args)
                    });
                match res {
                    Err(TestError::Fail(_, value)) => {
                        panic!("Found minimal(shrinked) failing case: {:?}", value);
                    },
                    Ok(_) => (),
                    _ => panic!("Unexpected test result: {:?}", res),
    }
            }
        });
    };
}

#[allow(unused_macros)]
macro_rules! test_func_two_arg {
    ($name:ident, $func:path, $a_ty:tt, $b_ty:tt, $res_ty:tt) => {
        concat_idents::concat_idents!(test_name = $name, _, $a_ty, _, $b_ty {
            #[test]
            fn test_name() {
                let func_name_str = stringify!($func);
                let a_ty_str = stringify!($a_ty);
                let b_ty_str = stringify!($b_ty);
                let res_ty_str = stringify!($res_ty);
                let main_fn = format!("(a: {a_ty_str}, b: {b_ty_str}) -> {res_ty_str} {{ {func_name_str}(a, b) }}");
                let mut test = CompilerTest::rust_fn_body(&main_fn);
                // Test expected compilation artifacts
                let artifact_name = format!("{}_{}_{}", stringify!($func), stringify!($a_ty), stringify!($b_ty));
                test.expect_wasm(expect_file![format!("../../expected/{artifact_name}.wat")]);
                test.expect_ir(expect_file![format!("../../expected/{artifact_name}.hir")]);
                test.expect_masm(expect_file![format!("../../expected/{artifact_name}.masm")]);
                let ir_masm = test.ir_masm_program();
                let vm_program = test.vm_masm_program();

                // Run the Rust and compiled MASM code against a bunch of random inputs and compare the results
                let res = TestRunner::default()
                    .run(&(0..$a_ty::MAX/2, any::<$b_ty>()), move |(a, b)| {
                        let rust_out = $func(a, b);
                        dbg!(&rust_out);
                        let args = [TestFelt::from(a).0, TestFelt::from(b).0];
                        run_masm(rust_out, &vm_program, ir_masm.clone(), &args)
                    });
                match res {
                    Err(TestError::Fail(_, value)) => {
                        panic!("Found minimal(shrinked) failing case: {:?}", value);
                    },
                    Ok(_) => (),
                    _ => panic!("Unexpected test result: {:?}", res),
    }
            }
        });
    };
}
fn run_masm<T>(
    rust_out: T,
    vm_program: &miden_core::Program,
    ir_masm: Arc<miden_codegen_masm::Program>,
    args: &[Felt],
) -> Result<(), TestCaseError>
where
    T: Clone + From<TestFelt> + std::cmp::PartialEq + std::fmt::Debug,
{
    let vm_out: T = execute_vm(&vm_program, &args).first().unwrap().clone().into();
    prop_assert_eq!(rust_out.clone(), vm_out, "VM output mismatch");
    let emul_out: T = execute_emulator(ir_masm.clone(), &args).first().unwrap().clone().into();
    prop_assert_eq!(rust_out, emul_out, "Emulator output mismatch");
    Ok(())
}

macro_rules! test_bool_op_total {
    ($name:ident, $op:tt, $op_ty:tt) => {
        test_bin_op!($name, $op, $op_ty, bool, any::<$op_ty>(), any::<$op_ty>());
    };
}

macro_rules! test_int_op {
    ($name:ident, $op:tt, $op_ty:tt, $a_range:expr, $b_range:expr) => {
        test_bin_op!($name, $op, $op_ty, $op_ty, $a_range, $b_range);
    };
}

macro_rules! test_int_op_total {
    ($name:ident, $op:tt, $op_ty:tt) => {
        test_bin_op!($name, $op, $op_ty, $op_ty, any::<$op_ty>(), any::<$op_ty>());
    };
}

macro_rules! test_unary_op_total {
    ($name:ident, $op:tt, $op_ty:tt) => {
        test_unary_op!($name, $op, $op_ty, any::<$op_ty>());
    };
}

// 64-bit ops are not implemented yet
// test_bool_op!(ge, >=, u64);
// test_bool_op!(ge, >=, i64);
// test_bool_op!(gt, >, u64);
// test_bool_op!(gt, >, i64);
// test_bool_op!(le, <=, u64);
// test_bool_op!(le, <=, i64);
// test_bool_op!(lt, <, u64);
// test_bool_op!(lt, <, i64);
// test_int_op!(add, +, u64);
// test_int_op!(add, +, i64);
// test_int_op!(sub, -, u64);
// test_int_op!(sub, -, i64);
// test_int_op!(mul, *, u64);
// test_int_op!(mul, *, i64);
// test_int_op!(div, /, u64);
// test_int_op!(div, /, i64);
// test_int_op!(rem, %, u64);
// test_int_op!(rem, %, i64);
// test_unary_op!(neg, -, u64);
// test_unary_op!(neg, -, i64);
// test_unary_op!(not, !, u64);
// test_unary_op!(not, !, i64);
// test_int_op!(shl, <<, u64);
// test_int_op!(shl, <<, i64);
// test_int_op!(shr, >>, u64);
// test_int_op!(shr, >>, i64);
// test_unary_op!(neg, -, i64);

// MASM compilation error (missing import for intrinsic)
//
// Comparison ops
//
// test_bool_op!(ge, >=, i32);
// test_bool_op!(ge, >=, i16);
// test_bool_op!(ge, >=, i8);
//
// test_bool_op!(gt, >, i32);
// test_bool_op!(gt, >, i16);
// test_bool_op!(gt, >, i8);
//
// test_bool_op!(le, <=, i32);
// test_bool_op!(le, <=, i16);
// test_bool_op!(le, <=, i8);
//
// test_bool_op!(lt, <, i32);
// test_bool_op!(lt, <, i16);
// test_bool_op!(lt, <, i8);
//
// Arithmetic ops
//
// test_int_op!(mul, *, u32);
// test_int_op!(mul, *, u16);
// test_int_op!(mul, *, u8);
// test_int_op!(mul, *, i32);
// test_int_op!(mul, *, i16);
// test_int_op!(mul, *, i8);
//
// Bitwise ops
//
// test_int_op!(shr, >>, i8);
// test_int_op!(shr, >>, i16);
// test_int_op!(shr, >>, i32);
//
// test_unary_op!(not, !, u64);
// test_unary_op!(not, !, i64);

// stdlib is not linked (missing import for stdlib)
// test_int_op!(and, &, u64);
// test_int_op!(and, &, i64);
// test_int_op!(or, |, u64);
// test_int_op!(or, |, i64);
// test_int_op!(xor, ^, u64);
// test_int_op!(xor, ^, i64);

// TODO: build with cargo to avoid core::panicking
// TODO: separate macro for div and rem tests to filter out division by zero
// test_int_op!(div, /, u32);
// ...
// add tests for div, rem,

// enable when https://github.com/0xPolygonMiden/compiler/issues/56 is fixed
//test_func_two_arg!(min, core::cmp::min, i32, i32, i32);
// test_func_two_arg!(min, core::cmp::min, u32, u32, u32);
// test_func_two_arg!(min, core::cmp::min, u8, u8, u8);
// test_func_two_arg!(max, core::cmp::max, u8, u8, u8);

// TODO: fails, when a or b => i32::MAX, see https://github.com/0xPolygonMiden/compiler/issues/174
// test_bool_op!(ge, >=, u32);
test_bool_op_total!(ge, >=, u16);
test_bool_op_total!(ge, >=, u8);

// TODO: fails, when a or b => i32::MAX, see https://github.com/0xPolygonMiden/compiler/issues/174
// test_bool_op!(gt, >, u32);
test_bool_op_total!(gt, >, u16);
test_bool_op_total!(gt, >, u8);

// TODO: fails, when a or b => i32::MAX, see https://github.com/0xPolygonMiden/compiler/issues/174
// test_bool_op!(le, <=, u32);
test_bool_op_total!(le, <=, u16);
test_bool_op_total!(le, <=, u8);

// TODO: fails, when a or b => i32::MAX, see https://github.com/0xPolygonMiden/compiler/issues/174
// test_bool_op!(lt, <, u32);
test_bool_op_total!(lt, <, u16);
test_bool_op_total!(lt, <, u8);

test_bool_op_total!(eq, ==, u64);
test_bool_op_total!(eq, ==, u32);
test_bool_op_total!(eq, ==, u16);
test_bool_op_total!(eq, ==, u8);
test_bool_op_total!(eq, ==, i64);
test_bool_op_total!(eq, ==, i32);
test_bool_op_total!(eq, ==, i16);
test_bool_op_total!(eq, ==, i8);

test_int_op!(add, +, u32, 0..=u32::MAX/2, 0..=u32::MAX/2);
test_int_op!(add, +, u16, 0..=u16::MAX/2, 0..=u16::MAX/2);
test_int_op!(add, +, u8, 0..=u8::MAX/2, 0..=u8::MAX/2);
test_int_op!(add, +, i32, 0..=i32::MAX/2, 0..=i32::MAX/2);
test_int_op!(add, +, i16, 0..=i16::MAX/2, 0..=i16::MAX/2);
test_int_op!(add, +, i8, 0..=i8::MAX/2, 0..=i8::MAX/2);

test_int_op!(sub, -, u32, u32::MAX/2..=u32::MAX, 0..=u32::MAX/2);
test_int_op!(sub, -, u16, u16::MAX/2..=u16::MAX, 0..=u16::MAX/2);
test_int_op!(sub, -, u8, u8::MAX/2..=u8::MAX, 0..=u8::MAX/2);
test_int_op!(sub, -, i32, i32::MIN..=0, i32::MIN..=0);
test_int_op!(sub, -, i16, i16::MIN..=0, i16::MIN..=0);
test_int_op!(sub, -, i8, i8::MIN..=0, i8::MIN..=0);

test_bool_op_total!(and, &&, bool);
test_bool_op_total!(or, ||, bool);
test_bool_op_total!(xor, ^, bool);

test_int_op_total!(and, &, u8);
test_int_op_total!(and, &, u16);
test_int_op_total!(and, &, u32);
test_int_op_total!(and, &, i8);
test_int_op_total!(and, &, i16);
test_int_op_total!(and, &, i32);

test_int_op_total!(or, |, u8);
test_int_op_total!(or, |, u16);
test_int_op_total!(or, |, u32);
test_int_op_total!(or, |, i8);
test_int_op_total!(or, |, i16);
test_int_op_total!(or, |, i32);

test_int_op_total!(xor, ^, u8);
test_int_op_total!(xor, ^, u16);
test_int_op_total!(xor, ^, u32);
test_int_op_total!(xor, ^, i8);
test_int_op_total!(xor, ^, i16);
test_int_op_total!(xor, ^, i32);

test_int_op!(shl, <<, u8, 0..u8::MAX, 0..8);
test_int_op!(shl, <<, u16, 0..u16::MAX, 0..16);
test_int_op!(shl, <<, u32, 0..u32::MAX, 0..32);
test_int_op!(shl, <<, i8, 0..i8::MAX, 0..8);
test_int_op!(shl, <<, i16, 0..i16::MAX, 0..16);
test_int_op!(shl, <<, i32, 0..i32::MAX, 0..32);

test_int_op!(shr, >>, u8, 0..u8::MAX, 0..8);
test_int_op!(shr, >>, u16, 0..u16::MAX, 0..16);
// TODO: fails, when a or b => i32::MAX, see https://github.com/0xPolygonMiden/compiler/issues/174
// test_int_op!(shr, >>, u32, 0..u32::MAX, 0..32);

test_unary_op!(neg, -, i32, (i32::MIN + 1)..=i32::MAX);
test_unary_op!(neg, -, i16, (i16::MIN + 1)..=i16::MAX);
test_unary_op!(neg, -, i8, (i8::MIN + 1)..=i8::MAX);

test_unary_op_total!(not, !, i32);
test_unary_op_total!(not, !, i16);
test_unary_op_total!(not, !, i8);
test_unary_op_total!(not, !, u32);
test_unary_op_total!(not, !, u16);
test_unary_op_total!(not, !, u8);

test_unary_op_total!(not, !, bool);
