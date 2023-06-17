use rust_basic::{red_black_tree::RedBlackTree, Vector};

#[test]
fn set_existing() {
    let a = [
        ("key: 0".to_string(), "value: 0".to_string()),
        ("key: 1".to_string(), "value: 1".to_string()),
        ("key: 2".to_string(), "value: 2".to_string()),
    ];
    let mut t = RedBlackTree::from(a.clone());
    for i in 0..t.size() {
        let old = t.set(a[i].0.clone(), format!("new value: {}", i)).unwrap();
        assert_eq!(old, a[i].1);
        assert_eq!(t.size(), a.len());
    }
    for i in 0..t.size() {
        assert_eq!(t.get(&a[i].0), &format!("new value: {}", i));
    }
}

#[test]
fn from_array() {
    let t = RedBlackTree::from([
        ("key: 0".to_string(), "value: 0".to_string()),
        ("key: 1".to_string(), "value: 1".to_string()),
        ("key: 2".to_string(), "value: 2".to_string()),
    ]);
    assert_eq!(t.size(), 3);
    for i in 0..t.size() {
        let v = t.get(&format!("key: {}", i));
        assert_eq!(v, &format!("value: {}", i));
    }
}

#[test]
fn from_iter() {
    let t = RedBlackTree::from_iter([
        ("key: 0".to_string(), "value: 0".to_string()),
        ("key: 1".to_string(), "value: 1".to_string()),
        ("key: 2".to_string(), "value: 2".to_string()),
    ]);
    assert_eq!(t.size(), 3);
    for i in 0..t.size() {
        let v = t.get(&format!("key: {}", i));
        assert_eq!(v, &format!("value: {}", i));
    }
}

#[test]
fn has() {
    let t = build_sample();
    let n = t.size();
    for i in 0..n {
        assert_eq!(t.has(&format!("key: {}", i)), true);
    }
}

#[test]
fn get_mut() {
    let mut t = RedBlackTree::from_iter([
        ("key: 0".to_string(), "value: 0".to_string()),
        ("key: 1".to_string(), "value: 1".to_string()),
        ("key: 2".to_string(), "value: 2".to_string()),
    ]);
    let k = "key: 1".to_string();
    let v = t.get_mut(&k);
    let new_content = "new value: 1";
    v.replace_range(.., new_content);
    assert_eq!(t.get(&k), new_content);
}

#[test]
fn remove() {
    let mut t = build_sample();
    let n = t.size();
    for i in 0..n {
        let k = format!("key: {}", i);
        let v = format!("value: {}", i);
        let actual = t.remove(&k);
        assert_eq!(actual, Some(v.clone()));
        assert_eq!(t.has(&v), false);
        assert_eq!(t.size(), n - i - 1);
    }
}

#[test]
fn iter() {
    let a = [
        ("key: 0".to_string(), "value: 0".to_string()),
        ("key: 1".to_string(), "value: 1".to_string()),
        ("key: 2".to_string(), "value: 2".to_string()),
    ];
    let t = RedBlackTree::from(a.clone());
    let mut actual: Vector<(&String, &String)> = t.iter().collect();
    let mut expected: Vector<(&String, &String)> =
        a.iter().map(|(k, v)| (k, v)).collect();
    actual.sort();
    expected.sort();
    assert_eq!(actual, expected);
}

#[test]
fn keys() {
    let a = [
        ("key: 0".to_string(), "value: 0".to_string()),
        ("key: 1".to_string(), "value: 1".to_string()),
        ("key: 2".to_string(), "value: 2".to_string()),
    ];
    let t = RedBlackTree::from(a.clone());
    let mut actual: Vector<&String> = t.keys().collect();
    let mut expected: Vector<&String> = a.iter().map(|(k, _)| k).collect();
    actual.sort();
    expected.sort();
    assert_eq!(actual, expected);
}

#[test]
fn values() {
    let a = [
        ("key: 0".to_string(), "value: 0".to_string()),
        ("key: 1".to_string(), "value: 1".to_string()),
        ("key: 2".to_string(), "value: 2".to_string()),
    ];
    let t = RedBlackTree::from(a.clone());
    let mut actual: Vector<&String> = t.values().collect();
    let mut expected: Vector<&String> = a.iter().map(|(_, v)| v).collect();
    actual.sort();
    expected.sort();
    assert_eq!(actual, expected);
}

#[test]
fn min() {
    let t = build_sample();
    assert_eq!(t.min(), "key: 0");
}

#[test]
fn max() {
    let t = build_sample();
    assert_eq!(t.max(), "key: 99999");
}

#[test]
fn clear() {
    let mut t = build_sample();
    assert!(t.size() > 0);
    t.clear();
    assert_eq!(t.size(), 0);
}

fn build_sample() -> RedBlackTree<String, String> {
    let mut t = RedBlackTree::<String, String>::new();
    for i in 0..100000 {
        t.set(format!("key: {}", i), format!("value: {}", i));
    }
    return t;
}
