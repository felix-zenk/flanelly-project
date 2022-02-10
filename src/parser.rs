use nom::{combinator::{peek, verify, not}, character::complete::multispace0};
use crate::ast::{*, ProgAtom::*};

use crate::aexp::{*, AExp::*};
use crate::bexp::{*, BExp::*};
use crate::common::{VarName};

use itertools::join;

use nom::character::{complete::{alpha1, digit1, anychar, multispace1}, is_alphanumeric};
use nom::branch::alt;
use nom::{multi::{separated_nonempty_list, }, IResult, bytes::complete::{tag}};
use nom::{sequence::delimited};
use nom::{sequence::{pair}};

/// Main function that does the parsing: It takes a string and produces the AST for it.
pub fn parse(s: &str) -> Result<Prog, String> {
    // First remove any comments
    // Rust Expl.: The following line declares a new binding `s`. It does not change the value of the immutable argument `s`, but merely shadows the binding and introduces a new binding `s`.
    let s = join(s.lines().map(
        |line| {
            line.find("#")
                .map(|idx| &line[..idx])
                .unwrap_or(line)
        }), "\n");
    
    // Then, remove surrounding whitespace.
    let s = s.trim();

    // Then, parse.
    match prog(&s) {
        Ok((rest, p)) => {
            if rest.is_empty() {
                Ok(p)
            }
            else {
                Err(format!("Parsing failed. The following code was not parsed. {:}", rest))
            }
        }
        Err(e) => {Err(format!("Parsing failed. {:}", e))}
    }
} 

/// Grammar for the concrete syntax:
///
/// ```latex
/// prog      ::= prog_atom ; ... ; prog_atom
/// prog_atom ::= `skip` | assign | cond | while
/// assign    ::= x `:=` aexp
/// cond      ::= `if` bexp `then` prog `else` prog `end`
/// while     ::= `while` bexp `do` prog `end`
///
/// aexp      ::= num_neg | add
/// num_neg   ::= `-`n
/// add       ::= mul + ... + mul
/// mul       ::= aexp_atom * ... * aexp_atom
/// aexp_atom ::= n | x | `(` aexp `)`
///
/// bexp      ::= lesseq
/// lesseq    ::= aexp `<=` aexp
///
/// with $n \in \mathbb{N}$ and $x \in \mathit{Var}$
/// ```

////////////////////////////////////
// Top-level Syntactic Categories //
////////////////////////////////////

/// Parse a program by first parsing semicolon-separated sub-programs, then sequencing those.
fn prog(s: &str) -> IResult<&str, Prog> {
    // TODO: Get rid of the closure in the next line
    let (s, subprogs) = separated_nonempty_list(|s2| bin_op(";", s2), prog_atom)(s)?;
    Ok((s, Prog::Prog(subprogs)))
}

/// An arithmentic expression is either a negative number or an addition term.
fn aexp(s: &str) -> IResult<&str, AExp> {
    alt((num_neg, add))(s)
}

/// A boolean expression is a less-eq comparison.
fn bexp(s: &str) -> IResult<&str, BExp> {
    lesseq(s)
}

//////////
// Misc //
//////////

/// A variable name is a non-empty alphabetical string.
fn varname(s: &str) -> IResult<&str, VarName> {
    let (s, v) = alpha1(s)?;
    Ok((s, VarName::new(v)))
}

/// A given keyword `k` is parsed. It is checked to stand by itself, i.e. cannot be followed by an alphanumeric character (whitespace or some other special character is okay).
fn keyword<'a>(k: &str, s: &'a str) -> IResult<&'a str, ()> {
    let (s, _) = tag(k)(s)?;
    // If there is a next char, it must be non-alphanumeric
    peek(not(verify(anychar, |c| is_alphanumeric(*c as u8))))(s)?;
    Ok((s, ()))
}

/// A binary operator `op` (e.g. `+`) is parsed. Whitespace before or afterwards is consumed.
fn bin_op<'a>(op: &str, s: &'a str) -> IResult<&'a str, ()> {
    let (s, _) = delimited(multispace0, tag(op), multispace0)(s)?;
    Ok((s, ()))
}

////////////////////////////
// Arithmetic Expressions //
////////////////////////////

/// A negative number
fn num_neg(s: &str) -> IResult<&str, AExp> {
    let (s, _) = tag("-")(s)?;
    let (s, n_str) = digit1(s)?;
    let n: i32 = n_str.parse().unwrap();
    Ok((s, {Num(-n)}))
}

