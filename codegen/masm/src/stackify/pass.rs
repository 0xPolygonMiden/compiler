use std::{collections::VecDeque, fmt, rc::Rc};

use cranelift_entity::packed_option::ReservedValue;
use miden_hir::{self as hir, assert_matches, BranchInfo, Immediate, Instruction, ProgramPoint};
use miden_hir_analysis::{DominatorTree, FunctionAnalysis, LivenessAnalysis, Loop, LoopAnalysis};
use miden_hir_pass::Pass;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;

use crate::masm::{self, Op};

use super::{
    emit::{InstOpEmitter, OpEmitter},
    *,
};

/// This pass transforms Miden IR to MASM IR, which is a representation of Miden
/// Assembly we use a subset of in Miden IR already for inline assembly, but is
/// extended in this crate with modules and functions.
///
/// # Background
///
/// MASM IR is a superset of the representation we use for inline assembly in Miden IR,
/// extended with functions and modules so that we can represent the artifacts we produce
/// during code generation. MASM IR is a stack machine representation, unlike Miden IR
/// which is in SSA form, a type of register machine with infinite registers.
///
/// ## Miden vs Other Stack VMs
///
/// The machine represented by MASM IR (the Miden virtual machine), is not only a stack
/// machine, but one with some unique constraints compared to your typical run-of-the-mill
/// stack machine:
///
/// * The control flow graph of the entire program must be a tree, not a directed (a)cylic graph.
/// * Miden does provide facilities for cyclic control flow, in the form of very basic looping
/// idioms: `while.true` and `repeat.N` (where `N` is a constant). Rather than representing these
/// using back edges in a control flow graph, these instructions are self-contained nodes in a
/// control flow tree, representing a sub-tree of the program that will be executed on each iteration
/// of the loop.
/// * As a result of the control flow graph constraints, recursion is not permitted. The call graph
/// of the program must be topographically orderable.
/// * In addition to the recursion constraint, indirect function calls are not supported either.
/// All callees must be known statically. This may change with the introduction of the `PCALL`
/// instruction, but for now this is a hard restriction.
///
/// ## MAST
///
/// These restrictions are primarily imposed due to the design tradeoffs made in Miden. In particular,
/// Miden Assembly is not the form executed by the Miden VM, rather it is compiled to a MAST, or
/// _Merkelized Abstract Syntax Tree_. A MAST provides both data integrity and compression. By relying
/// on some of the properties of Merkle trees, they can be used to verify the integrity of the program
/// to be executed. As you might imagine, this data structure is a tree, so programs represented in
/// MAST form are themselves required to be trees. In a MAST, the root of the tree represents the full
/// program, and each node in the tree represents a subprogram. Each path in the tree represents a possible
/// execution path the program can take. More precisely, the root of the MAST is the hash of the whole
/// program, and each child of the root node is either a leaf node, a hash of the subprogram that node
/// represents, or a non-leaf node, which like the root, is a hash of its children.
///
/// By using a MAST representation, it is possible to represent the execution of a program as a sequence
/// of hashes which identify which node of the MAST is executed next. So the first hash identifies where
/// execution begins, i.e. the root, the second hash identifies which child node of the root is executed
/// next, and so on. This makes it possible to represent an execution trace of a program without shipping
/// the entire program, only the hashes of the MAST nodes that were actually executed.
///
/// So in short, while Miden Assembly (and MASM IR as a result) seem oddly designed, there is a rationale
/// behind that design, and its tradeoffs are what have dictated some of the more awkward parts of the
/// instruction set. There are some other considerations that affect things like why we have u32 instructions
/// but no other "standard" integral types. We won't get into that here.
///
/// One final note: you might wonder why Miden Assembly doesn't have a particular instruction. In most
/// cases the fundamental limit is either due to running out of space in the opcode byte (we only have
/// 8 bits to encode the opcode). There are a few other implementation details in the VM that I'm less
/// familiar with that also constraint what instructions we can add (instructions typically require
/// helper registers, and we have a very limited set of those as well).
///
/// # Stackification
///
/// We call the set of transformations applied by this pass _stackification_, which refers to one of the most
/// important tasks it performs: converting the Miden IR instruction set, with its use of SSA values (which
/// can be thought of as virtual registers) into the MASM instruction set, where equivalent semantics require
/// us to maneuver values on the operand stack; put another way, we're "stackifying" registers.
///
/// This pass has a couple prerequisites, which are implemented in transformation passes in [miden-hir-transform]:
///
/// * The linker has been run on all modules that will be in the final [Program], and we have that on hand.
/// * No cycles in the control flow graph (except loop headers). This is handled by the [Treeify] pass.
/// * All blocks have only a single predecessor (except loop headers). Also handled by the [Treeify] pass.
/// * All unconditional branches have been inlined (except those leading to loop headers). Handled by the [InlineBlocks] pass.
/// * Implicitly, there can be no critical edges due to the above constraints, but that is handled well before
///   [Treeify], just prior to liveness analysis, by the [SplitCriticalEdges] pass.
///
/// The Miden IR we are given is then transformed as follows:
///
/// * In a reverse postorder traversal of the control flow graph, we visit each block, and
///   emit MASM IR according to a scheduling we compute.
/// * The instruction schedule for a Miden IR basic block is based on the [TreeGraph] data structure.
///   This scheduling is intended to make maximal use of the operand stack without needing to spill locals,
///   or perform excess operand stack manipulation. The treegraph is computed based on the data flow dependency
///   graph of the instructions in each basic block, represented by [DependencyGraph].
/// * As we emit code, we emulate the state of the operand stack at each point, so that we can determine
///   what stack manipulation ops are needed. See [OperandStack].
/// * Miden IR instructions are not a 1:1 match with MASM instructions, so some additional work is done
///   to map the higher-level instructions and their semantics to the more limited set of MASM instructions.
/// * We convert global variable accesses into their actual memory addresses here. The linker has done the
///   work of laying things out for us, so we are simply asking the [Program] at what offset a given global
///   has been allocated, and then using that address.
/// * When emitting the entrypoint for a program, we insert a prologue which initializes any data segments
///   used by the program.
///
/// The output of the pass is a MASM IR program which can be run via the emulator, or emitted to disk.
///
/// # Instruction Scheduling
///
/// Instruction scheduling is determined by a combination of factors:
///
/// * The data dependencies between instructions, and between blocks
/// * The order of arguments required by each instruction
/// * Control dependencies, i.e. we can't execute the terminator of a block until we execute all other
///   instructions in the block.
///
/// The actual algorithm is outlined below, and is performed on a per-block basis:
///
/// 1. Construct a [DependencyGraph] for the block. This graph represents the data flow dependencies
///    for each instruction in the block, as well as accounting for values inherited from dominating blocks,
///    in the form of either instruction results or block arguments.
/// 2. Condense the dependency graph into a [TreeGraph].
///   * Each node in the tree graph represents a node in the dependency graph which either:
///     * Has no predecessors, i.e. it is the root of an expression tree. For example, the expression
///       `1 + 1 * 2` is a tree, whose root is the `*` operator, and whose operands are the leaf node `2`,
///       and the subtree expression `1 + 1`.
///     * Has multiple uses, i.e. it is a value which must be duplicated on the operand stack in order
///       to keep it live across all uses.
///   * Edges in the tree graph represent data dependencies between an instruction in the condensed subtree
///     of the dependency graph represented by a given treegraph node (the dependent node), and another treegraph
///     node that represents an instruction which produces a value with multiple uses (the dependency node).
///     The dependency node may also be a condensed subtree, but the root of that tree is always the instruction
///     which produces the value in question. Each edge in the treegraph carries with it all of the necessary
///     information to identify what values are used, and by which instructions.
/// 3. Compute the topographical ordering of the [TreeGraph]. This ordering ensures that all dependencies
///    come before their dependents, falling back to the original program order for nodes with no data
///    dependencies between them. The block terminator is always placed last, to reflect the control dependency.
/// 4. Schedule the treegraph nodes by visiting them in reverse topological order.
/// 5. Schedule the instructions in the condensed subtree of the dependency graph represented by each treegraph node.
///    This is done using a postorder DFS traversal of the dependency graph starting from the point corresponding
///    to the treegraph node. The order in which sibling dependencies are visited is in reverse argument order
///    of the instruction being scheduled. The dependency graph (and tree graph) are consulted during this process
///    to determine when values can be safely consumed or require copies to be made. The operand stack tells us
///    where values are on the operand stack at each step, so we can emit the proper stack manipulation instructions.
///
/// This approach has the effect of placing operands on the operand stack in as close to optimal order as possible,
/// in fact it is guaranteed that for data dependency graphs which are trees, the order _is_ optimal. In general though,
/// the order is not always optimal, due to the presence of multiply-used values, or instructions with multiple results
/// whose order is fixed, and may require some stack manipulation to adjust.
///
/// Importantly, this approach allows us to forgo the need for locals/temporaries, as we are able to keep values
/// on the operand stack for their entire live range, and only for as long as necessary. We do still use locals
/// for automatic allocations (i.e. temporaries that we'd ordinarily need to allocate heap memory for, but
/// should be freed when the call returns).
///
/// # Recovering Structured Control Flow
///
/// Miden Assembly only provides us with a very limited set of (3) structured control flow ops, two of which interest
/// us in terms of code generation: `if.true` and `while.true`. There are no arbitrary jumps between blocks of code
/// in a function, each control operator has a well-defined entry and exit point, with no other means of entering or
/// exiting the block of code which constitutes the body of that control operator.
///
/// Miden IR however does not have these kind of high-level structured control flow instructions. A Miden IR function is
/// a flat list of basic blocks, and each basic block is a flat sequence of instructions. Control flow in Miden IR is
/// unstructured, using jumps (conditional, unconditional, and table-based) to form the edges of a directed, possibly
/// cyclic, graph.
///
/// Because Miden IR control flow is unstructured, and we need structured control flow for Miden Assembly, we must perform
/// a transformation which recovers structured control flow from an unstructed control flow graph. Because unstructured
/// control flow is so flexible, we must decompose it into some combination of structured control ops that give us equivalent
/// semantics to the original control flow graph. This is a complex process, but uses some simple building blocks:
///
/// First, let's examine the two MASM instructions mentioned above. Both pop a condition off the operand stack, and take
/// one of two paths through the program, depending on the instruction. For `if.true`, one of the two blocks representing
/// the branches of the if/else. For `while.true`, the loop is either (re)entered, or it is skipped/exited, and control
/// resumes after the `while.true` body. The boolean which controls the loop must be fetched/recomputed both before
/// the `while.true` is reached, and at the very end of the `while.true` body.
///
/// Looking closer at `while.true`, we see that how the loop is controlled is a bit different than Miden IR.
/// The way we'd represent the equivalent of `while.true` is composed of three basic blocks:
///
/// 1. The loop header. This is where the predicate that controls the loop is evaluated. This block is terminated
///    by a conditional jump to either the loop body, or the loop exit.
/// 2. The loop body. In practice this could be many blocks, depending on what is in the body of the `while.true`,
///    but in this case we'll say its just one, at the end of which we have an unconditional jump back to the loop header.
/// 3. The loop exit. This represents the point where control flow is joined when either bypassing the loop or exiting it,
///    in MASM it represents the code that immediately follows the end of the `while.true` statement.
///
/// However we cannot directly translate the loop structure I just described to Miden Assembly, because the `while.true`
/// instruction does not have a code block which corresponds to the loop header! Instead the predicate exists in two
/// places: at the instruction just before the `while.true`; and at the end of the `while.true` body. So when we have
/// a loop like this in Miden IR, we must actually lower it as an `if.true` nested in a `while.true`. The outer `while.true`
/// is entered unconditionally (by pushing `true` on the operand stack), and then the inner `if.true` is used to represent
/// the semantics of the Miden IR loop, i.e. the true and false branches represent entry into the loop body and exit from
/// it, respectively. The false branch of the `if.true` contains a `push.0` to break out of the outer `while.true`.
/// This lowering mimics the structure of the Miden IR loop.
///
/// In generalizing this approach to more complex loop structures, I concluded the following:
///
/// * When you reach the end of a MASM code block, this is equivalent to reaching an edge in the IR control flow graph (CFG):
///   1. Unless otherwise noted, a code block implicitly ends with an unconditional jump to the next code block to be executed
///   2. If there are no further code blocks to be executed, instead of an unconditional jump, control returns from the
///      function with whatever is on the operand stack.
///   3. Both `if.true` and `while.true` can be thought of as forks in the control flow graph, where control is typically
///      rejoined at the first instruction following the control operator. However, when there are no following instructions
///      the transfer of control depends on whether there are more code blocks to be executed or not. If there are, then
///      the join point is the start of the next block; if there are not, then control can be considered to return from the
///      function directly. An alternative, and semantically equivalent view is that control rejoins after the
///      `if.true`/`while.true`, but before evaluating any other control flow rules described here. Due to the way the
///      treeify and block inliner passes interact, in practice the IR does not have join points in the CFG except for
///      along loopback edges.
/// * We can implement a variety of loop idioms by using `while.true` as an infinite loop primitive:
///   * `while(<predicate>) { <body> }`:
///       ```masm,ignore
///       push.1
///       while.true
///         <predicate>
///         if.true
///           <body>
///         else
///           push.0
///         end
///       end
///       ```
///   * `do { <body> } while(<predicate>)`, this is the ideal case for `while.true`:
///       ```masm,ignore
///       push.1
///       while.true
///         <body>
///         <predicate>
///       end
///       ```
///   * `for (i = 0; i++; i < len) { <body> }`:
///       ```masm,ignore
///       push.len
///       push.0     # i = 0
///       push.1
///       while.true
///         dup.0    # copy i, stack before this is: [i, len], after it's: [i, i, len]
///         dup.2    # copy len
///         lt       # i < len
///         if.true
///           <body>
///           incr   # i++, we assume here that <body> leaves the stack as: [i, len]
///           push.1 # unconditionally continue loop
///         else
///           push.0
///         end
///       end
///       ```
///   * Generalizing this a bit for typical condition-controlled loops, we more or less end up with the following:
///       ```masm,ignore
///       <prologue>      # loop invariant expressions go here
///       push.1          # unconditionally enter the loop
///       while.true
///         <loop header> # the stuff that happens at the start of every iteration goes here
///         <predicate>   # technically part of the header, but must always be the last thing done
///         if.true       # this controls whether we are entering/exiting the loop
///           <loop body> # the loop body may use its position in the control flow tree to
///                       # break out of the loop directly with push.0, or continue the loop using push.1
///         else
///           <epilogue>  # this is where you'd put code that is run when exiting the loop normally
///           push.0
///         end
///       end
///       <join>          # this is where control joins whether the loop was taken or not
///       ```
/// * We can nest loops arbitrarily deep in Miden IR, and in MASM; however in Miden IR, control can
///   transfer directly to a containing loop, or even out of any containing loop, from any loop depth.
///   This can't be represented directly in MASM, instead we must break out of each intermediate loop
///   to get to the desired depth, using `push.0`, and continue with `push.1` (if applicable).
/// * Any join points in the control flow graph require that the operand stack be in the same abstract state
///   regardless of the path taken to get there. This means that from the perspective of the code at/following the
///   join point, the position of all live values on the operand stack is consistent. A program which violates
///   this rule has undefined behavior from that point onward.
/// * As implied by the previous point, any code block which is an immediate predecessor of a join point in the
///   control flow graph, must agree with the other predecessors on the state of the stack by the end of that block.
///   To ensure this, instructions may be inserted in the block to get the stack into the desired state.
///
/// These rules are used to recover structured control flow for arbitrarily complex loop patterns. The resulting
/// code is not necessarily optimal in terms of size, due to duplication of loop headers and such, but this
/// doesn't make any performance tradeoffs that I'm aware of.
pub struct Stackify<'a> {
    program: &'a hir::Program,
    analysis: &'a FunctionAnalysis,
}
impl<'a> Stackify<'a> {
    pub fn new(program: &'a hir::Program, analysis: &'a FunctionAnalysis) -> Self {
        Self { program, analysis }
    }
}
impl<'p> Pass for Stackify<'p> {
    type Input<'a> = &'a hir::Function;
    type Output<'a> = Box<masm::Function>;
    type Error = anyhow::Error;

