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

use crate::environ::{FuncEnvironment, ModuleInfo};
use crate::error::WasmResult;
use crate::func_translation_state::ControlStackFrame;
use crate::func_translation_state::{ElseData, FuncTranslationState};
use crate::function_builder_ext::FunctionBuilderExt;
use crate::ssa::Variable;
use crate::translation_utils::{block_with_params, f64_translation};
use crate::unsupported_diag;
use crate::wasm_types::BlockType;
use miden_diagnostics::{DiagnosticsHandler, SourceSpan};
use miden_ir::cranelift_entity::packed_option::ReservedValue;
use miden_ir::hir::{Block, Inst, InstBuilder, Value};
use miden_ir::types::Type;
use miden_ir::types::Type::*;
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
    environ: &mut FuncEnvironment,
    diagnostics: &DiagnosticsHandler,
    span: SourceSpan,
) -> WasmResult<()> {
    if !state.reachable {
        translate_unreachable_operator(&op, builder, state, environ.mod_info, span)?;
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
        /********************************* Stack misc **************************************/
        Operator::Drop => _ = state.pop1(),
        Operator::Select => {
            // TODO: Missing in Miden IR? Or should be implemented via `CondBr`?
            // let (mut arg1, mut arg2, cond) = state.pop3();
            // if cond is zero returns arg2, else returns arg1
            todo!("Wasm Operator::Select translation is not yet implemented");
        }
        Operator::Nop => {}
        Operator::Unreachable => {
            todo!("Wasm Operator::Unreachable translation is not yet implemented");
            // TODO: halt the program as it reached the point that should never be executed
            // state.reachable = false;
        }
        /***************************** Control flow blocks *********************************/
        Operator::Block { blockty } => {
            translate_block(blockty, builder, state, environ.mod_info, span)?
        }
        Operator::Loop { blockty } => {
            translate_loop(blockty, builder, state, environ.mod_info, span)?
        }
        Operator::If { blockty } => translate_if(blockty, state, builder, environ.mod_info, span)?,
        Operator::Else => translate_else(state, builder, span)?,
        Operator::End => translate_end(state, builder, span),

        /**************************** Branch instructions *********************************/
        Operator::Br { relative_depth } => translate_br(state, relative_depth, builder, span),
        Operator::BrIf { relative_depth } => translate_br_if(*relative_depth, builder, state, span),
        Operator::Return => translate_return(state, builder, diagnostics, span)?,
        /************************************ Calls ****************************************/
        Operator::Call { function_index } => {
            translate_call(state, builder, function_index, environ, span);
        }
        /******************************* Memory management *********************************/
        Operator::MemoryGrow { .. } => {
            // Do nothing and return total Miden memory size
            state.push1(builder.ins().i32(mem_total_pages(), span));
        }
        Operator::MemorySize { .. } => {
            state.push1(builder.ins().i32(mem_total_pages(), span));
        }
        /******************************* Load instructions ***********************************/
        Operator::I32Load8U { memarg } => {
            translate_load_zext(I8, I32, memarg, state, builder, span)
        }
        Operator::I32Load16U { memarg } => {
            translate_load_zext(I16, I32, memarg, state, builder, span)
        }
        Operator::I32Load8S { memarg } => {
            translate_load_sext(I8, I32, memarg, state, builder, span);
        }
        Operator::I32Load16S { memarg } => {
            translate_load_sext(I16, I32, memarg, state, builder, span);
        }
        Operator::I64Load8U { memarg } => {
            translate_load_zext(I8, I64, memarg, state, builder, span)
        }
        Operator::I64Load16U { memarg } => {
            translate_load_zext(I16, I64, memarg, state, builder, span)
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
            translate_load_zext(I32, I64, memarg, state, builder, span)
        }
        Operator::I32Load { memarg } => translate_load(I32, memarg, state, builder, span),
        Operator::I64Load { memarg } => translate_load(I64, memarg, state, builder, span),
        Operator::F64Load { memarg } => translate_load(F64, memarg, state, builder, span),
        /****************************** Store instructions ***********************************/
        Operator::I32Store { memarg } => translate_store(I32, memarg, state, builder, span),
        Operator::I64Store { memarg } => translate_store(I64, memarg, state, builder, span),
        Operator::F64Store { memarg } => translate_store(F64, memarg, state, builder, span),
        Operator::I32Store8 { memarg } | Operator::I64Store8 { memarg } => {
            translate_store(I8, memarg, state, builder, span);
        }
        Operator::I32Store16 { memarg } | Operator::I64Store16 { memarg } => {
            translate_store(I16, memarg, state, builder, span);
        }
        Operator::I64Store32 { memarg } => translate_store(I32, memarg, state, builder, span),
        /****************************** Nullary Operators **********************************/
        Operator::I32Const { value } => state.push1(builder.ins().i32(*value, span)),
        Operator::I64Const { value } => state.push1(builder.ins().i64(*value, span)),
        Operator::F64Const { value } => {
            state.push1(builder.ins().f64(f64_translation(*value), span));
        }

        /******************************* Unary Operators *************************************/
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
        /****************************** Binary Operators ************************************/
        Operator::I32Add | Operator::I64Add => {
            let (arg1, arg2) = state.pop2();
            state.push1(builder.ins().add(arg1, arg2, span));
        }
        Operator::I32And | Operator::I64And => {
            let (arg1, arg2) = state.pop2();
            state.push1(builder.ins().and(arg1, arg2, span));
        }
        Operator::I32Or | Operator::I64Or => {
            let (arg1, arg2) = state.pop2();
            state.push1(builder.ins().or(arg1, arg2, span));
        }
        Operator::I32Sub | Operator::I64Sub => {
            let (arg1, arg2) = state.pop2();
            state.push1(builder.ins().sub(arg1, arg2, span));
        }
        Operator::I32Xor | Operator::I64Xor => {
            let (arg1, arg2) = state.pop2();
            state.push1(builder.ins().xor(arg1, arg2, span));
        }
        Operator::I32Shl | Operator::I64Shl => {
            let (arg1, arg2) = state.pop2();
            state.push1(builder.ins().shl(arg1, arg2, span));
        }
        op => {
            unsupported_diag!(diagnostics, "Wasm op {:?} is not supported", op);
        }
    };
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
    builder: &mut FunctionBuilderExt<'_>,
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
    builder: &mut FunctionBuilderExt<'_>,
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
    builder: &mut FunctionBuilderExt<'_>,
    span: SourceSpan,
) {
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
    builder: &mut FunctionBuilderExt<'_>,
    span: SourceSpan,
) {
    let (addr_int, val) = state.pop2();
    let val_ty = builder.func().dfg.value_type(val);
    let arg = if ptr_ty != val_ty {
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
    let full_addr_int = if memarg.offset != 0 {
        let offset = builder.ins().i32(memarg.offset as i32, span);
        builder.ins().add(addr_int, offset, span)
    } else {
        addr_int
    };
    builder
        .ins()
        .inttoptr(full_addr_int, Type::Ptr(ptr_ty.clone().into()), span)
}

fn translate_call(
    state: &mut FuncTranslationState,
    builder: &mut FunctionBuilderExt<'_>,
    function_index: &u32,
    environ: &mut FuncEnvironment<'_>,
    span: SourceSpan,
) {
    let (fref, num_args) = state.get_direct_func(builder.inner.func, *function_index, environ);
    let args = state.peekn_mut(num_args);
    let call = builder.ins().call(fref, &args, span);
    let inst_results = builder.inner.inst_results(call);
    state.popn(num_args);
    state.pushn(inst_results);
}

fn translate_return(
    state: &mut FuncTranslationState,
    builder: &mut FunctionBuilderExt<'_>,
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
    builder: &mut FunctionBuilderExt<'_>,
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
    builder
        .ins()
        .cond_br(cond, then_dest, then_args, else_dest, else_args, span);
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
    builder: &mut FunctionBuilderExt<'_>,
    state: &mut FuncTranslationState,
    module_info: &ModuleInfo,
    span: SourceSpan,
) -> WasmResult<()> {
    let blockty = BlockType::from_wasm(blockty, module_info)?;
    let next = block_with_params(builder, blockty.results.clone(), span)?;
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
                            block_with_params(builder, blocktype.params.clone(), span)?;
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
    builder: &mut FunctionBuilderExt<'_>,
    module_info: &ModuleInfo,
    span: SourceSpan,
) -> WasmResult<()> {
    let blockty = BlockType::from_wasm(blockty, module_info)?;
    let val = state.pop1();
    let next_block = builder.create_block();
    let (destination, else_data) = if blockty.params.eq(&blockty.results) {
        // It is possible there is no `else` block, so we will only
        // allocate a block for it if/when we find the `else`. For now,
        // we if the condition isn't true, then we jump directly to the
        // destination block following the whole `if...end`. If we do end
        // up discovering an `else`, then we will allocate a block for it
        // and go back and patch the jump.
        let destination = block_with_params(builder, blockty.results.clone(), span)?;
        let branch_inst = builder.ins().cond_br(
            val,
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
        let destination = block_with_params(builder, blockty.results.clone(), span)?;
        let else_block = block_with_params(builder, blockty.params.clone(), span)?;
        builder.ins().cond_br(
            val,
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
    builder: &mut FunctionBuilderExt<'_>,
    state: &mut FuncTranslationState,
    module_info: &ModuleInfo,
    span: SourceSpan,
) -> WasmResult<()> {
    let blockty = BlockType::from_wasm(blockty, module_info)?;
    let loop_body = block_with_params(builder, blockty.params.clone(), span)?;
    let next = block_with_params(builder, blockty.results.clone(), span)?;
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
                                let else_block =
                                    block_with_params(builder, blocktype.params.clone(), span)?;
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
