use miden_diagnostics::SourceSpan;
use miden_ir::hir::Function;
use miden_ir::hir::Module;
use miden_ir::hir::Signature;
use miden_ir::hir::Visibility;
use miden_ir::types::FunctionType;
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
    let mut module = Module::new("module_name".to_string(), None);
    let sig = Signature {
        visibility: Visibility::PUBLIC,
        name: "func_name".to_string(),
        ty: FunctionType::new(vec![], vec![]),
    };
    let fref = module.declare_function(sig.clone());
    let mut func = Function::new(
        fref,
        SourceSpan::default(),
        sig.clone(),
        module.signatures.clone(),
        module.names.clone(),
    );
    let mut fb_ctx = FunctionBuilderContext::new();
    let mut builder = FunctionBuilderExt::new(&mut func, &mut fb_ctx);
    let mut state = FuncTranslationState::new();
    let module_info = ModuleInfo::new();
    let mut func_environ = FuncEnvironment::new(&module_info);
    let result = translate_operator(
        op,
        &mut builder,
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
    /****************************** Memory Operators ************************************/
    CallIndirect {
        type_index: 0,
        table_index: 0,
        table_byte: 0,
    },
    // let (mut arg1, mut arg2, cond) = state.pop3();
    // if cond is zero returns arg2, else returns arg1
    Select,
    // Halt the program as it reached the point that should never be executed
    Unreachable,
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
];

#[test]
fn error_for_unsupported_wasm_v1_ops() {
    for op in UNSUPPORTED_WASM_V1_OPS.iter() {
        check_unsupported(op);
    }
}
