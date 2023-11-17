use std::{
    cmp,
    collections::{BTreeMap, VecDeque},
};

use miden_hir::pass::{Analysis, AnalysisManager, AnalysisResult, PreservedAnalyses};
use miden_hir::{self as hir, Block as BlockId, Inst as InstId, Value as ValueId, *};
use midenc_session::Session;
use rustc_hash::{FxHashMap, FxHashSet};

use super::{ControlFlowGraph, DominatorTree, LoopAnalysis};

/// This data structure is the result of computing liveness information over the
/// control flow graph of an HIR function. It uses a somewhat novel approach to
/// liveness that is ideally suited to optimal register allocation over programs
/// in SSA form, as described in _Register Spilling and Live-Range Splitting for
/// SSA-Form Programs_, by Matthias Braun and Sebastian Hack.
///
/// In short, rather than simply tracking what values are live at a particular
/// program point, we instead track the global next-use distance for all values
/// which are live at a given program point. Just like the more typical liveness
/// analysis, the next-use analysis computes next-use distances for a block by
/// taking the next-use distances at the exit of the block, and working backwards
/// to compute the next-use distances at the entry of the block. When computing
/// the next-use distances at the exit of a block with multiple successors, the
/// join function takes the minimum next-use distances of all variables live across
/// some edge.
///
/// By default, variables introduced by an instruction (i.e. their definition) are
/// assigned the next-use distance at the next instruction (or the block exit). If
/// no use of the variable has been observed, it is assigned a next-use distance of
/// infinity (here represented as `u32::MAX`). Each time we step backwards up through
/// the block, we increment the distance of all values observed so far by 1. If we
/// encounter a use of a variable at the current instruction, its next-use distance
/// is set to 0.
///
/// When we calculate the next-use distances for the exit of a block, the set is
/// initialized by taking the join of the next-use distances at the entry of all
/// successors of that block, or the empty set if there are no successors. However,
/// if the current block is inside a loop, and any successor is outside that loop,
/// then all next-use distances obtained from that successor are incremented by a
/// large constant (1000 in our case).
///
/// The algorithm follows the standard dataflow analysis approach of working until
/// a fixpoint is reached. We start by visiting the control flow graph in reverse
/// postorder, and revisit any blocks whose results change since the last time we
/// saw that block.
///
/// The resulting data structure is ideal for register allocation, but the real benefit
/// is that it allows us to be smart about how we spill values, since it can tell us
/// how "hot" a value is, allowing us to prioritize such values over those which may
/// not be used for awhile.
#[derive(Debug, Default)]
pub struct LivenessAnalysis {
    // Liveness/global next-uses at a given program point
    live_in: FxHashMap<ProgramPoint, NextUseSet>,
    // Liveness/global next-uses after a given program point
    live_out: FxHashMap<ProgramPoint, NextUseSet>,
    // Maximum register pressure for each block
    per_block_register_pressure: FxHashMap<BlockId, usize>,
}
impl Analysis for LivenessAnalysis {
    type Entity = Function;

    fn analyze(
        function: &Self::Entity,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> AnalysisResult<Self> {
        let cfg = analyses.get_or_compute(function, session)?;
        let domtree = analyses.get_or_compute(function, session)?;
        let loops = analyses.get_or_compute(function, session)?;
        Ok(LivenessAnalysis::compute(function, &cfg, &domtree, &loops))
    }

    fn is_invalidated(&self, preserved: &PreservedAnalyses) -> bool {
        !preserved.is_preserved::<ControlFlowGraph>()
            || !preserved.is_preserved::<DominatorTree>()
            || !preserved.is_preserved::<LoopAnalysis>()
    }
}
impl LivenessAnalysis {
    /// Computes liveness for the given function, using the provided analyses
    pub fn compute(
        function: &Function,
        cfg: &ControlFlowGraph,
        domtree: &DominatorTree,
        loops: &LoopAnalysis,
    ) -> Self {
        let mut liveness = Self::default();
        liveness.recompute(function, cfg, domtree, loops);
        liveness
    }

