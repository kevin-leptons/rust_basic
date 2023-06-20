use super::{Edge, Vertex};
use crate::{hash_map, HashMap, HashSet, Queue, UndirectedGraph};

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
            let current = self.queue.pop();
            for edge in self.graph.edges.values() {
                let [vertex0, vertex1] = (**edge).vertexes;
                if (*vertex0).identity == (*current).identity {
                    if !self.visited.has(&(*vertex1).identity) {
                        self.queue.push(vertex1);
                        self.visited.add((*vertex1).identity);
                    }
                }
                if (*vertex1).identity == (*current).identity {
                    if !self.visited.has(&(*vertex0).identity) {
                        self.queue.push(vertex0);
                        self.visited.add((*vertex0).identity);
                    }
                }
            }
            return Some(current);
        }
    }
}
