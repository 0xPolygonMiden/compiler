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

fn hash_string(inputs: &[&str]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    for input in inputs {
        hasher.update(input);
    }
    format!("{:x}", hasher.finalize())
}

fn compile_wasm(rust_source: &str) -> Vec<u8> {
    use std::fs;
    use std::process::Command;

    let rustc_opts = [
        "-C",
        "opt-level=z", // optimize for size
        "--target",
        "wasm32-unknown-unknown",
    ];

    // include rustc_opts in the hash to ensure that the output file changes when options change
    let file_name = hash_string(&[&rustc_opts.concat(), rust_source]);

    let temp_dir = std::env::temp_dir().join("miden-frontend-wasm-rust-comp-tests");
    if !temp_dir.exists() {
        fs::create_dir(&temp_dir).unwrap();
    }
    let input_file = temp_dir.join(format!("{file_name}.rs"));
    let output_file = temp_dir.join(format!("{file_name}.wasm"));

    // skip compilation if the output file already exists
    if output_file.exists() {
        return fs::read(output_file).unwrap();
    }

    fs::write(&input_file, rust_source).unwrap();

    let output = Command::new("rustc")
        .args(&rustc_opts)
        .arg(&input_file)
        .arg("-o")
        .arg(&output_file)
        .output()
        .expect("Failed to execute rustc.");

    if !output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        panic!("Rust to Wasm compilation failed!");
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
fn rust_add() {
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

            pub fn add(i32, i32) -> i32 {
            block0(v0: i32, v1: i32):
                v3 = add v1, v0  : i32
                br block1(v3)

            block1(v2: i32):
                v4 = ret v2  : ()
            }

            pub fn __main() -> i32 {
            block0:
                v1 = const.i32 1  : i32
                v2 = const.i32 2  : i32
                v3 = call noname::add(v1, v2)  : i32
                br block1(v3)

            block1(v0: i32):
                v4 = ret v0  : ()
            }
        "#]],
    );
}

#[test]
fn rust_fib() {
    check_ir(
        include_str!("rust_source/fib.rs"),
        expect![[r#"
            (module
              (type (;0;) (func (param i32) (result i32)))
              (type (;1;) (func (result i32)))
              (func $fib (;0;) (type 0) (param i32) (result i32)
                (local i32 i32 i32)
                i32.const 0
                local.set 1
                i32.const 1
                local.set 2
                loop (result i32) ;; label = @1
                  local.get 2
                  local.set 3
                  block ;; label = @2
                    local.get 0
                    br_if 0 (;@2;)
                    local.get 1
                    return
                  end
                  local.get 0
                  i32.const -1
                  i32.add
                  local.set 0
                  local.get 1
                  local.get 3
                  i32.add
                  local.set 2
                  local.get 3
                  local.set 1
                  br 0 (;@1;)
                end
              )
              (func $__main (;1;) (type 1) (result i32)
                i32.const 25
                call $fib
              )
              (memory (;0;) 16)
              (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
              (global (;1;) i32 i32.const 1048576)
              (global (;2;) i32 i32.const 1048576)
              (export "memory" (memory 0))
              (export "fib" (func $fib))
              (export "__main" (func $__main))
              (export "__data_end" (global 1))
              (export "__heap_base" (global 2))
            )"#]],
        expect![[r#"
            module noname

            pub fn fib(i32) -> i32 {
            block0(v0: i32):
                v2 = const.i32 0  : i32
                v3 = const.i32 0  : i32
                v4 = const.i32 1  : i32
                br block2(v4, v0, v3)

            block1(v1: i32):

            block2(v6: i32, v7: i32, v10: i32):
                v8 = const.i32 0  : i32
                v9 = neq v7, v8  : i1
                condbr v9, block4, block5

            block3(v5: i32):

            block4:
                v12 = const.i32 -1  : i32
                v13 = add v7, v12  : i32
                v14 = add v10, v6  : i32
                br block2(v14, v13, v6)

            block5:
                v11 = ret v10  : ()
            }

            pub fn __main() -> i32 {
            block0:
                v1 = const.i32 25  : i32
                v2 = call noname::fib(v1)  : i32
                br block1(v2)

            block1(v0: i32):
                v3 = ret v0  : ()
            }
        "#]],
    );
}
