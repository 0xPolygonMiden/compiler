use expect_test::expect;
use expect_test::expect_file;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::process::Command;
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
    let rustc_opts = [
        "-C",
        "opt-level=z", // optimize for size
        "--target",
        "wasm32-unknown-unknown",
    ];
    let file_name = hash_string(&[rust_source]);
    let temp_dir = std::env::temp_dir();
    let input_file = temp_dir.join(format!("{file_name}.rs"));
    let output_file = temp_dir.join(format!("{file_name}.wasm"));
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
    let wasm = fs::read(&output_file).unwrap();
    fs::remove_file(&input_file).unwrap();
    fs::remove_file(&output_file).unwrap();
    return wasm;
}

pub fn check_ir_files_cargo(
    bin_name: &str,
    expected_wat_file: expect_test::ExpectFile,
    expected_ir_file: expect_test::ExpectFile,
) {
    let bundle_name = "rust-wasm-tests";
    let manifest_path = format!("../tests/{}/Cargo.toml", bundle_name);
    // dbg!(&pwd);
    let temp_dir = std::env::temp_dir();
    let target_dir = temp_dir.join(format!("{bundle_name}-cargo/"));
    let output = Command::new("cargo")
        .arg(format!(
            "+{}",
            std::env::var("CARGO_MAKE_TOOLCHAIN").unwrap()
        ))
        .arg("build")
        .arg("--manifest-path")
        .arg(manifest_path)
        .arg("--release")
        .arg("--bins")
        .arg("--target=wasm32-unknown-unknown")
        .arg("--features=wasm-target")
        .arg("--target-dir")
        .arg(target_dir.clone())
        // compile std as part of crate graph compilation
        // https://doc.rust-lang.org/cargo/reference/unstable.html#build-std
        .arg("-Z")
        .arg("build-std=core,alloc")
        .arg("-Z")
        // abort on panic without message formatting (core::fmt uses call_indirect)
        .arg("build-std-features=panic_immediate_abort")
        .output()
        .expect("Failed to execute cargo build.");
    if !output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        panic!("Rust to Wasm compilation failed!");
    }
    let target_bin_file_path = Path::new(&target_dir)
        .join("wasm32-unknown-unknown")
        .join("release")
        .join(bin_name)
        .with_extension("wasm");
    let mut target_bin_file = fs::File::open(target_bin_file_path).unwrap();
    let mut wasm_bytes = vec![];
    Read::read_to_end(&mut target_bin_file, &mut wasm_bytes).unwrap();
    fs::remove_dir_all(target_dir).unwrap();
    check_ir_files_wasm(wasm_bytes, expected_wat_file, expected_ir_file);
}

fn check_ir_files_wasm(
    wasm_bytes: Vec<u8>,
    expected_wat_file: expect_test::ExpectFile,
    expected_ir_file: expect_test::ExpectFile,
) {
    let wat = demangle(&wasm_to_wat(&wasm_bytes));
    expected_wat_file.assert_eq(&wat);
    let module = translate(wasm_bytes);
    let ir = demangle(&module.to_string());
    expected_ir_file.assert_eq(&ir);
}

fn check_ir(
    rust_source: &str,
    expected_wat: expect_test::Expect,
    expected_ir: expect_test::Expect,
) {
    let wasm_bytes = compile_wasm(rust_source);
    let wat = demangle(&wasm_to_wat(&wasm_bytes));
    expected_wat.assert_eq(&wat);
    let module = translate(wasm_bytes);
    let ir = demangle(&module.to_string());
    expected_ir.assert_eq(&ir);
}

#[allow(dead_code)]
fn check_ir_files(
    rust_source: &str,
    expected_wat_file: expect_test::ExpectFile,
    expected_ir_file: expect_test::ExpectFile,
) {
    let wasm_bytes = compile_wasm(rust_source);
    check_ir_files_wasm(wasm_bytes, expected_wat_file, expected_ir_file);
}

fn wasm_to_wat(wasm_bytes: &Vec<u8>) -> String {
    let mut wasm_printer = wasmprinter::Printer::new();
    // disable printing of the "producers" section because it contains a rustc version
    // to not brake tests when rustc is updated
    wasm_printer.add_custom_section_printer("producers", |_, _, _| Ok(()));
    let wat = wasm_printer.print(wasm_bytes.as_ref()).unwrap();
    wat
}

fn translate(wasm_bytes: Vec<u8>) -> miden_hir::Module {
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
    module
}

