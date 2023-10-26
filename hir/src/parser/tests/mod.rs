use miden_diagnostics::SourceSpan;

use crate::parser::ast::*;
use crate::{
    AbiParam, ArgumentExtension, ArgumentPurpose, CallConv, ExternalFunction, FunctionIdent, Ident,
    Linkage, Opcode, Overflow, Signature, StructType, Type,
};

macro_rules! ident {
    ($name:ident) => {
        Ident::new(
            crate::Symbol::intern(stringify!($name)),
            miden_diagnostics::SourceSpan::UNKNOWN,
        )
    };
}

mod utils;
use self::utils::ParseTest;

/// This test tries to exercise a broad swath of the IR syntax
#[test]
fn parser_integration_test() {
    let dummy_sourcespan = SourceSpan::UNKNOWN;

    // global internal @DEADBEEF : u32 = 0xdeadbeef { id = 0 };
    let deadbeef = GlobalVarDeclaration::new(
        dummy_sourcespan,
        crate::GlobalVariable::from_u32(0),
        ident!(DEADBEEF),
        Type::U32,
        Linkage::Internal,
        Some("deadbeef".parse().unwrap()),
    );

    // pub cc(fast) fn foo(u32, sext u32) -> u32 {
    let mut foo = FunctionDeclaration {
        span: dummy_sourcespan,
        name: ident!(foo),
        signature: Signature {
            params: vec![
                AbiParam::new(Type::U32),
                AbiParam {
                    ty: Type::U32,
                    purpose: Default::default(),
                    extension: ArgumentExtension::Sext,
                },
            ],
            results: vec![AbiParam::new(Type::U32)],
            cc: CallConv::Fast,
            linkage: Linkage::External,
        },
        blocks: vec![],
    };

    //     blk0(v1: u32, v2: u32):
    let v1 = crate::Value::from_u32(1);
    let v2 = crate::Value::from_u32(2);
    let blk0_id = crate::Block::from_u32(0);
    let mut blk0 = Block {
        span: dummy_sourcespan,
        id: blk0_id,
        params: vec![
            TypedValue::new(v1, Type::U32),
            TypedValue::new(v2, Type::U32),
        ],
        body: vec![],
    };

    // v3 = add.unchecked v1, v2 : u32
    let v3 = crate::Value::from_u32(3);
    let inst1 = Inst {
        span: dummy_sourcespan,
        ty: InstType::BinaryOp {
            opcode: Opcode::Add,
            overflow: Some(Overflow::Unchecked),
            operands: [Operand::Value(v1), Operand::Value(v2)],
        },
        outputs: vec![TypedValue::new(v3, Type::U32)],
    };
    blk0.body.push(inst1);

    // br blk1
    let blk1_id = crate::Block::from_u32(1);
    let inst2 = Inst {
        span: dummy_sourcespan,
        ty: InstType::Br {
            opcode: Opcode::Br,
            successor: Successor {
                span: dummy_sourcespan,
                id: blk1_id,
                args: vec![],
            },
        },
        outputs: vec![],
    };
    blk0.body.push(inst2);

    // blk1:
    let mut blk1 = Block {
        span: dummy_sourcespan,
        id: blk1_id,
        params: vec![],
        body: vec![],
    };
    // ret v3
    let inst3 = Inst {
        span: dummy_sourcespan,
        ty: InstType::Ret {
            opcode: Opcode::Ret,
            operands: vec![Operand::Value(v3)],
        },
        outputs: vec![],
    };
    blk1.body.push(inst3);

    foo.blocks.push(blk0);
    foo.blocks.push(blk1);

    // cc(kernel) fn tuple::make_pair (sret *mut { u32, u32 });
    let tuple = StructType::new([Type::U32, Type::U32]);
    let make_pair = ExternalFunction {
        id: FunctionIdent {
            module: ident!(tuple),
            function: ident!(make_pair),
        },
        signature: Signature {
            params: vec![AbiParam {
                ty: Type::Ptr(Box::new(Type::Struct(tuple))),
                purpose: ArgumentPurpose::StructReturn,
                extension: Default::default(),
            }],
            results: vec![],
            cc: CallConv::Kernel,
            linkage: Linkage::External,
        },
    };

    let expected = Module {
        span: dummy_sourcespan,
        name: ident!(test),
        global_vars: vec![deadbeef],
        functions: vec![foo],
        externals: vec![make_pair],
        is_kernel: false,
    };

    ParseTest::new().expect_module_ast_from_file("src/parser/tests/input/test.hir", expected);
}
