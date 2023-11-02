use std::collections::VecDeque;

use cranelift_entity::packed_option::ReservedValue;
use intrusive_collections::UnsafeRef;
use miden_diagnostics::{Severity, SourceSpan, Spanned};
use midenc_session::Session;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::parser::ParseError;
use crate::pass::{AnalysisManager, ConversionError, ConversionPass, ConversionResult};
use crate::{FunctionIdent, Ident, Immediate, Opcode, PassInfo, Type};

use super::*;

/// This pass converts the syntax tree of an HIR module to HIR
#[derive(PassInfo)]
pub struct ConvertAstToHir;
impl ConversionPass for ConvertAstToHir {
    type From = Box<Module>;
    type To = crate::Module;

    fn convert(
        &mut self,
        mut ast: Self::From,
        _analyses: &mut AnalysisManager,
        session: &Session,
    ) -> ConversionResult<Self::To> {
        use std::collections::hash_map::Entry;

        let mut module = if ast.is_kernel {
            crate::Module::new_kernel(ast.name)
        } else {
            crate::Module::new(ast.name)
        };

        let mut is_valid = true;

        // Validate constants
        let (constants_by_id, is_constants_valid) =
            ast.take_and_validate_constants(&session.diagnostics);
        is_valid &= is_constants_valid;

        // Validate globals
        let (globals_by_id, is_global_vars_valid) =
            ast.take_and_validate_globals(&constants_by_id, &session.diagnostics);
        is_valid &= is_global_vars_valid;

        for (_, gv_data) in globals_by_id.into_iter() {
            unsafe {
                module.globals.insert(gv_data.item);
            }
        }

        // Validate imports
        let (imports_by_id, is_externals_valid) =
            ast.take_and_validate_imports(&session.diagnostics);
        is_valid &= is_externals_valid;

        // Validate functions
        let mut functions_by_id = FxHashMap::<Ident, SourceSpan>::default();
        let mut worklist = Vec::with_capacity(ast.functions.len());
        for function in ast.functions.into_iter() {
            match functions_by_id.entry(function.name) {
                Entry::Vacant(entry) => {
                    entry.insert(function.name.span());
                    worklist.push(function);
                }
                Entry::Occupied(entry) => {
                    let prev = *entry.get();
                    session
                        .diagnostics
                        .diagnostic(Severity::Error)
                        .with_message("invalid function declaration")
                        .with_primary_label(
                            function.span(),
                            "a function with the same name has already been declared",
                        )
                        .with_secondary_label(prev, "previously declared here")
                        .emit();
                    is_valid = false;
                }
            }
        }

        let mut functions = crate::FunctionList::default();
        let mut values_by_id = ValuesById::default();
        for mut function in worklist.into_iter() {
            values_by_id.clear();

            let id = FunctionIdent {
                module: module.name,
                function: function.name,
            };

            is_valid &= function.is_declaration_valid(&session.diagnostics);
            let entry = function.blocks[0].id;
            let mut blocks_by_id = match function.populate_block_map(&session.diagnostics) {
                Ok(blocks) => blocks,
                Err(blocks) => {
                    is_valid = false;
                    blocks
                }
            };
            let mut blockq = VecDeque::from([entry]);

            // Build the HIR function
            let mut f = Box::new(crate::Function::new_uninit(id, function.signature));
            // The entry block is always the first in the layout
            f.dfg.entry = entry;
            // Visit each block and build it, but do not yet write to the DataFlowGraph
            let mut visited = FxHashSet::<crate::Block>::default();
            while let Some(block_id) = blockq.pop_front() {
                // Do not visit the same block twice
                if !visited.insert(block_id) {
                    continue;
                }

                let mut block_data = crate::BlockData::new(block_id);
                let block = blocks_by_id.remove(&block_id).unwrap();

                // Ensure block parameters are not yet defined
                for (num, param) in block.params.into_iter().enumerate() {
                    match try_insert_param_value(
                        param.id,
                        param.span(),
                        block.id,
                        num as u16,
                        param.ty,
                        &mut values_by_id,
                        &session.diagnostics,
                    ) {
                        Some(id) => {
                            block_data.params.push(id, &mut f.dfg.value_lists);
                        }
                        None => {
                            is_valid = false;
                            continue;
                        }
                    }
                }

                // Populate the block with instructions from the AST
                for (num, inst) in block.body.into_iter().enumerate() {
                    is_valid &= try_insert_inst(
                        inst,
                        num as u16,
                        &mut block_data,
                        &mut blockq,
                        &blocks_by_id,
                        &visited,
                        &mut values_by_id,
                        &imports_by_id,
                        &mut f,
                        &session.diagnostics,
                    );
                }
                f.dfg.blocks.append(block_id, block_data);
            }

            // Now that all of the blocks have been visited,
            // we can record the value data in the DataFlowGraph
            for (id, data) in core::mem::take(&mut values_by_id).into_iter() {
                let next_id = f.dfg.values.next_key();
                let next_raw_id = next_id.as_u32();
                let raw_id = id.as_u32();
                assert!(
                    raw_id >= next_raw_id,
                    "expected that {id} is always ahead of, or equal to {next_id}"
                );
                if raw_id == next_raw_id {
                    f.dfg.values.push(data.item);
                    continue;
                }

                // If we reach here, we need to insert dummy data until the next
                // key is the one that we have data for
                while raw_id > f.dfg.values.next_key().as_u32() {
                    f.dfg.values.push(crate::ValueData::Inst {
                        ty: Type::Unknown,
                        num: 0,
                        inst: crate::Inst::reserved_value(),
                    });
                }
                assert_eq!(f.dfg.values.push(data.item), id);
            }

            functions.push_back(f);
        }

        module.functions = functions;

        if is_valid {
            Ok(module)
        } else {
            Err(ConversionError::Failed(ParseError::InvalidModule.into()))
        }
    }
}

