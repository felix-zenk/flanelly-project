use std::fmt::Display;
use serde::{Serialize, Deserialize};

use crate::{common::{VarName}};
use crate::aexp::*;
use crate::bexp::*;
use crate::ast::{Prog::*, ProgAtom::*};


/// A `Prog`ram represents an AST (abstract syntax tree).
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Prog {
    Prog(Vec<ProgAtom>)
}

/// A `ProgAtom` ("program atom") represents atomic statements of a program
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum ProgAtom {
    // Rust Expl.: The `Box<BExp>` type represents *references to data of the `BExp` type on the heap*. This is the mechanism used to represent arbitrarily-large syntax trees, analogously to how linked lists are implemented.
    Skip,
    Assign(VarName, Box<AExp>),
    Cond(Box<BExp>, Box<Prog>, Box<Prog>),
    While(Box<BExp>, Box<Prog>),
}

impl Display for Prog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Prog(ps) = self;
        let mut iter = ps.iter();
        match iter.next() {
            Some(p) => {
                write!(f, "{}", p)?;
                iter.try_for_each(|p| {write!(f, "; {}", p)})
            }
            None => {Ok(())}
        }
    }
}

impl Display for ProgAtom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Skip                   => {write!(f, "skip")}
            Assign(v, aexp)        => {write!(f, "{} := {}", v, aexp)}
            Cond(bexp, p_tt, p_ff) => {write!(f, "if {} then {} else {} end", bexp, p_tt, p_ff)}
            While(bexp, p)         => {write!(f, "while {} do {} end", bexp, p)}
        }
    }
}