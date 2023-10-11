use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

use rustc_hash::FxHashMap;
use smallvec::SmallVec;

use super::*;

/// A [TreeGraph] represents a condensed [DependencyGraph]. Nodes in the graph
/// are the roots of expression trees, i.e. sequences of instructions which can
/// be evaluated in reverse DFS order such that the outputs of an instruction are
/// placed on the operand stack in the correct order to be used as inputs for the
/// next instruction. Edges in the graph represent multiply-used values, i.e. uses
/// of instruction results outside that expression tree.
///
/// The [TreeGraph] is produced from a [DependencyGraph] as follows:
///
/// * Nodes with no predecessors (dependents) become roots in the graph
/// * Nodes with a single predecessor are condensed into, i.e. represented by, whichever
/// root node they are a descendent of
/// * Nodes with multiple predecessors have their predecessor edges removed, making them
/// roots in the graph.
/// * The cut set, or edges that were removed in the previous step, are then added to back
/// to the graph, connecting tree graph nodes such that the dependencies between trees are
/// represented by those edges.
///
/// The reverse topological sort of a [TreeGraph] provides a valid scheduling of the
/// instructions in the original [DependencyGraph]. The edges in the treegraph are used to
/// determine when (and how many) copies of a given instruction result are needed.
#[derive(Default, Clone)]
pub struct TreeGraph {
    /// The nodes which are explicitly represented in the graph
    nodes: BTreeSet<Node>,
    /// Edges between nodes in the graph, where an edge may carry multiple dependencies
    edges: FxHashMap<Node, BTreeMap<Node, SmallVec<[DependencyId; 2]>>>,
    /// A mapping of condensed nodes to the root node of the tree they were condensed into
    condensed: FxHashMap<Node, Node>,
    /// Storage for dependency data referenced by edges in the graph
    dependencies: Vec<Dependency>,
}
impl TreeGraph {
    /// Returns true if `node` represents a tree in this graph
    #[inline(always)]
    pub fn is_root(&self, node: &Node) -> bool {
        self.nodes.contains(node)
    }

    /// Return the node representing the root of the tree to which `node` belongs
    ///
    /// If `node` is the root of its tree, it simply returns itself.
    ///
    /// NOTE: This function will panic if `node` is not a root OR a node condensed
    /// in any tree of the graph.
    #[inline]
    pub fn root(&self, node: &Node) -> Node {
        self.condensed[node]
    }

    /// Returns true if `node` is a member of the tree represented by `root`
    ///
    /// NOTE: This function will panic if `root` is not a tree root
    pub fn is_member_of(&self, node: &Node, root: &Node) -> bool {
        assert!(self.is_root(root));
        self.root(node) == *root
    }

    /// Return the number of times that `node` is referenced as a dependency.
    ///
    /// Here, `node` can be either explicitly represented in the graph as a tree
    /// root, or implicitly, as a condensed node belonging to a tree.
    ///
    /// For tree roots, the number of dependencies can be different than the
    /// number of predecessors, as edges can carry multiple dependencies, and
    /// those dependencies can reference the root. However, tree roots by construction
    /// must have either zero, or more than one dependent.
    ///
    /// Condensed nodes on the other hand, by construction have only one dependent, as they
    /// belong to a tree; but they may also be referenced by dependencies in the edges
    /// between treegraph nodes, so we must check all edges inbound on the tree containing
    /// the node for dependencies on `node`.
    pub fn num_dependents(&self, node: &Node) -> usize {
        if self.is_root(node) {
            self.dependents(node).count()
        } else {
            self.dependents(node).count() + 1
        }
    }