fn try_insert_inst(
    mut inst: Inst,
    num: u16,
    block_data: &mut crate::BlockData,
    blockq: &mut VecDeque<crate::Block>,
    blocks_by_id: &BlocksById,
    visited_blocks: &FxHashSet<crate::Block>,
    values_by_id: &mut ValuesById,
    imports_by_id: &ImportsById,
    function: &mut crate::Function,
    diagnostics: &DiagnosticsHandler,
) -> bool {
    use crate::{BinaryOp, BinaryOpImm, Instruction, PrimOp, PrimOpImm, UnaryOp, UnaryOpImm};

    let id = function.dfg.insts.alloc_key();
    let span = inst.span;
    let results = core::mem::take(&mut inst.outputs);

    // Translate instruction data from AST representation to HIR
    let data = match inst.ty {
        InstType::BinaryOp {
            opcode: op,
            overflow,
            operands: [Operand::Value(lhs), Operand::Value(rhs)],
        } => Some(Instruction::BinaryOp(BinaryOp {
            op,
            overflow,
            args: [lhs.item, rhs.item],
        })),
        InstType::BinaryOp {
            opcode: op,
            overflow,
            operands: [Operand::Value(lhs), imm],
        } => {
            let imm_ty = values_by_id
                .get(&lhs)
                .map(|v| v.ty().clone())
                .unwrap_or(Type::Unknown);
            operand_to_immediate(imm, &imm_ty, diagnostics).map(|imm| {
                Instruction::BinaryOpImm(BinaryOpImm {
                    op,
                    overflow,
                    arg: lhs.item,
                    imm,
                })
            })
        }
        InstType::BinaryOp {
            operands: [lhs, _], ..
        } => {
            diagnostics
                .diagnostic(Severity::Error)
                .with_message("invalid instruction")
                .with_primary_label(lhs.span(), "unexpected immediate operand")
                .with_secondary_label(
                    span,
                    "only the right-hand operand of a binary operator may be immediate",
                )
                .emit();
            None
        }
        InstType::UnaryOp {
            opcode: op,
            overflow,
            operand: Operand::Value(operand),
        } => Some(Instruction::UnaryOp(UnaryOp {
            op,
            overflow,
            arg: operand.item,
        })),
        InstType::UnaryOp {
            opcode: op,
            overflow,
            operand,
        } => {
            let imm = match op {
                Opcode::ImmI1 => operand_to_immediate(operand, &Type::I1, diagnostics),
                Opcode::ImmI8 => operand_to_immediate(operand, &Type::I8, diagnostics),
                Opcode::ImmU8 => operand_to_immediate(operand, &Type::U8, diagnostics),
                Opcode::ImmI16 => operand_to_immediate(operand, &Type::I16, diagnostics),
                Opcode::ImmU16 => operand_to_immediate(operand, &Type::U16, diagnostics),
                Opcode::ImmI32 => operand_to_immediate(operand, &Type::I32, diagnostics),
                Opcode::ImmU32 => operand_to_immediate(operand, &Type::U32, diagnostics),
                Opcode::ImmI64 => operand_to_immediate(operand, &Type::I64, diagnostics),
                Opcode::ImmU64 => operand_to_immediate(operand, &Type::U64, diagnostics),
                Opcode::ImmF64 => {
                    diagnostics
                        .diagnostic(Severity::Error)
                        .with_message("invalid instruction")
                        .with_primary_label(
                            span,
                            "f64 support is not yet implemented in the parser",
                        )
                        .emit();
                    None
                }
                Opcode::ImmFelt => operand_to_immediate(operand, &Type::Felt, diagnostics),
                _ => {
                    diagnostics
                        .diagnostic(Severity::Error)
                        .with_message("invalid instruction")
                        .with_primary_label(
                            span,
                            "this operation does not support immediate arguments",
                        )
                        .with_secondary_label(operand.span(), "this operand cannot be immediate")
                        .emit();
                    None
                }
            };
            imm.map(|imm| Instruction::UnaryOpImm(UnaryOpImm { op, overflow, imm }))
        }
        InstType::Br {
            opcode: op,
            successor,
        } => {
            match is_valid_successor(
                &successor,
                span,
                blocks_by_id,
                visited_blocks,
                values_by_id,
                diagnostics,
            ) {
                Ok(next) => {
                    if let Some(next) = next {
                        blockq.push_back(next);
                    }
                    let args = crate::ValueList::from_iter(
                        successor.args.iter().map(|arg| arg.item),
                        &mut function.dfg.value_lists,
                    );
                    Some(Instruction::Br(crate::Br {
                        op,
                        destination: successor.id,
                        args,
                    }))
                }
                Err(_) => None,
            }
        }
        InstType::CondBr {
            opcode: op,
            cond,
            then_dest,
            else_dest,
        } => {
            let mut is_valid = is_valid_value_reference(&cond, span, values_by_id, diagnostics);

            match is_valid_successor(
                &then_dest,
                span,
                blocks_by_id,
                visited_blocks,
                values_by_id,
                diagnostics,
            ) {
                Ok(next) => {
                    if let Some(next) = next {
                        blockq.push_back(next);
                    }
                }
                Err(_) => {
                    is_valid = false;
                }
            }

            match is_valid_successor(
                &else_dest,
                span,
                blocks_by_id,
                visited_blocks,
                values_by_id,
                diagnostics,
            ) {
                Ok(next) => {
                    if let Some(next) = next {
                        blockq.push_back(next);
                    }
                }
                Err(_) => {
                    is_valid = false;
                }
            }

            if is_valid {
                let then_args = crate::ValueList::from_iter(
                    then_dest.args.iter().map(|arg| arg.item),
                    &mut function.dfg.value_lists,
                );
                let else_args = crate::ValueList::from_iter(
                    else_dest.args.iter().map(|arg| arg.item),
                    &mut function.dfg.value_lists,
                );
                Some(Instruction::CondBr(crate::CondBr {
                    op,
                    cond: cond.item,
                    then_dest: (then_dest.id, then_args),
                    else_dest: (else_dest.id, else_args),
                }))
            } else {
                None
            }
        }
        InstType::Switch {
            opcode: op,
            input,
            successors,
            fallback,
        } => {
            let mut is_valid = is_valid_value_reference(&input, span, values_by_id, diagnostics);

            let mut used_discriminants = FxHashSet::<Span<u32>>::default();
            let mut arms = Vec::with_capacity(successors.len());
            for arm in successors.into_iter() {
                let arm_span = arm.span();
                let discriminant = arm.item.0;
                if !used_discriminants.insert(Span::new(arm_span, discriminant)) {
                    let prev = used_discriminants
                        .get(&Span::new(SourceSpan::UNKNOWN, discriminant))
                        .unwrap()
                        .span();
                    diagnostics
                        .diagnostic(Severity::Error)
                        .with_message("invalid instruction")
                        .with_primary_label(
                            arm_span,
                            "the discriminant for this switch case has already been used",
                        )
                        .with_secondary_label(prev, "previously used here")
                        .emit();
                    is_valid = false;
                }
                let successor = arm.item.1;
                arms.push((discriminant, successor.id));
                match is_valid_successor(
                    &successor,
                    span,
                    blocks_by_id,
                    visited_blocks,
                    values_by_id,
                    diagnostics,
                ) {
                    Ok(next) => {
                        if let Some(next) = next {
                            blockq.push_back(next);
                        }
                    }
                    Err(_) => {
                        is_valid = false;
                    }
                }
            }
            match is_valid_successor(
                &fallback,
                span,
                blocks_by_id,
                visited_blocks,
                values_by_id,
                diagnostics,
            ) {
                Ok(next) => {
                    if let Some(next) = next {
                        blockq.push_back(next);
                    }
                }
                Err(_) => {
                    is_valid = false;
                }
            }

            if is_valid {
                Some(Instruction::Switch(crate::Switch {
                    op,
                    arg: input.item,
                    arms,
                    default: fallback.id,
                }))
            } else {
                None
            }
        }
        InstType::Ret {
            opcode: op,
            mut operands,
        } => {
            let expected_returns = function.signature.results.len();
            let actual_returns = operands.len();
            if actual_returns != expected_returns {
                diagnostics.diagnostic(Severity::Error)
                    .with_message("invalid instruction")
                    .with_primary_label(span, format!("the current function expects {expected_returns} values to be returned, but {actual_returns} are returned here"))
                    .emit();
                None
            } else {
                match actual_returns {
                    0 => Some(Instruction::Ret(crate::Ret {
                        op,
                        args: Default::default(),
                    })),
                    1 => match operands.pop().unwrap() {
                        Operand::Value(v) => {
                            if is_valid_value_reference(&v, span, values_by_id, diagnostics) {
                                let args = crate::ValueList::from_slice(
                                    &[v.item],
                                    &mut function.dfg.value_lists,
                                );
                                Some(Instruction::Ret(crate::Ret { op, args }))
                            } else {
                                None
                            }
                        }
                        operand @ (Operand::Int(_) | Operand::BigInt(_)) => operand_to_immediate(
                            operand,
                            &function.signature.results[0].ty,
                            diagnostics,
                        )
                        .map(|arg| Instruction::RetImm(crate::RetImm { op, arg })),
                    },
                    _ => {
                        let mut is_valid = true;
                        let mut args = crate::ValueList::new();
                        for operand in operands.iter() {
                            if let Operand::Value(ref v) = operand {
                                args.push(v.item, &mut function.dfg.value_lists);
                                is_valid &=
                                    is_valid_value_reference(v, span, values_by_id, diagnostics);
                            } else {
                                diagnostics
                                    .diagnostic(Severity::Error)
                                    .with_message("invalid operand")
                                    .with_primary_label(
                                        operand.span(),
                                        "expected an ssa value here, but got an immediate",
                                    )
                                    .with_secondary_label(span, "occurs here")
                                    .emit();
                                is_valid = false;
                            }
                        }
                        if is_valid {
                            Some(Instruction::Ret(crate::Ret { op, args }))
                        } else {
                            None
                        }
                    }
                }
            }
        }
        InstType::Call {
            opcode: op,
            callee,
            operands,
        } => {
            let mut is_valid = true;
            if !function.dfg.imports.contains_key(&callee) {
                if let Some(ef) = imports_by_id.get(&callee) {
                    function.dfg.imports.insert(callee, ef.item.clone());
                } else {
                    diagnostics.diagnostic(Severity::Error)
                        .with_message("invalid call instruction")
                        .with_primary_label(callee.span(), "this function is not imported in the current module, you must do so in order to reference it")
                        .with_secondary_label(span, "invalid callee for this instruction")
                        .emit();
                    is_valid = false;
                }
            }

            is_valid &=
                is_valid_value_references(operands.as_slice(), span, values_by_id, diagnostics);

            if is_valid {
                let args = crate::ValueList::from_iter(
                    operands.iter().map(|arg| arg.item),
                    &mut function.dfg.value_lists,
                );
                Some(Instruction::Call(crate::Call { op, callee, args }))
            } else {
                None
            }
        }
        InstType::CallIndirect { .. } => {
            unimplemented!("indirect calls are not implemented in the parser yet")
        }
        InstType::PrimOp {
            opcode: op,
            operands,
        } => {
            if operands.is_empty() {
                Some(Instruction::PrimOp(PrimOp {
                    op,
                    args: Default::default(),
                }))
            } else {
                let mut is_valid = true;
                let mut imm = None;
                let mut args = crate::ValueList::new();
                for (i, operand) in operands.iter().cloned().enumerate() {
                    let is_first = i == 0;
                    match operand {
                        Operand::Value(v) => {
                            is_valid &=
                                is_valid_value_reference(&v, span, values_by_id, diagnostics);
                            args.push(v.item, &mut function.dfg.value_lists);
                        }
                        operand @ (Operand::Int(_) | Operand::BigInt(_)) if is_first => {
                            imm = match op {
                                Opcode::AssertEq => operands[i + 1]
                                    .as_value()
                                    .and_then(|v| values_by_id.get(&v.item).map(|vd| vd.ty()))
                                    .and_then(|ty| operand_to_immediate(operand, ty, diagnostics)),
                                Opcode::Store => {
                                    operand_to_immediate(operand, &Type::U32, diagnostics)
                                }
                                opcode => {
                                    unimplemented!("unsupported primop with immediate {opcode}")
                                }
                            };
                            if imm.is_none() {
                                is_valid = false;
                            }
                        }
                        operand @ (Operand::Int(_) | Operand::BigInt(_)) => {
                            diagnostics.diagnostic(Severity::Error)
                                .with_message("invalid immediate operand")
                                .with_primary_label(operand.span(), "expected an ssa value here, but got an immediate")
                                .with_secondary_label(span, "only the first argument of this instruction may be an immediate")
                                .emit();
                            is_valid = false;
                        }
                    }
                }
                if is_valid {
                    if let Some(imm) = imm {
                        Some(Instruction::PrimOpImm(PrimOpImm { op, imm, args }))
                    } else {
                        Some(Instruction::PrimOp(crate::PrimOp { op, args }))
                    }
                } else {
                    None
                }
            }
        }
        InstType::GlobalValue {
            opcode: _op,
            expr: _,
        } => {
            todo!()
        }
    };

    // If the instruction data is invalid, we still need to handle the instruction results
    // for subsequent instructions which may reference its results
    if let Some(data) = data {
        // Add instruction to DataFlowGraph
        let node = crate::InstNode::new(id, block_data.id, Span::new(span, data));
        function.dfg.insts.append(id, node);

        // Add results to values_by_id map
        for tv in results.into_iter() {
            try_insert_result_value(tv.id, tv.span(), id, num, tv.ty, values_by_id, diagnostics);
        }

        // Append instruction to block
        let unsafe_ref =
            unsafe { UnsafeRef::from_raw(function.dfg.insts.get_raw(id).unwrap().as_ptr()) };
        block_data.append(unsafe_ref);

        true
    } else {
        for tv in results.into_iter() {
            try_insert_result_value(tv.id, tv.span(), id, num, tv.ty, values_by_id, diagnostics);
        }

        false
    }
}

