use rust_basic::Vector;

use testkit::{NonZeroSize, ZeroSize};

pub fn zero_size_type() -> Vector<ZeroSize> {
    let mut vector = Vector::new();
    for _ in 0..100000 {
        vector.push_back(ZeroSize::new());
    }
    return vector;
}

pub fn non_zero_size_type() -> Vector<NonZeroSize> {
    let mut vector = Vector::new();
    for i in 0..100000 {
        vector.push_back(NonZeroSize::new(i));
    }
    return vector;
}
