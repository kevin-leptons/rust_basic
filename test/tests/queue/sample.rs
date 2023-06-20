use rust_basic::Queue;
use testkit::{NonZeroSize, ZeroSize};

pub fn zero_size_type() -> Queue<ZeroSize> {
    let mut queue = Queue::new();
    for _ in 0..10000 {
        queue.push(ZeroSize::new());
    }
    return queue;
}

pub fn non_zero_size_type() -> Queue<NonZeroSize> {
    let mut queue = Queue::new();
    for i in 0..10000 {
        queue.push(NonZeroSize::new(i));
    }
    return queue;
}