    fn run<'a>(&mut self, f: Self::Input<'a>) -> Result<Self::Output<'a>, Self::Error> {
        self.analysis.require_all()?;

        let mut f_prime = Box::new(masm::Function::new(f.id, f.signature.clone()));

        // Start at the function entry
        {
            let entry = f.dfg.entry_block();
            let entry_prime = f_prime.body;

            let domtree = self.analysis.domtree();
            let loops = self.analysis.loops();
            let liveness = self.analysis.liveness();
            let mut emitter =
                MasmEmitter::new(self.program, f, &mut f_prime, domtree, loops, liveness);

            let mut stack = OperandStack::default();
            for arg in f.dfg.block_args(entry).iter().rev().copied() {
                let ty = f.dfg.value_type(arg).clone();
                stack.push(TypedValue { value: arg, ty });
            }

            emitter.emit(entry, entry_prime, stack);
        }

        Ok(f_prime)
    }
}

/// This structure is used to emit code for a function in the SSA IR.
struct MasmEmitter<'a> {
    /// The program to which `f` belongs
    program: &'a hir::Program,
    /// The SSA IR function being translated
    f: &'a hir::Function,
    /// The resulting stack machine function being emitted
    f_prime: &'a mut masm::Function,
    domtree: &'a DominatorTree,
    loops: &'a LoopAnalysis,
    liveness: &'a LivenessAnalysis,
    /// The "controlling" loop corresponds to the current maximum loop depth
    /// reached along the current control flow path. When we reach a loopback
    /// edge in the control flow graph, we emit a trailing duplicate of the
    /// instructions in the loop header to which we are branching. The controlling
    /// loop is used during this process to determine what action, if any, must
    /// be taken to exit the current loop in order to reach the target loop.
    controlling_loop: Option<Loop>,
    /// This is the block we're currently emitting code for
    emitting: hir::Block,
    /// This is the code block in `f_prime` which we're emitting code to currently
    current_block: masm::BlockId,
    /// This is a cache for data structures associated with a basic block which
    /// are expensive to calculate, but are known to be used multiple times for
    /// a given block. We don't cache these structures for all blocks, only loop
    /// headers.
    cached: FxHashMap<hir::Block, Rc<CacheEntry>>,
    /// This set tells us which blocks we have previously emitted code for, and
    /// thus when we're emitting instructions for a first visit or subsequent one.
    ///
    /// When visiting blocks a second time, we emit code for branch instructions
    /// differently, so it is important to track this information.
    visited: FxHashSet<hir::Block>,
}

/// Represents a cached dependency graph, tree graph, and schedule for
/// a block which is a loop header. This allows us to avoid recalculating
/// these data structures for blocks which will be visited multiple times.
struct CacheEntry {
    depgraph: DependencyGraph,
    treegraph: TreeGraph,
    schedule: Vec<Node>,
}

impl<'a> MasmEmitter<'a> {
    fn new(
        program: &'a hir::Program,
        f: &'a hir::Function,
        f_prime: &'a mut masm::Function,
        domtree: &'a DominatorTree,
        loops: &'a LoopAnalysis,
        liveness: &'a LivenessAnalysis,
    ) -> Self {
        Self {
            program,
            f,
            f_prime,
            domtree,
            loops,
            liveness,
            controlling_loop: None,
            emitting: Default::default(),
            current_block: masm::BlockId::reserved_value(),
            cached: Default::default(),
            visited: Default::default(),
        }
    }

