use rust_basic::{DirectedGraph, Vector};

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
pub(super) fn dijkstra() -> DirectedGraph {
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
pub(super) fn topological() -> DirectedGraph {
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
pub(super) fn ordered_pair(
    vector: &Vector<u64>,
    before: u64,
    after: u64,
) -> bool {
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
