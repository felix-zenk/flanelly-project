use std::{fmt::Display, collections::HashSet};
use serde::{Serialize, Deserialize};

use crate::aexp::*;

/// Boolean expression
#[derive(PartialEq,Clone,Debug,Serialize,Deserialize,Eq,Hash)]
pub enum BExp {
    LessEq(Box<AExp>, Box<AExp>),
}

impl Display for BExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            BExp::LessEq(left, right) => {write!(f, "{} <= {}", left, right)}
        }
    }
}

impl BExp {
    pub fn sub_aexps(&self) -> HashSet<AExp> {
        match self {
            BExp::LessEq(a1, a2) => {
                // Rust Expl.: See also `AExp::sub_aexps` for a more detailed explanation 
                a1.sub_aexps().union(&a2.sub_aexps()).cloned().collect()
            }
        }
    }
}