    /// Emit code corresponding to the instructions in `b`, to `b_prime`, using `stack`
    /// as the state of the operand stack at the current point in the program.
    ///
    /// This function is called recursively, when reaching block terminators which transfer
    /// control to another block in the function. Thus we must keep track of when we're
    /// visiting a block for the first time, as well as what block we were in when we started
    /// emitting code for `b`, so that we can properly emit code for loopback edges.
    fn emit(&mut self, b: hir::Block, b_prime: masm::BlockId, mut stack: OperandStack) {
        let is_first_visit = self.visited.insert(b);
        // Update the current, controlling, and emitting blocks, but saving the previous
        // values so we can restore them when this function returns.
        let prev_block = core::mem::replace(&mut self.current_block, b_prime);
        let emitting = core::mem::replace(&mut self.emitting, b);
        let controlling_loop = if !emitting.is_reserved_value() {
            let current_loop = self.loops.innermost_loop(emitting);
            let target_loop = self.loops.innermost_loop(b);
            match (current_loop, target_loop) {
                // No loops involved
                (None, None) => {
                    assert!(is_first_visit);
                    assert_eq!(self.controlling_loop, None);
                    None
                }
                // Entering a top-level loop, set the controlling loop
                (None, Some(dst)) => {
                    assert!(is_first_visit);
                    assert_eq!(self.controlling_loop.replace(dst), None);
                    None
                }
                // Escaping a loop
                (Some(_), None) => {
                    assert!(is_first_visit);
                    // We're emitting a block along an exit edge of a loop, it must be the
                    // case here that the source block dominates the target block, so we
                    // leave the controlling loop alone, since it will be used to calculate
                    // the depth we're exiting from
                    assert!(
                        self.domtree.dominates(emitting, b, &self.f.dfg),
                        "expected {emitting} to dominate {b} here"
                    );
                    assert_matches!(self.controlling_loop, Some(_));
                    self.controlling_loop
                }
                (Some(src), Some(dst)) => {
                    let src_level = self.loops.level(src);
                    let dst_level = self.loops.level(dst);
                    if is_first_visit {
                        // We have not visited the target block before..
                        if src_level > dst_level {
                            // We're emitting a block along an exit edge of a loop, so we
                            // expect that the source block dominates the target block, and
                            // as such we will leave the controlling loop alone as it will
                            // be used to calculate the depth we're exiting to
                            assert!(
                                self.domtree.dominates(emitting, b, &self.f.dfg),
                                "expected {emitting} to dominate {b} here"
                            );
                            self.controlling_loop
                        } else if src_level < dst_level {
                            // If we're entering a nested loop, then we need to update the controlling loop
                            // to reflect the loop we've entered
                            self.controlling_loop.replace(dst)
                        } else {
                            self.controlling_loop
                        }
                    } else {
                        // We're looping back to the loop header, or a parent loop header,
                        // so leave the controlling loop unmodified, it will be reset by
                        // the emit_inst handling
                        self.controlling_loop
                    }
                }
            }
        } else {
            None
        };

        // Block arguments are already on the operand stack, but they are still named
        // after the values in the predecessor block. We rename them here with their
        // names as used in the current block. Renamed values are aliased, so it is
        // still possible to look them up by their original name.
        for (i, arg) in self.f.dfg.block_args(b).iter().copied().enumerate() {
            stack.rename(i, arg);
        }

        // If the block to be emitted is a loop header, we want to cache the results
        // of computing the dependency graph, tree graph, and schedule, as they will
        // be reused for every block which loops back to this block.
        //
        // For normal blocks, we know that those structures will only ever be used
        // once, so we have no need to cache them
        if self.loops.is_loop_header(b).is_some() {
            let cached = self
                .cached
                .entry(b)
                .or_insert_with(|| {
                    let depgraph = build_dependency_graph(b, self.f, self.liveness);
                    let treegraph = TreeGraph::from(depgraph.clone());
                    let schedule = treegraph
                        .toposort()
                        .expect("unable to topologically sort treegraph for block");
                    Rc::new(CacheEntry {
                        depgraph,
                        treegraph,
                        schedule,
                    })
                })
                .clone();
            self.emit_schedule(
                cached.schedule.as_slice(),
                &cached.depgraph,
                &cached.treegraph,
                stack,
                is_first_visit,
            );
        } else {
            assert!(is_first_visit, "unexpected cycle");
            let depgraph = build_dependency_graph(b, self.f, self.liveness);
            let treegraph = TreeGraph::from(depgraph.clone());
            let schedule = treegraph
                .toposort()
                .expect("unable to topologically sort treegraph for block");
            self.emit_schedule(
                schedule.as_slice(),
                &depgraph,
                &treegraph,
                stack,
                is_first_visit,
            );
        }

        // Restore the state of the emitter to where it was in the caller
        self.controlling_loop = controlling_loop;
        self.emitting = emitting;
        self.current_block = prev_block;
    }

    /// Emit code for the schedule corresponding to a basic block in the SSA IR
    ///
    /// The schedule is derived from the treegraph and dependency graph constructed
    /// from the instructions in the basic block.
    ///
    /// Basic blocks are emitted in CFG order.
    ///
    /// The `is_first_visit` flag marks this schedule as belonging to a basic block
    /// which was already emitted, which occurs when control flow loops back on a
    /// loop header block. This flag changes how code is emitted for the schedule,
    /// by omitting the terminator of the block, and emitting additional code to
    /// manage continuing/exiting loops between the parent block and the target block.
    #[inline]
    fn emit_schedule(
        &mut self,
        schedule: &[Node],
        depgraph: &DependencyGraph,
        treegraph: &TreeGraph,
        mut stack: OperandStack,
        is_first_visit: bool,
    ) {
        let mut emit_schedule = schedule.to_vec();
        emit_schedule.reverse();
        // In reverse topological order, visit each node of the treegraph..
        //
        // Nodes in the schedule appear in program order when no other constraints
        // are present due to dependency ordering.
        for node in schedule.iter().copied().rev() {
            self.emit_node(
                node,
                schedule,
                depgraph,
                treegraph,
                &mut stack,
                is_first_visit,
                None,
            );
        }
    }

    /// Emit code for a treegraph node (or a depedency of one)
    ///
    /// There are two different ways that this function is called, both have significance:
    ///
    /// 1. The node is being emitted from the schedule that was calculated from the current
    /// basic block's tree graph. The schedule is a topographical ordering of the tree graph,
    /// using the original program ordering to break ties. In short, it orders nodes (which
    /// correspond to the roots of expression trees) such that nodes with no dependents appear
    /// first, followed by nodes whose dependents all appear in the ordering before it, while
    /// preserving (roughly) the original program order where possible. The schedule is visited
    /// in reverse, which means we are going to emit code for a block roughly bottom-up, starting
    /// with expressions whose results are the most depended on.
    ///
    /// 2. The node is a dependency of another node being emitted. Dependencies are emitted before
    /// dependents, and some dependencies are depended upon more than once. Consider what happens
    /// when we call this function to emit a node from the schedule (each of which represents the
    /// root of an expression tree); we start by emitting the dependencies of that node, and those
    /// dependencies may have dependencies of their own, i.e. we're performing a postorder DFS
    /// traversal of the tree. Put another way, we emit code by working from the bottom of the
    /// tree upwards, processing sibling nodes (the dependencies of a given node) in LIFO order,
    /// i.e. such that the first argument of an instruction is on top of the stack by construction
    /// in the common case.
    ///
    /// As a result of emitting code this way, it can be a bit tricky to reason about what's actually
    /// happening when this function is called, but the intuition breaks down roughly like so:
    ///
    /// * A node representing an instruction only gets emitted once, all other dependencies on that
    /// instruction emit ops to copy/move the results of that instruction to where they are needed
    /// on the stack
    /// * A node's dependencies are emitted before the node itself
    /// * A node's dependencies are emitted in reverse argument order
    /// * The first time an instruction node is emitted is when code for the instruction itself
    /// is emitted, all subsequent appearances of that node result in emitting stack manipulation
    /// ops to copy/move the instruction's results into the desired position on the operand stack
    ///
    /// Some additional bits of information are provided as arguments to aid in tailoring the code
    /// emitted for a node to the context in which it is needed:
    ///
    /// * The schedule, dependency graph, and tree graph are provided for use in querying
    /// dependency information, and for constructing the last-use oracle that tells us when
    /// a node being emitted as a dependency, is the last use of that dependency
    /// * The state of the operand stack is tracked at each step, which allows us to query
    /// the location of specific values, and determine what type of operation is needed to
    /// copy/move a value to the top of the stack, or to a desired position on the stack
    /// * The position on the operand stack where we wish to place the value produced by
    /// the node being emitted can be provided, when the default value of 0 (top of stack)
    /// is not suitable
    /// * A flag indicating whether we're emitting code for the current block for the first
    /// time is provided, which controls how code for terminators is emitted
    /// * The current dependent node, if applicable, is provided so that we can query the
    /// last-use oracle to determine if that dependent represents the last use of the current
    /// dependency node, which allows us to elide unnecessary stack copies
    #[inline(never)]
    fn emit_node(
        &mut self,
        node: Node,
        schedule: &[Node],
        depgraph: &DependencyGraph,
        treegraph: &TreeGraph,
        stack: &mut OperandStack,
        is_first_visit: bool,
        dependent: Option<Node>,
    ) {
        match node {
            // We're emitting an instruction, or code to fetch one of the instruction results
            //
            // We emit code for the instruction itself when there is no dependent, or when there
            // is a dependent, and it is rooted under the same tree graph node.
            //
            // When there is a dependent, but it is rooted under a different tree graph node, it
            // represents a dependency on a multiply-used value. In such cases, the instruction
            // itself must have already been emitted, which is guaranteed by the topological ordering
            // of the graph which produced the schedule. As a result, we only need to emit stack ops
            // to copy/move the results of the referenced instruction into position.
            //
            // We know those results must still be on the operand stack, because we do not allow
            // values to be consumed unless they have no remaining dependents. By combining data
            // from liveness analysis and the dependency graph, we are able to determine when
            // values have dependencies, and whether the current dependent is the last use of a value.
            Node::Inst(inst, _) => match dependent {
                Some(dependent) => self.emit_inst_dependency(
                    node,
                    schedule,
                    depgraph,
                    treegraph,
                    stack,
                    is_first_visit,
                    dependent,
                    inst,
                ),
                None => self.emit_inst(
                    inst,
                    schedule,
                    depgraph,
                    treegraph,
                    stack,
                    is_first_visit,
                    node,
                ),
            },
            // We're emitting code for a value which is known to be on the operand stack
            // upon entry to the current block, i.e. it is the result of an instruction in
            // some predecessor block.
            //
            // These nodes are always roots in the tree graph, so when no dependent is set,
            // it means we are emitting this node from the schedule. Because dependencies are
            // always emitted before dependents, this is an opportune time for us to drop values
            // on the operand stack which are unused in this block. In practice, it is unlikely
            // that we will ever encounter a node of this type with no dependents, but we leave
            // this code in to aid in keeping the operand stack as small as possible.
            Node::Stack(value) => {
                if let Some(dependent) = dependent {
                    self.emit_stack_dependency(
                        node, schedule, depgraph, treegraph, stack, dependent, value,
                    );
                } else {
                    let pos = stack
                        .find(&value)
                        .expect("value not found on operand stack");
                    let num_dependents = treegraph.num_dependents(&node);
                    let is_live_after_block = self.liveness.is_live_after(
                        &value,
                        ProgramPoint::Inst(self.f.dfg.last_inst(self.emitting).unwrap()),
                    );
                    let mut emitter = self.emitter(stack);
                    if num_dependents == 0 && !is_live_after_block {
                        emitter.drop_operand_at_position(pos);
                    }
                }
            }
        }
    }

