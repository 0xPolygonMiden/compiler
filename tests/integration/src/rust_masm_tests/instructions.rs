use std::sync::Arc;

use crate::felt_conversion::TestFelt;
use expect_test::expect_file;
use miden_core::Felt;
use proptest::prelude::*;
use proptest::test_runner::TestError;
use proptest::test_runner::TestRunner;

use crate::execute_emulator;
use crate::execute_vm;
use crate::CompilerTest;

macro_rules! test_bin_op {
    ($name:ident, $op:tt, $op_ty:tt, $res_ty:tt) => {
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
                    .run(&(any::<$op_ty>(), any::<$op_ty>()), move |(a, b)| {
                        let rust_out = a $op b;
                        dbg!(&rust_out);
                        let args = [TestFelt::from(a).0, TestFelt::from(b).0];
                        run_masm(rust_out, &vm_program, ir_masm.clone(), &args)
                    });
                match res {
                    Err(TestError::Fail(_, value)) => {
                        println!("Found minimal(shrinked) failing case: {:?}", value);
                    },
                    Ok(_) => (),
                    _ => panic!("Unexpected test result: {:?}", res),
    }
            }
        });
    };
}

macro_rules! test_unary_op {
    ($name:ident, $op:tt, $op_ty:tt) => {
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
                    .run(&(any::<$op_ty>()), move |a| {
                        let rust_out = $op a;
                        dbg!(&rust_out);
                        let args = [TestFelt::from(a).0];
                        run_masm(rust_out, &vm_program, ir_masm.clone(), &args)
                    });
                match res {
                    Err(TestError::Fail(_, value)) => {
                        println!("Found minimal(shrinked) failing case: {:?}", value);
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
                    .run(&(any::<$a_ty>(), any::<$b_ty>()), move |(a, b)| {
                        let rust_out = $func(a, b);
                        dbg!(&rust_out);
                        let args = [TestFelt::from(a).0, TestFelt::from(b).0];
                        run_masm(rust_out, &vm_program, ir_masm.clone(), &args)
                    });
                match res {
                    Err(TestError::Fail(_, value)) => {
                        println!("Found minimal(shrinked) failing case: {:?}", value);
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
    let vm_out: T = execute_vm(&vm_program, &args)
        .first()
        .unwrap()
        .clone()
        .into();
    prop_assert_eq!(rust_out.clone(), vm_out, "VM output mismatch");
    let emul_out: T = execute_emulator(ir_masm.clone(), &args)
        .first()
        .unwrap()
        .clone()
        .into();
    prop_assert_eq!(rust_out, emul_out, "Emulator output mismatch");
    Ok(())
}

macro_rules! test_bool_op {
    ($name:ident, $op:tt, $op_ty:tt) => {
        test_bin_op!($name, $op, $op_ty, bool);
    };
}

#[allow(unused_macros)]
macro_rules! test_int_op {
    ($name:ident, $op:tt, $op_ty:tt) => {
        test_bin_op!($name, $op, $op_ty, $op_ty);
    };
}

// add u64 and i8, i16, i32, i64 tests when they are implemented in the codegen
// test_bool_op!(ge, >=, u64);
test_bool_op!(ge, >=, u32);
test_bool_op!(ge, >=, u16);
test_bool_op!(ge, >=, u8);
// test_bool_op!(ge, >=, i64);
// test_bool_op!(ge, >=, i32);
// test_bool_op!(ge, >=, i16);
// test_bool_op!(ge, >=, i8);

// test_bool_op!(gt, >, u64);
test_bool_op!(gt, >, u32);
test_bool_op!(gt, >, u16);
test_bool_op!(gt, >, u8);
// test_bool_op!(gt, >, i64);
// test_bool_op!(gt, >, i32);
// test_bool_op!(gt, >, i16);
// test_bool_op!(gt, >, i8);

// test_bool_op!(le, <=, u64);
test_bool_op!(le, <=, u32);
test_bool_op!(le, <=, u16);
test_bool_op!(le, <=, u8);
// test_bool_op!(le, <=, i64);
// test_bool_op!(le, <=, i32);
// test_bool_op!(le, <=, i16);
// test_bool_op!(le, <=, i8);

// test_bool_op!(lt, <, u64);
test_bool_op!(lt, <, u32);
test_bool_op!(lt, <, u16);
test_bool_op!(lt, <, u8);
// test_bool_op!(lt, <, i64);
// test_bool_op!(lt, <, i32);
// test_bool_op!(lt, <, i16);
// test_bool_op!(lt, <, i8);

test_bool_op!(eq, ==, u64);
test_bool_op!(eq, ==, u32);
test_bool_op!(eq, ==, u16);
test_bool_op!(eq, ==, u8);
test_bool_op!(eq, ==, i64);
test_bool_op!(eq, ==, i32);
test_bool_op!(eq, ==, i16);
test_bool_op!(eq, ==, i8);

// test_int_op!(add, +, u64);
test_int_op!(add, +, u32);
test_int_op!(add, +, u16);
test_int_op!(add, +, u8);
// test_int_op!(add, +, i64);
test_int_op!(add, +, i32);
test_int_op!(add, +, i16);
test_int_op!(add, +, i8);

// test_int_op!(sub, -, u64);
test_int_op!(sub, -, u32);
test_int_op!(sub, -, u16);
test_int_op!(sub, -, u8);
// test_int_op!(sub, -, i64);
test_int_op!(sub, -, i32);
test_int_op!(sub, -, i16);
test_int_op!(sub, -, i8);

// test_int_op!(mul, *, u64);
// test_int_op!(mul, *, u32);

// ...
// add tests for mul, div, rem,

test_bool_op!(and, &&, bool);
test_bool_op!(or, ||, bool);
test_bool_op!(xor, ^, bool);

// enable after miden stdlib is linked (missing `use` in the IR)
// test_int_op!(and, &, u64);
// test_int_op!(and, &, i64);
// add tests for or, xor

test_int_op!(and, &, u8);
test_int_op!(and, &, u16);
test_int_op!(and, &, u32);
test_int_op!(and, &, i8);
test_int_op!(and, &, i16);
test_int_op!(and, &, i32);

test_int_op!(or, |, u8);
test_int_op!(or, |, u16);
test_int_op!(or, |, u32);
test_int_op!(or, |, i8);
test_int_op!(or, |, i16);
test_int_op!(or, |, i32);

test_int_op!(xor, ^, u8);
test_int_op!(xor, ^, u16);
test_int_op!(xor, ^, u32);
test_int_op!(xor, ^, i8);
test_int_op!(xor, ^, i16);
test_int_op!(xor, ^, i32);

// enable when implemented in the codegen for i32
// test_int_op!(shl, <<, u8);
// test_int_op!(shl, <<, u16);
// test_int_op!(shl, <<, u32);
// test_int_op!(shl, <<, i8);
// test_int_op!(shl, <<, i16);
// test_int_op!(shl, <<, i32);

test_int_op!(shr, >>, u8);
test_int_op!(shr, >>, u16);
test_int_op!(shr, >>, u32);
// enable when implemented in the codegen for i32
// test_int_op!(shr, >>, i8);
// test_int_op!(shr, >>, i16);
// test_int_op!(shr, >>, i32);

// enable when subtraction is implemented in the codegen for i32
// test_unary_op!(neg, -, i32);

// enable when stdlib is linked
// test_unary_op!(not, !, u64);
// test_unary_op!(not, !, i64);

test_unary_op!(not, !, i32);
test_unary_op!(not, !, i16);
test_unary_op!(not, !, i8);
test_unary_op!(not, !, u32);
test_unary_op!(not, !, u16);
test_unary_op!(not, !, u8);

test_unary_op!(not, !, bool);

// enable when https://github.com/0xPolygonMiden/compiler/issues/56 is fixed
// test_func_two_arg!(min, core::cmp::min, i32, i32, i32);
// test_func_two_arg!(min, core::cmp::min, u32, u32, u32);
// test_func_two_arg!(min, core::cmp::min, u8, u8, u8);
// test_func_two_arg!(max, core::cmp::max, u8, u8, u8);
