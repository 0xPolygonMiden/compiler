use cranelift_entity::packed_option::PackedOption;
use cranelift_entity::{entity_impl, PrimaryMap, SecondaryMap};

use miden_hir::{Block, DataFlowGraph, Function};

use super::{BlockPredecessor, ControlFlowGraph, DominatorTree};

/// Represents a loop in the loop tree of a function
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Loop(u32);
entity_impl!(Loop, "loop");

/// A level in a loop nest.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LoopLevel(u8);
impl LoopLevel {
    const INVALID: u8 = u8::MAX;

    /// Get the root level (no loop).
    pub const fn root() -> Self {
        Self(0)
    }

    /// Get the loop level.
    pub const fn level(self) -> usize {
        self.0 as usize
    }

    /// Invalid loop level.
    pub const fn invalid() -> Self {
        Self(Self::INVALID)
    }

    /// One loop level deeper.
    pub fn inc(self) -> Self {
        if self.0 == (Self::INVALID - 1) {
            self
        } else {
            Self(self.0 + 1)
        }
    }

    /// A clamped loop level from a larger-width (usize) depth.
    pub fn clamped(level: usize) -> Self {
        Self(
            u8::try_from(std::cmp::min(level, (Self::INVALID as usize) - 1))
                .expect("invalid clamped loop level"),
        )
    }
}
impl Default for LoopLevel {
    fn default() -> Self {
        LoopLevel::invalid()
    }
}

struct LoopData {
    header: Block,
    parent: PackedOption<Loop>,
    level: LoopLevel,
}
impl LoopData {
    /// Creates a `LoopData` object with the loop header and its eventual parent in the loop tree.
    pub fn new(header: Block, parent: Option<Loop>) -> Self {
        Self {
            header,
            parent: parent.into(),
            level: LoopLevel::invalid(),
        }
    }
}

/// Loop tree information for a single function.
///
/// Loops are referenced by the `Loop` type, and for each loop you can
/// access its header block, its eventual parent in the loop tree, and
/// all the blocks belonging to the loop.
pub struct LoopAnalysis {
    loops: PrimaryMap<Loop, LoopData>,
    block_loop_map: SecondaryMap<Block, PackedOption<Loop>>,
    valid: bool,
}
impl Default for LoopAnalysis {
    fn default() -> Self {
        Self {
            valid: false,
            loops: PrimaryMap::new(),
            block_loop_map: SecondaryMap::new(),
        }
    }
}
impl LoopAnalysis {
    pub fn with_function(
        function: &Function,
        cfg: &ControlFlowGraph,
        domtree: &DominatorTree,
    ) -> Self {
        let mut this = Self::default();
        this.compute(function, cfg, domtree);
        this
    }

    /// Detects the loops in a function. Needs the control flow graph and the dominator tree.
    pub fn compute(&mut self, func: &Function, cfg: &ControlFlowGraph, domtree: &DominatorTree) {
        self.loops.clear();
        self.block_loop_map.clear();
        self.block_loop_map.resize(func.dfg.num_blocks());
        self.find_loop_headers(cfg, domtree, &func.dfg);
        self.discover_loop_blocks(cfg, domtree, &func.dfg);
        self.assign_loop_levels();
        self.valid = true;
    }

    /// Check if the loop analysis is in a valid state.
    ///
    /// Note that this doesn't perform any kind of validity checks. It simply checks if the
    /// `compute()` method has been called since the last `clear()`. It does not check that the
    /// loop analysis is consistent with the CFG.
    pub fn is_valid(&self) -> bool {
        self.valid
    }

    /// Clear all the data structures contained in the loop analysis. This will leave the
    /// analysis in a similar state to a context returned by `new()` except that allocated
    /// memory be retained.
    pub fn clear(&mut self) {
        self.loops.clear();
        self.block_loop_map.clear();
        self.valid = false;
    }

    /// Returns all the loops contained in a function.
    pub fn loops(&self) -> cranelift_entity::Keys<Loop> {
        self.loops.keys()
    }

    /// Returns the header block of a particular loop.
    ///
    /// The characteristic property of a loop header block is that it dominates some of its
    /// predecessors.
    pub fn loop_header(&self, lp: Loop) -> Block {
        self.loops[lp].header
    }

