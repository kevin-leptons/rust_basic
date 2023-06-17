use rust_basic::hash;

#[test]
fn murmur_v3_32() {
    let key0: [u8; 0] = [];
    let key1: [u8; 4] = [1, 2, 3, 5];
    let key2: [u8; 4] = key1.clone();
    let key3: [u8; 5] = [1, 2, 3, 5, 8];
    let seed = 981u32;
    let hash0 = hash::murmur_v3_32(&key0, seed);
    let hash1 = hash::murmur_v3_32(&key1, seed);
    let hash2 = hash::murmur_v3_32(&key2, seed);
    let hash3 = hash::murmur_v3_32(&key3, seed);
    assert_eq!(hash1, hash2);
    assert_ne!(hash0, hash1);
    assert_ne!(hash1, hash3);
}

#[test]
fn murmur_v2_64() {
    for seed in 0..1024 {
        let key0: [u8; 0] = [];
        let key1: [u8; 64] = key(seed);
        let key2: [u8; 64] = key(seed);
        let key3: [u8; 65] = key(seed + 3);
        let key4: [u8; 1027] = key(seed + 4);
        let hash0 = hash::murmur_v2_64(&key0, seed as u64);
        let hash1 = hash::murmur_v2_64(&key1, seed as u64);
        let hash2 = hash::murmur_v2_64(&key2, seed as u64);
        let hash3 = hash::murmur_v2_64(&key3, seed as u64);
        let hash4 = hash::murmur_v2_64(&key4, seed as u64);
        assert_eq!(hash1, hash2);
        assert_ne!(hash0, hash1);
        assert_ne!(hash0, hash2);
        assert_ne!(hash0, hash3);
        assert_ne!(hash0, hash4);
        assert_ne!(hash1, hash3);
        assert_ne!(hash1, hash4);
        assert_ne!(hash2, hash3);
        assert_ne!(hash2, hash4);
        assert_ne!(hash3, hash4);
    }
}

fn key<const N: usize>(mut seed: usize) -> [u8; N] {
    let mut d = [0u8; N];
    for i in 0..d.len() {
        seed = seed * i;
        d[i] = (seed / 0xff) as u8;
    }
    return d;
}
