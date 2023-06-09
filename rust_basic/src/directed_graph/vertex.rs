use super::{Edge, EdgeIter};
use crate::{HashMap, HashSet};

/// A vertex in a graph.
#[derive(Debug, PartialEq, Eq)]
pub struct Vertex {
    pub(super) identity: u64,
    pub(super) cost: u64,
    pub(super) edges: HashMap<u64, Edge>,
    pub(super) connections_from: HashSet<u64>,
}

/// Input data for creating a vertex.
///
/// * `1st` is indentity.
/// * `2nd` is cost to pass though the vertex.
///
/// # Example
///
/// ```
/// use rust_basic::directed_graph::RawVertex;
///
/// let v: RawVertex = (5, 100);
/// let (identity, cost) = v;
pub type RawVertex = (u64, u64);

impl Vertex {
    pub(super) fn new(identity: u64, cost: u64) -> Self {
        return Self {
            identity,
            cost,
            edges: HashMap::new(),
            connections_from: HashSet::new(),
        };
    }

    /// The identity of the vertex.
    pub fn identity(&self) -> u64 {
        return self.identity;
    }

    /// The cost to pass through the vertex.
    pub fn cost(&self) -> u64 {
        return self.cost;
    }

    /// For iteration over edges that begin from the vertex.
    pub fn edges(&self) -> EdgeIter {
        return EdgeIter::new(&self.edges);
    }
}
