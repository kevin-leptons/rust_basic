mod helper;
use rust_basic::{DirectedGraph, Vector};

#[test]
fn new() {
    let graph = DirectedGraph::new();
    assert_eq!(graph.vertexes().next(), None);
}

#[test]
fn new_vertex() {
    let mut graph = DirectedGraph::new();
    let vertexes = [(0, 1), (1, 2), (2, 1), (4, 3)];
    for (identity, cost) in vertexes {
        graph.new_vertex(identity, cost);
    }
    let mut actual = graph
        .vertexes()
        .map(|v| (v.identity(), v.cost()))
        .collect::<Vector<_>>();
    let mut expected = Vector::from(vertexes);
    actual.sort();
    expected.sort();
    assert_eq!(actual, expected);
}

#[test]
#[should_panic(expected = "expect: not existed vertex")]
fn new_vertex_panic() {
    let mut graph = DirectedGraph::new();
    graph.new_vertex(0, 1);
    graph.new_vertex(0, 1);
}

#[test]
fn new_vertexes_iter() {
    let mut graph = DirectedGraph::new();
    let vertexes = [(0, 1), (1, 2), (2, 1), (4, 3)];
    graph.new_vertexes_iter(vertexes.clone().into_iter());
    let mut actual = graph
        .vertexes()
        .map(|v| (v.identity(), v.cost()))
        .collect::<Vector<_>>();
    let mut expected = Vector::from(vertexes);
    actual.sort();
    expected.sort();
    assert_eq!(actual, expected);
}

#[test]
fn new_vertexes() {
    let mut graph = DirectedGraph::new();
    let vertexes = [(0, 1), (1, 2), (2, 1), (4, 3)];
    graph.new_vertexes(vertexes);
    let mut actual = graph
        .vertexes()
        .map(|v| (v.identity(), v.cost()))
        .collect::<Vector<_>>();
    let mut expected = Vector::from(vertexes);
    actual.sort();
    expected.sort();
    assert_eq!(actual, expected);
}

#[test]
fn new_edge() {
    let mut graph = DirectedGraph::new();
    graph.new_vertexes([(0, 2), (1, 5), (2, 9)]);
    let edges = [(0, 1, 7), (1, 2, 3), (2, 1, 4)];
    for (begin, end, cost) in edges {
        graph.new_edge(begin, end, cost);
    }
    let mut edge_count = 0;
    for (begin, to, cost) in edges {
        let vertex = graph.vertexes().find(|v| v.identity() == begin).unwrap();
        let edge = vertex.edges().position(|e| {
            return (e.from() == begin) && (e.to() == to) && (e.cost() == cost);
        });
        assert!(edge.is_some());
        edge_count += vertex.edges().count();
    }
    assert_eq!(edge_count, edges.len());
}

#[test]
#[should_panic(expected = "expect: not existed edge")]
fn new_edge_panic() {
    let mut graph = DirectedGraph::new();
    graph.new_vertexes([(0, 2), (1, 5), (2, 9)]);
    graph.new_edge(0, 1, 3);
    graph.new_edge(0, 1, 2);
}

#[test]
#[should_panic(expected = "expect: existed vertex")]
fn new_edge_panic_begin() {
    let mut graph = DirectedGraph::new();
    graph.new_vertexes([(0, 2), (1, 5), (2, 9)]);
    graph.new_edge(9, 1, 3);
}

#[test]
#[should_panic(expected = "expect: existed vertex")]
fn new_edge_panic_end() {
    let mut graph = DirectedGraph::new();
    graph.new_vertexes([(0, 2), (1, 5), (2, 9)]);
    graph.new_edge(1, 9, 3);
}

#[test]
fn new_edges_iter() {
    let mut graph = DirectedGraph::new();
    graph.new_vertexes([(0, 2), (1, 5), (2, 9)]);
    let edges = [(0, 1, 7), (0, 2, 5), (1, 2, 3), (2, 1, 4)];
    graph.new_edges_iter(edges.clone().into_iter());
    let mut edge_count = 0;
    for (begin, to, cost) in edges {
        let vertex = graph.vertexes().find(|v| v.identity() == begin).unwrap();
        let edge = vertex.edges().position(|e| {
            return (e.from() == begin) && (e.to() == to) && (e.cost() == cost);
        });
        assert!(edge.is_some());
        edge_count += 1;
    }
    assert_eq!(edge_count, edges.len());
}

#[test]
fn new_edges() {
    let mut graph = DirectedGraph::new();
    graph.new_vertexes([(0, 2), (1, 5), (2, 9)]);
    let edges = [(0, 1, 7), (0, 2, 5), (1, 2, 3), (2, 1, 4)];
    graph.new_edges(edges.clone());
    let mut edge_count = 0;
    for (begin, to, cost) in edges {
        let vertex = graph.vertexes().find(|v| v.identity() == begin).unwrap();
        let edge = vertex.edges().position(|e| {
            return (e.from() == begin) && (e.to() == to) && (e.cost() == cost);
        });
        assert!(edge.is_some());
        edge_count += 1;
    }
    assert_eq!(edge_count, edges.len());
}

#[test]
fn travel() {
    let graph = helper::dijkstra();
    let mut visited = graph
        .travel(0)
        .map(|vertex| vertex.identity())
        .collect::<Vector<_>>();
    visited.sort();
    assert_eq!(visited, Vector::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
}

#[test]
fn dijkstra() {
    let graph = helper::dijkstra();
    let path = graph.dijkstra(0, 9).unwrap();
    assert_eq!(path.vertexes(), &Vector::from([0, 1, 3, 6, 7, 5, 9]));
    assert_eq!(path.cost(), 30);
}

#[test]
fn kahn() {
    let graph = helper::topological();
    let actual = graph.kahn().unwrap();
    assert!(helper::ordered_pair(&actual, 0, 1));
    assert!(helper::ordered_pair(&actual, 0, 3));
    assert!(helper::ordered_pair(&actual, 3, 6));
    assert!(helper::ordered_pair(&actual, 6, 7));
    assert!(helper::ordered_pair(&actual, 6, 4));
    assert!(helper::ordered_pair(&actual, 6, 4));
    assert!(helper::ordered_pair(&actual, 7, 8));
    assert!(helper::ordered_pair(&actual, 8, 9));
    assert!(helper::ordered_pair(&actual, 7, 5));
    assert!(helper::ordered_pair(&actual, 7, 5));
    assert!(helper::ordered_pair(&actual, 5, 9));
    assert!(helper::ordered_pair(&actual, 4, 2));
    assert!(helper::ordered_pair(&actual, 2, 5));
}
