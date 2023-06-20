use crate::Vector;

/// A path from a vertex to an other one in a graph.
#[derive(Debug)]
pub struct Path {
    pub(super) vertexes: Vector<u64>,

    pub(super) cost: u64,
}

impl Path {
    /// List of vertexes in the path. The path begin at index `0`.
    pub fn vertexes(&self) -> &Vector<u64> {
        return &self.vertexes;
    }

    /// Total cost of edges and vertexes in the path.
    pub fn cost(&self) -> u64 {
        return self.cost;
    }
}
