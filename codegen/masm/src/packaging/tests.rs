use std::sync::Arc;

use miden_stdlib::StdLibrary;
use midenc_hir::{diagnostics::IntoDiagnostic, testing::TestContext, StructType, Type};
use midenc_session::{diagnostics::Report, Emit};

use super::*;
use crate::{MasmArtifact, NativePtr};

#[test]
fn packaging_serialization() -> Result<(), Report> {
    let context = TestContext::default_with_emitter(None);
    let package = example_package(&context)?;

    bitcode::serialize(package.as_ref()).map_err(Report::msg)?;

    Ok(())
}

#[test]
fn packaging_deserialization() -> Result<(), Report> {
    let context = TestContext::default_with_emitter(None);
    let expected = example_package(&context)?;

    let mut bytes = vec![];
    expected
        .write_to(&mut bytes, midenc_session::OutputMode::Binary, &context.session)
        .into_diagnostic()?;

    let package = Package::read_from_bytes(bytes)?;

    assert_eq!(package.name, expected.name);
    assert_eq!(package.digest, expected.digest);
    assert_eq!(package.rodata, expected.rodata);
    assert_eq!(package.manifest, expected.manifest);
    assert!(package.is_program());

    // Verify rodata serialization
    assert!(!package.rodata.is_empty());
    let expected_rodata_offset = NativePtr::from_ptr(65536 * 4);
    let foo_data = package
        .rodata
        .iter()
        .find(|rodata| rodata.start == expected_rodata_offset)
        .unwrap();
    let foo_bytes = foo_data.data.as_slice();

    let foo_ty = StructType::new([Type::U8, Type::U32, Type::U64]);
    let offset_u8 = foo_ty.get(0).offset as usize;
    let offset_u32 = foo_ty.get(1).offset as usize;
    let offset_u64 = foo_ty.get(2).offset as usize;
    assert_eq!(foo_bytes[offset_u8], 1);
    assert_eq!(
        u32::from_be_bytes([
            foo_bytes[offset_u32],
            foo_bytes[offset_u32 + 1],
            foo_bytes[offset_u32 + 2],
            foo_bytes[offset_u32 + 3]
        ]),
        2
    );
    assert_eq!(
        u32::from_be_bytes([
            foo_bytes[offset_u64],
            foo_bytes[offset_u64 + 1],
            foo_bytes[offset_u64 + 2],
            foo_bytes[offset_u64 + 3]
        ]),
        0
    );
    assert_eq!(
        u32::from_be_bytes([
            foo_bytes[offset_u64 + 4],
            foo_bytes[offset_u64 + 5],
            foo_bytes[offset_u64 + 6],
            foo_bytes[offset_u64 + 7]
        ]),
        3
    );

    // Verify the MAST
    let expected = expected.unwrap_program();
    let program = package.unwrap_program();
    assert_eq!(program.hash(), expected.hash());
    assert_eq!(program.mast_forest(), expected.mast_forest());

    Ok(())
}

fn example_package(context: &TestContext) -> Result<Arc<Package>, Report> {
    use midenc_hir::ProgramBuilder;

    // Build a simple program
    let mut builder = ProgramBuilder::new(&context.session.diagnostics);

    // Build test module with fib function
    let mut mb = builder.module("test");
    midenc_hir::testing::fib1(mb.as_mut(), context);

    // Ensure we have an example data segment or two to work with
    let foo_ty = StructType::new([Type::U8, Type::U32, Type::U64]);
    // Initialize the struct with some data
    let offset_u8 = foo_ty.get(0).offset as usize;
    let offset_u32 = foo_ty.get(1).offset as usize;
    let offset_u64 = foo_ty.get(2).offset as usize;
    let foo_ty = Type::Struct(foo_ty);
    let foo_size = foo_ty.size_in_bytes();
    let mut data = Vec::<u8>::with_capacity(foo_size);
    data.resize(foo_size, 0);
    unsafe {
        let data_ptr_range = data.as_mut_ptr_range();
        core::ptr::write(data_ptr_range.start.byte_add(offset_u8), 1u8);
        core::ptr::write(data_ptr_range.start.byte_add(offset_u32).cast(), 2u32.to_be_bytes());
        core::ptr::write(data_ptr_range.start.byte_add(offset_u64).cast(), 0u32.to_be_bytes()); // hi bits
        core::ptr::write(data_ptr_range.start.byte_add(offset_u64 + 4).cast(), 3u32.to_be_bytes()); // lo bits
    }
    mb.declare_data_segment(65536 * 4, foo_size as u32, data, true)?;

    mb.build().expect("unexpected error constructing test module");

    // Link the program
    let mut program = builder
        .with_entrypoint("test::fib".parse().unwrap())
        .link()
        .expect("failed to link program");

    program.add_library(StdLibrary::default().into());

    // Compile the program
    let mut compiler = crate::MasmCompiler::new(&context.session);
    let program = compiler.compile(program).expect("compilation failed").unwrap_executable();

    // Assemble the program
    let masm_artifact = MasmArtifact::Executable(program);
    let mast_artifact = masm_artifact.assemble(&context.session)?;

    // Package the program
    Ok(Arc::new(Package::new(mast_artifact, &masm_artifact, &context.session)))
}
