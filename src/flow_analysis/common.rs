use vec1::Vec1;

use crate::cfg::Node;

/// # Semi-Lattice
/// - We use semi-lattices here (require only the `join` operation)
/// - No `meet` needed for MFP
/// - `bot` is needed but is defined in `FlowSemantics` trait as `init()` as it is specifically needed for computing the MFP
pub trait SemiLat: Sized + Eq + Clone {
    fn join_bin(self: &Self, other: &Self) -> Self;

    /// Joining is possible for a non-empty set of elements
    fn join(vs: Vec1<&Self>) -> Self {
        let hd = *vs.first();
        let tl = vs.to_vec();
        tl.iter().fold(hd.clone(), |acc, x| Self::join_bin(&acc, x))
    }
}

/// Flow semantics represent a way of computing "through a CFG". This trait is typically implemented by some lattice (called the "property space") which represents the values that "flow" through the CFG. For every node then, one can take an incoming value and produce an outgoing value. This is also known as "evaluating the node's transfer function".
pub trait FlowSemantics {
    /// Evaluate a node's transfer function
    fn eval_transfer_function(n: &Node, x: &Self) -> Self;
    /// The element that is used as initialization of all annotations (except for the very first one, the init node - see `init_start()` for this)
    /// This element is either the "top" or the "bottom" element of the used semi-lattice.
    fn init() -> Self;
    /// The `init_start` value is an initial static value (an element of the property space) that is attached to the first node. See the concrete implementations of `FlowSemantics` for details.
    fn init_start() -> Self;
}