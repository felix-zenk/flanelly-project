use std::{fmt::{Display, Debug}, collections::HashSet, hash::Hash};

use petgraph::{graph::Graph, Directed, graph::NodeIndex, EdgeDirection::Incoming, EdgeDirection::Outgoing};
use vec1::Vec1;
use serde::{Serialize, Deserialize};

use crate::{common::VarName, ast::ProgAtom, ast::Prog};
use crate::aexp::*;
use crate::bexp::*;

////////////////////
// CFG Definition //
////////////////////

/// A node index is essentially just a number
pub type NodeIdx = NodeIndex<u32>;

/// A CFG is a graph containing annotated nodes and edges, as well as a pointer to the initial node
#[derive(Debug,Serialize,Deserialize)]
pub struct Cfg<A> {
    pub graph: Graph<AnnotNode<A>, Edge, Directed>,
    pub init: NodeIdx
}

/// Node of a CFG
/// - Init:     Used only once in every CFG to mark the program entry point
/// - Terminal: Used only to tie together program flows (i.e. edges) that don't
///             point to further code
/// - Skip:     Corresponds to a skip in the AST
/// - Assign:   Corresponds to an assignment in the AST
/// - Branch:   Corresponds to either a conditional or a while loop in the AST
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq,Hash)]
pub enum Node {
    Init,
    Terminal,
    Skip,
    Assign(VarName, Box<AExp>),
    Branch(Box<BExp>)
}

/// An annotated node with a generic annotation type `T`
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq,Hash)]
pub struct AnnotNode<T> {
    pub node: Node,
    pub annot: T
}

/// This annotation could store things like line number etc. in the future
#[derive(Debug,PartialEq,Clone,Serialize,Deserialize,Eq,Hash)]
pub struct RawAnnot { }

/// Three kinds of edges exist:
/// - Plain: Standard sequencing
/// - True:  Branch here if the guard evaluates to `true`
/// - False: Branch here if the guard evaluates to `false`
#[derive(PartialEq,Clone,Debug,Serialize,Deserialize,Eq,Hash)]
pub enum Edge {
    Plain,
    True,
    False
}

///////////////////////////////
// Convert an AST into a CFG //
///////////////////////////////

/// Convert an AST into a CFG
pub fn ast_to_cfg(p: &Prog) -> Cfg<RawAnnot> {
    // The CFG is essentially a graph
    let mut g = Graph::new();
    // Construct and add the initial node of the CFG
    let node_init = g.add_node(AnnotNode::new(Node::Init, RawAnnot {}));
    let mut cfg = Cfg::new(g, node_init);

    // The function `ast_to_cfg_extend` takes the fresh CFG and does the translation
    let terminals = ast_to_cfg_extend(&mut cfg, vec![UntargEdge(node_init, Edge::Plain)], p);

    // If there are any tt/ff-edges remaining, then connect them to a terminal node
    let mut terminals_relevant = terminals.iter().filter(|UntargEdge(_, e)| {*e != Edge::Plain}).peekable();
    match terminals_relevant.peek() {
        Some(_) => {
            let node_terminal = cfg.graph.add_node(AnnotNode::new(Node::Terminal, RawAnnot {}));
            terminals_relevant.for_each(|UntargEdge(t, e)| {
                cfg.graph.add_edge(*t, node_terminal, e.clone()); ()
            })
        }
        None => {}
    }

    cfg
}


/// # Untargeted Edge
/// Intermediate data structure, only needed during `ast_to_cfg`
/// Used to collect outgoing edges whose target is yet unknown during traversal of the AST 
#[derive(Clone)]
struct UntargEdge(NodeIdx, Edge);


/// # Arguments
/// `cfg`:          Mutable CFG
/// `untarg_edges`: Loose ends of the CFG, i.e. untargeted edges
/// `p`:            Sub-AST that needs to be translated to a sub-CFG which then is
///                 integrated into the CFG (via the `untarg_edges`)
/// # Result
/// Loose ends (untargeted edges) of the translation of `p`
fn ast_to_cfg_extend(cfg: &mut Cfg<RawAnnot>, untarg_edges: Vec<UntargEdge>, p: &Prog) -> Vec<UntargEdge> {
    let Prog::Prog(ps) = p;
    // Iterate through the sub-ASTs and successively translate and connect them to the CFG.
    let mut untarg_edges_cur = untarg_edges;
    ps.into_iter().for_each(|p| {
        untarg_edges_cur = ast_atom_to_cfg_extend(cfg, untarg_edges_cur.to_vec(), &p);
    });
    untarg_edges_cur
}