fn default_emitter(verbosity: Verbosity, color: ColorChoice) -> Arc<dyn Emitter> {
    match verbosity {
        Verbosity::Silent => Arc::new(NullEmitter::new(color)),
        _ => Arc::new(DefaultEmitter::new(color)),
    }
}

fn demangle(name: &str) -> String {
    let mut input = name.as_bytes();
    let mut demangled = Vec::new();
    let include_hash = false;
    rustc_demangle::demangle_stream(&mut input, &mut demangled, include_hash).unwrap();
    String::from_utf8(demangled).unwrap()
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

            const $0 = 0x00100000;

            global external @__stack_pointer : i32 = $0 { id = 0 };
            global external @gv1 : i32 = $0 { id = 1 };
            global external @gv2 : i32 = $0 { id = 2 };

            pub fn add(i32, i32) -> i32 {
            block0(v0: i32, v1: i32):
                v3 = add.wrapping v1, v0 : i32;
                br block1(v3);

            block1(v2: i32):
                ret v2;
            }

            pub fn __main() -> i32 {
            block0:
                v1 = const.i32 1 : i32;
                v2 = const.i32 2 : i32;
                v3 = call noname::add(v1, v2) : i32;
                br block1(v3);

            block1(v0: i32):
                ret v0;
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

            const $0 = 0x00100000;

            global external @__stack_pointer : i32 = $0 { id = 0 };
            global external @gv1 : i32 = $0 { id = 1 };
            global external @gv2 : i32 = $0 { id = 2 };

            pub fn fib(i32) -> i32 {
            block0(v0: i32):
                v2 = const.i32 0 : i32;
                v3 = const.i32 0 : i32;
                v4 = const.i32 1 : i32;
                br block2(v4, v0, v3);

            block1(v1: i32):

            block2(v6: i32, v7: i32, v9: i32):
                v8 = neq v7, 0 : i1;
                condbr v8, block4, block5;

            block3(v5: i32):

            block4:
                v10 = const.i32 -1 : i32;
                v11 = add.wrapping v7, v10 : i32;
                v12 = add.wrapping v9, v6 : i32;
                br block2(v12, v11, v6);

            block5:
                ret v9;
            }

            pub fn __main() -> i32 {
            block0:
                v1 = const.i32 25 : i32;
                v2 = call noname::fib(v1) : i32;
                br block1(v2);

            block1(v0: i32):
                ret v0;
            }
        "#]],
    );
}

