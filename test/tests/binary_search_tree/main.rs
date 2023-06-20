mod helper;

use rust_basic::{BinarySearchTree, Vector};
use testkit::NonZeroSize;

#[test]
fn new() {
    let tree = BinarySearchTree::<NonZeroSize, NonZeroSize>::new();
    assert_eq!(tree.size(), 0);
}

#[test]
fn set() {
    let mut tree = BinarySearchTree::new();
    let array = [
        (NonZeroSize::new(0), NonZeroSize::new(("value", 0))),
        (NonZeroSize::new(1), NonZeroSize::new(("value", 1))),
        (NonZeroSize::new(2), NonZeroSize::new(("value", 2))),
        (NonZeroSize::new(3), NonZeroSize::new(("value", 3))),
        (NonZeroSize::new(4), NonZeroSize::new(("value", 4))),
    ];
    for i in 0..array.len() {
        let (key, value) = array[i].clone();
        assert_eq!(tree.set(key.clone(), value.clone()), None);
        assert_eq!(tree.get(&key), Some(&value));
        assert_eq!(tree.has(&key), true);
        assert_eq!(tree.size(), i + 1);
    }
    for (key, value) in array {
        assert_eq!(tree.get(&key), Some(&value));
        assert_eq!(tree.has(&key), true);
    }
}

#[test]
fn set_return_some() {
    let array = [
        (NonZeroSize::new(0), NonZeroSize::new(("value", 0))),
        (NonZeroSize::new(1), NonZeroSize::new(("value", 1))),
        (NonZeroSize::new(2), NonZeroSize::new(("value", 2))),
        (NonZeroSize::new(3), NonZeroSize::new(("value", 3))),
        (NonZeroSize::new(4), NonZeroSize::new(("value", 4))),
    ];
    let mut tree = BinarySearchTree::new();
    for (key, value) in array.clone() {
        tree.set(key, value);
    }
    assert_eq!(tree.size(), array.len());
    for (key, old_value) in array.clone() {
        let new_value = NonZeroSize::new(("new value", key.value + 1));
        assert_eq!(tree.set(key, new_value), Some(old_value));
        assert_eq!(tree.size(), array.len());
    }
    for (key, _) in array {
        let value = NonZeroSize::new(("new value", key.value + 1));
        assert_eq!(tree.get(&key), Some(&value));
    }
}

#[test]
fn get_return_none() {
    let array = [
        (NonZeroSize::new(0), NonZeroSize::new(("value", 0))),
        (NonZeroSize::new(1), NonZeroSize::new(("value", 1))),
        (NonZeroSize::new(2), NonZeroSize::new(("value", 2))),
    ];
    let mut tree = BinarySearchTree::new();
    for (key, value) in array.clone() {
        tree.set(key, value);
    }
    assert_eq!(tree.get(&NonZeroSize::new(3)), None);
    assert_eq!(tree.get(&NonZeroSize::new(4)), None);
}

#[test]
fn get_return_none_empty() {
    let tree = BinarySearchTree::<NonZeroSize, NonZeroSize>::new();
    assert_eq!(tree.get(&NonZeroSize::new(1)), None);
}

#[test]
fn get_mut() {
    let mut tree = helper::sample_small();
    let size = tree.size();
    for i in 0..size {
        let key = NonZeroSize::new(i);
        let item = tree.get_mut(&key).unwrap();
        assert_eq!(item, &NonZeroSize::new(("value", i)));
        item.value = ("new value", i + 1);
    }
    for i in 0..size {
        let key = NonZeroSize::new(i);
        let item = tree.get_mut(&key).unwrap();
        assert_eq!(item, &NonZeroSize::new(("new value", i + 1)));
    }
}

#[test]
fn get_mut_return_none() {
    let array = [
        (NonZeroSize::new(0), NonZeroSize::new(("value", 0))),
        (NonZeroSize::new(1), NonZeroSize::new(("value", 1))),
        (NonZeroSize::new(2), NonZeroSize::new(("value", 2))),
    ];
    let mut tree = BinarySearchTree::new();
    for (key, value) in array.clone() {
        tree.set(key, value);
    }
    assert_eq!(tree.get_mut(&NonZeroSize::new(3)), None);
    assert_eq!(tree.get_mut(&NonZeroSize::new(4)), None);
}