    /// Recomputes liveness for the given function, using the provided analyses
    pub fn recompute(
        &mut self,
        function: &Function,
        cfg: &ControlFlowGraph,
        domtree: &DominatorTree,
        loops: &LoopAnalysis,
    ) {
        self.clear();
        compute_liveness(self, function, cfg, domtree, loops)
    }

    /// Clear all computed liveness information, without releasing the memory we allocated
    pub fn clear(&mut self) {
        self.live_in.clear();
        self.live_out.clear();
        self.per_block_register_pressure.clear();
    }

    /// Returns true if `value` is live at the given program point.
    ///
    /// To be live "at" means that the value is live entering that point of
    /// the program, but does not imply that it is live after that point.
    pub fn is_live_at(&self, value: &ValueId, pp: ProgramPoint) -> bool {
        self.live_in[&pp].contains(value)
    }

    /// Returns true if `value` is live after the given program point.
    ///
    /// To be live "after" means that the value is live exiting that point of
    /// the program, but does not imply that it is live before that point.
    pub fn is_live_after(&self, value: &ValueId, pp: ProgramPoint) -> bool {
        self.live_out[&pp].contains(value)
    }

    /// Returns the next-use distance of `value` at the given program point.
    pub fn next_use(&self, value: &ValueId, pp: ProgramPoint) -> u32 {
        self.live_in[&pp].distance(value)
    }

    /// Returns the next-use distance of `value` after the given program point.
    pub fn next_use_after(&self, value: &ValueId, pp: ProgramPoint) -> u32 {
        self.live_out[&pp].distance(value)
    }

    /// Returns the global next-use distances at the given program point
    pub fn next_uses(&self, pp: ProgramPoint) -> &NextUseSet {
        &self.live_in[&pp]
    }

    /// Returns the global next-use distances at the given program point
    pub fn next_uses_after(&self, pp: ProgramPoint) -> &NextUseSet {
        &self.live_out[&pp]
    }

    /// Returns an iterator over values which are live at the given program point
    pub fn live_at(&self, pp: ProgramPoint) -> impl Iterator<Item = ValueId> + '_ {
        self.live_out[&pp]
            .iter()
            .filter_map(|(v, dist)| if dist < &u32::MAX { Some(*v) } else { None })
    }

    /// Returns an iterator over values which are live after the given program point
    pub fn live_after(&self, pp: ProgramPoint) -> impl Iterator<Item = ValueId> + '_ {
        self.live_out[&pp]
            .iter()
            .filter_map(|(v, dist)| if dist < &u32::MAX { Some(*v) } else { None })
    }

    /// Returns the maximum register pressure in the given block
    pub fn max_register_pressure(&self, block: &BlockId) -> usize {
        self.per_block_register_pressure[block]
    }

    /// Returns the chromatic number of the interference graph implicit in the
    /// liveness data represented here, i.e. number of colors required to color
    /// the graph such that no two adjacent nodes share the same color, i.e. the
    /// minimum number of registers needed to perform register allocation over
    /// the function without spills.
    ///
    /// In a more practical sense, this returns the maximum number of live values
    /// at any one point in the analyzed function.
    ///
    /// # Explanation
    ///
    /// Because the interference graphs of SSA-form programs are chordal graphs,
    /// and chordal graphs are "perfect", this makes certain properties of the interference
    /// graph easy to derive. In particular, the chromatic number of a perfect graph is
    /// equal to the size of its largest clique (group of nodes which form a complete graph,
    /// i.e. have edges to each other).
    pub fn chromatic_number(&self) -> usize {
        let mut max = 0;
        for (_pp, next_used) in self.live_in.iter() {
            max = cmp::max(
                next_used.iter().filter(|(_, d)| *d < &u32::MAX).count(),
                max,
            );
        }
        max
    }
}