    /// Emit a node that represents an dependency on a value on the operand stack
    /// which was placed there by an instruction in a predecessor of the current block.
    ///
    /// Specifically, `dependent` is an instruction that requires `value` as an argument.
    /// We must move or copy it into the desired position on the operand stack, depending
    /// on whether this is the last known use of `value` in this block or it's successors.
    fn emit_stack_dependency(
        &mut self,
        node: Node,
        schedule: &[Node],
        depgraph: &DependencyGraph,
        treegraph: &TreeGraph,
        stack: &mut OperandStack,
        dependent: Node,
        value: hir::Value,
    ) {
        // We want to know if `value` is live at the end of the current block,
        // because if so, we must copy it for use within this block.
        let is_live_after_block = self.liveness.is_live_after(
            &value,
            ProgramPoint::Inst(self.f.dfg.last_inst(self.emitting).unwrap()),
        );
        let pos = stack
            .find(&value)
            .expect("value not found on operand stack");
        let num_dependents = treegraph.num_dependents(&node);
        // This is the last use of `value` if:
        //
        // 1. There is only a single dependent in this block, and `value` is not
        // live past the end of the current block
        // 2. There are multiple dependents, and:
        //   * There are no dependents left in the remaining schedule
        //   * Within the tree containing `dependent`, `dependent` is the last
        //     node in the dependency tree to be visited that has a dependency on `value`
        //   * `value` is not live past the end of the current block
        //
        // Only if one of these two options holds, can we consume `value`. In all other
        // circumstances, we must copy the value into place.
        let is_last_dependent = if num_dependents > 1 && !is_live_after_block {
            let dependent_tree = treegraph.root(&dependent);
            let current_index = schedule.iter().position(|n| n == &dependent_tree).unwrap();
            let remaining_schedule = &schedule[..current_index];
            let has_remaining_dependents = treegraph
                .predecessors(&node)
                .any(|p| remaining_schedule.contains(&p));
            let is_last_dependent_tree = !has_remaining_dependents;
            let is_last_occurrence = is_last_dependent_visited(
                dependent,
                dependent_tree,
                node,
                node,
                treegraph,
                depgraph,
                self.f,
            );
            is_last_dependent_tree && is_last_occurrence
        } else {
            !is_live_after_block
        };
        // This represents another optimization: if the dependent instruction is
        // a commutative operator, then the order of operands on the stack is less
        // strict, and we can elide moves which have no effect on the instruction result
        let is_operand_order_flexible = {
            let dependent_inst = dependent.as_instruction().unwrap();
            let ix = self.f.dfg.inst(dependent_inst);
            ix.is_binary() && ix.is_commutative()
        };
        let mut emitter = self.emitter(stack);
        if is_last_dependent {
            // This is the last usage, so move, rather than copy the value
            emitter.move_operand_to_position(pos, 0, is_operand_order_flexible);
        } else {
            // There are more usages of this value to come, so copy it to leave
            // it on the operand stack for the next usage
            emitter.copy_operand_to_position(pos, 0, is_operand_order_flexible);
        }
    }

    /// Emit a node that represents an instruction dependency.
    ///
    /// Specifically, `dependent` is an instruction that requires a result produced
    /// by `inst`. We must do some extra work to determine whether to emit code for
    /// `inst` itself, or to fetch the result needed by `dependent` from the operand
    /// stack.
    fn emit_inst_dependency(
        &mut self,
        node: Node,
        schedule: &[Node],
        depgraph: &DependencyGraph,
        treegraph: &TreeGraph,
        stack: &mut OperandStack,
        is_first_visit: bool,
        dependent: Node,
        inst: hir::Inst,
    ) {
        // When an instruction node is a dependency of a node in the same
        // tree graph tree, it is guaranteed to be the first time we have
        // observed that instruction, and thus we should emit code for the
        // instruction
        //
        // This guarantee is a result of how the tree graph is constructed,
        // and how the schedule is derived from the tree graph. Remember,
        // nodes in the tree graph represent the roots of condensed dependency
        // trees, wherein every node is either a root, or has only a single
        // predecessor, i.e. a single dependent. Edges in the tree graph
        // multiply-used values, but the topographical ordering of the schedule
        // ensures that we emit the tree containing the instruction that is
        // used before the tree containing the uses.
        //
        // Thus, if the dependent node is a member of the same tree, we know
        // that we need to emit the instruction, as it cannot possibly have
        // been emitted yet.
        //
        // Conversely, if the dependent is a member of a different tree, we
        // know that the code for the instruction _must_ have been emitted
        // already, and the results must be on the operand stack. As a result,
        // we simply need to copy/move the values we need into position on
        // the stack
        let dependent_tree = treegraph.root(&dependent);
        let dependency = depgraph.edge(depgraph.edge_id(&dependent, &node));
        debug_assert_eq!(dependency.dependent, dependent);

        if treegraph.is_member_of(&node, &dependent_tree) {
            // We still have to check if the desired value(s) are live after this
            // block, and if so, duplicate them.
            //
            // Determine which values we need copies of, and how many
            let mut copies = SmallVec::<[(hir::Value, u32); 2]>::default();
            let current_pp = ProgramPoint::Inst(self.f.dfg.last_inst(self.emitting).unwrap());
            for used in dependency.used().iter() {
                let is_live_after_block = self.liveness.is_live_after(&used.value, current_pp);
                if is_live_after_block {
                    copies.push((used.value, used.count));
                }
            }
            // Emit the instruction
            self.emit_inst(
                inst,
                schedule,
                depgraph,
                treegraph,
                stack,
                is_first_visit,
                node,
            );
            // Handle copies before we proceed
            let results = self.f.dfg.inst_results(inst);
            let mut stack_index = 0u8;
            let mut num_results = results.len();
            let mut emitter = self.emitter(stack);
            for result in results.iter().rev() {
                if let Some((_, count)) = copies.iter().find(|(v, _)| v == result) {
                    let count = *count as usize;
                    match num_results {
                        1 => emitter.dup(stack_index),
                        n => {
                            for _ in 0..count {
                                emitter.dup(stack_index);
                                emitter.movdn(n as u8);
                            }
                        }
                    }
                }
                if !dependency.is_used(result) {
                    num_results -= 1;
                    emitter.drop_operand_at_position(stack_index as usize);
                } else {
                    stack_index += 1;
                }
            }
        } else {
            // If `inst` is a root in the treegraph, then `num_dependents`
            // is equal to the number of dependencies on `inst`, as it has no
            // predecessors (dependents) in its own tree.
            //
            // If `inst` is not a root in the treegraph, then `num_dependents`
            // contains an extra dependent, representing the fact that it has
            // a dependent within its own tree, in addition to dependents in
            // other trees.
            //
            // We correct the count here so that `num_dependents` only refers
            // to the dependents in other trees of the tree graph. This count
            // tells us the maximum number of copies needed for the results of
            // `inst`.
            let num_dependents = if treegraph.is_root(&node) {
                treegraph.num_dependents(&node)
            } else {
                treegraph.num_dependents(&node) - 1
            };
            // We're emitting code for the last dependent if:
            //
            // 1. `dependent` is the only dependent, in which case the statement
            //    is vacuously true
            // 2. There are multiple dependents, and:
            //   * There are no dependents left in the remaining schedule
            //   * Within the tree containing `dependent`, `dependent` is the last
            //     node in the dependency tree to be visited that has a dependency on `inst`
            //
            // Knowing this determines if we will move or copy values on the operand
            // stack into position.
            let is_last_dependent = if num_dependents > 1 {
                // Determine the set of nodes remaining in the schedule to
                // be processed, based on the dependent we're currently processing.
                let current_index = schedule.iter().position(|n| n == &dependent_tree).unwrap();
                let remaining_schedule = &schedule[..current_index];
                // If any predecessor of the instruction node (or rather it's tree root)
                // in the treegraph appears before the dependent tree in the schedule,
                // then this isn't the last dependent.
                let inst_tree = treegraph.root(&node);
                let has_remaining_dependents = treegraph
                    .predecessors(&inst_tree)
                    .any(|p| remaining_schedule.contains(&p));
                let is_last_dependent_tree = !has_remaining_dependents;
                // If `dependent` is the last dependent node in its tree to be visisted
                // according to the way we traverse the dependency graph, then this is
                // the last use of `inst` and we can move operands rather than copy them
                let is_last_occurrence = is_last_dependent_visited(
                    dependent,
                    dependent_tree,
                    node,
                    inst_tree,
                    treegraph,
                    depgraph,
                    self.f,
                );
                is_last_dependent_tree && is_last_occurrence
            } else {
                true
            };
            // Place the values depended on top of the stack
            let inst_results = self.f.dfg.inst_results(inst);
            // This represents another optimization: if the dependent instruction is
            // a commutative operator, then the order of operands on the stack is less
            // strict, and we can elide moves which have no effect on the instruction result
            let is_operand_order_flexible = {
                let ix = self.f.dfg.inst(dependent.as_instruction().unwrap());
                ix.is_binary() && ix.is_commutative()
            };
            let mut emitter = self.emitter(stack);
            match inst_results.len() {
                // This case represents situations in which control/data dependencies on
                // an instruction are introduced, in order to affect the order in which code
                // is emitted, while not actually emitting any code for the dependency itself.
                0 => return,
                // Currently, instructions only produce 1 or no results
                1 => {
                    let operand = inst_results[0];
                    let pos = emitter
                        .stack()
                        .find(&operand)
                        .expect("could not find value on operand stack");
                    if is_last_dependent {
                        emitter.move_operand_to_position(pos, 0, is_operand_order_flexible);
                    } else {
                        emitter.copy_operand_to_position(pos, 0, is_operand_order_flexible);
                    }
                }
                // This is intended to handle instructions with multiple results in the future,
                // but for now this is entirely unused, and thus may have bugs.
                _ => {
                    // Place values on the stack in LIFO order
                    for used in dependency.used().iter().rev() {
                        assert!(inst_results.contains(&used.value));
                        let pos = emitter
                            .stack()
                            .find(&used.value)
                            .expect("could not find value on operand stack");
                        if is_last_dependent {
                            emitter.move_operand_to_position(pos, 0, is_operand_order_flexible);
                        } else {
                            emitter.copy_operand_to_position(pos, 0, is_operand_order_flexible);
                        }
                    }
                }
            }
        }
    }