#[test]
fn get_mut_return_none_empty() {
    let mut tree = BinarySearchTree::<NonZeroSize, NonZeroSize>::new();
    assert_eq!(tree.get_mut(&NonZeroSize::new(1)), None);
}

#[test]
fn has_return_false() {
    let tree = helper::sample_small();
    let key = NonZeroSize::new(tree.size());
    assert_eq!(tree.has(&key), false);
}

#[test]
fn from_iter() {
    let array = [
        (NonZeroSize::new(0), NonZeroSize::new(("value", 0))),
        (NonZeroSize::new(1), NonZeroSize::new(("value", 1))),
        (NonZeroSize::new(2), NonZeroSize::new(("value", 2))),
    ];
    let tree = BinarySearchTree::from_iter(array.clone());
    assert_eq!(tree.size(), array.len());
    for (key, value) in array {
        assert_eq!(tree.get(&key), Some(&value));
    }
}

#[test]
fn from_array() {
    let array = [
        (NonZeroSize::new(0), NonZeroSize::new(("value", 0))),
        (NonZeroSize::new(1), NonZeroSize::new(("value", 1))),
        (NonZeroSize::new(2), NonZeroSize::new(("value", 2))),
    ];
    let tree = BinarySearchTree::from(array.clone());
    assert_eq!(tree.size(), array.len());
    for (key, value) in array {
        assert_eq!(tree.get(&key), Some(&value));
    }
}

#[test]
fn iter() {
    let tree = helper::sample_small();
    let size = tree.size();
    let mut pairs = tree
        .iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect::<Vector<_>>();
    let mut expected = (0..size)
        .map(|i| (NonZeroSize::new(i), NonZeroSize::new(("value", i))))
        .collect::<Vector<_>>();
    pairs.sort();
    expected.sort();
    assert_eq!(pairs, expected);
}

#[test]
fn iter_next_empty() {
    let tree = BinarySearchTree::<NonZeroSize, NonZeroSize>::new();
    assert_eq!(tree.iter().next(), None);
}

#[test]
fn keys() {
    let tree = helper::sample_small();
    let size = tree.size();
    let mut keys = tree.keys().map(|k| k.clone()).collect::<Vector<_>>();
    let mut expected = (0..size)
        .map(|i| NonZeroSize::new(i))
        .collect::<Vector<_>>();
    keys.sort();
    expected.sort();
    assert_eq!(keys, expected);
}

#[test]
fn keys_next_empty() {
    let tree = BinarySearchTree::<NonZeroSize, NonZeroSize>::new();
    assert_eq!(tree.keys().next(), None);
}

#[test]
fn values() {
    let tree = helper::sample_small();
    let size = tree.size();
    assert!(size > 0);
    let mut values = tree.values().map(|v| v.clone()).collect::<Vector<_>>();
    let mut expected = (0..size)
        .map(|i| NonZeroSize::new(("value", i)))
        .collect::<Vector<_>>();
    values.sort();
    expected.sort();
    assert_eq!(values, expected);
}

#[test]
fn values_next_empty() {
    let tree = BinarySearchTree::<NonZeroSize, NonZeroSize>::new();
    assert_eq!(tree.values().next(), None);
}

#[test]
fn min() {
    let tree = helper::sample_small();
    assert_eq!(tree.min(), &NonZeroSize::new(0));
}

#[test]
#[should_panic(expected = "expect: not empty tree")]
fn min_panic() {
    let tree = BinarySearchTree::<NonZeroSize, NonZeroSize>::new();
    tree.min();
}

#[test]
fn max() {
    let tree = helper::sample_small();
    assert_eq!(tree.max(), &NonZeroSize::new(tree.size() - 1));
}

#[test]
#[should_panic(expected = "expect: not empty tree")]
fn max_panic() {
    let tree = BinarySearchTree::<NonZeroSize, NonZeroSize>::new();
    tree.max();
}

#[test]
fn remove_return_some() {
    let mut tree = helper::sample_small();
    let size = tree.size();
    for i in 0..size {
        let key = NonZeroSize::new(i);
        let value = NonZeroSize::new(("value", i));
        assert_eq!(tree.remove(&key), Some(value));
        assert_eq!(tree.has(&key), false);
        assert_eq!(tree.size(), size - i - 1);
    }
}

