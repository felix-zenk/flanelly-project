extern crate nom;

use flanelly::{cfg::Cfg, parser, cfg};
use petgraph::dot::Dot;
use flanelly::flow_analysis::{mfp:: mfp, mfp::MfpAnnot, const_prop::MultiConstLat, avail_exp::ExpSetLat};
use flanelly::interpreter::eval;
use std::io::{self, Read};
use clap::{Arg, App};

fn main() -> io::Result<()> {
  // Read command line arguments
  let arguments = App::new("Flow Analyzer")
      .about("Perform MFP analysis on WHILE programs for constant propagation and available expressions.")
      .arg(Arg::with_name("const_prop")
           .short("c")
           .help("constant propagation"))
      .arg(Arg::with_name("avail_exp")
           .short("a")
           .help("available expressions"))
      .arg(Arg::with_name("interpret")
           .short("i")
           .help("interpret")
           .takes_value(true)
           .allow_hyphen_values(true))
      .get_matches();

  // Read program from StdIn and parse AST
  let mut program_buffer = String::new();
  io::stdin().read_to_string(&mut program_buffer)?;
  let p = parser::parse(&program_buffer).unwrap();

  // Which action to do?
  if arguments.is_present("interpret") {
    let x = arguments.value_of("interpret").unwrap_or("0").parse::<i32>().unwrap_or(0);
    // May terminate or diverge
    let z = eval(&p, x);
    println!("{}", z)
  }
  else {
    let do_const_prop = arguments.is_present("const_prop") || !arguments.is_present("avail_exp");

    let cfg = cfg::ast_to_cfg(&p);
  
    // Analyze and output to StdOut
    if do_const_prop {
      // Rust Expl.: By giving the following type annotation, the compiler knows which type (and therefore which implementation) to fill in for the generic type variables `L` in the `mfp` function (namely, the `MultiConstLat` one).
      let cfg_mfp: Cfg<MfpAnnot<MultiConstLat>> = mfp(&cfg);
      println!("{}", Dot::new(&cfg_mfp.graph));
    }
    else {
      // Rust Expl.: By giving the following type annotation, the compiler knows which type (and therefore which implementation) to fill in for the generic type variables `L` in the `mfp` function (namely, the `ExpSetLat` one).
      let cfg_mfp: Cfg<MfpAnnot<ExpSetLat>> = mfp(&cfg);
      println!("{}", Dot::new(&cfg_mfp.graph));
    }
  }

  Ok(())
}