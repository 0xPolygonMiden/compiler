use std::cmp::Ordering;
use std::collections::VecDeque;
use std::rc::Rc;

use cranelift_entity::SecondaryMap;
use miden_hir::{
    self as hir,
    adt::{SmallMap, SmallSet, SparseMap, SparseMapValue},
    assert_matches, BranchInfo, ProgramPoint,
};
use miden_hir_analysis::{
    dependency_graph::{ArgumentNode, DependencyGraph, Node, NodeId},
    DominatorTree, LivenessAnalysis, Loop, LoopAnalysis, OrderedTreeGraph,
};

use smallvec::SmallVec;

use crate::codegen::Constraint;
use crate::masm;

/// Information about a block's successor
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Successor {
    pub block: hir::Block,
    pub argc: u16,
}

#[derive(Debug)]
pub struct ValueInfo {
    /// The value in question
    pub value: hir::Value,
    /// The node corresponding to this value (i.e. a Result or Stack node)
    pub node: NodeId,
    /// If the value is not explicitly used, but is live after the current
    /// block, it is considered used for purposes of dead code analysis
    pub is_externally_used: bool,
    /// The instruction nodes in the dependency graph which use this value.
    ///
    /// This vector is sorted such that the earlier a user appears in it,
    /// the later they are scheduled in the block.
    users: SmallVec<[NodeId; 1]>,
}
impl ValueInfo {
    pub fn is_used(&self) -> bool {
        self.is_externally_used || !self.users.is_empty()
    }

    /// Return the [NodeId] of the first user of this value to be emitted
    pub fn first_user(&self) -> Option<NodeId> {
        self.users.last().copied()
    }

    /// Return the [NodeId] of the last user of this value to be emitted
    #[allow(unused)]
    pub fn last_user(&self) -> Option<NodeId> {
        self.users.first().copied()
    }
}

/// Represents important metadata about an instruction used
/// to schedule its execution and data/control dependencies.
#[derive(Debug)]
pub struct InstInfo {
    /// The id of the instruction
    pub inst: hir::Inst,
    /// The node id of this instruction
    pub node: NodeId,
    /// The number of plain arguments this instruction expects
    pub arity: u8,
    /// Both plain arguments and block arguments are stored in this
    /// vector; plain arguments always come first, and block arguments
    /// start immediately after the last plain argument.
    ///
    /// The arguments for each block are stored consecutively based on
    /// the order of the successors in the instruction. So if there is
    /// one plain argument, and two blocks with two arguments each, then
    /// the layout of all arguments will be:
    ///
    /// ```text,ignore
    /// [plain, block0_arg0, block0_arg1, block1_arg0, block1_arg1]
    /// ```
    pub args: SmallVec<[Constraint; 4]>,
    /// Information about the values produced by this instruction as results
    ///
    /// This vector is sorted by the first use of each result, such that the
    /// earlier a value appears in it, the earlier that value is used in
    /// the scheduled block.
    pub results: SmallVec<[ValueInfo; 2]>,
    /// The set of dependencies which must be scheduled before
    /// the instruction starts executing
    ///
    /// This set is populated in argument order
    pub pre: SmallSet<NodeId, 4>,
    /// The set of dependencies which must be scheduled before
    /// the instruction finishes executing.
    ///
    /// This is largely relevant only for control flow instructions,
    /// particularly those such as `cond_br`, which may push down
    /// materialization of certain data dependencies into the control
    /// flow edge itself, rather than before the instruction starts
    /// execution. This can avoid unnecessarily executing instructions
    /// that aren't ultimately used due to a runtime condition.
    pub post: SmallSet<NodeId, 4>,
    /// The successor blocks and argument count for this instruction
    pub successors: SmallVec<[Successor; 2]>,
}
impl SparseMapValue<hir::Inst> for InstInfo {
    fn key(&self) -> hir::Inst {
        self.inst
    }
}
impl InstInfo {
    #[inline]
    pub const fn arity(&self) -> usize {
        self.arity as usize
    }

    /// Get the constraints for the plain arguments of this instruction
    pub fn plain_arguments(&self) -> &[Constraint] {
        &self.args[..self.arity()]
    }

    /// Get the constraints for the arguments of successor `block`
    pub fn block_arguments(&self, block: hir::Block) -> &[Constraint] {
        let range = self.block_argv_range(block);
        &self.args[range]
    }

    fn block_argv_range(&self, block: hir::Block) -> core::ops::Range<usize> {
        let mut start_idx = self.arity();
        let mut argc = 0;
        for successor in self.successors.iter() {
            if successor.block == block {
                argc = successor.argc as usize;
                break;
            }
            start_idx += successor.argc as usize;
        }
        start_idx..(start_idx + argc)
    }
}

/// [BlockInfo] describes information about a block relevant to scheduling
/// and code generation. Namely, it provides access to the computed dependency
/// graph and tree graph used to schedule the block; but it also provides
/// convenient access to commonly-queried block information.
#[derive(Debug)]
pub struct BlockInfo {
    /// The source HIR block this info is based on
    pub source: hir::Block,
    /// The target MASM block which will be emitted from this info
    pub target: masm::BlockId,
    /// The id of the last instruction in the source HIR block,
    /// this is commonly used to check for liveness after the end
    /// of a block
    pub last_inst: hir::Inst,
    /// The innermost loop to which this block belongs
    pub innermost_loop: Option<Loop>,
    /// If set, indicates that this block is the loop header
    /// for the specified loop.
    pub loop_header: Option<Loop>,
    /// The dependency graph of this block
    pub depgraph: DependencyGraph,
    /// The topologically-ordered tree graph of this block
    pub treegraph: OrderedTreeGraph,
}
impl BlockInfo {
    #[inline(always)]
    pub fn is_loop_header(&self) -> bool {
        self.loop_header.is_some()
    }
}
impl SparseMapValue<hir::Block> for BlockInfo {
    fn key(&self) -> hir::Block {
        self.source
    }
}

