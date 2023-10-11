use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use rustc_hash::FxHashMap;
use smallvec::SmallVec;

use miden_hir as hir;

/// This represents a node in a [DependencyGraph]
///
/// In general, nodes are instructions, with edges representing dependencies
/// between instructions. However, because a dependency graph is constructed
/// locally for a single basic block, references to values which were defined
/// in dominating blocks cannot be represented as instructions. Instead, we
/// choose to represent those dependencies as values on the operand stack, as
/// by definition those values _must_ be on the operand stack upon entry to the
/// current basic block.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Node {
    /// This node type represents a value known to be on the
    /// operand stack upon entry to the current block, i.e.
    /// it's definition is external to this block, but available.
    Stack(hir::Value),
    /// This node represents an instruction in the current block,
    /// as well as the index of that instruction in the block as
    /// it originally appeared. This is used for ordering nodes
    /// relative to each other, such that the original program order
    /// is preferred by default.
    Inst(hir::Inst, u16),
}
impl Node {
    /// Returns true if this node represents an instruction in the current block
    #[inline]
    pub fn is_instruction(&self) -> bool {
        matches!(self, Self::Inst(_, _))
    }

    /// Fallibly converts this node to its corresponding instruction identifier
    #[inline]
    pub fn as_instruction(&self) -> Option<hir::Inst> {
        match self {
            Self::Inst(inst, _) => Some(*inst),
            _ => None,
        }
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Stack(ref a), Self::Stack(ref b)) => a.cmp(b),
            (Self::Stack(_), _) => Ordering::Less,
            (_, Self::Stack(_)) => Ordering::Greater,
            (Self::Inst(ref a_i, ref a), Self::Inst(ref b_i, ref b)) => {
                a.cmp(b).reverse().then(a_i.cmp(b_i))
            }
        }
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Stack(value) => write!(f, "Stack({value})"),
            Self::Inst(inst, _) => write!(f, "Inst({inst})"),
        }
    }
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Stack(value) => write!(f, "{value}"),
            Self::Inst(inst, _) => write!(f, "{inst}"),
        }
    }
}

/// Uniquely identifies a [Dependency] in a [DependencyGraph]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DependencyId(usize);
impl DependencyId {
    #[inline(always)]
    pub const fn new(id: usize) -> Self {
        Self(id)
    }

    #[inline(always)]
    pub const fn as_usize(&self) -> usize {
        self.0
    }
}

/// This structure represents an edge in a [DependencyGraph].
///
/// It specifies which [Node] is the dependent, and which is the dependency,
/// along with which values produced by the dependency are used by the
/// dependent, and how many times each are used.
#[derive(Clone, PartialEq, Eq)]
pub struct Dependency {
    /// The instruction which has the dependency
    ///
    /// NOTE: Even though this is a [Node], it must always be an instruction, as
    /// stack values cannot have dependencies (they are always leaves in the graph).
    pub dependent: Node,
    /// The instruction or stack slot which is depended on
    pub dependency: Node,
    /// Dependencies on instruction nodes depend on one or more values
    /// produced by that instruction. Additionally, each value may be
    /// depended on multiple times. For example, the IR `add a, a` would
    /// be translated into the graph with the node corresponding to the `add`
    /// instruction having a dependency on the instruction which produces the
    /// value `a`, with that dependency having a single [Use] of `a` whose count
    /// is `2`.
    ///
    /// This representation is needed because some instructions may have zero, or
    /// multiple results, and we need to be able to determine which instruction
    /// results are needed by each dependent, and how many times, in order to correctly
    /// and efficiently emit stack manipulation code for each use.
    used: SmallVec<[Use; 1]>,
}
impl Dependency {
    /// Create a new dependency from `dependent` to `dependency`, with an empty set
    /// of uses. It is expected that uses will be added subsequently, otherwise this dependency
    /// will be considered "dead", i.e. it can be eliminated during codegen.
    #[inline]
    pub fn new(dependent: Node, dependency: Node) -> Self {
        let mut used = SmallVec::<[Use; 1]>::default();
        if let Node::Stack(value) = &dependency {
            used.push(Use {
                value: *value,
                count: 1,
            });
        }
        Self {
            dependent,
            dependency,
            used,
        }
    }

    /// Returns true if this dependency is useless/dead
    pub fn is_dead(&self) -> bool {
        self.dependency.is_instruction() && self.used.is_empty()
    }

    /// Returns the [Use] set for this dependency
    #[inline(always)]
    pub fn used(&self) -> &[Use] {
        self.used.as_slice()
    }

