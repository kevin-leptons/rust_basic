mod common;

use common::{get_key_custom_type, get_value_custom_type};
use rust_basic::{HashMap, Hashable};

#[test]
fn set() {
    let mut m = HashMap::<String, String>::new();
    let n = 10000usize;
    for i in 0..n {
        m.set(get_key_custom_type(i), get_value_custom_type(i));
        assert_eq!(m.size(), (i + 1) as usize);
    }
    assert_eq!(m.size(), n);
}

#[test]
fn get() {
    let mut m = HashMap::<String, String>::new();
    let n = 10000;
    for i in 0..n {
        m.set(get_key_custom_type(i), get_value_custom_type(i));
    }
    for i in 0..n {
        let actual = m.get(&get_key_custom_type(i)).unwrap();
        let expected = get_value_custom_type(i);
        assert_eq!(actual.eq(&expected), true);
    }
    assert_eq!(m.get(&get_key_custom_type(n + 1)), Option::None);
}

#[test]
fn has() {
    let mut m = HashMap::<String, String>::new();
    let n = 10000;
    for i in 0..n {
        m.set(get_key_custom_type(i), get_value_custom_type(i));
    }
    for i in 0..n {
        assert_eq!(m.has(&get_key_custom_type(i)), true);
    }
    assert_eq!(m.has(&get_key_custom_type(n + 1)), false);
}

#[test]
fn remove() {
    let mut m = HashMap::<String, String>::new();
    let n = 10000;
    for i in 0..n {
        m.set(get_key_custom_type(i), get_value_custom_type(i));
    }
    for i in 0..n {
        let actual = m.remove(&get_key_custom_type(i)).unwrap();
        let expected = get_value_custom_type(i);
        assert_eq!(actual, expected);
        assert_eq!(m.has(&get_key_custom_type(i)), false);
        assert_eq!(m.size(), n - i - 1);
    }
    let key = get_key_custom_type(1);
    assert_eq!(m.remove(&key), Option::None);
    assert_eq!(m.has(&key), false);
    assert_eq!(m.get(&key), Option::None);
    assert_eq!(m.size(), 0);
}

#[test]
fn values() {
    let mut m = HashMap::<String, String>::new();
    let n = 10000;
    for i in 0..n {
        m.set(get_key_custom_type(i), get_value_custom_type(i));
    }
    let mut c = 0;
    for _ in m.values() {
        c = c + 1;
    }
    assert_eq!(c, n);
}

#[test]
fn from_array() {
    let a = [
        ("key: 0".to_string(), "value: 0".to_string()),
        ("key: 1".to_string(), "value: 1".to_string()),
        ("key: 2".to_string(), "value: 2".to_string()),
    ];
    let m = HashMap::from(a.clone());
    assert_eq!(m.size(), a.len());
    for i in 0..a.len() {
        let key = format!("key: {}", i);
        let expected = format!("value: {}", i);
        let r = m.get(&key);
        assert_eq!(r.is_some(), true);
        assert_eq!(r.unwrap(), &expected);
    }
}

#[test]
fn from_iter() {
    let a = [
        ("key: 0".to_string(), "value: 0".to_string()),
        ("key: 1".to_string(), "value: 1".to_string()),
        ("key: 2".to_string(), "value: 2".to_string()),
    ];
    let m = HashMap::from_iter(a.clone().into_iter());
    assert_eq!(m.size(), a.len());
    for i in 0..a.len() {
        let key = format!("key: {}", i);
        let expected = format!("value: {}", i);
        let r = m.get(&key);
        assert_eq!(r.is_some(), true);
        assert_eq!(r.unwrap(), &expected);
    }
}

#[test]
fn iter() {
    let a = [
        ("key: 0".to_string(), "value: 0".to_string()),
        ("key: 1".to_string(), "value: 1".to_string()),
        ("key: 2".to_string(), "value: 2".to_string()),
    ];
    let m = HashMap::from(a.clone());
    let mut x = [false; 3];
    for (k, v) in m.iter() {
        let r = a.iter().position(|(a_k, a_v)| {
            return a_k == k && a_v == v;
        });
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
    let m0 = HashMap::from([
        ("key: 0".to_string(), "value: 0".to_string()),
        ("key: 1".to_string(), "value: 1".to_string()),
        ("key: 2".to_string(), "value: 2".to_string()),
    ]);
    let m1 = HashMap::from([
        ("key: 1".to_string(), "value: 1".to_string()),
        ("key: 2".to_string(), "value: 2".to_string()),
        ("key: 0".to_string(), "value: 0".to_string()),
    ]);
    assert_eq!(m0, m1);
}

#[test]
fn equal_no_key() {
    let m0 = HashMap::from([
        ("key: 0".to_string(), "value: 0".to_string()),
        ("key: 1".to_string(), "value: 1".to_string()),
        ("key: 2".to_string(), "value: 2".to_string()),
    ]);
    let m1 = HashMap::from([
        ("key: 0".to_string(), "value: 0".to_string()),
        ("key: 1".to_string(), "value: 1".to_string()),
        ("key: 3".to_string(), "value: 2".to_string()),
    ]);
    assert_ne!(m0, m1);
}

#[test]
fn equal_no_value() {
    let m0 = HashMap::from([
        ("key: 0".to_string(), "value: 0".to_string()),
        ("key: 1".to_string(), "value: 1".to_string()),
        ("key: 2".to_string(), "value: 2".to_string()),
    ]);
    let m1 = HashMap::from([
        ("key: 0".to_string(), "value: 0".to_string()),
        ("key: 1".to_string(), "value: 1".to_string()),
        ("key: 2".to_string(), "value: 3".to_string()),
    ]);
    assert_ne!(m0, m1);
}

#[test]
fn clear() {
    let a = [
        ("key: 0".to_string(), "value: 0".to_string()),
        ("key: 1".to_string(), "value: 1".to_string()),
        ("key: 2".to_string(), "value: 2".to_string()),
    ];
    let mut m = HashMap::from(a.clone());
    assert_eq!(m.size(), 3);
    m.clear();
    assert_eq!(m.size(), 0);
    for i in 0..a.len() {
        let key = format!("key: {}", i);
        assert_eq!(m.get(&key), None);
        assert_eq!(m.has(&key), false);
    }
}

#[test]
fn string_hash() {
    let v1 = String::from("");
    let v2 = String::from("key: 1001");
    let v3 = String::from("key: 1001");
    let v4 = String::from("foo bar: 3");
    let h1 = v1.hash();
    let h2 = v2.hash();
    let h3 = v3.hash();
    let h4 = v4.hash();
    assert_eq!(h2, h3);
    assert_ne!(h2, h1);
    assert_ne!(h2, h4);
}