fn is_valid_successor(
    successor: &Successor,
    parent_span: SourceSpan,
    blocks_by_id: &BlocksById,
    visited: &FxHashSet<crate::Block>,
    values_by_id: &ValuesById,
    diagnostics: &DiagnosticsHandler,
) -> Result<Option<crate::Block>, crate::Block> {
    let is_visited = visited.contains(&successor.id);
    let is_valid = is_valid_value_references(
        successor.args.as_slice(),
        parent_span,
        values_by_id,
        diagnostics,
    );
    if blocks_by_id.contains_key(&successor.id) || is_visited {
        if is_valid {
            if is_visited {
                Ok(None)
            } else {
                Ok(Some(successor.id))
            }
        } else {
            Err(successor.id)
        }
    } else {
        diagnostics
            .diagnostic(Severity::Error)
            .with_message("invalid instruction")
            .with_primary_label(
                successor.span,
                "invalid successor: the named block does not exist",
            )
            .with_secondary_label(parent_span, "found in this instruction")
            .emit();
        Err(successor.id)
    }
}

fn is_valid_value_references<'a, I: IntoIterator<Item = &'a Span<crate::Value>>>(
    values: I,
    parent_span: SourceSpan,
    values_by_id: &ValuesById,
    diagnostics: &DiagnosticsHandler,
) -> bool {
    let mut is_valid = false;
    let mut is_empty = true;
    for value in values.into_iter() {
        is_empty = false;
        is_valid &= is_valid_value_reference(value, parent_span, values_by_id, diagnostics);
    }
    is_empty || is_valid
}