    /// Emit code for a single instruction and it's dependencies
    fn emit_inst(
        &mut self,
        inst: hir::Inst,
        schedule: &[Node],
        depgraph: &DependencyGraph,
        treegraph: &TreeGraph,
        stack: &mut OperandStack,
        is_first_visit: bool,
        node: Node,
    ) {
        // Emit all dependencies of this node in LIFO order
        //
        // These dependencies roughly correspond to the instruction arguments, but we
        // may want to attach other kinds of control/data dependencies to instructions
        // which do not correspond to arguments directly. We do not do that as of yet,
        // but the idea here is to support that when the time comes.
        //
        // NOTE: There are not necessarily as many dependencies as arguments, as values
        // may be used by the current instruction multiple times, e.g. `add a, a`. What
        // we're doing here is simply ensuring that the instruction we depend on has been
        // emitted. The determination of whether a value should be copied or consumed is
        // handled when emitting the instruction itself.
        let dependencies =
            SmallVec::<[Node; 2]>::from_iter(depgraph.successors(&node).map(|d| d.dependency));
        for dependency in dependencies.into_iter().rev() {
            self.emit_node(
                dependency,
                schedule,
                depgraph,
                treegraph,
                stack,
                is_first_visit,
                Some(node),
            );
        }

        // Emit code for the instruction, as well as maintenance of the operand
        // stack as needed.
        //
        // NOTE: Instruction results are expected to be on the stack in LIFO order,
        // e.g. `x, y = inst` implies that `x` is top of stack, followed by `y`.
        match self.f.dfg.inst(inst) {
            Instruction::RetImm(hir::RetImm { arg, .. }) => {
                assert!(is_first_visit);
                let level = self.controlling_loop_level();
                let mut emitter = self.emitter(stack);
                // Upon return, the operand stack should only contain the function result(s),
                // so empty the stack before proceeding.
                emitter.truncate_stack(0);
                // Push the result on the stack
                emitter.literal(*arg);
                // If we're in a loop, push N zeroes on the stack, where N is the current loop depth
                for _ in 0..level {
                    emitter.literal(false);
                }
            }
            Instruction::Ret(hir::Ret { args, .. }) => {
                assert!(is_first_visit);
                let arg = {
                    let args = args.as_slice(&self.f.dfg.value_lists);
                    assert_eq!(args.len(), 1);
                    args[0]
                };
                let returning = stack
                    .peek()
                    .map(|o| o.value())
                    .expect("operand stack is empty");
                match returning {
                    OperandType::Value(TypedValue { value, .. }) => {
                        assert_eq!(*value, arg, "expected {arg} on top of the stack here");
                    }
                    operand => {
                        panic!(
                            "expected operand for {arg} on top of the stack here, got {operand:#?}"
                        );
                    }
                }
                let level = self.controlling_loop_level();
                let mut emitter = self.emitter(stack);
                // Upon return, the operand stack should only contain the function result(s),
                // so empty the stack, except for the return value, before proceeding.
                emitter.truncate_stack(1);
                // If we're in a loop, push N zeroes on the stack, where N is the current loop depth
                for _ in 0..level {
                    emitter.literal(false);
                }
            }
            // When we hit an unconditional branch instruction for the first time, one of the following
            // must be true:
            //
            // * We're in a normal block, branching to a loop header from outside that loop
            // * We're in a normal block, branching to a loop header from inside the loop
            // * We're in a loop header block, branching to the loop body
            //
            // Further, we know the following must be true:
            //
            // * All unconditional branches must be to a loop header, as the combination of
            // critical edge splitting, treeification, and block inlining will have removed all
            // unconditional branches except for those that lead to a loop header.
            //
            // We handle the cases described above slightly differently, depending on whether the
            // current block is a loop header or not:
            //
            // # Loop Header Blocks
            //
            // Because we're entering a loop body unconditionally, we emit `push.1, while.true`, and
            // then emit the target block in the `while.true` body.
            //
            // On edges looping back to this loop header block, we emit a copy of the loop header, sans
            // terminator, along with a `push.1` to continue the loop.
            //
            // On edges exiting the loop, we emit `push.0` to break out of the loop.
            //
            // # Normal Blocks
            //
            // We simply emit the destination block inline.
            //
            // # Block Arguments
            //
            // In both types of blocks, we must properly handle block arguments. Because blocks with block
            // arguments must by construction have multiple predecessors, we must ensure that all predecessors
            // agree on the state of the operand stack on entry to the target block. There are various ways we
            // could do this, but our approach is to ensure that all predecessors do the following:
            //
            // 1. Remove all operands on the operand stack which are no longer live upon entry to the target block,
            // with the exception of operands which are used as block arguments in the predecessor. This is necessary
            // because otherwise we can end up with different stack states on entry to the target block, depending on
            // what predecessor we came from, which will break things.
            // 2. Place block arguments on the operand stack in LIFO order, e.g. `blk1(%a, %b):` indicates that
            // the operand corresponding to `%a` should be on the top of stack upon entry to `blk1`. This is somewhat
            // arbitrary, but ensures that all predecessors, and the target block itself, agree on the state of the stack.
            Instruction::Br(hir::Br {
                destination,
                ref args,
                ..
            }) if is_first_visit => {
                let destination = *destination;
                {
                    let args = args
                        .as_slice(&self.f.dfg.value_lists)
                        .iter()
                        .copied()
                        .collect::<SmallVec<[hir::Value; 2]>>();
                    let mut emitter = OpEmitter::new(self.f_prime, self.current_block, stack);
                    drop_unused_operands_after(self.emitting, &args, &mut emitter, self.liveness);
                    prepare_stack_arguments(inst, &args, &mut emitter, self.liveness);
                }
                if self.loops.is_loop_header(self.emitting).is_some() {
                    // We're in a loop header, emit the target block inside a while loop
                    let body_blk = self.f_prime.create_block();
                    let block = self.current_block();
                    {
                        block.push(Op::PushU8(1));
                        block.push(Op::While(body_blk));
                    }
                    self.emit(destination, body_blk, stack.clone());
                } else {
                    // We're in a normal block, emit the target block inline
                    self.emit(destination, self.current_block, stack.clone());
                }
            }
            // When we reach an unconditional branch a second time, it is because a first-visit branch instruction
            // was reached which loops back to a block that was previously visited. We refer to the loop in which
            // the originating block belongs, as the "controlling" loop, and the originating block itself as the
            // "controlling" block. Loopback edges, due to how we emit code for loops, require us to emit a copy of
            // the loop header instructions (sans terminator) in the controlling block, as loop headers are always
            // outside the body of the corresponding `while.true` instruction, so any continuation of the loop requires
            // a separate copy of the header.
            //
            // In the simple case of the loopback edge targeting the loop header of the current loop, we simply emit
            // a `push.1` to continue the loop, after ensuring that the state of the stack matches the expected state.
            // See the comment for first-visit unconditional branches above for details.
            //
            // However, we must also handle the case when the loopback edge is actually to a loop header of an outer
            // loop. In such cases we emit a `push.1` to continue the target loop, but we must also emit a `push.0`
            // for each loop level between the controlling loop and the emitting block, to break out of each
            // intermediate loop.
            Instruction::Br(hir::Br { ref args, .. }) => {
                // We should only be emitting code for a block more than once if that block
                // is a loop header. All other blocks should only be visited a single time.
                assert!(
                    self.loops.is_loop_header(self.emitting).is_some(),
                    "unexpected cycle"
                );
                let args = args
                    .as_slice(&self.f.dfg.value_lists)
                    .iter()
                    .copied()
                    .collect::<SmallVec<[hir::Value; 2]>>();
                let mut emitter = OpEmitter::new(self.f_prime, self.current_block, stack);
                drop_unused_operands_after(self.emitting, &args, &mut emitter, self.liveness);
                prepare_stack_arguments(inst, &args, &mut emitter, self.liveness);
                let current_loop = self
                    .controlling_loop
                    .expect("expected controlling loop to be set");
                let current_level = self.loops.level(current_loop).level();
                let target_level = self.loops.loop_level(self.emitting).level();
                emitter.literal(true);
                for _ in 0..(current_level - target_level) {
                    emitter.literal(false);
                }
            }
            // When visiting a conditional branch for the first time, the process is much the same
            // as it is for unconditional branches, with the primary differences being:
            //
            // * When emitting code for the loop body, we must nest a `if.true` inside the body to
            // represent the conditional itself. This might seem weird, since the `while.true` op
            // is itself conditional, however there is a straightforward reason for this: code following
            // the `while.true`, representing the case where the conditional is false, would be executed on
            // all edges exiting the loop. This obviously assumes that all edges exiting a loop go through
            // the loop header, but that is commonly not the case (e.g. breaking out of a loop early). By
            // placing the conditional inside the loop, we are able to control the loop more precisely, and
            // can handle things such as continue/break without doing anything special.
            // * The code emitted for the loop, namely `while.true, if.true` is only evaluated when the loop
            // continues through the loop header block, so we are able to place stack manipulation ops in the
            // `if.true` branches specific to each edge emanating from the header. When we emit code for the
            // loop header on loopback edges, we are able to assume that the stack state is the same as it was
            // here, since the code emitted for the predecessor of the loopback edge will have ensured that for us.
            // We do not have to concern ourselves with control flow edges that continue the loop without going through
            // the loop header block, as that would constitute a nested loop in the loop analysis, so the code we
            // emit to break out of loops will handle those cases naturally.
            //
            // NOTE: With the support of additional analysis, we could emit more optimal
            // code for various loop idioms, but until that is done, we must stick with
            // the general solution.
            //
            Instruction::CondBr(hir::CondBr {
                then_dest: (then_dest, ref then_args),
                else_dest: (else_dest, ref else_args),
                ..
            }) if is_first_visit => {
                let then_blk = self.f_prime.create_block();
                let else_blk = self.f_prime.create_block();
                if let Some(current_loop_id) = self.loops.is_loop_header(self.emitting) {
                    // We need to emit a loop here
                    let body_blk = self.f_prime.create_block();
                    {
                        let block = self.current_block();
                        // We always unconditionally enter the loop the first time
                        block.extend_from_slice(&[Op::PushU8(1), Op::While(body_blk)]);
                        let block = self.block(body_blk);
                        block.push(Op::If(then_blk, else_blk));
                    }
                    // The code we're going to emit for handling block arguments and cleaning
                    // up the stack happens after the loop is entered and the conditional has
                    // been evaluated, so ensure the stack state reflects this
                    stack.pop().expect("operand stack is empty");
                    // if.true
                    let mut then_stack = stack.clone();
                    {
                        let then_args = then_args
                            .as_slice(&self.f.dfg.value_lists)
                            .iter()
                            .copied()
                            .collect::<SmallVec<[hir::Value; 2]>>();
                        let mut emitter = OpEmitter::new(self.f_prime, then_blk, &mut then_stack);
                        // Drop unused operands only if this edge is exiting the loop we're in
                        if !self.loops.is_in_loop(*then_dest, current_loop_id) {
                            drop_unused_operands_after(
                                self.emitting,
                                &then_args,
                                &mut emitter,
                                self.liveness,
                            );
                        }
                        prepare_stack_arguments(inst, &then_args, &mut emitter, self.liveness);
                    }
                    self.emit(*then_dest, then_blk, then_stack);
                    // if.false
                    let mut else_stack = stack.clone();
                    {
                        let else_args = else_args
                            .as_slice(&self.f.dfg.value_lists)
                            .iter()
                            .copied()
                            .collect::<SmallVec<[hir::Value; 2]>>();
                        let mut emitter = OpEmitter::new(self.f_prime, else_blk, &mut else_stack);
                        // Drop unused operands only if this edge is exiting the loop we're in
                        if !self.loops.is_in_loop(*else_dest, current_loop_id) {
                            drop_unused_operands_after(
                                self.emitting,
                                &else_args,
                                &mut emitter,
                                self.liveness,
                            );
                        }
                        prepare_stack_arguments(inst, &else_args, &mut emitter, self.liveness);
                    }
                    self.emit(*else_dest, else_blk, else_stack);
                } else {
                    // This is a simple conditional statement
                    {
                        let block = self.current_block();
                        block.push(Op::If(then_blk, else_blk));
                    }
                    stack.pop().expect("operand stack is empty");
                    // if.true
                    let mut then_stack = stack.clone();
                    {
                        let then_args = then_args
                            .as_slice(&self.f.dfg.value_lists)
                            .iter()
                            .copied()
                            .collect::<SmallVec<[hir::Value; 2]>>();
                        let mut emitter = OpEmitter::new(self.f_prime, then_blk, &mut then_stack);
                        drop_unused_operands_after(
                            self.emitting,
                            &then_args,
                            &mut emitter,
                            self.liveness,
                        );
                        prepare_stack_arguments(inst, &then_args, &mut emitter, self.liveness);
                    }
                    self.emit(*then_dest, then_blk, then_stack);
                    // if.false
                    let mut else_stack = stack.clone();
                    {
                        let else_args = else_args
                            .as_slice(&self.f.dfg.value_lists)
                            .iter()
                            .copied()
                            .collect::<SmallVec<[hir::Value; 2]>>();
                        let mut emitter = OpEmitter::new(self.f_prime, else_blk, &mut else_stack);
                        drop_unused_operands_after(
                            self.emitting,
                            &else_args,
                            &mut emitter,
                            self.liveness,
                        );
                        prepare_stack_arguments(inst, &else_args, &mut emitter, self.liveness);
                    }
                    self.emit(*else_dest, else_blk, else_stack);
                }
            }
            // Just like the unconditional case, when reaching a conditional branch a second time, we
            // are emitting code for a block we have already visited, i.e. a loopback edge was reached
            // while visiting a block, and we are duplicating the code for the target of that edge inline,
            // sans terminator. Like the unconditional case, there is some small amount of setup code we
            // must emit to maintain the loop state correctly.
            //
            // Unlike the unconditional case however, a conditional branch indicates that we emitted a
            // loop which has a conditional nested inside it's body. So we always continue the loop, and
            // let the nested `if.true` handle the code for this branch. We must also handle the case where
            // the loop level of the controlling loop is deeper than the emitting block, which indicates
            // that we must first break out of the intermediate loop(s) before continuing the target loop.
            //
            // NOTE: It must be the case that the state of the stack here matches that of the first visit
            // to the block being emitted, as code will have been emitted inside the `if.true` to manipulate
            // the stack based on that first visit.
            Instruction::CondBr(_) => {
                // We should only be emitting code for a block more than once if that block
                // is a loop header. All other blocks should only be visited a single time.
                assert!(
                    self.loops.is_loop_header(self.emitting).is_some(),
                    "unexpected cycle caused by branch to {}",
                    self.emitting,
                );

                let current_loop = self
                    .controlling_loop
                    .expect("expected controlling loop to be set");
                let current_level = self.loops.level(current_loop).level();
                let target_level = self.loops.loop_level(self.emitting).level();
                let mut emitter = self.emitter(stack);
                // Continue the target loop when it is reached, the top of the stack
                // prior to this push.1 instruction holds the actual conditional, which
                // will be evaluated by the `if.true` nested inside the target `while.true`
                emitter.literal(true);
                for _ in 0..(current_level - target_level) {
                    emitter.literal(false);
                }
            }
            Instruction::Switch(_) => {
                panic!("expected switch instructions to have been rewritten before stackification")
            }
            // This is a non-terminator instruction, so emit the code for it, and update the
            // stack state to reflect the changes made
            ix => self.emit_op(inst, ix, stack),
        }
    }