    /// Returns true if `value` is used by this dependency.
    #[allow(unused)]
    pub fn is_used(&self, value: &hir::Value) -> bool {
        match self.dependency {
            Node::Stack(ref dependency) => dependency == value,
            Node::Inst(_, _) => self.used.iter().any(|used| &used.value == value),
        }
    }

    /// Returns the number of times that `value` is used by this dependency.
    #[allow(unused)]
    pub fn used_count(&self, value: &hir::Value) -> usize {
        self.used
            .iter()
            .find_map(|used| {
                if &used.value == value {
                    Some(used.count as usize)
                } else {
                    None
                }
            })
            .unwrap_or(0)
    }

    /// Add a single use of `value` to this dependency
    pub fn add_use(&mut self, value: hir::Value) {
        if let Some(pos) = self.used.iter().position(|used| used.value == value) {
            let used = &mut self.used[pos];
            used.count += 1;
        } else {
            self.used.push(Use { value, count: 1 });
        }
    }

    /// Remove a use of `value` from this dependency
    ///
    /// If there are multiple uses of `value`, the counter is decremented,
    /// and when the counter reaches 0, the use is removed entirely.
    pub fn remove_use(&mut self, value: &hir::Value) {
        if let Some(pos) = self.used.iter().position(|used| &used.value == value) {
            let used = &mut self.used[pos];
            used.count -= 1;
            if used.count == 0 {
                self.used.remove(pos);
            }
        }
    }
}
impl Ord for Dependency {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.dependent
            .cmp(&other.dependent)
            .then(self.dependency.cmp(&other.dependency))
    }
}
impl PartialOrd for Dependency {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl fmt::Debug for Dependency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Dependency")
            .field("dependent", &self.dependent)
            .field("dependency", &self.dependency)
            .field("used", &self.used)
            .finish()
    }
}
impl fmt::Display for Dependency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} => {}", self.dependent, self.dependency)?;
        if self.dependency.is_instruction() {
            f.write_str(" ")?;
            let mut used = f.debug_map();
            for u in self.used.iter() {
                used.entry(&format_args!("{}", &u.value), &u.count);
            }
            used.finish()
        } else {
            Ok(())
        }
    }
}

/// Represents use of a specific `value`, `count` times.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Use {
    pub value: hir::Value,
    pub count: u32,
}

/// This structure represents the dependency graph for instructions in
/// a single basic block of the SSA IR. This graph is directed and acyclic.
///
/// Nodes in the graph are instructions, or values on the operand stack, when
/// those values are external to the current basic block (i.e. defined by an
/// instruction in a dominating block).
///
/// Edges in the graph represent the specific values produced by a dependency
/// that are used by the dependent. There can only ever be a single edge connecting
/// two nodes in the graph, and the edge metadata describes not only the nodes involved,
/// and the direction of the relationship, but also the specific values used by the dependent
/// that were produced by the dependency.
///
/// The direction of edges in the graph determines which node is the dependency and
/// which is the dependent. Specifically, edges are directed from dependents to their
/// dependencies, i.e. the predecessors of a node are the nodes which depend on it.
///
/// This graph is used to produce a [TreeGraph] for the function, and to aid in scheduling
/// instructions when generating stack machine code from an SSA IR function. It is also
/// used to generate an oracle that tells us when a value on the operand stack must be
/// copied vs moved/consumed.
#[derive(Default, Clone)]
pub struct DependencyGraph {
    /// The set of nodes in the graph
    nodes: BTreeSet<Node>,
    /// A map of every node in the graph to other nodes in the graph with which it has
    /// a relationship, and which dependencies describe that relationship.
    edges: FxHashMap<Node, BTreeMap<Node, DependencyId>>,
    /// Storage for each [Dependency] corresponding to an edge in the graph
    data: Vec<Dependency>,
}
impl DependencyGraph {
    /// Add `node` to the dependency graph, if it is not already present
    pub fn add_node(&mut self, node: Node) -> Node {
        if self.nodes.insert(node) {
            self.edges.insert(node, Default::default());
        }
        node
    }

