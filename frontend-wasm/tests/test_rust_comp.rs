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

fn check_ir(
    rust_source: &str,
    expected_wat: expect_test::Expect,
    expected_ir: expect_test::Expect,
) {
    let wasm_bytes = compile_wasm(rust_source);
    let mut wasm_printer = wasmprinter::Printer::new();
    // disable printing of the "producers" section because it contains a rustc version
    // to not brake tests when rustc is updated
    wasm_printer.add_custom_section_printer("producers", |_, _, _| Ok(()));
    let wat = wasm_printer.print(wasm_bytes.as_ref()).unwrap();
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

            pub fn match_enum(i32, i32, i32) -> i32 {
            block0(v0: i32, v1: i32, v2: i32):
                v4 = const.i32 255  : i32
                v5 = band v2, v4  : i32
                switch v5, 0 => block4, 1 => block3, 2 => block2, block4

            block1(v3: i32):
                v11 = ret v3  : ()

            block2:
                v10 = mul v1, v0  : i32
                br block1(v10)

            block3:
                v8 = sub v0, v1  : i32
                v9 = ret v8  : ()

            block4:
                v6 = add v1, v0  : i32
                v7 = ret v6  : ()
            }

            pub fn __main() -> i32 {
            block0:
                v1 = const.i32 3  : i32
                v2 = const.i32 5  : i32
                v3 = const.i32 0  : i32
                v4 = call noname::match_enum(v1, v2, v3)  : i32
                v5 = const.i32 3  : i32
                v6 = const.i32 5  : i32
                v7 = const.i32 1  : i32
                v8 = call noname::match_enum(v5, v6, v7)  : i32
                v9 = add v4, v8  : i32
                v10 = const.i32 3  : i32
                v11 = const.i32 5  : i32
                v12 = const.i32 2  : i32
                v13 = call noname::match_enum(v10, v11, v12)  : i32
                v14 = add v9, v13  : i32
                br block1(v14)

            block1(v0: i32):
                v15 = ret v0  : ()
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

            pub fn sum_arr(i32, i32) -> i32 {
            block0(v0: i32, v1: i32):
                v3 = const.i32 0  : i32
                v4 = const.i32 0  : i32
                v5 = const.i32 0  : i32
                v6 = eq v1, v5  : i1
                v7 = const.i32 0  : i32
                v8 = neq v6, v7  : i1
                condbr v8, block2(v4), block3

            block1(v2: i32):
                v22 = ret v2  : ()

            block2(v21: i32):
                br block1(v21)

            block3:
                br block4(v0, v4, v1)

            block4(v9: i32, v12: i32, v16: i32):
                v10 = inttoptr v9  : *mut i32
                v11 = load v10  : i32
                v13 = add v11, v12  : i32
                v14 = const.i32 4  : i32
                v15 = add v9, v14  : i32
                v17 = const.i32 -1  : i32
                v18 = add v16, v17  : i32
                v19 = const.i32 0  : i32
                v20 = neq v18, v19  : i1
                condbr v20, block4(v15, v13, v18), block6

            block5:
                br block2(v13)

            block6:
                br block5
            }

            pub fn __main() -> i32 {
            block0:
                v1 = const.i32 1048576  : i32
                v2 = const.i32 5  : i32
                v3 = call noname::sum_arr(v1, v2)  : i32
                v4 = const.i32 1048596  : i32
                v5 = const.i32 5  : i32
                v6 = call noname::sum_arr(v4, v5)  : i32
                v7 = add v3, v6  : i32
                br block1(v7)

            block1(v0: i32):
                v8 = ret v0  : ()
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

            pub fn global_var_update() {
            block0:
                v0 = const.i32 0  : i32
                v1 = const.i32 0  : i32
                v2 = const.i32 1048577  : i32
                v3 = add v1, v2  : i32
                v4 = inttoptr v3  : *mut i8
                v5 = load v4  : i8
                v6 = zext v5  : i32
                v7 = const.i32 1  : i32
                v8 = add v6, v7  : i32
                v9 = trunc v8  : i8
                v10 = const.i32 1048576  : i32
                v11 = add v0, v10  : i32
                v12 = inttoptr v11  : *mut i8
                store v12, v9
                br block1

            block1:
                v13 = ret   : ()
            }

            pub fn __main() -> i32 {
            block0:
                v1 = const.i32 0  : i32
                call noname::global_var_update()
                v2 = const.i32 0  : i32
                v3 = const.i32 -9  : i32
                br block2(v3, v2)

            block1(v0: i32):
                v18 = ret v0  : ()

            block2(v4: i32, v10: i32):
                v5 = const.i32 1048585  : i32
                v6 = add v4, v5  : i32
                v7 = inttoptr v6  : *mut i8
                v8 = load v7  : i8
                v9 = zext v8  : i32
                v11 = add v9, v10  : i32
                v12 = const.i32 1  : i32
                v13 = add v4, v12  : i32
                v14 = const.i32 0  : i32
                v15 = neq v13, v14  : i1
                condbr v15, block2(v13, v11), block4

            block3:
                v16 = const.i32 255  : i32
                v17 = band v11, v16  : i32
                br block1(v17)

            block4:
                br block3
            }
        "#]],
    );
}
