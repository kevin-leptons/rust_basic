use std::cmp::Ordering;

use super::Vertex;

/// Input for making a new edge.
///
/// * `1st` and `2nd` are vertexes of the edge.
/// * `3st` is the cost of going through the edge.
///
/// # Example
///
/// ```
/// use rust_basic::undirected_graph::RawEdge;
///
/// let edge: RawEdge = (1, 2, 7);
/// let (vertex1, vertex2, cost) = edge;
pub type RawEdge = (u64, u64, u64);

/// A edge in a graph.
#[derive(Debug)]
pub struct Edge {
    pub(super) identity: u128,
    pub(super) vertex1: *mut Vertex,
    pub(super) vertex2: *mut Vertex,
    pub(super) cost: u64,
}

impl Edge {
    pub(super) fn new(
        vertex1: *mut Vertex,
        vertex2: *mut Vertex,
        cost: u64,
    ) -> Self {
        unsafe {
            let id1 = (*vertex1).identity;
            let id2 = (*vertex2).identity;
            let (low, high) = if id1 <= id2 { (id1, id2) } else { (id2, id1) };
            return Self {
                identity: Self::get_identity(low, high),
                vertex1: vertex1,
                vertex2: vertex2,
                cost: cost,
            };
        }
    }

    /// Vertexes of the edge.
    pub fn vertexes(&self) -> [u64; 2] {
        unsafe {
            return [(*self.vertex1).identity, (*self.vertex2).identity];
        }
    }

    /// The cost for going through the edge.
    pub fn cost(&self) -> u64 {
        return self.cost;
    }

    fn get_identity(low_vertex: u64, high_vertex: u64) -> u128 {
        assert!(high_vertex >= low_vertex);
        let mut id = 0u128;
        id |= (high_vertex as u128);
        id <<= 64;
        id |= (low_vertex as u128);
        return id;
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.cost.cmp(&other.cost);
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Eq for Edge {}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        return self.cmp(other) == Ordering::Equal;
    }
}
