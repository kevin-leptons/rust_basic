use rust_basic::hash::{self, HashKey};

#[test]
fn murmur_v3_32() {
    let key0: [u8; 0] = [];
    let key1: [u8; 4] = [1, 2, 3, 5];
    let key2: [u8; 4] = key1.clone();
    let key3: [u8; 5] = [1, 2, 3, 5, 8];
    let seed: u32 = 981;
    let hash0 = hash::murmur_v3_32(&key0, seed);
    let hash1 = hash::murmur_v3_32(&key1, seed);
    let hash2 = hash::murmur_v3_32(&key2, seed);
    let hash3 = hash::murmur_v3_32(&key3, seed);
    assert_eq!(hash1, hash2);
    assert_ne!(hash0, hash1);
    assert_ne!(hash1, hash3);
}

#[test]
fn string_hash_key() {
    let v1 = String::from("");
    let v2 = String::from("key: 1001");
    let v3 = String::from("key: 1001");
    let v4 = String::from("foo bar: 3");
    let h1 = v1.hash_key();
    let h2 = v2.hash_key();
    let h3 = v3.hash_key();
    let h4 = v4.hash_key();
    assert_eq!(h2, h3);
    assert_ne!(h2, h1);
    assert_ne!(h2, h4);
}