/// This data structure is used to maintain a mapping from variables to
/// their next-use distance at a specific program point.
///
/// If a value is not in the set, we have not observed its definition, or
/// any uses at the associated program point.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct NextUseSet(BTreeMap<ValueId, u32>);
impl FromIterator<(ValueId, u32)> for NextUseSet {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (ValueId, u32)>,
    {
        let mut set = Self::default();
        for (k, v) in iter.into_iter() {
            set.insert(k, v);
        }
        set
    }
}
impl NextUseSet {
    /// Inserts `value` in this set with the given `distance`.
    ///
    /// A distance of `u32::MAX` signifies infinite distance, which is
    /// equivalent to saying that `value` is not live.
    ///
    /// If `value` is already in this set, the distance is updated to be the
    /// lesser of the two distances, e.g. if the previous distance was `u32::MAX`,
    /// and `distance` was `1`, the entry is updated to have a distance of `1` after
    /// this function returns.
    pub fn insert(&mut self, value: ValueId, distance: u32) {
        use std::collections::btree_map::Entry;
        match self.0.entry(value) {
            Entry::Vacant(entry) => {
                entry.insert(distance);
            }
            Entry::Occupied(mut entry) => {
                let prev_distance = entry.get_mut();
                *prev_distance = std::cmp::min(*prev_distance, distance);
            }
        }
    }

    /// Returns `true` if `value` is live in this set
    #[inline]
    pub fn is_live(&self, value: &ValueId) -> bool {
        self.distance(value) < u32::MAX
    }

    /// Returns the distance to the next use of `value` as an integer.
    ///
    /// If `value` is not live, or the distance is unknown, returns `u32::MAX`
    pub fn distance(&self, value: &ValueId) -> u32 {
        self.0.get(value).copied().unwrap_or(u32::MAX)
    }

    /// Returns `true` if `value` is in this set
    #[inline]
    pub fn contains(&self, value: &ValueId) -> bool {
        self.0.contains_key(value)
    }

    /// Gets the distance associated with the given `value`, if known
    #[inline]
    pub fn get(&self, value: &ValueId) -> Option<&u32> {
        self.0.get(value)
    }

    /// Gets a mutable reference to the distance associated with the given `value`, if known
    #[inline]
    pub fn get_mut(&mut self, value: &ValueId) -> Option<&mut u32> {
        self.0.get_mut(value)
    }

    pub fn entry(
        &mut self,
        value: ValueId,
    ) -> std::collections::btree_map::Entry<'_, ValueId, u32> {
        self.0.entry(value)
    }

    /// Removes the entry for `value` from this set
    pub fn remove(&mut self, value: &ValueId) -> Option<u32> {
        self.0.remove(value)
    }

    /// Returns a new set containing the union of `self` and `other`.
    ///
    /// The resulting set will preserve the minimum distances from both sets.
    pub fn union(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for (k, v) in other.iter() {
            result.insert(*k, *v);
        }
        result
    }

    /// Returns a new set containing the intersection of `self` and `other`.
    ///
    /// The resulting set will preserve the minimum distances from both sets.
    pub fn intersection(&self, other: &Self) -> Self {
        let mut result = Self::default();
        for (k, v1) in self.iter() {
            match other.get(k) {
                None => continue,
                Some(v2) => {
                    result.0.insert(*k, core::cmp::min(*v1, *v2));
                }
            }
        }
        result
    }

    /// Returns a new set containing the symmetric difference of `self` and `other`,
    /// i.e. the values that are in `self` or `other` but not in both.
    pub fn symmetric_difference(&self, other: &Self) -> Self {
        let mut result = Self::default();
        for (k, v) in self.iter() {
            if !other.0.contains_key(k) {
                result.0.insert(*k, *v);
            }
        }
        for (k, v) in other.iter() {
            if !self.0.contains_key(k) {
                result.0.insert(*k, *v);
            }
        }
        result
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ValueId, &u32)> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&ValueId, &mut u32)> {
        self.0.iter_mut()
    }

    pub fn keys(&self) -> impl Iterator<Item = ValueId> + '_ {
        self.0.keys().copied()
    }

    /// Remove the value in this set which is closest compared to the others
    ///
    /// If this set is empty, returns `None`.
    ///
    /// If more than one value have the same distance, this returns the value with
    /// the lowest id first.
    #[inline]
    pub fn pop_first(&mut self) -> Option<(ValueId, u32)> {
        self.0.pop_first()
    }

    /// Remove the value in this set which is furthest away compared to the others
    ///
    /// If this set is empty, returns `None`.
    ///
    /// If more than one value have the same distance, this returns the value with
    /// the highest id first.
    #[inline]
    pub fn pop_last(&mut self) -> Option<(ValueId, u32)> {
        self.0.pop_last()
    }
}
impl<'a, 'b> std::ops::BitOr<&'b NextUseSet> for &'a NextUseSet {
    type Output = NextUseSet;

