use miden_diagnostics::SourceSpan;
use miden_hir::CallConv;
use miden_hir::Ident;
use miden_hir::Linkage;
use miden_hir::ModuleBuilder;
use miden_hir::Signature;
use miden_hir::Symbol;

use wasmparser::MemArg;
use wasmparser::Operator;
use wasmparser::Operator::*;

use crate::environ::FuncEnvironment;
use crate::environ::ModuleInfo;
use crate::func_translation_state::FuncTranslationState;
use crate::function_builder_ext::FunctionBuilderContext;
use crate::function_builder_ext::FunctionBuilderExt;
use crate::test_utils::test_diagnostics;

use super::translate_operator;

fn check_unsupported(op: &Operator) {
    let diagnostics = test_diagnostics();
    let module_info = ModuleInfo::new(Ident::with_empty_span(Symbol::intern("noname")));
    let mut module_builder = ModuleBuilder::new(module_info.name.as_str());
    let sig = Signature {
        params: vec![],
        results: vec![],
        cc: CallConv::SystemV,
        linkage: Linkage::External,
    };
    let mut module_func_builder = module_builder
        .build_function("func_name", sig.clone(), SourceSpan::default())
        .unwrap();
    let mut fb_ctx = FunctionBuilderContext::new();
    let mut builder_ext = FunctionBuilderExt::new(module_func_builder.func_builder(), &mut fb_ctx);
    let mut state = FuncTranslationState::new();
    let mut func_environ = FuncEnvironment::new(&module_info);
    let result = translate_operator(
        op,
        &mut builder_ext,
        &mut state,
        &mut func_environ,
        &diagnostics,
        SourceSpan::default(),
    );
    assert!(
        result.is_err(),
        "Expected unsupported op error for {:?}",
        op
    );
    assert_eq!(
        result.unwrap_err().to_string(),
        format!("Unsupported Wasm: Wasm op {:?} is not supported", op)
    );
    assert!(
        diagnostics.has_errors(),
        "Expected diagnostics to have errors"
    );
}

// Wasm Spec v1.0
const UNSUPPORTED_WASM_V1_OPS: &[Operator] = &[
    CallIndirect {
        type_index: 0,
        table_index: 0,
        table_byte: 0,
    },
    /**************************** Branch instructions *********************************/
    // BrTable { targets: .. },
    /****************************** Memory Operators ************************************/
    F32Load {
        memarg: MemArg {
            align: 0,
            max_align: 0,
            offset: 0,
            memory: 0,
        },
    },
    F32Store {
        memarg: MemArg {
            align: 0,
            max_align: 0,
            offset: 0,
            memory: 0,
        },
    },
    /****************************** Nullary Operators ************************************/

    // Cannot construct since Ieee32 fields are private
    // F32Const {
    //     value: Ieee32(0),
    // },

    /****************************** Unary Operators ************************************/
    I32Ctz,
    I32Clz,
    I64Ctz,
    I64Clz,
    I32WrapI64,
    F32Sqrt,
    F64Sqrt,
    F32Ceil,
    F64Ceil,
    F32Floor,
    F64Floor,
    F32Trunc,
    F64Trunc,
    F32Nearest,
    F64Nearest,
    F32Abs,
    F64Abs,
    F32Neg,
    F64Neg,
    F64ConvertI64U,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI32S,
    F32ConvertI64S,
    F32ConvertI32S,
    F32ConvertI64U,
    F32ConvertI32U,
    F64PromoteF32,
    F32DemoteF64,
    I64TruncF64S,
    I64TruncF32S,
    I32TruncF64S,
    I32TruncF32S,
    I64TruncF64U,
    I64TruncF32U,
    I32TruncF64U,
    I32TruncF32U,
    I64TruncSatF64S,
    I64TruncSatF32S,
    I32TruncSatF64S,
    I32TruncSatF32S,
    I64TruncSatF64U,
    I64TruncSatF32U,
    I32TruncSatF64U,
    I32TruncSatF32U,
    F32ReinterpretI32,
    F64ReinterpretI64,
    I32ReinterpretF32,
    I64ReinterpretF64,
    /****************************** Binary Operators ************************************/
    I32ShrS,
    I64ShrS,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    I32DivS,
    I64DivS,
    I32RemS,
    I64RemS,
    F32Min,
    F32Max,
    F32Copysign,
    F64Copysign,
    F64Add,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    /**************************** Comparison Operators **********************************/
    I32LtS,
    I64LtS,
    I32LeS,
    I64LeS,
    I32GtS,
    I64GtS,
    I32GeS,
    I64GeS,
    F32Eq,
    F32Ne,
    F32Gt,
    F32Ge,
    F32Le,
    F32Lt,
    F64Eq,
    F64Ne,
    F64Gt,
    F64Ge,
    F64Le,
    F64Lt,
];

#[test]
fn error_for_unsupported_wasm_v1_ops() {
    for op in UNSUPPORTED_WASM_V1_OPS.iter() {
        check_unsupported(op);
    }
}