fn is_valid_value_reference(
    value: &Span<crate::Value>,
    parent_span: SourceSpan,
    values_by_id: &ValuesById,
    diagnostics: &DiagnosticsHandler,
) -> bool {
    let is_valid = values_by_id.contains_key(&value.item);
    if !is_valid {
        diagnostics
            .diagnostic(Severity::Error)
            .with_message("invalid value operand")
            .with_primary_label(
                value.span(),
                "this value is either undefined, or its definition does not dominate this use",
            )
            .with_secondary_label(parent_span, "invalid use occurs in this instruction")
            .emit();
    }
    is_valid
}

fn try_insert_param_value(
    id: crate::Value,
    span: SourceSpan,
    block: crate::Block,
    num: u16,
    ty: Type,
    values: &mut BTreeMap<crate::Value, Span<crate::ValueData>>,
    diagnostics: &DiagnosticsHandler,
) -> Option<crate::Value> {
    use std::collections::btree_map::Entry;

    match values.entry(id) {
        Entry::Vacant(entry) => {
            let data = crate::ValueData::Param {
                ty,
                num,
                block,
                span,
            };
            entry.insert(Span::new(span, data));
            Some(id)
        }
        Entry::Occupied(entry) => {
            let prev = entry.get().span();
            diagnostics
                .diagnostic(Severity::Error)
                .with_message("invalid block")
                .with_primary_label(span, "the name of this block parameter is already in use")
                .with_secondary_label(prev, "previously declared here")
                .emit();
            None
        }
    }
}

