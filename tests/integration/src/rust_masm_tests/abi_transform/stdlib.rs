use core::panic;

use expect_test::expect_file;
use proptest::{
    arbitrary::any,
    test_runner::{TestError, TestRunner},
};

use crate::CompilerTest;

#[ignore = "until the VM stack overflow during the MASM generation is fixed"]
#[test]
fn test_blake3_hash() {
    let main_fn = format!(
        "(a: [u8; 32], b: [u8; 32]) -> [u8; 32] {{ miden_prelude::blake3_hash_2to1(a, b) }}"
    );
    let artifact_name = "abi_transform_stdlib_blake3_hash";
    let mut test = CompilerTest::rust_fn_body_with_prelude(&artifact_name, &main_fn, true);
    // Test expected compilation artifacts
    test.expect_wasm(expect_file![format!("../../../expected/{artifact_name}.wat")]);
    test.expect_ir(expect_file![format!("../../../expected/{artifact_name}.hir")]);
    test.expect_masm(expect_file![format!("../../../expected/{artifact_name}.masm")]);
    // let ir_masm = test.ir_masm_program();
    // let vm_program = test.vm_masm_program();

    // Run the Rust and compiled MASM code against a bunch of random inputs and compare the results
    let res =
        TestRunner::default().run(&(any::<[u8; 32]>(), any::<[u8; 32]>()), move |(_a, _b)| {
            todo!("test against rust");
            // run_masm_vs_rust(rs_out, &vm_program, ir_masm.clone(), &args)
        });
    match res {
        Err(TestError::Fail(_, value)) => {
            panic!("Found minimal(shrinked) failing case: {:?}", value);
        }
        Ok(_) => (),
        _ => panic!("Unexpected test result: {:?}", res),
    }
}
