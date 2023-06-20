mod helper;
use rust_basic::UndirectedGraph;
use rust_basic::Vector;

#[test]
fn new() {
    let graph = UndirectedGraph::new();
    assert_eq!(graph.vertexes().next(), None);
    assert_eq!(graph.edges().next(), None);
}

#[test]
fn new_vertex() {
    let mut graph = UndirectedGraph::new();
    let vertexes = [0, 1, 2];
    for vertex in vertexes {
        graph.new_vertex(vertex);
    }
    let mut vertex_ids = graph
        .vertexes()
        .map(|v| v.identity())
        .collect::<Vector<_>>();
    let mut expected = Vector::from(vertexes);
    vertex_ids.sort();
    expected.sort();
    assert_eq!(vertex_ids, expected);
}

#[test]
#[should_panic(expected = "expect: a not existed vertex")]
fn new_vertex_panic() {
    let mut graph = UndirectedGraph::new();
    graph.new_vertex(0);
    graph.new_vertex(1);
    graph.new_vertex(1);
}

#[test]
fn new_vertexes_iter() {
    let mut graph = UndirectedGraph::new();
    let vertexes = [0, 1, 2];
    graph.new_vertexes_iter(vertexes.clone().into_iter());
    let mut vertex_ids = graph
        .vertexes()
        .map(|v| v.identity())
        .collect::<Vector<_>>();
    let mut expected = Vector::from(vertexes);
    vertex_ids.sort();
    expected.sort();
    assert_eq!(vertex_ids, expected);
}

#[test]
fn new_vertexes_array() {
    let mut graph = UndirectedGraph::new();
    let vertexes = [0, 1, 2];
    graph.new_vertexes(vertexes);
    let mut vertex_ids = graph
        .vertexes()
        .map(|v| v.identity())
        .collect::<Vector<_>>();
    let mut expected = Vector::from(vertexes);
    vertex_ids.sort();
    expected.sort();
    assert_eq!(vertex_ids, expected);
}

#[test]
fn new_edge() {
    let mut graph = UndirectedGraph::new();
    graph.new_vertexes([0, 1, 2]);
    let raw_edges = [(0, 1, 9), (1, 2, 7), (2, 0, 5)];
    for edge in raw_edges {
        graph.new_edge(edge);
    }
    let mut edges = graph
        .edges()
        .map(|e| (e.vertexes()[0], e.vertexes()[1], e.cost()))
        .collect::<Vector<_>>();
    let mut expected = raw_edges
        .into_iter()
        .map(|(vertex0, vertex1, cost)| {
            return match vertex0 <= vertex1 {
                true => (vertex0, vertex1, cost),
                false => (vertex1, vertex0, cost),
            };
        })
        .collect::<Vector<_>>();
    edges.sort();
    expected.sort();
    assert_eq!(edges, expected);
}

#[test]
#[should_panic(expected = "expect: a not existed edge")]
fn new_edge_panic() {
    let mut graph = UndirectedGraph::new();
    graph.new_vertexes([0, 1, 2]);
    let raw_edges = [(0, 1, 9), (1, 2, 5), (1, 2, 7)];
    for edge in raw_edges {
        graph.new_edge(edge);
    }
}

#[test]
#[should_panic(expected = "expect: existed vertexes")]
fn new_edge_panic_first_vertex() {
    let mut graph = UndirectedGraph::new();
    graph.new_vertexes([0, 1, 2]);
    let raw_edges = [(0, 1, 9), (9, 2, 5)];
    for edge in raw_edges {
        graph.new_edge(edge);
    }
}

#[test]
#[should_panic(expected = "expect: existed vertexes")]
fn new_edge_panic_second_vertex() {
    let mut graph = UndirectedGraph::new();
    graph.new_vertexes([0, 1, 2]);
    let raw_edges = [(0, 1, 9), (2, 9, 5)];
    for edge in raw_edges {
        graph.new_edge(edge);
    }
}

