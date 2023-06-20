use crate::helper::test_integer_hash;
use rust_basic::Hashable;
use std::{assert_eq, assert_ne};

test_integer_hash!(u8, hash_u8);
test_integer_hash!(u16, hash_u16);
test_integer_hash!(u32, hash_u32);
test_integer_hash!(u64, hash_u64);
test_integer_hash!(u128, hash_u128);
test_integer_hash!(usize, hash_usize);
test_integer_hash!(i8, hash_i8);
test_integer_hash!(i16, hash_i16);
test_integer_hash!(i32, hash_i32);
test_integer_hash!(i64, hash_i64);
test_integer_hash!(i128, hash_i128);
test_integer_hash!(isize, hash_isize);

#[test]
fn hash_str() {
    let hash0 = "".hash();
    let hash1 = "".hash();
    let hash2 = "one".hash();
    let hash3 = "one".hash();
    let hash4 = "two three".hash();
    assert_eq!(hash0, hash1);
    assert_eq!(hash2, hash3);
    assert_ne!(hash0, hash2);
    assert_ne!(hash0, hash3);
    assert_ne!(hash0, hash4);
    assert_ne!(hash2, hash4);
}

#[test]
fn hash_string() {
    let hash0 = "".to_string().hash();
    let hash1 = "".to_string().hash();
    let hash2 = "one".to_string().hash();
    let hash3 = "one".to_string().hash();
    let hash4 = "two three".to_string().hash();
    assert_eq!(hash0, hash1);
    assert_eq!(hash2, hash3);
    assert_ne!(hash0, hash2);
    assert_ne!(hash0, hash3);
    assert_ne!(hash0, hash4);
    assert_ne!(hash2, hash4);
}
