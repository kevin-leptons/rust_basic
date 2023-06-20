use crate::sample;
use rust_basic::Stack;
use testkit::NonZeroSize;

#[test]
fn new() {
    let stack = Stack::<NonZeroSize>::new();
    assert_eq!(stack.size(), 0);
}

#[test]
#[should_panic(expected = "expect: not empty stack")]
fn top_panic() {
    let s = Stack::<NonZeroSize>::new();
    s.top();
}

#[test]
#[should_panic(expected = "expect: not empty stack")]
fn pop_panic() {
    let mut s = Stack::<NonZeroSize>::new();
    s.pop();
}

#[test]
fn push() {
    let mut stack = Stack::new();
    let size = 1000;
    for i in 0..size {
        let item = NonZeroSize::new(i);
        stack.push(item.clone());
        assert_eq!(stack.top(), &item);
        assert_eq!(stack.size(), i + 1);
    }
}

#[test]
fn pop_all() {
    let mut stack = Stack::new();
    let mut size = 0;
    let round = 100;
    for i in 0..round {
        let push_size = 10 * i;
        for k in 0..push_size {
            let item = NonZeroSize::new((i, k));
            stack.push(item.clone());
            assert_eq!(stack.top(), &item);
            assert_eq!(stack.size(), size + 1);
            size += 1;
        }
        for k in 0..push_size {
            let top = NonZeroSize::new((i, push_size - k - 1));
            assert_eq!(stack.top(), &top);
            assert_eq!(stack.pop(), top);
            assert_eq!(stack.size(), size - 1);
            size -= 1;
        }
    }
    assert_eq!(size, 0);
    assert_eq!(stack.size(), 0);
}

#[test]
fn pop_half() {
    let mut stack = Stack::new();
    let mut size: usize = 0;
    let round = 100;
    for i in 0..round {
        let push_size = 10 * i;
        for k in 0..push_size {
            let item = NonZeroSize::new((i, k));
            stack.push(item.clone());
            assert_eq!(stack.top(), &item);
            assert_eq!(stack.size(), size + 1);
            size += 1;
        }
        let pop_size = push_size / 2;
        for k in 0..pop_size {
            let top = NonZeroSize::new((i, push_size - k - 1));
            assert_eq!(stack.top(), &top);
            assert_eq!(stack.pop(), top);
            assert_eq!(stack.size(), size - 1);
            size -= 1;
        }
    }
    assert!(size > 0);
    assert_eq!(stack.size(), size);
    for _ in 0..size {
        stack.pop();
        size -= 1;
        assert_eq!(stack.size(), size);
    }
    assert_eq!(stack.size(), 0);
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
    let mut stack = Stack::from_iter(array.clone());
    for i in 0..array.len() {
        let top = &array[array.len() - i - 1];
        assert_eq!(stack.top(), top);
        assert_eq!(stack.pop(), *top);
        assert_eq!(stack.size(), array.len() - i - 1);
    }
    assert_eq!(stack.size(), 0);
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
    let mut stack = Stack::from(array.clone());
    for i in 0..array.len() {
        let top = &array[array.len() - i - 1];
        assert_eq!(stack.top(), top);
        assert_eq!(stack.pop(), *top);
        assert_eq!(stack.size(), array.len() - i - 1);
    }
    assert_eq!(stack.size(), 0);
}

#[test]
fn index() {
    let stack = sample::non_zero_size_type();
    let size = stack.size();
    for i in 0..size {
        assert_eq!(stack[i], NonZeroSize::new(i));
    }
    assert_eq!(stack.size(), size);
    assert_eq!(stack.top(), &NonZeroSize::new(size - 1));
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn index_panic() {
    let stack = sample::non_zero_size_type();
    let _ = &stack[stack.size()];
}

#[test]
fn iter() {
    let stack = sample::non_zero_size_type();
    let size = stack.size();
    let mut count = 0;
    for item in stack.iter() {
        assert_eq!(item, &NonZeroSize::new(count));
        count += 1;
    }
    assert_eq!(count, size);
    assert_eq!(stack.size(), size);
    assert_eq!(stack.top(), &NonZeroSize::new(size - 1));
}

#[test]
fn iter_next_return_none() {
    let stack = Stack::<NonZeroSize>::new();
    assert_eq!(stack.iter().next(), None);
}

#[test]
fn clone() {
    let mut stack0 = sample::non_zero_size_type();
    let mut stack1 = stack0.clone();
    for _ in 0..stack0.size() {
        assert_eq!(stack0.size(), stack1.size());
        assert_eq!(stack0.top(), stack1.top());
        assert_eq!(stack0.pop(), stack1.pop());
    }
    assert_eq!(stack0.size(), 0);
    assert_eq!(stack1.size(), 0);
}

#[test]
fn clear() {
    let mut stack = sample::non_zero_size_type();
    stack.clear();
    assert_eq!(stack.size(), 0);
}

// This test does nothing but creating a non empty container to trigger memory
// release process. The test can not work alone, it requries an external tool
// such as Valgrind to diagnose memory issues.
//
// Warn: The test maybe still passed even memory release process has issues.
#[test]
fn drop() {
    let _ = sample::non_zero_size_type();
}

#[test]
fn sample_must_not_empty() {
    let stack = sample::non_zero_size_type();
    assert!(stack.size() > 0);
}
