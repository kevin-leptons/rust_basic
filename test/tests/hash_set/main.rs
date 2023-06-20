mod helper;
use rust_basic::{hash_set::HashSet, Vector};
use testkit::NonZeroSize;

#[test]
fn new() {
    let set = HashSet::<String>::new();
    assert_eq!(set.size(), 0);
}

#[test]
fn add() {
    let mut set = HashSet::new();
    let size = 10000;
    for i in 0..size {
        let item = NonZeroSize::new(i);
        assert_eq!(set.add(item.clone()), false);
        assert_eq!(set.has(&item), true);
        assert_eq!(set.size(), i + 1);
    }
}

#[test]
fn has_return_false() {
    let mut set = HashSet::new();
    set.add(NonZeroSize::new(0));
    set.add(NonZeroSize::new(1));
    set.add(NonZeroSize::new(2));
    assert_eq!(set.has(&NonZeroSize::new(3)), false);
    assert_eq!(set.has(&NonZeroSize::new(4)), false);
}

#[test]
fn remove_return_false() {
    let mut set = HashSet::new();
    set.add(NonZeroSize::new(0));
    set.add(NonZeroSize::new(1));
    set.add(NonZeroSize::new(2));
    let size = set.size();
    assert_eq!(set.remove(&NonZeroSize::new(3)), false);
    assert_eq!(set.remove(&NonZeroSize::new(4)), false);
    assert_eq!(set.size(), size);
}

#[test]
fn remove_return_true() {
    let mut set = helper::non_zero_size_type();
    for i in 0..set.size() {
        assert_eq!(set.remove(&NonZeroSize::new(i)), true);
    }
    assert_eq!(set.size(), 0);
}

#[test]
fn from_iter() {
    let array = [
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
    ];
    let set = HashSet::from_iter(array.clone());
    for i in 0..array.len() {
        let item = NonZeroSize::new(i);
        assert_eq!(set.has(&item), true);
    }
    assert_eq!(set.size(), array.len());
}

#[test]
fn from_array() {
    let array = [
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
    ];
    let set = HashSet::from(array.clone());
    for i in 0..array.len() {
        let item = NonZeroSize::new(i);
        assert_eq!(set.has(&item), true);
    }
    assert_eq!(set.size(), array.len());
}

#[test]
fn iter() {
    let set = helper::non_zero_size_type();
    let size = set.size();
    let mut items = set.iter().map(|item| item.clone()).collect::<Vector<_>>();
    let mut expected = (0..size)
        .map(|i| NonZeroSize::new(i))
        .collect::<Vector<_>>();
    items.sort();
    expected.sort();
    assert_eq!(items, expected);
}

#[test]
fn iter_empty() {
    let set = HashSet::<NonZeroSize>::new();
    assert_eq!(set.iter().next(), None);
}

#[test]
fn equal_true() {
    let set0 = helper::non_zero_size_type();
    let set1 = helper::non_zero_size_type();
    assert!(set0.size() > 0);
    assert!(set1.size() > 0);
    assert_eq!(set0, set1);
}

#[test]
fn equal_false_item() {
    let set0 = HashSet::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
    ]);
    let set1 = HashSet::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(9),
    ]);
    assert_ne!(set0, set1);
}

#[test]
fn equal_false_size() {
    let set0 = HashSet::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
    ]);
    let set1 = HashSet::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
        NonZeroSize::new(3),
    ]);
    assert_ne!(set0, set1);
}

#[test]
fn clear() {
    let mut set = helper::non_zero_size_type();
    let size = set.size();
    set.clear();
    assert_eq!(set.size(), 0);
    for i in 0..size {
        let item = NonZeroSize::new(i);
        assert_eq!(set.has(&item), false);
    }
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
fn sample_must_not_empty() {
    let map = helper::non_zero_size_type();
    assert!(map.size() > 0);
}
