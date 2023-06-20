use rust_basic::Stack;
use testkit::{NonZeroSize, ZeroSize};

pub fn zero_size_type() -> Stack<ZeroSize> {
    let mut stack = Stack::new();
    for _ in 0..100000 {
        stack.push(ZeroSize::new());
    }
    return stack;
}

pub fn non_zero_size_type() -> Stack<NonZeroSize> {
    let mut stack = Stack::new();
    for i in 0..100000 {
        let item = NonZeroSize::new(i);
        stack.push(item);
    }
    return stack;
}
