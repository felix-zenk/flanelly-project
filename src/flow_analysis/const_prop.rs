use crate::{common::VarName, cfg::Node};
use crate::aexp::*;
use serde::{Serialize, Deserialize};

use super::common::{SemiLat, FlowSemantics};

use std::{collections::HashMap, fmt::Display, hash::Hash};
use ConstLat::*;

/// # "Constant" Lattice 
/// - Used for tracking the content of a single variable for the constant propagation analysis
/// - Partial order: `Bot <= Const(n) <= Top` for all `n`
#[derive(Debug,PartialEq,Clone,Eq,Hash,Serialize,Deserialize)]
pub enum ConstLat {
    Top,
    Const(i32),
    Bot
}

impl SemiLat for ConstLat {
    fn join_bin(self: &Self, other: &Self) -> Self {
        todo!()
    }
}

/// # "Multi-Constant" Lattice
/// - Is the property space for constant propagation analysis
/// - Can be seen as a vector of `ConstLat` values, one for each variable
/// - Internal representation:
///     - `map`:     HashMap mapping variable names to `ConstLat` values
///     - `default`: The value assigned to any unspecified variable
/// - Operate on `MultiConstLat` only via its methods
#[derive(PartialEq,Clone,Eq,Debug,Serialize,Deserialize)]
pub struct MultiConstLat {
    map: HashMap<VarName, ConstLat>,
    default: ConstLat
}

impl Hash for MultiConstLat {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // TODO
        self.default.hash(state);
    }
}

impl MultiConstLat {
    pub fn new(map: HashMap<VarName, ConstLat>, default: ConstLat) -> Self { Self { map, default } }

    /// Update/insert a variable value. This mutates the object.
    pub fn insert(&mut self, x: VarName, v: ConstLat) {
        self.map.insert(x, v);
    }

    /// Lookup a variable value.
    pub fn lookup(&self, x: &VarName) -> &ConstLat {
        match self.map.get(x) {
            Some(v) => {v}
            None => {&self.default}
        }
    }

    /// Helper function: Evaluate an arithmetic expression on a `MultiConstLat` object.
    fn eval_aexp(self: &MultiConstLat, a: &AExp) -> ConstLat {
        match a {
            AExp::Num(n) => {Const(*n)}
            AExp::Var(v) => {self.lookup(v).clone()}
            AExp::Add(a1, a2) => {
                let plus = |x, y| x+y;
                self.eval_aexp(a1).eval_bin_op(plus, self.eval_aexp(a2))
            }
            AExp::Mul(a1, a2) => {
                let mul = |x, y| x*y;
                self.eval_aexp(a1).eval_bin_op(mul, self.eval_aexp(a2))
            }
        }
    }
}

impl ConstLat {
    /// Helper function: Evaluate a binary operation on a `ConstLat` object.
    fn eval_bin_op<F>(self: ConstLat, f: F, other: ConstLat) -> ConstLat
    where F: Fn(i32, i32) -> i32 {
        match (self, other) {
            (Const(v1), Const(v2)) => {Const(f(v1, v2))}
            (Top, _) | (_, Top)    => {Top}
            _                      => {Bot}
        }
    }
}

impl SemiLat for MultiConstLat {
    fn join_bin(self: &Self, other: &Self) -> Self {
        // Two phases:
        // 1) Deal with specific variable assignments (those stored in `map` attribute)
        // 2) Deal with other variable assignments (those represented by `default` attribute)

        // 1)
        // Create a new map
        let mut m = HashMap::new();
        // Iterate through all variable assignments of `self`
        self.map.iter().for_each(|(x, v1)| {
            // Get corresponding variable assignment of `other` and join
            m.insert(x.clone(), v1.join_bin(&other.lookup(x)));
        });
        // Iterate through all variable assignments of `other`:
        other.map.iter().for_each(|(x, v2)| {
            match self.map.get(x) {
                // If `x` has already an assignment, there is nothing to do.
                Some(_) => { }
                // Otherwise, join.
                None => { m.insert(x.clone(), v2.join_bin(&other.lookup(x))); }
            }
        });

        // 2)
        let d = self.default.join_bin(&other.default);
        
        MultiConstLat{ map: m, default: d}
    }
}

impl FlowSemantics for MultiConstLat {
    fn eval_transfer_function(n: &Node, mem: &Self) -> Self {
        match n {
            // `Init`, `Terminal`, `Skip` and `Branch` have no interesting semantics: They leave the memory untouched.
            Node::Init => {mem.clone()}
            Node::Terminal => {mem.clone()}
            Node::Skip => {mem.clone()}
            Node::Branch(_) => {mem.clone()}
            // Update variable on `Assign`
            Node::Assign(v, a) => {
                todo!()
            }
        }
    }

    /// According to the program semantics that were defined in the lecture, a program gets its input in the `x` variable and is executed with all other variables initially set to `0`.
    fn init_start() -> Self {
        let mut m = MultiConstLat::new(HashMap::new(), ConstLat::Const(0));
        m.insert(VarName::new("x"), ConstLat::Top);
        m
    }

    /// The init element is the "bot" element of the semi-lattice, i.e. all variables are assigned to `Bot`.
    fn init() -> Self {
        MultiConstLat { map: HashMap::new(),
                        default: ConstLat::Bot }
    }
}

/// Pretty-printer
impl Display for ConstLat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Top => {write!(f, "tt")}
            Const(c) => {write!(f, "{}", c)}
            Bot => {write!(f, "bb")}
        }
    }
}

/// Pretty-printer
impl Display for MultiConstLat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<")?;
        self.map.iter().try_for_each(|(x, v)| {write!(f, "{} = {}, ", x, v)})?;
        write!(f, "_ = {}>", self.default)
    }
}