    /// Return the eventual parent of a loop in the loop tree.
    pub fn loop_parent(&self, lp: Loop) -> Option<Loop> {
        self.loops[lp].parent.expand()
    }

    /// Return the innermost loop for a given block.
    pub fn innermost_loop(&self, block: Block) -> Option<Loop> {
        self.block_loop_map[block].expand()
    }

    /// Determine if a Block is a loop header. If so, return the loop.
    pub fn is_loop_header(&self, block: Block) -> Option<Loop> {
        self.innermost_loop(block)
            .filter(|&lp| self.loop_header(lp) == block)
    }

    /// Determine if a Block belongs to a loop by running a finger along the loop tree.
    ///
    /// Returns `true` if `block` is in loop `lp`.
    pub fn is_in_loop(&self, block: Block, lp: Loop) -> bool {
        let block_loop = self.block_loop_map[block];
        match block_loop.expand() {
            None => false,
            Some(block_loop) => self.is_child_loop(block_loop, lp),
        }
    }

    /// Determines if a loop is contained in another loop.
    ///
    /// `is_child_loop(child,parent)` returns `true` if and only if `child` is a child loop of
    /// `parent` (or `child == parent`).
    pub fn is_child_loop(&self, child: Loop, parent: Loop) -> bool {
        let mut finger = Some(child);
        while let Some(finger_loop) = finger {
            if finger_loop == parent {
                return true;
            }
            finger = self.loop_parent(finger_loop);
        }
        false
    }

    /// Returns the loop-nest level of a given block.
    pub fn loop_level(&self, block: Block) -> LoopLevel {
        self.innermost_loop(block)
            .map_or(LoopLevel(0), |lp| self.loops[lp].level)
    }

    /// Returns the loop level of the given level
    pub fn level(&self, lp: Loop) -> LoopLevel {
        self.loops[lp].level
    }

    // Traverses the CFG in reverse postorder and create a loop object for every block having a
    // back edge.
    fn find_loop_headers(
        &mut self,
        cfg: &ControlFlowGraph,
        domtree: &DominatorTree,
        dfg: &DataFlowGraph,
    ) {
        // We traverse the CFG in reverse postorder
        for &block in domtree.cfg_postorder().iter().rev() {
            for BlockPredecessor {
                inst: pred_inst, ..
            } in cfg.pred_iter(block)
            {
                // If the block dominates one of its predecessors it is a back edge
                if domtree.dominates(block, pred_inst, dfg) {
                    // This block is a loop header, so we create its associated loop
                    let lp = self.loops.push(LoopData::new(block, None));
                    self.block_loop_map[block] = lp.into();
                    break;
                    // We break because we only need one back edge to identify a loop header.
                }
            }
        }
    }

    // Intended to be called after `find_loop_headers`. For each detected loop header,
    // discovers all the block belonging to the loop and its inner loops. After a call to this
    // function, the loop tree is fully constructed.
    fn discover_loop_blocks(
        &mut self,
        cfg: &ControlFlowGraph,
        domtree: &DominatorTree,
        dfg: &DataFlowGraph,
    ) {
        let mut stack: Vec<Block> = Vec::new();
        // We handle each loop header in reverse order, corresponding to a pseudo postorder
        // traversal of the graph.
        for lp in self.loops().rev() {
            for BlockPredecessor {
                block: pred,
                inst: pred_inst,
            } in cfg.pred_iter(self.loops[lp].header)
            {
                // We follow the back edges
                if domtree.dominates(self.loops[lp].header, pred_inst, dfg) {
                    stack.push(pred);
                }
            }
            while let Some(node) = stack.pop() {
                let continue_dfs: Option<Block>;
                match self.block_loop_map[node].expand() {
                    None => {
                        // The node hasn't been visited yet, we tag it as part of the loop
                        self.block_loop_map[node] = PackedOption::from(lp);
                        continue_dfs = Some(node);
                    }
                    Some(node_loop) => {
                        // We copy the node_loop into a mutable reference passed along the while
                        let mut node_loop = node_loop;
                        // The node is part of a loop, which can be lp or an inner loop
                        let mut node_loop_parent_option = self.loops[node_loop].parent;
                        while let Some(node_loop_parent) = node_loop_parent_option.expand() {
                            if node_loop_parent == lp {
                                // We have encountered lp so we stop (already visited)
                                break;
                            } else {
                                //
                                node_loop = node_loop_parent;
                                // We lookup the parent loop
                                node_loop_parent_option = self.loops[node_loop].parent;
                            }
                        }
                        // Now node_loop_parent is either:
                        // - None and node_loop is an new inner loop of lp
                        // - Some(...) and the initial node_loop was a known inner loop of lp
                        match node_loop_parent_option.expand() {
                            Some(_) => continue_dfs = None,
                            None => {
                                if node_loop != lp {
                                    self.loops[node_loop].parent = lp.into();
                                    continue_dfs = Some(self.loops[node_loop].header)
                                } else {
                                    // If lp is a one-block loop then we make sure we stop
                                    continue_dfs = None
                                }
                            }
                        }
                    }
                }
                // Now we have handled the popped node and need to continue the DFS by adding the
                // predecessors of that node
                if let Some(continue_dfs) = continue_dfs {
                    for BlockPredecessor { block: pred, .. } in cfg.pred_iter(continue_dfs) {
                        stack.push(pred)
                    }
                }
            }
        }
    }

