//! Directed Graph - a data structure and related algorithms.
//!
//! This document - module level document does not contain other descriptions or
//! examples. To learn more about how to use, refer to specific Entry APIs which
//! is labeled `entry` in the following sections.
//!
//! If there is the first time, [Package Document](crate) is a good way to learn
//! more about how to explore APIs and access to [Entry APIs
//! List](crate#structs).

mod cost;
mod edge;
mod iter;
mod path;
mod vertex;

use crate::hash_map::HashMap;
use crate::priority_queue::PriorityQueue;
use crate::vector::Vector;
use crate::HashSet;
use crate::Queue;
use cost::Cost;
pub use edge::{Edge, RawEdge};
pub use iter::{EdgeIter, TravelIter, VertexIter};
pub use path::Path;
pub use vertex::RawVertex;
pub use vertex::Vertex;

/// `entry` A container for a directed graph.
///
/// A directed graph includes `V` - set of vertexes and `E` - set of edges.
/// `|V|` is quantity of vertexes and `|E|` is quantity of edges.
///
/// # Overview
///
/// ```txt
///
///                           +--------- vertex's identity
///                           |  +------ vertex's cost
///                           |  |
///         1           1     v  v
/// [0, 4]----->[1, 1]------>[2, 5]<--- vertex
///   |          |            |
///   |          |            |<-------- edge
///   4          1            |
///   |          |            |
///   |          |            9 <------- edge's cost
///   v          |            |
/// [3, 2]<-------+           |
///   |                       |
///   |               2       v
///   +--------------------->[4, 7]
/// ```
///
/// # Example
///
/// ```
/// use rust_basic::{DirectedGraph, Vector};
///
/// let mut g = DirectedGraph::new();
/// g.new_vertexes([
///     (0, 4),
///     (1, 1),
///     (2, 5),
///     (3, 2),
///     (4, 7),
/// ]);
/// g.new_edges([
///     (0, 1, 1),
///     (0, 3, 4),
///     (1, 2, 1),
///     (1, 3, 1),
///     (3, 4, 2),
///     (2, 4, 9),
/// ]);
/// let r = g.dijkstra(0, 4).unwrap();
/// assert_eq!(r.vertexes(), &Vector::from([0, 1, 3, 4]));
/// assert_eq!(r.cost(), 18);
#[derive(Debug)]
pub struct DirectedGraph {
    vertexes: HashMap<u64, Vertex>,
}

impl DirectedGraph {
    /// Create a new empty instance.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn new() -> Self {
        return Self {
            vertexes: HashMap::new(),
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

    /// Create a new vertex.
    ///
    /// Time complexity: O(1) or O(|V|).
    ///
    /// Space complexity: O(|V|).
    pub fn new_vertex(&mut self, identity: u64, cost: u64) {
        assert!(!self.vertexes.has(&identity), "expect: not existed vertex");
        let v = Vertex::new(identity, cost);
        self.vertexes.set(identity, v);
    }

    /// Create new vertexes from an array.
    ///
    /// Time complexity: O(1) or O(|V|).
    ///
    /// Space complexity: O(|V|).
    pub fn new_vertexes<const N: usize>(&mut self, vertexes: [RawVertex; N]) {
        self.new_vertexes_iter(vertexes.into_iter());
    }

    /// Create new vertexes from an iterator.
    ///
    /// Time complexity: O(1) or O(|V|).
    ///
    /// Space complexity: O(|V|).
    pub fn new_vertexes_iter(
        &mut self,
        vertexes: impl Iterator<Item = RawVertex>,
    ) {
        for (identity, cost) in vertexes {
            self.new_vertex(identity, cost);
        }
    }

    /// Create a new edge.
    ///
    /// Time complexity: O(1) or O(|E|).
    ///
    /// Space complexity: O(|E|).
    pub fn new_edge(&mut self, begin: u64, end: u64, cost: u64) {
        let edge = Edge::new(begin, end, cost);
        let v_begin = match self.vertexes.get_mut(&begin) {
            None => panic!("expect: `begin` is existed"),
            Some(v) => v,
        };
        v_begin.edges.set(end, edge);
        let v_end = match self.vertexes.get_mut(&end) {
            None => panic!("expect: `end` is existed"),
            Some(v) => v,
        };
        v_end.connected_from.add(begin);
    }

    /// Create new edges from an array.
    ///
    /// Time complexity: O(1) or O(|E|).
    ///
    /// Space complexity: O(|E|).
    pub fn new_edges<const N: usize>(&mut self, edges: [RawEdge; N]) {
        self.new_edges_iter(edges.into_iter());
    }

    /// Create new edges from an iterator.
    ///
    /// Time complexity: O(1) or O(|E|).
    ///
    /// Space complexity: O(|E|).
    pub fn new_edges_iter(&mut self, edges: impl Iterator<Item = RawEdge>) {
        for (begin, end, cost) in edges {
            self.new_edge(begin, end, cost);
        }
    }

    /// For iteration over vertexes which are connected with the vertex `from`.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn travel(&self, from: u64) -> TravelIter {
        return TravelIter::new(from, self);
    }

