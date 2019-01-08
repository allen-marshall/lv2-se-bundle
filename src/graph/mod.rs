//! Utilities for dealing with graphs.

use std::iter::{FromIterator, once};
use std::hash::Hash;
use std::collections::HashSet;

/// Trait for types that represent nodes in a directed graph, where each node can compute its
/// parents. Parents are generated by value and not by reference, so this trait may be inappropriate
/// for types that do not implement [`Copy`](std::marker::Copy). It is mainly intended for use with
/// graphs that are known statically at compile time.
pub(crate) trait ParentFindingDigraphNode: Hash + Eq + Clone + Sized {
    /// Gets the set of parents of this node. Root nodes should return an empty vector.
    fn parents(&self) -> Vec<Self>;

    /// Gets the set containing this node and all its ancestors.
    fn ancestors_and_self(&self) -> HashSet<Self> {
        let mut output = HashSet::from_iter(once(self.clone()));
        let mut visited = HashSet::new();
        let mut next_parents = self.parents();
        while !next_parents.is_empty() {
            let mut next_next_parents = Vec::new();
            for parent in next_parents.iter().map(Clone::clone) {
                if !visited.contains(&parent) {
                    next_next_parents.extend(parent.parents().into_iter());
                    visited.insert(parent);
                }
            }
            output.extend(next_parents.into_iter());
            next_parents = next_next_parents;
        }
        output
    }

