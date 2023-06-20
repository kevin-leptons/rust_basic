use rust_basic::UndirectedGraph;

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
pub(super) fn sample() -> UndirectedGraph {
    let mut graph = UndirectedGraph::new();
    graph.new_vertexes([00, 01, 02, 03, 04, 05, 06, 07, 08]);
    graph.new_edges([
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
    return graph;
}