use expect_test::expect;
use std::sync::Arc;

use miden_diagnostics::term::termcolor::ColorChoice;
use miden_diagnostics::CodeMap;
use miden_diagnostics::DefaultEmitter;
use miden_diagnostics::DiagnosticsConfig;
use miden_diagnostics::DiagnosticsHandler;
use miden_diagnostics::Emitter;
use miden_diagnostics::NullEmitter;
use miden_diagnostics::Verbosity;
use miden_frontend_wasm::translate_module;
use miden_frontend_wasm::WasmTranslationConfig;

fn hash_string(input: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(input);
    format!("{:x}", hasher.finalize())
}

fn compile_wasm(rust_source: &str) -> Vec<u8> {
    use std::fs;
    use std::process::Command;

    let file_name = hash_string(rust_source);
    let temp_dir = std::env::temp_dir();
    let input_file = temp_dir.join(format!("{file_name}.rs"));
    let output_file = temp_dir.join(format!("{file_name}.wasm"));

    // skip compilation if the output file already exists
    if output_file.exists() {
        return fs::read(output_file).unwrap();
    }

    fs::write(&input_file, rust_source).unwrap();

    let output = Command::new("rustc")
        .args(&[
            "--target",
            "wasm32-unknown-unknown",
            input_file.to_str().unwrap(),
            "-o",
            output_file.to_str().unwrap(),
            "-O",
        ])
        .output()
        .expect("Failed to execute rustc.");

    if !output.status.success() {
        panic!(
            "Compilation failed: {:?}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    fs::read(output_file).unwrap()
}

fn check_ir(
    rust_source: &str,
    expected_wat: expect_test::Expect,
    expected_ir: expect_test::Expect,
) {
    let wasm_bytes = compile_wasm(rust_source);
    let wat = wasmprinter::print_bytes(&wasm_bytes).unwrap();
    expected_wat.assert_eq(&wat);
    let codemap = Arc::new(CodeMap::new());
    let diagnostics = DiagnosticsHandler::new(
        DiagnosticsConfig {
            verbosity: Verbosity::Debug,
            warnings_as_errors: false,
            no_warn: false,
            display: Default::default(),
        },
        codemap,
        default_emitter(Verbosity::Debug, ColorChoice::Auto),
    );
    let module =
        translate_module(&wasm_bytes, &WasmTranslationConfig::default(), &diagnostics).unwrap();
    expected_ir.assert_eq(&module.to_string());
}

fn default_emitter(verbosity: Verbosity, color: ColorChoice) -> Arc<dyn Emitter> {
    match verbosity {
        Verbosity::Silent => Arc::new(NullEmitter::new(color)),
        _ => Arc::new(DefaultEmitter::new(color)),
    }
}

#[test]
fn test_rust_add() {
    check_ir(
        include_str!("rust_source/add.rs"),
        expect![[r#"
            (module
              (type (;0;) (func (param i32 i32) (result i32)))
              (type (;1;) (func (result i32)))
              (func $add (;0;) (type 0) (param i32 i32) (result i32)
                local.get 1
                local.get 0
                i32.add
              )
              (func $__main (;1;) (type 1) (result i32)
                i32.const 1
                i32.const 2
                call $add
              )
              (memory (;0;) 16)
              (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
              (global (;1;) i32 i32.const 1048576)
              (global (;2;) i32 i32.const 1048576)
              (export "memory" (memory 0))
              (export "add" (func $add))
              (export "__main" (func $__main))
              (export "__data_end" (global 1))
              (export "__heap_base" (global 2))
            )"#]],
        expect![[r#"
            module noname

            pub fn add(i32, i32) -> i32  {
            block0(v0: i32, v1: i32):
                v3 = add v1, v0  : i32
                br block1(v3)

            block1(v2: i32):
                v4 = ret v2  : ()
            }

            pub fn __main() -> i32  {
            block0:
                v1 = const.int 1  : i32
                v2 = const.int 2  : i32
                v3 = call add(v1, v2)  : i32
                br block1(v3)

            block1(v0: i32):
                v4 = ret v0  : ()
            }
        "#]],
    );
}