/// [Schedule] describes an instruction scheduling plan for a single IR function.
///
/// The plan describes the order in which blocks will be scheduled,
/// and the schedule for instructions in each block.
#[derive(Debug)]
pub struct Schedule {
    pub schedule: SmallVec<[hir::Block; 4]>,
    pub block_infos: SparseMap<hir::Block, Rc<BlockInfo>>,
    pub block_schedules: SecondaryMap<hir::Block, Vec<ScheduleOp>>,
}
impl Schedule {
    pub fn new() -> Self {
        Self {
            schedule: Default::default(),
            block_infos: Default::default(),
            block_schedules: SecondaryMap::new(),
        }
    }

    #[inline]
    pub fn block_info(&self, block: hir::Block) -> Rc<BlockInfo> {
        self.block_infos.get(block).cloned().unwrap()
    }

    #[inline]
    pub fn get(&self, block: hir::Block) -> &[ScheduleOp] {
        self.block_schedules[block].as_slice()
    }
}

/// [ScheduleOp] describes an action to be peformed by the code generator.
///
/// Code generation is driven by constructing a schedule consisting of these operations
/// in a specific order that represents a valid execution of the original IR, and
/// then traversing the schedule start-to-finish, translating these high-level operations
/// into equivalent Miden Assembly code. By performing code generation in this manner,
/// we are able to see a high-level description of how the compiler plans to generate
/// code for a function, and we also enable the code generator to make last-minute
/// optimizations based on the local context of each [ScheduleOp], much like a peephole
/// optimizer would.
#[derive(Debug, Clone)]
pub enum ScheduleOp {
    /// Always the first instruction in a schedule, represents entry into a function
    Init(hir::Block),
    /// Push the current block context on the context stack, and switch to the given block context
    Enter(hir::Block),
    /// Pop the most recent block context from the context stack and switch to it
    Exit,
    /// Emit the given instruction, using the provided analysis
    Inst(Rc<InstInfo>),
    /// Drop the first occurrance of the given value on the operand stack
    Drop(hir::Value),
}

/// Meta-instruction for the abstract scheduling machine used
/// to emit an instruction schedule for a function.
#[derive(Debug)]
pub enum Plan {
    /// Start a new block context
    ///
    /// This represents entering a block, so all further instructions
    /// are scheduled in the context of the given block until an ExitBlock
    /// meta-instruction is encountered.
    Start(hir::Block),
    /// Schedule execution of an instruction's pre-requisites
    PreInst(Rc<InstInfo>),
    /// Schedule execution of the given instruction
    Inst(Rc<InstInfo>),
    /// Schedule execution of any instruction cleanup
    ///
    /// This is primarily intended to support things like pushing materialization
    /// of data dependencies down along control flow edges when they are conditionally
    /// required, but can also be used to schedule instruction cleanup/etc.
    PostInst(Rc<InstInfo>),
    /// Schedule an unused instruction result to be dropped from the operand stack
    Drop(hir::Value),
    /// Indicate that the current block context should be popped for the previous
    /// one on the block stack
    Finish,
}

pub struct Scheduler<'a> {
    f: &'a hir::Function,
    f_prime: &'a mut masm::Function,
    domtree: &'a DominatorTree,
    loops: &'a LoopAnalysis,
    liveness: &'a LivenessAnalysis,
    schedule: Schedule,
}
impl<'a> Scheduler<'a> {
    pub fn new(
        f: &'a hir::Function,
        f_prime: &'a mut masm::Function,
        domtree: &'a DominatorTree,
        loops: &'a LoopAnalysis,
        liveness: &'a LivenessAnalysis,
    ) -> Self {
        Self {
            f,
            f_prime,
            domtree,
            loops,
            liveness,
            schedule: Schedule::new(),
        }
    }

    pub fn build(mut self) -> Schedule {
        self.precompute_block_infos();

        let entry_block_id = self.f.dfg.entry_block();
        let mut blockq = SmallVec::<[hir::Block; 8]>::from_slice(self.domtree.cfg_postorder());
        while let Some(block_id) = blockq.pop() {
            let is_entry_block = block_id == entry_block_id;
            let schedule = &mut self.schedule.block_schedules[block_id];
            if is_entry_block {
                schedule.push(ScheduleOp::Init(block_id));
            } else {
                schedule.push(ScheduleOp::Enter(block_id));
            }

            let block_info = self.schedule.block_infos.get(block_id).cloned().unwrap();
            let block_scheduler = BlockScheduler {
                f: self.f,
                liveness: self.liveness,
                schedule,
                block_info,
                inst_infos: Default::default(),
                worklist: SmallVec::from_iter([Plan::Start(block_id)]),
            };
            block_scheduler.schedule();
        }

        self.schedule
    }

    fn precompute_block_infos(&mut self) {
        let entry_block_id = self.f.dfg.entry_block();

        for block_id in self.domtree.cfg_postorder().iter().rev().copied() {
            // Ensure we have a target block for each source IR block being scheduled
            let masm_block_id = if block_id == entry_block_id {
                self.f_prime.body.id()
            } else {
                self.f_prime.create_block()
            };

            // Set the controlling loop
            let loop_header = self.loops.is_loop_header(block_id);
            let last_inst = self.f.dfg.last_inst(block_id).unwrap();
            let innermost_loop = self.loops.innermost_loop(block_id);
            let depgraph = build_dependency_graph(block_id, self.f, self.liveness);
            let treegraph = OrderedTreeGraph::new(&depgraph)
                .expect("unable to topologically sort treegraph for block");

            let info = Rc::new(BlockInfo {
                source: block_id,
                target: masm_block_id,
                last_inst,
                innermost_loop,
                loop_header,
                depgraph,
                treegraph,
            });

            self.schedule.block_infos.insert(info);
        }
    }
}