#[test]
fn new_edges_iter() {
    let mut graph = UndirectedGraph::new();
    graph.new_vertexes([0, 1, 2]);
    let raw_edges = [(0, 1, 9), (1, 2, 7), (2, 0, 5)];
    graph.new_edges_iter(raw_edges.clone().into_iter());
    let mut edges = graph
        .edges()
        .map(|e| (e.vertexes()[0], e.vertexes()[1], e.cost()))
        .collect::<Vector<_>>();
    let mut expected = raw_edges
        .into_iter()
        .map(|(vertex0, vertex1, cost)| {
            return match vertex0 <= vertex1 {
                true => (vertex0, vertex1, cost),
                false => (vertex1, vertex0, cost),
            };
        })
        .collect::<Vector<_>>();
    edges.sort();
    expected.sort();
    assert_eq!(edges, expected);
}

#[test]
fn new_edges_array() {
    let mut graph = UndirectedGraph::new();
    graph.new_vertexes([0, 1, 2]);
    let raw_edges = [(0, 1, 9), (1, 2, 7), (2, 0, 5)];
    graph.new_edges(raw_edges.clone());
    let mut edges = graph
        .edges()
        .map(|e| (e.vertexes()[0], e.vertexes()[1], e.cost()))
        .collect::<Vector<_>>();
    let mut expected = raw_edges
        .into_iter()
        .map(|(vertex0, vertex1, cost)| {
            return match vertex0 <= vertex1 {
                true => (vertex0, vertex1, cost),
                false => (vertex1, vertex0, cost),
            };
        })
        .collect::<Vector<_>>();
    edges.sort();
    expected.sort();
    assert_eq!(edges, expected);
}

#[test]
fn travel() {
    let graph = helper::sample();
    let mut visisted: Vector<u64> =
        graph.travel(06).map(|vertex| vertex.identity()).collect();
    visisted.sort();
    assert_eq!(visisted, Vector::from([04, 05, 06, 07, 08]));
}

#[test]
#[should_panic(expected = "expect: existed vertex")]
fn travel_panic() {
    let graph = helper::sample();
    graph.travel(u64::MAX);
}

#[test]
fn kruskal() {
    let graph = helper::sample();
    let forest = graph.kruskal();
    let mut actual_vertexes = forest
        .vertexes()
        .map(|v| v.identity())
        .collect::<Vector<_>>();
    let mut expected_vertexes =
        Vector::from([00, 01, 02, 03, 04, 05, 06, 07, 08]);
    actual_vertexes.sort();
    expected_vertexes.sort();
    assert_eq!(actual_vertexes, expected_vertexes);
    let mut actual_edges = forest
        .edges()
        .map(|e| (e.vertexes()[0], e.vertexes()[1], e.cost()))
        .collect::<Vector<_>>();
    let expected_edges_array = [
        (01, 03, 1),
        (02, 03, 2),
        (00, 01, 3),
        (04, 06, 1),
        (04, 07, 2),
        (06, 08, 4),
        (05, 08, 5),
    ];
    let mut expected_edges = expected_edges_array
        .into_iter()
        .map(|(vertex0, vertex1, cost)| {
            return match vertex0 <= vertex1 {
                true => (vertex0, vertex1, cost),
                false => (vertex1, vertex0, cost),
            };
        })
        .collect::<Vector<_>>();
    actual_edges.sort();
    expected_edges.sort();
    assert_eq!(actual_edges, expected_edges);
}

// This test does nothing but creating a non empty container to trigger memory
// release process. The test can not work alone, it requries an external tool
// such as Valgrind to diagnose memory issues.
//
// Warn: The test maybe still passed even memory release process has issues.
#[test]
fn drop() {
    let _ = helper::sample();
}

#[test]
fn sample_must_not_empty() {
    let graph = helper::sample();
    assert_ne!(graph.vertexes().next(), None);
    assert_ne!(graph.edges().next(), None);
}
