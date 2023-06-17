use rust_basic::hash_set::HashSet;
use rust_basic::undirected_graph::UndirectedGraph;
use rust_basic::Vector;

#[test]
fn new_vertex() {
    let mut g = UndirectedGraph::new();
    let vertexes = [0, 1, 2];
    for v in vertexes {
        g.new_vertex(v);
    }
    let mut expected = [false; 3];
    for v in g.vertexes() {
        let r = vertexes.iter().position(|i| *i == v.identity());
        assert_eq!(r.is_some(), true);
        let p = r.unwrap();
        assert_eq!(expected[p], false);
        expected[p] = true;
    }
    for v in expected {
        assert_eq!(v, true);
    }
}

#[test]
#[should_panic(expected = "expect: a not existed vertex")]
fn new_vertex_panic() {
    let mut g = UndirectedGraph::new();
    let vertexes = [0, 1, 1];
    for v in vertexes {
        g.new_vertex(v);
    }
}

#[test]
fn new_vertexes() {
    let mut g = UndirectedGraph::new();
    let vertexes = [0, 1, 2, 8, 9];
    g.new_vertexes(vertexes);
    let mut expected = [false; 5];
    for v in g.vertexes() {
        let r = vertexes.iter().position(|i| *i == v.identity());
        assert_eq!(r.is_some(), true);
        let p = r.unwrap();
        assert_eq!(expected[p], false);
        expected[p] = true;
    }
    for v in expected {
        assert_eq!(v, true);
    }
}

#[test]
fn new_vertexes_iter() {
    let mut g = UndirectedGraph::new();
    let vertexes = [0, 1, 2, 8, 9];
    g.new_vertexes_iter(vertexes.into_iter());
    let mut expected = [false; 5];
    for v in g.vertexes() {
        let r = vertexes.iter().position(|i| *i == v.identity());
        assert_eq!(r.is_some(), true);
        let p = r.unwrap();
        assert_eq!(expected[p], false);
        expected[p] = true;
    }
    for v in expected {
        assert_eq!(v, true);
    }
}

#[test]
fn new_edge() {
    let mut g = UndirectedGraph::new();
    g.new_vertexes([0, 1, 2]);
    let edges = [(0, 1, 9), (1, 2, 7), (2, 0, 5)];
    for e in edges {
        g.new_edge(e);
    }
    let mut expected = [false; 3];
    for e in g.edges() {
        let r = edges.iter().position(|(i_v1, i_v2, i_cost)| {
            let [e_v1, e_v2] = e.vertexes();
            return (*i_v1 == e_v1 && *i_v2 == e_v2 && *i_cost == e.cost())
                || (*i_v1 == e_v2 && *i_v2 == e_v1 && *i_cost == e.cost());
        });
        assert_eq!(r.is_some(), true);
        let p = r.unwrap();
        assert_eq!(expected[p], false);
        expected[p] = true;
    }
    for v in expected {
        assert_eq!(v, true);
    }
}

#[test]
#[should_panic(expected = "expect: a not existed edge")]
fn new_edge_panic_existed() {
    let mut g = UndirectedGraph::new();
    g.new_vertexes([0, 1, 2]);
    let edges = [(0, 1, 9), (1, 2, 5), (1, 2, 7)];
    for e in edges {
        g.new_edge(e);
    }
}

#[test]
#[should_panic(expected = "expect: existed vertexes")]
fn new_edge_panic_first_vertex() {
    let mut g = UndirectedGraph::new();
    g.new_vertexes([0, 1, 2]);
    let edges = [(0, 1, 9), (9, 2, 5)];
    for e in edges {
        g.new_edge(e);
    }
}

#[test]
#[should_panic(expected = "expect: existed vertexes")]
fn new_edge_panic_second_vertex() {
    let mut g = UndirectedGraph::new();
    g.new_vertexes([0, 1, 2]);
    let edges = [(0, 1, 9), (2, 9, 5)];
    for e in edges {
        g.new_edge(e);
    }
}