    /// Add a dependency from `a` to `b`
    ///
    /// For stack slot nodes, this automatically sets the use count for the dependency,
    /// but for instruction nodes, it is expected that the caller will add uses corresponding
    /// to instruction arguments subsequent to calling this function.
    pub fn add_dependency(&mut self, a: Node, b: Node) -> DependencyId {
        use std::collections::btree_map::Entry;
        match self.edges.get_mut(&a).unwrap().entry(b) {
            Entry::Vacant(entry) => {
                let id = DependencyId(self.data.len());
                self.data.push(Dependency::new(a, b));
                entry.insert(id);
                assert_eq!(self.edges.get_mut(&b).unwrap().insert(a, id), None);
                id
            }
            Entry::Occupied(entry) => {
                let id = *entry.get();
                if let Node::Stack(value) = b {
                    let dep = &mut self.data[id.as_usize()];
                    dep.add_use(value);
                }
                id
            }
        }
    }

    /// Get a reference to the [Dependency] corresponding to `id`
    #[inline]
    pub fn edge(&self, id: DependencyId) -> &Dependency {
        &self.data[id.as_usize()]
    }

    /// Get a mutable reference to the [Dependency] corresponding to `id`
    #[inline]
    pub fn edge_mut(&mut self, id: DependencyId) -> &mut Dependency {
        &mut self.data[id.as_usize()]
    }

    /// Get the [DependencyId] of the edge between `a` and `b`
    ///
    /// NOTE: This function will panic if there is no such edge.
    #[inline]
    pub fn edge_id(&self, a: &Node, b: &Node) -> DependencyId {
        self.edges[a][b]
    }

    /// Get an oracle structure which assigns indices to nodes in the
    /// graph in the order in which they will be emitted during code generation.
    ///
    /// To illustrate here is a sketch of the data that would be associated
    /// for a simple graph where `root` has three dependencies, each of which
    /// has zero or more dependencies of their own:
    ///
    /// ```text,ignore
    ///    6
    ///  / | \
    /// v  v  v
    /// 5  3  0
    ///    |  | \
    ///    v  v  v
    ///    4  2  1
    /// ```
    ///
    /// During code generation, we visit arguments of an instruction in LIFO order,
    /// i.e. reverse argument order, such that the first argument will be on the top
    /// of the stack, and so on. In the graph above, nodes are shown in "normal" order,
    /// i.e. going from left to right, nodes match the order in which they appear in the
    /// argument list.
    ///
    /// As you can see, the order in which dependencies are emitted constitutes a preorder,
    /// depth-first traversal of the graph, in which successors of a node are visited in
    /// reverse sorted order.
    pub fn indexed(&self, root: &Node) -> DependencyGraphIndices {
        let mut indices = DependencyGraphIndices::default();
        let mut counter = 0;
        let mut worklist = SmallVec::<[Node; 4]>::from_iter(
            self.successors(root).into_iter().map(|n| n.dependency),
        );
        while let Some(n) = worklist.pop() {
            if indices.insert(n, counter) {
                counter += 1;
            }
            for succ in self.successors(&n) {
                worklist.push(succ.dependency);
            }
        }

        // The root node is always last to be emitted
        indices.insert(*root, counter);

        indices
    }

    /// Removes `node` from the graph, along with all edges in which it appears
    pub fn remove_node(&mut self, node: &Node) {
        if self.nodes.remove(node) {
            let edges = self.edges.remove(node).unwrap();
            for (other_node, _) in edges.into_iter() {
                self.edges.get_mut(&other_node).unwrap().remove(node);
            }
        }
    }

    /// Removes an edge from `a` to `b`.
    ///
    /// If `value` is provided, the use corresponding to that value is removed, rather than
    /// the entire edge from `a` to `b`. However, if removing `value` makes the edge dead, or
    /// `value` is not provided, then the entire edge is removed.
    pub fn remove_edge(&mut self, a: &Node, b: &Node, value: Option<hir::Value>) {
        // Get the edge id that connects a <-> b
        let id = self.edges[a][b];
        let edge = &mut self.data[id.as_usize()];
        // Only continue if the direction of the edge is a->b
        if &edge.dependent != a {
            return;
        }
        if let Some(value) = value {
            // We're removing a specific value dependency
            assert!(
                b.is_instruction(),
                "invalid node type for dependency: expected instruction, got {b:?}"
            );
            // Only remove the dependency if the direction is a match
            edge.remove_use(&value);
            // If removing the dependency makes the edge meaningless, remove the edge
            if edge.is_dead() {
                self.edges.get_mut(&a).unwrap().remove(&b);
                self.edges.get_mut(&b).unwrap().remove(&a);
            }
        } else {
            // We're removing the edge
            self.edges.get_mut(&a).unwrap().remove(&b);
            self.edges.get_mut(&b).unwrap().remove(&a);
        }
    }

