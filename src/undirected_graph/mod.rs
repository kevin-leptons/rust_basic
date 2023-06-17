//! Undirected Graph - a graph with directed edges.
//!
//! This document - module level document does not contain other descriptions or
//! examples. To learn more about how to use, refer to specific Entry APIs which
//! is labeled `entry` in the following sections.
//!
//! If there is the first time, [Package Document](crate) is a good way to learn
//! more about how to explore APIs and access to [Entry APIs
//! List](crate#structs).

mod edge;
mod iter;
mod vertex;

use crate::hash_map::HashMap;
use crate::vector::Vector;
pub use edge::{Edge, RawEdge};
pub use iter::{EdgeIter, TravelIter, VertexIter};
pub use vertex::Vertex;

use self::iter::TravelMutIter;

/// `entry` A container for the undirected graph.
///
/// # Overview
///
/// ```txt
///          3 <----------- edge's cost
/// [ 0 ]---------[ 1 ]<--- vertex
///   |             |<----- edge
///   |             |
///   4             1
///   |             |
///   |      2      |
/// [ 2 ]---------[ 3 ]
/// ```
///
/// # Example
///
/// ````
/// use rust_basic::{UndirectedGraph, Vector};
///
/// let mut g = UndirectedGraph::new();
/// g.new_vertexes([0, 1, 2, 3]);
/// g.new_edges([
///     (0, 1, 3),
///     (0, 2, 4),
///     (1, 3, 1),
///     (3, 2, 2),
/// ]);
/// let mut visited: Vector<u64> = g.travel(1)
///     .map(|v| v.identity())
///     .collect();
/// visited.sort();
/// assert_eq!(visited, Vector::from([0, 1, 2, 3]));
#[derive(Debug)]
pub struct UndirectedGraph {
    vertexes: HashMap<u64, *mut Vertex>,
    edges: HashMap<u128, *mut Edge>,
}

