use crate::{hash_map, HashMap, HashSet, Queue, UndirectedGraph};

use super::{Edge, Vertex};

/// For iteration over vertexes in [super::UndirectedGraph].
pub struct VertexIter<'a> {
    iter: hash_map::ValueIter<'a, u64, *mut Vertex>,
}

impl<'a> VertexIter<'a> {
    pub(super) fn new(vertexes: &'a HashMap<u64, *mut Vertex>) -> Self {
        return VertexIter {
            iter: vertexes.values(),
        };
    }
}

impl<'a> Iterator for VertexIter<'a> {
    type Item = &'a Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            match self.iter.next() {
                None => None,
                Some(v) => Some(&**v),
            }
        }
    }
}

/// For iteration over edges in [super::UndirectedGraph].
pub struct EdgeIter<'a> {
    iter: hash_map::ValueIter<'a, u128, *mut Edge>,
}

impl<'a> EdgeIter<'a> {
    pub(super) fn new(edges: &'a HashMap<u128, *mut Edge>) -> Self {
        return Self {
            iter: edges.values(),
        };
    }
}

impl<'a> Iterator for EdgeIter<'a> {
    type Item = &'a Edge;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            match self.iter.next() {
                None => return None,
                Some(v) => return Some(&**v),
            }
        }
    }
}

/// For travel over immutable vertexes in the graph.
pub struct TravelIter<'a> {
    iter: TravelMutIter<'a>,
}

impl<'a> TravelIter<'a> {
    pub(super) fn new(from: *mut Vertex, graph: &'a UndirectedGraph) -> Self {
        let iter = TravelMutIter::new(from, graph);
        return Self { iter };
    }
}

impl<'a> Iterator for TravelIter<'a> {
    type Item = &'a Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            match self.iter.next() {
                None => return None,
                Some(v) => {
                    return Some(&*v);
                }
            };
        }
    }
}

/// For travel over mutable vertexes in the graph.
pub(super) struct TravelMutIter<'a> {
    graph: &'a UndirectedGraph,
    queue: Queue<*mut Vertex>,
    visited: HashSet<u64>,
}

impl<'a> TravelMutIter<'a> {
    pub(super) fn new(from: *mut Vertex, graph: &'a UndirectedGraph) -> Self {
        unsafe {
            return Self {
                queue: Queue::from([from]),
                visited: HashSet::from([(*from).identity]),
                graph: graph,
            };
        }
    }
}

impl<'a> Iterator for TravelMutIter<'a> {
    type Item = *mut Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.queue.size() == 0 {
                return None;
            }
            let v = self.queue.pop();
            for e in self.graph.edges.values() {
                if (*(**e).vertex1).identity == (*v).identity {
                    if !self.visited.has(&(*(**e).vertex2).identity) {
                        self.queue.push((**e).vertex2);
                        self.visited.add((*(**e).vertex2).identity);
                    }
                }
                if (*(**e).vertex2).identity == (*v).identity {
                    if !self.visited.has(&(*(**e).vertex1).identity) {
                        self.queue.push((**e).vertex1);
                        self.visited.add((*(**e).vertex1).identity);
                    }
                }
            }
            return Some(v);
        }
    }
}