    /// Emit code for a non-terminator instruction, which consumes and produces values on the operand stack
    fn emit_op(&mut self, inst: hir::Inst, ix: &hir::Instruction, stack: &mut OperandStack) {
        assert!(
            !ix.opcode().is_terminator(),
            "unhandled terminator in non-terminator context: {:?}",
            ix
        );
        match ix {
            Instruction::GlobalValue(op) => self.emit_global_value(inst, op, stack),
            Instruction::UnaryOpImm(op) => self.emit_unary_imm_op(inst, op, stack),
            Instruction::UnaryOp(op) => self.emit_unary_op(inst, op, stack),
            Instruction::BinaryOpImm(op) => self.emit_binary_imm_op(inst, op, stack),
            Instruction::BinaryOp(op) => self.emit_binary_op(inst, op, stack),
            Instruction::Test(op) => self.emit_test_op(inst, op, stack),
            Instruction::Load(op) => self.emit_load_op(inst, op, stack),
            Instruction::PrimOp(op) => self.emit_primop(inst, op, stack),
            Instruction::PrimOpImm(op) => self.emit_primop_imm(inst, op, stack),
            Instruction::Call(op) => self.emit_call_op(inst, op, stack),
            Instruction::InlineAsm(op) => self.emit_inline_asm(inst, op, stack),
            // Control flow instructions are handled before `emit_op` is called
            Instruction::RetImm(_)
            | Instruction::Ret(_)
            | Instruction::Br(_)
            | Instruction::CondBr(_)
            | Instruction::Switch(_) => unreachable!(),
        }
    }

    fn emit_global_value(
        &mut self,
        inst: hir::Inst,
        op: &hir::GlobalValueOp,
        stack: &mut OperandStack,
    ) {
        assert_eq!(op.op, hir::Opcode::GlobalValue);
        let addr = self.calculate_global_value_addr(op.global);
        match self.f.dfg.global_value(op.global) {
            hir::GlobalValueData::Load { ref ty, .. } => {
                let mut emitter = self.inst_emitter(inst, stack);
                emitter.load_imm(addr, ty.clone());
            }
            hir::GlobalValueData::IAddImm { .. } | hir::GlobalValueData::Symbol { .. } => {
                let mut emitter = self.inst_emitter(inst, stack);
                emitter.stack_mut().push(addr);
            }
        }
    }