#[test]
fn rust_enum() {
    check_ir(
        include_str!("rust_source/enum.rs"),
        expect![[r#"
            (module
              (type (;0;) (func (param i32 i32 i32) (result i32)))
              (type (;1;) (func (result i32)))
              (func $match_enum (;0;) (type 0) (param i32 i32 i32) (result i32)
                block ;; label = @1
                  block ;; label = @2
                    block ;; label = @3
                      local.get 2
                      i32.const 255
                      i32.and
                      br_table 0 (;@3;) 1 (;@2;) 2 (;@1;) 0 (;@3;)
                    end
                    local.get 1
                    local.get 0
                    i32.add
                    return
                  end
                  local.get 0
                  local.get 1
                  i32.sub
                  return
                end
                local.get 1
                local.get 0
                i32.mul
              )
              (func $__main (;1;) (type 1) (result i32)
                i32.const 3
                i32.const 5
                i32.const 0
                call $match_enum
                i32.const 3
                i32.const 5
                i32.const 1
                call $match_enum
                i32.add
                i32.const 3
                i32.const 5
                i32.const 2
                call $match_enum
                i32.add
              )
              (memory (;0;) 16)
              (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
              (global (;1;) i32 i32.const 1048576)
              (global (;2;) i32 i32.const 1048576)
              (export "memory" (memory 0))
              (export "match_enum" (func $match_enum))
              (export "__main" (func $__main))
              (export "__data_end" (global 1))
              (export "__heap_base" (global 2))
            )"#]],
        expect![[r#"
            module noname

            const $0 = 0x00100000;

            global external @__stack_pointer : i32 = $0 { id = 0 };
            global external @gv1 : i32 = $0 { id = 1 };
            global external @gv2 : i32 = $0 { id = 2 };

            pub fn match_enum(i32, i32, i32) -> i32 {
            block0(v0: i32, v1: i32, v2: i32):
                v4 = const.i32 255 : i32;
                v5 = band v2, v4 : i32;
                v6 = cast v5 : u32;
                switch v6 {
                    0 => block4,
                    1 => block3,
                    2 => block2,
                    _ => block4
                };

            block1(v3: i32):
                ret v3;

            block2:
                v9 = mul.wrapping v1, v0 : i32;
                br block1(v9);

            block3:
                v8 = sub.wrapping v0, v1 : i32;
                ret v8;

            block4:
                v7 = add.wrapping v1, v0 : i32;
                ret v7;
            }

            pub fn __main() -> i32 {
            block0:
                v1 = const.i32 3 : i32;
                v2 = const.i32 5 : i32;
                v3 = const.i32 0 : i32;
                v4 = call noname::match_enum(v1, v2, v3) : i32;
                v5 = const.i32 3 : i32;
                v6 = const.i32 5 : i32;
                v7 = const.i32 1 : i32;
                v8 = call noname::match_enum(v5, v6, v7) : i32;
                v9 = add.wrapping v4, v8 : i32;
                v10 = const.i32 3 : i32;
                v11 = const.i32 5 : i32;
                v12 = const.i32 2 : i32;
                v13 = call noname::match_enum(v10, v11, v12) : i32;
                v14 = add.wrapping v9, v13 : i32;
                br block1(v14);

            block1(v0: i32):
                ret v0;
            }
        "#]],
    )
}

#[test]
fn rust_array() {
    check_ir(
        include_str!("rust_source/array.rs"),
        expect![[r#"
            (module
              (type (;0;) (func (param i32 i32) (result i32)))
              (type (;1;) (func (result i32)))
              (func $sum_arr (;0;) (type 0) (param i32 i32) (result i32)
                (local i32)
                i32.const 0
                local.set 2
                block ;; label = @1
                  local.get 1
                  i32.eqz
                  br_if 0 (;@1;)
                  loop ;; label = @2
                    local.get 0
                    i32.load
                    local.get 2
                    i32.add
                    local.set 2
                    local.get 0
                    i32.const 4
                    i32.add
                    local.set 0
                    local.get 1
                    i32.const -1
                    i32.add
                    local.tee 1
                    br_if 0 (;@2;)
                  end
                end
                local.get 2
              )
              (func $__main (;1;) (type 1) (result i32)
                i32.const 1048576
                i32.const 5
                call $sum_arr
                i32.const 1048596
                i32.const 5
                call $sum_arr
                i32.add
              )
              (memory (;0;) 17)
              (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
              (global (;1;) i32 i32.const 1048616)
              (global (;2;) i32 i32.const 1048624)
              (export "memory" (memory 0))
              (export "sum_arr" (func $sum_arr))
              (export "__main" (func $__main))
              (export "__data_end" (global 1))
              (export "__heap_base" (global 2))
              (data $.rodata (;0;) (i32.const 1048576) "\01\00\00\00\02\00\00\00\03\00\00\00\04\00\00\00\05\00\00\00\06\00\00\00\07\00\00\00\08\00\00\00\09\00\00\00\0a\00\00\00")
            )"#]],
        expect![[r#"
            module noname

            memory {
                segment @0x100000 x 40 = 0x0000000a000000090000000800000007000000060000000500000004000000030000000200000001;
            }

            const $0 = 0x00100000;
            const $1 = 0x00100028;
            const $2 = 0x00100030;

            global external @__stack_pointer : i32 = $0 { id = 0 };
            global external @gv1 : i32 = $1 { id = 1 };
            global external @gv2 : i32 = $2 { id = 2 };

            pub fn sum_arr(i32, i32) -> i32 {
            block0(v0: i32, v1: i32):
                v3 = const.i32 0 : i32;
                v4 = const.i32 0 : i32;
                v5 = eq v1, 0 : i1;
                v6 = cast v5 : i32;
                v7 = neq v6, 0 : i1;
                condbr v7, block2(v4), block3;

            block1(v2: i32):
                ret v2;

            block2(v20: i32):
                br block1(v20);

            block3:
                br block4(v0, v4, v1);

            block4(v8: i32, v12: i32, v16: i32):
                v9 = cast v8 : u32;
                v10 = inttoptr v9 : *mut i32;
                v11 = load v10 : i32;
                v13 = add.wrapping v11, v12 : i32;
                v14 = const.i32 4 : i32;
                v15 = add.wrapping v8, v14 : i32;
                v17 = const.i32 -1 : i32;
                v18 = add.wrapping v16, v17 : i32;
                v19 = neq v18, 0 : i1;
                condbr v19, block4(v15, v13, v18), block6;

            block5:
                br block2(v13);

            block6:
                br block5;
            }

            pub fn __main() -> i32 {
            block0:
                v1 = const.i32 1048576 : i32;
                v2 = const.i32 5 : i32;
                v3 = call noname::sum_arr(v1, v2) : i32;
                v4 = const.i32 1048596 : i32;
                v5 = const.i32 5 : i32;
                v6 = call noname::sum_arr(v4, v5) : i32;
                v7 = add.wrapping v3, v6 : i32;
                br block1(v7);

            block1(v0: i32):
                ret v0;
            }
        "#]],
    )
}

#[test]
fn rust_static_mut() {
    check_ir(
        include_str!("rust_source/static_mut.rs"),
        expect![[r#"
            (module
              (type (;0;) (func))
              (type (;1;) (func (result i32)))
              (func $global_var_update (;0;) (type 0)
                i32.const 0
                i32.const 0
                i32.load8_u offset=1048577
                i32.const 1
                i32.add
                i32.store8 offset=1048576
              )
              (func $__main (;1;) (type 1) (result i32)
                (local i32 i32 i32)
                call $global_var_update
                i32.const 0
                local.set 0
                i32.const -9
                local.set 1
                loop ;; label = @1
                  local.get 1
                  i32.const 1048585
                  i32.add
                  i32.load8_u
                  local.get 0
                  i32.add
                  local.set 0
                  local.get 1
                  i32.const 1
                  i32.add
                  local.tee 2
                  local.set 1
                  local.get 2
                  br_if 0 (;@1;)
                end
                local.get 0
                i32.const 255
                i32.and
              )
              (memory (;0;) 17)
              (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
              (global (;1;) i32 i32.const 1048585)
              (global (;2;) i32 i32.const 1048592)
              (export "memory" (memory 0))
              (export "global_var_update" (func $global_var_update))
              (export "__main" (func $__main))
              (export "__data_end" (global 1))
              (export "__heap_base" (global 2))
              (data $.data (;0;) (i32.const 1048576) "\01\02\03\04\05\06\07\08\09")
            )"#]],
        expect![[r#"
            module noname

            memory {
                segment @0x100000 x 9 = 0x090807060504030201;
            }

            const $0 = 0x00100000;
            const $1 = 0x00100009;
            const $2 = 0x00100010;

            global external @__stack_pointer : i32 = $0 { id = 0 };
            global external @gv1 : i32 = $1 { id = 1 };
            global external @gv2 : i32 = $2 { id = 2 };

            pub fn global_var_update() {
            block0:
                v0 = const.i32 0 : i32;
                v1 = const.i32 0 : i32;
                v2 = cast v1 : u32;
                v3 = add.checked v2, 1048577 : u32;
                v4 = inttoptr v3 : *mut u8;
                v5 = load v4 : u8;
                v6 = zext v5 : i32;
                v7 = const.i32 1 : i32;
                v8 = add.wrapping v6, v7 : i32;
                v9 = trunc v8 : u8;
                v10 = cast v0 : u32;
                v11 = add.checked v10, 1048576 : u32;
                v12 = inttoptr v11 : *mut u8;
                store v12, v9;
                br block1;

            block1:
                ret;
            }

            pub fn __main() -> i32 {
            block0:
                v1 = const.i32 0 : i32;
                call noname::global_var_update();
                v2 = const.i32 0 : i32;
                v3 = const.i32 -9 : i32;
                br block2(v3, v2);

            block1(v0: i32):
                ret v0;

            block2(v4: i32, v11: i32):
                v5 = const.i32 1048585 : i32;
                v6 = add.wrapping v4, v5 : i32;
                v7 = cast v6 : u32;
                v8 = inttoptr v7 : *mut u8;
                v9 = load v8 : u8;
                v10 = zext v9 : i32;
                v12 = add.wrapping v10, v11 : i32;
                v13 = const.i32 1 : i32;
                v14 = add.wrapping v4, v13 : i32;
                v15 = neq v14, 0 : i1;
                condbr v15, block2(v14, v12), block4;

            block3:
                v16 = const.i32 255 : i32;
                v17 = band v12, v16 : i32;
                br block1(v17);

            block4:
                br block3;
            }
        "#]],
    );
}

#[test]
fn dlmalloc() {
    check_ir_files_cargo(
        "dlmalloc_app",
        expect_file!["./expected/dlmalloc.wat"],
        expect_file!["./expected/dlmalloc.hir"],
    )
}

#[test]
#[ignore = "Being reworked"]
fn signed_arith() {
    check_ir_files(
        include_str!("rust_source/signed_arith.rs"),
        expect_file!["./expected/signed_arith.wat"],
        expect_file!["./expected/signed_arith.hir"],
    );
}
