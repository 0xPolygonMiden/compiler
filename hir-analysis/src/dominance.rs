use std::{
    cmp::{self, Ordering},
    mem,
};

use cranelift_entity::packed_option::PackedOption;
use cranelift_entity::SecondaryMap;

use rustc_hash::FxHashSet;

use miden_hir::{Block, BranchInfo, DataFlowGraph, Function, Inst, ProgramPoint};

use super::{BlockPredecessor, ControlFlowGraph};

/// RPO numbers are assigned as multiples of STRIDE to leave room
/// for modifications to the dominator tree.
const STRIDE: u32 = 4;

/// A special RPO number used during `compute_postorder`.
const SEEN: u32 = 1;

/// A node in the dominator tree. Each block has one of these.
#[derive(Clone, Default)]
struct Node {
    /// Number of this node in a reverse post-order traversal of the control-flow graph, starting from 1.
    ///
    /// This number is monotonic in the reverse post-order but not contiguous, as we leave holes for
    /// localized modifications of the dominator tree after it is initially computed.
    ///
    /// Unreachable nodes get number 0, all others are > 0.
    rpo_number: u32,
    /// The immediate dominator of this block, represented as the instruction at the end of the dominating
    /// block which transfers control to this block.
    ///
    /// This is `None` for unreachable blocks, as well as the entry block, which has no dominators.
    idom: PackedOption<Inst>,
}

/// DFT stack state marker for computing the cfg post-order.
enum Visit {
    First,
    Last,
}

#[derive(Default)]
pub struct DominatorTree {
    nodes: SecondaryMap<Block, Node>,
    /// Post-order of all reachable blocks in the control flow graph
    postorder: Vec<Block>,
    /// Scratch buffer used by `compute_postorder`
    stack: Vec<(Visit, Block)>,
    valid: bool,
}
impl DominatorTree {
    /// Allocate a new blank dominator tree. Use `compute` to compute the dominator tree for a
    /// function.
    pub fn new() -> Self {
        Self::default()
    }

    /// Allocate and compute a dominator tree.
    pub fn with_function(func: &Function, cfg: &ControlFlowGraph) -> Self {
        let block_capacity = func.dfg.num_blocks();
        let mut domtree = Self {
            nodes: SecondaryMap::with_capacity(block_capacity),
            postorder: Vec::with_capacity(block_capacity),
            stack: Vec::new(),
            valid: false,
        };
        domtree.compute(func, cfg);
        domtree
    }

    /// Reset and compute a CFG post-order and dominator tree.
    pub fn compute(&mut self, func: &Function, cfg: &ControlFlowGraph) {
        debug_assert!(cfg.is_valid());
        self.compute_postorder(func);
        self.compute_domtree(func, cfg);
        self.valid = true;
    }

