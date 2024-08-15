use core::panic;
use std::collections::VecDeque;

use expect_test::expect_file;
use miden_core::utils::group_slice_elements;
use midenc_debug::{MidenExecutor, PopFromStack, PushToStack, TestFelt};
use midenc_hir::Felt;
use proptest::{
    arbitrary::any,
    prop_assert_eq,
    test_runner::{TestError, TestRunner},
};

use crate::CompilerTest;

#[test]
#[ignore = "pending rodata fixes"]
fn test_blake3_hash() {
    let main_fn =
        "(a: [u8; 32]) -> [u8; 32] {  miden_stdlib_sys::blake3_hash_1to1(a) }".to_string();
    let artifact_name = "abi_transform_stdlib_blake3_hash";
    let mut test = CompilerTest::rust_fn_body_with_stdlib_sys(
        artifact_name,
        &main_fn,
        true,
        ["--test-harness".into()],
    );
    // Test expected compilation artifacts
    test.expect_wasm(expect_file![format!("../../../expected/{artifact_name}.wat")]);
    test.expect_ir(expect_file![format!("../../../expected/{artifact_name}.hir")]);
    test.expect_masm(expect_file![format!("../../../expected/{artifact_name}.masm")]);
    let ir_program = test.ir_masm_program();
    let vm_program = test.vm_masm_program();

    let advice_inputs = ir_program.advice_inputs();

    println!("{ir_program}");

    // Run the Rust and compiled MASM code against a bunch of random inputs and compare the results
    let res = TestRunner::default().run(&any::<[u8; 32]>(), move |ibytes| {
        let hash_bytes = blake3::hash(&ibytes);
        let rs_out = hash_bytes.as_bytes();
        let mut frame = Vec::<Felt>::default();
        PushToStack::try_push(&ibytes, &mut frame); // words
        PushToStack::try_push(&2u32, &mut frame); // num_words
        PushToStack::try_push(&0u32, &mut frame); // dest_ptr
                                                  //let rs_ofelts = group_slice_elements::<u8, 4>(rs_out)
                                                  //    .iter()
                                                  //    .map(|&bytes| u32::from_le_bytes(bytes).into())
                                                  //    .collect::<Vec<TestFelt>>();
                                                  //let ifelts = group_slice_elements::<u8, 4>(&ibytes)
                                                  //    .iter()
                                                  //    .map(|&bytes| u32::from_le_bytes(bytes).into())
                                                  //    .collect::<Vec<Felt>>();
        dbg!(&ibytes, &frame, rs_out);
        // Arguments are: [hash_input_ptr, hash_output_ptr]
        let mut exec = MidenExecutor::new(vec![Felt::new(0), Felt::new(128 * 1024)]);
        let mut advice_inputs = advice_inputs.clone();
        advice_inputs.extend_stack(frame);
        exec.with_advice_inputs(advice_inputs);
        let trace = exec.execute(&vm_program, &test.session);
        let vm_out: [u8; 32] = trace
            .read_from_rust_memory(128 * 1024)
            .expect("expected memory to have been written");
        dbg!(&vm_out);
        prop_assert_eq!(rs_out, &vm_out, "VM output mismatch");
        Ok(())
    });

    match res {
        Err(TestError::Fail(_, value)) => {
            panic!("Found minimal(shrinked) failing case: {:?}", value);
        }
        Ok(_) => (),
        _ => panic!("Unexpected test result: {:?}", res),
    }
}
