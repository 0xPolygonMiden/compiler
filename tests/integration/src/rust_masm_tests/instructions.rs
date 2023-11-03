use expect_test::expect_file;
use miden_hir::Felt;
use proptest::prelude::*;
use proptest::test_runner::TestRunner;

use crate::execute_emulator;
use crate::execute_vm;
use crate::CompilerTest;

macro_rules! test_op {
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
                TestRunner::default()
                    .run(&(any::<$op_ty>(), any::<$op_ty>()), move |(a, b)| {
                        let rust_out = (a $op b) as u64;
                        let mut args = [Felt::from(a), Felt::from(b)];
                        args.reverse();
                        let vm_out = execute_vm(&vm_program, &args).first().unwrap().clone();
                        prop_assert_eq!(rust_out, vm_out);
                        args.reverse();
                        let emul_out = execute_emulator(ir_masm.clone(), &args)
                            .first()
                            .unwrap()
                            .clone();
                        prop_assert_eq!(rust_out, emul_out);
                        Ok(())
                    })
                    .unwrap();
            }
        });
    };
}

macro_rules! test_comparison_op {
    ($name:ident, $op:tt, $op_ty:tt) => {
        test_op!($name, $op, $op_ty, bool);
    };
}

#[allow(unused_macros)]
macro_rules! test_arith_op {
    ($name:ident, $op:tt, $op_ty:tt) => {
        test_op!($name, $op, $op_ty, $op_ty);
    };
}

// test_comparison_op!(ge, >=, u32);
test_comparison_op!(ge, >=, u16);
test_comparison_op!(ge, >=, u8);

test_comparison_op!(gt, >, u16);
test_comparison_op!(gt, >, u8);

test_comparison_op!(le, <=, u16);
test_comparison_op!(le, <=, u8);

test_comparison_op!(lt, <, u16);
test_comparison_op!(lt, <, u8);

test_comparison_op!(eq, ==, u32);
test_comparison_op!(eq, ==, u16);
test_comparison_op!(eq, ==, u8);

// enable when i32 ops support is merged https://github.com/0xPolygonMiden/compiler/pull/37
// test_arith_op!(add, +, u32);
// test_arith_op!(add, +, u16);
// test_arith_op!(add, +, u8);
// test_arith_op!(sub, -, u32);
// ...
