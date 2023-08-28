//! Helper functions and structures for the translation.

use crate::{error::WasmResult, function_builder_ext::FunctionBuilderExt};
use miden_diagnostics::SourceSpan;
use miden_ir::{
    hir::{Block, FunctionBuilder, InstBuilder, Value},
    types::Type,
};

/// Create a `Block` with the given parameters.
pub fn block_with_params(
    builder: &mut FunctionBuilderExt,
    params: impl IntoIterator<Item = Type>,
    span: SourceSpan,
) -> WasmResult<Block> {
    // TODO: make this a method `create_block_with_params` in FunctionBuilderExt?
    let block = builder.create_block();
    for ty in params {
        builder.inner.append_block_param(block, ty, span);
    }
    Ok(block)
}

/// Turns a `wasmparser` `f64` into a `Miden` one.
pub fn f64_translation(_x: wasmparser::Ieee64) -> f64 {
    todo!("f64_translation")
}

/// Emit instructions to produce a zero value in the given type.
pub fn emit_zero(ty: &Type, builder: &mut FunctionBuilder) -> Value {
    match ty {
        Type::I1 => builder.ins().i1(false, SourceSpan::default()),
        Type::I8 => builder.ins().i8(0, SourceSpan::default()),
        Type::I16 => builder.ins().i16(0, SourceSpan::default()),
        Type::I32 => builder.ins().i32(0, SourceSpan::default()),
        Type::I64 => builder.ins().i64(0, SourceSpan::default()),
        Type::I128 => builder.ins().i128(0, SourceSpan::default()),
        Type::I256 => todo!(),
        Type::Isize => todo!(),
        Type::F64 => builder.ins().f64(0.0, SourceSpan::default()),
        Type::Felt => todo!(),
        Type::Ptr(_) => panic!("cannot emit zero for pointer type"),
        Type::Struct(_) => panic!("cannot emit zero for struct type"),
        Type::Array(_, _) => panic!("cannot emit zero for array type"),
        Type::Unknown => panic!("cannot emit zero for unknown type"),
        Type::Unit => panic!("cannot emit zero for unit type"),
        Type::Never => panic!("cannot emit zero for never type"),
    }
}
