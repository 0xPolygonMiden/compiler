use core::panic;

use expect_test::expect_file;
use proptest::{
    arbitrary::any,
    test_runner::{TestError, TestRunner},
};

use crate::{felt_conversion::TestFelt, rust_masm_tests::run_masm_vs_rust, CompilerTest};

#[test]
fn test_felt_plus() {
    let main_fn = format!("(a: Felt, b: Felt) -> Felt {{ a + b }}");
    let artifact_name = "felt_plus";
    let mut test = CompilerTest::rust_fn_body_with_prelude(&artifact_name, &main_fn);
    // Test expected compilation artifacts
    test.expect_wasm(expect_file![format!("../../expected/{artifact_name}.wat")]);
    test.expect_ir(expect_file![format!("../../expected/{artifact_name}.hir")]);
    test.expect_masm(expect_file![format!("../../expected/{artifact_name}.masm")]);
    let ir_masm = test.ir_masm_program();
    let vm_program = test.vm_masm_program();

    // Run the Rust and compiled MASM code against a bunch of random inputs and compare the results
    // TODO: generate proper felt values, i.e. in the range [0, M)
    let res = TestRunner::default().run(&(any::<u32>(), any::<u32>()), move |(a, b)| {
        let rust_out = a.wrapping_add(b);
        let args = [TestFelt::from(a).0, TestFelt::from(b).0];
        run_masm_vs_rust(rust_out, &vm_program, ir_masm.clone(), &args)
    });
    match res {
        Err(TestError::Fail(_, value)) => {
            panic!("Found minimal(shrinked) failing case: {:?}", value);
        }
        Ok(_) => (),
        _ => panic!("Unexpected test result: {:?}", res),
    }
}