fn try_insert_result_value(
    id: crate::Value,
    span: SourceSpan,
    inst: crate::Inst,
    num: u16,
    ty: Type,
    values: &mut BTreeMap<crate::Value, Span<crate::ValueData>>,
    diagnostics: &DiagnosticsHandler,
) -> Option<crate::Value> {
    use std::collections::btree_map::Entry;

    match values.entry(id) {
        Entry::Vacant(entry) => {
            let data = crate::ValueData::Inst { ty, num, inst };
            entry.insert(Span::new(span, data));
            Some(id)
        }
        Entry::Occupied(entry) => {
            let prev = entry.get().span();
            diagnostics
                .diagnostic(Severity::Error)
                .with_message("invalid instruction")
                .with_primary_label(span, "a value may only be declared a single time")
                .with_secondary_label(prev, "previously declared here")
                .emit();
            None
        }
    }
}

fn operand_to_immediate(
    operand: Operand,
    ty: &Type,
    diagnostics: &DiagnosticsHandler,
) -> Option<Immediate> {
    match operand {
        Operand::Int(i) => smallint_to_immediate(i.span(), i.item, ty, diagnostics),
        Operand::BigInt(i) => bigint_to_immediate(i.span(), i.item, ty, diagnostics),
        Operand::Value(_) => panic!("cannot convert ssa values to immediate"),
    }
}

