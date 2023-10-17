//! This module contains the bulk of the code performing the translation between
//! WebAssembly and Miden IR.
//!
//! The translation is done in one pass, opcode by opcode. Two main data structures are used during
//! code translations: the value stack and the control stack. The value stack mimics the execution
//! of the WebAssembly stack machine: each instruction result is pushed onto the stack and
//! instruction arguments are popped off the stack. Similarly, when encountering a control flow
//! block, it is pushed onto the control stack and popped off when encountering the corresponding
//! `End`.
//!
//! Another data structure, the translation state, records information concerning unreachable code
//! status and about if inserting a return at the end of the function is necessary.
//!
//! Based on Cranelift's Wasm -> CLIF translator v11.0.0

use std::collections::{hash_map, HashMap};

use crate::error::{WasmError, WasmResult};
use crate::func_translation_state::ControlStackFrame;
use crate::func_translation_state::{ElseData, FuncTranslationState};
use crate::function_builder_ext::FunctionBuilderExt;
use crate::module_env::ModuleInfo;
use crate::ssa::Variable;
use crate::unsupported_diag;
use crate::wasm_types::{BlockType, GlobalIndex};
use miden_diagnostics::{DiagnosticsHandler, SourceSpan};
use miden_hir::cranelift_entity::packed_option::ReservedValue;
use miden_hir::Type;
use miden_hir::Type::*;
use miden_hir::{Block, Inst, InstBuilder, Value};
use wasmparser::{MemArg, Operator};

#[cfg(test)]
mod tests;

#[cfg(test)]
mod tests_unsupported;

