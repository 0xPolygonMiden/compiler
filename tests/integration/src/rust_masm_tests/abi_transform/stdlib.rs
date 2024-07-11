use core::panic;

use expect_test::expect_file;
use miden_core::utils::group_slice_elements;
use midenc_hir::Felt;
use proptest::{
    arbitrary::any,
    prop_assert_eq,
    test_runner::{TestError, TestRunner},
};

use crate::{execute_vm, felt_conversion::TestFelt, CompilerTest};

#[ignore = "until the VM stack overflow during the MASM generation is fixed"]
#[test]
fn test_blake3_hash() {
    let main_fn = "(a: [u8; 32], b: [u8; 32]) -> [u8; 32] { miden_stdlib_sys::blake3_hash_2to1(a, \
                   b) }"
        .to_string();
    let artifact_name = "abi_transform_stdlib_blake3_hash";
    let mut test = CompilerTest::rust_fn_body_with_stdlib_sys(artifact_name, &main_fn, true);
    // Test expected compilation artifacts
    test.expect_wasm(expect_file![format!("../../../expected/{artifact_name}.wat")]);
    test.expect_ir(expect_file![format!("../../../expected/{artifact_name}.hir")]);
    test.expect_masm(expect_file![format!("../../../expected/{artifact_name}.masm")]);
    let vm_program = test.vm_masm_program();

    // Run the Rust and compiled MASM code against a bunch of random inputs and compare the results
    let res = TestRunner::default().run(&any::<[u8; 64]>(), move |ibytes| {
        let hash_bytes = blake3::hash(&ibytes);
        let rs_out = hash_bytes.as_bytes();
        let rs_ofelts = group_slice_elements::<u8, 4>(rs_out)
            .iter()
            .map(|&bytes| u32::from_le_bytes(bytes).into())
            .collect::<Vec<TestFelt>>();
        let ifelts = group_slice_elements::<u8, 4>(&ibytes)
            .iter()
            .map(|&bytes| u32::from_le_bytes(bytes).into())
            .collect::<Vec<Felt>>();
        let vm_out = execute_vm(&vm_program, &ifelts);
        prop_assert_eq!(rs_ofelts, vm_out, "VM output mismatch");
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