#[test]
fn new_edges() {
    let mut g = UndirectedGraph::new();
    g.new_vertexes([0, 1, 2]);
    let edges = [(0, 1, 9), (1, 2, 7), (2, 0, 5)];
    g.new_edges(edges);
    let mut expected = [false; 3];
    for e in g.edges() {
        let r = edges.iter().position(|(i_v1, i_v2, i_cost)| {
            let [e_v1, e_v2] = e.vertexes();
            return (*i_v1 == e_v1 && *i_v2 == e_v2 && *i_cost == e.cost())
                || (*i_v1 == e_v2 && *i_v2 == e_v1 && *i_cost == e.cost());
        });
        assert_eq!(r.is_some(), true);
        let p = r.unwrap();
        assert_eq!(expected[p], false);
        expected[p] = true;
    }
    for v in expected {
        assert_eq!(v, true);
    }
}

#[test]
fn new_edges_iter() {
    let mut g = UndirectedGraph::new();
    g.new_vertexes([0, 1, 2]);
    let edges = [(0, 1, 9), (1, 2, 7), (2, 0, 5)];
    g.new_edges_iter(edges.into_iter());
    let mut expected = [false; 3];
    for e in g.edges() {
        let r = edges.iter().position(|(i_v1, i_v2, i_cost)| {
            let [e_v1, e_v2] = e.vertexes();
            return (*i_v1 == e_v1 && *i_v2 == e_v2 && *i_cost == e.cost())
                || (*i_v1 == e_v2 && *i_v2 == e_v1 && *i_cost == e.cost());
        });
        assert_eq!(r.is_some(), true);
        let p = r.unwrap();
        assert_eq!(expected[p], false);
        expected[p] = true;
    }
    for v in expected {
        assert_eq!(v, true);
    }
}

#[test]
fn travel() {
    let mut g = sample();
    let mut visisted: Vector<u64> =
        g.travel(06).map(|v| v.identity()).collect();
    visisted.sort();
    assert_eq!(visisted, Vector::from([04, 05, 06, 07, 08]));
}

#[test]
fn kruskal() {
    let g = sample();
    let r = g.kruskal();
    let actual_vertexes =
        HashSet::<u64>::from_iter(r.vertexes().map(|v| v.identity()));
    let expected_vertexes =
        HashSet::<u64>::from([00, 01, 02, 03, 04, 05, 06, 07, 08]);
    assert_eq!(actual_vertexes, expected_vertexes);
    let expected_edges = [
        [01, 03],
        [02, 03],
        [00, 01],
        [04, 06],
        [04, 07],
        [06, 08],
        [05, 08],
    ];
    let mut x = [false; 7];
    for e in r.edges() {
        let i = expected_edges.iter().position(|[i_v1, i_v2]| {
            let [e_v1, e_v2] = e.vertexes();
            return (e_v1 == *i_v1 && e_v2 == *i_v2)
                || (e_v1 == *i_v2 && e_v2 == *i_v1);
        });
        assert_eq!(i.is_some(), true);
        let p = i.unwrap();
        assert_eq!(x[p], false);
        x[p] = true;
    }
    for i in x {
        assert_eq!(i, true);
    }
}

/// Build a graph like this:
///
/// ```txt
///           3
/// [ 00 ]---------[ 01 ]
///   |              |
///   |              |
///   4              1
///   |              |
///   |       2      |
/// [ 02 ]---------[ 03 ]
///
///
///                   7
/// [ 04 ]--------------------------[ 05 ]
///  |  |                            |  |
///  |  |                            |  |
///  |  |                            |  |
///  |  |                            |  |
///  |  |    1                6      |  |
///  |  +----------[ 06 ]------------+  |
///  |              |  |                5
///  2              |  |                |
///  |       3      |  |      4         |
///  |  +-----------+  +-------------+  |
///  |  |                            |  |
/// [ 07 ]--------------------------[ 08 ]
/// ```
fn sample() -> UndirectedGraph {
    let mut g = UndirectedGraph::new();
    g.new_vertexes([00, 01, 02, 03, 04, 05, 06, 07, 08]);
    g.new_edges([
        (0, 1, 3),
        (0, 2, 4),
        (1, 3, 1),
        (3, 2, 2),
        (4, 5, 7),
        (4, 6, 1),
        (4, 7, 2),
        (5, 8, 5),
        (5, 6, 6),
        (6, 7, 3),
        (6, 8, 4),
        (7, 8, 8),
    ]);
    return g;
}
