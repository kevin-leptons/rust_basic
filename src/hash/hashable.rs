use crate::hash::murmur_v2_64;

/// Specify the type that can be hashed.
///
/// # Example
///
/// ```
/// use rust_basic::{murmur_v2_64, Hashable};
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
///         return murmur_v2_64(&self.value.to_be_bytes(), 0x1357);
///     }
/// }
///
/// let h0 = Digit::new(1).hash();
/// let h1 = Digit::new(1).hash();
/// let h2 = Digit::new(2).hash();
/// assert_eq!(h0, h1);
/// assert_ne!(h1, h2);
pub trait Hashable {
    /// The requirement is: If `k1 == k2` then `hash(k1) == hash(k2)`. The
    /// result should be distributed as much as possible.
    fn hash(&self) -> u64;
}

impl Hashable for u8 {
    fn hash(&self) -> u64 {
        let input = self.to_be_bytes();
        return murmur_v2_64(&input, 0x0587);
    }
}

impl Hashable for u32 {
    fn hash(&self) -> u64 {
        let input = self.to_be_bytes();
        return murmur_v2_64(&input, 0x1f2f);
    }
}

impl Hashable for i32 {
    fn hash(&self) -> u64 {
        let input = self.to_be_bytes();
        return murmur_v2_64(&input, 0x1f2f);
    }
}

impl Hashable for u64 {
    fn hash(&self) -> u64 {
        let input = self.to_be_bytes();
        return murmur_v2_64(&input, 0x1f2f);
    }
}

impl Hashable for u128 {
    fn hash(&self) -> u64 {
        let input = self.to_be_bytes();
        return murmur_v2_64(&input, 0x1f2f);
    }
}

impl Hashable for usize {
    fn hash(&self) -> u64 {
        let input = self.to_be_bytes();
        return murmur_v2_64(&input, 0x1f2f);
    }
}

impl Hashable for String {
    fn hash(&self) -> u64 {
        let input = self.as_bytes();
        return murmur_v2_64(input, 0x012f0e);
    }
}