    /// Return an iterator over every node which depends on `node`
    pub fn dependents(&self, node: &Node) -> impl Iterator<Item = Node> + '_ {
        let root = self.root(node);
        DependentIterator {
            target: *node,
            dependencies: self.dependencies.as_slice(),
            predecessors: self.edges[&root].iter(),
            current_node: None,
            current_id: 0,
        }
    }

    /// Return an iterator over each [Dependency] in the edge from `a` to `b`
    ///
    /// NOTE: This function will panic if either `a` or `b` are not tree roots
    pub fn edges(&self, a: &Node, b: &Node) -> EdgeIterator<'_> {
        EdgeIterator {
            edges: self.edges[a].get(b).map(|ids| ids.as_slice()),
            dependencies: self.dependencies.as_slice(),
        }
    }

    /// Return the number of predecessors for `node` in this graph.
    ///
    /// NOTE: This function will panic if `node` is not a tree root
    pub fn num_predecessors(&self, node: &Node) -> usize {
        let mut count = 0;
        for (m, ids) in self.edges[node].iter() {
            let is_pred = ids.iter().any(|id| self.dependent_root(id) == *m);
            count += is_pred as usize;
        }
        count
    }

    /// Return an iterator over [Node]s which are predecessors of `node` in this graph.
    ///
    /// NOTE: This function will panic if `node` is not a tree root
    pub fn predecessors<'a, 'b: 'a>(&'a self, node: &'b Node) -> impl Iterator<Item = Node> + 'a {
        self.edges[node].iter().filter_map(move |(m, ids)| {
            let is_pred = ids.iter().any(|id| self.dependent_root(id) == *m);
            if is_pred {
                Some(*m)
            } else {
                None
            }
        })
    }

    /// Return an iterator over [Node]s which are successors of `node` in this graph.
    ///
    /// NOTE: This function will panic if `node` is not a tree root
    pub fn successors<'a, 'b: 'a>(&'a self, node: &'b Node) -> impl Iterator<Item = Node> + 'a {
        self.edges[node].iter().filter_map(move |(m, ids)| {
            let is_pred = ids.iter().any(|id| self.dependency_root(id) == *m);
            if is_pred {
                Some(*m)
            } else {
                None
            }
        })
    }

    /// Remove the edge connecting `a` and `b`.
    ///
    /// NOTE: This function will panic if either `a` or `b` are not tree roots
    pub fn remove_edge(&mut self, a: Node, b: Node) {
        self.edges.get_mut(&a).unwrap().remove(&b);
        self.edges.get_mut(&b).unwrap().remove(&a);
    }

    /// Compute the topological sorting of this graph
    ///
    /// Returns `Ok` with a sorted vector of nodes, or `Err` if a cycle is detected.
    ///
    /// The above sort will ensure that the resulting topological sort evaluates
    /// dependencies in inverse argument order, e.g. consider the following IR:
    ///
    /// ```miden-ir,ignore
    /// blk0(a, b):
    ///   c = mul b, b          % inst1
    ///   d = add a, c          % inst2
    ///   e = eq.imm d, 0       % inst3
    ///   br blk1(d, e)         % inst4
    /// ```
    ///
    /// Our depgraph would look like so:
    ///
    /// ```text,ignore
    ///       inst4
    ///       /   \
    ///      v     v
    ///   inst2<--inst3
    ///    /   \
    ///   v     v
    /// inst1   a
    ///   |
    ///   v
    ///   b
    /// ```
    ///
    /// And the resulting treegraph would look like so:
    ///
    /// ```text,ignore
    /// inst4
    ///   |
    ///   v
    /// inst2
    /// ```
    ///
    /// So the topological sort of the treegraph, considering only the original
    /// program order, would be `inst4, inst2`, which we visit in reverse during
    /// stackification, i.e. `inst2, inst4`.
    ///
    /// The actual visit order for these nodes and their dependencies will be:
    ///
    /// 1. inst2
    ///   a. place 'a' on top of stack (visiting arguments in order)
    ///   b. inst1
    ///     * place 'b' on top of stack
    ///     * place 'b' on top of stack
    ///     * emit 'mul b, b'
    ///   c. emit 'add a, c'
    /// 2. inst4
    ///   a. place 'd' on top of stack
    ///   b. inst3
    ///     * place 'd' on top of stack
    ///     * place '0' on top of stack
    ///     * emit 'eq.imm d, 0'
    ///   c. emit 'br blk1(d, e)'
    ///
    /// Giving us the following codegen:
    ///
    /// ```miden-ir,ignore
    /// inst1:
    ///             % 1.a) no-op, 'a' is on top of stack already
    ///     swap    % 1.b.aa) swap the position of 'a', 'b', so 'b' is top of stack
    ///             %         we swap vs fetch because 'b' is not live after inst1
    ///     dup     % 1.b.bb) duplicate 'b' on top of stack, as it is used twice
    ///     mul     % 1.b.cc) pop 'b', 'b', push 'c'
    /// inst2:
    ///     swap    % 1.c) the stack at this point is [c, a], so
    ///             % we need to swap the top two elements so 'a' is top of stack
    ///             % again we swap vs fetch because 'a' is not live after inst2
    ///     add     % pop 'a', 'c', push 'd'
    /// inst3:
    ///             % 2.a) no-op, as 'd' is on top of stack already
    ///     dup     % 2.b.aa) duplicate 'd' as it is live after inst3
    ///     push.0  % 2.b.bb) push the literal '0' on top of stack
    ///     swap    % 2.b.cc) swap the top 2 elements, as we need 'd' on top,
    ///     eq      %         then pop 'd', '0', push 'e'
    /// inst4:
    ///     swap    % 2.c) the stack at this point is [e, d], so
    ///             % we need to swap the top two elements so 'd' is top of stack,
    ///             % matching the argument order expected by blk1
    /// ```
    ///
    /// This is decent, but introduces some inefficiences due to the order in which
    /// dependencies are scheduled. If we schedule dependencies such that the order
    /// they are emitted matches the order we want them to appear on the stack, we
    /// can elide a number of unnecessary stack ops. Consider if rather than visiting
    /// dependencies in program order, we visited them in LIFO order, i.e. the last
    /// dependency visited is the first one needed by the dependent:
    ///
    /// This is the same visit order as outlined above, but we instead visit instruction
    /// arguments in reverse (LIFO) order, it looks basically indistinguishable:
    ///
    /// 1. inst2
    ///   a. inst1
    ///     * place 'b' on top of stack
    ///     * place 'b' on top of stack
    ///     * emit 'mul b, b'
    ///   b. place 'a' on top of stack (visiting arguments in order)
    ///   c. emit 'add a, c'
    /// 2. inst4
    ///   a. inst3
    ///     * place '0' on top of stack
    ///     * place 'd' on top of stack
    ///     * emit 'eq.imm d, 0'
    ///   b. place 'd' on top of stack
    ///   c. emit 'br blk1(d, e)'
    ///
    /// However, let's see how that effects codegen:
    ///
    /// ```text,ignore
    /// inst1:
    ///     swap    % 1.a.aa) make the stack [b, a], rather than [a, b]
    ///             %         we swap vs fetch because 'b' is not live after inst1
    ///     dup     % 1.a.bb) duplicate 'b' on top of stack, as it is used twice
    ///     mul     % 1.a.cc) pop 'b', 'b', push 'c'
    ///             % stack is now [c, a]
    /// inst2:
    ///     swap    % 1.c) make the stack [a, c], rather than [c, a]
    ///             %      we swap vs fetch because 'a' is not live after inst2
    ///     add     % pop 'a', 'c', push 'd'
    ///             % stack is now [d]
    /// inst3:
    ///     push.0  % 2.a.aa) make the stack [0, d]
    ///     fetch.1 % 2.a.bb) duplicate 'd' to top of stack, as it is live after inst3
    ///     eq      % 2.a.cc) pop 'd', '0', push 'e'
    ///             % stack is now [e, d]
    /// inst4:
    ///     swap    % 2.b) make the stack [d, e], rather than [e, d]
    ///             %      we swap vs fetch because 'd' is not live after inst4
    ///             % 2.c) no-op, the stack matches the argument order for blk1
    /// ```
    ///
    /// This ordering has allowed us to remove an extraneous `swap` instruction. By
    /// recognizing that `add` is commutative, we can further optimize this by eliding
    /// the swap that juggles the argument order for that instruction, though that optimization
    /// can be applied in both approaches. However, visiting dependencies in LIFO order already
    /// produces better code in this tiny example, and more complex programs are likely to benefit
    /// to a greater degree
    pub fn toposort(&self) -> Result<Vec<Node>, ()> {
        let mut treegraph = self.clone();
        let mut output = Vec::<Node>::with_capacity(treegraph.nodes.len());
        let mut roots = treegraph
            .nodes
            .iter()
            .copied()
            .filter(|n| treegraph.num_predecessors(n) == 0)
            .collect::<VecDeque<_>>();

        let mut successors = SmallVec::<[Node; 4]>::default();
        while let Some(n) = roots.pop_front() {
            output.push(n);
            successors.clear();
            successors.extend(treegraph.successors(&n));
            for m in successors.iter().copied() {
                treegraph.remove_edge(n, m);
                if treegraph.num_predecessors(&m) == 0 {
                    roots.push_back(m);
                }
            }
        }

        if treegraph.edges.values().all(|es| es.is_empty()) {
            Ok(output)
        } else {
            Err(())
        }
    }

    #[inline(always)]
    fn dependency_root(&self, id: &DependencyId) -> Node {
        self.root(&self.dependencies[id.as_usize()].dependency)
    }

    #[inline(always)]
    fn dependent_root(&self, id: &DependencyId) -> Node {
        self.root(&self.dependencies[id.as_usize()].dependent)
    }
}
impl From<DependencyGraph> for TreeGraph {
    fn from(mut depgraph: DependencyGraph) -> Self {
        let mut cutset = Vec::<Dependency>::default();
        let mut treegraph = Self::default();

        // Build cutset
        for node in depgraph.nodes() {
            let is_multi_use = depgraph.num_predecessors(&node) > 1;
            if is_multi_use {
                cutset.extend(depgraph.predecessors(&node).cloned());
            }
        }

        // Apply cutset
        for edge in cutset.iter() {
            depgraph.remove_edge(&edge.dependent, &edge.dependency, None);
        }

        // Add roots to treegraph
        for node in depgraph.nodes() {
            if depgraph.num_predecessors(&node) == 0 {
                treegraph.nodes.insert(node);
                treegraph.edges.insert(node, Default::default());
            }
        }

        // Construct mapping from dependency graph nodes to their
        // corresponding treegraph nodes
        let mut worklist = VecDeque::<Node>::default();
        for root in treegraph.nodes.iter().copied() {
            worklist.push_back(root);
            while let Some(node) = worklist.pop_front() {
                treegraph.condensed.insert(node, root);
                for succ in depgraph.successors(&node) {
                    worklist.push_back(succ.dependency);
                }
            }
        }

        // Add cutset edges to treegraph
        for edge in cutset.into_iter() {
            let id = DependencyId::new(treegraph.dependencies.len());
            let a = treegraph.condensed[&edge.dependent];
            let b = treegraph.condensed[&edge.dependency];
            treegraph.dependencies.push(edge);
            let outgoing = treegraph.edges.get_mut(&a).unwrap();
            outgoing.entry(b).or_insert_with(Default::default).push(id);
            let incoming = treegraph.edges.get_mut(&b).unwrap();
            incoming.entry(a).or_insert_with(Default::default).push(id);
        }

        treegraph
    }
}
impl fmt::Debug for TreeGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TreeGraph")
            .field("nodes", &DebugNodes(self))
            .field("edges", &DebugEdges(self))
            .finish()
    }
}

