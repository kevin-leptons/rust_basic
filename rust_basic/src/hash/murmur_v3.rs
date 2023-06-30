use std::num::Wrapping;
use std::ops::MulAssign;
use std::ptr;

/// `entry` Compute a hash by MurmurHash version 3 32bits.
///
/// # Example
///
/// ```
/// use rust_basic::murmur_v3_32;
///
/// let seed = 0x99;
/// let hash0 = murmur_v3_32(&[1, 3, 5, 7], seed);
/// let hash1 = murmur_v3_32(&[1, 3, 5, 7], seed);
/// let hash2 = murmur_v3_32(&[2, 4, 6, 8], seed);
/// assert_eq!(hash0, hash1);
/// assert_ne!(hash1, hash2);
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
