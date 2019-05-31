//! Representation of graphs of compile-time constant values.

use std::cmp::Ordering;
use enumset::{EnumSetType, EnumSet};
use enum_map::{Enum, EnumMap};

/// Represents a directed graph with unlabeled edges (self loops allowed), where the nodes are
/// compile-time constants defined by an [`EnumSetType`](enumset::EnumSetType) enum.
///
/// # Parameters
/// - `T`: Type of node to use for the graph. The graph always contains all nodes in the enum,
///   though some nodes might not be connected to any other nodes.
#[derive(Debug, PartialEq, Eq, Hash, Default)]
pub struct EnumSetDiGraph<T: EnumSetType + Enum<EnumSet<T>>> {
    /// An adjacency set for each node in the enum.
    adj_sets: EnumMap<T, EnumSet<T>>
}

impl<T: EnumSetType + Enum<EnumSet<T>>> EnumSetDiGraph<T> {
    /// Constructs a new directed graph with no edges.
    pub fn new() -> Self {
        EnumSetDiGraph {
            adj_sets: EnumMap::from(|_| EnumSet::empty())
        }
    }

    /// Checks if the specified directed edge is present in the graph.
    ///
    /// # Parameters
    /// - `from`: Start node for the edge.
    /// - `to`: End node for the edge.
    pub fn has_edge(&self, from: T, to: T) -> bool {
        self.adj_sets[from].contains(to)
    }

    /// Checks if the graph contains a directed path from the specified start node to the specified
    /// end node. Zero-length paths are included, so this will always return true if `from` and `to`
    /// are the same node.
    ///
    /// # Parameters
    /// - `from`: Start node for the path.
    /// - `to`: End node for the path.
    pub fn has_path(&self, from: T, to: T) -> bool {
        self.reachable_nodes(from).contains(to)
    }

    /// Finds the set of nodes directly reachable by a one-edge directed path starting at the
    /// specified node. Zero-length paths are not included, so the returned set will not contain the
    /// start node unless the start node has a self loop.
    pub fn adjacent_nodes(&self, from: T) -> EnumSet<T> {
        self.adj_sets[from]
    }

    /// Finds the set of nodes reachable by any directed path starting at the specified node.
    /// Zero-length paths are included, so the returned set will always contain the start node.
    ///
    /// # Parameters
    /// - `from`: Node to start from.
    pub fn reachable_nodes(&self, from: T) -> EnumSet<T> {
        // Perform a depth-first search.
        let mut visited = EnumSet::empty();
        let mut stack = vec![from];
        while !stack.is_empty() {
            let neighbor = stack.pop().unwrap();
            if !visited.contains(neighbor) {
                visited.insert(neighbor);
                for next in self.adj_sets[neighbor] {
                    stack.push(next)
                }
            }
        }

        visited
    }

    /// Adds the specified directed edge to the graph if it is not present.
    ///
    /// # Parameters
    /// - `from`: Start node for the edge.
    /// - `to`: End node for the edge.
    pub fn insert_edge(&mut self, from: T, to: T) {
        self.adj_sets[from].insert(to);
    }

    /// Constructs the transitive closure of this graph. The resulting graph has an edge from `x` to
    /// `y` for every pair of nodes `x` and `y` such that
    /// [`self.has_path(x, y)`](self::EnumSetDiGraph::has_path). Note that the resulting graph will
    /// have a self loop on every node.
    pub fn transitive_closure(&self) -> Self {
        let mut output = EnumSetDiGraph::new();
        for from in EnumSet::all() {
            output.adj_sets[from] = self.reachable_nodes(from);
        }

        output
    }

    /// Reverses the directions of all edges in the graph.
    pub fn reverse(&self) -> Self {
        let mut output = EnumSetDiGraph::new();
        for from in EnumSet::all() {
            for to in self.adj_sets[from] {
                output.insert_edge(to, from);
            }
        }

        output
    }
}

impl<T: EnumSetType + Enum<EnumSet<T>>> Clone for EnumSetDiGraph<T> where
    EnumMap<T, EnumSet<T>>: Clone
{
    fn clone(&self) -> Self {
        EnumSetDiGraph {
            adj_sets: self.adj_sets.clone()
        }
    }
}

impl<T: EnumSetType + Enum<EnumSet<T>>> Copy for EnumSetDiGraph<T> where
    EnumMap<T, EnumSet<T>>: Copy
{}

impl<T: EnumSetType + Enum<EnumSet<T>>> PartialOrd for EnumSetDiGraph<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: EnumSetType + Enum<EnumSet<T>>> Ord for EnumSetDiGraph<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Here we assume that two EnumMaps of the same type will always return their values in the
        // same key order.
        // TODO: Maybe don't make this assumption.
        self.adj_sets.values().cmp(other.adj_sets.values())
    }
}