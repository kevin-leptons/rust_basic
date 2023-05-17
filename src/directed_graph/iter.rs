use crate::{hash_map::ValueIter, HashMap};

use super::{edge::Edge, Vertex};

/// For iteration over vertexes.
pub struct VertexIter<'a> {
    iter: ValueIter<'a, u64, Vertex>,
}

impl<'a> VertexIter<'a> {
    pub(super) fn new(map: &'a HashMap<u64, Vertex>) -> Self {
        return Self { iter: map.values() };
    }
}

impl<'a> Iterator for VertexIter<'a> {
    type Item = &'a Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        return self.iter.next();
    }
}

/// For iteration over edges.
pub struct EdgeIter<'a> {
    iter: ValueIter<'a, u64, Edge>,
}

impl<'a> EdgeIter<'a> {
    pub(super) fn new(map: &'a HashMap<u64, Edge>) -> Self {
        return Self { iter: map.values() };
    }
}
impl<'a> Iterator for EdgeIter<'a> {
    type Item = &'a Edge;

    fn next(&mut self) -> Option<Self::Item> {
        return self.iter.next();
    }
}