    fn emit_unary_imm_op(
        &mut self,
        inst: hir::Inst,
        op: &hir::UnaryOpImm,
        stack: &mut OperandStack,
    ) {
        let mut emitter = self.inst_emitter(inst, stack);
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

    fn emit_unary_op(&mut self, inst: hir::Inst, op: &hir::UnaryOp, stack: &mut OperandStack) {
        let result = self.f.dfg.first_result(inst);
        let mut emitter = self.inst_emitter(inst, stack);
        match op.op {
            hir::Opcode::Neg => emitter.neg(),
            hir::Opcode::Inv => emitter.inv(),
            hir::Opcode::Incr => emitter.incr(),
            hir::Opcode::Pow2 => emitter.pow2(),
            hir::Opcode::Not => emitter.not(),
            hir::Opcode::Bnot => emitter.bnot(),
            hir::Opcode::Popcnt => emitter.popcnt(),
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

    fn emit_binary_imm_op(
        &mut self,
        inst: hir::Inst,
        op: &hir::BinaryOpImm,
        stack: &mut OperandStack,
    ) {
        let mut emitter = self.inst_emitter(inst, stack);
        match op.op {
            hir::Opcode::Eq => emitter.eq_imm(op.imm),
            hir::Opcode::Neq => emitter.neq_imm(op.imm),
            hir::Opcode::Gt => emitter.gt_imm(op.imm),
            hir::Opcode::Gte => emitter.gte_imm(op.imm),
            hir::Opcode::Lt => emitter.lt_imm(op.imm),
            hir::Opcode::Lte => emitter.lte_imm(op.imm),
            hir::Opcode::Add => emitter.add_imm(op.imm, op.overflow),
            hir::Opcode::Sub => emitter.sub_imm(op.imm, op.overflow),
            hir::Opcode::Mul => emitter.mul_imm(op.imm, op.overflow),
            hir::Opcode::Div if op.overflow.is_checked() => emitter.checked_div_imm(op.imm),
            hir::Opcode::Div => emitter.unchecked_div_imm(op.imm),
            hir::Opcode::Min => emitter.min_imm(op.imm),
            hir::Opcode::Max => emitter.max_imm(op.imm),
            hir::Opcode::Mod if op.overflow.is_checked() => emitter.checked_mod_imm(op.imm),
            hir::Opcode::Mod => emitter.unchecked_mod_imm(op.imm),
            hir::Opcode::DivMod if op.overflow.is_checked() => emitter.checked_divmod_imm(op.imm),
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

    fn emit_binary_op(&mut self, inst: hir::Inst, op: &hir::BinaryOp, stack: &mut OperandStack) {
        let mut emitter = self.inst_emitter(inst, stack);
        match op.op {
            hir::Opcode::Eq => emitter.eq(),
            hir::Opcode::Neq => emitter.neq(),
            hir::Opcode::Gt => emitter.gt(),
            hir::Opcode::Gte => emitter.gte(),
            hir::Opcode::Lt => emitter.lt(),
            hir::Opcode::Lte => emitter.lte(),
            hir::Opcode::Add => emitter.add(op.overflow),
            hir::Opcode::Sub => emitter.sub(op.overflow),
            hir::Opcode::Mul => emitter.mul(op.overflow),
            hir::Opcode::Div if op.overflow.is_checked() => emitter.checked_div(),
            hir::Opcode::Div => emitter.unchecked_div(),
            hir::Opcode::Min => emitter.min(),
            hir::Opcode::Max => emitter.max(),
            hir::Opcode::Mod if op.overflow.is_checked() => emitter.checked_mod(),
            hir::Opcode::Mod => emitter.unchecked_mod(),
            hir::Opcode::DivMod if op.overflow.is_checked() => emitter.checked_divmod(),
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

    fn emit_test_op(&mut self, _inst: hir::Inst, op: &hir::Test, _stack: &mut OperandStack) {
        unimplemented!("unrecognized test opcode: '{}'", &op.op);
    }

    fn emit_load_op(&mut self, inst: hir::Inst, op: &hir::LoadOp, stack: &mut OperandStack) {
        let mut emitter = self.inst_emitter(inst, stack);
        emitter.load(op.ty.clone());
    }

    fn emit_primop_imm(&mut self, inst: hir::Inst, op: &hir::PrimOpImm, stack: &mut OperandStack) {
        let mut emitter = self.inst_emitter(inst, stack);
        match op.op {
            hir::Opcode::AssertEq => {
                emitter.assert_eq_imm(op.imm);
            }
            // Store a value at a constant address
            hir::Opcode::Store => {
                emitter.store_imm(
                    op.imm
                        .as_u32()
                        .expect("invalid address immediate: out of range"),
                );
            }
            opcode => unimplemented!("unrecognized primop with immediate opcode: '{opcode}'"),
        }
    }

    fn emit_primop(&mut self, inst: hir::Inst, op: &hir::PrimOp, stack: &mut OperandStack) {
        let args = op.args.as_slice(&self.f.dfg.value_lists);
        let mut emitter = self.inst_emitter(inst, stack);
        match op.op {
            // Pop a value of the given type off the stack and assert it's value is one
            hir::Opcode::Assert => {
                assert_eq!(args.len(), 1);
                emitter.assert();
            }
            // Pop a value of the given type off the stack and assert it's value is zero
            hir::Opcode::Assertz => {
                assert_eq!(args.len(), 1);
                emitter.assertz();
            }
            // Pop two values of the given type off the stack and assert equality
            hir::Opcode::AssertEq => {
                assert_eq!(args.len(), 2);
                emitter.assert_eq();
            }
            // Allocate a local and push its address on the operand stack
            hir::Opcode::Alloca => {
                assert!(args.is_empty());
                let result = emitter.dfg().first_result(inst);
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

    fn emit_call_op(&mut self, inst: hir::Inst, op: &hir::Call, stack: &mut OperandStack) {
        assert_ne!(op.callee, self.f.id, "unexpected recursive call");

        let mut emitter = self.inst_emitter(inst, stack);
        match op.op {
            hir::Opcode::Syscall => emitter.syscall(op.callee),
            hir::Opcode::Call => emitter.exec(op.callee),
            opcode => unimplemented!("unrecognized procedure call opcode: '{opcode}'"),
        }
    }

    fn emit_inline_asm(&mut self, inst: hir::Inst, op: &hir::InlineAsm, stack: &mut OperandStack) {
        // Port over the blocks from the inline assembly chunk, except the body block, which will
        // be inlined at the current block
        let mut mapped = FxHashMap::<masm::BlockId, masm::BlockId>::default();
        for (inline_blk, _) in op.blocks.iter() {
            if inline_blk == op.body {
                continue;
            }
            let mapped_blk = self.f_prime.create_block();
            mapped.insert(inline_blk, mapped_blk);
        }

        // Inline the body, rewriting any references to other blocks
        rewrite_inline_assembly_block(self.f_prime, op, op.body, self.current_block, &mapped);

        // Pop arguments, push results
        stack.dropn(op.args.len(&self.f.dfg.value_lists));
        for result in self.f.dfg.inst_results(inst).iter().copied().rev() {
            let ty = self.f.dfg.value_type(result).clone();
            stack.push(TypedValue { value: result, ty });
        }
    }

    /// Computes the absolute offset (address) represented by the given global value
    fn calculate_global_value_addr(&self, mut gv: hir::GlobalValue) -> u32 {
        let global_table_offset = self.program.segments().next_available_offset();
        let mut relative_offset = 0;
        let globals = self.program.globals();
        loop {
            let gv_data = self.f.dfg.global_value(gv);
            relative_offset += gv_data.offset();
            match gv_data {
                hir::GlobalValueData::Symbol { name, .. } => {
                    let var = globals
                        .find(*name)
                        .expect("linker should have caught undefined global variables");
                    let base_offset = unsafe { globals.offset_of(var) };
                    if relative_offset >= 0 {
                        return (global_table_offset + base_offset) + relative_offset as u32;
                    } else {
                        return (global_table_offset + base_offset) - relative_offset.abs() as u32;
                    }
                }
                hir::GlobalValueData::IAddImm { base, .. } => {
                    gv = *base;
                }
                hir::GlobalValueData::Load { base, .. } => {
                    gv = *base;
                }
            }
        }
    }

    /// Get a mutable reference to the current block of code in the stack machine IR
    #[inline(always)]
    fn current_block(&mut self) -> &mut masm::Block {
        &mut self.f_prime.blocks[self.current_block]
    }

    /// Get a mutable reference to a specific block of code in the stack machine IR
    #[inline(always)]
    fn block(&mut self, block: masm::BlockId) -> &mut masm::Block {
        &mut self.f_prime.blocks[block]
    }

    #[inline(always)]
    fn inst_emitter<'c, 'b: 'c>(
        &'b mut self,
        inst: hir::Inst,
        stack: &'c mut OperandStack,
    ) -> InstOpEmitter<'c> {
        InstOpEmitter::new(self.f_prime, &self.f.dfg, inst, self.current_block, stack)
    }

    #[inline(always)]
    fn emitter<'c, 'b: 'c>(&'b mut self, stack: &'c mut OperandStack) -> OpEmitter<'c> {
        OpEmitter::new(self.f_prime, self.current_block, stack)
    }

    /// Get the loop level of the block we're currently emitting code for
    ///
    /// When emitting trailing loop headers, the block in which we are emitting
    /// that code, i.e. the "controlling" block, is the one whose loop level we
    /// care about.
    ///
    /// In all other circumstances, it is the loop level of the block we're emitting.
    #[inline]
    fn controlling_loop_level(&self) -> usize {
        if let Some(lp) = self.controlling_loop {
            self.loops.level(lp).level()
        } else {
            self.loops.loop_level(self.emitting).level()
        }
    }
}

fn rewrite_inline_assembly_block(
    f_prime: &mut masm::Function,
    asm: &hir::InlineAsm,
    prev: masm::BlockId,
    new: masm::BlockId,
    rewrites: &FxHashMap<masm::BlockId, masm::BlockId>,
) {
    let body = asm.blocks[prev].ops.clone();
    for mut op in body.into_iter() {
        match op {
            Op::If(ref mut then_blk, ref mut else_blk) => {
                let prev_then_blk = *then_blk;
                let prev_else_blk = *else_blk;
                *then_blk = rewrites[&prev_then_blk];
                *else_blk = rewrites[&prev_else_blk];
                rewrite_inline_assembly_block(f_prime, asm, prev_then_blk, *then_blk, rewrites);
                rewrite_inline_assembly_block(f_prime, asm, prev_else_blk, *else_blk, rewrites);
            }
            Op::While(ref mut body_blk) | Op::Repeat(_, ref mut body_blk) => {
                let prev_body_blk = *body_blk;
                *body_blk = rewrites[&prev_body_blk];
                rewrite_inline_assembly_block(f_prime, asm, prev_body_blk, *body_blk, rewrites);
            }
            Op::LocAddr(_) => todo!(),
            _ => (),
        }
        f_prime.blocks[new].push(op);
    }
}

/// This function ensures that the values of `args` used by `inst` are on the
/// top of `stack`, in order such that when popping the top item off the stack,
/// the first item in `args` is returned, and so on.
///
/// This takes into account liveness data, so that values which are only used
/// by `inst` are consumed, but values used by later instructions are duplicated
/// so that they remain available on the stack.
fn prepare_stack_arguments(
    inst: hir::Inst,
    args: &[hir::Value],
    emitter: &mut OpEmitter<'_>,
    liveness: &LivenessAnalysis,
) {
    match args.len() {
        // No alignment needed
        0 => return,
        // If there is only one argument, then if that argument is on top
        // of the stack, we're done, otherwise we should fetch it to the top
        // of the stack
        1 => {
            let arg = &args[0];
            let pos = emitter
                .stack()
                .find(arg)
                .expect("could not find value on the operand stack");
            let is_used_later = liveness.is_live_after(&arg, ProgramPoint::Inst(inst));
            if is_used_later {
                emitter.copy_operand_to_position(pos, 0, false);
            } else {
                emitter.move_operand_to_position(pos, 0, false);
            }
        }
        // There are multiple arguments, and we need to determine what the most
        // efficient set of swaps is needed to get the stack in the state we want
        // it. We must also factor in values which are used later vs consumed by
        // the destination block.
        n => {
            // Compute the minimal set of ops needed to get the block arguments
            // into position on top of the stack.
            let mut ops = SmallVec::<[Op; 2]>::default();
            let mut visited = FxHashSet::<usize>::default();
            let stack = emitter.stack();
            for i in 0..n {
                if visited.insert(i) {
                    let expected = &args[i];
                    let mut j = stack.find(expected).unwrap();
                    while visited.insert(j) {
                        let is_used_later =
                            liveness.is_live_after(expected, ProgramPoint::Inst(inst));
                        if j >= n {
                            // The expected value is not within a permutation
                            // of the top of the stack, so we must either fetch
                            // it or move it to the top of the stack, depending
                            // on liveness
                            if is_used_later {
                                ops.push(Op::Dup(j as u8));
                            } else {
                                ops.push(Op::Movup(j as u8));
                            }
                            // There is no cycle to break here, so go back
                            // to the outer loop
                            break;
                        } else {
                            // We've found a cycle, so perform the swap, and follow
                            // the location of the swapped value to check for additional
                            // members of the cycle.
                            ops.push(Op::Swap(j as u8));
                            // The next item to visit is given by the position on the stack
                            // containing the value which is supposed to be the `j`th item.
                            j = stack.find(&args[j]).unwrap();
                        }
                    }
                }
            }

            // Emit the stack ops we determined were needed
            for op in ops.into_iter() {
                match op {
                    Op::Dup(i) => {
                        emitter.dup(i);
                    }
                    Op::Movup(i) => {
                        emitter.movup(i);
                    }
                    Op::Swap(i) => {
                        emitter.swap(i);
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

/// Emit code to remove values on the operand stack which are no longer live after `block`,
/// while preserving those values which are in `used`.
///
/// This function visits values on the operand stack top to bottom, keeping values in
/// order, while grouping drops to the extent possible.
fn drop_unused_operands_after(
    block: hir::Block,
    used: &[hir::Value],
    emitter: &mut OpEmitter<'_>,
    liveness: &LivenessAnalysis,
) {
    let pp = ProgramPoint::Block(block);
    let mut index = 0;
    while index < emitter.stack_len() {
        match emitter.stack()[index].value() {
            OperandType::Value(TypedValue { value: ref v, .. }) => {
                let keep = used.contains(v) || liveness.is_live_after(v, pp);
                let is_duplicate = false; // TODO: !seen.insert(*v);
                if is_duplicate || !keep {
                    emitter.drop_operand_at_position(index);
                } else {
                    index += 1;
                }
            }
            OperandType::Const(_) => {
                emitter.drop_operand_at_position(index);
            }
            operand @ OperandType::Type(_) => {
                panic!("unexpected operand type on stack: {operand:?}")
            }
        }
    }
}

/// Determine if `dependent` is the last dependent on `dependency` in the dependency graph.
///
/// This function is used as an oracle for choosing between moving or copying operands on
/// the stack when emitting code for an instruction.
fn is_last_dependent_visited(
    dependent: Node,
    dependent_tree: Node,
    dependency: Node,
    dependency_tree: Node,
    treegraph: &TreeGraph,
    depgraph: &DependencyGraph,
    _function: &hir::Function,
) -> bool {
    let indices = depgraph.indexed(&dependent_tree);
    let dependent_index = indices.get(&dependent);
    for succ in treegraph.edges(&dependent_tree, &dependency_tree) {
        if succ.dependent != dependent && succ.dependency == dependency {
            let index = indices.get(&succ.dependent);
            // We've found another dependent that comes after `dependent` in the dependency graph
            if index > dependent_index {
                return false;
            }
        }
    }

    // If we reach here, `dependent` is the shallowest dependent in the dependency graph,
    // but if the edge between `dependent` and `dependency` is multiplexed, i.e. the same
    // instruction result is used twice, we must determine if the current index corresponds
    // to the last visit of any value on that edge.
    if let Node::Inst(_inst, _) = dependency {
        let dependency_id = depgraph.edge_id(&dependent, &dependency);
        let dependency = depgraph.edge(dependency_id);
        let used = dependency.used();
        if used.len() > 1 {
            // Look at instruction arguments to see if the current index is the last possible one to visit
            todo!()
        } else {
            // If there is only one value used, and it is only used once, this is the shallowest dependent
            used[0].count == 1
        }
    } else {
        // This is a stack value, so this must be the last use
        true
    }
}

fn build_dependency_graph(
    block_id: hir::Block,
    function: &hir::Function,
    liveness: &LivenessAnalysis,
) -> DependencyGraph {
    let mut graph = DependencyGraph::default();

    // For each instruction, record it's uses + defs in the graph
    for (inst_index, inst) in function.dfg.block_insts(block_id).enumerate() {
        let node = graph.add_node(Node::Inst(inst, inst_index as u16 + 1));

        let pp = ProgramPoint::Inst(inst);
        for arg in function.dfg.inst_args(inst).iter().copied() {
            add_data_dependency(node, arg, pp, function, &mut graph);
        }

        match function.dfg.analyze_branch(inst) {
            BranchInfo::SingleDest(_, args) => {
                // Add edges representing these data dependencies in later blocks
                for arg in args.iter().copied() {
                    add_data_dependency(node, arg, pp, function, &mut graph);
                }
            }
            BranchInfo::MultiDest(ref jts) => {
                // Add edges representing these data dependencies in later blocks
                for jt in jts.iter() {
                    for arg in jt.args.iter().copied() {
                        add_data_dependency(node, arg, pp, function, &mut graph);
                    }
                }
            }
            BranchInfo::NotABranch => (),
        }
    }

    // Perform dead-code elimination
    //
    // For every node in the graph with no predecessors (no uses), and which
    // produces no values live beyond it's defining block, then if the node
    // corresponds to an instruction with no side-effects, it may be eliminated
    // as dead.
    let mut worklist = VecDeque::<(hir::Inst, u16)>::default();
    for node in graph.nodes() {
        if let Node::Inst(inst, inst_index) = node {
            // If there are predecessors in the graph, there are local uses of the instruction
            if graph.num_predecessors(&node) > 0 {
                continue;
            }

            // If there are no predecessors in the graph, but the instruction produces
            // results which are live after the instruction, then there are non-local uses of
            // the instruction, and cannot be DCE'd
            let pp = ProgramPoint::Inst(inst);
            let has_live_results = function
                .dfg
                .inst_results(inst)
                .iter()
                .any(|v| liveness.is_live_after(v, pp));
            if has_live_results {
                continue;
            }

            // Visit this instruction during dead code elimination
            worklist.push_back((inst, inst_index));
        }
    }

    while let Some((inst, inst_index)) = worklist.pop_front() {
        let has_effect = function.dfg.inst(inst).has_side_effects();
        // If this instruction has no side effects, it can be removed
        if !has_effect {
            // Add direct children to worklist, if they have no other predecessors
            let node = Node::Inst(inst, inst_index);
            for succ in graph.successors(&node) {
                // We only care about instruction nodes
                if let Node::Inst(inst, inst_index) = succ.dependency {
                    // And only if we're the only predecessor
                    if graph.num_predecessors(&succ.dependency) == 1 {
                        worklist.push_back((inst, inst_index));
                    }
                }
            }
            // Remove this node
            graph.remove_node(&node);
        }
    }

    graph
}

fn add_data_dependency(
    node: Node,
    value: hir::Value,
    pp: ProgramPoint,
    function: &hir::Function,
    graph: &mut DependencyGraph,
) {
    match function.dfg.value_data(value) {
        hir::ValueData::Inst { inst: dep_inst, .. } => {
            let dep_inst = *dep_inst;
            let block_id = function.dfg.pp_block(pp);
            if function.dfg.insts[dep_inst].block == block_id {
                let dep_inst_index = function
                    .dfg
                    .block_insts(block_id)
                    .position(|id| id == dep_inst)
                    .unwrap();
                let dep_node = graph.add_node(Node::Inst(dep_inst, dep_inst_index as u16 + 1));
                let id = graph.add_dependency(node, dep_node);
                let dep = graph.edge_mut(id);
                dep.add_use(value);
            } else {
                let dep_node = graph.add_node(Node::Stack(value));
                graph.add_dependency(node, dep_node);
            };
        }
        hir::ValueData::Param { .. } => {
            let dep_node = graph.add_node(Node::Stack(value));
            graph.add_dependency(node, dep_node);
        }
    }
}

/// Used to print an instruction schedule during debugging
struct DebugSchedule<'a>(&'a [Node], &'a hir::Function);
impl<'a> fmt::Debug for DebugSchedule<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut items = f.debug_list();
        for node in self.0.iter() {
            match node {
                Node::Stack(v) => {
                    items.entry(&format_args!("Stack({})", v));
                }
                Node::Inst(i, _) => {
                    items.entry(&format_args!("{}:{:?}", i, self.1.dfg.inst(*i)));
                }
            }
        }
        items.finish()
    }
}