/// # Arguments
/// `cfg`:          Mutable CFG
/// `untarg_edges`: Loose ends of the CFG, i.e. untargeted edges
/// `p`:            Sub-AST that needs to be translated to a sub-CFG which then is
///                 integrated into the CFG (via the `untarg_edges`)
/// # Result
/// Loose ends (untargeted edges) of the translation of `p`
fn ast_atom_to_cfg_extend(cfg: &mut Cfg<RawAnnot>, untarg_edges: Vec<UntargEdge>, p: &ProgAtom) -> Vec<UntargEdge> {
    match p {
        ProgAtom::Skip => {
            // Create new skip node.
            let skip = cfg.graph.add_node(AnnotNode::new(Node::Skip, RawAnnot {}));
            // Connect the untargeted edges to the skip node.
            untarg_edges.into_iter().for_each(|UntargEdge(t, e)| {cfg.graph.add_edge(t, skip, e); ()});
            // The skip node has exactly one untargeted edge.
            vec![UntargEdge(skip, Edge::Plain)]
        }
        ProgAtom::Assign(v, aexp) => {
            // Create new assign node.
            let assign = cfg.graph.add_node(AnnotNode::new(Node::Assign(v.clone(), aexp.clone()), RawAnnot {}));
            // Connect the untargeted edges to the assign node.
            untarg_edges.into_iter().for_each(|UntargEdge(t, e)| {cfg.graph.add_edge(t, assign, e); ()});
            // The skip node has exactly one untargeted edge.
            vec![UntargEdge(assign, Edge::Plain)]
        }
        ProgAtom::Cond(bexp, p_tt, p_ff) => {
            // Create new branch node
            let branch = cfg.graph.add_node(AnnotNode::new(Node::Branch(bexp.clone()), RawAnnot {}));
            // Connect the untargeted edges to the assign node.
            untarg_edges.into_iter().for_each(|UntargEdge(t, e)| {cfg.graph.add_edge(t, branch, e); ()});
            // Recursively translate the sub-ASTs and connect the resulting sub-CFGs via a `True` and a `False` edge.
            let node_true_untarg_edges = ast_to_cfg_extend(cfg, vec!(UntargEdge(branch, Edge::True)), p_tt);
            let node_false_untarg_edges = ast_to_cfg_extend(cfg, vec!(UntargEdge(branch, Edge::False)), p_ff);
            // Combine the untargeted edges of both sub-CFGs.
            let mut res: Vec<UntargEdge> = vec![];
            res.extend(node_true_untarg_edges);
            res.extend(node_false_untarg_edges);
            res
        }
        ProgAtom::While(bexp, p) => {
            // Create new branch node.
            let branch = cfg.graph.add_node(AnnotNode::new(Node::Branch(bexp.clone()), RawAnnot {}));
            // Connect the untargeted edges to the branch node.
            untarg_edges.into_iter().for_each(|UntargEdge(t, e)| {cfg.graph.add_edge(t, branch, e); ()});
            // Recursively translate the sub-AST and connect the resulting sub-CFG via a `True` edge.
            let p_untarg_edges = ast_to_cfg_extend(cfg, vec!(UntargEdge(branch, Edge::True)), p);
            // Connect the loose ends of the sub-CFG back to the branch node (this closes the cycle).
            p_untarg_edges.into_iter().for_each(|UntargEdge(t, e)| {cfg.graph.add_edge(t, branch, e); ()});
            // The resulting CFG has exactly one untargeted edge, labelled by `False`.
            vec![UntargEdge(branch, Edge::False)]
        }
    }
}

//////////////////////////////
// Auxiliary Infrastructure //
//////////////////////////////

/// Constructor and `map` function for CFGs
impl<A> Cfg<A> {
    /// Standard constructor
    pub fn new(graph: Graph<AnnotNode<A>, Edge, Directed>, init: NodeIdx) -> Self {
        Self { graph, init }
    }

    /// Map a Cfg<A> to a Cfg<B> by mapping the node annotations according to `f`
    pub fn map<B, F>(self: &Cfg<A>, f: F) -> Cfg<B>
    where F: Fn(&A) -> B {
        todo!()
    }

    /// Return the predecessor nodes of a given node. If there are no predecessors (only possible for the initial node), then return `None`.
    pub fn predecessors(self: &Cfg<A>, n: NodeIdx) -> Option<Vec1<NodeIdx>> {
        let predecs_vec = self.graph.neighbors_directed(n, Incoming).collect();
        match Vec1::try_from_vec(predecs_vec) {
            Ok(v) => {Some(v)}
            Err(_) => {None}
        }
    }

    /// Return the successor nodes of a given node.
    pub fn successors(self: &Cfg<A>, n: NodeIdx) -> Vec<NodeIdx> {
        self.graph.neighbors_directed(n, Outgoing).collect()
    }
}

/// Two CFGs are equal if they have the same nodes and the same edges
impl<A: PartialEq + Eq + Hash> PartialEq for Cfg<A> {
    fn eq(&self, other: &Self) -> bool {
        let a_nodes: HashSet<_> = self.graph.raw_nodes().iter().map(|n| &n.weight).collect();
        let b_nodes: HashSet<_> = other.graph.raw_nodes().iter().map(|n| &n.weight).collect();

        let a_edges: HashSet<_> = self.graph.raw_edges().iter().map(|e| (&self.graph[e.source()], &self.graph[e.target()], &e.weight)).collect();
        let b_edges: HashSet<_> = other.graph.raw_edges().iter().map(|e| (&other.graph[e.source()], &other.graph[e.target()], &e.weight)).collect();

        a_nodes == b_nodes &&
        a_edges == b_edges &&
        self.graph[self.init] == other.graph[other.init]
    }
}

impl Display for Node {
    /// Display a node
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Node::Init            => {write!(f, "init")}
            Node::Terminal        => {write!(f, "terminal")}
            Node::Skip            => {write!(f, "skip")}
            Node::Assign(v, aexp) => {write!(f, "{} := {}", v, aexp)}
            Node::Branch(bexp)    => {write!(f, "{}", bexp)}
        }
    }
}

impl<T> AnnotNode<T> {
    /// Standard constructor
    pub fn new(node: Node, annot: T) -> Self {
        Self {node, annot}
    }
}

impl<T: Display> Display for AnnotNode<T> {
    /// Display a node and its annoation, separated by a new-line
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.node, f)?;
        let mut annot_output = String::new();
        std::fmt::write(&mut annot_output, format_args!("{}", &self.annot))?;
        if !annot_output.is_empty() {
            write!(f, "\n{}", annot_output)
        }
        else {Ok(())}
    }
}

impl Display for RawAnnot {
    /// Nothing to display (could be extended in the future)
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { Ok(()) }
}

impl Display for Edge {
    /// Display an edge
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Edge::Plain => {write!(f, "")}
            Edge::True  => {write!(f, "tt")}
            Edge::False => {write!(f, "ff")}
        }
    }
}