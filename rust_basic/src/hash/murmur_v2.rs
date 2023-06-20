use std::num::Wrapping;
use std::ops::MulAssign;
use std::ptr;

/// `entry` Compute a hash by MurmurHash version 2 64bits.
///
/// # Example
///
/// ```
/// use rust_basic::murmur_v2_64;
///
/// let seed = 0x99;
/// let hash0 = murmur_v2_64(&[1, 3, 5, 7], seed);
/// let hash1 = murmur_v2_64(&[1, 3, 5, 7], seed);
/// let hash2 = murmur_v2_64(&[2, 4, 6, 8], seed);
/// assert_eq!(hash0, hash1);
/// assert_ne!(hash1, hash2);
pub fn murmur_v2_64(value: &[u8], seed: u64) -> u64 {
    let len = value.len() as u64;
    let m = 0xc6a4a7935bd1e995u64;
    let r = 47u64;
    let mut h = Wrapping(m);
    h.mul_assign(len);
    h = Wrapping(h.0 ^ seed);
    let block64 = value.len() / 8;
    let p = value.as_ptr() as *const u64;
    for i in 0..block64 {
        let mut k = Wrapping(unsafe { ptr::read_unaligned(p.add(i)) });
        k.mul_assign(m);
        k = Wrapping(k.0 ^ (k.0 >> r));
        k.mul_assign(m);
        h = Wrapping(h.0 ^ k.0);
        h.mul_assign(m);
    }
    let remain_bytes = value.len() & 7;
    if remain_bytes > 0 {
        let last = unsafe { ptr::read_unaligned(p.add(block64)).to_ne_bytes() };
        for i in (1..remain_bytes + 1).rev() {
            let shift = 8 * (i - 1);
            let byte = last[i - 1] as u64;
            h = Wrapping(h.0 ^ (byte << shift));
        }
        h.mul_assign(m);
    }
    h = Wrapping(h.0 ^ (h.0 >> r));
    h.mul_assign(m);
    h = Wrapping(h.0 ^ (h.0 >> r));
    return h.0;
}
