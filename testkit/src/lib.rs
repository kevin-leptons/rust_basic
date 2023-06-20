//! For supporting tests of very basic and foundation data types. This crate
//! must not depend on other crates. It depends on [std::alloc], [std::mem],
//! [std::ptr], [rust_basic] that's all.

mod non_zero_size;
mod zero_size;

pub use non_zero_size::NonZeroSize;
pub use zero_size::ZeroSize;
