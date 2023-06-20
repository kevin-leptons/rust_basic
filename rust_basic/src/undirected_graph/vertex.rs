use crate::Hashable;

/// A vertex in a graph.
#[derive(Debug)]
pub struct Vertex {
    pub(super) identity: u64,
    pub(super) zone: u64,
}

impl Vertex {
    /// Identity of the vertex.
    pub fn identity(&self) -> u64 {
        return self.identity;
    }
}
impl Hashable for Vertex {
    fn hash(&self) -> u64 {
        return self.identity.hash();
    }
}

impl Eq for Vertex {}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        return self.identity == other.identity;
    }
}
