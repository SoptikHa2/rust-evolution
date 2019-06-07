
/// A Node is something that describes
/// behaviour of an animal. It is
/// essentially one part of animal's brain.
/// It can be "go north", or "eat".
/// 
/// Nodes are connected togehter a tree structure.
/// Each node has one output and zero or more inputs.
/// Functionality of each node can depend on input.
/// For example input could select if the animal
/// should move to north, east, west, or south.
pub trait Node {
    /// Evaluate node. This might
    /// affect host animal, for example
    /// it might move depending on node type.
    /// Depending on node implementation,
    /// invoking this will likely make the node
    /// call this on all children.
    fn evaluate() -> isize;
}