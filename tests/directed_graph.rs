use rust_basic::directed_graph::DirectedGraph;
use rust_basic::{HashMap, Vector};

#[test]
fn new_vertex() {
    let mut g = DirectedGraph::new();
    let vertexes = [(0, 1), (1, 2), (2, 1), (4, 3)];
    for (identity, cost) in vertexes {
        g.new_vertex(identity, cost);
    }
    let iter_result = g.vertexes().map(|v| (v.identity(), v.cost()));
    let actual = HashMap::from_iter(iter_result);
    let expected = HashMap::from(vertexes);
    assert_eq!(actual, expected);
}

#[test]
fn new_vertexes() {
    let mut g = DirectedGraph::new();
    let vertexes = [(0, 1), (1, 2), (2, 1), (4, 3)];
    g.new_vertexes(vertexes);
    let iter_result = g.vertexes().map(|v| (v.identity(), v.cost()));
    let actual = HashMap::from_iter(iter_result);
    let expected = HashMap::from(vertexes);
    assert_eq!(actual, expected);
}

#[test]
fn new_vertexes_iter() {
    let mut g = DirectedGraph::new();
    let vertexes = [(0, 1), (1, 2), (2, 1), (4, 3)];
    g.new_vertexes_iter(vertexes.into_iter());
    let iter_result = g.vertexes().map(|v| (v.identity(), v.cost()));
    let actual = HashMap::from_iter(iter_result);
    let expected = HashMap::from(vertexes);
    assert_eq!(actual, expected);
}

#[test]
fn new_edge() {
    let mut g = DirectedGraph::new();
    g.new_vertexes([(0, 2), (1, 5), (2, 9)]);
    let edges = [(0, 1, 7), (1, 2, 3), (2, 1, 4)];
    for (begin, end, cost) in edges {
        g.new_edge(begin, end, cost);
    }
    for (begin, to, cost) in edges {
        let vertex = g.vertexes().find(|v| v.identity() == begin).unwrap();
        let edge = vertex.edges().position(|e| {
            return (e.from() == begin) && (e.to() == to) && (e.cost() == cost);
        });
        assert!(edge.is_some());
    }
}

#[test]
fn new_edges() {
    let mut g = DirectedGraph::new();
    g.new_vertexes([(0, 2), (1, 5), (2, 9)]);
    let edges = [(0, 1, 7), (1, 2, 3), (2, 1, 4)];
    g.new_edges(edges);
    for (begin, to, cost) in edges {
        let vertex = g.vertexes().find(|v| v.identity() == begin).unwrap();
        let edge = vertex.edges().position(|e| {
            return (e.from() == begin) && (e.to() == to) && (e.cost() == cost);
        });
        assert!(edge.is_some());
    }
}

#[test]
fn new_edges_iter() {
    let mut g = DirectedGraph::new();
    g.new_vertexes([(0, 2), (1, 5), (2, 9)]);
    let edges = [(0, 1, 7), (1, 2, 3), (2, 1, 4)];
    g.new_edges_iter(edges.into_iter());
    for (begin, to, cost) in edges {
        let vertex = g.vertexes().find(|v| v.identity() == begin).unwrap();
        let edge = vertex.edges().position(|e| {
            return (e.from() == begin) && (e.to() == to) && (e.cost() == cost);
        });
        assert!(edge.is_some());
    }
}

