use std::assert_eq;

use rust_basic::murmur_v2_64;

#[test]
fn empty_data_same_seed() {
    let data0: [u8; 0] = [];
    let data1 = data0.clone();
    let seed = 0xfd;
    let hash0 = murmur_v2_64(&data0, seed);
    let hash1 = murmur_v2_64(&data1, seed);
    assert_eq!(hash0, hash1);
}

#[test]
fn empty_data_different_seed() {
    let data0: [u8; 0] = [];
    let data1 = data0.clone();
    let seed0 = 0xfd;
    let seed1 = 0xfe;
    let hash0 = murmur_v2_64(&data0, seed0);
    let hash1 = murmur_v2_64(&data1, seed1);
    assert_ne!(hash0, hash1);
}

#[test]
fn same_data_same_seed() {
    let data0: [u8; 5] = [0, 1, 2, 3, 4];
    let data1 = data0.clone();
    let seed = 0x9ab;
    let hash0 = murmur_v2_64(&data0, seed);
    let hash1 = murmur_v2_64(&data1, seed);
    assert_eq!(hash0, hash1);
}

#[test]
fn same_data_different_seed() {
    let data0: [u8; 5] = [0, 1, 2, 3, 4];
    let data1 = data0.clone();
    let seed0 = 0x9ab;
    let seed1 = 0x343;
    let hash0 = murmur_v2_64(&data0, seed0);
    let hash1 = murmur_v2_64(&data1, seed1);
    assert_ne!(hash0, hash1);
}

#[test]
fn different_data_same_seed() {
    let data0: [u8; 5] = [0, 1, 2, 3, 4];
    let data1: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let seed = 0x74;
    let hash0 = murmur_v2_64(&data0, seed);
    let hash1 = murmur_v2_64(&data1, seed);
    assert_ne!(hash0, hash1);
}

#[test]
fn different_data_different_seed() {
    let data0: [u8; 5] = [0, 1, 2, 3, 4];
    let data1: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let seed0 = 0x911;
    let seed1 = 0x912;
    let hash0 = murmur_v2_64(&data0, seed0);
    let hash1 = murmur_v2_64(&data1, seed1);
    assert_ne!(hash0, hash1);
}
