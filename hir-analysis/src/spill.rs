use std::collections::{BTreeMap, BTreeSet, VecDeque};

use cranelift_entity::{entity_impl, EntityRef};
use midenc_hir::{
    adt::{SmallMap, SmallSet},
    pass::{Analysis, AnalysisManager, AnalysisResult},
    Block, BranchInfo, Function, InsertionPoint, Inst, ProgramPoint, SourceSpan, Type, Value,
};
use midenc_session::Session;
use smallvec::SmallVec;

use crate::{
    BlockPredecessor, ControlFlowGraph, DominatorTree, LivenessAnalysis, Loop, LoopAnalysis,
};

/// This analysis is responsible for simulating the state of the operand stack at each program
/// point, taking into account the results of liveness analysis, and computing whether or not to
/// insert spills/reloads of values which would cause the operand stack depth to exceed 16 elements,
/// the maximum addressable depth.
///
/// The algorithm here is based on the paper _Register Spilling and Live-Range Splitting for
/// SSA-form Programs_ by Matthias Braun and Sebastian Hack, which also happens to describe the
/// algorithm we based our liveness analysis on. While the broad strokes are the same, various
/// modifications/tweaks to the algorithm they describe are needed in order to be suitable for our
/// use case. In particular, we must distinguish between the SSA values which uniquely identify each
/// operand, from the raw elements on the operand stack which represent those values. The need for
/// spills is determined solely on the low-level operand stack representation, _not_ the number of
/// live SSA values (although there can be a correspondance in cases where each SSA value has an
/// effective size of 1 stack element). As this is a type-sensitive analysis, it differs from the
/// algorithm in the paper, which is based on an assumption that all operands are machine-word
/// sized, and thus each value only requires a single register to hold.
///
/// Despite these differences, the overall approach is effectively identical. We still are largely
/// concerned with the SSA values, the primary difference being that we are computing spills based
/// on the raw operand stack state, rather than virtual register pressure as described in the paper.
/// As a result, the number of spills needed at a given program point are not necessarily 1:1, as
/// it may be necessary to spill multiple values in order to free sufficient capacity on the operand
/// stack to hold the required operands; or conversely, we may evict operands that free up more
/// operand stack space than is strictly needed due to the size of those values.
///
/// The general algorithm, once liveness has been computed (see [LivenessAnalysis] for more
/// details), can be summarized as follows:
///
/// In reverse CFG postorder, visit each block B, and:
///
/// 1. Determine initialization of W at entry to B (W^entry). W is the set of operands on the
/// operand stack. From this we are able to determine what, if any, actions are required to
/// keep |W| <= K where K is the maximum allowed operand stack depth.
///
/// 2. Determine initialization of S at entry to B (S^entry). S is the set of values which have
/// been spilled up to that point in the program. We can use S to determine whether or not
/// to actually emit a spill instruction when a spill is necessary, as due to the SSA form of
/// the program, every value has a single definition, so we need only emit a spill for a given
/// value once.
///
/// 3. For each predecessor P of B, determine what, if any, spills and/or reloads are needed to
/// ensure that W and S are consistent regardless of what path is taken to reach B, and that
/// |W| <= K. Depending on whether P has multiple successors, it may be necessary to split the
/// edge between P and B, so that the emitted spills/reloads only apply along that edge.
///
/// 4. Perform the MIN algorithm on B, which is used to determine spill/reloads at each instruction
/// in the block. MIN is designed to make optimal decisions about what to spill, so as to
/// minimize the number of spill/reload-related instructions executed by any given program
/// execution trace. It does this by using the next-use distance associated with values in W,
/// which is computed as part of our liveness analysis. Unlike traditional liveness analysis
/// which only tracks what is live at a given program point, next-use distances not only tell
/// you whether a value is live or dead, but how far away the next use of that value is. MIN
/// uses this information to select spill candidates from W furthest away from the current
/// instruction; and on top of this we also add an additional heuristic based on the size of
/// each candidate as represented on the operand stack. Given two values with equal next-use
/// distances, the largest candidates are spilled first, allowing us to free more operand stack
/// space with fewer spills.
///
/// The MIN algorithm works as follows:
///
/// 1. Starting at the top of the block, B, W is initialized with the set W^entry(B), and S with
/// S^entry(B)
///
/// 2. For each instruction, I, in the block, update W and S according to the needs of I, while
/// attempting to preserve as many live values in W as possible. Each instruction fundamentally
/// requires that: On entry, W contains all the operands of I; on exit, W contains all of the
/// results of I; and that at all times, |W| <= K. This means that we may need to reload operands
/// of I that are not in W (because they were spilled), and we may need to spill values from W to
/// ensure that the stack depth <= K. The specific effects for I are computed as follows:
///   a. All operands of I not in W, must be reloaded in front of I, thus adding them to W.
///   This is also one means by which values are added to S, as by definition a reload
///   implies that the value must have been spilled, or it would still be in W. Thus, when
///   we emit reloads, we also ensure that the reloaded value is added to S.
///   b. If a reload would cause |W| to exceed K, we must select values in W to spill. Candidates
///   are selected from the set of values in W which are not operands of I, prioritized first
///   by greatest next-use distance, then by stack consumption, as determined by the
///   representation of the value type on the operand stack.
///   c. By definition, none of I's results can be in W directly in front of I, so we must
///   always ensure that W has sufficient capacity to hold all of I's results. The analysis
///   of sufficient capacity is somewhat subtle:
///     - Any of I's operands that are live-at I, but _not_ live-after I, do _not_ count towards
///     the operand stack usage when calculating available capacity for the results. This is
///     because those operands will be consumed, and their space can be re-used for results.
///     - Any of I's operands that are live-after I, however, _do_ count towards the stack usage
///     - If W still has insufficient capacity for all the results, we must select candidates
///     to spill. Candidates are the set of values in W which are either not operands of I,
///     or are operands of I which are live-after I. Selection criteria is the same as before.
///   d. Operands of I which are _not_ live-after I, are removed from W on exit from I, thus W
///   reflects only those values which are live at the current program point.
///   e. Lastly, when we select a value to be spilled, we only emit spill instructions for those
///   values which are not yet in S, i.e. they have not yet been spilled; and which have a
///   finite next-use distance, i.e. the value is still live. If a value to be spilled _is_
///   in S and/or is unused after that point in the program, we can elide the spill entirely.
///
/// What we've described above represents both the analysis itself, as well as the effects of
/// applying that analysis to the actual control flow graph of the function. However, doing so
/// introduces a problem that must be addressed: SSA-form programs can only have a single definition
/// of each value, but by introducing spills (and consequently, reloads of the spilled values), we
/// have introduced new definitions of those values - each reload constitutes a new definition.
/// As a result, our program is no longer in SSA form, and we must restore that property in order
/// to proceed with compilation.
///
/// **NOTE:** The way that we represent reloads doesn't _literally_ introduce multiple definitions
/// of a given [Value], our IR does not permit representing that. Instead, we represent reloads as
/// an instruction which takes the spilled SSA value we want to reload as an argument, and produces
/// a new SSA value representing the reloaded spill. As a result of this representation, our program
/// always remains tecnically in SSA form, but the essence of the problem remains the same: When a
/// value is spilled, its live range is terminated; a reload effectively brings the spilled value
/// back to life, starting a new live range. Thus references to the spilled value which are now
/// dominated by a reload in the control flow graph, are no longer semantically correct - they must
/// be rewritten to reference the nearest dominating definition.
///
/// Restoring SSA form is not the responsibility of this analysis, however I will briefly describe
/// the method here, while you have the context at hand. The obvious assumption would be that we
/// simply treat each reload as a new SSA value, and update any uses of the original value with the
/// nearest dominating definition. The way we represent reloads in HIR already does the first step
/// for us, however there is a subtle problem with the second part: join points in the control flow
/// graph. Consider the following:
///
/// ```text,ignore
/// (block 0 (param v0) (param v1)
///   (cond_br v1 (block 1) (block 2)))
///
/// (block 1
///   (spill v0)
///   ...
///   (let v2 (reload v0)) ; here we've assigned the reload of v0 a new SSA value
///   (br (block 3)))
///
/// (block 2
///   ...
///   (br (block 3)))
///
/// (block 3
///    (ret v2)) ; here we've updated a v0 reference to the nearest definition
/// ```
///
/// Above, control flow branches in one of two directions from the entry block, and along one of
/// those branches `v0` is spilled and later reloaded. Control flow joins again in the final block
/// where `v0` is returned. We attempted to restore the program to SSA form as described above,
/// first by assigning reloads a new SSA value, then by finding all uses of the spilled value and
/// rewriting those uses to reference the nearest dominating definition.
///
/// Because the use of `v0` in block 3 is dominated by the reload in block 1, it is rewritten to
/// reference `v2` instead. The problem with that is obvious - the reload in block 1 does not
/// _strictly_ dominate the use in block 3, i.e. there are paths through the function which can
/// reach block 3 without passing through block 1 first, and `v2` will be undefined along those
/// paths!
///
/// However this problem also has an obvious solution: introduce a new block parameter in block 3
/// to represent the appropriate definition of `v0` that applies based on the predecessor used to
/// reach block 3. This ensures that the use in block 3 is strictly dominated by an appropriate
/// definition.
///
/// So now that we've understood the problem with the naive approach, and the essence of the
/// solution to that particular problem, we can walk through the generalized solution that can be
/// used to reconstruct SSA form for any program we can represent in our IR.
///
/// 1. Given the set of spilled values, S, visit the dominance tree in postorder (bottom-up)
/// 2. In each block, working towards the start of the block from the end, visit each instruction
///    until one of the following occurs:
///   a. We find a use of a value in S. We append the use to the list of other uses of that value
///      which are awaiting a rewrite while we search for the nearest dominating definition.
///   b. We find a reload of a value in S. This reload is, by construction, the nearest dominating
///      definition for all uses of the reloaded value that we have found so far. We rewrite all of
///      those uses to reference the reloaded value, and remove them from the list.
///   c. We find the original definition of a value in S. This is similar to what happens when we
///      find a reload, except no rewrite is needed, so we simply remove all pending uses of that
///      value from the list.
///   d. We reach the top of the block. Note that block parameters are treated as definitions, so
///      those are handled first as described in the previous point. However, an additional step
///      is required here: If the current block is in the iterated dominance frontier for S, i.e.
///      for any value in S, the current block is in the dominance frontier of the original
///      definition of that value - then for each such value for which we have found at least one
///      use, we must add a new block parameter representing that value; rewrite all uses we have
///      found so far to use the block parameter instead; remove those uses from the list; and
///      lastly, rewrite the branch instruction in each predecessor to pass the value as a new block
///      argument when branching to the current block.
/// 3. When we start processing a block, the union of the set of unresolved uses found in each
///    successor, forms the initial state of that set for the current block. If a block has no
///    successors, then the set is initially empty. This is how we propagate uses up the dominance
///    tree until we find an appropriate definition. Since we ensure that block parameters are added
///    along the dominance frontier for each spilled value, we guarantee that the first definition
///    we reach always strictly dominates the uses we have propagated to that point.
///
/// NOTE: A nice side effect of this algorithm is that any reloads we reach for which we have
/// no uses, are dead and can be eliminated. Similarly, a reload we never reach must also be
/// dead code - but in practice that won't happen, since we do not visit unreachable blocks
/// during the spill analysis anyway.
#[derive(Debug, Default, Clone)]
pub struct SpillAnalysis {
    // The set of control flow edges that must be split to accommodate spills/reloads.
    pub splits: Vec<SplitInfo>,
    // The set of values that have been spilled
    pub spilled: BTreeSet<Value>,
    // The spills themselves
    pub spills: Vec<SpillInfo>,
    // The set of instructions corresponding to the reload of a spilled value
    pub reloads: Vec<ReloadInfo>,
    // The set of operands in registers on entry to a given block
    w_entries: BTreeMap<Block, SmallSet<Operand, 4>>,
    // The set of operands in registers on exit from a given block
    w_exits: BTreeMap<Block, SmallSet<Operand, 4>>,
    // The set of operands that have been spilled so far, on exit from a given block
    s_exits: BTreeMap<Block, SmallSet<Operand, 4>>,
}

