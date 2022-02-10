# Programming Lab: Writing a Flow Analyzer in Rust

## Motivation

The motivation for this programming lab is two-fold:
1. Get comfortable with Rust by doing a software project that is both not too small and not too large (ca. 1000 lines of code)
2. Get a hands-on experience on the *static analysis* of code for which flow analysis provides an easy start

## Functionality

For an overview of the complete functionality of the project that is to be developed in this lab, see also the `README.md`.

## Components

The programming lab provides a scaffold that integrates all the components. The essential parts of the components are then to be implemented by you. What follows is an overview of the components:

- **Main program**: User interaction (`main.rs`)
- **Expressions and AST**: Arithmetic expressions (`aexp.rs`), boolean expressions (`bexp.rs`) and the abstract syntax tree (AST, `ast.rs`)
- **Parser**: Parsing *WHILE* programs, resulting in AST representations (`parser.rs`)
- **CFG**: Definition of control flow graphs (CFG) and transformation from AST to CFG (`cfg.rs`)
- **Interpreter**: This component is not part of the analzyer. Nevertheless, it is desirable to simply execute a WHILE program (`interpreter.rs`)
  
- **Flow Analysis** (`flow_analysis/` directory)
  
  - **Lattice and Transfer Function**: Abstract definitions for flow analysis schemes given via traits (`common.rs`)
  - **Analysis**: The maximal-fixed-point (MFP) algorithm implemented for an abstract flow analysis scheme (`mfp.rs`)
  - **Constant Propagation**: Concrete implementation of constant propagation scheme (`const_prop.rs`)
  - **Available Expressions**: Concrete implementation of available expressions scheme (`avail_exp.rs`)


## Exercise 1: Traversing an Arithmetic Expression

Have a look at the definition of the arithmetic expressions (`AExp` in `aexp.rs`). The `contains_var` method takes a variable name `x` and traverses the arithmetic expression searching for an occurrence of `x`. This method is later used in the *available expressions* analysis to check whether an expression becomes *unavailable*.

**Task:** Implement `contains_var` using pattern-matching and recursion.

## Exercise 2: Interpreter

Have a look at the interpreter (`interpreter.rs`).

**Task:** Implement the `todo!`s.

## Exercise 3: Boolean Operators

Have a look at the definition of the boolean expressions (`BExp` in `bexp.rs`). So far, only one variant is possible, namely `LessEq`. Add conjunction, disjunction and boolean negation. For a complete definition, four components need to be extended:
1. Definition in the AST (`BExp` in `bexp.rs`)
2. Interpreter (`eval_bexp` in `interpreter.rs`)
3. Parser (`bexp` and the BNF grammar given in `parser.rs`)
4. Pretty-Printer (`Display::fmt` in `bexp.rs`)

**Task:** Implement all four extensions. Start with 1 and 2. For 3 and 4, you have to make sure to deal with parentheses in the right way (e.g., conjunction has a higher precedence than disjunction). Take inspiration from how parentheses are handled in arithmetic expressions: You may proceed in an analogous way! (Multiplication behaves like conjunction, addition behaves like disjunction, arithmetic negation behaves like boolean negation)

## Exercise 3: AST to CFG

Have a look at the definition of a CFG (`cfg.rs`), which is a graph together with an initial node. For the representation of the graph, we use the `petgraph` library. A `Graph` is parameterized by some `Node` and `Edge` type, which we carefully define so that it fulfills the requirements of a CFG:

- `Node`: More precisely, we allow the `Node` type to be annotated by arbitrary annotation types `A`, yielding a wrapper type `AnnotNode<A>`. The `Node` type then consists of the different nodes, see also the documentation at its definition.
- `Edge`: Edges in a CFG are either unlabelled (`Plain`) or labelled by "true" (`True`) or "false" (`False`).

The `ast_to_cfg` function performs the transformation. It recursively traverses the AST, thereby constructing the CFG piece-by-piece. If you are interested, take a look at the implementation.

Now have a look now at the `map` method of `Cfg<A>`. It transforms a `Cfg<A>` into a `Cfg<B>`. What is this method supposed to do? It simply goes through the graph, mapping each node annotation of type `A` to a new node annotation of type `B` according to `f`. This `map` method is needed e.g. when taking the raw CFG and then adding annotation fields to each node for some flow analyis (see also the first lines of `mfp` in `flow_analysis/mfp.rs`).

**Task:** Fill in the implementation details of the `map` method of `Cfg<A>`, marked by `todo!`. Use the `map` method of `petgraph.Graph`. Because a new CFG is to be constructed (`map` is "immutable"), you will need to clone nodes (which is possible, since the `Clone` trait is implemented for `Node`).


## Exercise 4: Constant Propagation Lattices

Have a look at the abstract definitions of a semi-lattice (`SemiLat` in `flow_analysis/common.rs`) and a transfer function (`eval_transfer_function` in `flow_analysis/common.rs`). Then take a further look at their concrete implementations for the constant propagation scheme (`flow_analysis/const_prop.rs`).

For the constant propagation scheme, two semi-lattices are defined: `ConstLat` which represents values for a single variable; and `MultiConstLat` which represents values for all variables. The latter is defined in terms of the former.

All in all, the *constant propagation* consists of the following components:
- `ConstLat`, `MultiConstLat`
- `join_bin` operation
- `eval_transfer_function`: The core of the analysis scheme. It specifies how an annotation flows through a node
- `init_start`, `init`: The initial annotations, where the init node has all variables set to `0` except for the `x` variable 
- `fmt`: A pretty-printer for the annotation

**Task:** Fill in the implementation details for `ConstLat` and `MultiConstLat`, marked by `todo!`.

## Exercise 5: MFP Analysis

The core algorithm, namely the MFP algorithm, will be implemented in the `mfp` function (`flow_analysis/mfp.rs`). First, take a look at the type signature of `mfp`: It requires `L` to be a semi-lattice (`SemiLat`) and it requires the definition of the transfer functions (`FlowSemantics`). When given an unannotated CFG (`RawAnnot`), the analysis will then yield a CFG with each node annoted by two elements of `L` (`MfpAnnot<L>`).

**Task:** Fill in the implementation details of the `mfp` function marked by `todo!`.

## Exercise 6: Available Expressions Analysis

Have a look at the implementation of the *available expressions* scheme (`flow_analysis/avail_exp.rs`). As for the *constant propagation* scheme, it contains different components:
- `ExpSetLat`, which is the semi-lattice used by the analysis. It is simply a wrapper of `HashSet`
- `join_bin` operation: Combining here means to intersect the two `HashSet`'s
- `eval_transfer_function`: The core of the analysis scheme. It specifies how an annotation flows through a node
- `init_start`, `init`: The initial annotation which is just the empty set
- `fmt`: A pretty-printer for the annotation

**Task:** Fill in the implementation details of the `eval_transfer_function`.