struct BlockScheduler<'a> {
    f: &'a hir::Function,
    liveness: &'a LivenessAnalysis,
    schedule: &'a mut Vec<ScheduleOp>,
    block_info: Rc<BlockInfo>,
    inst_infos: SparseMap<hir::Inst, Rc<InstInfo>>,
    worklist: SmallVec<[Plan; 4]>,
}
impl<'a> BlockScheduler<'a> {
    pub fn schedule(mut self) {
        // Planning tasks are added to the worklist in reverse, e.g. we push
        // Plan::Finish before Plan::Start. So by popping tasks off the stack
        // here, we will emit scheduling operations in "normal" order.
        while let Some(plan) = self.worklist.pop() {
            match plan {
                Plan::Start(_) => self.visit_block(),
                Plan::Finish => {
                    self.schedule.push(ScheduleOp::Exit);
                }
                // We're emitting code required to execute an instruction, such as materialization of
                // data dependencies used as direct arguments. This is only emitted when an instruction
                // has arguments which are derived from the results of an instruction that has not been
                // scheduled yet
                Plan::PreInst(inst_info) => self.schedule_pre_inst(inst_info),
                // We're emitting code for an instruction whose pre-requisite dependencies are already
                // materialized, so we need only worry about how a specific instruction is lowered.
                Plan::Inst(inst_info) => self.schedule_inst(inst_info),
                // We're emitting code for an instruction that has started executing, and in some specific
                // cases, may have dependencies which have been deferred until this point. This is only emitted
                // currently for block arguments which are conditionally materialized
                Plan::PostInst(inst_info) => self.schedule_post_inst(inst_info),
                Plan::Drop(value) => {
                    self.schedule.push(ScheduleOp::Drop(value));
                }
            }
        }
    }

    /// Visit the current block and enqueue planning operations to drive scheduling
    fn visit_block(&mut self) {
        // When all scheduling meta-ops are complete for this block, this instruction
        // will switch context back to the parent block in the stack
        self.worklist.push(Plan::Finish);

        // The treegraph iterator visits each node before any of that node's successors,
        // i.e. dependencies. As a result, all code that emits planning tasks (i.e. Plan),
        // must be written in the order opposite of the resulting scheduling tasks (i.e. ScheduleOp).
        //
        // This is critical to internalize, because reasoning about dependencies and when things
        // are live/dead is _inverted_ here. You must ensure that when planning things based on
        // dependency order or liveness, that you do so taking this workflow inversion into account.
        //
        // The actual scheduling tasks are handled in the proper program execution order, but we
        // have to handle this inversion somewhere, and planning seemed like the best place, since
        // it is relatively limited in size and scope.
        //
        // The goal here is to produce a stack of scheduling instructions that when evaluated,
        // will emit code in the correct program order, i.e. instructions which produce results
        // used by another instruction will be emitted so that those results are available on
        // the operand stack when we lower the instruction, and we need only copy/move those
        // operands into place.
        let current_block_info = self.block_info.clone();
        for node_id in current_block_info.treegraph.iter() {
            match node_id.into() {
                // Result nodes are treegraph roots by two paths:
                //
                // * The result is used multiple times. Here, in the planning phase, we must have just
                // finished visiting all of its dependents, so to ensure that during scheduling the
                // result is materialized before its first use, we must do so here. If this result is
                // one of many produced by the same instruction, we must determine if this is the first
                // result to be seen, or the last. The last result to be visited during planning is the
                // one that will actually materialize all of the results for the corresponding instruction.
                // Thus, if this is not the last result visited.
                //
                // * The result is never used, in which case, like above, we must determine if this result
                // should force materialization of the instruction. The caveat here is that if this is the
                // only result produced by its instruction, and it is not live after the end of the current
                // block, then we will avoid materializing at all if the instruction has no side effects.
                // If it _does_ have side effects, then we will force materialization of the result, but
                // then schedule it to be immediately dropped
                Node::Result { value, .. } => {
                    self.maybe_force_materialize_inst_results(value, node_id)
                }
                // During the planning phase, there is only one useful thing to do with this node type,
                // which is to determine if it has any dependents in the current block, and if not,
                // schedule a drop of the value if liveness analysis tells us that the value is not
                // used after the current block.
                Node::Stack(value) => {
                    // If this value is live after the end of this block, it cannot be dropped
                    if self
                        .liveness
                        .is_live_after(&value, ProgramPoint::Block(current_block_info.source))
                    {
                        continue;
                    }
                    // If this value is used within this block, then the last use will consume it
                    if current_block_info.treegraph.num_predecessors(node_id) > 0 {
                        continue;
                    }
                    // Otherwise, we must ensure it gets dropped, so do so immediately
                    self.worklist.push(Plan::Drop(value));
                }
                // We will only ever observe the instruction node type as a treegraph root
                // when it has no results (and thus no dependents/predecessors in the graph),
                // because in all other cases it will always have a predecessor of Result type.
                //
                // In practice, we only observe these nodes when handling block terminators.
                Node::Inst { id: inst, .. } => {
                    let inst_info = self.get_or_analyze_inst_info(inst, node_id);
                    self.plan_inst(inst_info);
                }
                // It can never be the case that argument nodes are unused or multiply-used,
                // they will always be successors of an Inst node
                Node::Argument(_) => unreachable!(),
            }
        }
    }

    fn plan_inst(&mut self, inst_info: Rc<InstInfo>) {
        // Only push an item for post-inst scheduling if we have something to do
        if !inst_info.post.is_empty() {
            // This meta-op will emit code that is required along the control flow edge
            // represented by the branch.
            self.worklist.push(Plan::PostInst(inst_info.clone()));
        }
        // Only push an item for pre-inst scheduling if we have something to do
        if inst_info.pre.is_empty() {
            // This meta-op will emit code for the unconditional branch
            self.worklist.push(Plan::Inst(inst_info));
        } else {
            // This meta-op will emit code for the unconditional branch
            self.worklist.push(Plan::Inst(inst_info.clone()));
            // This meta-op will emit code that is required to evaluate the instruction
            //
            // It is also responsible for scheduling dependencies
            self.worklist.push(Plan::PreInst(inst_info));
        }
    }

