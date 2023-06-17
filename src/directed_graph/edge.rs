/// Input for making a new edge.
///
/// * `1st` is begin vertex of the edge.
/// * `2nd` is end vertex of the edge.
/// * `3st` is the cost of going through the edge.
///
/// # Example
///
/// ```
/// use rust_basic::directed_graph::RawEdge;
///
/// let edge: RawEdge = (1, 2, 7);
/// let (from, to, cost) = edge;
pub type RawEdge = (u64, u64, u64);

/// A edge in a graph.
#[derive(Debug)]
pub struct Edge {
    pub(super) from: u64,
    pub(super) to: u64,
    pub(super) cost: u64,
}

impl Edge {
    pub(super) fn new(from: u64, to: u64, cost: u64) -> Self {
        return Self { from, to, cost };
    }

    /// The vertex where the edge begin from.
    pub fn from(&self) -> u64 {
        return self.from;
    }

    /// The vertex where the edge end to.
    pub fn to(&self) -> u64 {
        return self.to;
    }

    /// Cost to pass through the edge.
    pub fn cost(&self) -> u64 {
        return self.cost;
    }
}
impl Clone for Edge {
    fn clone(&self) -> Self {
        return Self {
            from: self.from,
            to: self.to,
            cost: self.cost,
        };
    }
}
