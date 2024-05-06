use std::rc::Rc;

use cranelift_entity::SecondaryMap;
use miden_hir::{self as hir, adt::SparseMap, assert_matches};
use miden_hir_analysis::{
    DominatorTree, GlobalVariableLayout, LivenessAnalysis, Loop, LoopAnalysis,
};
use smallvec::SmallVec;

use super::{
    emit::{InstOpEmitter, OpEmitter},
    opt::{OperandMovementConstraintSolver, SolverError},
    scheduler::{BlockInfo, InstInfo, Schedule, ScheduleOp},
    Constraint, OperandStack,
};
use crate::masm::{self, Op};

pub struct FunctionEmitter<'a> {
    f: &'a hir::Function,
    f_prime: &'a mut masm::Function,
    domtree: &'a DominatorTree,
    loops: &'a LoopAnalysis,
    liveness: &'a LivenessAnalysis,
    globals: &'a GlobalVariableLayout,
    visited: SecondaryMap<hir::Block, bool>,
}

struct BlockEmitter<'b, 'f: 'b> {
    function: &'b mut FunctionEmitter<'f>,
    block_infos: &'b SparseMap<hir::Block, Rc<BlockInfo>>,
    block_info: Rc<BlockInfo>,
    /// The "controlling" loop corresponds to the current maximum loop depth
    /// reached along the control flow path reaching this block. When we reach
    /// a loopback edge in the control flow graph, we emit a trailing duplicate of the
    /// instructions in the loop header to which we are branching. The controlling
    /// loop is used during this process to determine what action, if any, must
    /// be taken to exit the current loop in order to reach the target loop.
    ///
    /// Because we expect the IR we process to have been treeified, each block
    /// can only have a single controlling loop, or none. This is because the only
    /// blocks with multiple predecessors are loop headers, where the additional
    /// predecessors must be loopback edges. Since a loopback edge does not modify
    /// the controlling loop of the loop header block, it can only have a single
    /// controlling loop.
    controlling_loop: Option<Loop>,
    stack: OperandStack,
    target: masm::BlockId,
    visited: bool,
}

/// Represents a task to execute during function emission
#[derive(Debug)]
enum Task {
    /// Emit a block into a fresh block
    Block {
        /// The block to emit
        block: hir::Block,
        /// If set, the given loop should be used as the controlling
        /// loop when determining what loop level is being exited
        /// from.
        ///
        /// This gets set on any blocks emitted along a loopback edge
        /// in the control flow graph, and is otherwise None.
        controlling_loop: Option<Loop>,
        /// The state of the operand stack at block entry
        stack: OperandStack,
    },
    /// Emit a block by appending it to an existing block
    Inline {
        /// The block to inline into
        target: masm::BlockId,
        /// The block to emit
        block: hir::Block,
        /// If set, the given loop should be used as the controlling
        /// loop when determining what loop level is being exited
        /// from.
        ///
        /// This gets set on any blocks emitted along a loopback edge
        /// in the control flow graph, and is otherwise None.
        controlling_loop: Option<Loop>,
        /// The state of the operand stack at block entry
        stack: OperandStack,
    },
}

/// The task stack used during function emission
type Tasks = SmallVec<[Task; 4]>;

impl<'a> FunctionEmitter<'a> {
    pub fn new(
        f: &'a hir::Function,
        f_prime: &'a mut masm::Function,
        domtree: &'a DominatorTree,
        loops: &'a LoopAnalysis,
        liveness: &'a LivenessAnalysis,
        globals: &'a GlobalVariableLayout,
    ) -> Self {
        Self {
            f,
            f_prime,
            domtree,
            loops,
            liveness,
            globals,
            visited: SecondaryMap::new(),
        }
    }

    pub fn emit(mut self, schedule: Schedule, stack: OperandStack) {
        let mut tasks = Tasks::from_iter([Task::Block {
            block: self.f.dfg.entry_block(),
            controlling_loop: None,
            stack,
        }]);
        while let Some(task) = tasks.pop() {
            match task {
                Task::Block {
                    block: block_id,
                    controlling_loop,
                    stack,
                } => {
                    let block_info = schedule.block_info(block_id);
                    let target = block_info.target;
                    let block_schedule = schedule.get(block_id);
                    let visited = core::mem::replace(&mut self.visited[block_id], true);
                    let emitter = BlockEmitter {
                        function: &mut self,
                        block_infos: &schedule.block_infos,
                        block_info,
                        controlling_loop,
                        target,
                        stack,
                        visited,
                    };
                    emitter.emit(block_schedule, &mut tasks);
                }
                Task::Inline {
                    target,
                    block: block_id,
                    controlling_loop,
                    stack,
                } => {
                    let block_info = schedule.block_info(block_id);
                    let block_schedule = schedule.get(block_id);
                    let visited = core::mem::replace(&mut self.visited[block_id], true);
                    let emitter = BlockEmitter {
                        function: &mut self,
                        block_infos: &schedule.block_infos,
                        block_info,
                        controlling_loop,
                        target,
                        stack,
                        visited,
                    };
                    emitter.emit(block_schedule, &mut tasks);
                }
            }
        }
    }
}

