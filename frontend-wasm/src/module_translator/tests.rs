use crate::module_translator::translate_module;
use crate::test_utils::test_diagnostics;
use crate::WasmTranslationConfig;

use expect_test::expect;

fn check_ir(wat: &str, expected_ir: expect_test::Expect) {
    let wasm = wat::parse_str(wat).unwrap();
    let diagnostics = test_diagnostics();
    let module = translate_module(&wasm, &WasmTranslationConfig::default(), &diagnostics).unwrap();
    expected_ir.assert_eq(&module.to_string());
}

#[test]
fn module() {
    check_ir(
        r#"
        (module
            (func $main
                i32.const 0
                drop
            )
        )
    "#,
        expect![[r#"
            module noname

            pub fn main() {
            block0:
                v0 = const.i32 0  : i32
                br block1

            block1:
                v1 = ret   : ()
            }
        "#]],
    );
}

#[test]
fn locals() {
    check_ir(
        r#"
        (module
            (func $main (local i32)
                i32.const 1
                local.set 0
                local.get 0
                drop
            )
        )
    "#,
        expect![[r#"
            module noname

            pub fn main() {
            block0:
                v0 = const.i32 0  : i32
                v1 = const.i32 1  : i32
                br block1

            block1:
                v2 = ret   : ()
            }
        "#]],
    );
}

#[test]
fn locals_inter_block() {
    check_ir(
        r#"
        (module
            (func $main (result i32) (local i32)
                block
                    i32.const 3
                    local.set 0
                end
                block
                    local.get 0
                    i32.const 5
                    i32.add
                    local.set 0
                end
                i32.const 7
                local.get 0
                i32.add
            )
        )
    "#,
        expect![[r#"
            module noname

            pub fn main() -> i32 {
            block0:
                v1 = const.i32 0  : i32
                v2 = const.i32 3  : i32
                br block2

            block1(v0: i32):
                v7 = ret v0  : ()

            block2:
                v3 = const.i32 5  : i32
                v4 = add v2, v3  : i32
                br block3

            block3:
                v5 = const.i32 7  : i32
                v6 = add v5, v4  : i32
                br block1(v6)
            }
        "#]],
    );
}

#[test]
fn func_call() {
    check_ir(
        r#"
        (module
            (func $add (param i32 i32) (result i32)
                local.get 0
                local.get 1
                i32.add
            )
            (func $main (result i32)
                i32.const 3
                i32.const 5
                call $add
            )
        )
    "#,
        expect![[r#"
            module noname

            pub fn add(i32, i32) -> i32 {
            block0(v0: i32, v1: i32):
                v3 = add v0, v1  : i32
                br block1(v3)

            block1(v2: i32):
                v4 = ret v2  : ()
            }

            pub fn main() -> i32 {
            block0:
                v1 = const.i32 3  : i32
                v2 = const.i32 5  : i32
                v3 = call noname::add(v1, v2)  : i32
                br block1(v3)

            block1(v0: i32):
                v4 = ret v0  : ()
            }
        "#]],
    );
}

#[test]
fn br() {
    check_ir(
        r#"
        (module
            (func $main (result i32) (local i32)
                block
                    i32.const 3
                    local.set 0
                    br 0
                end
                local.get 0
            )
        )
    "#,
        expect![[r#"
            module noname

            pub fn main() -> i32 {
            block0:
                v1 = const.i32 0  : i32
                v2 = const.i32 3  : i32
                br block2

            block1(v0: i32):
                v3 = ret v0  : ()

            block2:
                br block1(v2)
            }
        "#]],
    );
}

#[test]
fn loop_br_if() {
    // sum the decreasing numbers from 2 to 0, i.e. 2 + 1 + 0, then exit the loop
    check_ir(
        r#"
        (module
            (func $main (result i32) (local i32 i32)
                i32.const 2
                local.set 0
                loop
                    local.get 0
                    local.get 1
                    i32.add
                    local.set 1
                    local.get 0
                    i32.const 1
                    i32.sub
                    local.tee 0
                    br_if 0
                end
                local.get 1
            )
        )
    "#,
        expect![[r#"
            module noname

            pub fn main() -> i32 {
            block0:
                v1 = const.i32 0  : i32
                v2 = const.i32 2  : i32
                br block2(v2, v1)

            block1(v0: i32):
                v10 = ret v0  : ()

            block2(v3: i32, v4: i32):
                v5 = add v3, v4  : i32
                v6 = const.i32 1  : i32
                v7 = sub v3, v6  : i32
                v8 = const.i32 0  : i32
                v9 = neq v7, v8  : i1
                condbr v9, block2(v7, v5), block4

            block3:
                br block1(v5)

            block4:
                br block3
            }
        "#]],
    );
}

#[test]
fn if_then_else() {
    check_ir(
        r#"
        (module
            (func $main (result i32)
                i32.const 2
                if (result i32)
                    i32.const 3
                else
                    i32.const 5
                end
            )
        )
    "#,
        expect![[r#"
            module noname

            pub fn main() -> i32 {
            block0:
                v1 = const.i32 2  : i32
                v2 = const.i32 0  : i32
                v3 = neq v1, v2  : i1
                condbr v3, block2, block4

            block1(v0: i32):
                v7 = ret v0  : ()

            block2:
                v5 = const.i32 3  : i32
                br block3(v5)

            block3(v4: i32):
                br block1(v4)

            block4:
                v6 = const.i32 5  : i32
                br block3(v6)
            }
        "#]],
    );
}

#[test]
fn global_var() {
    check_ir(
        r#"
        (module
            (global $MyGlobalVal (mut i32) i32.const 42)
            (func $main
                global.get $MyGlobalVal
                i32.const 9
                i32.add
                global.set $MyGlobalVal
            )
        )
    "#,
        expect![[r#"
            module noname

            pub fn main() {
            block0:
                v0 = global.load (@MyGlobalVal) as *mut i8  : i32
                v1 = const.i32 9  : i32
                v2 = add v0, v1  : i32
                v3 = global.symbol @MyGlobalVal  : *mut i8
                v4 = ptrtoint v3  : i32
                v5 = inttoptr v4  : *mut i32
                store v5, v2
                br block1

            block1:
                v6 = ret   : ()
            }
        "#]],
    );
}
