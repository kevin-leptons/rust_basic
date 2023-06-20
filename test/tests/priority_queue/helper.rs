use rust_basic::PriorityQueue;
use testkit::NonZeroSize;

pub(super) fn non_zero_size_type() -> PriorityQueue<NonZeroSize> {
    let mut queue = PriorityQueue::new();
    for i in 0..10000 {
        queue.push(NonZeroSize::new(i));
    }
    return queue;
}
