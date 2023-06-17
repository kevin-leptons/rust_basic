//! Hashing - mapping data into a value that is fixed in size and nearly unique.
//!
//! This document - module level document does not contain other descriptions or
//! examples. To learn more about how to use, refer to specific Entry APIs which
//! is labeled `entry` in the following sections.
//!
//! If there is the first time, [Package Document](crate) is a good way to learn
//! more about how to explore APIs and access to [Entry APIs
//! List](crate#structs).

mod murmur_v2;
mod murmur_v3;
mod hashable;

pub use murmur_v2::murmur_v2_64;
pub use murmur_v3::murmur_v3_32;
pub use hashable::Hashable;
