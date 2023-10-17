//! Helper functions and structures for the translation.

use miden_diagnostics::SourceSpan;
use miden_hir::{AbiParam, CallConv, InstBuilder, Linkage, Signature, Value};
use miden_hir_type::{FunctionType, Type};

use crate::function_builder_ext::FunctionBuilderExt;

/// Emit instructions to produce a zero value in the given type.
pub fn emit_zero(ty: &Type, builder: &mut FunctionBuilderExt) -> Value {
    match ty {
        Type::I1 => builder.ins().i1(false, SourceSpan::default()),
        Type::I8 => builder.ins().i8(0, SourceSpan::default()),
        Type::I16 => builder.ins().i16(0, SourceSpan::default()),
        Type::I32 => builder.ins().i32(0, SourceSpan::default()),
        Type::I64 => builder.ins().i64(0, SourceSpan::default()),
        Type::I128 => todo!(),
        Type::U256 => todo!(),
        Type::U8 => todo!(),
        Type::U16 => todo!(),
        Type::U32 => todo!(),
        Type::U64 => todo!(),
        Type::U128 => todo!(),
        Type::F64 => builder.ins().f64(0.0, SourceSpan::default()),
        Type::Felt => todo!(),
        Type::Ptr(_) => panic!("cannot emit zero for pointer type"),
        Type::Struct(_) => panic!("cannot emit zero for struct type"),
        Type::Array(_, _) => panic!("cannot emit zero for array type"),
        Type::Unknown => panic!("cannot emit zero for unknown type"),
        Type::Unit => panic!("cannot emit zero for unit type"),
        Type::Never => panic!("cannot emit zero for never type"),
        Type::NativePtr(_, _) => todo!(),
    }
}

pub fn sig_from_funct_type(
    func_type: &FunctionType,
    call_conv: CallConv,
    linkage: Linkage,
) -> Signature {
    Signature {
        params: func_type
            .params
            .iter()
            .map(|ty| AbiParam::new(ty.clone()))
            .collect(),
        results: func_type
            .results
            .iter()
            .map(|ty| AbiParam::new(ty.clone()))
            .collect(),
        cc: call_conv,
        linkage,
    }
}
