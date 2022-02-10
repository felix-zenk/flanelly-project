use crate::{common::VarName, cfg::Node};
use crate::aexp::*;
use serde::{Serialize, Deserialize};

use super::common::{FlowSemantics, SemiLat};

use std::{collections::HashSet, fmt::Display, hash::Hash};

/// # "Expression Set" Lattice 
/// - Used for tracking the available expressions for the available expressions analysis
/// - Partial order: `s1 <= s2   <=>   s1.set.is_superset(s2)` (See how subset vs. superset is exchanged here - but this is just convention, so that it fits our definition of `join_bin` instead of `meet_bin`)
/// - For now: Only arithmetic expressions, could be extended in the future to boolean expressions and more
/// - Internal implementation as a hash set
#[derive(Debug,PartialEq,Clone,Eq,Serialize,Deserialize)]
pub struct ExpSetLat {
    set: HashSet<AExp>
}

impl Hash for ExpSetLat {
    fn hash<H: std::hash::Hasher>(&self, _: &mut H) { }
}

impl ExpSetLat {
    pub fn new(set: HashSet<AExp>) -> Self { Self { set } }

    /// Remove all expressions that contain a variable `x`
    pub fn clear_var(&mut self, x: &VarName) {
        self.set.retain(|a| !a.contains_var(x));
    }

    /// Add a set of expressions
    pub fn extend(&mut self, set: HashSet<AExp>) {
        set.into_iter().for_each(|a| {self.set.insert(a);});
    }
}

/// `ExpSetLat` forms a semi-lattice, where the `join_bin` operation is identified as "meet_bin" in the literature. Note that this is all about conventions, any "join-semi-lattice" can be viewed as an upside-down "meet-semi-lattice"
/// Here thus, `join_bin` means "intersection"
impl SemiLat for ExpSetLat {
    fn join_bin(self: &Self, other: &Self) -> Self {
        let intersection: HashSet<AExp, _> = self.set.intersection(&other.set).cloned().collect();
        ExpSetLat::new(intersection)
    }
}

impl FlowSemantics for ExpSetLat {
    fn eval_transfer_function(n: &Node, set: &Self) -> Self {
        todo!()
    }

    /// In the beginning, no expression is available
    fn init_start() -> Self {
        Self::init()
    }

    /// The init element is the "top" element of the semi-lattice, i.e. the empty set
    fn init() -> Self {
        ExpSetLat::new(HashSet::new())
    }
}

/// Pretty-printer
impl Display for ExpSetLat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        let mut iter = self.set.iter();
        match iter.next() {
            Some(a) => {
                write!(f, "{:}", a)?;
                iter.try_for_each(|a| {write!(f, ", {:}", a)})?;
            }
            None => {}
        }
        write!(f, "}}")
    }
}