    /// Gets the set containing the provided nodes and all their ancestors.
    ///
    /// # Parameters
    /// - I: Type of iterator to use for getting the child nodes.
    /// - children: Iterator of nodes whose ancestors (and selves) are to be returned. If this
    ///   iterator generates no nodes, the returned set will be empty.
    fn ancestors_and_self_multiple<I>(children: I) -> HashSet<Self> where
        I: Iterator<Item = Self>
    {
        let mut output = HashSet::new();
        for child in children {
            output.extend(child.ancestors_and_self());
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fmt::Debug;

    /// Asserts that a [`HashSet`](std::collections::HashSet) contains the same set of elements as a
    /// [`Vec`](std::vec::Vec), ignoring any duplicate elements in the vector.
    fn assert_set_eq_vec<T>(set: HashSet<T>, vector: Vec<T>) where
        T: Hash + Eq + Debug
    {
        assert_eq!(set, HashSet::from_iter(vector.into_iter()));
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum TestTreeNode {
        Root,
        BranchLeft,
        BranchRight,
        LeafLeft,
        LeafRight
    }

    impl ParentFindingDigraphNode for TestTreeNode {
        fn parents(&self) -> Vec<Self> {
            match self {
                TestTreeNode::BranchLeft => vec![TestTreeNode::Root],
                TestTreeNode::BranchRight => vec![TestTreeNode::Root],
                TestTreeNode::LeafLeft => vec![TestTreeNode::BranchLeft],
                TestTreeNode::LeafRight => vec![TestTreeNode::BranchRight],
                _ => vec![]
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum TestForestNode {
        Root0,
        Root1,
        Root2,
        Leaf0,
        Leaf1,
        Branch2,
        Leaf2
    }

    impl ParentFindingDigraphNode for TestForestNode {
        fn parents(&self) -> Vec<Self> {
            match self {
                TestForestNode::Leaf0 => vec![TestForestNode::Root0],
                TestForestNode::Leaf1 => vec![TestForestNode::Root1],
                TestForestNode::Branch2 => vec![TestForestNode::Root2],
                TestForestNode::Leaf2 => vec![TestForestNode::Branch2],
                _ => vec![]
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum TestDagNode {
        Root,
        Branch0,
        Branch1,
        Leaf0,
        LeafBoth
    }

    impl ParentFindingDigraphNode for TestDagNode {
        fn parents(&self) -> Vec<Self> {
            match self {
                TestDagNode::Branch0 => vec![TestDagNode::Root],
                TestDagNode::Branch1 => vec![TestDagNode::Root],
                TestDagNode::Leaf0 => vec![TestDagNode::Branch0],
                TestDagNode::LeafBoth => vec![TestDagNode::Branch0, TestDagNode::Branch1],
                _ => vec![]
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum TestCycleNode {
        Cycle0,
        Cycle1,
        SelfLoop
    }

    impl ParentFindingDigraphNode for TestCycleNode {
        fn parents(&self) -> Vec<Self> {
            match self {
                TestCycleNode::Cycle0 => vec![TestCycleNode::Cycle1],
                TestCycleNode::Cycle1 => vec![TestCycleNode::Cycle0],
                TestCycleNode::SelfLoop => vec![TestCycleNode::SelfLoop]
            }
        }
    }

    #[test]
    fn tree() {
        assert_set_eq_vec(TestTreeNode::Root.ancestors_and_self(), vec![TestTreeNode::Root]);
        assert_set_eq_vec(TestTreeNode::BranchLeft.ancestors_and_self(), vec![TestTreeNode::Root, TestTreeNode::BranchLeft]);
        assert_set_eq_vec(TestTreeNode::BranchRight.ancestors_and_self(), vec![TestTreeNode::Root, TestTreeNode::BranchRight]);
        assert_set_eq_vec(TestTreeNode::LeafLeft.ancestors_and_self(), vec![TestTreeNode::Root, TestTreeNode::BranchLeft, TestTreeNode::LeafLeft]);
        assert_set_eq_vec(TestTreeNode::LeafRight.ancestors_and_self(), vec![TestTreeNode::Root, TestTreeNode::BranchRight, TestTreeNode::LeafRight]);

        assert_set_eq_vec(TestTreeNode::ancestors_and_self_multiple(vec![].into_iter()), vec![]);
        assert_set_eq_vec(TestTreeNode::ancestors_and_self_multiple(vec![TestTreeNode::BranchLeft, TestTreeNode::LeafRight].into_iter()), vec![TestTreeNode::Root, TestTreeNode::BranchLeft, TestTreeNode::BranchRight, TestTreeNode::LeafRight]);
        assert_set_eq_vec(TestTreeNode::ancestors_and_self_multiple(vec![TestTreeNode::Root, TestTreeNode::Root, TestTreeNode::BranchRight].into_iter()), vec![TestTreeNode::Root, TestTreeNode::BranchRight]);
        assert_set_eq_vec(TestTreeNode::ancestors_and_self_multiple(vec![TestTreeNode::Root].into_iter()), vec![TestTreeNode::Root]);
        assert_set_eq_vec(TestTreeNode::ancestors_and_self_multiple(vec![TestTreeNode::BranchLeft].into_iter()), vec![TestTreeNode::Root, TestTreeNode::BranchLeft]);
    }

    #[test]
    fn forest() {
        assert_set_eq_vec(TestForestNode::Root0.ancestors_and_self(), vec![TestForestNode::Root0]);
        assert_set_eq_vec(TestForestNode::Root1.ancestors_and_self(), vec![TestForestNode::Root1]);
        assert_set_eq_vec(TestForestNode::Root2.ancestors_and_self(), vec![TestForestNode::Root2]);
        assert_set_eq_vec(TestForestNode::Leaf0.ancestors_and_self(), vec![TestForestNode::Root0, TestForestNode::Leaf0]);
        assert_set_eq_vec(TestForestNode::Leaf1.ancestors_and_self(), vec![TestForestNode::Root1, TestForestNode::Leaf1]);
        assert_set_eq_vec(TestForestNode::Branch2.ancestors_and_self(), vec![TestForestNode::Root2, TestForestNode::Branch2]);
        assert_set_eq_vec(TestForestNode::Leaf2.ancestors_and_self(), vec![TestForestNode::Root2, TestForestNode::Branch2, TestForestNode::Leaf2]);

        assert_set_eq_vec(TestForestNode::ancestors_and_self_multiple(vec![].into_iter()), vec![]);
        assert_set_eq_vec(TestForestNode::ancestors_and_self_multiple(vec![TestForestNode::Root0].into_iter()), vec![TestForestNode::Root0]);
        assert_set_eq_vec(TestForestNode::ancestors_and_self_multiple(vec![TestForestNode::Root0, TestForestNode::Root0].into_iter()), vec![TestForestNode::Root0]);
        assert_set_eq_vec(TestForestNode::ancestors_and_self_multiple(vec![TestForestNode::Root0, TestForestNode::Root1].into_iter()), vec![TestForestNode::Root0, TestForestNode::Root1]);
        assert_set_eq_vec(TestForestNode::ancestors_and_self_multiple(vec![TestForestNode::Leaf0, TestForestNode::Branch2].into_iter()), vec![TestForestNode::Root0, TestForestNode::Leaf0, TestForestNode::Root2, TestForestNode::Branch2]);
    }

    #[test]
    fn dag() {
        assert_set_eq_vec(TestDagNode::Root.ancestors_and_self(), vec![TestDagNode::Root]);
        assert_set_eq_vec(TestDagNode::Branch0.ancestors_and_self(), vec![TestDagNode::Root, TestDagNode::Branch0]);
        assert_set_eq_vec(TestDagNode::Branch1.ancestors_and_self(), vec![TestDagNode::Root, TestDagNode::Branch1]);
        assert_set_eq_vec(TestDagNode::Leaf0.ancestors_and_self(), vec![TestDagNode::Root, TestDagNode::Branch0, TestDagNode::Leaf0]);
        assert_set_eq_vec(TestDagNode::LeafBoth.ancestors_and_self(), vec![TestDagNode::Root, TestDagNode::Branch0, TestDagNode::Branch1, TestDagNode::LeafBoth]);

        assert_set_eq_vec(TestDagNode::ancestors_and_self_multiple(vec![].into_iter()), vec![]);
        assert_set_eq_vec(TestDagNode::ancestors_and_self_multiple(vec![TestDagNode::Root].into_iter()), vec![TestDagNode::Root]);
        assert_set_eq_vec(TestDagNode::ancestors_and_self_multiple(vec![TestDagNode::Root, TestDagNode::Root].into_iter()), vec![TestDagNode::Root]);
        assert_set_eq_vec(TestDagNode::ancestors_and_self_multiple(vec![TestDagNode::Branch1, TestDagNode::Leaf0].into_iter()), vec![TestDagNode::Root, TestDagNode::Branch0, TestDagNode::Branch1, TestDagNode::Leaf0]);
        assert_set_eq_vec(TestDagNode::ancestors_and_self_multiple(vec![TestDagNode::LeafBoth, TestDagNode::Root].into_iter()), vec![TestDagNode::Root, TestDagNode::Branch0, TestDagNode::Branch1, TestDagNode::LeafBoth]);
    }

    #[test]
    fn cycle() {
        assert_set_eq_vec(TestCycleNode::Cycle0.ancestors_and_self(), vec![TestCycleNode::Cycle0, TestCycleNode::Cycle1]);
        assert_set_eq_vec(TestCycleNode::Cycle1.ancestors_and_self(), vec![TestCycleNode::Cycle0, TestCycleNode::Cycle1]);
        assert_set_eq_vec(TestCycleNode::SelfLoop.ancestors_and_self(), vec![TestCycleNode::SelfLoop]);

        assert_set_eq_vec(TestCycleNode::ancestors_and_self_multiple(vec![].into_iter()), vec![]);
        assert_set_eq_vec(TestCycleNode::ancestors_and_self_multiple(vec![TestCycleNode::Cycle0].into_iter()), vec![TestCycleNode::Cycle0, TestCycleNode::Cycle1]);
        assert_set_eq_vec(TestCycleNode::ancestors_and_self_multiple(vec![TestCycleNode::Cycle0, TestCycleNode::Cycle1].into_iter()), vec![TestCycleNode::Cycle0, TestCycleNode::Cycle1]);
        assert_set_eq_vec(TestCycleNode::ancestors_and_self_multiple(vec![TestCycleNode::SelfLoop].into_iter()), vec![TestCycleNode::SelfLoop]);
        assert_set_eq_vec(TestCycleNode::ancestors_and_self_multiple(vec![TestCycleNode::Cycle0, TestCycleNode::SelfLoop].into_iter()), vec![TestCycleNode::Cycle0, TestCycleNode::Cycle1, TestCycleNode::SelfLoop]);
    }
}