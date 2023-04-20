use crate::hir::*;
use cranelift_entity::SecondaryMap;
use miden_assembly::ast::{self as masm};
use winter_utils::collections::Vec;

//#[derive(Debug, thiserror::Error)]
//pub enum CodegenError {
//    #[error("Unsupported feature during code generation: {feature:?}")]
//    Unsupported {
//        feature: String
//    }
//}

//pub type CodegenResult<T> = std::result::Result<T, CodegenError>;
//
//impl ToDiagnostic for CodegenError {
//    fn to_diagnostic(self) -> Diagnostic {
//        match self {
//            CodegenError::Unsupported { feature } => Diagnostic::error().with_message(feature.to_string()),
//        }
//    }
//}

fn masm_instruction_as_node(inst: masm::Instruction) -> masm::Node {
    masm::Node::Instruction(inst)
}

fn immediate_as_u32(source: &Immediate) -> u32 {
    match source {
        Immediate::I1(true) => 1,
        Immediate::I1(false) => 0,
        Immediate::I8(v) => {
            if let Ok(data) = u32::try_from(*v) {
                data
            } else {
                panic!("Not a u32: {}", source)
            }
        }
        Immediate::I16(v) => {
            if let Ok(data) = u32::try_from(*v) {
                data
            } else {
                panic!("Not a u32: {}", source)
            }
        }
        Immediate::I32(v) => {
            if let Ok(data) = u32::try_from(*v) {
                data
            } else {
                panic!("Not a u32: {}", source)
            }
        }
        Immediate::Felt(v) => {
            if let Ok(data) = u32::try_from(*v) {
                data
            } else {
                panic!("Not a u32: {}", source)
            }
        }
        Immediate::I64(v) => {
            if let Ok(data) = u32::try_from(*v) {
                data
            } else {
                panic!("Not a u32: {}", source)
            }
        }
        Immediate::I128(v) => {
            if let Ok(data) = u32::try_from(*v) {
                data
            } else {
                panic!("Not a u32: {}", source)
            }
        }
        Immediate::Isize(v) => {
            if let Ok(data) = u32::try_from(*v) {
                data
            } else {
                panic!("Not a u32: {}", source)
            }
        }
        Immediate::F64(_) => panic!("Unable to convert immediate {} to u32", source),
    }
}

fn codegen_binary_op(op: Opcode, overflow: &Overflow) -> masm::Node {
    match op {
        Opcode::Store => panic!("Unsupported instruction: BinaryOp (opcode Store)"),
        Opcode::Add =>
            match overflow {
                Overflow::Unchecked => panic!("No such instruction: BinaryOp, Opcode Add, Overflow Unchecked"),
                Overflow::Checked => masm_instruction_as_node(masm::Instruction::U32CheckedAdd),
                Overflow::Wrapping => masm_instruction_as_node(masm::Instruction::U32WrappingAdd),
                Overflow::Overflowing => masm_instruction_as_node(masm::Instruction::U32OverflowingAdd),
            },
        Opcode::Sub =>
            match overflow {
                Overflow::Unchecked => panic!("No such instruction: BinaryOp, Opcode Sub, Overflow Unchecked"),
                Overflow::Checked => masm_instruction_as_node(masm::Instruction::U32CheckedSub),
                Overflow::Wrapping => masm_instruction_as_node(masm::Instruction::U32WrappingSub),
                Overflow::Overflowing => masm_instruction_as_node(masm::Instruction::U32OverflowingSub),
            },
        Opcode::Mul =>
            match overflow {
                Overflow::Unchecked => panic!("No such instruction: BinaryOp, Opcode Mul, Overflow Unchecked"),
                Overflow::Checked => masm_instruction_as_node(masm::Instruction::U32CheckedMul),
                Overflow::Wrapping => masm_instruction_as_node(masm::Instruction::U32WrappingMul),
                Overflow::Overflowing => masm_instruction_as_node(masm::Instruction::U32OverflowingMul),
            },
        Opcode::Div =>
            match overflow {
                Overflow::Unchecked => masm_instruction_as_node(masm::Instruction::U32UncheckedDiv),
                Overflow::Checked => masm_instruction_as_node(masm::Instruction::U32CheckedDiv),
                Overflow::Wrapping => panic!("No such instruction: BinaryOp, Opcode Div, Overflow Wrapping"),
                Overflow::Overflowing => panic!("No such instruction: BinaryOp, Opcode Div, Overflow Overflowing"),
            },
        Opcode::Mod =>
            match overflow {
                Overflow::Unchecked => masm_instruction_as_node(masm::Instruction::U32UncheckedMod),
                Overflow::Checked => masm_instruction_as_node(masm::Instruction::U32CheckedMod),
                Overflow::Wrapping => panic!("No such instruction: BinaryOp, Opcode Mod, Overflow Wrapping"),
                Overflow::Overflowing => panic!("No such instruction: BinaryOp, Opcode Mod, Overflow Overflowing"),
            },
        Opcode::DivMod =>
            match overflow {
                Overflow::Unchecked => masm_instruction_as_node(masm::Instruction::U32UncheckedDivMod),
                Overflow::Checked => masm_instruction_as_node(masm::Instruction::U32CheckedDivMod),
                Overflow::Wrapping => panic!("No such instruction: BinaryOp, Opcode DivMod, Overflow Wrapping"),
                Overflow::Overflowing => panic!("No such instruction: BinaryOp, Opcode DivMod, Overflow Overflowing"),
            },
        Opcode::Exp // Not handling this, since there is no 32-bit Exp operation in MASM
        | Opcode::And
        | Opcode::Or
        | Opcode::Xor
        | Opcode::Shl
        | Opcode::Shr
        | Opcode::Rotl
        | Opcode::Rotr
        | Opcode::Eq
        | Opcode::Neq
        | Opcode::Gt
        | Opcode::Gte
        | Opcode::Lt
        | Opcode::Lte
        | Opcode::Min
        | Opcode::Max => panic!("Unsupported instruction: BinaryOp (opcode {:?})", op),
        _ => panic!("Illegal opcode {:?} for instruction BinaryOp", op),
    }
}

