use rust_basic::{tree::Tree, Vector};

#[test]
fn add_child() {
    let mut t0 = Tree::new("tree: 0".to_string());
    let t1 = Tree::new("tree: 1".to_string());
    let t2 = Tree::new("tree: 2".to_string());
    t0.add_children([t1, t2]);
    assert_eq!(t0.children_size(), 2);
    for i in 0..t0.children_size() {
        assert_eq!(t0.get_child(i).value, format!("tree: {}", i + 1));
    }
}

#[test]
fn remove_child() {
    let mut t0 = Tree::new("tree: 0".to_string());
    let t1 = Tree::new("tree: 1".to_string());
    let t2 = Tree::new("tree: 2".to_string());
    let t3 = Tree::new("tree: 3".to_string());
    t0.add_children([t1, t2, t3]);
    assert_eq!(t0.children_size(), 3);
    t0.remove_child(1);
    assert_eq!(t0.children_size(), 2);
    assert_eq!(t0.get_child(0).value, "tree: 1".to_string());
    assert_eq!(t0.get_child(1).value, "tree: 3".to_string());
}

#[test]
fn iter() {
    let mut t0 = Tree::new("tree: 0".to_string());
    let t1 = Tree::new("tree: 1".to_string());
    let t2 = Tree::new("tree: 2".to_string());
    let t3 = Tree::new("tree: 3".to_string());
    t0.add_children([t1, t2, t3]);
    assert_eq!(t0.children_size(), 3);
    let values: Vector<&str> =
        t0.children().map(|t| t.value.as_str()).collect();
    assert_eq!(values, Vector::from(["tree: 1", "tree: 2", "tree: 3"]));
}

#[test]
fn travel_pre_order() {
    let t = build();
    let values: Vector<&str> = t
        .travel_pre_order()
        .map(|tree| tree.value.as_str())
        .collect();
    assert_eq!(
        values,
        Vector::from([
            "tree: 0", "tree: 1", "tree: 4", "tree: 5", "tree: 6", "tree: 2",
            "tree: 3", "tree: 7", "tree: 8", "tree: 9",
        ])
    );
}

#[test]
fn travel_post_order() {
    let t = build();
    let values: Vector<&str> = t
        .travel_post_order()
        .map(|tree| tree.value.as_str())
        .collect();
    assert_eq!(
        values,
        Vector::from([
            "tree: 4", "tree: 5", "tree: 6", "tree: 1", "tree: 7", "tree: 9",
            "tree: 8", "tree: 3", "tree: 2", "tree: 0",
        ])
    );
}

#[test]
fn travel_level_order() {
    let t = build();
    let values: Vector<&str> = t
        .travel_level_order()
        .map(|tree| tree.value.as_str())
        .collect();
    assert_eq!(
        values,
        Vector::from([
            "tree: 0", "tree: 1", "tree: 2", "tree: 4", "tree: 5", "tree: 6",
            "tree: 3", "tree: 7", "tree: 8", "tree: 9",
        ])
    );
}

/// Build from bottom to top, a tree like this:
///
///             0
///            / \
///           /   \
///          /      \
///         1        2
///      /  |  \       \
///     4   5   6       3
///                    /  \
///                   7    8
///                         \
///                          9
fn build() -> Tree<String> {
    let mut t0 = Tree::new("tree: 0".to_string());
    let mut t1 = Tree::new("tree: 1".to_string());
    let mut t2 = Tree::new("tree: 2".to_string());
    let mut t3 = Tree::new("tree: 3".to_string());
    let t4 = Tree::new("tree: 4".to_string());
    let t5 = Tree::new("tree: 5".to_string());
    let t6 = Tree::new("tree: 6".to_string());
    let t7 = Tree::new("tree: 7".to_string());
    let mut t8 = Tree::new("tree: 8".to_string());
    let t9 = Tree::new("tree: 9".to_string());
    t1.add_children([t4, t5, t6]);
    t0.add_child(t1);
    t8.add_child(t9);
    t3.add_children([t7, t8]);
    t2.add_child(t3);
    t0.add_child(t2);
    return t0;
}
