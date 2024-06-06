#![allow(unused)]
use expect_test::expect_file;

use crate::{execute_vm, CompilerTest};

#[allow(unused)]
fn setup_log() {
    use log::LevelFilter;
    let _ = env_logger::builder()
        .filter_level(LevelFilter::Trace)
        .format_timestamp(None)
        .is_test(true)
        .try_init();
}

#[test]
fn test_get_inputs() {
    // setup_log();
    let main_fn = "() -> Vec<Felt> { get_inputs() }";
    let artifact_name = "abi_transform_tx_kernel_get_inputs";
    let mut test = CompilerTest::rust_fn_body_with_sdk(artifact_name, main_fn, true);
    // Test expected compilation artifacts
    test.expect_wasm(expect_file![format!("../../../expected/{artifact_name}.wat")]);
    test.expect_ir(expect_file![format!("../../../expected/{artifact_name}.hir")]);
    test.expect_masm(expect_file![format!("../../../expected/{artifact_name}.masm")]);
    // let vm_program = test.vm_masm_program();

    // let _vm_out = execute_vm(&vm_program, &[]);
    // Run the Rust and compiled MASM code against a bunch of random inputs and compare the results
    // let res =
    //     TestRunner::default().run(&(any::<[u8; 32]>(), any::<[u8; 32]>()), move |(_a, _b)| {
    //         todo!("test against rust");
    //         // run_masm_vs_rust(rs_out, &vm_program, ir_masm.clone(), &args)
    //     });
    // match res {
    //     Err(TestError::Fail(_, value)) => {
    //         panic!("Found minimal(shrinked) failing case: {:?}", value);
    //     }
    //     Ok(_) => (),
    //     _ => panic!("Unexpected test result: {:?}", res),
    // }
}