    /// Clear the data structures used to represent the dominator tree. This will leave the tree in
    /// a state where `is_valid()` returns false.
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.postorder.clear();
        debug_assert!(self.stack.is_empty());
        self.valid = false;
    }

    /// Check if the dominator tree is in a valid state.
    ///
    /// Note that this doesn't perform any kind of validity checks. It simply checks if the
    /// `compute()` method has been called since the last `clear()`. It does not check that the
    /// dominator tree is consistent with the CFG.
    pub fn is_valid(&self) -> bool {
        self.valid
    }

    /// Is `block` reachable from the entry block?
    pub fn is_reachable(&self, block: Block) -> bool {
        self.nodes[block].rpo_number != 0
    }

    /// Get the blocks in cfg post-order used to compute the dominator tree.
    ///
    /// NOTE: This order is not updated automatically when the control-flow graph is modified,
    /// it is computed from scratch and cached by `compute`.
    pub fn cfg_postorder(&self) -> &[Block] {
        debug_assert!(self.is_valid());
        &self.postorder
    }

    /// Returns the immediate dominator of `block`.
    ///
    /// The immediate dominator of a basic block is the instruction which transfers control to that
    /// block (and implicitly, its enclosing block). This instruction does not have to be the terminator
    /// of its block, though it typically is.
    ///
    /// An instruction "dominates" `block` if all control flow paths from the function entry to `block`
    /// must go through that instruction.
    ///
    /// The "immediate dominator" is the dominator that is closest to `block`. All other dominators
    /// also dominate the immediate dominator.
    ///
    /// This returns `None` if `block` is not reachable from the entry block, or if it is the entry block
    /// which has no dominators.
    pub fn idom(&self, block: Block) -> Option<Inst> {
        self.nodes[block].idom.into()
    }

    /// Compare two blocks relative to the reverse post-order.
    pub fn rpo_cmp_block(&self, a: Block, b: Block) -> Ordering {
        self.nodes[a].rpo_number.cmp(&self.nodes[b].rpo_number)
    }

    /// Compare two program points relative to a reverse post-order traversal of the control-flow
    /// graph.
    ///
    /// Return `Ordering::Less` if `a` comes before `b` in the RPO.
    ///
    /// If `a` and `b` belong to the same block, compare their relative position in the block.
    pub fn rpo_cmp<A, B>(&self, a: A, b: B, dfg: &DataFlowGraph) -> Ordering
    where
        A: Into<ProgramPoint>,
        B: Into<ProgramPoint>,
    {
        let a = a.into();
        let b = b.into();
        self.rpo_cmp_block(dfg.pp_block(a), dfg.pp_block(b))
            .then_with(|| dfg.pp_cmp(a, b))
    }

    /// Returns `true` if `a` dominates `b`.
    ///
    /// This means that every control-flow path from the function entry to `b` must go through `a`.
    ///
    /// Dominance is ill defined for unreachable blocks. This function can always determine
    /// dominance for instructions in the same block, but otherwise returns `false` if either block
    /// is unreachable.
    ///
    /// An instruction is considered to dominate itself.
    pub fn dominates<A, B>(&self, a: A, b: B, dfg: &DataFlowGraph) -> bool
    where
        A: Into<ProgramPoint>,
        B: Into<ProgramPoint>,
    {
        let a = a.into();
        let b = b.into();
        match a {
            ProgramPoint::Block(block_a) => {
                a == b || self.last_dominator(block_a, b, dfg).is_some()
            }
            ProgramPoint::Inst(inst_a) => {
                let block_a = dfg.inst_block(inst_a).expect("Instruction not in layout.");
                match self.last_dominator(block_a, b, dfg) {
                    Some(last) => dfg.pp_cmp(inst_a, last) != Ordering::Greater,
                    None => false,
                }
            }
        }
    }

    /// Find the last instruction in `a` that dominates `b`.
    /// If no instructions in `a` dominate `b`, return `None`.
    pub fn last_dominator<B>(&self, a: Block, b: B, dfg: &DataFlowGraph) -> Option<Inst>
    where
        B: Into<ProgramPoint>,
    {
        let (mut block_b, mut inst_b) = match b.into() {
            ProgramPoint::Block(block) => (block, None),
            ProgramPoint::Inst(inst) => (
                dfg.inst_block(inst).expect("Instruction not in layout."),
                Some(inst),
            ),
        };
        let rpo_a = self.nodes[a].rpo_number;

        // Run a finger up the dominator tree from b until we see a.
        // Do nothing if b is unreachable.
        while rpo_a < self.nodes[block_b].rpo_number {
            let idom = match self.idom(block_b) {
                Some(idom) => idom,
                None => return None, // a is unreachable, so we climbed past the entry
            };
            block_b = dfg.inst_block(idom).expect("Dominator got removed.");
            inst_b = Some(idom);
        }
        if a == block_b {
            inst_b
        } else {
            None
        }
    }

    /// Compute the common dominator of two basic blocks.
    ///
    /// Both basic blocks are assumed to be reachable.
    pub fn common_dominator(
        &self,
        mut a: BlockPredecessor,
        mut b: BlockPredecessor,
        dfg: &DataFlowGraph,
    ) -> BlockPredecessor {
        loop {
            match self.rpo_cmp_block(a.block, b.block) {
                Ordering::Less => {
                    // `a` comes before `b` in the RPO. Move `b` up.
                    let idom = self.nodes[b.block].idom.expect("Unreachable basic block?");
                    b = BlockPredecessor::new(
                        dfg.inst_block(idom).expect("Dangling idom instruction"),
                        idom,
                    );
                }
                Ordering::Greater => {
                    // `b` comes before `a` in the RPO. Move `a` up.
                    let idom = self.nodes[a.block].idom.expect("Unreachable basic block?");
                    a = BlockPredecessor::new(
                        dfg.inst_block(idom).expect("Dangling idom instruction"),
                        idom,
                    );
                }
                Ordering::Equal => break,
            }
        }

        debug_assert_eq!(
            a.block, b.block,
            "Unreachable block passed to common_dominator?"
        );

        // We're in the same block. The common dominator is the earlier instruction.
        if dfg.pp_cmp(a.inst, b.inst) == Ordering::Less {
            a
        } else {
            b
        }
    }

    /// Reset all internal data structures and compute a post-order of the control flow graph.
    ///
    /// This leaves `rpo_number == 1` for all reachable blocks, 0 for unreachable ones.
    fn compute_postorder(&mut self, func: &Function) {
        self.clear();
        self.nodes.resize(func.dfg.num_blocks());

        // This algorithm is a depth first traversal (DFT) of the control flow graph, computing a
        // post-order of the blocks that are reachable form the entry block. A DFT post-order is not
        // unique. The specific order we get is controlled by the order each node's children are
        // visited.
        //
        // We view the CFG as a graph where each `BlockCall` value of a terminating branch
        // instruction is an edge. A consequence of this is that we visit successor nodes in the
        // reverse order specified by the branch instruction that terminates the basic block.
        // (Reversed because we are using a stack to control traversal, and push the successors in
        // the order the branch instruction specifies -- there's no good reason for this particular
        // order.)
        //
        // During this algorithm only, use `rpo_number` to hold the following state:
        //
        //   0:    block has not yet had its first visit
        //   SEEN: block has been visited at least once, implying that all of its successors are on
        //         the stack
        self.stack.push((Visit::First, func.dfg.entry_block()));

        while let Some((visit, block)) = self.stack.pop() {
            match visit {
                Visit::First => {
                    if self.nodes[block].rpo_number == 0 {
                        // This is the first time we pop the block, so we need to scan its
                        // successors and then revisit it.
                        self.nodes[block].rpo_number = SEEN;
                        self.stack.push((Visit::Last, block));
                        if let Some(inst) = func.dfg.last_inst(block) {
                            // Heuristic: chase the children in reverse. This puts the first
                            // successor block first in the postorder, all other things being
                            // equal, which tends to prioritize loop backedges over out-edges,
                            // putting the edge-block closer to the loop body and minimizing
                            // live-ranges in linear instruction space. This heuristic doesn't have
                            // any effect on the computation of dominators, and is purely for other
                            // consumers of the postorder we cache here.
                            match func.dfg.analyze_branch(inst) {
                                BranchInfo::NotABranch => (),
                                BranchInfo::SingleDest(dest, _) => {
                                    if self.nodes[dest].rpo_number == 0 {
                                        self.stack.push((Visit::First, dest));
                                    }
                                }
                                BranchInfo::MultiDest(ref jt) => {
                                    for dest in jt.iter().map(|entry| entry.destination) {
                                        if self.nodes[dest].rpo_number == 0 {
                                            self.stack.push((Visit::First, dest));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                Visit::Last => {
                    // We've finished all this node's successors.
                    self.postorder.push(block);
                }
            }
        }
    }

    /// Build a dominator tree from a control flow graph using Keith D. Cooper's
    /// "Simple, Fast Dominator Algorithm."
    fn compute_domtree(&mut self, func: &Function, cfg: &ControlFlowGraph) {
        // During this algorithm, `rpo_number` has the following values:
        //
        // 0: block is not reachable.
        // 1: block is reachable, but has not yet been visited during the first pass. This is set by
        // `compute_postorder`.
        // 2+: block is reachable and has an assigned RPO number.

        // We'll be iterating over a reverse post-order of the CFG, skipping the entry block.
        let (entry_block, postorder) = match self.postorder.as_slice().split_last() {
            Some((&eb, rest)) => (eb, rest),
            None => return,
        };

        // Do a first pass where we assign RPO numbers to all reachable nodes.
        self.nodes[entry_block].rpo_number = 2 * STRIDE;
        for (rpo_idx, &block) in postorder.iter().rev().enumerate() {
            // Update the current node and give it an RPO number.
            // The entry block got 2, the rest start at 3 by multiples of STRIDE to leave
            // room for future dominator tree modifications.
            //
            // Since `compute_idom` will only look at nodes with an assigned RPO number, the
            // function will never see an uninitialized predecessor.
            //
            // Due to the nature of the post-order traversal, every node we visit will have at
            // least one predecessor that has previously been visited during this RPO.
            self.nodes[block] = Node {
                idom: self.compute_idom(block, cfg, &func.dfg).into(),
                rpo_number: (rpo_idx as u32 + 3) * STRIDE,
            }
        }

        // Now that we have RPO numbers for everything and initial immediate dominator estimates,
        // iterate until convergence.
        //
        // If the function is free of irreducible control flow, this will exit after one iteration.
        let mut changed = true;
        while changed {
            changed = false;
            for &block in postorder.iter().rev() {
                let idom = self.compute_idom(block, cfg, &func.dfg).into();
                if self.nodes[block].idom != idom {
                    self.nodes[block].idom = idom;
                    changed = true;
                }
            }
        }
    }

    // Compute the immediate dominator for `block` using the current `idom` states for the reachable
    // nodes.
    fn compute_idom(&self, block: Block, cfg: &ControlFlowGraph, dfg: &DataFlowGraph) -> Inst {
        // Get an iterator with just the reachable, already visited predecessors to `block`.
        // Note that during the first pass, `rpo_number` is 1 for reachable blocks that haven't
        // been visited yet, 0 for unreachable blocks.
        let mut reachable_preds = cfg
            .pred_iter(block)
            .filter(|&BlockPredecessor { block: pred, .. }| self.nodes[pred].rpo_number > 1);

        // The RPO must visit at least one predecessor before this node.
        let mut idom = reachable_preds
            .next()
            .expect("block node must have one reachable predecessor");

        for pred in reachable_preds {
            idom = self.common_dominator(idom, pred, dfg);
        }

        idom.inst
    }
}

/// Auxiliary structure for `DominatorTree` which provides:
///
/// - Traversal of the dominator tree in pre-order
/// - Ordering of blocks in dominator tree pre-order
/// - Constant-time dominance checks per-block
pub struct DominatorTreePreorder {
    nodes: SecondaryMap<Block, PreorderNode>,
    stack: Vec<Block>,
}

#[derive(Default, Clone)]
struct PreorderNode {
    /// First child node in the dominator tree
    child: Option<Block>,
    /// Next sibling node in the dominator tree, ordered
    /// according to the control-flow graph reverse post-order.
    sibling: Option<Block>,
    /// Sequence number for this node in a pre-order traversal of the dominator tree
    ///
    /// Unreachable blocks are 0, entry block is 1
    pre_number: u32,
    /// Maximum `pre_number` for the sub-tree of the dominator tree that is rooted at this node.
    ///
    /// This is always greater than or equal to `pre_number`
    pre_max: u32,
}

impl DominatorTreePreorder {
    pub fn new() -> Self {
        Self {
            nodes: Default::default(),
            stack: vec![],
        }
    }

    pub fn with_function(domtree: &DominatorTree, function: &Function) -> Self {
        let mut this = Self::new();
        this.compute(domtree, function);
        this
    }

    pub fn compute(&mut self, domtree: &DominatorTree, function: &Function) {
        self.nodes.clear();
        debug_assert_eq!(self.stack.len(), 0);

        // Step 1: Populate the child and sibling links.
        //
        // By following the CFG post-order and pushing to the front of the lists, we make sure that
        // sibling lists are ordered according to the CFG reverse post-order.
        for &block in domtree.cfg_postorder() {
            if let Some(idom_inst) = domtree.idom(block) {
                let idom = function.dfg.inst_block(idom_inst).unwrap();
                let sib = mem::replace(&mut self.nodes[idom].child, Some(block));
                self.nodes[block].sibling = sib;
            } else {
                // The only block without an immediate dominator is the entry.
                self.stack.push(block);
            }
        }

        // Step 2. Assign pre-order numbers from a DFS of the dominator tree.
        debug_assert!(self.stack.len() <= 1);
        let mut n = 0;
        while let Some(block) = self.stack.pop() {
            n += 1;
            let node = &mut self.nodes[block];
            node.pre_number = n;
            node.pre_max = n;
            if let Some(n) = node.sibling {
                self.stack.push(n);
            }
            if let Some(n) = node.child {
                self.stack.push(n);
            }
        }

        // Step 3. Propagate the `pre_max` numbers up the tree.
        // The CFG post-order is topologically ordered w.r.t. dominance so a node comes after all
        // its dominator tree children.
        for &block in domtree.cfg_postorder() {
            if let Some(idom_inst) = domtree.idom(block) {
                let idom = function.dfg.inst_block(idom_inst).unwrap();
                let pre_max = cmp::max(self.nodes[block].pre_max, self.nodes[idom].pre_max);
                self.nodes[idom].pre_max = pre_max;
            }
        }
    }

    /// Get an iterator over the immediate children of `block` in the dominator tree.
    ///
    /// These are the blocks whose immediate dominator is an instruction in `block`, ordered according
    /// to the CFG reverse post-order.
    pub fn children(&self, block: Block) -> ChildIter {
        ChildIter {
            dtpo: self,
            next: self.nodes[block].child,
        }
    }

    /// Fast, constant time dominance check with block granularity.
    ///
    /// This computes the same result as `domtree.dominates(a, b)`, but in guaranteed fast constant
    /// time. This is less general than the `DominatorTree` method because it only works with block
    /// program points.
    ///
    /// A block is considered to dominate itself.
    pub fn dominates(&self, a: Block, b: Block) -> bool {
        let na = &self.nodes[a];
        let nb = &self.nodes[b];
        na.pre_number <= nb.pre_number && na.pre_max >= nb.pre_max
    }

    /// Compare two blocks according to the dominator pre-order.
    pub fn pre_cmp_block(&self, a: Block, b: Block) -> Ordering {
        self.nodes[a].pre_number.cmp(&self.nodes[b].pre_number)
    }

    /// Compare two program points according to the dominator tree pre-order.
    ///
    /// This ordering of program points have the property that given a program point, pp, all the
    /// program points dominated by pp follow immediately and contiguously after pp in the order.
    pub fn pre_cmp<A, B>(&self, a: A, b: B, function: &Function) -> Ordering
    where
        A: Into<ProgramPoint>,
        B: Into<ProgramPoint>,
    {
        let a = a.into();
        let b = b.into();
        self.pre_cmp_block(function.dfg.pp_block(a), function.dfg.pp_block(b))
            .then_with(|| function.dfg.pp_cmp(a, b))
    }
}

/// An iterator that enumerates the direct children of a block in the dominator tree.
pub struct ChildIter<'a> {
    dtpo: &'a DominatorTreePreorder,
    next: Option<Block>,
}

impl<'a> Iterator for ChildIter<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.next;
        if let Some(block) = n {
            self.next = self.dtpo.nodes[block].sibling;
        }
        n
    }
}

/// Calculates the dominance frontier for every block in a given `DominatorTree`
///
/// A dominance frontier of a block `B` is the set of blocks `N` where control flow
/// join points exist, where multiple value definitions together as one.
///
/// More formally, the dominance frontier is every block `Ni` in `N` where the following
/// properties hold:
///
/// * `B` dominates an immediate predecessor of `Ni`
/// * `B` does not strictly dominate `Ni`; strict dominance is when `B` dominates
/// `Ni`, but `B != Ni`
///
/// Consider the following example:
///
///
/// ```text,ignore
/// block0(v0):
///   v1 = ...
///   cond_br v0, block1, block2
///
/// block1():
///   br block3(v1)
///
/// block2():
///   v2 = ...
///   br block3(v2)
///
/// block3(v3):
///   ...
/// ```
///
/// Here, `block0` strictly dominates all other blocks; but neither `block1` or `block2`
/// dominate `block3`. This tells us that `block3` must be in the dominance frontier of `block1`
/// and `block2`, because:
///
/// * By definition, every block dominates itself, but does not strictly dominate itself
/// * Both `block1` and `block2` are immediate predecessors of `block3`
/// * Thus, both `block1` and `block2` technically dominate a predecessor of `block3`
/// * Neither `block1` nor `block2` strictly dominate `block3`
///
/// It is also obvious that `block3` must be in the dominance frontier of `block1` and `block2`,
/// because we can observe that `block3` is a join point for control that flows through `block1` and
/// `block2` - the value of `v3` depends on which path is taken to reach `block3`.
///
/// You might wonder if `block3` is in the dominance frontier of `block0`, and the answer is no.
/// That's because `block0` strictly dominates `block3`, i.e. all control flow must pass through it
/// to reach `block3`. The reason why strict dominance matters becomes more clear when you consider
/// that any value defined in `block0` will have the same definition same regardless of which path is
/// taken to reach `block3`.
///
/// ## Purpose
///
/// The dominance frontier is used to place new phi nodes (which in our IR are represented by block arguments)
/// after introducing register spills/reloads. Reloads would naturally introduce multiple definitions for
/// a given value, which would break the SSA property of the IR, so to preserve it, reloads introduce new
/// definitions, and all uses of the original definition dominated by the reload are updated.
///
/// However, that alone is insufficient, since there may be uses of the original definition which are _not_
/// dominated by the reload due to branching control flow. To address this, we must introduce new block
/// arguments to every block in the dominance frontier of the block in which reloads occur, and where
/// the reloaded value is live. All uses of either the original definition dominated by that phi node are
/// rewritten to use the definition produced by the phi.
///
/// The actual algorithm works bottom-up, rather than top-down, but the relationship to the dominance frontier
/// is the same in both cases.
#[derive(Default)]
pub struct DominanceFrontier {
    /// The dominance frontier for each block, as a set of blocks
    dfs: SecondaryMap<Block, FxHashSet<Block>>,
}
impl DominanceFrontier {
    pub fn compute(domtree: &DominatorTree, cfg: &ControlFlowGraph, function: &Function) -> Self {
        let mut dfs = SecondaryMap::<Block, FxHashSet<Block>>::default();

        for id in domtree.cfg_postorder() {
            let id = *id;
            if cfg.num_predecessors(id) < 2 {
                continue;
            }
            let idom = domtree.idom(id).unwrap();
            for BlockPredecessor { block: p, inst: i } in cfg.pred_iter(id) {
                let mut p = p;
                let mut i = i;
                while i != idom {
                    dfs[p].insert(id);
                    let Some(idom_p) = domtree.idom(p) else {
                        break;
                    };
                    i = idom_p;
                    p = function.dfg.inst_block(idom_p).unwrap();
                }
            }
        }

        Self { dfs }
    }

    /// Get an iterator over the dominance frontier of `block`
    pub fn iter(&self, block: &Block) -> impl Iterator<Item = Block> + '_ {
        DominanceFrontierIter {
            df: self.dfs.get(*block).map(|set| set.iter().copied()),
        }
    }

    /// Get the set of blocks in the dominance frontier of `block`,
    /// or `None` if `block` has an empty dominance frontier.
    #[inline]
    pub fn get(&self, block: &Block) -> Option<&FxHashSet<Block>> {
        self.dfs.get(*block)
    }
}

struct DominanceFrontierIter<I> {
    df: Option<I>,
}
impl<'a, I> Iterator for DominanceFrontierIter<I>
where
    I: Iterator<Item = Block> + 'a,
{
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(i) = self.df.as_mut() {
            i.next()
        } else {
            None
        }
    }
}
