use std::{collections::HashSet, fmt::Debug, fmt::Display};
use serde::{Serialize, Deserialize};

use vec1::Vec1;

use crate::cfg::{NodeIdx, RawAnnot};
use crate::cfg::Cfg;
use crate::flow_analysis::common::SemiLat;

use super::common::FlowSemantics;

/// An annotation consisting of a pre-value and a post-value. Both values will be elements of the property space `T`.
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Eq, Hash)]
pub struct MfpAnnot<L> {
    pre: L,
    post: L,
}

pub fn mfp<L: SemiLat + FlowSemantics>(cfg_raw: &Cfg<RawAnnot>) -> Cfg<MfpAnnot<L>> {
    // Init CFG
    let mut cfg = cfg_raw.map(|_| MfpAnnot::new(L::init(), L::init()));
    // Init node gets a special initialization
    cfg.graph[cfg.init].annot = MfpAnnot::new(L::init_start(), L::init_start());

    // Init worklist
    let mut worklist: HashSet<NodeIdx> = cfg.graph.node_indices().collect();
    // The init node is not really part of the CFG (it does not have any predecessors but only serves as a predecessor itself)
    worklist.remove(&cfg.init);

    while !worklist.is_empty() {
        // Take a node out of worklist
        let n = *worklist.iter().next().unwrap();
        worklist.remove(&n);

        // Combine annotations of predecessors
        let predecs: Vec1<&L> = cfg.predecessors(n).unwrap().mapped(|n_pre| &cfg.graph[n_pre].annot.post);
        cfg.graph[n].annot.pre = SemiLat::join(predecs);

        // Compute f(in_n)
        let f_in_n = FlowSemantics::eval_transfer_function(&cfg.graph[n].node, &cfg.graph[n].annot.pre);

        // If n is not stable...
        if f_in_n.ne(&cfg.graph[n].annot.post) {
            // ...update and...
            cfg.graph[n].annot.post = f_in_n;

            // ...mark successors
            // let successors = HashSet::from_iter(cfg.successors(n));  // TODO REMOVE ?
            worklist.extend(cfg.successors(n));
        }
    }

    cfg
}

/// Standard constructor
impl<L> MfpAnnot<L> {
    pub fn new(pre: L, post: L) -> Self {
        Self { pre, post }
    }
}

/// Pretty-printer
impl<L: Display> Display for MfpAnnot<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pre: ")?;
        Display::fmt(&self.pre, f)?;
        write!(f, "\npost: ")?;
        Display::fmt(&self.post, f)
    }
}