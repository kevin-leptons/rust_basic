use crate::sample;
use rust_basic::queue::Queue;
use testkit::NonZeroSize;

#[test]
fn new() {
    let queue = Queue::<NonZeroSize>::new();
    assert_eq!(queue.size(), 0);
}

#[test]
#[should_panic(expected = "expect: not empty queue")]
fn top_panic() {
    let queue = Queue::<NonZeroSize>::new();
    queue.top();
}

#[test]
#[should_panic(expected = "expect: not empty queue")]
fn pop_panic() {
    let mut queue = Queue::<NonZeroSize>::new();
    queue.pop();
}

#[test]
fn push() {
    let mut queue = Queue::new();
    let top = NonZeroSize::new(0);
    let size = 100;
    for i in 0..size {
        queue.push(NonZeroSize::new(i));
        assert_eq!(queue.top(), &top);
        assert_eq!(queue.size(), i + 1);
    }
}

#[test]
fn pop_all() {
    let mut queue = Queue::new();
    let mut next_identity = 0;
    let mut top_identity = 0;
    let mut size = 0;
    let round = 100;
    for i in 0..round {
        let push_size = 10 * i;
        for _ in 0..push_size {
            let item = NonZeroSize::new(next_identity);
            queue.push(item);
            assert_eq!(queue.top(), &NonZeroSize::new(top_identity));
            assert_eq!(queue.size(), size + 1);
            size += 1;
            next_identity += 1;
        }
        for _ in 0..push_size {
            let top = NonZeroSize::new(top_identity);
            assert_eq!(queue.top(), &top);
            assert_eq!(queue.pop(), top);
            assert_eq!(queue.size(), size - 1);
            size -= 1;
            top_identity += 1;
        }
    }
    assert_eq!(size, 0);
    assert_eq!(queue.size(), 0);
}

#[test]
fn pop_half() {
    let mut queue = Queue::new();
    let mut next_identity = 0;
    let mut top_identity = 0;
    let mut size = 0;
    let round = 100;
    for i in 0..round {
        let push_size = 10 * i;
        for _ in 0..push_size {
            let item = NonZeroSize::new(next_identity);
            queue.push(item);
            assert_eq!(queue.top(), &NonZeroSize::new(top_identity));
            assert_eq!(queue.size(), size + 1);
            size += 1;
            next_identity += 1;
        }
        for _ in 0..(push_size / 2) {
            let top = NonZeroSize::new(top_identity);
            assert_eq!(queue.top(), &top);
            assert_eq!(queue.pop(), top);
            assert_eq!(queue.size(), size - 1);
            size -= 1;
            top_identity += 1;
        }
    }
    assert!(size > 0);
    assert_eq!(queue.size(), size);
    for _ in 0..size {
        let top = NonZeroSize::new(top_identity);
        assert_eq!(queue.top(), &top);
        assert_eq!(queue.pop(), top);
        assert_eq!(queue.size(), size - 1);
        size -= 1;
        top_identity += 1;
    }
    assert_eq!(queue.size(), 0);
}

#[test]
fn from_iter() {
    let array = [
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
        NonZeroSize::new(3),
        NonZeroSize::new(4),
        NonZeroSize::new(5),
    ];
    let mut queue = Queue::from_iter(array.clone());
    for i in 0..array.len() {
        assert_eq!(queue.size(), array.len() - i);
        assert_eq!(queue.top(), &array[i]);
        assert_eq!(queue.pop(), array[i]);
    }
    assert_eq!(queue.size(), 0);
}

#[test]
fn from_array() {
    let array = [
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
        NonZeroSize::new(3),
        NonZeroSize::new(4),
        NonZeroSize::new(5),
    ];
    let mut queue = Queue::from(array.clone());
    for i in 0..array.len() {
        assert_eq!(queue.size(), array.len() - i);
        assert_eq!(queue.top(), &array[i]);
        assert_eq!(queue.pop(), array[i]);
    }
    assert_eq!(queue.size(), 0);
}

#[test]
fn iter() {
    let queue = sample::non_zero_size_type();
    let size = queue.size();
    let mut count = 0;
    for item in queue.iter() {
        assert_eq!(item, &NonZeroSize::new(count));
        count += 1;
    }
    assert_eq!(count, size);
    assert_eq!(queue.size(), size);
}

#[test]
fn iter_next_return_none() {
    let stack = Queue::<NonZeroSize>::new();
    assert_eq!(stack.iter().next(), None);
}

#[test]
fn index() {
    let queue = sample::non_zero_size_type();
    for i in 0..queue.size() {
        assert_eq!(queue[i], NonZeroSize::new(i));
    }
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn index_panic() {
    let queue = sample::non_zero_size_type();
    let _ = &queue[queue.size()];
}

#[test]
fn clone() {
    let mut queue1 = sample::non_zero_size_type();
    let mut queue2 = queue1.clone();
    for _ in 0..queue1.size() {
        assert_eq!(queue1.size(), queue2.size());
        assert_eq!(queue1.top(), queue2.top());
        assert_eq!(queue1.pop(), queue2.pop());
    }
    assert_eq!(queue1.size(), 0);
    assert_eq!(queue2.size(), 0);
}

#[test]
fn clear() {
    let mut queue = sample::non_zero_size_type();
    queue.clear();
    assert_eq!(queue.size(), 0);
}

/// This test does nothing but creating a non empty container to trigger memory
/// release process. The test can not work alone, it requries an external tool
/// such as Valgrind to diagnose memory issues.
///
/// Warn: The test maybe still passed even memory release process has issues.
#[test]
fn drop() {
    let _ = sample::non_zero_size_type();
}

#[test]
fn sample_must_not_empty() {
    let stack = sample::non_zero_size_type();
    assert!(stack.size() > 0);
}
