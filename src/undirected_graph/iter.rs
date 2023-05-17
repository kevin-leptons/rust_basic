use crate::{hash_map, HashMap, Queue, UndirectedGraph};

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
}

impl<'a> TravelMutIter<'a> {
    pub(super) fn new(from: *mut Vertex, graph: &'a UndirectedGraph) -> Self {
        unsafe {
            for v in graph.vertexes.values() {
                (**v).visited = false;
            }
            return Self {
                queue: Queue::from([from]),
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
            (*v).visited = true;
            for e in self.graph.edges.values() {
                if (*(**e).vertex1).identity == (*v).identity {
                    if (*(**e).vertex2).visited == false {
                        self.queue.push((**e).vertex2);
                    }
                }
                if (*(**e).vertex2).identity == (*v).identity {
                    if (*(**e).vertex1).visited == false {
                        self.queue.push((**e).vertex1);
                    }
                }
            }
            return Some(v);
        }
    }
}
