use crate::{hash_map::ValueIter, DirectedGraph, HashMap, HashSet, Queue};

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

/// For iteration over vertexes.
pub struct TravelIter<'a> {
    queue: Queue<&'a Vertex>,
    graph: &'a DirectedGraph,
    visited: HashSet<u64>,
}

impl<'a> TravelIter<'a> {
    pub(super) fn new(from: u64, graph: &'a DirectedGraph) -> Self {
        let v = match graph.vertexes.get(&from) {
            None => panic!("expect: an existing vertex"),
            Some(v) => v,
        };
        return Self {
            queue: Queue::from([v]),
            visited: HashSet::new(),
            graph: graph,
        };
    }
}

impl<'a> Iterator for TravelIter<'a> {
    type Item = &'a Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.size() == 0 {
            return None;
        }
        let v = self.queue.pop();
        for i in v.edges.keys() {
            let c = self.graph.vertexes.get(i).unwrap();
            if self.visited.has(&c.identity) {
                continue;
            }
            self.queue.push(c);
            self.visited.add(c.identity);
        }
        return Some(v);
    }
}