/// Translates wasm operators into Miden IR instructions.
pub fn translate_operator(
    op: &Operator,
    builder: &mut FunctionBuilderExt,
    state: &mut FuncTranslationState,
    mod_info: &ModuleInfo,
    diagnostics: &DiagnosticsHandler,
    span: SourceSpan,
) -> WasmResult<()> {
    if !state.reachable {
        translate_unreachable_operator(&op, builder, state, mod_info, span)?;
        return Ok(());
    }

    // Given that we believe the current block is reachable, the FunctionBuilderExt ought to agree.
    debug_assert!(!builder.is_unreachable());

    match op {
        /********************************** Locals ****************************************
         *  `get_local` and `set_local` are treated as non-SSA variables and will completely
         *  disappear in the Miden IR
         ***********************************************************************************/
        Operator::LocalGet { local_index } => {
            let val = builder.use_var(Variable::from_u32(*local_index));
            state.push1(val);
        }
        Operator::LocalSet { local_index } => {
            let val = state.pop1();
            builder.def_var(Variable::from_u32(*local_index), val);
        }
        Operator::LocalTee { local_index } => {
            let val = state.peek1();
            builder.def_var(Variable::from_u32(*local_index), val);
        }
        /********************************** Globals ****************************************/
        Operator::GlobalGet { global_index } => {
            let global_index = GlobalIndex::from_u32(*global_index);
            let name = mod_info.global_name(global_index);
            let ty = mod_info.globals[global_index].ty.clone();
            state.push1(builder.ins().load_symbol(name, ty, span));
        }
        Operator::GlobalSet { global_index } => {
            let global_index = GlobalIndex::from_u32(*global_index);
            let name = mod_info.global_name(global_index);
            let ty = (&mod_info.globals[global_index]).ty.clone();
            let ptr = builder
                .ins()
                .symbol_addr(name, Ptr(ty.clone().into()), span);
            let val = state.pop1();
            builder.ins().store(ptr, val, span);
        }
        /********************************* Stack misc **************************************/
        Operator::Drop => _ = state.pop1(),
        Operator::Select => {
            let (arg1, arg2, cond) = state.pop3();
            // if cond is not 0, return arg1, else return arg2
            // https://www.w3.org/TR/wasm-core-1/#-hrefsyntax-instr-parametricmathsfselect%E2%91%A0
            let zero = builder.ins().i32(0, span);
            let cond_i1 = builder.ins().neq(cond, zero, span);
            state.push1(builder.ins().select(cond_i1, arg1, arg2, span));
        }
        Operator::Unreachable => {
            builder.ins().unreachable(span);
            state.reachable = false;
        }
        Operator::Nop => {}
        /***************************** Control flow blocks *********************************/
        Operator::Block { blockty } => translate_block(blockty, builder, state, mod_info, span)?,
        Operator::Loop { blockty } => translate_loop(blockty, builder, state, mod_info, span)?,
        Operator::If { blockty } => translate_if(blockty, state, builder, mod_info, span)?,
        Operator::Else => translate_else(state, builder, span)?,
        Operator::End => translate_end(state, builder, span),

        /**************************** Branch instructions *********************************/
        Operator::Br { relative_depth } => translate_br(state, relative_depth, builder, span),
        Operator::BrIf { relative_depth } => translate_br_if(*relative_depth, builder, state, span),
        Operator::BrTable { targets } => translate_br_table(targets, state, builder, span)?,
        Operator::Return => translate_return(state, builder, diagnostics, span)?,
        /************************************ Calls ****************************************/
        Operator::Call { function_index } => {
            translate_call(state, builder, function_index, mod_info, span, diagnostics)?;
        }
        /******************************* Memory management *********************************/
        Operator::MemoryGrow { .. } => {
            let arg = state.pop1_casted(U32, builder, span);
            state.push1(builder.ins().mem_grow(arg, span));
        }
        Operator::MemorySize { .. } => {
            // Return total Miden memory size
            state.push1(builder.ins().i32(mem_total_pages(), span));
        }
        /******************************* Load instructions ***********************************/
        Operator::I32Load8U { memarg } => {
            translate_load_zext(U8, I32, memarg, state, builder, span)
        }
        Operator::I32Load16U { memarg } => {
            translate_load_zext(U16, I32, memarg, state, builder, span)
        }
        Operator::I32Load8S { memarg } => {
            translate_load_sext(I8, I32, memarg, state, builder, span);
        }
        Operator::I32Load16S { memarg } => {
            translate_load_sext(I16, I32, memarg, state, builder, span);
        }
        Operator::I64Load8U { memarg } => {
            translate_load_zext(U8, I64, memarg, state, builder, span)
        }
        Operator::I64Load16U { memarg } => {
            translate_load_zext(U16, I64, memarg, state, builder, span)
        }
        Operator::I64Load8S { memarg } => {
            translate_load_sext(I8, I64, memarg, state, builder, span);
        }
        Operator::I64Load16S { memarg } => {
            translate_load_sext(I16, I64, memarg, state, builder, span);
        }
        Operator::I64Load32S { memarg } => {
            translate_load_sext(I32, I64, memarg, state, builder, span)
        }
        Operator::I64Load32U { memarg } => {
            translate_load_zext(U32, I64, memarg, state, builder, span)
        }
        Operator::I32Load { memarg } => translate_load(I32, memarg, state, builder, span),
        Operator::I64Load { memarg } => translate_load(I64, memarg, state, builder, span),
        /****************************** Store instructions ***********************************/
        Operator::I32Store { memarg } => translate_store(I32, memarg, state, builder, span),
        Operator::I64Store { memarg } => translate_store(I64, memarg, state, builder, span),
        Operator::I32Store8 { memarg } | Operator::I64Store8 { memarg } => {
            translate_store(U8, memarg, state, builder, span);
        }
        Operator::I32Store16 { memarg } | Operator::I64Store16 { memarg } => {
            translate_store(U16, memarg, state, builder, span);
        }
        Operator::I64Store32 { memarg } => translate_store(U32, memarg, state, builder, span),
        /****************************** Nullary Operators **********************************/
        Operator::I32Const { value } => state.push1(builder.ins().i32(*value, span)),
        Operator::I64Const { value } => state.push1(builder.ins().i64(*value, span)),

        /******************************* Unary Operators *************************************/
        Operator::I32Clz | Operator::I32Ctz => {
            // Temporary workaround to allow further code translations
            // until clz and ctz are available in Miden IR
            // TODO: use the `clz` and `ctz` instructions when they are available
            let val = state.pop1();
            state.push1(builder.ins().popcnt(val, span));
        }
        Operator::I32Popcnt | Operator::I64Popcnt => {
            let val = state.pop1();
            state.push1(builder.ins().popcnt(val, span));
        }
        Operator::I64ExtendI32S => {
            let val = state.pop1();
            state.push1(builder.ins().sext(val, I64, span));
        }
        Operator::I64ExtendI32U => {
            let val = state.pop1();
            state.push1(builder.ins().zext(val, I64, span));
        }
        Operator::I32WrapI64 => {
            let val = state.pop1();
            state.push1(builder.ins().trunc(val, I32, span));
        }
        /****************************** Binary Operators ************************************/
        Operator::I32Add | Operator::I64Add => {
            let (arg1, arg2) = state.pop2();
            // wrapping because the result is mod 2^N
            // https://www.w3.org/TR/wasm-core-1/#op-iadd
            state.push1(builder.ins().add_wrapping(arg1, arg2, span));
        }
        Operator::I32And | Operator::I64And => {
            let (arg1, arg2) = state.pop2();
            state.push1(builder.ins().band(arg1, arg2, span));
        }
        Operator::I32Or | Operator::I64Or => {
            let (arg1, arg2) = state.pop2();
            state.push1(builder.ins().bor(arg1, arg2, span));
        }
        Operator::I32Xor | Operator::I64Xor => {
            let (arg1, arg2) = state.pop2();
            state.push1(builder.ins().bxor(arg1, arg2, span));
        }
        Operator::I32Shl | Operator::I64Shl => {
            let (arg1, arg2) = state.pop2();
            state.push1(builder.ins().shl(arg1, arg2, span));
        }
        Operator::I32ShrU => {
            let (arg1, arg2) = state.pop2_casted(U32, builder, span);
            let val = builder.ins().shr(arg1, arg2, span);
            state.push1(builder.ins().cast(val, I32, span));
        }
        Operator::I64ShrU => {
            let (arg1, arg2) = state.pop2_casted(U64, builder, span);
            let val = builder.ins().shr(arg1, arg2, span);
            state.push1(builder.ins().cast(val, I64, span));
        }
        Operator::I32ShrS | Operator::I64ShrS => {
            let (arg1, arg2) = state.pop2();
            state.push1(builder.ins().shr(arg1, arg2, span));
        }
        Operator::I32Rotl | Operator::I64Rotl => {
            let (arg1, arg2) = state.pop2();
            state.push1(builder.ins().shl_wrapping(arg1, arg2, span));
        }
        Operator::I32Rotr | Operator::I64Rotr => {
            let (arg1, arg2) = state.pop2();
            state.push1(builder.ins().shr_wrapping(arg1, arg2, span));
        }
        Operator::I32Sub | Operator::I64Sub => {
            let (arg1, arg2) = state.pop2();
            // wrapping because the result is mod 2^N
            // https://www.w3.org/TR/wasm-core-1/#op-isub
            state.push1(builder.ins().sub_wrapping(arg1, arg2, span));
        }
        Operator::F64Sub => {
            let (arg1, arg2) = state.pop2();
            state.push1(builder.ins().sub(arg1, arg2, span));
        }
        Operator::I32Mul | Operator::I64Mul => {
            let (arg1, arg2) = state.pop2();
            // wrapping because the result is mod 2^N
            // https://www.w3.org/TR/wasm-core-1/#op-imul
            state.push1(builder.ins().mul_wrapping(arg1, arg2, span));
        }
        Operator::I32DivS | Operator::I64DivS => {
            let (arg1, arg2) = state.pop2();
            state.push1(builder.ins().div(arg1, arg2, span));
        }
        Operator::I32DivU => {
            let (arg1, arg2) = state.pop2_casted(U32, builder, span);
            let val = builder.ins().div(arg1, arg2, span);
            state.push1(builder.ins().cast(val, I32, span));
        }
        Operator::I64DivU => {
            let (arg1, arg2) = state.pop2_casted(U64, builder, span);
            let val = builder.ins().div(arg1, arg2, span);
            state.push1(builder.ins().cast(val, I64, span));
        }
        Operator::I32RemU => {
            let (arg1, arg2) = state.pop2_casted(U32, builder, span);
            let val = builder.ins().r#mod(arg1, arg2, span);
            state.push1(builder.ins().cast(val, I32, span));
        }
        Operator::I64RemU => {
            let (arg1, arg2) = state.pop2_casted(U64, builder, span);
            let val = builder.ins().r#mod(arg1, arg2, span);
            state.push1(builder.ins().cast(val, I64, span));
        }
        Operator::I32RemS | Operator::I64RemS => {
            let (arg1, arg2) = state.pop2();
            state.push1(builder.ins().r#mod(arg1, arg2, span));
        }
        /**************************** Comparison Operators **********************************/
        Operator::I32LtU => {
            let (arg0, arg1) = state.pop2_casted(U32, builder, span);
            let val = builder.ins().lt(arg0, arg1, span);
            state.push1(builder.ins().cast(val, I32, span));
        }
        Operator::I64LtU => {
            let (arg0, arg1) = state.pop2_casted(U64, builder, span);
            let val = builder.ins().lt(arg0, arg1, span);
            state.push1(builder.ins().cast(val, I32, span));
        }
        Operator::I32LtS => {
            let (arg0, arg1) = state.pop2();
            let val = builder.ins().lt(arg0, arg1, span);
            state.push1(builder.ins().cast(val, I32, span));
        }
        Operator::I64LtS => {
            let (arg0, arg1) = state.pop2();
            let val = builder.ins().lt(arg0, arg1, span);
            state.push1(builder.ins().cast(val, I32, span));
        }
        Operator::I32LeU => {
            let (arg0, arg1) = state.pop2_casted(U32, builder, span);
            let val = builder.ins().lte(arg0, arg1, span);
            state.push1(builder.ins().cast(val, I32, span));
        }
        Operator::I64LeU => {
            let (arg0, arg1) = state.pop2_casted(U64, builder, span);
            let val = builder.ins().lte(arg0, arg1, span);
            state.push1(builder.ins().cast(val, I32, span));
        }
        Operator::I32LeS => {
            let (arg0, arg1) = state.pop2();
            let val = builder.ins().lte(arg0, arg1, span);
            state.push1(builder.ins().cast(val, I32, span));
        }
        Operator::I64LeS => {
            let (arg0, arg1) = state.pop2();
            let val = builder.ins().lte(arg0, arg1, span);
            state.push1(builder.ins().cast(val, I32, span));
        }
        Operator::I32GtU => {
            let (arg0, arg1) = state.pop2_casted(U32, builder, span);
            let val = builder.ins().gt(arg0, arg1, span);
            state.push1(builder.ins().cast(val, I32, span));
        }
        Operator::I64GtU => {
            let (arg0, arg1) = state.pop2_casted(U64, builder, span);
            let val = builder.ins().gt(arg0, arg1, span);
            state.push1(builder.ins().cast(val, I32, span));
        }
        Operator::I32GtS | Operator::I64GtS => {
            let (arg0, arg1) = state.pop2();
            let val = builder.ins().gt(arg0, arg1, span);
            state.push1(builder.ins().cast(val, I32, span));
        }
        Operator::I32GeU => {
            let (arg0, arg1) = state.pop2_casted(U32, builder, span);
            let val = builder.ins().gte(arg0, arg1, span);
            state.push1(builder.ins().cast(val, I32, span));
        }
        Operator::I64GeU => {
            let (arg0, arg1) = state.pop2_casted(U64, builder, span);
            let val = builder.ins().gte(arg0, arg1, span);
            state.push1(builder.ins().cast(val, I32, span));
        }
        Operator::I32GeS => {
            let (arg0, arg1) = state.pop2();
            let val = builder.ins().gte(arg0, arg1, span);
            state.push1(builder.ins().cast(val, I32, span));
        }
        Operator::I64GeS => {
            let (arg0, arg1) = state.pop2();
            let val = builder.ins().gte(arg0, arg1, span);
            state.push1(builder.ins().cast(val, I32, span));
        }
        Operator::I32Eqz => {
            let arg = state.pop1();
            let imm_zero = builder.ins().i32(0, span);
            let val = builder.ins().eq(arg, imm_zero, span);
            state.push1(builder.ins().cast(val, I32, span));
        }
        Operator::I64Eqz => {
            let arg = state.pop1();
            let imm_zero = builder.ins().i64(0, span);
            let val = builder.ins().eq(arg, imm_zero, span);
            state.push1(builder.ins().cast(val, I32, span));
        }
        Operator::I32Eq => {
            let (arg0, arg1) = state.pop2();
            let val = builder.ins().eq(arg0, arg1, span);
            state.push1(builder.ins().cast(val, I32, span));
        }
        Operator::I64Eq => {
            let (arg0, arg1) = state.pop2();
            let val = builder.ins().eq(arg0, arg1, span);
            state.push1(builder.ins().cast(val, I32, span));
        }
        Operator::I32Ne => {
            let (arg0, arg1) = state.pop2();
            let val = builder.ins().neq(arg0, arg1, span);
            state.push1(builder.ins().cast(val, I32, span));
        }
        Operator::I64Ne => {
            let (arg0, arg1) = state.pop2();
            let val = builder.ins().neq(arg0, arg1, span);
            state.push1(builder.ins().cast(val, I32, span));
        }
        op => {
            unsupported_diag!(diagnostics, "Wasm op {:?} is not supported", op);
        }
    };
    Ok(())
}

fn translate_br_table(
    targets: &wasmparser::BrTable<'_>,
    state: &mut FuncTranslationState,
    builder: &mut FunctionBuilderExt,
    span: SourceSpan,
) -> Result<(), WasmError> {
    let default = targets.default();
    let mut min_depth = default;
    for depth in targets.targets() {
        let depth = depth?;
        if depth < min_depth {
            min_depth = depth;
        }
    }
    let jump_args_count = {
        let i = state.control_stack.len() - 1 - (min_depth as usize);
        let min_depth_frame = &state.control_stack[i];
        if min_depth_frame.is_loop() {
            min_depth_frame.num_param_values()
        } else {
            min_depth_frame.num_return_values()
        }
    };
    let val = state.pop1();
    let val = if builder.data_flow_graph().value_type(val) != &U32 {
        builder.ins().cast(val, U32, span)
    } else {
        val
    };
    let mut data = Vec::with_capacity(targets.len() as usize);
    if jump_args_count == 0 {
        // No jump arguments
        for depth in targets.targets() {
            let depth = depth?;
            let block = {
                let i = state.control_stack.len() - 1 - (depth as usize);
                let frame = &mut state.control_stack[i];
                frame.set_branched_to_exit();
                frame.br_destination()
            };
            data.push((depth, block));
        }
        let def_block = {
            let i = state.control_stack.len() - 1 - (default as usize);
            let frame = &mut state.control_stack[i];
            frame.set_branched_to_exit();
            frame.br_destination()
        };
        builder.ins().switch(val, data, def_block, span);
    } else {
        // Here we have jump arguments, but Midens's switch op doesn't support them
        // We then proceed to split the edges going out of the br_table
        let return_count = jump_args_count;
        let mut dest_block_sequence = vec![];
        let mut dest_block_map = HashMap::new();
        for depth in targets.targets() {
            let depth = depth?;
            let branch_block = match dest_block_map.entry(depth as usize) {
                hash_map::Entry::Occupied(entry) => *entry.get(),
                hash_map::Entry::Vacant(entry) => {
                    let block = builder.create_block();
                    dest_block_sequence.push((depth as usize, block));
                    *entry.insert(block)
                }
            };
            data.push((depth, branch_block));
        }
        let default_branch_block = match dest_block_map.entry(default as usize) {
            hash_map::Entry::Occupied(entry) => *entry.get(),
            hash_map::Entry::Vacant(entry) => {
                let block = builder.create_block();
                dest_block_sequence.push((default as usize, block));
                *entry.insert(block)
            }
        };
        builder.ins().switch(val, data, default_branch_block, span);
        for (depth, dest_block) in dest_block_sequence {
            builder.switch_to_block(dest_block);
            builder.seal_block(dest_block);
            let real_dest_block = {
                let i = state.control_stack.len() - 1 - depth;
                let frame = &mut state.control_stack[i];
                frame.set_branched_to_exit();
                frame.br_destination()
            };
            let destination_args = state.peekn_mut(return_count);
            builder.ins().br(real_dest_block, destination_args, span);
        }
        state.popn(return_count);
    }
    state.reachable = false;
    Ok(())
}

/// Return the total Miden VM memory size (2^32 addresses * word (4 felts) bytes) in 64KB pages
const fn mem_total_pages() -> i32 {
    let bytes_fit_in_felt = 4; // although more than 32 bits can fit into a felt, use 32 bits to be safe
    let felts_in_word = 4;
    let total_addresses = u32::MAX as i64;
    (total_addresses * (felts_in_word * bytes_fit_in_felt) / (64 * 1024)) as i32
}

fn translate_load(
    ptr_ty: Type,
    memarg: &MemArg,
    state: &mut FuncTranslationState,
    builder: &mut FunctionBuilderExt,
    span: SourceSpan,
) {
    let addr_int = state.pop1();
    let addr = prepare_addr(addr_int, &ptr_ty, memarg, builder, span);
    state.push1(builder.ins().load(addr, span));
}

fn translate_load_sext(
    ptr_ty: Type,
    sext_ty: Type,
    memarg: &MemArg,
    state: &mut FuncTranslationState,
    builder: &mut FunctionBuilderExt,
    span: SourceSpan,
) {
    let addr_int = state.pop1();
    let addr = prepare_addr(addr_int, &ptr_ty, memarg, builder, span);
    let val = builder.ins().load(addr, span);
    let sext_val = builder.ins().sext(val, sext_ty, span);
    state.push1(sext_val);
}

fn translate_load_zext(
    ptr_ty: Type,
    zext_ty: Type,
    memarg: &MemArg,
    state: &mut FuncTranslationState,
    builder: &mut FunctionBuilderExt,
    span: SourceSpan,
) {
    assert!(ptr_ty.is_unsigned_integer());
    let addr_int = state.pop1();
    let addr = prepare_addr(addr_int, &ptr_ty, memarg, builder, span);
    let val = builder.ins().load(addr, span);
    let sext_val = builder.ins().zext(val, zext_ty, span);
    state.push1(sext_val);
}

fn translate_store(
    ptr_ty: Type,
    memarg: &MemArg,
    state: &mut FuncTranslationState,
    builder: &mut FunctionBuilderExt,
    span: SourceSpan,
) {
    let (addr_int, val) = state.pop2();
    let val_ty = builder.data_flow_graph().value_type(val);
    let arg = if ptr_ty != *val_ty {
        builder.ins().trunc(val, ptr_ty.clone(), span)
    } else {
        val
    };
    let addr = prepare_addr(addr_int, &ptr_ty, memarg, builder, span);
    builder.ins().store(addr, arg, span);
}

fn prepare_addr(
    addr_int: Value,
    ptr_ty: &Type,
    memarg: &MemArg,
    builder: &mut FunctionBuilderExt,
    span: SourceSpan,
) -> Value {
    let addr_int_ty = builder.data_flow_graph().value_type(addr_int);
    let addr_u32 = if *addr_int_ty == U32 {
        addr_int
    } else {
        builder.ins().cast(addr_int, U32, span)
    };
    let full_addr_int = if memarg.offset != 0 {
        builder.ins().add_imm(addr_u32, memarg.offset.into(), span)
    } else {
        addr_u32
    };
    builder
        .ins()
        .inttoptr(full_addr_int, Type::Ptr(ptr_ty.clone().into()), span)
}

fn translate_call(
    state: &mut FuncTranslationState,
    builder: &mut FunctionBuilderExt,
    function_index: &u32,
    mod_info: &ModuleInfo,
    span: SourceSpan,
    diagnostics: &DiagnosticsHandler,
) -> WasmResult<()> {
    let (fident, num_args) = state.get_direct_func(
        builder.data_flow_graph_mut(),
        *function_index,
        mod_info,
        diagnostics,
    )?;
    let args = state.peekn_mut(num_args);
    let call = builder.ins().call(fident, &args, span);
    let inst_results = builder.inst_results(call);
    state.popn(num_args);
    state.pushn(inst_results);
    Ok(())
}

fn translate_return(
    state: &mut FuncTranslationState,
    builder: &mut FunctionBuilderExt,
    diagnostics: &DiagnosticsHandler,
    span: SourceSpan,
) -> WasmResult<()> {
    let return_count = {
        let frame = &mut state.control_stack[0];
        frame.num_return_values()
    };
    {
        let return_args = match return_count {
            0 => None,
            1 => Some(state.peekn_mut(return_count).first().unwrap().clone()),
            _ => {
                unsupported_diag!(diagnostics, "Multiple values are not supported");
            }
        };

        builder.ins().ret(return_args, span);
    }
    state.popn(return_count);
    state.reachable = false;
    Ok(())
}

fn translate_br(
    state: &mut FuncTranslationState,
    relative_depth: &u32,
    builder: &mut FunctionBuilderExt,
    span: SourceSpan,
) {
    let i = state.control_stack.len() - 1 - (*relative_depth as usize);
    let (return_count, br_destination) = {
        let frame = &mut state.control_stack[i];
        // We signal that all the code that follows until the next End is unreachable
        frame.set_branched_to_exit();
        let return_count = if frame.is_loop() {
            frame.num_param_values()
        } else {
            frame.num_return_values()
        };
        (return_count, frame.br_destination())
    };
    let destination_args = state.peekn_mut(return_count);
    builder.ins().br(br_destination, &destination_args, span);
    state.popn(return_count);
    state.reachable = false;
}

fn translate_br_if(
    relative_depth: u32,
    builder: &mut FunctionBuilderExt,
    state: &mut FuncTranslationState,
    span: SourceSpan,
) {
    let cond = state.pop1();
    let (br_destination, inputs) = translate_br_if_args(relative_depth, state);
    let next_block = builder.create_block();
    let then_dest = br_destination;
    let then_args = inputs;
    let else_dest = next_block;
    let else_args = &[];
    let zero = builder.ins().i32(0, span);
    let cond_i1 = builder.ins().neq(cond, zero, span);
    builder
        .ins()
        .cond_br(cond_i1, then_dest, then_args, else_dest, else_args, span);
    builder.seal_block(next_block); // The only predecessor is the current block.
    builder.switch_to_block(next_block);
}

fn translate_br_if_args(
    relative_depth: u32,
    state: &mut FuncTranslationState,
) -> (Block, &mut [Value]) {
    let i = state.control_stack.len() - 1 - (relative_depth as usize);
    let (return_count, br_destination) = {
        let frame = &mut state.control_stack[i];
        // The values returned by the branch are still available for the reachable
        // code that comes after it
        frame.set_branched_to_exit();
        let return_count = if frame.is_loop() {
            frame.num_param_values()
        } else {
            frame.num_return_values()
        };
        (return_count, frame.br_destination())
    };
    let inputs = state.peekn_mut(return_count);
    (br_destination, inputs)
}

fn translate_block(
    blockty: &wasmparser::BlockType,
    builder: &mut FunctionBuilderExt,
    state: &mut FuncTranslationState,
    module_info: &ModuleInfo,
    span: SourceSpan,
) -> WasmResult<()> {
    let blockty = BlockType::from_wasm(blockty, module_info)?;
    let next = builder.create_block_with_params(blockty.results.clone(), span);
    state.push_block(next, blockty.params.len(), blockty.results.len());
    Ok(())
}

fn translate_end(
    state: &mut FuncTranslationState,
    builder: &mut FunctionBuilderExt,
    span: SourceSpan,
) {
    // The `End` instruction pops the last control frame from the control stack, seals
    // the destination block (since `br` instructions targeting it only appear inside the
    // block and have already been translated) and modify the value stack to use the
    // possible `Block`'s arguments values.
    let frame = state.control_stack.pop().unwrap();
    let next_block = frame.following_code();
    let return_count = frame.num_return_values();
    let return_args = state.peekn_mut(return_count);

    builder.ins().br(next_block, return_args, span);

    // You might expect that if we just finished an `if` block that
    // didn't have a corresponding `else` block, then we would clean
    // up our duplicate set of parameters that we pushed earlier
    // right here. However, we don't have to explicitly do that,
    // since we truncate the stack back to the original height
    // below.

    builder.switch_to_block(next_block);
    builder.seal_block(next_block);

    // If it is a loop we also have to seal the body loop block
    if let ControlStackFrame::Loop { header, .. } = frame {
        builder.seal_block(header)
    }

    frame.truncate_value_stack_to_original_size(&mut state.stack);
    state
        .stack
        .extend_from_slice(builder.block_params(next_block));
}

fn translate_else(
    state: &mut FuncTranslationState,
    builder: &mut FunctionBuilderExt,
    span: SourceSpan,
) -> WasmResult<()> {
    let i = state.control_stack.len() - 1;
    Ok(match state.control_stack[i] {
        ControlStackFrame::If {
            ref else_data,
            head_is_reachable,
            ref mut consequent_ends_reachable,
            num_return_values,
            ref blocktype,
            destination,
            ..
        } => {
            // We finished the consequent, so record its final
            // reachability state.
            debug_assert!(consequent_ends_reachable.is_none());
            *consequent_ends_reachable = Some(state.reachable);

            if head_is_reachable {
                // We have a branch from the head of the `if` to the `else`.
                state.reachable = true;

                // Ensure we have a block for the `else` block (it may have
                // already been pre-allocated, see `ElseData` for details).
                let else_block = match *else_data {
                    ElseData::NoElse {
                        branch_inst,
                        placeholder,
                    } => {
                        debug_assert_eq!(blocktype.params.len(), num_return_values);
                        let else_block =
                            builder.create_block_with_params(blocktype.params.clone(), span);
                        let params_len = blocktype.params.len();
                        builder.ins().br(destination, state.peekn(params_len), span);
                        state.popn(params_len);

                        builder.change_jump_destination(branch_inst, placeholder, else_block);
                        builder.seal_block(else_block);
                        else_block
                    }
                    ElseData::WithElse { else_block } => {
                        builder
                            .ins()
                            .br(destination, state.peekn(num_return_values), span);
                        state.popn(num_return_values);
                        else_block
                    }
                };

                // You might be expecting that we push the parameters for this
                // `else` block here, something like this:
                //
                //     state.pushn(&control_stack_frame.params);
                //
                // We don't do that because they are already on the top of the stack
                // for us: we pushed the parameters twice when we saw the initial
                // `if` so that we wouldn't have to save the parameters in the
                // `ControlStackFrame` as another `Vec` allocation.

                builder.switch_to_block(else_block);

                // We don't bother updating the control frame's `ElseData`
                // to `WithElse` because nothing else will read it.
            }
        }
        _ => unreachable!(),
    })
}

fn translate_if(
    blockty: &wasmparser::BlockType,
    state: &mut FuncTranslationState,
    builder: &mut FunctionBuilderExt,
    module_info: &ModuleInfo,
    span: SourceSpan,
) -> WasmResult<()> {
    let blockty = BlockType::from_wasm(blockty, module_info)?;
    let cond = state.pop1();
    let zero = builder.ins().i32(0, span);
    let cond_i1 = builder.ins().neq(cond, zero, span);
    let next_block = builder.create_block();
    let (destination, else_data) = if blockty.params.eq(&blockty.results) {
        // It is possible there is no `else` block, so we will only
        // allocate a block for it if/when we find the `else`. For now,
        // we if the condition isn't true, then we jump directly to the
        // destination block following the whole `if...end`. If we do end
        // up discovering an `else`, then we will allocate a block for it
        // and go back and patch the jump.
        let destination = builder.create_block_with_params(blockty.results.clone(), span);
        let branch_inst = builder.ins().cond_br(
            cond_i1,
            next_block,
            &[],
            destination,
            state.peekn(blockty.params.len()),
            span,
        );
        (
            destination,
            ElseData::NoElse {
                branch_inst,
                placeholder: destination,
            },
        )
    } else {
        // The `if` type signature is not valid without an `else` block,
        // so we eagerly allocate the `else` block here.
        let destination = builder.create_block_with_params(blockty.results.clone(), span);
        let else_block = builder.create_block_with_params(blockty.params.clone(), span);
        builder.ins().cond_br(
            cond_i1,
            next_block,
            &[],
            else_block,
            state.peekn(blockty.params.len()),
            span,
        );
        builder.seal_block(else_block);
        (destination, ElseData::WithElse { else_block })
    };
    builder.seal_block(next_block);
    builder.switch_to_block(next_block);
    state.push_if(
        destination,
        else_data,
        blockty.params.len(),
        blockty.results.len(),
        blockty,
    );
    Ok(())
}

fn translate_loop(
    blockty: &wasmparser::BlockType,
    builder: &mut FunctionBuilderExt,
    state: &mut FuncTranslationState,
    module_info: &ModuleInfo,
    span: SourceSpan,
) -> WasmResult<()> {
    let blockty = BlockType::from_wasm(blockty, module_info)?;
    let loop_body = builder.create_block_with_params(blockty.params.clone(), span);
    let next = builder.create_block_with_params(blockty.results.clone(), span);
    builder
        .ins()
        .br(loop_body, state.peekn(blockty.params.len()), span);
    state.push_loop(loop_body, next, blockty.params.len(), blockty.results.len());
    state.popn(blockty.params.len());
    state
        .stack
        .extend_from_slice(builder.block_params(loop_body));
    builder.switch_to_block(loop_body);
    Ok(())
}

/// Deals with a Wasm instruction located in an unreachable portion of the code. Most of them
/// are dropped but special ones like `End` or `Else` signal the potential end of the unreachable
/// portion so the translation state must be updated accordingly.
fn translate_unreachable_operator(
    op: &Operator,
    builder: &mut FunctionBuilderExt,
    state: &mut FuncTranslationState,
    module_info: &ModuleInfo,
    span: SourceSpan,
) -> WasmResult<()> {
    debug_assert!(!state.reachable);
    match *op {
        Operator::If { blockty } => {
            // Push a placeholder control stack entry. The if isn't reachable,
            // so we don't have any branches anywhere.
            let blockty = BlockType::from_wasm(&blockty, module_info)?;
            state.push_if(
                Block::reserved_value(),
                ElseData::NoElse {
                    branch_inst: Inst::reserved_value(),
                    placeholder: Block::reserved_value(),
                },
                0,
                0,
                blockty,
            );
        }
        Operator::Loop { blockty: _ } | Operator::Block { blockty: _ } => {
            state.push_block(Block::reserved_value(), 0, 0);
        }
        Operator::Else => {
            let i = state.control_stack.len() - 1;
            match state.control_stack[i] {
                ControlStackFrame::If {
                    ref else_data,
                    head_is_reachable,
                    ref mut consequent_ends_reachable,
                    ref blocktype,
                    ..
                } => {
                    debug_assert!(consequent_ends_reachable.is_none());
                    *consequent_ends_reachable = Some(state.reachable);

                    if head_is_reachable {
                        // We have a branch from the head of the `if` to the `else`.
                        state.reachable = true;

                        let else_block = match *else_data {
                            ElseData::NoElse {
                                branch_inst,
                                placeholder,
                            } => {
                                let else_block = builder
                                    .create_block_with_params(blocktype.params.clone(), span);
                                let frame = state.control_stack.last().unwrap();
                                frame.truncate_value_stack_to_else_params(&mut state.stack);

                                // We change the target of the branch instruction.
                                builder.change_jump_destination(
                                    branch_inst,
                                    placeholder,
                                    else_block,
                                );
                                builder.seal_block(else_block);
                                else_block
                            }
                            ElseData::WithElse { else_block } => {
                                let frame = state.control_stack.last().unwrap();
                                frame.truncate_value_stack_to_else_params(&mut state.stack);
                                else_block
                            }
                        };

                        builder.switch_to_block(else_block);

                        // Again, no need to push the parameters for the `else`,
                        // since we already did when we saw the original `if`. See
                        // the comment for translating `Operator::Else` in
                        // `translate_operator` for details.
                    }
                }
                _ => unreachable!(),
            }
        }
        Operator::End => {
            let stack = &mut state.stack;
            let control_stack = &mut state.control_stack;
            let frame = control_stack.pop().unwrap();

            // Pop unused parameters from stack.
            frame.truncate_value_stack_to_original_size(stack);

            let reachable_anyway = match frame {
                // If it is a loop we also have to seal the body loop block
                ControlStackFrame::Loop { header, .. } => {
                    builder.seal_block(header);
                    // And loops can't have branches to the end.
                    false
                }
                // If we never set `consequent_ends_reachable` then that means
                // we are finishing the consequent now, and there was no
                // `else`. Whether the following block is reachable depends only
                // on if the head was reachable.
                ControlStackFrame::If {
                    head_is_reachable,
                    consequent_ends_reachable: None,
                    ..
                } => head_is_reachable,
                // Since we are only in this function when in unreachable code,
                // we know that the alternative just ended unreachable. Whether
                // the following block is reachable depends on if the consequent
                // ended reachable or not.
                ControlStackFrame::If {
                    head_is_reachable,
                    consequent_ends_reachable: Some(consequent_ends_reachable),
                    ..
                } => head_is_reachable && consequent_ends_reachable,
                // All other control constructs are already handled.
                _ => false,
            };

            if frame.exit_is_branched_to() || reachable_anyway {
                builder.switch_to_block(frame.following_code());
                builder.seal_block(frame.following_code());

                // And add the return values of the block but only if the next block is reachable
                // (which corresponds to testing if the stack depth is 1)
                stack.extend_from_slice(builder.block_params(frame.following_code()));
                state.reachable = true;
            }
        }
        _ => {
            // We don't translate because this is unreachable code
        }
    }

    Ok(())
}