impl UndirectedGraph {
    /// Create a new empty instance.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn new() -> Self {
        return Self {
            vertexes: HashMap::new(),
            edges: HashMap::new(),
        };
    }

    /// For iteration over vertexes.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn vertexes(&self) -> VertexIter {
        return VertexIter::new(&self.vertexes);
    }

    /// For iteration over edges.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn edges(&self) -> EdgeIter {
        return EdgeIter::new(&self.edges);
    }

    /// Create a new vertex.
    ///
    /// Time complexity: O(1) or O(|V|).
    ///
    /// Space complexity: O(|V|).
    pub fn new_vertex(&mut self, identity: u64) {
        assert!(
            !self.vertexes.has(&identity),
            "expect: a not existed vertex"
        );
        let v = Vertex {
            identity: identity,
            zone: identity,
        };
        let b = Box::new(v);
        self.vertexes.set(identity, Box::leak(b));
    }

    /// Create new vertexes from an array.
    ///
    /// Time complexity: O(|V|).
    ///
    /// Space complexity: O(|V|).
    pub fn new_vertexes<const N: usize>(&mut self, vertexes: [u64; N]) {
        self.new_vertexes_iter(vertexes.into_iter());
    }

    /// Create new vertexes from an iterator.
    ///
    /// Time complexity: O(|V|).
    ///
    /// Space complexity: O(|V|).
    pub fn new_vertexes_iter(&mut self, vertexes: impl Iterator<Item = u64>) {
        for v in vertexes {
            self.new_vertex(v);
        }
    }

    /// Create a new edge.
    ///
    /// Time complexity: O(1) or O(|E|).
    ///
    /// Space complexity: O(|E|).
    pub fn new_edge(&mut self, raw: RawEdge) {
        let (v1_id, v2_id, cost) = raw;
        let v1 = match self.vertexes.get(&v1_id) {
            None => panic!("expect: existed vertexes"),
            Some(v) => v.clone(),
        };
        let v2 = match self.vertexes.get(&v2_id) {
            None => panic!("expect: existed vertexes"),
            Some(v) => v.clone(),
        };
        let e = Edge::new(v1, v2, cost);
        let i = match self.edges.has(&e.identity) {
            false => e.identity,
            true => panic!("expect: a not existed edge"),
        };
        let b = Box::new(e);
        self.edges.set(i, Box::leak(b));
    }

    /// Create new edges from an array.
    ///
    /// Time complexity: O(|E|).
    ///
    /// Space complexity: O(|E|).
    pub fn new_edges<const N: usize>(&mut self, edges: [RawEdge; N]) {
        self.new_edges_iter(edges.into_iter());
    }

    /// Create new edges from an iterator.
    ///
    /// Time complexity: O(|E|).
    ///
    /// Space complexity: O(|E|).
    pub fn new_edges_iter(&mut self, edges: impl Iterator<Item = RawEdge>) {
        for e in edges {
            self.new_edge(e);
        }
    }

    /// For iteration over vertexes which are connected with the vertex `from`.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn travel(&self, from: u64) -> TravelIter {
        let from_vertex = match self.vertexes.get(&from) {
            None => panic!("expect: `from` is existed vertex"),
            Some(v) => v.clone(),
        };
        return TravelIter::new(from_vertex, self);
    }

    /// Find the first minimum spanning forest.
    ///
    /// Algorithm: Kruskal.
    ///
    /// Time complexity: O(|E|).log(|V|)).
    ///
    /// Space complexity: O(|V| + |E|).
    pub fn kruskal(&self) -> UndirectedGraph {
        unsafe {
            let mut g = self.clone_with_no_edges();
            let mut edges = self.clone_edges_and_sort(&g);
            loop {
                if edges.size() == 0 {
                    break;
                }
                let e = edges.pop_front();
                if (*e.vertex1).zone == (*e.vertex2).zone {
                    continue;
                }
                Self::merge_zone(e.vertex1, e.vertex2, &mut g);
                let b = Box::new(e);
                g.edges.set(b.identity, Box::leak(b));
            }
            return g;
        }
    }

    fn travel_mut(&self, from: *mut Vertex) -> TravelMutIter {
        return TravelMutIter::new(from, self);
    }

    fn clone_with_no_edges(&self) -> UndirectedGraph {
        let mut g = UndirectedGraph::new();
        g.new_vertexes_iter(self.vertexes.keys().map(|i| *i));
        return g;
    }

    unsafe fn clone_edges_and_sort(
        &self,
        target_graph: &UndirectedGraph,
    ) -> Vector<Edge> {
        let mut edges = Vector::<Edge>::new();
        for e in self.edges.values() {
            let v1 = target_graph
                .vertexes
                .get(&(*(**e).vertex1).identity)
                .unwrap();
            let v2 = target_graph
                .vertexes
                .get(&(*(**e).vertex2).identity)
                .unwrap();
            let new_edge = Edge::new(*v1, *v2, (**e).cost);
            edges.push_back(new_edge);
        }
        edges.sort();
        return edges;
    }

    unsafe fn merge_zone(
        vertex1: *mut Vertex,
        vertex2: *mut Vertex,
        graph: &mut UndirectedGraph,
    ) {
        let v1_zone = (*vertex1).zone;
        let v2_zone = (*vertex2).zone;
        if v1_zone <= v2_zone {
            Self::set_zone(v1_zone, vertex2, graph);
        } else {
            Self::set_zone(v2_zone, vertex1, graph);
        }
    }

    unsafe fn set_zone(
        zone: u64,
        from: *mut Vertex,
        graph: &mut UndirectedGraph,
    ) {
        for v in graph.travel_mut(from) {
            (*v).zone = zone;
        }
    }
}

impl Drop for UndirectedGraph {
    /// Time complexity: O(|V| + |E|).
    ///
    /// Space complexity: O(|V| + |E|).
    fn drop(&mut self) {
        unsafe {
            for v in self.vertexes.values() {
                drop(Box::from_raw(*v));
            }
            for e in self.edges.values() {
                drop(Box::from_raw(*e));
            }
        }
    }
}
