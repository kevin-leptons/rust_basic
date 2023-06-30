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

use self::iter::TravelMutIter;
use crate::{HashMap, Vector};
pub use edge::{Edge, RawEdge};
pub use iter::{EdgeIter, TravelIter, VertexIter};
pub use vertex::Vertex;

/// `entry` A container for the undirected graph.
///
/// # Model
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
/// # Panic
///
/// * Call [new_vertex](Self::new_vertex), [new_vertexes](Self::new_vertexes) or
///   [new_vertexes_iter](Self::new_vertexes_iter) to a graph that already has
///   [usize::MAX] vertexes; or with existed vertexes;
/// * Call [new_edge](Self::new_edge), [new_edges](Self::new_edges) or
///   [new_edges_iter](Self::new_edges_iter) to vertxes that already have
///   [usize::MAX] edges; or with existed edges.
///
/// # Example
///
/// ````
/// use rust_basic::{UndirectedGraph, Vector};
///
/// let mut graph = UndirectedGraph::new();
/// graph.new_vertexes([0, 1, 2, 3]);
/// graph.new_edges([
///     (0, 1, 3),
///     (0, 2, 4),
///     (1, 3, 1),
///     (3, 2, 2),
/// ]);
/// let mut visited = graph.travel(1)
///     .map(|v| v.identity())
///     .collect::<Vector<_>>();
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
        let vertex = Vertex {
            identity,
            zone: identity,
        };
        self.vertexes.set(identity, Box::leak(Box::new(vertex)));
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
        for vertex in vertexes {
            self.new_vertex(vertex);
        }
    }

    /// Create a new edge.
    ///
    /// Time complexity: O(1) or O(|E|).
    ///
    /// Space complexity: O(|E|).
    pub fn new_edge(&mut self, raw: RawEdge) {
        let (vertex0_id, vertex1_id, cost) = raw;
        let vertex0 = match self.vertexes.get(&vertex0_id) {
            None => panic!("expect: existed vertexes"),
            Some(v) => v.clone(),
        };
        let vertex1 = match self.vertexes.get(&vertex1_id) {
            None => panic!("expect: existed vertexes"),
            Some(v) => v.clone(),
        };
        let edge = Edge::new(vertex0, vertex1, cost);
        let edge_id = match self.edges.has(&edge.identity) {
            false => edge.identity,
            true => panic!("expect: a not existed edge"),
        };
        self.edges.set(edge_id, Box::leak(Box::new(edge)));
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
        for edge in edges {
            self.new_edge(edge);
        }
    }

    /// For iteration over vertexes which are connected with the vertex `from`.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn travel(&self, from: u64) -> TravelIter {
        let from_vertex = match self.vertexes.get(&from) {
            None => panic!("expect: existed vertex"),
            Some(v) => v.clone(),
        };
        return TravelIter::new(from_vertex, self);
    }

    /// Find the first minimum spanning forest.
    ///
    /// Algorithm: Kruskal.
    ///
    /// Time complexity: O(|E|).log(|V|).
    ///
    /// Space complexity: O(|V| + |E|).
    pub fn kruskal(&self) -> UndirectedGraph {
        unsafe {
            let mut result = self.clone_without_edges();
            let mut edges = self.clone_edges_for(&result);
            edges.sort();
            loop {
                if edges.size() == 0 {
                    break;
                }
                let edge = edges.pop_front();
                if (*edge.vertexes[0]).zone == (*edge.vertexes[1]).zone {
                    continue;
                }
                Self::merge_zone(
                    edge.vertexes[0],
                    edge.vertexes[1],
                    &mut result,
                );
                let edge_id = edge.identity;
                result.edges.set(edge_id, Box::leak(Box::new(edge)));
            }
            return result;
        }
    }

    fn travel_mut(&self, from: *mut Vertex) -> TravelMutIter {
        return TravelMutIter::new(from, self);
    }

    fn clone_without_edges(&self) -> UndirectedGraph {
        let mut result = UndirectedGraph::new();
        result.new_vertexes_iter(self.vertexes.keys().map(|i| *i));
        return result;
    }

    unsafe fn clone_edges_for(&self, target: &UndirectedGraph) -> Vector<Edge> {
        return self
            .edges
            .values()
            .map(|edge| {
                let [vertex0, vertex1] = (**edge)
                    .vertexes()
                    .map(|id| target.vertexes.get(&id).unwrap());
                return Edge::new(*vertex0, *vertex1, (**edge).cost);
            })
            .collect();
    }

    unsafe fn merge_zone(
        vertex0: *mut Vertex,
        vertex1: *mut Vertex,
        graph: &mut UndirectedGraph,
    ) {
        if (*vertex0).zone <= (*vertex1).zone {
            Self::set_zone((*vertex0).zone, vertex1, graph);
        } else {
            Self::set_zone((*vertex1).zone, vertex0, graph);
        }
    }

    unsafe fn set_zone(
        zone: u64,
        from: *mut Vertex,
        graph: &mut UndirectedGraph,
    ) {
        for vertex in graph.travel_mut(from) {
            (*vertex).zone = zone;
        }
    }
}

impl Drop for UndirectedGraph {
    /// Time complexity: O(|V| + |E|).
    ///
    /// Space complexity: O(|V| + |E|).
    fn drop(&mut self) {
        unsafe {
            for edge in self.edges.values() {
                drop(Box::from_raw(*edge));
            }
            for vertex in self.vertexes.values() {
                drop(Box::from_raw(*vertex));
            }
        }
    }
}
