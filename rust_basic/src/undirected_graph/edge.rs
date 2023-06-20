use super::Vertex;
use std::cmp::Ordering;

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
/// let (vertex0, vertex1, cost) = edge;
pub type RawEdge = (u64, u64, u64);

/// A edge in a graph.
#[derive(Debug)]
pub struct Edge {
    pub(super) identity: u128,
    pub(super) vertexes: [*mut Vertex; 2],
    pub(super) cost: u64,
}

impl Edge {
    pub(super) fn new(
        vertex0: *mut Vertex,
        vertex1: *mut Vertex,
        cost: u64,
    ) -> Self {
        unsafe {
            let (low, high) = match (*vertex0).identity <= (*vertex1).identity {
                true => (vertex0, vertex1),
                false => (vertex1, vertex0),
            };
            return Self {
                identity: Self::get_identity((*low).identity, (*high).identity),
                vertexes: [low, high],
                cost,
            };
        }
    }

    /// Vertexes of the edge.
    pub fn vertexes(&self) -> [u64; 2] {
        unsafe {
            return [
                (*self.vertexes[0]).identity,
                (*self.vertexes[1]).identity,
            ];
        }
    }

    /// The cost for going through the edge.
    pub fn cost(&self) -> u64 {
        return self.cost;
    }

    fn get_identity(low_vertex: u64, high_vertex: u64) -> u128 {
        assert!(high_vertex >= low_vertex);
        let mut result = 0u128;
        result |= high_vertex as u128;
        result <<= 64;
        result |= low_vertex as u128;
        return result;
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
