use expect_test::expect_file;
use miden_core::crypto::hash::RpoDigest;
use miden_frontend_wasm::{ImportMetadata, WasmTranslationConfig};
use miden_hir::{FunctionType, Ident, InterfaceFunctionIdent, InterfaceIdent, Symbol, Type};

use crate::{cargo_proj::project, CompilerTest};

#[test]
fn wcm_no_imports() {
    let config = Default::default();

    let proj = project("wcm_no_imports")
        .file(
            "Cargo.toml",
            r#"
            [package]
            name = "add-wasm-component"
            version = "0.0.1"
            edition = "2015"
            authors = []

            [dependencies]
            wit-bindgen = { version = "0.17.0", default-features = false, features = ["realloc"] }
            wee_alloc = { version = "0.4.5", default-features = false}

            [lib]
            crate-type = ["cdylib"]

            [package.metadata.component]
            package = "miden:add"

            [profile.release]
            panic = "abort"
        "#,
        )
        .file(
            "src/lib.rs",
            r#"
            #![no_std]

            #[global_allocator]
            static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

            #[panic_handler]
            fn my_panic(_info: &core::panic::PanicInfo) -> ! {
                loop {}
            }

            extern crate wit_bindgen;

            mod bindings;

            use crate::bindings::exports::miden::add_package::add_interface::Guest;

            struct Component;

            impl Guest for Component {
                fn add(a: u32, b: u32) -> u32 {
                    a + b
                }
            }
        "#,
        )
        .file(
            "wit/add.wit",
            r#"
            package miden:add-package@1.0.0;

            interface add-interface {
                add: func(a: u32, b: u32) -> u32;
            }

            world add-world {
                export add-interface;
            }
        "#,
        )
        .build();
    let mut test = CompilerTest::rust_source_cargo_component(proj.root(), config);
    let artifact_name = test.source.artifact_name();
    test.expect_wasm(expect_file![format!("../../expected/components/{artifact_name}.wat")]);
    test.expect_ir(expect_file![format!("../../expected/components/{artifact_name}.hir")]);
}

#[test]
fn wcm_import() {
    // Imports an add component used in the above test

    let interface_function_ident = InterfaceFunctionIdent {
        interface: InterfaceIdent::from_full_ident(
            "miden:add-package/add-interface@1.0.0".to_string(),
        ),
        function: Symbol::intern("add"),
    };
    let import_metadata = [(
        interface_function_ident,
        ImportMetadata {
            digest: RpoDigest::default(),
        },
    )]
    .into_iter()
    .collect();

    let config = WasmTranslationConfig {
        import_metadata,
        ..Default::default()
    };

    // Create the add component that will be imported in the wcm_import project
    let _add_proj_dep = project("wcm_import_add")
        .file(
            "wit/add.wit",
            r#"
            package miden:add-package@1.0.0;

            interface add-interface {
                add: func(a: u32, b: u32) -> u32;
            }

            world add-world {
                export add-interface;
            }
        "#,
        )
        .no_manifest()
        .build();

    let proj = project("wcm_import")
        .file(
            "Cargo.toml",
            r#"
            [package]
            name = "inc-wasm-component"
            version = "0.0.1"
            edition = "2015"
            authors = []

            [dependencies]
            wit-bindgen = { version = "0.17.0", default-features = false, features = ["realloc"] }
            wee_alloc = { version = "0.4.5", default-features = false}

            [lib]
            crate-type = ["cdylib"]

            [package.metadata.component]
            package = "miden:inc"

            [package.metadata.component.target.dependencies]
            "miden:add" = { path = "../wcm_import_add/wit" }

            [profile.release]
            panic = "abort"
        "#,
        )
        .file(
            "src/lib.rs",
            r#"
            #![no_std]

            #[global_allocator]
            static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

            #[panic_handler]
            fn my_panic(_info: &core::panic::PanicInfo) -> ! {
                loop {}
            }

            extern crate wit_bindgen;

            mod bindings;

            use crate::bindings::{miden::add_package::add_interface::add, Guest};

            struct Component;

            impl Guest for Component {
                fn inc(a: u32) -> u32 {
                    add(a, 1)
                }
            }
        "#,
        )
        .file(
            "wit/inc.wit",
            r#"
            package miden:inc-package@1.0.0;

            use miden:add-package/add-interface@1.0.0;

            world inc {
                import add-interface;
                export inc: func(a: u32) -> u32;
            }
        "#,
        )
        .build();

    let mut test = CompilerTest::rust_source_cargo_component(proj.root(), config);
    let artifact_name = test.source.artifact_name();
    test.expect_wasm(expect_file![format!("../../expected/components/{artifact_name}.wat")]);
    test.expect_ir(expect_file![format!("../../expected/components/{artifact_name}.hir")]);

    let ir_component = test.hir().unwrap_component();

    assert!(!ir_component.modules().is_empty());

    let export_name_sym = Symbol::intern("inc");
    let export = ir_component.exports().get(&export_name_sym.into()).unwrap();
    assert_eq!(export.function.function.as_symbol(), export_name_sym);

    let expected_export_func_ty = FunctionType::new_wasm(vec![Type::U32], vec![Type::U32]);
    assert_eq!(export.function_ty, expected_export_func_ty);
    let module = ir_component.modules().first().unwrap().1;
    dbg!(&module.imports());
    let import_info = module.imports();
    let function_id = *import_info
        .imported(&Ident::from("miden:add-package/add-interface@1.0.0"))
        .unwrap()
        .iter()
        .collect::<Vec<_>>()
        .first()
        .cloned()
        .unwrap();
    let component_import =
        ir_component.imports().get(&function_id).unwrap().unwrap_canon_abi_import();
    assert_eq!(component_import.interface_function, interface_function_ident);
    assert!(!component_import.function_ty.params.is_empty());
    let expected_import_func_ty =
        FunctionType::new_wasm(vec![Type::U32, Type::U32], vec![Type::U32]);
    assert_eq!(component_import.function_ty, expected_import_func_ty);
}