    /// Returns the number of predecessors, i.e. dependents, for `node` in the graph
    pub fn num_predecessors(&self, node: &Node) -> usize {
        self.edges
            .get(node)
            .map(|es| {
                es.values()
                    .filter(|id| &self.data[id.as_usize()].dependency == node)
                    .count()
            })
            .unwrap_or_default()
    }

    /// Returns an iterator over the nodes in this graph
    pub fn nodes(&self) -> impl Iterator<Item = Node> + '_ {
        self.nodes.iter().copied()
    }

    /// Returns an iterator over the predecessors, or dependents, of `node` in the graph
    pub fn predecessors<'a, 'b: 'a>(&'a self, node: &'b Node) -> Predecessors<'a> {
        Predecessors {
            node: *node,
            iter: self.edges[node].iter(),
            graph: self,
        }
    }

    /// Returns an iterator over the successors, or dependencies, of `node` in the graph
    pub fn successors<'a, 'b: 'a>(&'a self, node: &'b Node) -> Successors<'a> {
        Successors {
            node: *node,
            iter: self.edges[node].iter(),
            graph: self,
        }
    }
}
impl fmt::Debug for DependencyGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("DependencyGraph")
            .field("nodes", &DebugNodes(self))
            .field("edges", &DebugEdges(self))
            .finish()
    }
}

/// This structure is produced by [DependencyGraph::indexed], which assigns
/// an ordinal index to every [Node] in the graph based on the order in which it
/// is visited during code generation. The lower the index, the earlier it is
/// visited.
///
/// This is used to compare nodes in the graph with a common dependency to see which
/// one is the last dependent, which allows us to be more precise when we manipulate
/// the operand stack.
#[derive(Debug, Default)]
pub struct DependencyGraphIndices(FxHashMap<Node, usize>);
impl DependencyGraphIndices {
    /// Get the index of `node`
    ///
    /// NOTE: This function will panic if `node` was not in the corresponding dependency graph
    #[inline]
    pub fn get(&self, node: &Node) -> usize {
        self.0[node]
    }

    /// Assign `index` to `node`
    fn insert(&mut self, node: Node, index: usize) -> bool {
        if self.0.contains_key(&node) {
            false
        } else {
            self.0.insert(node, index);
            true
        }
    }
}

/// An iterator over each successor edge, or [Dependency], of a given node in a [DependencyGraph]
pub struct Successors<'a> {
    node: Node,
    iter: std::collections::btree_map::Iter<'a, Node, DependencyId>,
    graph: &'a DependencyGraph,
}
impl<'a> Iterator for Successors<'a> {
    type Item = &'a Dependency;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((_, id)) = self.iter.next() {
            let data = &self.graph.data[id.as_usize()];
            if data.dependent == self.node {
                return Some(data);
            }
        }

        None
    }
}
impl<'a> DoubleEndedIterator for Successors<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some((_, id)) = self.iter.next_back() {
            let data = &self.graph.data[id.as_usize()];
            if data.dependent == self.node {
                return Some(data);
            }
        }

        None
    }
}
impl<'a> ExactSizeIterator for Successors<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

/// An iterator over each predecessor edge, or [Dependency], of a given node in a [DependencyGraph]
pub struct Predecessors<'a> {
    node: Node,
    iter: std::collections::btree_map::Iter<'a, Node, DependencyId>,
    graph: &'a DependencyGraph,
}
impl<'a> Iterator for Predecessors<'a> {
    type Item = &'a Dependency;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((_, id)) = self.iter.next() {
            let data = &self.graph.data[id.as_usize()];
            if data.dependency == self.node {
                return Some(data);
            }
        }

        None
    }
}
impl<'a> DoubleEndedIterator for Predecessors<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some((_, id)) = self.iter.next_back() {
            let data = &self.graph.data[id.as_usize()];
            if data.dependency == self.node {
                return Some(data);
            }
        }

        None
    }
}
impl<'a> ExactSizeIterator for Predecessors<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

struct DebugNodes<'a>(&'a DependencyGraph);
impl<'a> fmt::Debug for DebugNodes<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.0.nodes.iter()).finish()
    }
}

struct DebugEdges<'a>(&'a DependencyGraph);
impl<'a> fmt::Debug for DebugEdges<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut edges = f.debug_list();
        for node in self.0.nodes.iter() {
            for edge in self.0.successors(node) {
                edges.entry(&format_args!("{}", edge));
            }
        }
        edges.finish()
    }
}
