use crate::HashKey;

/// A vertex in a graph.
#[derive(Debug)]
pub struct Vertex {
    pub(super) identity: u64,
    pub(super) zone: u64,
    pub(super) visited: bool,
}

impl Vertex {
    /// Identity of the vertex.
    pub fn identity(&self) -> u64 {
        return self.identity;
    }
}
impl HashKey for Vertex {
    fn hash_key(&self) -> u32 {
        return self.identity.hash_key();
    }
}

impl Eq for Vertex {}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        return self.identity == other.identity;
    }
}