    /// Find a lowest cost path from a vertex to another.
    ///
    /// Algorithm: Dijkstra.
    ///
    /// Time complexity: O(|V| + |E|).
    ///
    /// Space complexity: O(|V| + |E|).
    pub fn dijkstra(&self, from: u64, to: u64) -> Option<Path> {
        let source = match self.vertexes.get(&from) {
            None => panic!("expect: a existed node"),
            Some(v) => v,
        };
        match self.vertexes.get(&to) {
            None => panic!("expect: a existed node"),
            Some(v) => v,
        };
        let mut costs = HashMap::<u64, u64>::new();
        let mut prevs = HashMap::<u64, Option<&Vertex>>::new();
        let mut lowest = PriorityQueue::<Cost>::new();
        for v in self.vertexes.values() {
            costs.set(v.identity, u64::MAX);
            prevs.set(v.identity, None);
        }
        lowest.push(Cost::new(source, source.cost));
        loop {
            if lowest.size() == 0 {
                return None;
            }
            let cost = lowest.pop();
            if cost.vertex.identity == to {
                return self.build_path(from, to, &prevs);
            }
            for edge in cost.vertex.edges.values() {
                let end = self.vertexes.get(&edge.to).unwrap();
                let new_cost = cost.value + edge.cost + end.cost;
                let old_cost = costs.get(&edge.to).unwrap();
                if new_cost < *old_cost {
                    costs.set(edge.to, new_cost);
                    lowest.push(Cost::new(end, new_cost));
                    prevs.set(edge.to, Some(cost.vertex));
                }
            }
        }
    }

    /// Find a topological sort.
    ///
    /// Algorithm: Kahn.
    ///
    /// Time complexity: O(|V| + |E|).
    ///
    /// Space complexity: O(|V| + |E|).
    pub fn kahn(&self) -> Option<Vector<u64>> {
        let mut path = Vector::<u64>::new();
        let mut pool = self.find_independent_vetexes();
        let mut topological = HashMap::<u64, HashSet<u64>>::new();
        for v in self.vertexes.values() {
            let c = v.connected_from.clone();
            topological.set(v.identity, c);
        }
        loop {
            if pool.size() == 0 {
                break;
            }
            let v_id = pool.pop();
            let edges: Vector<Edge> = self
                .vertexes
                .get(&v_id)
                .unwrap()
                .edges
                .values()
                .map(|e| e.clone())
                .collect();
            path.push_back(v_id);
            for edge in edges.iter() {
                let end = self.vertexes.get(&edge.to).unwrap();
                let t = topological.get_mut(&end.identity).unwrap();
                t.remove(&v_id);
                if t.size() == 0 {
                    pool.push(end.identity);
                }
            }
        }
        for t in topological.values() {
            if t.size() > 0 {
                return None;
            }
        }
        return Some(path);
    }

    fn find_independent_vetexes(&self) -> Queue<u64> {
        let mut result = Queue::new();
        for v in self.vertexes.values() {
            if v.connected_from.size() == 0 {
                result.push(v.identity);
            }
        }
        return result;
    }

    fn build_path(
        &self,
        from: u64,
        to: u64,
        prevs: &HashMap<u64, Option<&Vertex>>,
    ) -> Option<Path> {
        let mut v_path = Vector::<u64>::new();
        let target = self.vertexes.get(&to).unwrap();
        let mut cost = target.cost;
        v_path.push_front(target.identity);
        loop {
            let current = v_path.get(0);
            if *current == from {
                return Some(Path {
                    vertexes: v_path,
                    cost: cost,
                });
            }
            let prev = match prevs.get(&current).unwrap() {
                None => return None,
                Some(v) => v,
            };
            let link_cost = prev.edges.get(current).unwrap().cost;
            cost = cost + link_cost + prev.cost;
            v_path.push_front(prev.identity);
        }
    }
}