fn codegen_binary_op_imm(op: Opcode, overflow: &Overflow, imm_value: u32) -> masm::Node {
    match op {
        Opcode::Store => panic!("Unsupported instruction: BinaryOpImm (opcode Store)"),
        Opcode::Add =>
            match overflow {
                Overflow::Unchecked => panic!("No such instruction: BinaryOpImm, Opcode Add, Overflow Unchecked"),
                Overflow::Checked => masm_instruction_as_node(masm::Instruction::U32CheckedAddImm(imm_value)),
                Overflow::Wrapping => masm_instruction_as_node(masm::Instruction::U32WrappingAddImm(imm_value)),
                Overflow::Overflowing => masm_instruction_as_node(masm::Instruction::U32OverflowingAddImm(imm_value)),
            },
        Opcode::Sub =>
            match overflow {
                Overflow::Unchecked => panic!("No such instruction: BinaryOpImm, Opcode Sub, Overflow Unchecked"),
                Overflow::Checked => masm_instruction_as_node(masm::Instruction::U32CheckedSubImm(imm_value)),
                Overflow::Wrapping => masm_instruction_as_node(masm::Instruction::U32WrappingSubImm(imm_value)),
                Overflow::Overflowing => masm_instruction_as_node(masm::Instruction::U32OverflowingSubImm(imm_value)),
            },
        Opcode::Mul =>
            match overflow {
                Overflow::Unchecked => panic!("No such instruction: BinaryOpImm, Opcode Mul, Overflow Unchecked"),
                Overflow::Checked => masm_instruction_as_node(masm::Instruction::U32CheckedMulImm(imm_value)),
                Overflow::Wrapping => masm_instruction_as_node(masm::Instruction::U32WrappingMulImm(imm_value)),
                Overflow::Overflowing => masm_instruction_as_node(masm::Instruction::U32OverflowingMulImm(imm_value)),
            },
        Opcode::Div =>
            match overflow {
                Overflow::Unchecked => masm_instruction_as_node(masm::Instruction::U32UncheckedDivImm(imm_value)),
                Overflow::Checked => masm_instruction_as_node(masm::Instruction::U32CheckedDivImm(imm_value)),
                Overflow::Wrapping => panic!("No such instruction: BinaryOpImm, Opcode Div, Overflow Wrapping"),
                Overflow::Overflowing => panic!("No such instruction: BinaryOpImm, Opcode Div, Overflow Overflowing"),
            },
        Opcode::Mod =>
            match overflow {
                Overflow::Unchecked => masm_instruction_as_node(masm::Instruction::U32UncheckedModImm(imm_value)),
                Overflow::Checked => masm_instruction_as_node(masm::Instruction::U32CheckedModImm(imm_value)),
                Overflow::Wrapping => panic!("No such instruction: BinaryOpImm, Opcode Mod, Overflow Wrapping"),
                Overflow::Overflowing => panic!("No such instruction: BinaryOpImm, Opcode Mod, Overflow Overflowing"),
            },
        Opcode::DivMod =>
            match overflow {
                Overflow::Unchecked => masm_instruction_as_node(masm::Instruction::U32UncheckedDivModImm(imm_value)),
                Overflow::Checked => masm_instruction_as_node(masm::Instruction::U32CheckedDivModImm(imm_value)),
                Overflow::Wrapping => panic!("No such instruction: BinaryOpImm, Opcode DivMod, Overflow Wrapping"),
                Overflow::Overflowing => panic!("No such instruction: BinaryOpImm, Opcode DivMod, Overflow Overflowing"),
            },
        Opcode::Exp // Not handling this, since there is no 32-bit Exp operation in MASM
        | Opcode::And
        | Opcode::Or
        | Opcode::Xor
        | Opcode::Shl
        | Opcode::Shr
        | Opcode::Rotl
        | Opcode::Rotr
        | Opcode::Eq
        | Opcode::Neq
        | Opcode::Gt
        | Opcode::Gte
        | Opcode::Lt
        | Opcode::Lte
        | Opcode::Min
        | Opcode::Max => panic!("Unsupported instruction: BinaryOpImm (opcode {:?})", op),
        _ => panic!("Illegal opcode {:?} for instruction BinaryOpImm", op),
    }
}