/// The state of the W and S sets on entry to a given block
#[derive(Debug)]
struct BlockInfo {
    block_id: Block,
    w_entry: SmallSet<Operand, 4>,
    s_entry: SmallSet<Operand, 4>,
}

/// Uniquely identifies a computed split control flow edge in a [SpillAnalysis]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Split(u32);
entity_impl!(Split, "split");

/// Metadata about a control flow edge which needs to be split in order to accommodate spills and/or
/// reloads along that edge.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SplitInfo {
    pub id: Split,
    /// The destination block for the control flow edge being split
    pub block: Block,
    /// The predecessor, or origin, of the control flow edge being split
    pub predecessor: BlockPredecessor,
    /// The block representing the split, if materialized
    pub split: Option<Block>,
}

/// Uniquely identifies a computed spill in a [SpillAnalysis]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Spill(u32);
entity_impl!(Spill, "spill");

/// Metadata about a computed spill
#[derive(Debug, Clone)]
pub struct SpillInfo {
    pub id: Spill,
    /// The point in the program where this spill should be placed
    pub place: Placement,
    /// The value to be spilled
    pub value: Value,
    /// The type of the spilled value
    pub ty: Type,
    /// The span associated with the source code that triggered the spill
    pub span: SourceSpan,
    /// The spill instruction, if materialized
    pub inst: Option<Inst>,
}

/// Uniquely identifies a computed reload in a [SpillAnalysis]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Reload(u32);
entity_impl!(Reload, "reload");

/// Metadata about a computed reload
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ReloadInfo {
    pub id: Reload,
    /// The point in the program where this spill should be placed
    pub place: Placement,
    /// The spilled value to be reloaded
    pub value: Value,
    /// The span associated with the source code that triggered the spill
    pub span: SourceSpan,
    /// The reload instruction, if materialized
    pub inst: Option<Inst>,
}

/// This enumeration represents a program location where a spill or reload operation should be
/// placed.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Placement {
    /// A concrete location in the current program.
    ///
    /// The operation will be placed according to the semantics of the given [InsertionPoint]
    At(InsertionPoint),
    /// A pseudo-location, corresponding to the end of the block that will be materialized
    /// to split the control flow edge represented by [Split].
    Split(Split),
}

/// An [Operand] is a possibly-aliased [Value], combined with the size of that value on the
/// Miden operand stack. This extra information is used to not only compute whether or not we
/// need to spill values during execution of a function, and how to prioritize those spills;
/// but also to track aliases of a [Value] introduced when we insert reloads of a spilled value.
///
/// Once a spilled value is reloaded, the SSA property of the CFG is broken, as we now have two
/// definitions of the same [Value]. To restore the SSA property, we have to assign the reloaded
/// value a new id, and then update all uses of the reloaded value dominated by that reload to
/// refer to the new [Value]. We use the `alias` field of [Operand] to track distinct reloads of
/// a given [Value] during the initial insertion of reloads.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Operand {
    /// The SSA value of this operand
    pub value: Value,
    /// When an SSA value is used multiple times by an instruction, each use must be accounted for
    /// on the operand stack in order to properly determine whether a spill is needed or not.
    /// We assign each unique copy an integer id in the register file to ensure this.
    pub alias: u16,
    /// The size in elements on the operand stack required by this operand
    pub size: u16,
}
impl core::borrow::Borrow<Value> for Operand {
    #[inline(always)]
    fn borrow(&self) -> &Value {
        &self.value
    }
}

impl Operand {
    pub fn new(value: Value, function: &Function) -> Self {
        let size = u16::try_from(function.dfg.value_type(value).size_in_felts())
            .expect("invalid value type: ssa values cannot be larger than a word");
        Self {
            value,
            alias: 0,
            size,
        }
    }
}

/// The maximum number of operand stack slots which can be assigned without spills.
const K: usize = 16;

impl Analysis for SpillAnalysis {
    type Entity = Function;

    fn analyze(
        function: &Self::Entity,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> AnalysisResult<Self> {
        let cfg = analyses.get_or_compute(function, session)?;
        let domtree = analyses.get_or_compute(function, session)?;
        let loops = analyses.get_or_compute(function, session)?;
        let liveness = analyses.get_or_compute(function, session)?;
        SpillAnalysis::compute(function, &cfg, &domtree, &loops, &liveness)
    }
}

/// Queries
impl SpillAnalysis {
    /// Returns true if at least one value must be spilled
    pub fn has_spills(&self) -> bool {
        !self.spills.is_empty()
    }

    /// Returns the set of control flow edges that must be split to accommodate spills/reloads
    pub fn splits(&self) -> &[SplitInfo] {
        self.splits.as_slice()
    }

    /// Same as [SpillAnalysis::splits], but as a mutable reference
    pub fn splits_mut(&mut self) -> &mut [SplitInfo] {
        self.splits.as_mut_slice()
    }

    pub fn get_split(&self, split: Split) -> &SplitInfo {
        &self.splits[split.index()]
    }

    /// Returns the set of values which require spills
    pub fn spilled(&self) -> &BTreeSet<Value> {
        &self.spilled
    }

    /// Returns true if `value` is spilled at some point
    pub fn is_spilled(&self, value: &Value) -> bool {
        self.spilled.contains(value)
    }

    /// Returns true if `value` is spilled at the given program point (i.e. inserted before)
    pub fn is_spilled_at(&self, value: Value, pp: impl Into<ProgramPoint>) -> bool {
        let place = match pp.into() {
            ProgramPoint::Block(split_block) => {
                match self.splits.iter().find(|split| split.split == Some(split_block)) {
                    Some(split) => Placement::Split(split.id),
                    None => Placement::At(InsertionPoint::after(split_block.into())),
                }
            }
            pp @ ProgramPoint::Inst(_) => Placement::At(InsertionPoint::before(pp)),
        };
        self.spills.iter().any(|info| info.value == value && info.place == place)
    }

    /// Returns true if `value` will be spilled in the given split
    pub fn is_spilled_in_split(&self, value: Value, split: Split) -> bool {
        self.spills.iter().any(|info| {
            info.value == value && matches!(info.place, Placement::Split(s) if s == split)
        })
    }

    /// Returns the set of computed spills
    pub fn spills(&self) -> &[SpillInfo] {
        self.spills.as_slice()
    }

    /// Same as [SpillAnalysis::spills], but as a mutable reference
    pub fn spills_mut(&mut self) -> &mut [SpillInfo] {
        self.spills.as_mut_slice()
    }

    /// Returns true if `value` is reloaded at some point
    pub fn is_reloaded(&self, value: &Value) -> bool {
        self.reloads.iter().any(|info| &info.value == value)
    }

    /// Returns true if `value` is reloaded at the given program point (i.e. inserted before)
    pub fn is_reloaded_at(&self, value: Value, pp: impl Into<ProgramPoint>) -> bool {
        let place = match pp.into() {
            ProgramPoint::Block(split_block) => {
                match self.splits.iter().find(|split| split.split == Some(split_block)) {
                    Some(split) => Placement::Split(split.id),
                    None => Placement::At(InsertionPoint::after(split_block.into())),
                }
            }
            pp @ ProgramPoint::Inst(_) => Placement::At(InsertionPoint::before(pp)),
        };
        self.reloads.iter().any(|info| info.value == value && info.place == place)
    }