#[test]
fn remove_return_none() {
    let mut tree = helper::sample_small();
    let size = tree.size();
    let key = NonZeroSize::new(size);
    assert_eq!(tree.remove(&key), None);
    assert_eq!(tree.size(), size);
}

#[test]
fn remove_return_none_empty() {
    let mut tree = BinarySearchTree::<NonZeroSize, NonZeroSize>::new();
    assert_eq!(tree.remove(&NonZeroSize::new(1)), None);
}

#[test]
fn equal_true() {
    let array = [
        (NonZeroSize::new(0), NonZeroSize::new(("value", 0))),
        (NonZeroSize::new(1), NonZeroSize::new(("value", 1))),
        (NonZeroSize::new(2), NonZeroSize::new(("value", 2))),
    ];
    let tree0 = BinarySearchTree::from(array.clone());
    let tree1 = BinarySearchTree::from(array);
    assert_eq!(tree0, tree1);
}

#[test]
fn equal_false_key() {
    let tree0 = BinarySearchTree::from([
        (NonZeroSize::new(0), NonZeroSize::new(("value", 0))),
        (NonZeroSize::new(1), NonZeroSize::new(("value", 1))),
        (NonZeroSize::new(2), NonZeroSize::new(("value", 2))),
    ]);
    let tree1 = BinarySearchTree::from([
        (NonZeroSize::new(0), NonZeroSize::new(("value", 0))),
        (NonZeroSize::new(1), NonZeroSize::new(("value", 1))),
        (NonZeroSize::new(9), NonZeroSize::new(("value", 3))),
    ]);
    assert_ne!(tree0, tree1);
}

#[test]
fn equal_false_value() {
    let tree0 = BinarySearchTree::from([
        (NonZeroSize::new(0), NonZeroSize::new(("value", 0))),
        (NonZeroSize::new(1), NonZeroSize::new(("value", 1))),
        (NonZeroSize::new(2), NonZeroSize::new(("value", 2))),
    ]);
    let tree1 = BinarySearchTree::from([
        (NonZeroSize::new(0), NonZeroSize::new(("value", 0))),
        (NonZeroSize::new(1), NonZeroSize::new(("value", 1))),
        (NonZeroSize::new(3), NonZeroSize::new(("value", 9))),
    ]);
    assert_ne!(tree0, tree1);
}

#[test]
fn equal_false_size() {
    let tree0 = BinarySearchTree::from([
        (NonZeroSize::new(0), NonZeroSize::new(("value", 0))),
        (NonZeroSize::new(1), NonZeroSize::new(("value", 1))),
        (NonZeroSize::new(2), NonZeroSize::new(("value", 2))),
    ]);
    let tree1 = BinarySearchTree::from([
        (NonZeroSize::new(0), NonZeroSize::new(("value", 0))),
        (NonZeroSize::new(1), NonZeroSize::new(("value", 1))),
    ]);
    assert_ne!(tree0, tree1);
}

#[test]
fn equal_true_empty() {
    let tree0 = BinarySearchTree::<NonZeroSize, NonZeroSize>::new();
    let tree1 = BinarySearchTree::<NonZeroSize, NonZeroSize>::new();
    assert_eq!(tree0, tree1);
}

#[test]
fn clone() {
    let tree0 = helper::sample_small();
    let tree1 = tree0.clone();
    assert_eq!(tree0, tree1);
}

#[test]
fn clone_empty() {
    let tree0 = BinarySearchTree::<NonZeroSize, NonZeroSize>::new();
    let tree1 = tree0.clone();
    assert_eq!(tree0, tree1);
}

#[test]
fn clear() {
    let mut tree = helper::sample_small();
    tree.clear();
    assert_eq!(tree.size(), 0);
}

// This test does nothing but creating a non empty container to trigger memory
// release process. The test can not work alone, it requries an external tool
// such as Valgrind to diagnose memory issues.
//
// Warn: The test maybe still passed even memory release process has issues.
#[test]
fn drop() {
    let _ = helper::sample_small();
}

#[test]
fn sample_must_not_null() {
    assert!(helper::sample_small().size() > 0);
}