fn codegen_unary_op(op: Opcode) -> masm::Node {
    match op {
        Opcode::AddrOf
        | Opcode::Load
        | Opcode::PtrToInt
        | Opcode::IntToPtr
        | Opcode::Cast
        | Opcode::Trunc
        | Opcode::Zext
        | Opcode::Sext
        | Opcode::Neg
        | Opcode::Inv
        | Opcode::Pow2 // Not handling this, since there is no 32-bit Pow2 operation in MASM
        | Opcode::Not
        | Opcode::Popcnt
        | Opcode::IsOdd => panic!("Unsupported instruction: UnaryOp (opcode {:?})", op),
        _ => panic!("Illegal opcode {:?} for instruction UnaryOp", op),
    }
}

fn codegen_unary_op_imm(op: Opcode, _imm_value: u32) -> masm::Node {
    match op {
        Opcode::AddrOf
        | Opcode::Load
        | Opcode::PtrToInt
        | Opcode::IntToPtr
        | Opcode::Cast
        | Opcode::Trunc
        | Opcode::Zext
        | Opcode::Sext
        | Opcode::Neg
        | Opcode::Inv
        | Opcode::Pow2 // Not handling this, since there is no 32-bit Pow2 operation in MASM
        | Opcode::Not
        | Opcode::Popcnt
        | Opcode::IsOdd => panic!("Unsupported instruction: UnaryOpImm (opcode {:?})", op),
        _ => panic!("Illegal opcode {:?} for instruction UnaryOpImm", op),
    }
}

fn codegen_call(op: Opcode) -> masm::Node {
    match op {
        Opcode::Call => panic!("Unsupported instruction: Call (opcode {:?})", op),
        _ => panic!("Illegal opcode {:?} for instruction Call", op),
    }
}

fn codegen_br(op: Opcode) -> masm::Node {
    match op {
        Opcode::Br => panic!("Unsupported instruction: Br (opcode {:?})", op),
        _ => panic!("Illegal opcode {:?} for instruction Br", op),
    }
}

fn codegen_cond_br(op: Opcode) -> masm::Node {
    match op {
        Opcode::CondBr => panic!("Unsupported instruction: CondBr (opcode {:?})", op),
        _ => panic!("Illegal opcode {:?} for instruction CondBr", op),
    }
}

fn codegen_switch(op: Opcode) -> masm::Node {
    match op {
        Opcode::Switch => panic!("Unsupported instruction: Switch (opcode {:?})", op),
        _ => panic!("Illegal opcode {:?} for instruction Switch", op),
    }
}

fn codegen_ret(op: Opcode) -> masm::Node {
    match op {
        Opcode::Ret => panic!("Unsupported instruction: Ret (opcode {:?})", op),
        _ => panic!("Illegal opcode {:?} for instruction Ret", op),
    }
}

fn codegen_mem_cpy(op: Opcode) -> masm::Node {
    match op {
        Opcode::MemCpy => panic!("Unsupported instruction: MemCpy (opcode {:?})", op),
        _ => panic!("Illegal opcode {:?} for instruction MemCpy", op),
    }
}

fn codegen_prim_op(op: Opcode) -> masm::Node {
    match op {
        Opcode::Assert | Opcode::Assertz | Opcode::AssertEq => {
            panic!("Unsupported instruction: PrimOp (opcode {:?})", op)
        }
        _ => panic!("Illegal opcode {:?} for instruction PrimOp", op),
    }
}

fn codegen_prim_op_imm(op: Opcode, _imm_value: u32) -> masm::Node {
    match op {
        Opcode::AssertEq => panic!("Unsupported instruction: PrimOpImm (opcode {:?})", op),
        _ => panic!("Illegal opcode {:?} for instruction PrimOpImm", op),
    }
}