    /// Returns true if `value` will be reloaded in the given split
    pub fn is_reloaded_in_split(&self, value: Value, split: Split) -> bool {
        self.reloads.iter().any(|info| {
            info.value == value && matches!(info.place, Placement::Split(s) if s == split)
        })
    }

    /// Returns the set of computed reloads
    pub fn reloads(&self) -> &[ReloadInfo] {
        self.reloads.as_slice()
    }

    /// Same as [SpillAnalysis::reloads], but as a mutable reference
    pub fn reloads_mut(&mut self) -> &mut [ReloadInfo] {
        self.reloads.as_mut_slice()
    }

    /// Returns the operands in W upon entry to `block`
    pub fn w_entry(&self, block: &Block) -> &[Operand] {
        self.w_entries[block].as_slice()
    }

    /// Returns the operands in W upon exit from `block`
    pub fn w_exit(&self, block: &Block) -> &[Operand] {
        self.w_exits[block].as_slice()
    }

    /// Returns the operands in S upon exit from `block`
    pub fn s_exit(&self, block: &Block) -> &[Operand] {
        self.s_exits[block].as_slice()
    }
}

/// Analysis
impl SpillAnalysis {
    pub fn compute(
        function: &Function,
        cfg: &ControlFlowGraph,
        domtree: &DominatorTree,
        loops: &LoopAnalysis,
        liveness: &LivenessAnalysis,
    ) -> AnalysisResult<Self> {
        let mut analysis = Self::default();

        // Visit the blocks of the CFG in reverse postorder (top-down)
        let mut block_q = VecDeque::<Block>::default();
        block_q.extend(domtree.cfg_postorder().iter().rev().copied());

        // If a block has a predecessor which it dominates (i.e. control flow always flows through
        // the block in question before the given predecessor), then we must defer computing spills
        // and reloads for that edge until we have visited the predecessor. This map is used to
        // track deferred edges for each block.
        let mut deferred = Vec::<(Block, SmallVec<[Block; 2]>)>::default();

        // This is used to track the set of instructions in the current block being analyzed
        let mut inst_q = VecDeque::<Inst>::with_capacity(32);

        while let Some(block_id) = block_q.pop_front() {
            inst_q.clear();

            // Compute W^entry(B)
            compute_w_entry(block_id, &mut analysis, function, cfg, loops, liveness);
            let w_entry = analysis.w_entries[&block_id].clone();

            // Compute S^entry(B)
            let mut s_entry = SmallSet::<Operand, 4>::default();
            for pred in cfg.pred_iter(block_id) {
                if let Some(s_exitp) = analysis.s_exits.get(&pred.block) {
                    s_entry = s_entry.into_union(s_exitp);
                }
            }
            s_entry = s_entry.into_intersection(&w_entry);

            let block_info = BlockInfo {
                block_id,
                w_entry,
                s_entry,
            };

            // For each predecessor P of B, insert spills/reloads along the inbound control flow
            // edge as follwos:
            //
            // * All variables in W^entry(B) \ W^exit(P) need to be reloaded
            // * All variables in (S^entry(B) \ S^exit(P)) ∩ W^exit(P) need to be spilled
            //
            // If a given predecessor has not been processed yet, skip P, and revisit the edge later
            // after we have processed P.
            //
            // NOTE: Because W^exit(P) does not contain the block parameters for any given
            // successor, as those values are renamed predecessor operands, some work must be done
            // to determine the true contents of W^exit(P) for each predecessor/successor edge, and
            // only then insert spills/reloads as described above.
            let mut deferred_preds = SmallVec::<[Block; 2]>::default();
            for pred in cfg.pred_iter(block_id) {
                // As soon as we need to start inserting spills/reloads, mark the function changed
                compute_control_flow_edge_spills_and_reloads(
                    &mut analysis,
                    &block_info,
                    &pred,
                    &mut deferred_preds,
                    function,
                    cfg,
                    liveness,
                );
            }
            if !deferred_preds.is_empty() {
                deferred.push((block_id, deferred_preds));
            }

            // We have our W and S sets for the entry of B, and we have inserted all spills/reloads
            // needed on incoming control flow edges to ensure that the contents of W and S are the
            // same regardless of which predecessor we reach B from.
            //
            // Now, we essentially repeat this process for each instruction I in B, i.e. we apply
            // the MIN algorithm to B. As a result, we will also have computed the contents of W
            // and S at the exit of B, which will be needed subsequently for the successors of B
            // when we process them.
            //
            // The primary differences here, are that we:
            //
            // * Assume that if a reload is needed (not in W), that it was previously spilled (must
            //   be in S)
            // * We do not issue spills for values that have already been spilled
            // * We do not emit spill instructions for values which are dead, they are just dropped
            // * We must spill from W to make room for operands and results of I, if there is
            //   insufficient space to hold the current contents of W + whatever operands of I we
            //   need to reload + the results of I that will be placed on the operand stack. We do
            //   so by spilling values with the greatest next-use distance first, preferring to
            //   spill larger values where we have an option. We also may factor in liveness - if an
            //   operand of I is dead after I, we do not need to count that operand when computing
            //   the operand stack usage for results (thus reusing the space of the operand for one
            //   or more results).
            // * It is important to note that we must count _all_ uses of the same value towards the
            //   operand stack usage, unless the semantics of an instruction explicitly dictate that
            //   a specific operand pattern only requires a single copy on the operand stack.
            //   Currently that is not the case for any instructions, and we would prefer to be more
            //   conservative at this point anyway.
            let mut w = block_info.w_entry;
            let mut s = block_info.s_entry;
            inst_q.extend(function.dfg.block_insts(block_id));
            while let Some(current_inst) = inst_q.pop_front() {
                min(current_inst, &mut w, &mut s, &mut analysis, function, liveness);
            }

            analysis.w_exits.insert(block_id, w);
            analysis.s_exits.insert(block_id, s);
        }

        // We've visited all blocks at least once, now we need to go back and insert
        // spills/reloads along loopback edges, as we skipped those on the first pass
        for (block_id, preds) in deferred {
            // W^entry(B)
            let w_entry = analysis.w_entries[&block_id].clone();

            // Compute S^entry(B)
            let mut s_entry = SmallSet::<Operand, 4>::default();
            for pred in cfg.pred_iter(block_id) {
                s_entry = s_entry.into_union(&analysis.s_exits[&pred.block]);
            }
            s_entry = s_entry.into_intersection(&w_entry);

            let block_info = BlockInfo {
                block_id,
                w_entry,
                s_entry,
            };

            // For each predecessor P of B, insert spills/reloads along the inbound control flow
            // edge as follwos:
            //
            // * All variables in W^entry(B) \ W^exit(P) need to be reloaded
            // * All variables in (S^entry(B) \ S^exit(P)) ∩ W^exit(P) need to be spilled
            //
            // If a given predecessor has not been processed yet, skip P, and revisit the edge later
            // after we have processed P.
            let mut _defer = SmallVec::default();
            for pred in cfg.pred_iter(block_id) {
                // Only visit predecessors that were deferred
                if !preds.contains(&pred.block) {
                    continue;
                }

                compute_control_flow_edge_spills_and_reloads(
                    &mut analysis,
                    &block_info,
                    &pred,
                    &mut _defer,
                    function,
                    cfg,
                    liveness,
                );
            }
        }

        Ok(analysis)
    }

    pub fn set_materialized_split(&mut self, split: Split, block: Block) {
        self.splits[split.index()].split = Some(block);
    }

    pub fn set_materialized_spill(&mut self, spill: Spill, inst: Inst) {
        self.spills[spill.index()].inst = Some(inst);
    }

    pub fn set_materialized_reload(&mut self, reload: Reload, inst: Inst) {
        self.reloads[reload.index()].inst = Some(inst);
    }

    fn spill(
        &mut self,
        place: Placement,
        value: Value,
        span: SourceSpan,
        function: &Function,
    ) -> Spill {
        let id = Spill::new(self.spills.len());
        let ty = function.dfg.value_type(value).clone();
        self.spilled.insert(value);
        self.spills.push(SpillInfo {
            id,
            place,
            value,
            ty,
            span,
            inst: None,
        });
        id
    }

    fn reload(&mut self, place: Placement, value: Value, span: SourceSpan) -> Reload {
        let id = Reload::new(self.reloads.len());
        self.reloads.push(ReloadInfo {
            id,
            place,
            value,
            span,
            inst: None,
        });
        id
    }

