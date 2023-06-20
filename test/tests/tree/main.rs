mod helper;

use rust_basic::{Tree, Vector};
use testkit::NonZeroSize;

#[test]
fn new() {
    let value = NonZeroSize::new(1);
    let tree = Tree::new(value.clone());
    assert_eq!(tree.children_size(), 0);
    assert_eq!(tree.children().next(), None);
    assert_eq!(tree.value, value);
}

#[test]
fn add_children() {
    let mut tree0 = Tree::new(NonZeroSize::new(0));
    let children = [
        Tree::new(NonZeroSize::new(1)),
        Tree::new(NonZeroSize::new(2)),
        Tree::new(NonZeroSize::new(3)),
    ];
    let size = children.len();
    tree0.add_children(children);
    let mut children = tree0
        .children()
        .map(|tree| tree.value.clone())
        .collect::<Vector<_>>();
    assert_eq!(children.size(), size);
    let mut expected = (1..size + 1)
        .map(|i| NonZeroSize::new(i))
        .collect::<Vector<_>>();
    children.sort();
    expected.sort();
    assert_eq!(children, expected);
    assert_eq!(tree0.children_size(), size);
}

#[test]
fn remove_children() {
    let mut tree = helper::non_zero_size_type();
    let children_size = tree.children_size();
    for i in 0..children_size {
        let child = tree.remove_child(children_size - i - 1);
        assert_eq!(child.value, NonZeroSize::new(children_size - i));
        assert_eq!(tree.children_size(), children_size - i - 1);
    }
}

#[test]
fn children() {
    let tree = helper::non_zero_size_type();
    let children = tree
        .children()
        .map(|tree| tree.value.clone())
        .collect::<Vector<_>>();
    let expected = Vector::from([
        NonZeroSize::new(1),
        NonZeroSize::new(2),
        NonZeroSize::new(3),
    ]);
    assert_eq!(children, expected);
}

#[test]
fn travel_pre_order() {
    let tree = helper::non_zero_size_type();
    let children = tree
        .travel_pre_order()
        .map(|tree| tree.value.clone())
        .collect::<Vector<_>>();
    let expected = Vector::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
        NonZeroSize::new(4),
        NonZeroSize::new(3),
        NonZeroSize::new(5),
        NonZeroSize::new(6),
        NonZeroSize::new(7),
        NonZeroSize::new(8),
        NonZeroSize::new(9),
    ]);
    assert_eq!(children, expected);
}

#[test]
fn travel_post_order() {
    let tree = helper::non_zero_size_type();
    let children = tree
        .travel_post_order()
        .map(|tree| tree.value.clone())
        .collect::<Vector<_>>();
    let expected = Vector::from([
        NonZeroSize::new(1),
        NonZeroSize::new(4),
        NonZeroSize::new(2),
        NonZeroSize::new(5),
        NonZeroSize::new(7),
        NonZeroSize::new(9),
        NonZeroSize::new(8),
        NonZeroSize::new(6),
        NonZeroSize::new(3),
        NonZeroSize::new(0),
    ]);
    assert_eq!(children, expected);
}

#[test]
fn travel_level_order() {
    let tree = helper::non_zero_size_type();
    let children = tree
        .travel_level_order()
        .map(|tree| tree.value.clone())
        .collect::<Vector<_>>();
    let expected = Vector::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
        NonZeroSize::new(3),
        NonZeroSize::new(4),
        NonZeroSize::new(5),
        NonZeroSize::new(6),
        NonZeroSize::new(7),
        NonZeroSize::new(8),
        NonZeroSize::new(9),
    ]);
    assert_eq!(children, expected);
}

// This test does nothing but creating a non empty container to trigger memory
// release process. The test can not work alone, it requries an external tool
// such as Valgrind to diagnose memory issues.
//
// Warn: The test maybe still passed even memory release process has issues.
#[test]
fn drop() {
    let _ = helper::non_zero_size_type();
}

#[test]
fn drop_empty() {
    let _ = Tree::new(NonZeroSize::new(1));
}