    #[inline]
    fn bitor(self, rhs: &'b NextUseSet) -> Self::Output {
        self.union(rhs)
    }
}
impl<'a, 'b> std::ops::BitAnd<&'b NextUseSet> for &'a NextUseSet {
    type Output = NextUseSet;

    #[inline]
    fn bitand(self, rhs: &'b NextUseSet) -> Self::Output {
        self.intersection(rhs)
    }
}
impl<'a, 'b> std::ops::BitXor<&'b NextUseSet> for &'a NextUseSet {
    type Output = NextUseSet;

    #[inline]
    fn bitxor(self, rhs: &'b NextUseSet) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

/// This function computes global next-use distances/liveness until a fixpoint is reached.
///
/// The resulting data structure associates two [NextUseSet]s with every block and instruction in `function`.
/// One set provides next-use distances _at_ a given program point, the other _after_ a given program point.
/// Intuitively, these are basically what they sound like. If a value is used "at" a given instruction, it's
/// next-use distance will be 0, and if it is never used after that, it won't be present in the "after" set.
/// However, if it used again later, it's distance in the "after" set will be the distance from the instruction
/// following the current one (or the block exit if the current one is a terminator).
fn compute_liveness(
    liveness: &mut LivenessAnalysis,
    function: &Function,
    cfg: &ControlFlowGraph,
    domtree: &DominatorTree,
    loops: &LoopAnalysis,
) {
    use std::collections::hash_map::Entry;

    // The distance penalty applied to an edge which exits a loop
    const LOOP_EXIT_DISTANCE: u32 = 100_000;

    let mut worklist = VecDeque::new();
    for block in domtree.cfg_postorder().iter().copied() {
        worklist.push_back(block);
    }

    // Compute liveness and global next-uses for each block, bottom-up
    //
    // For each block, liveness is determined by the next-use distance for
    // each variable, where a distance of `u32::MAX` is our representation of
    // infinity, and infinity is the distance assigned a value which is dead.
    // All values implicitly start with an infinite distance, and the distance
    // to next use is calculated by taking the length of control flow edges (0
    // for regular edges, LOOP_EXIT_DISTANCE for edges which leave a loop) and
    // adding that to the next-use distances of values which are live over the edge.
    //
    // The next-use distance for a value within a block, is the distance between
    // that use and it's definition in terms of the number of instructions in the block
    // which precede it. As such, we visit instructions in a block bottom-up, where at
    // each program point, we record the following next-use information:
    //
    // * Values defined by the current instruction are given the next-use distances
    // observed thus far at that instruction, e.g. if no use of a value is observed,
    // that distance is `u32::MAX`. For all preceding instructions, the value is
    // removed from the next-use set.
    // if the value is used by the next instruction the distance is `1`, and so on
    // * Values used by the current instruction are given a next-use distance of `0`
    // * All other values known at the current program point have their next-use distances
    // incremented by 1
    let mut inst_uses = FxHashSet::<Value>::default();
    while let Some(block_id) = worklist.pop_front() {
        let block = &function.dfg.blocks[block_id];
        let block_loop = loops.innermost_loop(block_id);
        let mut max_register_pressure = 0;
        let mut inst_cursor = block.insts.back();
        while let Some(inst_data) = inst_cursor.get() {
            inst_uses.clear();
            let inst = inst_data.key;
            let pp = ProgramPoint::Inst(inst);
            let branch_info = inst_data.analyze_branch(&function.dfg.value_lists);
            // Values used by `inst`
            inst_uses.extend(
                inst_data
                    .arguments(&function.dfg.value_lists)
                    .iter()
                    .copied(),
            );
            match branch_info {
                BranchInfo::SingleDest(_, args) => {
                    inst_uses.extend(args.iter().copied());
                }
                BranchInfo::MultiDest(ref jts) => {
                    for jt in jts.iter() {
                        inst_uses.extend(jt.args.iter().copied());
                    }
                }
                BranchInfo::NotABranch => (),
            }
            // Compute the initial next-use set of this program point
            //
            // * For terminators which are branches, next-use distances are given by the next-use
            // distances live over the control-flow graph edges. Multiple successors are handled by
            // joining the next-use sets of all edges. In both cases, block arguments of successors
            // are removed from the set, unless the successor dominates the instruction.
            // * For all other terminators (i.e. `ret`), the next-use set is empty.
            // * For regular instructions, the next-use set is initialized using the next-use set of
            // the succeeding instruction, with all distances incremented by 1, but with all values
            // defined by the successor removed from the set.
            //
            let (mut inst_next_uses, mut inst_next_uses_after) = match branch_info {
                // This is either a non-branching terminator, or a regular instruction
                BranchInfo::NotABranch => {
                    let succ_cursor = inst_cursor.peek_next();
                    if let Some(succ) = succ_cursor.get() {
                        // Start with the next-use set of the next instruction
                        let mut inst_next_uses = liveness
                            .live_in
                            .entry(ProgramPoint::Inst(succ.key))
                            .or_default()
                            .clone();
                        for (_value, dist) in inst_next_uses.iter_mut() {
                            *dist = dist.saturating_add(1);
                        }
                        let inst_next_uses_after = inst_next_uses.clone();
                        for value in function.dfg.inst_results(succ.key).iter() {
                            inst_next_uses.remove(value);
                        }
                        (inst_next_uses, inst_next_uses_after)
                    } else {
                        // This must be a non-branching terminator, i.e. the `ret` instruction
                        // Thus, the next-use set is empty initially
                        (NextUseSet::default(), NextUseSet::default())
                    }
                }
                // This is a branch instruction, so get the next-use set at the entry of each successor,
                // increment the distances in those sets based on the distance of the edge, and then take
                // the join of those sets as the initial next-use set for `inst`
                BranchInfo::SingleDest(succ, inst_uses) => {
                    let mut inst_next_uses = liveness
                        .live_in
                        .entry(ProgramPoint::Block(succ))
                        .or_default()
                        .clone();
                    let mut inst_next_uses_after = inst_next_uses.clone();

                    // For every argument passed to the successor block, mark those values used
                    // by this instruction IF the next-use distance of the corresponding block
                    // argument is < u32::MAX
                    for (in_arg, out_arg) in inst_uses
                        .iter()
                        .copied()
                        .zip(function.dfg.block_args(succ).iter())
                    {
                        if inst_next_uses.is_live(out_arg) {
                            inst_next_uses.insert(in_arg, 0);
                        }
                    }

                    // Remove the successor block arguments, as those values cannot be live before
                    // they are defined, and even if the destination block dominates the current
                    // block (via loop), those values must be dead at this instruction as we're
                    // providing new definitions for them.
                    for value in function.dfg.block_args(succ) {
                        inst_next_uses.remove(value);
                        inst_next_uses_after.remove(value);
                    }

                    // If this block is in a loop, make sure we add the loop exit distance for edges leaving the loop
                    if let Some(block_loop) = block_loop {
                        let succ_loop = loops.innermost_loop(succ);
                        let is_loop_exit = succ_loop
                            .map(|l| block_loop != l && loops.is_child_loop(block_loop, l))
                            .unwrap_or(true);
                        if is_loop_exit {
                            for (_, dist) in inst_next_uses.iter_mut() {
                                *dist = dist.saturating_add(LOOP_EXIT_DISTANCE);
                            }
                            for (_, dist) in inst_next_uses_after.iter_mut() {
                                *dist = dist.saturating_add(LOOP_EXIT_DISTANCE);
                            }
                        }
                    }
                    (inst_next_uses, inst_next_uses_after)
                }
                // Same as above
                //
                // NOTE: We additionally assert here that all critical edges in the control flow graph have been split,
                // as we cannot proceed correctly otherwise. It is expected that either no critical edges
                // exist, or that they have been split by a prior transformation.
                BranchInfo::MultiDest(jts) => {
                    let mut inst_next_uses = NextUseSet::default();
                    let mut inst_next_uses_after = NextUseSet::default();
                    for JumpTable {
                        destination,
                        args: inst_uses,
                    } in jts.iter()
                    {
                        let destination = *destination;
                        // If the successor block has multiple predecessors, this is a critical edge, as by
                        // definition this instruction means the current block has multiple successors
                        assert_eq!(
                            cfg.num_predecessors(destination),
                            1,
                            "expected all critical edges of {} to have been split!",
                            destination,
                        );
                        let mut jt_next_uses = liveness
                            .live_in
                            .entry(ProgramPoint::Block(destination))
                            .or_default()
                            .clone();
                        let mut jt_next_uses_after = jt_next_uses.clone();
                        // As is done for unconditional branches, propagate next-use distance for
                        // the successor arguments.
                        for (in_arg, out_arg) in inst_uses
                            .iter()
                            .copied()
                            .zip(function.dfg.block_args(destination).iter())
                        {
                            if jt_next_uses.is_live(out_arg) {
                                jt_next_uses.insert(in_arg, 0);
                            }
                        }
                        // Likewise, remove block arguments of the successor from the set
                        for value in function.dfg.block_args(destination) {
                            jt_next_uses.remove(value);
                            jt_next_uses_after.remove(value);
                        }
                        // If this block is in a loop, make sure we add the loop exit distance for edges leaving the loop
                        if let Some(block_loop) = block_loop {
                            let succ_loop = loops.innermost_loop(destination);
                            let is_loop_exit = succ_loop
                                .map(|l| block_loop != l && loops.is_child_loop(block_loop, l))
                                .unwrap_or(true);
                            if is_loop_exit {
                                for (_, dist) in jt_next_uses.iter_mut() {
                                    *dist = dist.saturating_add(LOOP_EXIT_DISTANCE);
                                }
                                for (_, dist) in jt_next_uses_after.iter_mut() {
                                    *dist = dist.saturating_add(LOOP_EXIT_DISTANCE);
                                }
                            }
                        }
                        inst_next_uses = inst_next_uses.union(&jt_next_uses);
                        inst_next_uses_after = inst_next_uses_after.union(&jt_next_uses_after);
                    }
                    (inst_next_uses, inst_next_uses_after)
                }
            };

            // If a value is defined by `inst`, it's next-use distance is whatever the distance at the next instruction is + 1
            //
            // We've already incremented the next-use distance if it was previously known, so here we are simply making
            // sure that we have the default distance set if it has no known distance
            for v in function.dfg.inst_results(inst).iter().copied() {
                inst_next_uses.entry(v).or_insert(u32::MAX);
                inst_next_uses_after.entry(v).or_insert(u32::MAX);
            }

            // If a value is used by `inst`, it's next-use distance is `0`
            for v in inst_uses.iter().copied() {
                inst_next_uses.insert(v, 0);
            }

            // The maximum register pressure for this block is the greatest number of
            // live-in values at any point within the block.
            max_register_pressure = cmp::max(
                max_register_pressure,
                inst_next_uses
                    .iter()
                    .filter(|(_, d)| *d < &u32::MAX)
                    .count(),
            );

            // Record the next-use distances for this program point
            liveness.live_in.insert(pp, inst_next_uses);
            liveness.live_out.insert(pp, inst_next_uses_after);

            // Move to the instruction preceding this one
            inst_cursor.move_prev();
        }

        // Handle the block header
        let pp = ProgramPoint::Block(block_id);
        // The block header derives it's next-use distances from it's first instruction, sans defs
        let first_inst = block.insts.front().get().unwrap().key;
        let mut block_next_uses = liveness.live_in[&ProgramPoint::Inst(first_inst)].clone();
        for value in function.dfg.inst_results(first_inst).iter() {
            block_next_uses.remove(value);
        }
        // For each block argument, set the next-use distance to u32::MAX unless present
        for arg in block
            .params
            .as_slice(&function.dfg.value_lists)
            .iter()
            .copied()
        {
            block_next_uses.entry(arg).or_insert(u32::MAX);
        }
        // For block's, the "after" set corresponds to the next-use set "after" the block
        // terminator. This makes it easy to query liveness at entry and exit to a block.
        let last_inst = block.insts.back().get().unwrap().key;
        let block_next_uses_after = liveness.live_out[&ProgramPoint::Inst(last_inst)].clone();
        // Re-enqueue this block until analysis reaches a stable state, i.e. the next-use
        // distances remain unchanged.
        match liveness.live_in.entry(pp) {
            Entry::Vacant(entry) => {
                entry.insert(block_next_uses);
                liveness.live_out.insert(pp, block_next_uses_after);
                // We haven't visited this block before, so re-enqueue it as by
                // definition the next-use set has changed.
                worklist.push_back(block_id);
            }
            Entry::Occupied(mut entry) => {
                let prev = entry.get();
                // We've seen this block before, but if nothing has changed since
                // our last visit, then we have reached a fixpoint. Otherwise,
                // we must re-enqueue it again.
                if prev != &block_next_uses {
                    worklist.push_back(block_id);
                }
                entry.insert(block_next_uses);
                liveness.live_out.insert(pp, block_next_uses_after);
            }
        }
        liveness
            .per_block_register_pressure
            .insert(block_id, max_register_pressure);
    }
}

impl hir::Decorator for &LivenessAnalysis {
    type Display<'a> = DisplayLiveness<'a> where Self: 'a;

    fn decorate_block<'a, 'l: 'a>(&'l self, block: BlockId) -> Self::Display<'a> {
        DisplayLiveness {
            pp: ProgramPoint::Block(block),
            lr: self,
        }
    }

    fn decorate_inst<'a, 'l: 'a>(&'l self, inst: InstId) -> Self::Display<'a> {
        DisplayLiveness {
            pp: ProgramPoint::Inst(inst),
            lr: self,
        }
    }
}

struct NextUse<'a> {
    value: &'a ValueId,
    distance: &'a u32,
}
impl<'a> core::fmt::Display for NextUse<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}:{}", self.value, self.distance)
    }
}

#[doc(hidden)]
pub struct DisplayLiveness<'a> {
    pp: ProgramPoint,
    lr: &'a LivenessAnalysis,
}
impl<'a> core::fmt::Display for DisplayLiveness<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let live = self
            .lr
            .next_uses(self.pp)
            .iter()
            .map(|(value, distance)| NextUse { value, distance });
        write!(f, " # next_used=[{}]", DisplayValues::new(live),)
    }
}