    fn assign_loop_levels(&mut self) {
        use smallvec::{smallvec, SmallVec};

        let mut stack: SmallVec<[Loop; 8]> = smallvec![];
        for lp in self.loops.keys() {
            if self.loops[lp].level == LoopLevel::invalid() {
                stack.push(lp);
                while let Some(&lp) = stack.last() {
                    if let Some(parent) = self.loops[lp].parent.into() {
                        if self.loops[parent].level != LoopLevel::invalid() {
                            self.loops[lp].level = self.loops[parent].level.inc();
                            stack.pop();
                        } else {
                            stack.push(parent);
                        }
                    } else {
                        self.loops[lp].level = LoopLevel::root().inc();
                        stack.pop();
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FunctionAnalysis;
    use miden_hir::{
        AbiParam, Function, FunctionBuilder, InstBuilder, Signature, SourceSpan, Type,
    };

    #[test]
    fn nested_loops_variant1_detection() {
        let id = "test::nested_loops_test".parse().unwrap();
        let mut function = Function::new(id, Signature::new([AbiParam::new(Type::I1)], []));

        let block0 = function.dfg.entry_block();
        let block1 = function.dfg.create_block();
        let block2 = function.dfg.create_block();
        let block3 = function.dfg.create_block();
        let block4 = function.dfg.create_block();
        {
            let mut builder = FunctionBuilder::new(&mut function);
            let cond = {
                let args = builder.block_params(block0);
                args[0]
            };

            builder.switch_to_block(block0);
            builder.ins().br(block1, &[], SourceSpan::UNKNOWN);

            builder.switch_to_block(block1);
            builder.ins().br(block2, &[], SourceSpan::UNKNOWN);

            builder.switch_to_block(block2);
            builder
                .ins()
                .cond_br(cond, block1, &[], block3, &[], SourceSpan::UNKNOWN);

            builder.switch_to_block(block3);
            builder
                .ins()
                .cond_br(cond, block0, &[], block4, &[], SourceSpan::UNKNOWN);

            builder.switch_to_block(block4);
            builder.ins().ret(None, SourceSpan::UNKNOWN);
        }

        let mut analysis = FunctionAnalysis::new(&function);
        analysis.ensure_loops(&function);
        let loop_analysis = analysis.loops();

        let loops = loop_analysis.loops().collect::<Vec<Loop>>();
        assert_eq!(loops.len(), 2);
        assert_eq!(loop_analysis.loop_header(loops[0]), block0);
        assert_eq!(loop_analysis.loop_header(loops[1]), block1);
        assert_eq!(loop_analysis.loop_parent(loops[1]), Some(loops[0]));
        assert_eq!(loop_analysis.loop_parent(loops[0]), None);
        assert_eq!(loop_analysis.is_in_loop(block0, loops[0]), true);
        assert_eq!(loop_analysis.is_in_loop(block0, loops[1]), false);
        assert_eq!(loop_analysis.is_in_loop(block1, loops[1]), true);
        assert_eq!(loop_analysis.is_in_loop(block1, loops[0]), true);
        assert_eq!(loop_analysis.is_in_loop(block2, loops[1]), true);
        assert_eq!(loop_analysis.is_in_loop(block2, loops[0]), true);
        assert_eq!(loop_analysis.is_in_loop(block3, loops[0]), true);
        assert_eq!(loop_analysis.is_in_loop(block0, loops[1]), false);
        assert_eq!(loop_analysis.loop_level(block0).level(), 1);
        assert_eq!(loop_analysis.loop_level(block1).level(), 2);
        assert_eq!(loop_analysis.loop_level(block2).level(), 2);
        assert_eq!(loop_analysis.loop_level(block3).level(), 1);
    }

    #[test]
    fn nested_loops_variant2_detection() {
        let id = "test::nested_loops_test".parse().unwrap();
        let mut function = Function::new(id, Signature::new([AbiParam::new(Type::I1)], []));

        let block0 = function.dfg.entry_block();
        let block1 = function.dfg.create_block();
        let block2 = function.dfg.create_block();
        let block3 = function.dfg.create_block();
        let block4 = function.dfg.create_block();
        let block5 = function.dfg.create_block();
        let block6 = function.dfg.create_block();
        let exit = function.dfg.create_block();
        {
            let mut builder = FunctionBuilder::new(&mut function);
            let cond = {
                let args = builder.block_params(block0);
                args[0]
            };

            // block0 is outside of any loop
            builder.switch_to_block(block0);
            builder
                .ins()
                .cond_br(cond, block1, &[], exit, &[], SourceSpan::UNKNOWN);

            // block1 simply branches to a loop header
            builder.switch_to_block(block1);
            builder.ins().br(block2, &[], SourceSpan::UNKNOWN);

            // block2 is the outer loop, which is conditionally entered
            builder.switch_to_block(block2);
            builder
                .ins()
                .cond_br(cond, block3, &[], exit, &[], SourceSpan::UNKNOWN);

            // block3 simply branches to a nested loop header
            builder.switch_to_block(block3);
            builder.ins().br(block4, &[], SourceSpan::UNKNOWN);

            // block4 is the inner loop, which is conditionally escaped to the outer loop
            builder.switch_to_block(block4);
            builder
                .ins()
                .cond_br(cond, block5, &[], block6, &[], SourceSpan::UNKNOWN);

            // block5 is the loop body of the inner loop
            builder.switch_to_block(block5);
            builder.ins().br(block4, &[], SourceSpan::UNKNOWN);

            // block6 is a block along the exit edge of the inner loop to the outer loop
            builder.switch_to_block(block6);
            builder.ins().br(block2, &[], SourceSpan::UNKNOWN);

            // the exit loop leaves the function
            builder.switch_to_block(exit);
            builder.ins().ret(None, SourceSpan::UNKNOWN);
        }

        let mut analysis = FunctionAnalysis::new(&function);
        analysis.ensure_loops(&function);
        let loop_analysis = analysis.loops();
        let domtree = analysis.domtree();

        let loops = loop_analysis.loops().collect::<Vec<Loop>>();
        assert_eq!(loops.len(), 2);
        assert_eq!(loop_analysis.loop_header(loops[0]), block2);
        assert_eq!(loop_analysis.loop_header(loops[1]), block4);
        assert_eq!(loop_analysis.loop_parent(loops[1]), Some(loops[0]));
        assert_eq!(loop_analysis.loop_parent(loops[0]), None);
        assert_eq!(loop_analysis.is_in_loop(block0, loops[0]), false);
        assert_eq!(loop_analysis.is_in_loop(block0, loops[1]), false);
        assert_eq!(loop_analysis.is_in_loop(block1, loops[0]), false);
        assert_eq!(loop_analysis.is_in_loop(block1, loops[1]), false);
        assert_eq!(loop_analysis.is_in_loop(block2, loops[0]), true);
        assert_eq!(loop_analysis.is_in_loop(block2, loops[1]), false);
        assert_eq!(loop_analysis.is_in_loop(block3, loops[0]), true);
        assert_eq!(loop_analysis.is_in_loop(block3, loops[1]), false);
        assert_eq!(loop_analysis.is_in_loop(block4, loops[0]), true);
        assert_eq!(loop_analysis.is_in_loop(block4, loops[1]), true);
        assert_eq!(loop_analysis.is_in_loop(block5, loops[0]), true);
        assert_eq!(loop_analysis.is_in_loop(block5, loops[1]), true);
        assert_eq!(loop_analysis.is_in_loop(block6, loops[0]), true);
        assert_eq!(loop_analysis.is_in_loop(block6, loops[1]), false);
        assert!(domtree.dominates(block4, block6, &function.dfg));
        assert_eq!(loop_analysis.is_in_loop(exit, loops[0]), false);
        assert_eq!(loop_analysis.is_in_loop(exit, loops[1]), false);
        assert_eq!(loop_analysis.loop_level(block0).level(), 0);
        assert_eq!(loop_analysis.loop_level(block1).level(), 0);
        assert_eq!(loop_analysis.loop_level(block2).level(), 1);
        assert_eq!(loop_analysis.loop_level(block3).level(), 1);
        assert_eq!(loop_analysis.loop_level(block4).level(), 2);
        assert_eq!(loop_analysis.loop_level(block5).level(), 2);
        assert_eq!(loop_analysis.loop_level(block6).level(), 1);
    }

    #[test]
    fn complex_loop_detection() {
        let id = "test::complex_loop_test".parse().unwrap();
        let mut function = Function::new(id, Signature::new([AbiParam::new(Type::I1)], []));

        let entry = function.dfg.entry_block();
        let block1 = function.dfg.create_block();
        let block2 = function.dfg.create_block();
        let block3 = function.dfg.create_block();
        let block4 = function.dfg.create_block();
        let block5 = function.dfg.create_block();
        let block6 = function.dfg.create_block();
        let block7 = function.dfg.create_block();
        {
            let mut builder = FunctionBuilder::new(&mut function);
            let cond = {
                let args = builder.block_params(entry);
                args[0]
            };

            builder.switch_to_block(entry);
            builder.ins().br(block1, &[], SourceSpan::UNKNOWN);

            builder.switch_to_block(block1);
            builder
                .ins()
                .cond_br(cond, block2, &[], block4, &[], SourceSpan::UNKNOWN);

            builder.switch_to_block(block2);
            builder.ins().br(block3, &[], SourceSpan::UNKNOWN);

            builder.switch_to_block(block3);
            builder
                .ins()
                .cond_br(cond, block2, &[], block6, &[], SourceSpan::UNKNOWN);

            builder.switch_to_block(block4);
            builder.ins().br(block5, &[], SourceSpan::UNKNOWN);

            builder.switch_to_block(block5);
            builder
                .ins()
                .cond_br(cond, block4, &[], block6, &[], SourceSpan::UNKNOWN);

            builder.switch_to_block(block6);
            builder
                .ins()
                .cond_br(cond, block1, &[], block7, &[], SourceSpan::UNKNOWN);

            builder.switch_to_block(block7);
            builder.ins().ret(None, SourceSpan::UNKNOWN);
        }

        let mut analysis = FunctionAnalysis::new(&function);
        analysis.ensure_loops(&function);
        let loop_analysis = analysis.loops();

        let loops = loop_analysis.loops().collect::<Vec<Loop>>();
        assert_eq!(loops.len(), 3);
        assert_eq!(loop_analysis.loop_header(loops[0]), block1);
        assert_eq!(loop_analysis.loop_header(loops[1]), block4);
        assert_eq!(loop_analysis.loop_header(loops[2]), block2);
        assert_eq!(loop_analysis.loop_parent(loops[1]), Some(loops[0]));
        assert_eq!(loop_analysis.loop_parent(loops[2]), Some(loops[0]));
        assert_eq!(loop_analysis.loop_parent(loops[0]), None);
        assert_eq!(loop_analysis.is_in_loop(block1, loops[0]), true);
        assert_eq!(loop_analysis.is_in_loop(block2, loops[2]), true);
        assert_eq!(loop_analysis.is_in_loop(block3, loops[2]), true);
        assert_eq!(loop_analysis.is_in_loop(block4, loops[1]), true);
        assert_eq!(loop_analysis.is_in_loop(block5, loops[1]), true);
        assert_eq!(loop_analysis.is_in_loop(block6, loops[0]), true);
        assert_eq!(loop_analysis.loop_level(block1).level(), 1);
        assert_eq!(loop_analysis.loop_level(block2).level(), 2);
        assert_eq!(loop_analysis.loop_level(block3).level(), 2);
        assert_eq!(loop_analysis.loop_level(block4).level(), 2);
        assert_eq!(loop_analysis.loop_level(block5).level(), 2);
        assert_eq!(loop_analysis.loop_level(block6).level(), 1);
    }
}
