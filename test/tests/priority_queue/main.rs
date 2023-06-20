mod helper;

use rust_basic::{PriorityQueue, Vector};
use testkit::NonZeroSize;

#[test]
fn new() {
    let queue = PriorityQueue::<NonZeroSize>::new();
    assert_eq!(queue.size(), 0);
}

#[test]
fn push() {
    let array = [
        NonZeroSize::new(1),
        NonZeroSize::new(2),
        NonZeroSize::new(3),
        NonZeroSize::new(4),
    ];
    let mut queue = PriorityQueue::new();
    for i in 0..array.len() {
        queue.push(array[i].clone());
        assert_eq!(queue.top(), &array[i]);
        assert_eq!(queue.size(), i + 1);
    }
}

#[test]
fn pop() {
    let array = [
        NonZeroSize::new(1),
        NonZeroSize::new(2),
        NonZeroSize::new(3),
        NonZeroSize::new(4),
    ];
    let mut queue = PriorityQueue::new();
    for item in array.clone() {
        queue.push(item);
    }
    for i in 0..array.len() {
        assert_eq!(queue.size(), array.len() - i);
        assert_eq!(queue.pop(), array[array.len() - i - 1]);
    }
    assert_eq!(queue.size(), 0);
}

#[test]
fn from_iter() {
    let array = [
        NonZeroSize::new(1),
        NonZeroSize::new(2),
        NonZeroSize::new(3),
        NonZeroSize::new(4),
    ];
    let mut queue = PriorityQueue::from_iter(array.clone());
    for i in 0..array.len() {
        assert_eq!(queue.size(), array.len() - i);
        assert_eq!(queue.pop(), NonZeroSize::new(array.len() - i));
    }
    assert_eq!(queue.size(), 0);
}

#[test]
fn from_array() {
    let array = [
        NonZeroSize::new(1),
        NonZeroSize::new(2),
        NonZeroSize::new(3),
        NonZeroSize::new(4),
    ];
    let mut queue = PriorityQueue::from(array.clone());
    for i in 0..array.len() {
        assert_eq!(queue.size(), array.len() - i);
        assert_eq!(queue.pop(), NonZeroSize::new(array.len() - i));
    }
    assert_eq!(queue.size(), 0);
}

#[test]
fn iter() {
    let array = [
        NonZeroSize::new(1),
        NonZeroSize::new(2),
        NonZeroSize::new(3),
        NonZeroSize::new(4),
        NonZeroSize::new(5),
    ];
    let queue = PriorityQueue::from(array.clone());
    let mut actual =
        queue.iter().map(|item| item.clone()).collect::<Vector<_>>();
    actual.sort();
    assert_eq!(actual, Vector::from(array));
}

#[test]
fn iter_empty() {
    let queue = PriorityQueue::<NonZeroSize>::new();
    assert_eq!(queue.iter().next(), None);
}

#[test]
fn clone() {
    let mut queue0 = helper::non_zero_size_type();
    let mut queue1 = queue0.clone();
    for _ in 0..queue0.size() {
        assert_eq!(queue0.size(), queue1.size());
        assert_eq!(queue0.pop(), queue1.pop());
    }
    assert_eq!(queue0.size(), 0);
    assert_eq!(queue1.size(), 0);
}

#[test]
fn clear() {
    let mut queue = helper::non_zero_size_type();
    assert!(queue.size() > 0);
    queue.clear();
    assert_eq!(queue.size(), 0);
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
    let queue = helper::non_zero_size_type();
    assert!(queue.size() > 0);
}