impl<'b, 'f: 'b> BlockEmitter<'b, 'f> {
    pub fn emit(mut self, block_schedule: &[ScheduleOp], tasks: &mut Tasks) {
        // Before we emit any scheduling operations, compare the current stack
        // against the set of live-in values expected by this block. If there are
        // any values on the stack which are not live-in, then they should be dropped
        // here.
        //
        // This situation occurs when the scheduler deems that a value must
        // be copied for all uses in a given block because it is live in at least one
        // successor, causing the value to be kept on the stack when transferring control
        // to any successor of that block. When this interacts with conditional branches,
        // where the value is only used in a subset of successors, there will be at least
        // one successor where the value is left dangling and essentially never cleaned
        // up. This causes issues with operand stack coherence in loops. We can't avoid
        // making the copy in the original block, instead responsibility for cleaning
        // up these unused values is pushed into the successor on entry.
        self.drop_unused_operands();

        // Continue normally, by emitting the contents of the block based on the given schedule
        for op in block_schedule.iter() {
            match op {
                ScheduleOp::Init(_) | ScheduleOp::Enter(_) | ScheduleOp::Exit => continue,
                ScheduleOp::Inst(inst_info) => self.emit_inst(inst_info, tasks),
                ScheduleOp::Drop(value) => {
                    let mut emitter = self.emitter();
                    let pos = emitter
                        .stack()
                        .find(value)
                        .expect("could not find value on the operand stack");
                    emitter.drop_operand_at_position(pos);
                }
            }
        }
    }

    fn emit_inst(&mut self, inst_info: &InstInfo, tasks: &mut Tasks) {
        use miden_hir::Instruction;

        // Move instruction operands into place, minimizing unnecessary stack manipulation ops
        //
        // NOTE: This does not include block arguments for control flow instructions, those are
        // handled separately within the specific handlers for those instructions
        let args = self.function.f.dfg.inst_args(inst_info.inst);
        self.schedule_operands(args, inst_info.plain_arguments()).unwrap_or_else(|err| {
            panic!("failed to schedule operands for {}: {err:?}", inst_info.inst)
        });

        match self.function.f.dfg.inst(inst_info.inst) {
            ix @ (Instruction::RetImm(_) | Instruction::Ret(_)) => self.emit_ret(inst_info, ix),
            Instruction::Br(ref op) => self.emit_br(inst_info, op, tasks),
            Instruction::CondBr(ref op) => self.emit_cond_br(inst_info, op, tasks),
            Instruction::GlobalValue(op) => self.emit_global_value(inst_info, op),
            Instruction::UnaryOpImm(op) => self.emit_unary_imm_op(inst_info, op),
            Instruction::UnaryOp(op) => self.emit_unary_op(inst_info, op),
            Instruction::BinaryOpImm(op) => self.emit_binary_imm_op(inst_info, op),
            Instruction::BinaryOp(op) => self.emit_binary_op(inst_info, op),
            Instruction::Test(op) => self.emit_test_op(inst_info, op),
            Instruction::Load(op) => self.emit_load_op(inst_info, op),
            Instruction::PrimOp(op) => self.emit_primop(inst_info, op),
            Instruction::PrimOpImm(op) => self.emit_primop_imm(inst_info, op),
            Instruction::Call(op) => self.emit_call_op(inst_info, op),
            Instruction::InlineAsm(op) => self.emit_inline_asm(inst_info, op),
            Instruction::Switch(_) => {
                panic!("expected switch instructions to have been rewritten before stackification")
            }
        }
    }

    fn emit_ret(&mut self, inst_info: &InstInfo, ix: &hir::Instruction) {
        use miden_hir::Instruction;
        assert!(
            !self.visited,
            "invalid control flow graph: unexpected return instruction in loop in {}",
            self.function.f.dfg.inst_block(inst_info.inst).unwrap(),
        );

        let num_args = self.function.f.dfg.inst_args(inst_info.inst).len();
        let level = self.controlling_loop_level().unwrap_or(0);

        let mut emitter = self.emitter();
        // Upon return, the operand stack should only contain the function result(s),
        // so empty the stack before proceeding.
        emitter.truncate_stack(num_args);
        // If this instruction is the immediate variant, we need to push the return
        // value on the stack at this point.
        if let Instruction::RetImm(hir::RetImm { arg, .. }) = ix {
            emitter.literal(*arg);
        }

        // If we're in a loop, push N zeroes on the stack, where N is the current loop depth
        for _ in 0..level {
            emitter.literal(false);
        }
    }

    /// Lower an unconditional branch instruction.
    ///
    /// There are two ways in which code generation lowers these instructions, depending on
    /// whether we have visited the successor block previously, nor not.
    ///
    /// * If this is the first visit to the successor, then due to the transformation passes
    /// we expect to have been run on the input IR (namely treeification and block inlining),
    /// it should be the case that these unconditional branches only exist in the IR when the
    /// current block is a loop header, or the successor is a loop header.
    ///
    /// * If we have visited the successor previously, then we are emitting code for a loopback
    /// edge, and the successor must be a loop header. We must emit the loop header inline in the
    /// current block, up to the terminator, and then emit instructions to either continue the
    /// loop, or exit the current loop to the target loop.
    fn emit_br(&mut self, inst_info: &InstInfo, op: &hir::Br, tasks: &mut Tasks) {
        let destination = op.destination;

        let is_first_visit = !self.visited;
        let in_loop_header = self.block_info.is_loop_header();

        // Move block arguments into position
        let args = op.args.as_slice(&self.function.f.dfg.value_lists);
        self.schedule_operands(args, inst_info.block_arguments(destination))
            .unwrap_or_else(|err| {
                panic!("failed to schedule operands for {}: {err:?}", inst_info.inst)
            });
        // Rename operands on stack to destination block parameters
        let params = self.function.f.dfg.block_params(destination);
        for (idx, param) in params.iter().enumerate() {
            self.stack.rename(idx, *param);
        }

        if is_first_visit {
            let controlling_loop = self.target_controlling_loop(destination);
            if in_loop_header {
                // We're in a loop header, emit the target block inside a while loop
                let body_blk = self.masm_block_id(destination);
                self.emit_ops([Op::PushU8(1), Op::While(body_blk)]);
                tasks.push(Task::Block {
                    block: destination,
                    controlling_loop,
                    stack: self.stack.clone(),
                });
            } else {
                // We're in a normal block, emit the target block inline
                tasks.push(Task::Inline {
                    target: self.target,
                    block: destination,
                    controlling_loop,
                    stack: self.stack.clone(),
                });
            }
        } else {
            // We should only be emitting code for a block more than once if that block
            // is a loop header. All other blocks should only be visited a single time.
            assert!(in_loop_header, "unexpected cycle at {}", self.block_info.source);

            // Calculate
            let current_level = self.controlling_loop_level().unwrap_or_else(|| {
                panic!("expected controlling loop to be set in {}", self.block_info.source)
            });
            let target_level = self.loop_level(self.block_info.source);
            let mut emitter = self.emitter();
            emitter.literal(true);
            for _ in 0..(current_level - target_level) {
                emitter.literal(false);
            }
        }
    }

    fn emit_cond_br(&mut self, inst_info: &InstInfo, op: &hir::CondBr, tasks: &mut Tasks) {
        let cond = op.cond;
        let then_dest = op.then_dest.0;
        let else_dest = op.else_dest.0;

        // Ensure `cond` is on top of the stack, and remove it at the same time
        assert_eq!(
            self.stack.pop().unwrap().as_value(),
            Some(cond),
            "expected {} on top of the stack",
            cond
        );

        if !self.visited {
            let then_blk = self.masm_block_id(then_dest);
            let else_blk = self.masm_block_id(else_dest);

            // If the current block is a loop header, we're emitting a conditional loop,
            // otherwise we're emitting a simple if/else conditional expression.
            if self.block_info.is_loop_header() {
                let body_blk = self.function.f_prime.create_block();
                // We always unconditionally enter the loop the first time
                self.emit_ops([Op::PushU8(1), Op::While(body_blk)]);
                self.emit_op_to(body_blk, Op::If(then_blk, else_blk));
            } else {
                self.emit_op(Op::If(then_blk, else_blk));
            }

            let successors =
                [(then_dest, then_blk, op.then_dest.1), (else_dest, else_blk, op.else_dest.1)];
            for (block, masm_block, args) in successors.into_iter() {
                // Make a copy of the operand stack in the current block
                // to be used as the state of the operand stack in the
                // successor block
                let mut stack = self.stack.clone();

                // Move block arguments for this successor into place, along
                // the control flow edge to that successor, i.e. we do not emit
                // these stack ops in the current block, but in the successor block
                let args = args.as_slice(&self.function.f.dfg.value_lists);
                self.schedule_operands_in_block(
                    args,
                    inst_info.block_arguments(block),
                    masm_block,
                    &mut stack,
                )
                .unwrap_or_else(|err| {
                    panic!(
                        "failed to schedule operands for successor {block} of {}: {err:?}",
                        inst_info.inst
                    )
                });

                // Now that the block arguments are in place, we need to rename
                // the stack operands to use the value names the successor expects
                let params = self.function.f.dfg.block_params(block);
                for (idx, param) in params.iter().enumerate() {
                    stack.rename(idx, *param);
                }

                // Enqueue a task to emit code for the successor block
                let controlling_loop = self.target_controlling_loop(block);
                tasks.push(Task::Block {
                    block,
                    controlling_loop,
                    stack,
                });
            }
        } else {
            // We should only be emitting code for a block more than once if that block
            // is a loop header. All other blocks should only be visited a single time.
            assert!(
                self.block_info.is_loop_header(),
                "unexpected cycle caused by branch to {}",
                self.block_info.source,
            );

            let current_level = self.controlling_loop_level().unwrap_or_else(|| {
                panic!("expected controlling loop to be set in {}", self.block_info.source)
            });
            let target_level = self.loop_level(self.block_info.source);
            // Continue the target loop when it is reached, the top of the stack
            // prior to this push.1 instruction holds the actual conditional, which
            // will be evaluated by the `if.true` nested inside the target `while.true`
            let mut emitter = self.emitter();
            emitter.literal(true);
            for _ in 0..(current_level - target_level) {
                emitter.literal(false);
            }
        }
    }

    fn emit_global_value(&mut self, inst_info: &InstInfo, op: &hir::GlobalValueOp) {
        assert_eq!(op.op, hir::Opcode::GlobalValue);
        let addr = self
            .function
            .globals
            .get_computed_addr(&self.function.f.id, op.global)
            .expect("expected linker to identify all undefined symbols");
        match self.function.f.dfg.global_value(op.global) {
            hir::GlobalValueData::Load { ref ty, .. } => {
                let mut emitter = self.inst_emitter(inst_info.inst);
                emitter.load_imm(addr, ty.clone());
            }
            hir::GlobalValueData::IAddImm { .. } | hir::GlobalValueData::Symbol { .. } => {
                let mut emitter = self.inst_emitter(inst_info.inst);
                emitter.stack_mut().push(addr);
            }
        }
    }

    fn emit_unary_imm_op(&mut self, inst_info: &InstInfo, op: &hir::UnaryOpImm) {
        use miden_hir::Immediate;

        let mut emitter = self.inst_emitter(inst_info.inst);
        match op.op {
            hir::Opcode::ImmI1 => {
                assert_matches!(op.imm, Immediate::I1(_));
                emitter.literal(op.imm);
            }
            hir::Opcode::ImmI8 => {
                assert_matches!(op.imm, Immediate::I8(_));
                emitter.literal(op.imm);
            }
            hir::Opcode::ImmU8 => {
                assert_matches!(op.imm, Immediate::U8(_));
                emitter.literal(op.imm);
            }
            hir::Opcode::ImmI16 => {
                assert_matches!(op.imm, Immediate::I16(_));
                emitter.literal(op.imm);
            }
            hir::Opcode::ImmU16 => {
                assert_matches!(op.imm, Immediate::U16(_));
                emitter.literal(op.imm);
            }
            hir::Opcode::ImmI32 => {
                assert_matches!(op.imm, Immediate::I32(_));
                emitter.literal(op.imm);
            }
            hir::Opcode::ImmU32 => {
                assert_matches!(op.imm, Immediate::U32(_));
                emitter.literal(op.imm);
            }
            hir::Opcode::ImmI64 => {
                assert_matches!(op.imm, Immediate::I64(_));
                emitter.literal(op.imm);
            }
            hir::Opcode::ImmU64 => {
                assert_matches!(op.imm, Immediate::U64(_));
                emitter.literal(op.imm);
            }
            hir::Opcode::ImmFelt => {
                assert_matches!(op.imm, Immediate::Felt(_));
                emitter.literal(op.imm);
            }
            hir::Opcode::ImmF64 => {
                assert_matches!(op.imm, Immediate::F64(_));
                emitter.literal(op.imm);
            }
            opcode => unimplemented!("unrecognized unary with immediate opcode: '{opcode}'"),
        }
    }

    fn emit_unary_op(&mut self, inst_info: &InstInfo, op: &hir::UnaryOp) {
        let inst = inst_info.inst;
        let result = self.function.f.dfg.first_result(inst);
        let mut emitter = self.inst_emitter(inst);
        match op.op {
            hir::Opcode::Neg => emitter.neg(),
            hir::Opcode::Inv => emitter.inv(),
            hir::Opcode::Incr => emitter.incr(),
            hir::Opcode::Ilog2 => emitter.ilog2(),
            hir::Opcode::Pow2 => emitter.pow2(),
            hir::Opcode::Not => emitter.not(),
            hir::Opcode::Bnot => emitter.bnot(),
            hir::Opcode::Popcnt => emitter.popcnt(),
            hir::Opcode::Clz => emitter.clz(),
            hir::Opcode::Ctz => emitter.ctz(),
            hir::Opcode::Clo => emitter.clo(),
            hir::Opcode::Cto => emitter.cto(),
            // This opcode is a no-op
            hir::Opcode::PtrToInt => {
                let result_ty = emitter.value_type(result).clone();
                let stack = emitter.stack_mut();
                stack.pop().expect("operand stack is empty");
                stack.push(result_ty);
            }
            // We lower this cast to an assertion, to ensure the value is a valid pointer
            hir::Opcode::IntToPtr => {
                let ptr_ty = emitter.value_type(result).clone();
                emitter.inttoptr(&ptr_ty);
            }
            // The semantics of cast for now are basically your standard integer coercion rules
            //
            // We may eliminate this in favor of more specific casts in the future
            hir::Opcode::Cast => {
                let dst_ty = emitter.value_type(result).clone();
                emitter.cast(&dst_ty);
            }
            hir::Opcode::Trunc => {
                let dst_ty = emitter.value_type(result).clone();
                emitter.trunc(&dst_ty);
            }
            hir::Opcode::Zext => {
                let dst_ty = emitter.value_type(result).clone();
                emitter.zext(&dst_ty);
            }
            hir::Opcode::Sext => {
                let dst_ty = emitter.value_type(result).clone();
                emitter.sext(&dst_ty);
            }
            hir::Opcode::IsOdd => emitter.is_odd(),
            opcode => unimplemented!("unrecognized unary opcode: '{opcode}'"),
        }
    }

    fn emit_binary_imm_op(&mut self, inst_info: &InstInfo, op: &hir::BinaryOpImm) {
        use miden_hir::Overflow;

        let mut emitter = self.inst_emitter(inst_info.inst);
        let overflow = op.overflow.unwrap_or(Overflow::Checked);
        match op.op {
            hir::Opcode::Eq => emitter.eq_imm(op.imm),
            hir::Opcode::Neq => emitter.neq_imm(op.imm),
            hir::Opcode::Gt => emitter.gt_imm(op.imm),
            hir::Opcode::Gte => emitter.gte_imm(op.imm),
            hir::Opcode::Lt => emitter.lt_imm(op.imm),
            hir::Opcode::Lte => emitter.lte_imm(op.imm),
            hir::Opcode::Add => emitter.add_imm(op.imm, overflow),
            hir::Opcode::Sub => emitter.sub_imm(op.imm, overflow),
            hir::Opcode::Mul => emitter.mul_imm(op.imm, overflow),
            hir::Opcode::Div if overflow.is_checked() => emitter.checked_div_imm(op.imm),
            hir::Opcode::Div => emitter.unchecked_div_imm(op.imm),
            hir::Opcode::Min => emitter.min_imm(op.imm),
            hir::Opcode::Max => emitter.max_imm(op.imm),
            hir::Opcode::Mod if overflow.is_checked() => emitter.checked_mod_imm(op.imm),
            hir::Opcode::Mod => emitter.unchecked_mod_imm(op.imm),
            hir::Opcode::DivMod if overflow.is_checked() => emitter.checked_divmod_imm(op.imm),
            hir::Opcode::DivMod => emitter.unchecked_divmod_imm(op.imm),
            hir::Opcode::Exp => emitter.exp_imm(op.imm),
            hir::Opcode::And => emitter.and_imm(op.imm),
            hir::Opcode::Band => emitter.band_imm(op.imm),
            hir::Opcode::Or => emitter.or_imm(op.imm),
            hir::Opcode::Bor => emitter.bor_imm(op.imm),
            hir::Opcode::Xor => emitter.xor_imm(op.imm),
            hir::Opcode::Bxor => emitter.bxor_imm(op.imm),
            hir::Opcode::Shl => emitter.shl_imm(op.imm),
            hir::Opcode::Shr => emitter.shr_imm(op.imm),
            hir::Opcode::Rotl => emitter.rotl_imm(op.imm),
            hir::Opcode::Rotr => emitter.rotr_imm(op.imm),
            opcode => unimplemented!("unrecognized binary with immediate opcode: '{opcode}'"),
        }
    }

    fn emit_binary_op(&mut self, inst_info: &InstInfo, op: &hir::BinaryOp) {
        use miden_hir::Overflow;

        let mut emitter = self.inst_emitter(inst_info.inst);
        let overflow = op.overflow.unwrap_or(Overflow::Checked);
        match op.op {
            hir::Opcode::Eq => emitter.eq(),
            hir::Opcode::Neq => emitter.neq(),
            hir::Opcode::Gt => emitter.gt(),
            hir::Opcode::Gte => emitter.gte(),
            hir::Opcode::Lt => emitter.lt(),
            hir::Opcode::Lte => emitter.lte(),
            hir::Opcode::Add => emitter.add(overflow),
            hir::Opcode::Sub => emitter.sub(overflow),
            hir::Opcode::Mul => emitter.mul(overflow),
            hir::Opcode::Div if overflow.is_checked() => emitter.checked_div(),
            hir::Opcode::Div => emitter.unchecked_div(),
            hir::Opcode::Min => emitter.min(),
            hir::Opcode::Max => emitter.max(),
            hir::Opcode::Mod if overflow.is_checked() => emitter.checked_mod(),
            hir::Opcode::Mod => emitter.unchecked_mod(),
            hir::Opcode::DivMod if overflow.is_checked() => emitter.checked_divmod(),
            hir::Opcode::DivMod => emitter.unchecked_divmod(),
            hir::Opcode::Exp => emitter.exp(),
            hir::Opcode::And => emitter.and(),
            hir::Opcode::Band => emitter.band(),
            hir::Opcode::Or => emitter.or(),
            hir::Opcode::Bor => emitter.bor(),
            hir::Opcode::Xor => emitter.xor(),
            hir::Opcode::Bxor => emitter.bxor(),
            hir::Opcode::Shl => emitter.shl(),
            hir::Opcode::Shr => emitter.shr(),
            hir::Opcode::Rotl => emitter.rotl(),
            hir::Opcode::Rotr => emitter.rotr(),
            opcode => unimplemented!("unrecognized binary opcode: '{opcode}'"),
        }
    }

    fn emit_test_op(&mut self, _inst_info: &InstInfo, op: &hir::Test) {
        unimplemented!("unrecognized test opcode: '{}'", &op.op);
    }

    fn emit_load_op(&mut self, inst_info: &InstInfo, op: &hir::LoadOp) {
        let mut emitter = self.inst_emitter(inst_info.inst);
        emitter.load(op.ty.clone());
    }

    fn emit_primop_imm(&mut self, inst_info: &InstInfo, op: &hir::PrimOpImm) {
        let args = op.args.as_slice(&self.function.f.dfg.value_lists);
        let mut emitter = self.inst_emitter(inst_info.inst);
        match op.op {
            hir::Opcode::Assert => {
                assert_eq!(args.len(), 1);
                emitter
                    .assert(Some(op.imm.as_u32().expect("invalid assertion error code immediate")));
            }
            hir::Opcode::Assertz => {
                assert_eq!(args.len(), 1);
                emitter.assertz(Some(
                    op.imm.as_u32().expect("invalid assertion error code immediate"),
                ));
            }
            hir::Opcode::AssertEq => {
                emitter.assert_eq_imm(op.imm);
            }
            // Store a value at a constant address
            hir::Opcode::Store => {
                emitter
                    .store_imm(op.imm.as_u32().expect("invalid address immediate: out of range"));
            }
            opcode => unimplemented!("unrecognized primop with immediate opcode: '{opcode}'"),
        }
    }

    fn emit_primop(&mut self, inst_info: &InstInfo, op: &hir::PrimOp) {
        let args = op.args.as_slice(&self.function.f.dfg.value_lists);
        let mut emitter = self.inst_emitter(inst_info.inst);
        match op.op {
            // Pop a value of the given type off the stack and assert it's value is one
            hir::Opcode::Assert => {
                assert_eq!(args.len(), 1);
                emitter.assert(None);
            }
            // Pop a value of the given type off the stack and assert it's value is zero
            hir::Opcode::Assertz => {
                assert_eq!(args.len(), 1);
                emitter.assertz(None);
            }
            // Pop two values of the given type off the stack and assert equality
            hir::Opcode::AssertEq => {
                assert_eq!(args.len(), 2);
                emitter.assert_eq();
            }
            // Allocate a local and push its address on the operand stack
            hir::Opcode::Alloca => {
                assert!(args.is_empty());
                let result = emitter.dfg().first_result(inst_info.inst);
                let ty = emitter.value_type(result).clone();
                emitter.alloca(&ty);
            }
            // Store a value at a given pointer
            hir::Opcode::Store => {
                assert_eq!(args.len(), 2);
                emitter.store();
            }
            // Copy `count * sizeof(ctrl_ty)` bytes from source to destination address
            hir::Opcode::MemCpy => {
                assert_eq!(args.len(), 3);
                emitter.memcpy();
            }
            // Conditionally select between two values
            hir::Opcode::Select => {
                assert_eq!(args.len(), 3);
                emitter.select();
            }
            // This instruction should not be reachable at runtime, so we emit an assertion
            // that will always fail if for some reason it is reached
            hir::Opcode::Unreachable => {
                // assert(false)
                emitter.emit_all(&[Op::PushU32(0), Op::Assert]);
            }
            opcode => unimplemented!("unrecognized primop with immediate opcode: '{opcode}'"),
        }
    }

    fn emit_call_op(&mut self, inst_info: &InstInfo, op: &hir::Call) {
        assert_ne!(op.callee, self.function.f.id, "unexpected recursive call");

        let mut emitter = self.inst_emitter(inst_info.inst);
        match op.op {
            hir::Opcode::Syscall => emitter.syscall(op.callee),
            hir::Opcode::Call => emitter.exec(op.callee),
            opcode => unimplemented!("unrecognized procedure call opcode: '{opcode}'"),
        }
    }

    fn emit_inline_asm(&mut self, inst_info: &InstInfo, op: &hir::InlineAsm) {
        use super::TypedValue;

        // Port over the blocks from the inline assembly chunk, except the body block, which will
        // be inlined at the current block
        let mut mapped = SecondaryMap::<masm::BlockId, masm::BlockId>::new();
        for (inline_blk, _) in op.blocks.iter() {
            if inline_blk == op.body {
                continue;
            }
            let mapped_blk = self.function.f_prime.create_block();
            mapped[inline_blk] = mapped_blk;
        }

        // Inline the body, rewriting any references to other blocks
        let original_body_block = op.body;
        let mapped_body_block = self.masm_block_id(self.block_info.source);
        let mut rewrites = SmallVec::<[(masm::BlockId, masm::BlockId); 4]>::from_iter([(
            original_body_block,
            mapped_body_block,
        )]);
        self.rewrite_inline_assembly_block(op, &mut rewrites, &mapped);

        // Pop arguments, push results
        self.stack.dropn(op.args.len(&self.function.f.dfg.value_lists));
        for result in self.function.f.dfg.inst_results(inst_info.inst).iter().copied().rev() {
            let ty = self.function.f.dfg.value_type(result).clone();
            self.stack.push(TypedValue { value: result, ty });
        }
    }

    fn rewrite_inline_assembly_block(
        &mut self,
        asm: &hir::InlineAsm,
        rewrites: &mut SmallVec<[(masm::BlockId, masm::BlockId); 4]>,
        mapped_blocks: &SecondaryMap<masm::BlockId, masm::BlockId>,
    ) {
        while let Some((prev, new)) = rewrites.pop() {
            for mut op in asm.blocks[prev].ops.iter().cloned() {
                match op {
                    Op::If(ref mut then_blk, ref mut else_blk) => {
                        let prev_then_blk = *then_blk;
                        let prev_else_blk = *else_blk;
                        *then_blk = mapped_blocks[prev_then_blk];
                        *else_blk = mapped_blocks[prev_else_blk];
                        rewrites.push((prev_then_blk, *then_blk));
                        rewrites.push((prev_else_blk, *else_blk));
                    }
                    Op::While(ref mut body_blk) | Op::Repeat(_, ref mut body_blk) => {
                        let prev_body_blk = *body_blk;
                        *body_blk = mapped_blocks[prev_body_blk];
                        rewrites.push((prev_body_blk, *body_blk));
                    }
                    Op::Exec(id) => {
                        self.function.f_prime.register_absolute_invocation_target(
                            miden_assembly::ast::InvokeKind::Exec,
                            id,
                        );
                    }
                    Op::Call(id) => {
                        self.function.f_prime.register_absolute_invocation_target(
                            miden_assembly::ast::InvokeKind::Call,
                            id,
                        );
                    }
                    Op::Syscall(id) => {
                        self.function.f_prime.register_absolute_invocation_target(
                            miden_assembly::ast::InvokeKind::SysCall,
                            id,
                        );
                    }
                    Op::LocAddr(_)
                    | Op::LocLoad(_)
                    | Op::LocLoadw(_)
                    | Op::LocStore(_)
                    | Op::LocStorew(_) => {
                        unimplemented!(
                            "locals are not currently supported in inline assembly blocks"
                        )
                    }
                    _ => (),
                }
                self.function.f_prime.body.block_mut(new).push(op);
            }
        }
    }

    /// Drop the operands on the stack which are no longer live upon entry into
    /// the current block.
    ///
    /// This is intended to be called before scheduling any instructions in the block.
    fn drop_unused_operands(&mut self) {
        // We start by computing the set of unused operands on the stack at this point
        // in the program. We will use the resulting vectors to schedule instructions
        // that will move those operands to the top of the stack to be discarded
        let pp = hir::ProgramPoint::Block(self.block_info.source);
        let mut unused = SmallVec::<[hir::Value; 4]>::default();
        let mut constraints = SmallVec::<[Constraint; 4]>::default();
        for operand in self.stack.iter().rev() {
            let value = operand.as_value().expect("unexpected non-ssa value on stack");
            // If the given value is not live on entry to this block, it should be dropped
            if !self.function.liveness.is_live_at(&value, pp) {
                println!(
                    "should drop {value} at {} (visited={})",
                    self.block_info.source, self.visited
                );
                unused.push(value);
                constraints.push(Constraint::Move);
            }
        }

        // Next, emit the optimal set of moves to get the unused operands to the top
        if !unused.is_empty() {
            // If the number of unused operands is greater than the number
            // of used operands, then we will schedule manually, since this
            // is a pathological use case for the operand scheduler.
            let num_used = self.stack.len() - unused.len();
            if unused.len() > num_used {
                // In this case, we emit code starting from the top
                // of the stack, i.e. if we encounter an unused value
                // on top, then we increment a counter and check the
                // next value, and so on, until we reach a used value
                // or the end of the stack. At that point, we emit drops
                // for the unused batch, and reset the counter.
                //
                // If we encounter a used value on top, or we have dropped
                // an unused batch and left a used value on top, we look
                // to see if the next value is used/unused:
                //
                // * If used, we increment the counter until we reach an
                // unused value or the end of the stack. We then move any
                // unused value found to the top and drop it, subtract 1
                // from the counter, and resume where we left off
                //
                // * If unused, we check if it is just a single unused value,
                // or if there is a string of unused values starting there.
                // In the former case, we swap it to the top of the stack and
                // drop it, and start over. In the latter case, we move the
                // used value on top of the stack down past the last unused
                // value, and then drop the unused batch.
                let mut batch_size = 0;
                let mut current_index = 0;
                let mut unused_batch = false;
                while self.stack.len() > current_index {
                    let value = self.stack[current_index].as_value().unwrap();
                    let is_unused = unused.contains(&value);
                    // If we're looking at the top operand, start
                    // a new batch of either used or unused operands
                    if current_index == 0 {
                        unused_batch = is_unused;
                        current_index += 1;
                        batch_size += 1;
                        continue;
                    }

                    // If we're putting together a batch of unused values,
                    // and the current value is unused, extend the batch
                    if unused_batch && is_unused {
                        batch_size += 1;
                        current_index += 1;
                        continue;
                    }

                    // If we're putting together a batch of unused values,
                    // and the current value is used, drop the unused values
                    // we've found so far, and then reset our cursor to the top
                    if unused_batch {
                        let mut emitter = self.emitter();
                        emitter.dropn(batch_size);
                        batch_size = 0;
                        current_index = 0;
                        continue;
                    }

                    // If we're putting together a batch of used values,
                    // and the current value is used, extend the batch
                    if !is_unused {
                        batch_size += 1;
                        current_index += 1;
                        continue;
                    }

                    // Otherwise, we have found more unused value(s) behind
                    // a batch of used value(s), so we need to determine the
                    // best course of action
                    match batch_size {
                        // If we've only found a single used value so far,
                        // and there is more than two unused values behind it,
                        // then move the used value down the stack and drop the unused.
                        1 => {
                            let unused_chunk_size = self
                                .stack
                                .iter()
                                .rev()
                                .skip(1)
                                .take_while(|o| unused.contains(&o.as_value().unwrap()))
                                .count();
                            let mut emitter = self.emitter();
                            if unused_chunk_size > 1 {
                                emitter.movdn(unused_chunk_size as u8);
                                emitter.dropn(unused_chunk_size);
                            } else {
                                emitter.swap(1);
                                emitter.drop();
                            }
                        }
                        // We've got multiple unused values together, so choose instead
                        // to move the unused value to the top and drop it
                        _ => {
                            let mut emitter = self.emitter();
                            emitter.movup(current_index as u8);
                            emitter.drop();
                        }
                    }
                    batch_size = 0;
                    current_index = 0;
                }
            } else {
                self.schedule_operands(&unused, &constraints).unwrap_or_else(|err| {
                    panic!(
                        "failed to schedule unused operands for {}: {err:?}",
                        self.block_info.source
                    )
                });
                let mut emitter = self.emitter();
                emitter.dropn(unused.len());
            }
        }
    }

    fn schedule_operands(
        &mut self,
        expected: &[hir::Value],
        constraints: &[Constraint],
    ) -> Result<(), SolverError> {
        match OperandMovementConstraintSolver::new(expected, constraints, &self.stack) {
            Ok(solver) => {
                let mut emitter = self.emitter();
                solver.solve_and_apply(&mut emitter)
            }
            Err(SolverError::AlreadySolved) => Ok(()),
            Err(err) => {
                panic!("unexpected error constructing operand movement constraint solver: {err:?}")
            }
        }
    }

    fn schedule_operands_in_block(
        &mut self,
        expected: &[hir::Value],
        constraints: &[Constraint],
        block: masm::BlockId,
        stack: &mut OperandStack,
    ) -> Result<(), SolverError> {
        match OperandMovementConstraintSolver::new(expected, constraints, stack) {
            Ok(solver) => {
                let mut emitter = OpEmitter::new(self.function.f_prime, block, stack);
                solver.solve_and_apply(&mut emitter)
            }
            Err(SolverError::AlreadySolved) => Ok(()),
            Err(err) => {
                panic!("unexpected error constructing operand movement constraint solver: {err:?}")
            }
        }
    }

    fn target_controlling_loop(&self, target_block: hir::Block) -> Option<Loop> {
        use core::cmp::Ordering;

        let is_first_visit = !self.visited;
        let current_block = self.block_info.source;
        let current_loop = self.function.loops.innermost_loop(current_block);
        let target_loop = self.function.loops.innermost_loop(target_block);
        match (current_loop, target_loop) {
            // No loops involved
            (None, None) => {
                assert!(is_first_visit);
                assert_eq!(self.controlling_loop, None);
                None
            }
            // Entering a top-level loop, set the controlling loop
            (None, controlling_loop @ Some(_)) => {
                assert!(is_first_visit);
                assert_eq!(self.controlling_loop, None);
                controlling_loop
            }
            // Escaping a loop
            (Some(_), None) => {
                assert!(is_first_visit);
                // We're emitting a block along an exit edge of a loop, it must be the
                // case here that the source block dominates the target block, so we
                // leave the controlling loop alone, since it will be used to calculate
                // the depth we're exiting from
                assert!(
                    self.function.domtree.dominates(
                        current_block,
                        target_block,
                        &self.function.f.dfg
                    ),
                    "expected {current_block} to dominate {target_block} here"
                );
                assert_matches!(self.controlling_loop, Some(_));
                self.controlling_loop
            }
            (Some(src), Some(dst)) => {
                let src_level = self.function.loops.level(src);
                let dst_level = self.function.loops.level(dst);
                if is_first_visit {
                    // We have not visited the target block before..
                    match src_level.cmp(&dst_level) {
                        // We're emitting a block along an exit edge of a loop, so we
                        // expect that the source block dominates the target block, and
                        // as such we will leave the controlling loop alone as it will
                        // be used to calculate the depth we're exiting to
                        Ordering::Greater => {
                            assert!(
                                self.function.domtree.dominates(
                                    current_block,
                                    target_block,
                                    &self.function.f.dfg
                                ),
                                "expected {current_block} to dominate {target_block} here"
                            );
                            self.controlling_loop
                        }
                        // If we're entering a nested loop, then we need to update the controlling
                        // loop to reflect the loop we've entered
                        Ordering::Less => Some(dst),
                        Ordering::Equal => self.controlling_loop,
                    }
                } else {
                    // We're looping back to the loop header, or a parent loop header,
                    // so leave the controlling loop unmodified, it will be reset by
                    // the emit_inst handling
                    self.controlling_loop
                }
            }
        }
    }

    fn masm_block_id(&self, block: hir::Block) -> masm::BlockId {
        self.block_infos.get(block).unwrap().target
    }

    /// Get a mutable reference to the current block of code in the stack machine IR
    #[inline(always)]
    fn current_block(&mut self) -> &mut masm::Block {
        self.function.f_prime.body.block_mut(self.target)
    }

    /// Get a mutable reference to a specific block of code in the stack machine IR
    #[inline(always)]
    fn block(&mut self, block: masm::BlockId) -> &mut masm::Block {
        self.function.f_prime.body.block_mut(block)
    }

    #[inline]
    fn emit_op(&mut self, op: Op) {
        self.current_block().push(op);
    }

    #[inline]
    fn emit_op_to(&mut self, block: masm::BlockId, op: Op) {
        self.block(block).push(op);
    }

    #[inline]
    fn emit_ops(&mut self, ops: impl IntoIterator<Item = Op>) {
        self.current_block().extend(ops);
    }

    fn controlling_loop_level(&self) -> Option<usize> {
        self.controlling_loop.map(|lp| self.function.loops.level(lp).level())
    }

    fn loop_level(&self, block: hir::Block) -> usize {
        self.function.loops.loop_level(block).level()
    }

    #[inline(always)]
    fn inst_emitter<'short, 'long: 'short>(
        &'long mut self,
        inst: hir::Inst,
    ) -> InstOpEmitter<'short> {
        InstOpEmitter::new(
            self.function.f_prime,
            &self.function.f.dfg,
            inst,
            self.target,
            &mut self.stack,
        )
    }

    #[inline(always)]
    fn emitter<'short, 'long: 'short>(&'long mut self) -> OpEmitter<'short> {
        OpEmitter::new(self.function.f_prime, self.target, &mut self.stack)
    }
}