    /// Schedule pre-requisites for an instruction based on the given analysis, see [Plan::PreInst] docs for more.
    ///
    /// This function, while nominally occuring during the scheduling phase, actually emits planning
    /// tasks which are processed _before_ the [Plan::Inst] task corresponding to `inst_info`. As a
    /// result, we visit the pre-requisites in planning order, _not_ execution order.
    fn schedule_pre_inst(&mut self, inst_info: Rc<InstInfo>) {
        // Schedule dependencies for execution in the order that they must execute
        if inst_info
            .pre
            .as_slice()
            .is_sorted_by(|a, b| Some(self.block_info.treegraph.cmp_scheduling(*a, *b)))
        {
            self.schedule_inst_dependencies(&inst_info, inst_info.pre.as_slice());
        } else {
            let mut deps = inst_info.pre.clone().into_vec();
            deps.sort_by(|a, b| self.block_info.treegraph.cmp_scheduling(*a, *b));
            self.schedule_inst_dependencies(&inst_info, deps.as_slice());
        }
    }

    fn schedule_inst_dependencies(&mut self, inst_info: &InstInfo, dependencies: &[NodeId]) {
        for dependency_id in dependencies.iter().copied() {
            // If the dependency is a treegraph root, we do not need to do anything,
            // as that dependency will be scheduled at a more appropriate time prior
            // the code generated here, and the dependency will be available on the
            // operand stack
            if self.block_info.treegraph.is_root(dependency_id) {
                continue;
            }

            // Otherwise, dispatch based on the node type of the dependency
            match dependency_id.into() {
                // We are the only user of this result, otherwise it would be a
                // treegraph root. As a result, we simply need to determine whether
                // or not the instruction which produces it must be materialized by
                // us, or via another result
                Node::Result { value, .. } => {
                    self.maybe_materialize_inst_results(value, dependency_id, inst_info)
                }
                // We avoid adding these nodes as pre-requisites as they are assumed to
                // be on the operand stack already, but we handle it gracefully here anyway
                Node::Stack(_) => continue,
                // This is a control dependency, so it must be materialized
                //
                // Currently, control dependencies are only attached to a single instruction,
                // so we do not need to check if another dependent will materialize it, it is
                // definitely on us.
                Node::Inst { id: inst, .. } => {
                    let inst_info = self.get_or_analyze_inst_info(inst, dependency_id);
                    self.materialize_inst_results(inst_info);
                }
                // This node type is never added as a pre-requisite
                Node::Argument(_) => unreachable!(),
            }
        }
    }

    /// Schedule execution of a given instruction, see [Plan::Inst] docs for specific semantics.
    fn schedule_inst(&mut self, inst_info: Rc<InstInfo>) {
        self.schedule.push(ScheduleOp::Inst(inst_info));
    }

    /// Schedule instructions which were deferred until after an instruction executes.
    ///
    /// See [Plan::PostInst] docs for more.
    fn schedule_post_inst(&mut self, inst_info: Rc<InstInfo>) {
        todo!("post-execution instruction dependencies are not supported yet: {inst_info:?}");
    }

    /// We are visiting a `Node::Result` during planning, and need to determine whether or
    /// not to materialize the result at this point, or if we must defer to another result of
    /// the same instruction which is visited later in planning.
    ///
    /// The name of this function refers to the fact that we may have to "force" materialization
    /// of an instruction when the Result node has no predecessors in the graph, but is either
    /// live _after_ the current block, or its instruction has side effects that require it to
    /// be materialized anyway.
    fn maybe_force_materialize_inst_results(&mut self, result: hir::Value, result_node: NodeId) {
        let inst_node = self.block_info.depgraph.unwrap_child(result_node);
        let inst = inst_node.unwrap_inst();
        debug_assert_eq!(inst, self.f.dfg.value_data(result).unwrap_inst());
        let inst_info = self.get_or_analyze_inst_info(inst, inst_node);

        // Do not force materialization because the first result scheduled is responsible for that
        let is_first_result_used = inst_info.results.first().unwrap().node == result_node;
        if !is_first_result_used {
            return;
        }

        // We're the first result scheduled, whether used or not. If the result is
        // not actually used, we may need to force materialization if the following
        // two conditions hold:
        //
        // * There are no results used
        // * The instruction has side effects
        //
        // However, if any of the results are used, we must materialize them now, since
        // we are scheduled before any of the others.
        let is_used = inst_info.results.iter().any(|v| v.is_used());
        let has_side_effects = self.f.dfg.inst(inst).has_side_effects();

        if is_used || has_side_effects {
            self.materialize_inst_results(inst_info);
        }
    }

    fn maybe_materialize_inst_results(
        &mut self,
        result: hir::Value,
        result_node: NodeId,
        dependent_info: &InstInfo,
    ) {
        let inst_node = self.block_info.depgraph.unwrap_child(result_node);
        let inst = inst_node.unwrap_inst();
        debug_assert_eq!(inst, self.f.dfg.value_data(result).unwrap_inst());
        let inst_info = self.get_or_analyze_inst_info(inst, inst_node);

        // We must materialize the instruction the first time it is referenced
        let is_first_result_used = inst_info.results.first().unwrap().node == result_node;
        // If this is not the first result of the referenced instruction to be
        // scheduled, then the instruction results are already materialized
        if !is_first_result_used {
            return;
        }

        // If this result is the first one to be used, but we are not the first user,
        // we also do nothing, since the first user materializes
        let first_user = inst_info.results[0].first_user().unwrap();
        let is_first_use = first_user == dependent_info.node;
        if !is_first_use {
            return;
        }

        // We're the first use of the referenced instruction, so materialize its
        // results, and drop any that have no uses.
        self.materialize_inst_results(inst_info);
    }

    fn materialize_inst_results(&mut self, inst_info: Rc<InstInfo>) {
        let inst_results = self.f.dfg.inst_results(inst_info.inst);
        for result in inst_results.iter().copied() {
            let is_used = inst_info
                .results
                .iter()
                .any(|v| v.value == result && v.is_used());
            if !is_used {
                self.worklist.push(Plan::Drop(result));
            }
        }
        self.plan_inst(inst_info);
    }

    /// Get the analysis for `inst`, or perform the analysis now and cache it for future queries
    fn get_or_analyze_inst_info(&mut self, inst: hir::Inst, inst_node_id: NodeId) -> Rc<InstInfo> {
        match self.inst_infos.get(inst).cloned() {
            Some(info) => info,
            None => {
                let info = self.analyze_inst(inst, inst_node_id);
                self.inst_infos.insert(info.clone());
                info
            }
        }
    }

