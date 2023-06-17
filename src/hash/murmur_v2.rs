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
/// let h0 = murmur_v2_64(&[1, 3, 5, 7], seed);
/// let h1 = murmur_v2_64(&[1, 3, 5, 7], seed);
/// let h2 = murmur_v2_64(&[2, 4, 6, 8], seed);
/// assert_eq!(h0, h1);
/// assert_ne!(h1, h2);
pub fn murmur_v2_64(value: &[u8], seed: u64) -> u64 {
    let len = value.len() as u64;
    let m = 0xc6a4a7935bd1e995u64;
    let r = 47u64;
    let mut h = Wrapping(m);
    h.mul_assign(len);
    h = Wrapping(h.0 ^ seed);
    let len64 = value.len() / 8;
    let p = value.as_ptr() as *const u64;
    for i in 0..len64 {
        let mut k = Wrapping(unsafe { ptr::read_unaligned(p.add(i)) });
        k.mul_assign(m);
        k = Wrapping(k.0 ^ k.0 >> r);
        k.mul_assign(m);
        h = Wrapping(h.0 ^ k.0);
        h.mul_assign(m);
    }
    let remains = value.len() % 8;
    if remains > 0 {
        let last =
            unsafe { ptr::read_unaligned(p.add(len64) as *const [u8; 8]) };
        h = match value.len() & 7 {
            7 => Wrapping(h.0 ^ (last[6] as u64) << 48),
            6 => Wrapping(h.0 ^ (last[5] as u64) << 40),
            5 => Wrapping(h.0 ^ (last[4] as u64) << 32),
            4 => Wrapping(h.0 ^ (last[3] as u64) << 24),
            3 => Wrapping(h.0 ^ (last[2] as u64) << 16),
            2 => Wrapping(h.0 ^ (last[1] as u64) << 8),
            1 => Wrapping(h.0 ^ (last[0] as u64) << 48),
            _ => {
                h.mul_assign(m);
                h
            }
        };
    }
    h = Wrapping(h.0 ^ h.0 >> r);
    h.mul_assign(m);
    h = Wrapping(h.0 ^ h.0 >> r);
    return h.0;
}
