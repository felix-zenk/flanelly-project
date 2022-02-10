use std::{fmt::{Display, Debug}};
use serde::{Serialize, Deserialize};

/// A program variable `x` is just a reference to a string.
#[derive(PartialEq,Clone,Eq,Hash,Debug,Serialize,Deserialize)]
pub struct VarName(String);

impl VarName {
    pub fn new(s: &str) -> VarName { VarName(s.to_string()) }
}

impl Display for VarName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let VarName(s) = self;
        write!(f, "{}", s)
    }
}