    /// Analyze an instruction node from the current block's dependency graph.
    ///
    /// This analysis produces an [InstInfo] that is used during scheduling and
    /// code generation. It provides information about how dependencies of the instruction
    /// should be scheduled, and whether to copy or move data dependencies when needed,
    /// along with some commonly requested pieces of information about the instruction,
    /// such as the number of arguments, its successors, etc.
    ///
    /// NOTE: This can be called either during planning or scheduling, but the
    /// analysis is always scheduling-oriented.
    fn analyze_inst(&self, inst: hir::Inst, inst_node_id: NodeId) -> Rc<InstInfo> {
        let inst_args = self.f.dfg.inst_args(inst);
        let arity = inst_args.len() as u8;

        let results = self.analyze_inst_results(inst);
        let mut inst_info = Box::new(InstInfo {
            inst,
            node: inst_node_id,
            arity,
            args: Default::default(),
            results,
            pre: Default::default(),
            post: Default::default(),
            successors: Default::default(),
        });

        match self.f.dfg.analyze_branch(inst) {
            BranchInfo::SingleDest(block, block_args) => {
                inst_info.successors.push(Successor {
                    block,
                    argc: block_args.len() as u16,
                });
                for (succ_idx, arg) in self
                    .block_info
                    .depgraph
                    .successors(inst_node_id)
                    .filter(|succ| succ.dependency.is_argument())
                    .enumerate()
                {
                    let arg_id = arg.dependency;
                    let arg_source_id = self.block_info.depgraph.unwrap_child(arg_id);
                    if !arg_source_id.is_stack() {
                        inst_info.pre.insert(arg_source_id);
                    }
                    match arg.dependency.into() {
                        Node::Argument(ArgumentNode::Direct { index, .. }) => {
                            debug_assert_eq!(
                                succ_idx, index as usize,
                                "successor ordering constraint violation: {arg:?}"
                            );
                            inst_info.args.push(self.constraint(
                                inst_args[index as usize],
                                arg_id,
                                arg_source_id,
                                None,
                            ));
                        }
                        Node::Argument(ArgumentNode::Indirect { index, .. }) => {
                            debug_assert_eq!(
                                succ_idx + inst_args.len(),
                                index as usize,
                                "successor ordering constraint violation: {arg:?}"
                            );
                            let jt = hir::JumpTable {
                                destination: block,
                                args: block_args,
                            };
                            inst_info.args.push(self.constraint(
                                block_args[index as usize],
                                arg_id,
                                arg_source_id,
                                Some(&[jt]),
                            ));
                        }
                        Node::Argument(arg) => {
                            panic!("invalid argument type for single-destination branch: {arg:?}")
                        }
                        _ => unreachable!(),
                    }
                }
            }
            BranchInfo::MultiDest(ref jts) => {
                for jt in jts.iter() {
                    inst_info.successors.push(Successor {
                        block: jt.destination,
                        argc: jt.args.len() as u16,
                    });
                }
                for (succ_idx, arg) in self
                    .block_info
                    .depgraph
                    .successors(inst_node_id)
                    .filter(|succ| succ.dependency.is_argument())
                    .enumerate()
                {
                    let arg_id = arg.dependency;
                    let arg_source_id = self.block_info.depgraph.unwrap_child(arg_id);
                    match arg.dependency.into() {
                        Node::Argument(ArgumentNode::Direct { index, .. }) => {
                            debug_assert_eq!(
                                succ_idx, index as usize,
                                "successor ordering constraint violation: {arg:?}"
                            );
                            if !arg_source_id.is_stack() {
                                inst_info.pre.insert(arg_source_id);
                            }
                            inst_info.args.push(self.constraint(
                                inst_args[index as usize],
                                arg_id,
                                arg_source_id,
                                Some(jts),
                            ));
                        }
                        Node::Argument(ArgumentNode::Indirect {
                            successor, index, ..
                        }) => {
                            debug_assert_eq!(
                                succ_idx
                                    - inst_args.len()
                                    - jts[..(successor as usize)]
                                        .iter()
                                        .map(|jt| jt.args.len())
                                        .sum::<usize>(),
                                index as usize,
                                "successor ordering constraint violation: {arg:?}"
                            );
                            if !arg_source_id.is_stack() {
                                inst_info.pre.insert(arg_source_id);
                            }
                            let block_arg = jts[successor as usize].args[index as usize];
                            inst_info.args.push(self.constraint(
                                block_arg,
                                arg_id,
                                arg_source_id,
                                Some(jts),
                            ));
                        }
                        Node::Argument(ArgumentNode::Conditional {
                            successor, index, ..
                        }) => {
                            debug_assert_eq!(
                                succ_idx
                                    - inst_args.len()
                                    - jts[..(successor as usize)]
                                        .iter()
                                        .map(|jt| jt.args.len())
                                        .sum::<usize>(),
                                index as usize,
                                "successor ordering constraint violation: {arg:?}"
                            );
                            if !arg_source_id.is_stack() {
                                // TODO: We need to figure out a better way to avoid
                                // materializing conditionally-required results. For
                                // now, we treat these like regular argument dependencies
                                // so that they are on the operand stack when needed.
                                //inst_info.post.insert(arg_source_id);
                                inst_info.pre.insert(arg_source_id);
                            }
                            let block_arg = jts[successor as usize].args[index as usize];
                            inst_info.args.push(self.constraint(
                                block_arg,
                                arg_id,
                                arg_source_id,
                                Some(jts),
                            ));
                        }
                        _ => unreachable!(),
                    }
                }
            }
            BranchInfo::NotABranch => {
                for (succ_idx, arg) in self
                    .block_info
                    .depgraph
                    .successors(inst_node_id)
                    .filter(|succ| succ.dependency.is_argument())
                    .enumerate()
                {
                    let arg_id = arg.dependency;
                    let arg_source_id = self.block_info.depgraph.unwrap_child(arg_id);
                    match arg.dependency.into() {
                        Node::Argument(ArgumentNode::Direct { index, .. }) => {
                            debug_assert_eq!(
                                succ_idx, index as usize,
                                "successor ordering constraint violation: {arg:?}"
                            );
                            if !arg_source_id.is_stack() {
                                inst_info.pre.insert(arg_source_id);
                            }
                            inst_info.args.push(self.constraint(
                                inst_args[index as usize],
                                arg_id,
                                arg_source_id,
                                None,
                            ));
                        }
                        Node::Argument(arg) => {
                            panic!("invalid argument type for non-branching instruction: {arg:?}")
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }

        // Add any control dependencies after the argument dependencies to
        // ensure that any results they produce do not interfere with the
        // placement of operands on the stack (to the degree possible).
        for succ in self
            .block_info
            .depgraph
            .successors(inst_node_id)
            .filter(|succ| !succ.dependency.is_argument())
        {
            let succ_node_id = if succ.dependency.is_instruction() {
                succ.dependency
            } else {
                assert!(succ.dependency.is_result());
                self.block_info.depgraph.unwrap_child(succ.dependency)
            };
            inst_info.pre.insert(succ_node_id);
        }

        Rc::from(inst_info)
    }

    /// Analyze the liveness/usage of the results produced by `inst`, and determine
    /// the order that they will be scheduled in.
    fn analyze_inst_results(&self, inst: hir::Inst) -> SmallVec<[ValueInfo; 2]> {
        let mut infos = SmallVec::<[ValueInfo; 2]>::default();
        // NOTE: Instruction results are presumed to appear on the stack in
        // the same order as produced by the instruction.
        for (result_idx, value) in self.f.dfg.inst_results(inst).iter().copied().enumerate() {
            let result_node = Node::Result {
                value,
                index: result_idx as u8,
            };
            let result_node_id = result_node.id();

            // A result is "externally used" if it is live after the current block
            let is_externally_used = self
                .liveness
                .is_live_after(&value, ProgramPoint::Block(self.block_info.source));

            let mut info = ValueInfo {
                value,
                node: result_node_id,
                is_externally_used,
                users: Default::default(),
            };

            // Record all of the instructions in the current block which use this result
            for pred in self.block_info.depgraph.predecessors(result_node_id) {
                if pred.dependent.is_argument() {
                    info.users
                        .push(self.block_info.depgraph.unwrap_parent(pred.dependent));
                } else {
                    assert!(pred.dependent.is_instruction());
                    info.users.push(pred.dependent);
                }
            }

            // Sort users in scheduling order, i.e. the earlier they appear in the
            // list, the earlier they are visited during planning, and thus the later
            // they are scheduled in the block during codegen.
            info.users
                .sort_unstable_by(|a, b| self.block_info.treegraph.cmp_scheduling(*a, *b));
            infos.push(info);
        }

        // Sort the results by the scheduling order of their first use, i.e. the earlier
        // they appear in the list, the later they are visited during planning, and
        // thus the earlier they are actually used.
        infos.sort_unstable_by(|a, b| match (a.first_user(), b.first_user()) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Greater,
            (Some(_), None) => Ordering::Less,
            (Some(a), Some(b)) => self.block_info.treegraph.cmp_scheduling(a, b).reverse(),
        });
        infos
    }

    fn constraint(
        &self,
        arg: hir::Value,
        arg_node: NodeId,
        arg_sourced_from: NodeId,
        successors: Option<&[hir::JumpTable<'_>]>,
    ) -> Constraint {
        if cfg!(debug_assertions) {
            assert_matches!(
                arg_sourced_from.into(),
                Node::Stack(_) | Node::Result { .. },
                "unexpected argument source: {:?}",
                arg_sourced_from
            );
        }
        let mut transitive_dependents =
            transitive_instruction_dependents(arg_sourced_from, &self.block_info);
        let this_dependent = self.block_info.depgraph.unwrap_parent(arg_node);
        transitive_dependents.remove(&this_dependent);
        let is_last_dependent = transitive_dependents.iter().copied().all(|td| {
            self.block_info
                .treegraph
                .is_scheduled_after(td, this_dependent)
        });
        let is_live_after = match arg_node.into() {
            Node::Argument(ArgumentNode::Direct { .. }) => successors
                .map(|jts| {
                    jts.iter().any(|jt| {
                        let defined_by = self.f.dfg.block_args(jt.destination).contains(&arg);
                        let is_live_at = self
                            .liveness
                            .is_live_at(&arg, ProgramPoint::Block(jt.destination));
                        is_live_at && !defined_by
                    })
                })
                .unwrap_or_else(|| {
                    self.liveness
                        .is_live_after(&arg, ProgramPoint::Block(self.block_info.source))
                }),
            Node::Argument(ArgumentNode::Indirect { successor, .. })
            | Node::Argument(ArgumentNode::Conditional { successor, .. }) => {
                let successors = successors.unwrap();
                let successor = successors[successor as usize].destination;
                let defined_by = self.f.dfg.block_args(successor).contains(&arg);
                let is_live_at = self
                    .liveness
                    .is_live_at(&arg, ProgramPoint::Block(successor));
                is_live_at && !defined_by
            }
            _ => unreachable!(),
        };
        let is_last_use = !is_live_after && is_last_dependent;
        if is_last_use {
            Constraint::Move
        } else {
            Constraint::Copy
        }
    }
}

fn transitive_instruction_dependents(
    stack_or_result_node: NodeId,
    current_block_info: &BlockInfo,
) -> SmallSet<NodeId, 2> {
    current_block_info
        .depgraph
        .predecessors(stack_or_result_node)
        .map(|p| {
            assert!(p.dependent.is_argument());
            current_block_info.depgraph.unwrap_parent(p.dependent)
        })
        .collect()
}

fn build_dependency_graph(
    block_id: hir::Block,
    function: &hir::Function,
    liveness: &LivenessAnalysis,
) -> DependencyGraph {
    let mut graph = DependencyGraph::default();

    // This set represents the values which are guaranteed to be materialized for an instruction
    let mut materialized_args = SmallSet::<hir::Value, 4>::default();
    // This map represents values used as block arguments, and the successors which use them
    let mut block_arg_uses = SmallMap::<hir::Value, SmallSet<hir::Block, 2>>::default();

    // For each instruction, record it and it's arguments/results in the graph
    for (inst_index, inst) in function.dfg.block_insts(block_id).enumerate() {
        materialized_args.clear();
        block_arg_uses.clear();

        let node_id = graph.add_node(Node::Inst {
            id: inst,
            pos: inst_index as u16 + 1,
        });

        let pp = ProgramPoint::Inst(inst);
        for (arg_idx, arg) in function.dfg.inst_args(inst).iter().copied().enumerate() {
            materialized_args.insert(arg);
            let arg_node = ArgumentNode::Direct {
                inst,
                index: arg_idx.try_into().expect("too many arguments"),
            };
            graph.add_data_dependency(node_id, arg_node, arg, pp, function);
        }

        // Ensure all result nodes are added to the graph, otherwise unused results will not be
        // present in the graph which will cause problems when we check for those results later
        for (result_idx, result) in function.dfg.inst_results(inst).iter().copied().enumerate() {
            let result_node = Node::Result {
                value: result,
                index: result_idx as u8,
            };
            let result_node_id = graph.add_node(result_node);
            graph.add_dependency(result_node_id, node_id);
        }

        match function.dfg.analyze_branch(inst) {
            BranchInfo::SingleDest(_, args) => {
                // Add edges representing these data dependencies in later blocks
                for (arg_idx, arg) in args.iter().copied().enumerate() {
                    let arg_node = ArgumentNode::Indirect {
                        inst,
                        index: arg_idx.try_into().expect("too many successor arguments"),
                        successor: 0,
                    };
                    graph.add_data_dependency(node_id, arg_node, arg, pp, function);
                }
            }
            BranchInfo::MultiDest(ref jts) => {
                // Preprocess the arguments which are used so we can determine materialization requirements
                for jt in jts.iter() {
                    for arg in jt.args.iter().copied() {
                        block_arg_uses
                            .entry(arg)
                            .or_insert_with(Default::default)
                            .insert(jt.destination);
                    }
                }
                // For each successor, check if we should implicitly require an argument along that edge due
                // to liveness analysis indicating that it is used somewhere downstream. We only consider
                // block arguments passed to at least one other successor, and which are not already explicitly
                // provided to this successor.
                let materialization_threshold = jts.len();
                // Finally, add edges to the dependency graph representing the nature of each argument
                for (succ_idx, jt) in jts.iter().enumerate() {
                    for (arg_idx, arg) in jt.args.iter().copied().enumerate() {
                        let is_conditionally_materialized =
                            block_arg_uses[&arg].len() < materialization_threshold;
                        let must_materialize =
                            materialized_args.contains(&arg) || !is_conditionally_materialized;
                        let index = arg_idx.try_into().expect("too many successor arguments");
                        let successor = succ_idx.try_into().expect("too many successors");
                        let arg_node = if must_materialize {
                            ArgumentNode::Indirect {
                                inst,
                                index,
                                successor,
                            }
                        } else {
                            ArgumentNode::Conditional {
                                inst,
                                index,
                                successor,
                            }
                        };
                        graph.add_data_dependency(node_id, arg_node, arg, pp, function);
                    }
                }
            }
            BranchInfo::NotABranch => (),
        }
    }

    // HACK: If there are any instruction nodes with no predecessors, with the exception of the block terminator,
    // then we must add a control dependency to the graph to reflect the fact that the instruction
    // must have been placed in this block intentionally. However, we are free to schedule the instruction
    // as we see fit to avoid de-optimizing the normal instruction schedule unintentionally.
    //
    // We also avoid adding control dependencies for instructions without side effects that are not live
    // beyond the current block, as those are dead code and should be eliminated in the DCE step.
    //
    // The actual scheduling decision for the instruction is deferred to `analyze_inst`, where we
    // treat the instruction similarly to argument materialization, and either make it a pre-requisite
    // of the instruction or execute it in the post-execution phase depending on the terminator type
    assign_control_dependencies(&mut graph, block_id, function, liveness);

    // Eliminate dead code as indicated by the state of the dependency graph
    dce(&mut graph, block_id, function, liveness);

    graph
}

/// Discover any instructions in the given block that have no predecessors, but that must be scheduled
/// anyway, i.e. due to side effects - and make the block terminator dependent on them to ensure that
/// they are scheduled.
///
/// We call these instruction->instruction dependencies "control dependencies", since control flow in the
/// block depends on them being executed first. In a way these dependencies are control-flow sensitive, but
/// because the instruction has no direct predecessors, we assume that we are free to schedule them anywhere
/// in the block. For the time being, we choose to schedule them just prior to leaving the block, but in the
/// future we may wish to do more intelligent scheduling of these items, either to reduce the live ranges of
/// values which are used as instruction operands, or if we find that we must attempt to more faithfully
/// preserve the original program ordering for some reason.
///
/// NOTE: This function only assigns control dependencies for instructions _with_ side effects. An
/// instruction with no dependents, and no side effects, is treated as dead code, since by definition
/// its effects cannot be visible. It should be noted however that we are quite conservative about
/// determining if an instruction has side effects - e.g., all function calls are assumed to have
/// side effects at this point in time.
fn assign_control_dependencies(
    graph: &mut DependencyGraph,
    block_id: hir::Block,
    function: &hir::Function,
    liveness: &LivenessAnalysis,
) {
    let terminator = {
        let block = function.dfg.block(block_id);
        let id = block.last().unwrap();
        Node::Inst {
            id,
            pos: block.len() as u16,
        }
    };
    let terminator_id = terminator.into();
    for (inst_index, inst) in function.dfg.block_insts(block_id).enumerate() {
        let opcode = function.dfg.inst(inst).opcode();
        // Skip the block terminator
        if opcode.is_terminator() {
            continue;
        }

        let node = Node::Inst {
            id: inst,
            pos: inst_index as u16 + 1,
        };
        let node_id = node.id();

        // Skip instructions with transitive dependents on at least one result, or a direct dependent
        let has_dependents = graph.predecessors(node_id).any(|pred| {
            if pred.dependent.is_result() {
                graph.num_predecessors(pred.dependent) > 0
            } else {
                true
            }
        });
        if has_dependents {
            continue;
        }

        // Instructions with no side effects require a control dependency if at least
        // one result is live after the end of the current block. We add the dependency
        // to the instruction results if present, otherwise to the instruction itself.
        let pp = ProgramPoint::Block(block_id);
        let mut live_results = SmallVec::<[NodeId; 2]>::default();
        for pred in graph.predecessors(node_id) {
            match pred.dependent.into() {
                Node::Result { value, .. } => {
                    let is_live_after = liveness.is_live_after(&value, pp);
                    if is_live_after {
                        live_results.push(pred.dependent);
                    }
                }
                _ => continue,
            }
        }

        let has_live_results = !live_results.is_empty();
        for result_node in live_results.into_iter() {
            graph.add_dependency(terminator_id, result_node);
        }

        // Instructions with side effects but no live results require a control dependency
        if opcode.has_side_effects() && !has_live_results {
            graph.add_dependency(terminator_id, node_id);
            continue;
        }
    }
}

fn dce(
    graph: &mut DependencyGraph,
    block_id: hir::Block,
    function: &hir::Function,
    liveness: &LivenessAnalysis,
) {
    // Perform dead-code elimination
    //
    // Find all instruction nodes in the graph, and if none of the instruction results
    // are used, or are live beyond it's containing block; and the instruction has no
    // side-effects, then remove all of the nodes related to that instruction, continuing
    // until there are no more nodes to process.
    let mut worklist = VecDeque::<(hir::Inst, NodeId)>::from_iter(
        function
            .dfg
            .block_insts(block_id)
            .enumerate()
            .map(|(i, inst)| {
                (
                    inst,
                    Node::Inst {
                        id: inst,
                        pos: i as u16 + 1,
                    }
                    .into(),
                )
            }),
    );
    let mut remove_nodes = Vec::<NodeId>::default();
    while let Some((inst, inst_node)) = worklist.pop_front() {
        // If the instruction is not dead at this point, leave it alone
        if !is_dead_instruction(inst, block_id, function, liveness, graph) {
            continue;
        }
        let inst_block = function.dfg.insts[inst].block;
        let inst_args = function.dfg.inst_args(inst);
        let branch_info = function.dfg.analyze_branch(inst);
        // Visit the immediate successors of the instruction node in the dependency graph,
        // these by construction may only be Argument or BlockArgument nodes.
        for succ in graph.successors(inst_node) {
            let dependency_node_id = succ.dependency;
            // For each argument, remove the edge from instruction to argument, and from
            // argument to the item it references. If the argument references an instruction
            // result in the same block, add that instruction back to the worklist to check
            // again in case we have made it dead
            match succ.dependency.into() {
                Node::Argument(ArgumentNode::Direct { index, .. }) => {
                    let value = inst_args[index as usize];
                    match function.dfg.value_data(value) {
                        hir::ValueData::Inst {
                            inst: value_inst, ..
                        } => {
                            let value_inst = *value_inst;
                            let value_inst_block = function.dfg.insts[value_inst].block;
                            if value_inst_block == inst_block {
                                let pos = function
                                    .dfg
                                    .block_insts(inst_block)
                                    .position(|id| id == value_inst)
                                    .unwrap();
                                // Check `value_inst` later to see if it has been made dead
                                worklist.push_back((
                                    value_inst,
                                    Node::Inst {
                                        id: value_inst,
                                        pos: pos as u16,
                                    }
                                    .into(),
                                ));
                            }
                        }
                        hir::ValueData::Param { .. } => {}
                    }
                }
                Node::Argument(
                    ArgumentNode::Indirect {
                        successor, index, ..
                    }
                    | ArgumentNode::Conditional {
                        successor, index, ..
                    },
                ) => {
                    let successor = successor as usize;
                    let index = index as usize;
                    let value = match &branch_info {
                        BranchInfo::SingleDest(_, args) => {
                            assert_eq!(successor, 0);
                            args[index]
                        }
                        BranchInfo::MultiDest(ref jts) => jts[successor].args[index],
                        BranchInfo::NotABranch => unreachable!(),
                    };
                    match function.dfg.value_data(value) {
                        hir::ValueData::Inst {
                            inst: value_inst, ..
                        } => {
                            let value_inst = *value_inst;
                            let value_inst_block = function.dfg.insts[value_inst].block;
                            if value_inst_block == inst_block {
                                let pos = function
                                    .dfg
                                    .block_insts(inst_block)
                                    .position(|id| id == value_inst)
                                    .unwrap();
                                // Check `value_inst` later to see if it has been made dead
                                worklist.push_back((
                                    value_inst,
                                    Node::Inst {
                                        id: value_inst,
                                        pos: pos as u16,
                                    }
                                    .into(),
                                ));
                            }
                        }
                        hir::ValueData::Param { .. } => {}
                    }
                }
                // This is a control dependency added intentionally, skip it
                Node::Inst { .. } => continue,
                // No other node types are possible
                _ => unreachable!(),
            }
            remove_nodes.push(dependency_node_id);
        }

        // Remove all of the result nodes because the instruction is going away
        for pred in graph.predecessors(inst_node) {
            remove_nodes.push(pred.dependent);
        }

        // Remove the instruction last
        remove_nodes.push(inst_node);

        // All of the nodes to be removed are queued, so remove them now before we proceed
        for remove_id in remove_nodes.iter().copied() {
            graph.remove_node(remove_id);
        }
    }
}

fn is_dead_instruction(
    inst: hir::Inst,
    block_id: hir::Block,
    function: &hir::Function,
    liveness: &LivenessAnalysis,
    graph: &DependencyGraph,
) -> bool {
    let results = function.dfg.inst_results(inst);
    let has_side_effects = function.dfg.inst(inst).has_side_effects();
    if results.is_empty() && !has_side_effects {
        return true;
    }

    let pp = ProgramPoint::Block(block_id);
    let is_live = results
        .iter()
        .copied()
        .enumerate()
        .any(|(result_idx, result)| {
            let result_node = Node::Result {
                value: result,
                index: result_idx as u8,
            };
            if graph.num_predecessors(result_node) > 0 {
                return true;
            }
            liveness.is_live_after(&result, pp)
        });

    !is_live && !has_side_effects
}