fn smallint_to_immediate(
    span: SourceSpan,
    i: isize,
    ty: &Type,
    diagnostics: &DiagnosticsHandler,
) -> Option<Immediate> {
    match ty {
        Type::I1 => Some(Immediate::I1(i != 0)),
        Type::I8 => i8::try_from(i).ok().map(Immediate::I8),
        Type::I16 => i16::try_from(i).ok().map(Immediate::I16),
        Type::I32 => i32::try_from(i).ok().map(Immediate::I32),
        Type::U8 | Type::U16 | Type::U32 | Type::U64 | Type::U128 | Type::U256 if i < 0 => None,
        Type::U8 => u8::try_from(i as usize).ok().map(Immediate::U8),
        Type::U16 => u16::try_from(i as usize).ok().map(Immediate::U16),
        Type::U32 => u32::try_from(i as usize).ok().map(Immediate::U32),
        Type::I64 => return Some(Immediate::I64(i as i64)),
        Type::U64 => return Some(Immediate::U64(i as u64)),
        Type::I128 => return Some(Immediate::I128(i as i128)),
        Type::U128 | Type::U256 | Type::F64 => {
            diagnostics
                .diagnostic(Severity::Error)
                .with_message("invalid immediate operand")
                .with_primary_label(
                    span,
                    format!("immediates of type {ty} are not yet supported"),
                )
                .emit();
            None
        }
        ty => {
            diagnostics
                .diagnostic(Severity::Error)
                .with_message("invalid immediate operand")
                .with_primary_label(
                    span,
                    format!("expected an immediate of type {ty}, but got an integer"),
                )
                .emit();
            None
        }
    }
}