#[test]
fn travel() {
    let mut g = sample_dijkstra();
    let mut visited: Vector<u64> = g.travel(0).map(|v| v.identity()).collect();
    visited.sort();
    assert_eq!(visited, Vector::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
}

#[test]
fn dijkstra() {
    let g = sample_dijkstra();
    let result = g.dijkstra(0, 9);
    assert!(result.is_some());
    let path = result.unwrap();
    assert_eq!(path.vertexes(), &Vector::from([0, 1, 3, 6, 7, 5, 9]));
    assert_eq!(path.cost(), 30);
}

#[test]
fn kahn() {
    let mut g = sample_topological();
    let result = g.kahn();
    assert!(result.is_some());
    let actual = result.unwrap();
    assert!(is_ordered(&actual, 0, 1));
    assert!(is_ordered(&actual, 0, 3));
    assert!(is_ordered(&actual, 3, 6));
    assert!(is_ordered(&actual, 6, 7));
    assert!(is_ordered(&actual, 6, 4));
    assert!(is_ordered(&actual, 6, 4));
    assert!(is_ordered(&actual, 7, 8));
    assert!(is_ordered(&actual, 8, 9));
    assert!(is_ordered(&actual, 7, 5));
    assert!(is_ordered(&actual, 7, 5));
    assert!(is_ordered(&actual, 5, 9));
    assert!(is_ordered(&actual, 4, 2));
    assert!(is_ordered(&actual, 2, 5));
}

/// Build a graph like this:
///
/// ```
///          1
/// [0, 2] -----> [1, 1]        [2, 3]-----+
///   |             |             ^        |
///   |             |             |        |
///   4             1             2        3
///   |             |             |        |
///   v             |             |        |                  2
/// [3, 2]<---------+           [4, 7]<----+        [5, 1]-------->[9, 4]
///   |                           ^                  ^  ^            ^
///   |                           |                  |  |            |
///   3                  1        |                  |  |            |
///   |    +----------------------+                  |  |            9
///   |    /                                6        |  |            |
///   |   /               +--------------------------+  |            |
///   |  /               /                              |            |
///   v /     2        /      1                1        |            |
/// [6, 2]--------->[7, 3]------->[8, 5]----------------+            |
///                   ^            |  |                              |
///                   |      0     |  |                              |
///                   +------------+  +------------------------------+
fn sample_dijkstra() -> DirectedGraph {
    let mut g = DirectedGraph::new();
    g.new_vertexes([
        (0, 2),
        (1, 1),
        (2, 3),
        (3, 2),
        (4, 7),
        (5, 1),
        (6, 2),
        (7, 3),
        (8, 5),
        (9, 4),
    ]);
    g.new_edges([
        (0, 1, 1),
        (0, 3, 4),
        (1, 3, 1),
        (3, 6, 3),
        (6, 4, 1),
        (4, 2, 2),
        (2, 4, 3),
        (6, 7, 2),
        (7, 5, 6),
        (7, 8, 1),
        (8, 7, 0),
        (8, 5, 1),
        (8, 9, 9),
        (5, 9, 2),
    ]);
    return g;
}

/// Build a graph like this:
///
/// ```
///          0
/// [0, 2] -----> [1, 1]        [2, 3]----------------+
///   |             |             ^                   |
///   |             |             |                   |
///   0             0             0                   |
///   |             |             |                   |
///   v             |             |                   v
/// [3, 2]<---------+           [4, 7]              [5, 1]-------->[9, 4]
///   |                           ^                  ^  ^            ^
///   |                           |                  |  |            |
///   0                  0        |                  |  |            |
///   |    +----------------------+                  |  |            0
///   |    /                                0        |  |            |
///   |   /               +--------------------------+  |            |
///   |  /               /                              |            |
///   v /     0        /      0                0        |            |
/// [6, 2]--------->[7, 3]------->[8, 5]----------------+            |
///                                   |                              |
///                                   |                              |
///                                   +------------------------------+
fn sample_topological() -> DirectedGraph {
    let mut g = DirectedGraph::new();
    g.new_vertexes([
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (8, 0),
        (9, 0),
    ]);
    g.new_edges([
        (0, 1, 0),
        (0, 3, 0),
        (1, 3, 0),
        (3, 6, 0),
        (6, 4, 0),
        (4, 2, 0),
        (2, 5, 0),
        (5, 9, 0),
        (6, 7, 0),
        (7, 5, 0),
        (7, 8, 0),
        (8, 9, 0),
    ]);
    return g;
}

/// * Time complexity: O(n).
/// * Space complexity: O(n).
fn is_ordered(vector: &Vector<u64>, before: u64, after: u64) -> bool {
    let mut before_index = Option::<u64>::None;
    let mut after_index = Option::<u64>::None;
    for i in 0..vector.size() {
        if vector[i] == before {
            before_index = Some(i as u64);
        }
        if vector[i] == after {
            after_index = Some(i as u64);
        }
        if before_index.is_some() && after_index.is_some() {
            return before_index < after_index;
        }
    }
    return false;
}
