use std::{collections::HashSet, fmt::Display};
use serde::{Serialize, Deserialize};

use crate::common::*;

/// Arithmetic expression
#[derive(PartialEq,Clone,Debug,Eq,Hash,Serialize,Deserialize)]
pub enum AExp {
    Num(i32),
    Var(VarName),
    Add(Box<AExp>, Box<AExp>),
    Mul(Box<AExp>, Box<AExp>)
}

impl AExp {
    /// Return `true` if there exists a variable somewhere in the arithmetic expression
    pub fn contains_var(&self, x: &VarName) -> bool {
        return match self {
            AExp::Add(a1, a2) | AExp::Mul(a1, a2) => {
                a1.contains_var(x) | a2.contains_var(x)
            }
            AExp::Num(_) => {
                false
            }
            AExp::Var(v) => {
                v == x
            }
        }
    }

    pub fn sub_aexps(&self) -> HashSet<AExp> {
        match self {
            AExp::Num(_) | AExp::Var(_) => {
                // Singleton set
                let mut set = HashSet::new();
                set.insert(self.clone());
                set
            }
            AExp::Add(a1, a2) | AExp::Mul(a1, a2) => {
                let sub_aexps1 = a1.sub_aexps();
                let sub_aexps2 = a2.sub_aexps();
                // Rust Expl.: Create an iterator over `&AExp`
                let iter = sub_aexps1.union(&sub_aexps2);
                // Rust Expl.: `iter.cloned()` creates an iterator over `AExp` (this is possible because `AExp` implements `Clone`). `collect()` uses this iterator to fill a `HashSet`.
                let mut set: HashSet<AExp> = iter.cloned().collect();
                set.insert(self.clone());
                set
            }
        }
    }

    /// This helper function pretty-prints an arithmetic expression just like `fmt`, but inserting parentheses for addition terms. It (mutually) recurses on `fmt`.
    fn fmt_with_parens(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AExp::Num(n) => {write!(f, "{}", n)}
            AExp::Var(v) => {write!(f, "{}", v)}
            AExp::Add(left, right) => {write!(f, "({} + {})", left, right)}
            AExp::Mul(left, right) => {write!(f, "{}*{}", left, right)}
        }
    }
}

impl Display for AExp {
    /// This function recurses on itself (by the `write!` macro) and it (mutually) recurses on `fmt_with_parens` in order to add parentheses when needed.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            AExp::Num(n) => {write!(f, "{}", n)}
            AExp::Var(v) => {write!(f, "{}", v)}
            AExp::Add(left, right) => {write!(f, "{} + {}", left, right)}
            AExp::Mul(left, right) => {
                left.fmt_with_parens(f)?;
                write!(f, "*")?;
                right.fmt_with_parens(f)
            }
        }
    }
}