/// An addition term consists of multiple multiplication terms. mul + ... + mul
fn add(s: &str) -> IResult<&str, AExp> {
    // TODO: Get rid of the closure in the next line
    let (s, summands) = separated_nonempty_list(|s2| bin_op("+", s2), mul)(s)?;
    // TODO: Use `fold_first` in the future: https://github.com/rust-lang/rust/issues/68125
    let mut iter = summands.into_iter();
    let hd = iter.next().unwrap();
    let res = iter.fold(hd, |acc: AExp, x: AExp| -> AExp {Add(Box::new(acc), Box::new(x))});
    Ok((s, res))
}

/// A multiplication term consists of multiple arithmetic atomic terms.  aexp_atom * ... * aexp_atom
fn mul(s: &str) -> IResult<&str, AExp> {
    // TODO: Get rid of the closure in the next line
    let (s, factors) = separated_nonempty_list(|s2| bin_op("*", s2), aexp_atom)(s)?;
    // TODO: Use `fold_first` in the future: https://github.com/rust-lang/rust/issues/68125
    let mut iter = factors.into_iter();
    let hd = iter.next().unwrap();
    let res = iter.fold(hd, |acc: AExp, x: AExp| -> AExp {Mul(Box::new(acc), Box::new(x))});
    Ok((s, res))
}

/// An arithmetic atomic term is either a non-negative number, a variable or an parenthesized arithmetic expression.
fn aexp_atom(s: &str) -> IResult<&str, AExp> {
    alt((num_nonneg, var, aexp_parens))(s)
}

/// A non-negative number
fn num_nonneg(s: &str) -> IResult<&str, AExp> {
    let (s, n_str) = digit1(s)?;
    let n: i32 = n_str.parse().unwrap();
    Ok((s, {Num(n)}))
}

/// A variable
fn var(s: &str) -> IResult<&str, AExp> {
    let (s, v) = varname(s)?;
    Ok((s, Var(v)))
}

/// A parenthesized arithmetic expression
fn aexp_parens(s: &str) -> IResult<&str, AExp> {
    delimited(pair(tag("("), multispace0),
              aexp,
              pair(multispace0, tag(")")))(s)
}

/////////////////////////
// Boolean Expressions //
/////////////////////////

/// A less-equal comparison
fn lesseq(s: &str) -> IResult<&str, BExp> {
    let (s, left) = aexp(s)?;
    let (s, _) =  bin_op("<=", s)?;
    let (s, right) = aexp(s)?;
    Ok((s, LessEq(Box::new(left), Box::new(right))))
}

//////////////
// Programs //
//////////////

/// An atomic program is either a skip, an assignment, a conditional or a while loop.
fn prog_atom(s: &str) -> IResult<&str, ProgAtom> {
    alt((skip, assign, cond, wwhile))(s)
}

/// A skip.
fn skip(s: &str) -> IResult<&str, ProgAtom> {
    let (s, _) = keyword("skip", s)?;
    Ok((s, Skip))
}

/// An assignment.
fn assign(s: &str) -> IResult<&str, ProgAtom> {
    let (s, v) = varname(s)?;
    let (s, _) = bin_op(":=", s)?;
    let (s, aexp) = aexp(s)?;
    Ok((s, Assign(v, Box::new(aexp))))
}

/// A conditional.
fn cond(s: &str) -> IResult<&str, ProgAtom> {
    let (s, _) = keyword("if", s)?;
    let (s, _) = multispace1(s)?;
    let (s, bexp) = bexp(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = keyword("then", s)?;
    let (s, _) = multispace1(s)?;
    let (s, prog_true) = prog(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = keyword("else", s)?;
    let (s, _) = multispace1(s)?;
    let (s, prog_false) = prog(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = keyword("end", s)?;

    Ok((s, Cond(Box::new(bexp), Box::new(prog_true), Box::new(prog_false))))
}

/// A while loop.
fn wwhile(s: &str) -> IResult<&str, ProgAtom> {
    let (s, _) = keyword("while", s)?;
    let (s, _) = multispace1(s)?;
    let (s, bexp) = bexp(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = keyword("do", s)?;
    let (s, _) = multispace1(s)?;
    let (s, prog) = prog(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = keyword("end", s)?;

    Ok((s, While(Box::new(bexp), Box::new(prog))))
}