fn bigint_to_immediate(
    span: SourceSpan,
    i: num_bigint::BigInt,
    ty: &Type,
    diagnostics: &DiagnosticsHandler,
) -> Option<Immediate> {
    use num_traits::cast::ToPrimitive;

    let is_negative = i.sign() == num_bigint::Sign::Minus;
    // NOTE: If we are calling this function, then `i` was too large to fit in an `isize`, so it must be a large integer type
    let imm = match ty {
        Type::U32 if !is_negative => i.to_u32().map(Immediate::U32),
        Type::I64 => i.to_i64().map(Immediate::I64),
        Type::U64 if !is_negative => i.to_u64().map(Immediate::U64),
        Type::I128 => i.to_i128().map(Immediate::I128),
        Type::U128 | Type::U256 | Type::F64 => {
            diagnostics
                .diagnostic(Severity::Error)
                .with_message("invalid immediate operand")
                .with_primary_label(
                    span,
                    format!("immediates of type {ty} are not yet supported"),
                )
                .emit();
            return None;
        }
        ty if ty.is_integer() => {
            diagnostics.diagnostic(Severity::Error)
                .with_message("invalid immediate operand")
                .with_primary_label(span, format!("expected an immediate of type {ty}, but got {i}, which is out of range for that type"))
                .emit();
            return None;
        }
        ty => {
            diagnostics
                .diagnostic(Severity::Error)
                .with_message("invalid immediate operand")
                .with_primary_label(
                    span,
                    format!("expected an immediate of type {ty}, but got an integer"),
                )
                .emit();
            return None;
        }
    };
    if imm.is_none() {
        diagnostics.diagnostic(Severity::Error)
            .with_message("invalid immediate operand")
            .with_primary_label(span, format!("expected an immediate of type {ty}, but got {i}, which is out of range for that type"))
            .emit();
    }
    imm
}
