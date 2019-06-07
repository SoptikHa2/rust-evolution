use crate::node::Node;
use crate::map::*;

/// One animal, with it's own brain.
/// This animal evolves on it's own
/// and will be used to breed new animals.
pub struct Animal<N: Node> {
    /// Animal energy, represents score.
    /// This is used to evaluate effectivity of
    /// animal brain.
    pub score: isize,

    /// This is animal brain, or the root node.
    ///
    /// Type `N` is any structure which implements
    /// the `Node` trait.
    root: N,

    /// Current animal coordinates.
    x: usize,
    /// Current animal coordinates
    y: usize,
}

impl<N: Node> Animal<N> {
    /// Try to eat whatever's on current tile.
    ///
    /// *Run into the ~~bullets~~ friendliness pellets!*
    pub fn eat() {}

    /// Move in some direction.
    /// 
    /// Direction `Direction::Here` is ignored,
    /// but move will still cost energy.
    pub fn go(direction: Direction) {}

    /// Find direction to closest food pellet.
    /// 
    /// If none is found, return None.
    pub fn scan_food() -> Option<Direction> {}
}
