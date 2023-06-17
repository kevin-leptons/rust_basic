mod common;

use common::get_key_custom_type;
use rust_basic::hash_set::HashSet;

#[test]
fn add() {
    let mut s = HashSet::<String>::new();
    let n = 10000;
    for i in 0..n {
        assert_eq!(s.add(get_key_custom_type(i)), false);
        assert_eq!(s.size(), i + 1);
    }
    assert_eq!(s.size(), n);
}

#[test]
fn has() {
    let mut s = HashSet::<String>::new();
    let n = 10000;
    for i in 0..n {
        s.add(get_key_custom_type(i));
    }
    for i in 0..n {
        assert_eq!(s.has(&get_key_custom_type(i)), true);
    }
    assert_eq!(s.has(&get_key_custom_type(n + 1)), false);
}

#[test]
fn remove() {
    let mut s = HashSet::<String>::new();
    let n = 10000;
    for i in 0..n {
        s.add(get_key_custom_type(i));
    }
    for i in 0..n {
        assert_eq!(s.remove(&get_key_custom_type(i)), true);
        assert_eq!(s.size(), n - i - 1);
    }
    assert_eq!(s.remove(&get_key_custom_type(n + 1)), false);
    assert_eq!(s.has(&get_key_custom_type(1)), false);
    assert_eq!(s.size(), 0);
}

#[test]
fn from_array() {
    let a = [
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ];
    let s = HashSet::from(a.clone());
    assert_eq!(s.size(), 3);
    for k in 0..a.len() {
        let i = format!("item: {}", k);
        assert_eq!(s.has(&i), true);
    }
}

#[test]
fn from_iter() {
    let a = [
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ];
    let s = HashSet::from_iter(a.clone().into_iter());
    assert_eq!(s.size(), 3);
    for k in 0..a.len() {
        let i = format!("item: {}", k);
        assert_eq!(s.has(&i), true);
    }
}

#[test]
fn iter() {
    let a = [
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ];
    let s = HashSet::from(a.clone());
    let mut x = [false; 3];
    for i in s.iter() {
        let r = a.iter().position(|item| item == i);
        assert_eq!(r.is_some(), true);
        let p = r.unwrap();
        assert_eq!(x[p], false);
        x[p] = true;
    }
    for i in x {
        assert_eq!(i, true);
    }
}

#[test]
fn equal() {
    let s0 = HashSet::from([
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ]);
    let s1 = HashSet::from([
        "item: 2".to_string(),
        "item: 0".to_string(),
        "item: 1".to_string(),
    ]);
    assert_eq!(s0, s1);
}

#[test]
fn equal_no() {
    let s0 = HashSet::from([
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ]);
    let s1 = HashSet::from([
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 3".to_string(),
    ]);
    assert_ne!(s0, s1);
}

#[test]
fn clear() {
    let a = [
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ];
    let mut s = HashSet::from(a.clone());
    assert_eq!(s.size(), 3);
    s.clear();
    assert_eq!(s.size(), 0);
    for i in 0..a.len() {
        let key = format!("item: {}", i);
        assert_eq!(s.has(&key), false);
    }
}