    fn split(&mut self, block: Block, predecessor: BlockPredecessor) -> Split {
        let id = Split::new(self.splits.len());
        self.splits.push(SplitInfo {
            id,
            block,
            predecessor,
            split: None,
        });
        id
    }
}

fn compute_w_entry(
    block_id: Block,
    analysis: &mut SpillAnalysis,
    function: &Function,
    cfg: &ControlFlowGraph,
    loops: &LoopAnalysis,
    liveness: &LivenessAnalysis,
) {
    if let Some(loop_id) = loops.is_loop_header(block_id) {
        compute_w_entry_loop(block_id, loop_id, analysis, function, cfg, loops, liveness);
    } else {
        compute_w_entry_normal(block_id, analysis, function, cfg, liveness);
    }
}

fn compute_w_entry_normal(
    block_id: Block,
    analysis: &mut SpillAnalysis,
    function: &Function,
    cfg: &ControlFlowGraph,
    liveness: &LivenessAnalysis,
) {
    let mut freq = SmallMap::<Operand, u8, 4>::default();
    let mut take = SmallSet::<Operand, 4>::default();
    let mut cand = SmallSet::<Operand, 4>::default();

    // Block arguments are always in w_entry by definition
    for arg in function.dfg.block_params(block_id) {
        take.insert(Operand::new(*arg, function));
    }

    // TODO(pauls): We likely need to account for the implicit spilling that occurs when the
    // operand stack space required by the function arguments exceeds K. In such cases, the W set
    // contains the function parameters up to the first parameter that would cause the operand
    // stack to overflow, all subsequent parameters are placed on the advice stack, and are assumed
    // to be moved from the advice stack to locals in the same order as they appear in the function
    // signature as part of the function prologue. Thus, the S set is preloaded with those values
    // which were spilled in this manner.
    //
    // NOTE: It should never be the case that the set of block arguments consumes more than K
    assert!(
        take.iter().map(|o| o.size as usize).sum::<usize>() <= K,
        "unhandled spills implied by function/block parameter list"
    );

    // If this is the entry block, the operands in w_entry are guaranteed to be equal to the set of
    // function arguments, so we're done.
    if block_id == function.dfg.entry_block() {
        analysis.w_entries.insert(block_id, take);
        return;
    }

    for pred in cfg.pred_iter(block_id) {
        for o in analysis.w_exits[&pred.block].iter().cloned() {
            // Do not add candidates which are not live-after the predecessor
            if liveness.is_live_after(&o.value, ProgramPoint::Block(pred.block)) {
                *freq.entry(o).or_insert(0) += 1;
                cand.insert(o);
            }
        }
    }

    let num_preds = cfg.num_predecessors(block_id);
    for (v, count) in freq.iter() {
        if *count as usize == num_preds {
            cand.remove(v);
            take.insert(*v);
        }
    }

    // If there are paths to B containing > K values on the operand stack, this must be due to the
    // successor arguments that are renamed on entry to B, remaining live in B, which implicitly
    // requires copying so that both the block parameter and the source value are both live in B.
    //
    // However, in order to actually fail this assertion, it would have to be the case that all
    // predecessors of this block are passing the same value as a successor argument, _and_ that the
    // value is still live in this block. This would imply that the block parameter is unnecessary
    // in the first place.
    //
    // Since that is extraordinarily unlikely to occur, and we want to catch any situations in which
    // this assertion fails, we do not attempt to handle it automatically.
    let taken = take.iter().map(|o| o.size as usize).sum::<usize>();
    assert!(
        taken <= K,
        "implicit operand stack overflow along incoming control flow edges of {block_id}"
    );

    let entry = ProgramPoint::Inst(function.dfg.block_insts(block_id).next().unwrap());

    // Prefer to select candidates with the smallest next-use distance, otherwise all else being
    // equal, choose to keep smaller values on the operand stack, and spill larger values, thus
    // freeing more space when spills are needed.
    let mut cand = cand.into_vec();
    cand.sort_by(|a, b| {
        liveness
            .next_use(&a.value, entry)
            .cmp(&liveness.next_use(&b.value, entry))
            .then(a.size.cmp(&b.size))
    });

    let mut available = K - taken;
    let mut cand = cand.into_iter();
    while available > 0 {
        if let Some(candidate) = cand.next() {
            let size = candidate.size as usize;
            if size <= available {
                take.insert(candidate);
                available -= size;
                continue;
            }
        }
        break;
    }

    analysis.w_entries.insert(block_id, take);
}

fn compute_w_entry_loop(
    block_id: Block,
    loop_id: Loop,
    analysis: &mut SpillAnalysis,
    function: &Function,
    cfg: &ControlFlowGraph,
    loops: &LoopAnalysis,
    liveness: &LivenessAnalysis,
) {
    const LOOP_EXIT_DISTANCE: u32 = 100_000;

    let entry = ProgramPoint::Inst(function.dfg.block_insts(block_id).next().unwrap());

    let params = function.dfg.block_params(block_id);
    let mut alive = params
        .iter()
        .copied()
        .map(|v| Operand::new(v, function))
        .collect::<SmallSet<Operand, 4>>();
    alive.extend(
        liveness
            .live_at(ProgramPoint::Block(block_id))
            .map(|v| Operand::new(v, function)),
    );

    // Initial candidates are values live at block entry which are used in the loop body
    let mut cand = alive
        .iter()
        .filter(|o| liveness.next_use(&o.value, ProgramPoint::Block(block_id)) < LOOP_EXIT_DISTANCE)
        .cloned()
        .collect::<SmallSet<Operand, 4>>();

    // Values which are "live through" the loop, are those which are live at entry, but not
    // used within the body of the loop. If we have excess available operand stack capacity,
    // then we can avoid issuing spills/reloads for at least some of these values.
    let live_through = alive.difference(&cand);

    let w_used = cand.iter().map(|o| o.size as usize).sum::<usize>();
    if w_used < K {
        if let Some(mut free_in_loop) =
            K.checked_sub(max_loop_pressure(loop_id, liveness, cfg, loops))
        {
            let mut live_through = live_through.into_vec();
            live_through.sort_by(|a, b| {
                liveness
                    .next_use(&a.value, entry)
                    .cmp(&liveness.next_use(&b.value, entry))
                    .then(a.size.cmp(&b.size))
            });

            let mut live_through = live_through.into_iter();
            while free_in_loop > 0 {
                if let Some(operand) = live_through.next() {
                    if let Some(new_free) = free_in_loop.checked_sub(operand.size as usize) {
                        if cand.insert(operand) {
                            free_in_loop = new_free;
                        }
                        continue;
                    }
                }
                break;
            }
        }

        analysis.w_entries.insert(block_id, cand);
    } else {
        // We require the block parameters to be in W on entry
        let mut take =
            SmallSet::<_, 4>::from_iter(params.iter().map(|v| Operand::new(*v, function)));

        // So remove them from the set of candidates, then sort remaining by next-use and size
        let mut cand = cand.into_vec();
        cand.retain(|o| !params.contains(&o.value));
        cand.sort_by(|a, b| {
            liveness
                .next_use(&a.value, entry)
                .cmp(&liveness.next_use(&b.value, entry))
                .then(a.size.cmp(&b.size))
        });

        // Fill `take` with as many of the candidates as we can
        let mut taken = take.iter().map(|o| o.size as usize).sum::<usize>();
        take.extend(cand.into_iter().take_while(|operand| {
            let size = operand.size as usize;
            let new_size = taken + size;
            if new_size <= K {
                taken = new_size;
                true
            } else {
                false
            }
        }));
        analysis.w_entries.insert(block_id, take);
    }
}

/// Compute the maximum operand stack depth required within the body of the given loop.
///
/// If the stack depth never reaches K, the excess capacity represents an opportunity to
/// avoid issuing spills/reloads for values which are live through the loop.
fn max_loop_pressure(
    loop_id: Loop,
    liveness: &LivenessAnalysis,
    cfg: &ControlFlowGraph,
    loops: &LoopAnalysis,
) -> usize {
    let header = loops.loop_header(loop_id);
    let mut max = liveness.max_operand_stack_pressure(&header);
    let mut block_q = VecDeque::from_iter([header]);
    let mut visited = SmallSet::<Block, 4>::default();

    while let Some(block) = block_q.pop_front() {
        if !visited.insert(block) {
            continue;
        }

        block_q.extend(cfg.succ_iter(block).filter(|b| loops.is_in_loop(*b, loop_id)));

        max = core::cmp::max(max, liveness.max_operand_stack_pressure(&block));
    }

    max
}

/// At join points in the control flow graph, the set of live and spilled values may, and likely
/// will, differ depending on which predecessor is taken to reach it. We must ensure that for
/// any given predecessor:
///
/// * Spills are inserted for any values expected in S upon entry to the successor block, which have
///   not been spilled yet. This occurs when a spill is needed in some predecessor, but not in
///   another, thus we must make sure the spill slot is written to at join points.
/// * Reloads are inserted for any values expected in W upon entry to the successor block, which are
///   not in W yet. This occurs when a value is spilled on the path taken through a given
///   predecessor, and hasn't been reloaded again, thus we need to reload it now.
///
/// NOTE: We are not actually mutating the function and inserting instructions here. Instead, we
/// are computing what instructions need to be inserted, and where, as part of the analysis. A
/// rewrite pass can then apply the analysis results to the function, if desired.
fn compute_control_flow_edge_spills_and_reloads(
    analysis: &mut SpillAnalysis,
    block_info: &BlockInfo,
    pred: &BlockPredecessor,
    deferred: &mut SmallVec<[Block; 2]>,
    function: &Function,
    _cfg: &ControlFlowGraph,
    liveness: &LivenessAnalysis,
) {
    // If we don't have W^exit(P), then P hasn't been processed yet
    let Some(w_exitp) = analysis.w_exits.get(&pred.block) else {
        deferred.push(pred.block);
        return;
    };

    let mut to_reload = block_info.w_entry.difference(w_exitp);
    let mut to_spill = block_info
        .s_entry
        .difference(&analysis.s_exits[&pred.block])
        .into_intersection(w_exitp);

    // We need to issue spills for any items in W^exit(P) / W^entry(B) that are not in S^exit(P),
    // but are live-after P.
    //
    // This can occur when B is a loop header, and the computed W^entry(B) does not include values
    // in W^exit(P) that are live-through the loop, typically because of loop pressure within the
    // loop requiring us to place spills of those values outside the loop.
    let must_spill = w_exitp
        .difference(&block_info.w_entry)
        .into_difference(&analysis.s_exits[&pred.block]);
    to_spill.extend(
        must_spill
            .into_iter()
            .filter(|o| liveness.is_live_at(&o.value, ProgramPoint::Block(block_info.block_id))),
    );

    // We expect any block parameters present to be in `to_reload` at this point, as they will never
    // be in W^exit(P) (the parameters are not in scope at the end of P). The arguments provided in
    // the predecessor corresponding to the block parameters may or may not be in W^exit(P), so we
    // must determine which of those values need to be reloaded, and whether or not to spill any of
    // them so that there is sufficient room in W to hold all the block parameters. Spills may be
    // needed for two reasons:
    //
    // 1. There are multiple predecessors, and we need to spill a value to ensure it is spilled on
    //    all paths to the current block
    //
    // 2. An argument corresponding to a block parameter for this block is still live in/through
    //    this block. Due to values being renamed when used as block arguments, we must ensure there
    //    is a new copy of the argument so that the original value, and the renamed alias, are both
    //    live simultaneously. If there is insufficient operand stack space to accommodate both,
    //    then we must spill values from W to make room.
    //
    // So in short, we post-process `to_reload` by matching any values in the set which are block
    // parameters, with the corresponding source values in W^exit(P) (issuing reloads if the value
    // given as argument in the predecessor is not in W^exit(P))
    let pred_args = match function.dfg.analyze_branch(pred.inst) {
        BranchInfo::SingleDest(_, pred_args) => pred_args,
        BranchInfo::MultiDest(jts) => jts
            .iter()
            .find_map(|jt| {
                if jt.destination == block_info.block_id {
                    Some(jt.args)
                } else {
                    None
                }
            })
            .unwrap(),
        BranchInfo::NotABranch => unreachable!(),
    };

    // Remove block params from `to_reload`, and replace them, as needed, with reloads of the value
    // in the predecessor which was used as the successor argument
    for (i, param) in function.dfg.block_params(block_info.block_id).iter().enumerate() {
        to_reload.remove(param);
        // Match up this parameter with its source argument, and if the source value is not in
        // W^exit(P), then a reload is needed
        let src = pred_args[i];
        if !w_exitp.contains(&src) {
            to_reload.insert(Operand::new(src, function));
        }
    }

    // If there are no reloads or spills needed, we're done
    if to_reload.is_empty() && to_spill.is_empty() {
        return;
    }

    // Otherwise, we need to split the edge from P to B, and place any spills/reloads in the split,
    // S, moving any block arguments for B, to the unconditional branch in S.
    let split = analysis.split(block_info.block_id, *pred);
    let place = Placement::Split(split);
    let span = function.dfg.inst_span(pred.inst);

    // Insert spills first, to end the live ranges of as many variables as possible
    for spill in to_spill {
        analysis.spill(place, spill.value, span, function);
    }

    // Then insert needed reloads
    for reload in to_reload {
        analysis.reload(place, reload.value, span);
    }
}

/// The MIN algorithm is used to compute the spills and reloads to insert at each instruction in a
/// block, so as to ensure that there is sufficient space to hold all instruction operands and
/// results without exceeding K elements on the operand stack simultaneously, and allocating spills
/// so as to minimize the number of live ranges needing to be split.
///
/// MIN will spill values with the greatest next-use distance first, using the size of the operand
/// as a tie-breaker for values with equidistant next uses (i.e. the larger values get spilled
/// first, thus making more room on the operand stack).
///
/// It is expected that upon entry to a given block, that the W and S sets are accurate, regardless
/// of which predecessor edge was used to reach the block. This is handled earlier during analysis
/// by computing the necessary spills and reloads to be inserted along each control flow edge, as
/// required.
fn min(
    current_inst: Inst,
    w: &mut SmallSet<Operand, 4>,
    s: &mut SmallSet<Operand, 4>,
    analysis: &mut SpillAnalysis,
    function: &Function,
    liveness: &LivenessAnalysis,
) {
    let current_pp = ProgramPoint::Inst(current_inst);
    let ip = InsertionPoint::before(current_pp);
    let place = Placement::At(ip);
    let span = function.dfg.inst_span(current_inst);
    let opcode = function.dfg.inst(current_inst).opcode();
    let args = function.dfg.inst_args(current_inst);
    match function.dfg.analyze_branch(current_inst) {
        BranchInfo::NotABranch if opcode.is_terminator() => {
            // A non-branching terminator is either a return, or an unreachable.
            // In the latter case, there are no operands or results, so there is no
            // effect on W or S In the former case, the operands to the instruction are
            // the "results" from the perspective of the operand stack, so we are simply
            // ensuring that those values are in W by issuing reloads as necessary, all
            // other values are dead, so we do not actually issue any spills.
            w.retain(|o| liveness.is_live_at(&o.value, current_pp));
            let to_reload = args.iter().map(|v| Operand::new(*v, function));
            for reload in to_reload {
                if w.insert(Operand::new(reload.value, function)) {
                    analysis.reload(place, reload.value, span);
                }
            }
        }
        // All other instructions are handled more or less identically according to the effects
        // an instruction has, as described in the documentation of the MIN algorithm.
        //
        // In the case of branch instructions, successor arguments are not considered inputs to
        // the instruction. Instead, we handle spills/reloads for each control flow edge
        // independently, as if they occur on exit from this instruction. The result is that
        // we may or may not have all successor arguments in W on exit from I, but by the time
        // each successor block is reached, all block parameters are guaranteed to be in W
        branch_info => {
            let mut to_reload = args
                .iter()
                .map(|v| Operand::new(*v, function))
                .collect::<SmallVec<[Operand; 2]>>();

            // Remove the first occurrance of any operand already in W, remaining uses
            // must be considered against the stack usage calculation (but will not
            // actually be reloaded)
            for operand in w.iter() {
                if let Some(pos) = to_reload.iter().position(|o| o == operand) {
                    to_reload.swap_remove(pos);
                }
            }

            // Precompute the starting stack usage of W
            let w_used = w.iter().map(|o| o.size as usize).sum::<usize>();

            // Compute the needed operand stack space for all operands not currently
            // in W, i.e. those which must be reloaded from a spill slot
            let in_needed = to_reload.iter().map(|o| o.size as usize).sum::<usize>();

            // Compute the needed operand stack space for results of I
            let results = function.dfg.inst_results(current_inst);
            let out_needed = results
                .iter()
                .map(|v| function.dfg.value_type(*v).size_in_felts())
                .sum::<usize>();

            // Compute the amount of operand stack space needed for operands which are
            // not live across the instruction, i.e. which do not consume stack space
            // concurrently with the results.
            let in_consumed = args
                .iter()
                .filter_map(|v| {
                    if liveness.is_live_after(v, current_pp) {
                        None
                    } else {
                        Some(function.dfg.value_type(*v).size_in_felts())
                    }
                })
                .sum::<usize>();

            // If we have room for operands and results in W, then no spills are needed,
            // otherwise we require two passes to compute the spills we will need to issue
            let mut to_spill = SmallSet::<Operand, 4>::default();

            // First pass: compute spills for entry to I (making room for operands)
            //
            // The max usage in is determined by the size of values currently in W, plus the size
            // of any duplicate operands (i.e. values used as operands more than once), as well as
            // the size of any operands which must be reloaded.
            let max_usage_in = w_used + in_needed;
            if max_usage_in > K {
                // We must spill enough capacity to keep K >= 16
                let mut must_spill = max_usage_in - K;
                // Our initial set of candidates consists of values in W which are not operands
                // of the current instruction.
                let mut candidates =
                    w.iter().filter(|o| !args.contains(&o.value)).cloned().collect::<Vec<_>>();
                // We order the candidates such that those whose next-use distance is greatest, are
                // placed last, and thus will be selected first. We further break ties between
                // values with equal next-use distances by ordering them by the
                // effective size on the operand stack, so that larger values are
                // spilled first.
                candidates.sort_by(|a, b| {
                    let a_dist = liveness.next_use_after(&a.value, current_pp);
                    let b_dist = liveness.next_use_after(&b.value, current_pp);
                    a_dist.cmp(&b_dist).then(a.size.cmp(&b.size))
                });
                // Spill until we have made enough room
                while must_spill > 0 {
                    let candidate = candidates.pop().unwrap_or_else(|| {
                        panic!(
                            "unable to spill sufficient capacity to hold all operands on stack at \
                             one time at {current_inst}"
                        )
                    });
                    must_spill = must_spill.saturating_sub(candidate.size as usize);
                    to_spill.insert(candidate);
                }
            }

            // Second pass: compute spills for exit from I (making room for results)
            let spilled = to_spill.iter().map(|o| o.size as usize).sum::<usize>();
            // The max usage out is computed by adding the space required for all results of I, to
            // the max usage in, then subtracting the size of all operands which are consumed by I,
            // as well as the size of those values in W which we have spilled.
            let max_usage_out = (max_usage_in + out_needed).saturating_sub(in_consumed + spilled);
            if max_usage_out > K {
                // We must spill enough capacity to keep K >= 16
                let mut must_spill = max_usage_out - K;
                // For this pass, the set of candidates consists of values in W which are not
                // operands of I, and which have not been spilled yet, as well as values in W
                // which are operands of I that are live-after I. The latter group may sound
                // contradictory, how can you spill something before it is used? However, what
                // is actually happening is that we spill those values before I, so that we
                // can treat those values as being "consumed" by I, such that their space in W
                // can be reused by the results of I.
                let mut candidates = w
                    .iter()
                    .filter(|o| {
                        if !args.contains(&o.value) {
                            // Not an argument, not yet spilled
                            !to_spill.contains(*o)
                        } else {
                            // A spillable argument
                            liveness.is_live_after(&o.value, current_pp)
                        }
                    })
                    .cloned()
                    .collect::<Vec<_>>();
                candidates.sort_by(|a, b| {
                    let a_dist = liveness.next_use_after(&a.value, current_pp);
                    let b_dist = liveness.next_use_after(&b.value, current_pp);
                    a_dist.cmp(&b_dist).then(a.size.cmp(&b.size))
                });
                while must_spill > 0 {
                    let candidate = candidates.pop().unwrap_or_else(|| {
                        panic!(
                            "unable to spill sufficient capacity to hold all operands on stack at \
                             one time at {current_inst}"
                        )
                    });
                    // If we're spilling an operand of I, we can multiple the amount of space
                    // freed by the spill by the number of uses of the spilled value in I
                    let num_uses =
                        core::cmp::max(1, args.iter().filter(|v| *v == &candidate.value).count());
                    let freed = candidate.size as usize * num_uses;
                    must_spill = must_spill.saturating_sub(freed);
                    to_spill.insert(candidate);
                }
            }

            // Emit spills first, to make space for reloaded values on the operand stack
            for spill in to_spill.iter() {
                if s.insert(*spill) {
                    analysis.spill(place, spill.value, span, function);
                }

                // Remove spilled values from W
                w.remove(spill);
            }

            // Emit reloads for those operands of I not yet in W
            for reload in to_reload {
                // We only need to emit a reload for a given value once
                if w.insert(reload) {
                    // By definition, if we are emitting a reload, the value must have been spilled
                    s.insert(reload);
                    analysis.reload(place, reload.value, span);
                }
            }

            // At this point, we've emitted our spills/reloads, so we need to prepare W for the next
            // instruction by applying the effects of the instruction to W, i.e. consuming those
            // operands which are consumed, and adding instruction results.
            //
            // First, we remove operands from W which are no longer live-after I, _except_ any
            // which are used as successor arguments. This is because we must know which successor
            // arguments are still in W at the block terminator when we are computing what to spill
            // or reload along each control flow edge.
            //
            // Second, if applicable, we add in the instruction results
            match branch_info {
                BranchInfo::NotABranch => {
                    w.retain(|o| liveness.is_live_after(&o.value, current_pp));
                    w.extend(results.iter().map(|v| Operand::new(*v, function)));
                }
                BranchInfo::SingleDest(_, succ_args) => {
                    w.retain(|o| {
                        succ_args.contains(&o.value) || liveness.is_live_after(&o.value, current_pp)
                    });
                }
                BranchInfo::MultiDest(jts) => {
                    w.retain(|o| {
                        let is_succ_arg = jts.iter().any(|jt| jt.args.contains(&o.value));
                        is_succ_arg || liveness.is_live_after(&o.value, current_pp)
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use midenc_hir::{
        testing::TestContext, AbiParam, FunctionBuilder, Immediate, InstBuilder, Signature,
    };
    use pretty_assertions::assert_eq;

    use super::*;

    /// In this test, we force several values to be live simultaneously inside the same block,
    /// of sufficient size on the operand stack so as to require some of them to be spilled
    /// at least once, and later reloaded.
    ///
    /// The purpose here is to validate the MIN algorithm that determines whether or not we need
    /// to spill operands at each program point, in the following ways:
    ///
    /// * Ensure that we spill values we expect to be spilled
    /// * Ensure that spills are inserted at the appropriate locations
    /// * Ensure that we reload values that were previously spilled
    /// * Ensure that reloads are inserted at the appropriate locations
    ///
    /// The following HIR is constructed for this test:
    ///
    /// * `in` indicates the set of values in W at an instruction, with reloads included
    /// * `out` indicates the set of values in W after an instruction, with spills excluded
    /// * `spills` indicates the candidates from W which were selected to be spilled at the
    ///   instruction
    /// * `reloads` indicates the set of values in S which must be reloaded at the instruction
    ///
    /// ```text,ignore
    /// (func (export #spill) (param (ptr u8)) (result u32)
    ///   (block 0 (param v0 (ptr u8))
    ///     (let (v1 u32) (ptrtoint v0))              ; in=[v0] out=[v1]
    ///     (let (v2 u32) (add v1 32))                ; in=[v1] out=[v1 v2]
    ///     (let (v3 (ptr u128)) (inttoptr v2))       ; in=[v1 v2] out=[v1 v2 v3]
    ///     (let (v4 u128) (load v3))                 ; in=[v1 v2 v3] out=[v1 v2 v3 v4]
    ///     (let (v5 u32) (add v1 64))                ; in=[v1 v2 v3 v4] out=[v1 v2 v3 v4 v5]
    ///     (let (v6 (ptr u128)) (inttoptr v5))       ; in=[v1 v2 v3 v4 v5] out=[v1 v2 v3 v4 v6]
    ///     (let (v7 u128) (load v6))                 ; in=[v1 v2 v3 v4 v6] out=[v1 v2 v3 v4 v6 v7]
    ///     (let (v8 u64) (const.u64 1))              ; in=[v1 v2 v3 v4 v6 v7] out=[v1 v2 v3 v4 v6 v7 v8]
    ///     (let (v9 u32) (call (#example) v6 v4 v7 v7 v8)) <-- operand stack pressure hits 18 here
    ///                                               ; in=[v1 v2 v3 v4 v6 v7 v7 v8] out=[v1 v7 v9]
    ///                                               ; spills=[v2 v3] (v2 is furthest next-use, followed by v3)
    ///     (let (v10 u32) (add v1 72))               ; in=[v1 v7] out=[v7 v10]
    ///     (let (v11 (ptr u64)) (inttoptr v10))      ; in=[v7 v10] out=[v7 v11]
    ///     (store v3 v7)                             ; reload=[v3] in=[v3 v7 v11] out=[v11]
    ///     (let (v12 u64) (load v11))                ; in=[v11] out=[v12]
    ///     (ret v2)                                  ; reload=[v2] in=[v2] out=[v2]
    /// )
    /// ```
    #[test]
    fn spills_intra_block() -> AnalysisResult<()> {
        let context = TestContext::default();
        let id = "test::spill".parse().unwrap();
        let mut function = Function::new(
            id,
            Signature::new(
                [AbiParam::new(Type::Ptr(Box::new(Type::U8)))],
                [AbiParam::new(Type::U32)],
            ),
        );

        let v2;
        let v3;
        let call;
        let store;
        let ret;
        {
            let mut builder = FunctionBuilder::new(&mut function);
            let example = builder
                .import_function(
                    "foo",
                    "example",
                    Signature::new(
                        [
                            AbiParam::new(Type::Ptr(Box::new(Type::U128))),
                            AbiParam::new(Type::U128),
                            AbiParam::new(Type::U128),
                            AbiParam::new(Type::U128),
                            AbiParam::new(Type::U64),
                        ],
                        [AbiParam::new(Type::U32)],
                    ),
                    SourceSpan::UNKNOWN,
                )
                .unwrap();
            let entry = builder.current_block();
            let v0 = {
                let args = builder.block_params(entry);
                args[0]
            };

            // entry
            let v1 = builder.ins().ptrtoint(v0, Type::U32, SourceSpan::UNKNOWN);
            v2 = builder.ins().add_imm_unchecked(v1, Immediate::U32(32), SourceSpan::UNKNOWN);
            v3 = builder.ins().inttoptr(v2, Type::Ptr(Box::new(Type::U128)), SourceSpan::UNKNOWN);
            let v4 = builder.ins().load(v3, SourceSpan::UNKNOWN);
            let v5 = builder.ins().add_imm_unchecked(v1, Immediate::U32(64), SourceSpan::UNKNOWN);
            let v6 =
                builder.ins().inttoptr(v5, Type::Ptr(Box::new(Type::U128)), SourceSpan::UNKNOWN);
            let v7 = builder.ins().load(v6, SourceSpan::UNKNOWN);
            let v8 = builder.ins().u64(1, SourceSpan::UNKNOWN);
            call = builder.ins().call(example, &[v6, v4, v7, v7, v8], SourceSpan::UNKNOWN);
            let v10 = builder.ins().add_imm_unchecked(v1, Immediate::U32(72), SourceSpan::UNKNOWN);
            store = builder.ins().store(v3, v7, SourceSpan::UNKNOWN);
            let v11 =
                builder.ins().inttoptr(v10, Type::Ptr(Box::new(Type::U64)), SourceSpan::UNKNOWN);
            let _v12 = builder.ins().load(v11, SourceSpan::UNKNOWN);
            ret = builder.ins().ret(Some(v2), SourceSpan::UNKNOWN);
        }

        let mut analyses = AnalysisManager::default();
        let spills = analyses.get_or_compute::<SpillAnalysis>(&function, &context.session)?;

        assert!(spills.has_spills());
        assert_eq!(spills.spills().len(), 2);
        assert!(spills.is_spilled(&v2));
        assert!(spills.is_spilled_at(v2, ProgramPoint::Inst(call)));
        assert!(spills.is_spilled(&v3));
        assert!(spills.is_spilled_at(v3, ProgramPoint::Inst(call)));
        assert_eq!(spills.reloads().len(), 2);
        assert!(spills.is_reloaded_at(v3, ProgramPoint::Inst(store)));
        assert!(spills.is_reloaded_at(v2, ProgramPoint::Inst(ret)));

        Ok(())
    }

    /// In this test, we are verifying the behavior of the spill analysis when applied to a
    /// control flow graph with branching control flow, where spills are required along one
    /// branch and not the other. This verifies the following:
    ///
    /// * Control flow edges are split as necessary to insert required spills/reloads
    /// * Propagation of the W and S sets from predecessors to successors is correct
    /// * The W and S sets are properly computed at join points in the CFG
    ///
    /// The following HIR is constructed for this test (see the first test in this file for
    /// a description of the notation used, if unclear):
    ///
    /// ```text,ignore
    /// (func (export #spill) (param (ptr u8)) (result u32)
    ///   (block 0 (param v0 (ptr u8))
    ///     (let (v1 u32) (ptrtoint v0))              ; in=[v0] out=[v1]
    ///     (let (v2 u32) (add v1 32))                ; in=[v1] out=[v1 v2]
    ///     (let (v3 (ptr u128)) (inttoptr v2))       ; in=[v1 v2] out=[v1 v2 v3]
    ///     (let (v4 u128) (load v3))                 ; in=[v1 v2 v3] out=[v1 v2 v3 v4]
    ///     (let (v5 u32) (add v1 64))                ; in=[v1 v2 v3 v4] out=[v1 v2 v3 v4 v5]
    ///     (let (v6 (ptr u128)) (inttoptr v5))       ; in=[v1 v2 v3 v4 v5] out=[v1 v2 v3 v4 v6]
    ///     (let (v7 u128) (load v6))                 ; in=[v1 v2 v3 v4 v6] out=[v1 v2 v3 v4 v6 v7]
    ///     (let (v8 i1) (eq v1 0))                   ; in=[v1 v2 v3 v4 v6, v7] out=[v1 v2 v3 v4 v6 v7, v8]
    ///     (cond_br v8 (block 1) (block 2)))
    ///
    ///   (block 1
    ///     (let (v9 u64) (const.u64 1))              ; in=[v1 v2 v3 v4 v6 v7] out=[v1 v2 v3 v4 v6 v7 v9]
    ///     (let (v10 u32) (call (#example) v6 v4 v7 v7 v9)) <-- operand stack pressure hits 18 here
    ///                                               ; in=[v1 v2 v3 v4 v6 v7 v7 v9] out=[v1 v7 v10]
    ///                                               ; spills=[v2 v3] (v2 is furthest next-use, followed by v3)
    ///     (br (block 3 v10))) ; this edge will be split to reload v2/v3 as expected by block3
    ///
    ///   (block 2
    ///     (let (v11 u32) (add v1 8))                ; in=[v1 v2 v3 v7] out=[v1 v2 v3 v7 v11]
    ///     (br (block 3 v11))) ; this edge will be split to spill v2/v3 to match the edge from block1
    ///
    ///   (block 3 (param v12 u32)) ; we expect that the edge between block 2 and 3 will be split, and spills of v2/v3 inserted
    ///     (let (v13 u32) (add v1 72))               ; in=[v1 v7 v12] out=[v7 v12 v13]
    ///     (let (v14 u32) (add v13 v12))             ; in=[v7 v12 v13] out=[v7 v14]
    ///     (let (v15 (ptr u64)) (inttoptr v14))      ; in=[v7 v14] out=[v7 v15]
    ///     (store v3 v7)                             ; reload=[v3] in=[v3 v7 v15] out=[v15]
    ///     (let (v16 u64) (load v15))                ; in=[v15] out=[v16]
    ///     (ret v2))                                 ; reload=[v2] in=[v2] out=[v2]
    /// )
    /// ```
    #[test]
    fn spills_branching_control_flow() -> AnalysisResult<()> {
        let context = TestContext::default();
        let id = "test::spill".parse().unwrap();
        let mut function = Function::new(
            id,
            Signature::new(
                [AbiParam::new(Type::Ptr(Box::new(Type::U8)))],
                [AbiParam::new(Type::U32)],
            ),
        );

        let v2;
        let v3;
        let call;
        {
            let mut builder = FunctionBuilder::new(&mut function);
            let example = builder
                .import_function(
                    "foo",
                    "example",
                    Signature::new(
                        [
                            AbiParam::new(Type::Ptr(Box::new(Type::U128))),
                            AbiParam::new(Type::U128),
                            AbiParam::new(Type::U128),
                            AbiParam::new(Type::U128),
                            AbiParam::new(Type::U64),
                        ],
                        [AbiParam::new(Type::U32)],
                    ),
                    SourceSpan::UNKNOWN,
                )
                .unwrap();
            let entry = builder.current_block();
            let block1 = builder.create_block();
            let block2 = builder.create_block();
            let block3 = builder.create_block();
            let v0 = {
                let args = builder.block_params(entry);
                args[0]
            };

            // entry
            let v1 = builder.ins().ptrtoint(v0, Type::U32, SourceSpan::UNKNOWN);
            v2 = builder.ins().add_imm_unchecked(v1, Immediate::U32(32), SourceSpan::UNKNOWN);
            v3 = builder.ins().inttoptr(v2, Type::Ptr(Box::new(Type::U128)), SourceSpan::UNKNOWN);
            let v4 = builder.ins().load(v3, SourceSpan::UNKNOWN);
            let v5 = builder.ins().add_imm_unchecked(v1, Immediate::U32(64), SourceSpan::UNKNOWN);
            let v6 =
                builder.ins().inttoptr(v5, Type::Ptr(Box::new(Type::U128)), SourceSpan::UNKNOWN);
            let v7 = builder.ins().load(v6, SourceSpan::UNKNOWN);
            let v8 = builder.ins().eq_imm(v1, Immediate::U32(0), SourceSpan::UNKNOWN);
            builder.ins().cond_br(v8, block1, &[], block2, &[], SourceSpan::UNKNOWN);

            // block1
            builder.switch_to_block(block1);
            let v9 = builder.ins().u64(1, SourceSpan::UNKNOWN);
            call = builder.ins().call(example, &[v6, v4, v7, v7, v9], SourceSpan::UNKNOWN);
            let v10 = builder.func.dfg.first_result(call);
            builder.ins().br(block3, &[v10], SourceSpan::UNKNOWN);

            // block2
            builder.switch_to_block(block2);
            let v11 = builder.ins().add_imm_unchecked(v1, Immediate::U32(8), SourceSpan::UNKNOWN);
            builder.ins().br(block3, &[v11], SourceSpan::UNKNOWN);

            // block3
            let v12 = builder.append_block_param(block3, Type::U32, SourceSpan::UNKNOWN);
            builder.switch_to_block(block3);
            let v13 = builder.ins().add_imm_unchecked(v1, Immediate::U32(72), SourceSpan::UNKNOWN);
            let v14 = builder.ins().add_unchecked(v13, v12, SourceSpan::UNKNOWN);
            let v15 =
                builder.ins().inttoptr(v14, Type::Ptr(Box::new(Type::U64)), SourceSpan::UNKNOWN);
            builder.ins().store(v3, v7, SourceSpan::UNKNOWN);
            let _v16 = builder.ins().load(v15, SourceSpan::UNKNOWN);
            builder.ins().ret(Some(v2), SourceSpan::UNKNOWN);
        }

        let mut analyses = AnalysisManager::default();
        let spills = analyses.get_or_compute::<SpillAnalysis>(&function, &context.session)?;

        dbg!(&spills);

        assert!(spills.has_spills());
        assert_eq!(spills.spills().len(), 4);
        // Block created by splitting edge between block 3 and its predecessors
        assert_eq!(spills.splits().len(), 2);
        let block1 = Block::from_u32(1);
        let block2 = Block::from_u32(2);
        let block3 = Block::from_u32(3);
        let spill_blk1_blk3 = &spills.splits()[0];
        assert_eq!(spill_blk1_blk3.block, block3);
        assert_eq!(spill_blk1_blk3.predecessor.block, block1);
        let spill_blk2_blk3 = &spills.splits()[1];
        assert_eq!(spill_blk2_blk3.block, block3);
        assert_eq!(spill_blk2_blk3.predecessor.block, block2);
        assert!(spills.is_spilled(&v2));
        assert!(spills.is_spilled_at(v2, ProgramPoint::Inst(call)));
        // v2 should have a spill inserted from block2 to block3, as it is spilled in block1
        assert!(spills.is_spilled_in_split(v2, spill_blk2_blk3.id));
        assert!(spills.is_spilled(&v3));
        // v3 should have a spill inserted from block2 to block3, as it is spilled in block1
        assert!(spills.is_spilled_at(v3, ProgramPoint::Inst(call)));
        assert!(spills.is_spilled_in_split(v3, spill_blk2_blk3.id));
        // v2 and v3 should be reloaded on the edge from block1 to block3, since they were
        // spilled previously, but must be in W on entry to block3
        assert_eq!(spills.reloads().len(), 2);
        assert!(spills.is_reloaded_in_split(v2, spill_blk1_blk3.id));
        assert!(spills.is_reloaded_in_split(v3, spill_blk1_blk3.id));

        Ok(())
    }

    /// In this test, we are verifying the behavior of the spill analysis when applied to a
    /// control flow graph with cyclical control flow, i.e. loops. We're interested specifically in
    /// the following properties:
    ///
    /// * W and S at entry to a loop are computed correctly
    /// * Values live-through - but not live-in - a loop, which cannot survive the loop due to
    ///   operand stack pressure within the loop, are spilled outside of the loop, with reloads
    ///   placed on exit edges from the loop where needed
    /// * W and S upon exit from a loop are computed correctly
    ///
    /// The following HIR is constructed for this test (see the first test in this file for
    /// a description of the notation used, if unclear):
    ///
    /// ```text,ignore
    /// (func (export #spill) (param (ptr u64)) (param u32) (param u32) (result u64)
    ///   (block 0 (param v0 (ptr u64)) (param v1 u32) (param v2 u32)
    ///     (let (v3 u32) (const.u32 0))         ; in=[v0, v1, v2] out=[v0, v1, v2, v3]
    ///     (let (v4 u32) (const.u32 0))         ; in=[v0, v1, v2, v3] out=[v0, v1, v2, v3, v4]
    ///     (let (v5 u64) (const.u64 0))         ; in=[v0, v1, v2, v3, v4] out=[v0, v1, v2, v3, v4, v5]
    ///     (br (block 1 v3 v4 v5)))
    ///
    ///   (block 1 (param v6 u32) (param v7 u32) (param v8 u64)) ; outer loop
    ///     (let (v9 i1) (eq v6 v1))             ; in=[v0, v2, v6, v7, v8] out=[v0, v1, v2, v6, v7, v8, v9]
    ///     (cond_br v9 (block 2) (block 3)))    ; in=[v0, v1, v2, v6, v7, v8, v9] out=[v0, v1, v2, v6, v7, v8]
    ///
    ///   (block 2 ; exit outer loop, return from function
    ///     (ret v8))                            ; in=[v0, v1, v2, v6, v7, v8] out=[v8]
    ///
    ///   (block 3 ; split edge
    ///     (br (block 4 v7 v8)))                ; in=[v0, v1, v2, v6, v7, v8] out=[v0, v1, v2, v6]
    ///
    ///   (block 4 (param v10 u32) (param v11 u64) ; inner loop
    ///     (let (v12 i1) (eq v10 v2))           ; in=[v0, v1, v2, v6, v10, v11] out=[v0, v1, v2, v6, v10, v11, v12]
    ///     (cond_br v12 (block 5) (block 6)))   ; in=[v0, v1, v2, v6, v10, v11, v12] out=[v0, v1, v2, v6, v10, v11]
    ///
    ///   (block 5 ; increment row count, continue outer loop
    ///     (let (v13 u32) (add v6 1))           ; in=[v0, v1, v2, v6, v10, v11] out=[v0, v1, v2, v10, v11, v13]
    ///     (br (block 1 v13 v10 v11)))
    ///
    ///   (block 6 ; load value at v0[row][col], add to sum, increment col, continue inner loop
    ///     (let (v14 u32) (sub.saturating v6 1)) ; row_offset := ROW_SIZE * row.saturating_sub(1)
    ///                                           ; in=[v0, v1, v2, v6, v10, v11] out=[v0, v1, v2, v6, v10, v11, 14]
    ///     (let (v15 u32) (mul v14 v2))          ; in=[v0, v1, v2, v6, v10, v11, 14] out=[v0, v1, v2, v6, v10, v11, 15]
    ///     (let (v16 u32) (add v10 v15))         ; offset := col + row_offset
    ///                                           ; in=[v0, v1, v2, v6, v10, v11, 15] out=[v0, v1, v2, v6, v10, v11, v16]
    ///     (let (v17 u32) (ptrtoint v0))         ; ptr := (v0 as u32 + offset) as *u64
    ///                                           ; in=[v0, v1, v2, v6, v10, v11, v16] out=[v0, v1, v2, v6, v10, v11, v16, 17]
    ///     (let (v18 u32) (add v17 v16))         ; in=[v0, v1, v2, v6, v10, v11, v16, v17] out=[v0, v1, v2, v6, v10, v11, v18]
    ///     (let (v19 (ptr u64)) (ptrtoint v18))  ; in=[v0, v1, v2, v6, v10, v11, v18] out=[v0, v1, v2, v6, v10, v11, v19]
    ///     (let (v20 u64) (load v19))            ; sum += *ptr
    ///                                           ; in=[v0, v1, v2, v6, v10, v11, v19] out=[v0, v1, v2, v6, v10, v11, v20]
    ///     (let (v21 u64) (add v11 v20))         ; in=[v0, v1, v2, v6, v10, v11, v20] out=[v0, v1, v2, v6, v10, v21]
    ///     (let (v22 u32) (add v10 1))           ; col++
    ///                                           ; in=[v0, v1, v2, v6, v10, v21] out=[v0, v1, v2, v6, v21, v22]
    ///     (br (block 4 v22 v21)))
    /// )
    /// ```
    #[test]
    fn spills_loop_nest() -> AnalysisResult<()> {
        let context = TestContext::default();
        let id = "test::spill".parse().unwrap();
        let mut function = Function::new(
            id,
            Signature::new(
                [
                    AbiParam::new(Type::Ptr(Box::new(Type::U64))),
                    AbiParam::new(Type::U64),
                    AbiParam::new(Type::U64),
                ],
                [AbiParam::new(Type::U64)],
            ),
        );

        {
            let mut builder = FunctionBuilder::new(&mut function);
            let entry = builder.current_block();
            let (v0, v1, v2) = {
                let args = builder.block_params(entry);
                (args[0], args[1], args[2])
            };

            let block1 = builder.create_block();
            let block2 = builder.create_block();
            let block3 = builder.create_block();
            let block4 = builder.create_block();
            let block5 = builder.create_block();
            let block6 = builder.create_block();

            // entry
            let v3 = builder.ins().u64(0, SourceSpan::UNKNOWN);
            let v4 = builder.ins().u64(0, SourceSpan::UNKNOWN);
            let v5 = builder.ins().u64(0, SourceSpan::UNKNOWN);
            builder.ins().br(block1, &[v3, v4, v5], SourceSpan::UNKNOWN);

            // block1 - outer loop header
            let v6 = builder.append_block_param(block1, Type::U64, SourceSpan::UNKNOWN);
            let v7 = builder.append_block_param(block1, Type::U64, SourceSpan::UNKNOWN);
            let v8 = builder.append_block_param(block1, Type::U64, SourceSpan::UNKNOWN);
            builder.switch_to_block(block1);
            let v9 = builder.ins().eq(v6, v1, SourceSpan::UNKNOWN);
            builder.ins().cond_br(v9, block2, &[], block3, &[], SourceSpan::UNKNOWN);

            // block2 - outer exit
            builder.switch_to_block(block2);
            builder.ins().ret(Some(v8), SourceSpan::UNKNOWN);

            // block3 - split edge
            builder.switch_to_block(block3);
            builder.ins().br(block4, &[v7, v8], SourceSpan::UNKNOWN);

            // block4 - inner loop
            let v10 = builder.append_block_param(block4, Type::U64, SourceSpan::UNKNOWN);
            let v11 = builder.append_block_param(block4, Type::U64, SourceSpan::UNKNOWN);
            builder.switch_to_block(block4);
            let v12 = builder.ins().eq(v10, v2, SourceSpan::UNKNOWN);
            builder.ins().cond_br(v12, block5, &[], block6, &[], SourceSpan::UNKNOWN);

            // block5 - inner latch
            builder.switch_to_block(block5);
            let v13 = builder.ins().add_imm_unchecked(v6, Immediate::U64(1), SourceSpan::UNKNOWN);
            builder.ins().br(block1, &[v13, v10, v11], SourceSpan::UNKNOWN);

            // block6 - inner body
            builder.switch_to_block(block6);
            let v14 = builder.ins().add_imm_unchecked(v6, Immediate::U64(1), SourceSpan::UNKNOWN);
            let v15 = builder.ins().mul_unchecked(v14, v2, SourceSpan::UNKNOWN);
            let v16 = builder.ins().add_unchecked(v10, v15, SourceSpan::UNKNOWN);
            let v17 = builder.ins().ptrtoint(v0, Type::U64, SourceSpan::UNKNOWN);
            let v18 = builder.ins().add_unchecked(v17, v16, SourceSpan::UNKNOWN);
            let v19 =
                builder.ins().inttoptr(v18, Type::Ptr(Box::new(Type::U64)), SourceSpan::UNKNOWN);
            let v20 = builder.ins().load(v19, SourceSpan::UNKNOWN);
            let v21 = builder.ins().add_unchecked(v11, v20, SourceSpan::UNKNOWN);
            let v22 = builder.ins().add_imm_unchecked(v10, Immediate::U64(1), SourceSpan::UNKNOWN);
            builder.ins().br(block4, &[v22, v21], SourceSpan::UNKNOWN);
        }

        let mut analyses = AnalysisManager::default();
        let spills = analyses.get_or_compute::<SpillAnalysis>(&function, &context.session)?;

        dbg!(&spills);

        let block1 = Block::from_u32(1);
        let block3 = Block::from_u32(3);
        let block4 = Block::from_u32(4);
        let block5 = Block::from_u32(5);

        assert!(spills.has_spills());
        assert_eq!(spills.splits().len(), 2);

        // We expect a spill from block3 to block4, as due to operand stack pressure in the loop,
        // there is insufficient space to keep v1 on the operand stack through the loop
        assert_eq!(spills.spills().len(), 1);
        let split_blk3_blk4 = &spills.splits()[0];
        assert_eq!(split_blk3_blk4.block, block4);
        assert_eq!(split_blk3_blk4.predecessor.block, block3);
        let v1 = Value::from_u32(1);
        assert!(spills.is_spilled(&v1));
        assert!(spills.is_spilled_in_split(v1, split_blk3_blk4.id));

        // We expect a reload of v1 from block5 to block1, as block1 expects v1 on the operand stack
        assert_eq!(spills.reloads().len(), 1);
        let split_blk5_blk1 = &spills.splits()[1];
        assert_eq!(split_blk5_blk1.block, block1);
        assert_eq!(split_blk5_blk1.predecessor.block, block5);
        assert!(spills.is_reloaded(&v1));
        assert!(spills.is_reloaded_in_split(v1, split_blk5_blk1.id));

        Ok(())
    }
}
