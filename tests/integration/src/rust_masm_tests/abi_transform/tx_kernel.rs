use std::fmt::Write;

use expect_test::expect_file;
use miden_assembly::LibraryPath;
use miden_core::{Felt, FieldElement};
use miden_processor::ExecutionError;

use crate::{exec_vm::execute_vm_tracing, execute_emulator, execute_vm, CompilerTest};

#[allow(unused)]
fn setup_log() {
    use log::LevelFilter;
    let _ = env_logger::builder()
        .filter_level(LevelFilter::Trace)
        .format_timestamp(None)
        .is_test(true)
        .try_init();
}

fn test_get_inputs(test_name: &str, expected_inputs: Vec<Felt>) {
    assert!(expected_inputs.len() == 4, "for now only word-sized inputs are supported");
    let mut main_fn = String::new();
    writeln!(main_fn, "() -> Vec<Felt> {{\n").unwrap();
    writeln!(main_fn, "    let inputs = get_inputs();").unwrap();
    for (i, expected_input) in expected_inputs.iter().enumerate() {
        writeln!(main_fn, "    assert_eq(inputs[{i}], felt!({expected_input}));").unwrap();
    }
    writeln!(main_fn, "    inputs").unwrap();
    writeln!(main_fn, "}}").unwrap();

    let artifact_name = format!("abi_transform_tx_kernel_get_inputs_{}", test_name);
    let mut test = CompilerTest::rust_fn_body_with_sdk(&artifact_name, &main_fn, true);
    let mut masm = String::new();
    writeln!(masm, "export.get_inputs").unwrap();
    for expected_input in expected_inputs.iter() {
        writeln!(masm, "    push.{expected_input}").unwrap();
    }
    // copy the pointer to the top of the stack
    writeln!(masm, "    dup.4").unwrap();
    writeln!(masm, "    mem_storew").unwrap();
    // push the inputs len on the stack
    writeln!(masm, "    push.{}", expected_inputs.len()).unwrap();
    writeln!(masm, "    end").unwrap();
    test.link_masm_modules = vec![(LibraryPath::new("miden::note").unwrap(), masm)];

    // Test expected compilation artifacts
    test.expect_wasm(expect_file![format!("../../../expected/{artifact_name}.wat")]);
    test.expect_ir(expect_file![format!("../../../expected/{artifact_name}.hir")]);
    test.expect_masm(expect_file![format!("../../../expected/{artifact_name}.masm")]);

    let vm_program = test.vm_masm_program();
    let vm_out = execute_vm_tracing(&vm_program, &[]).unwrap();
    // let vm_out = execute_vm(&vm_program, &[]);

    // let ir_program = test.ir_masm_program();
    // let emul_out = execute_emulator(ir_program.clone(), &[]);
}

#[test]
fn test_get_inputs_4() {
    test_get_inputs("4", vec![u32::MAX.into(), Felt::ONE, Felt::ZERO, u32::MAX.into()]);
}