struct DebugNodes<'a>(&'a TreeGraph);
impl<'a> fmt::Debug for DebugNodes<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.0.nodes.iter()).finish()
    }
}

struct DebugEdges<'a>(&'a TreeGraph);
impl<'a> fmt::Debug for DebugEdges<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut edges = f.debug_list();
        for node in self.0.nodes.iter() {
            for edge in self.0.successors(node) {
                edges.entry(&format_args!("{} => {}", node, edge));
            }
        }
        edges.finish()
    }
}

/// An iterator over each [Dependency] represented by the edge between two treegraph nodes.
pub struct EdgeIterator<'a> {
    edges: Option<&'a [DependencyId]>,
    dependencies: &'a [Dependency],
}
impl<'a> Iterator for EdgeIterator<'a> {
    type Item = &'a Dependency;

    fn next(&mut self) -> Option<Self::Item> {
        let edges = self.edges.as_mut()?;
        let (item, rest) = edges.split_first()?;
        *edges = rest;
        Some(&self.dependencies[item.as_usize()])
    }
}

struct DependentIterator<'a> {
    target: Node,
    dependencies: &'a [Dependency],
    predecessors: std::collections::btree_map::Iter<'a, Node, SmallVec<[DependencyId; 2]>>,
    current_node: Option<(Node, &'a [DependencyId])>,
    current_id: usize,
}
impl<'a> Iterator for DependentIterator<'a> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((node, ids)) = self.current_node.take().or_else(|| {
            self.predecessors
                .next()
                .map(|(n, ids)| (*n, ids.as_slice()))
        }) {
            for current_id in self.current_id..ids.len() {
                let dependency = &self.dependencies[ids[current_id].as_usize()];
                if dependency.dependency == self.target {
                    let next_id = current_id + 1;
                    if next_id < ids.len() {
                        self.current_node = Some((node, ids));
                        self.current_id = next_id;
                    } else {
                        self.current_node = None;
                        self.current_id = 0;
                    }
                    return Some(node);
                }
            }
        }

        None
    }
}
