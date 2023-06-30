use crate::hash::murmur_v2_64;

/// Specify the type that can be hashed.
///
/// There are types that is pre-implemented: primitive integers, [str] and
/// [String].
///
/// # Example
///
/// ```
/// use rust_basic::{Hashable, murmur_v2_64};
///
/// struct Digit {
///     value: u8
/// }
///
/// impl Digit {
///     pub fn new(value: u8) -> Self {
///         assert!(value <= 9, "expect: a digit");
///         return Self { value };
///    }
/// }
///
/// impl Hashable for Digit {
///     fn hash(&self) -> u64 {
///         let seed = 0x1357;
///         return murmur_v2_64(&self.value.to_ne_bytes(), seed);
///     }
/// }
///
/// let hash0 = Digit::new(1).hash();
/// let hash1 = Digit::new(1).hash();
/// let hash2 = Digit::new(2).hash();
/// assert_eq!(hash0, hash1);
/// assert_ne!(hash0, hash2);
pub trait Hashable {
    /// The requirement is: If `k1 == k2` then `hash(k1) == hash(k2)`. The
    /// result should be distributed as much as possible.
    fn hash(&self) -> u64;
}

macro_rules! implement_hashable_integer {
    ($T: ty) => {
        impl Hashable for $T {
            fn hash(&self) -> u64 {
                return murmur_v2_64(&self.to_ne_bytes(), 0xf1f1);
            }
        }
    };
}

macro_rules! implement_hashable_string {
    ($T: ty) => {
        impl Hashable for $T {
            fn hash(&self) -> u64 {
                return murmur_v2_64(&self.as_bytes(), 0xe2e2);
            }
        }
    };
}

implement_hashable_integer!(u8);
implement_hashable_integer!(u16);
implement_hashable_integer!(u32);
implement_hashable_integer!(u64);
implement_hashable_integer!(u128);
implement_hashable_integer!(usize);
implement_hashable_integer!(i8);
implement_hashable_integer!(i16);
implement_hashable_integer!(i32);
implement_hashable_integer!(i64);
implement_hashable_integer!(i128);
implement_hashable_integer!(isize);
implement_hashable_string!(str);
implement_hashable_string!(String);
