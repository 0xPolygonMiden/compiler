use arbitrary::Unstructured;
use prop::collection::vec;
use proptest::{prelude::*, test_runner::TestRunner};
use wasm_smith::{Config, InstructionKind, InstructionKinds};

use crate::{compiler_test::wasm_to_wat, CompilerTest};

fn wasm_smith_default_config() -> Config {
    Config {
        allow_start_export: false,
        allowed_instructions: InstructionKinds::new(&[InstructionKind::Control]),
        bulk_memory_enabled: false,
        exceptions_enabled: false,
        gc_enabled: false,
        max_aliases: 0,
        max_data_segments: 0,
        max_element_segments: 0,
        max_elements: 0,
        max_exports: 0,
        max_funcs: 1,
        max_imports: 0,
        max_table_elements: 0,
        max_tables: 0,
        max_tags: 0,
        memory64_enabled: false,
        min_funcs: 1,
        multi_value_enabled: false,
        reference_types_enabled: false,
        relaxed_simd_enabled: false,
        saturating_float_to_int_enabled: false,
        sign_extension_ops_enabled: false,
        simd_enabled: false,
        tail_call_enabled: false,
        ..Config::default()
    }
}

#[test]
fn simple_ctrl() {
    TestRunner::default()
        .run(&vec(0..=255u8, 100), move |bytes| {
            let config = wasm_smith_default_config();
            let mut wasm_module =
                wasm_smith::Module::new(config, &mut Unstructured::new(&bytes)).unwrap();
            wasm_module.ensure_termination(100).unwrap();
            let wasm_module_bytes = wasm_module.to_bytes();
            let wat = wasm_to_wat(&wasm_module_bytes);
            eprintln!("wat:\n{}", wat);
            let mut test = CompilerTest::wasm_module(wasm_module_bytes);
            let vm_program = &test.masm_program();
            eprintln!("vm_program: {}", vm_program);
            // let rust_out = miden_integration_tests_rust_fib::fib(a);
            // let args = [Felt::from(a)];
            // let vm_out: u32 = (*execute_vm(vm_program, &args).first().unwrap()).into();
            // prop_assert_eq!(rust_out, vm_out);
            Ok(())
        })
        .unwrap();
}