fn codegen_test(op: Opcode) -> masm::Node {
    match op {
        Opcode::AssertTest | Opcode::Test => {
            panic!("Unsupported instruction: Test (opcode {:?})", op)
        }
        _ => panic!("Illegal opcode {:?} for instruction Test", op),
    }
}

fn codegen_inline_asm(op: Opcode) -> masm::Node {
    match op {
        Opcode::InlineAsm => panic!("Unsupported instruction: InlineAsm (opcode {:?})", op),
        _ => panic!("Illegal opcode {:?} for instruction InlineAsm", op),
    }
}

fn codegen_instruction(
    source: Inst,
    dfg: &DataFlowGraph,
    locals_map: &SecondaryMap<Value, u16>,
    res_insts: &mut Vec<masm::Node>,
) {
    let instruction = &dfg.insts[source].item;

    // Push arguments onto the stack
    let args = instruction.arguments(&dfg.value_lists);
    for v in args.iter() {
        res_insts.push(masm_instruction_as_node(masm::Instruction::LocStore(
            locals_map[*v],
        )));
    }

    // Perform the op
    // TODO: We are currently assuming that all operations are 32-bit. This will not be a valid assumption in general.
    let op = instruction.opcode();
    let op_res = match instruction {
        Instruction::BinaryOp(BinaryOp { ref overflow, .. }) => codegen_binary_op(op, overflow),
        Instruction::BinaryOpImm(BinaryOpImm {
            ref overflow,
            ref imm,
            ..
        }) => codegen_binary_op_imm(op, overflow, immediate_as_u32(imm)),
        Instruction::UnaryOp(_) => codegen_unary_op(op),
        Instruction::UnaryOpImm(UnaryOpImm { ref imm, .. }) => {
            codegen_unary_op_imm(op, immediate_as_u32(imm))
        }
        Instruction::Call(_) => codegen_call(op),
        Instruction::Br(_) => codegen_br(op),
        Instruction::CondBr(_) => codegen_cond_br(op),
        Instruction::Switch(_) => codegen_switch(op),
        Instruction::Ret(_) => codegen_ret(op),
        Instruction::MemCpy(_) => codegen_mem_cpy(op),
        Instruction::PrimOp(_) => codegen_prim_op(op),
        Instruction::PrimOpImm(PrimOpImm { ref imm, .. }) => {
            codegen_prim_op_imm(op, immediate_as_u32(imm))
        }
        Instruction::Test(_) => codegen_test(op),
        Instruction::InlineAsm(_) => codegen_inline_asm(op),
    };
    res_insts.push(op_res);

    // Any results of the operation are now on the stack. Store them as locals.
    let res_vals = dfg.inst_results(source);
    // Start from the last value, since that is the one on top of the stack.
    for v in res_vals.iter().rev() {
        res_insts.push(masm_instruction_as_node(masm::Instruction::LocStore(
            locals_map[*v],
        )));
    }
}

fn codegen_function(source: &Function) -> masm::ProcedureAst {
    let dfg = &source.dfg;
    //Determine the number of locals. We assign
    // - a local for each value in the dfg
    // - a local for each parameter
    let locals_count = dfg.values.len();
    let mut locals_map: SecondaryMap<Value, u16> = SecondaryMap::with_capacity(locals_count);
    let mut i = 0;
    for v in dfg.values.keys() {
        locals_map[v] = i;
        i += 1;
    }

    let mut compiled_instructions: Vec<masm::Node> = Vec::new();

    //Parameters are initially stored on the stack, but we need to read them from local memory, so store parameters as locals
    let entry_block = dfg
        .entry_block()
        .expect("No entry block found for function {source}");
    for v in dfg.block_params(entry_block) {
        compiled_instructions.push(masm_instruction_as_node(masm::Instruction::LocStore(
            locals_map[*v],
        )));
    }

    //TODO: Handle branches properly - we currently assume no branching = only one block.

    debug_assert!(dfg.num_blocks() == 1, "too many blocks");
    for inst in dfg.block_insts(entry_block) {
        codegen_instruction(inst, &dfg, &locals_map, &mut compiled_instructions);
    }

    masm::ProcedureAst::new(
        masm::ProcedureName::try_from(source.signature.name.clone())
            .expect("Illegal function name"),
        u16::try_from(locals_count).expect("Illegal number of locals (this cannot happen)"),
        compiled_instructions,
        true,
        Option::None,
    )
}

fn codegen_module(source: &Module) -> masm::ModuleAst {
    let compiled_functions: Vec<masm::ProcedureAst> = source
        .functions
        .iter()
        .map(|f| codegen_function(f))
        .collect();
    match masm::ModuleAst::new(compiled_functions, Option::None) {
        Err(err) => panic!("{}", err.message()),
        Ok(res) => res,
    }
}

fn codegen(source: &Module) -> masm::ModuleAst {
    codegen_module(source)
}
