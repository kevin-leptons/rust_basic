//! Hashing - mapping data into a value that is fixed in size and nearly unique.
//!
//! This document - module level document does not contain other descriptions or
//! examples. To learn more about how to use, refer to specific Entry APIs which
//! is labeled `entry` in the following sections.
//!
//! If there is the first time, [Package Document](crate) is a good way to learn
//! more about how to explore APIs and access to [Entry APIs
//! List](crate#structs).

use std::ptr;
use std::{num::Wrapping, ops::MulAssign};

/// For the type that can be hashed and use as a key for hash map.
pub trait HashKey: Eq {
    /// Requirements for the implementation:
    ///  * If `k1 == k2` then `hash_key(k1) == hash_key(k2)`.
    ///  * If `k1 != k2` then `hash_key(k1) != hash_key(k2)` in almost cases.
    fn hash_key(&self) -> u32;
}

/// `entry` Compute a hash by [MurmurHash version 3
///   32bits](https://en.wikipedia.org/wiki/MurmurHash#MurmurHash3).
///
/// # Example
///
/// ```
/// use rust_basic::murmur_v3_32;
///
/// let seed = 0x99;
/// let h0 = murmur_v3_32(&[1, 3, 5, 7], seed);
/// let h1 = murmur_v3_32(&[1, 3, 5, 7], seed);
/// let h2 = murmur_v3_32(&[2, 4, 6, 8], seed);
/// assert_eq!(h0, h1);
/// assert_ne!(h1, h2);
pub fn murmur_v3_32(value: &[u8], seed: u32) -> u32 {
    let len32 = value.len() / 4;
    let c1 = 0xcc9e2d51u32;
    let c2 = 0x1b873593u32;
    let r1 = 15u32;
    let r2 = 13u32;
    let m = Wrapping(5u32);
    let n = Wrapping(0xe6546b64u32);
    let mut h = Wrapping(seed);
    let p = value.as_ptr();
    for i in 0..len32 {
        let mut k = Wrapping(unsafe {
            ptr::read_unaligned(p.add(4 * i) as *const u32)
        });
        k.mul_assign(c1);
        k = Wrapping(k.0.rotate_left(r1));
        k.mul_assign(c2);
        h = Wrapping(h.0 ^ k.0);
        h = Wrapping(h.0.rotate_left(r2));
        h = h * m + n;
    }
    let remain_bytes = value.len() % 4;
    if remain_bytes > 0 {
        let mut remain = 0u32;
        for i in value.len() - 1..4 * len32 {
            remain = remain << 8;
            remain = remain | value[i] as u32;
        }
        let mut tail = Wrapping(remain.to_le());
        tail.mul_assign(c1);
        tail = Wrapping(tail.0.rotate_left(r1));
        tail.mul_assign(c2);
        h = Wrapping(h.0 ^ tail.0);
    }
    h = Wrapping(h.0 ^ value.len() as u32);
    h = Wrapping(h.0 ^ (h.0 >> 16));
    h.mul_assign(0x85ebca6bu32);
    h = Wrapping(h.0 ^ (h.0 >> 13));
    h.mul_assign(0xc2b2ae35u32);
    h = Wrapping(h.0 ^ (h.0 >> 16));
    return h.0;
}

impl HashKey for u8 {
    fn hash_key(&self) -> u32 {
        let input = self.to_be_bytes();
        return murmur_v3_32(&input, 0x0587);
    }
}

impl HashKey for u32 {
    fn hash_key(&self) -> u32 {
        let input = self.to_be_bytes();
        return murmur_v3_32(&input, 0x1f2f);
    }
}

impl HashKey for u64 {
    fn hash_key(&self) -> u32 {
        let input = self.to_be_bytes();
        return murmur_v3_32(&input, 0x1f2f);
    }
}

impl HashKey for u128 {
    fn hash_key(&self) -> u32 {
        let input = self.to_be_bytes();
        return murmur_v3_32(&input, 0x1f2f);
    }
}

impl HashKey for usize {
    fn hash_key(&self) -> u32 {
        let input = self.to_be_bytes();
        return murmur_v3_32(&input, 0x1f2f);
    }
}

impl HashKey for String {
    fn hash_key(&self) -> u32 {
        let input = self.as_bytes();
        return murmur_v3_32(input, 0x012f0e);
    }
}
