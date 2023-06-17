use rust_basic::{binary_search_tree::BinarySearchTree, HashSet};

#[test]
fn set() {
    let mut t = BinarySearchTree::new();
    let a = [
        ("key: 0".to_string(), "value: 0".to_string()),
        ("key: 1".to_string(), "value: 1".to_string()),
        ("key: 2".to_string(), "value: 2".to_string()),
    ];
    for (k, v) in a.clone() {
        assert_eq!(t.set(k, v), None);
    }
    assert_eq!(t.size(), a.len());
    for (k, _) in a.as_ref() {
        assert_eq!(t.has(k), true);
    }
}

#[test]
fn set_existing() {
    let a = [
        ("key: 0".to_string(), "value: 0".to_string()),
        ("key: 1".to_string(), "value: 1".to_string()),
        ("key: 2".to_string(), "value: 2".to_string()),
    ];
    let mut t = BinarySearchTree::from(a.clone());
    for i in 0..a.len() {
        let old_value =
            t.set(a[i].0.clone(), format!("new value: {}", i)).unwrap();
        assert_eq!(old_value, format!("value: {}", i));
    }
    for i in 0..a.len() {
        let v = t.get(&a[i].0.clone());
        assert_eq!(v, &format!("new value: {}", i));
    }
}

#[test]
fn get_mut() {
    let mut t = BinarySearchTree::new();
    let a = [
        ("key: 0".to_string(), "value: 0".to_string()),
        ("key: 1".to_string(), "value: 1".to_string()),
        ("key: 2".to_string(), "value: 2".to_string()),
    ];
    for (k, v) in a.clone() {
        t.set(k, v);
    }
    let (k, _) = &a[1];
    t.get_mut(k).replace_range(0..5, "new value");
    assert_eq!(t.get(k), &"new value: 1".to_string());
}

#[test]
fn has() {
    let t = build_example();
    for i in 0..t.size() {
        assert_eq!(t.has(&format!("key: {}", i)), true);
    }
    assert_eq!(t.has(&format!("key: {}", t.size())), false);
}

#[test]
fn from_array() {
    let a = [
        ("key: 0".to_string(), "value: 0".to_string()),
        ("key: 1".to_string(), "value: 1".to_string()),
        ("key: 2".to_string(), "value: 2".to_string()),
    ];
    let t = BinarySearchTree::from(a.clone());
    assert_eq!(t.size(), a.len());
    for (k, v) in a {
        assert_eq!(t.get(&k), &v);
    }
}

#[test]
fn from_iter() {
    let a = [
        ("key: 0".to_string(), "value: 0".to_string()),
        ("key: 1".to_string(), "value: 1".to_string()),
        ("key: 2".to_string(), "value: 2".to_string()),
    ];
    let t = BinarySearchTree::from_iter(a.clone());
    assert_eq!(t.size(), a.len());
    for (k, v) in a {
        assert_eq!(t.get(&k), &v);
    }
}

#[test]
fn iter() {
    let a = [
        ("key: 0".to_string(), "value: 0".to_string()),
        ("key: 1".to_string(), "value: 1".to_string()),
        ("key: 2".to_string(), "value: 2".to_string()),
    ];
    let t = BinarySearchTree::from(a.clone());
    let mut s = HashSet::new();
    for (i_k, i_v) in t.iter() {
        let i = a.iter().position(|(k, v)| k == i_k && v == i_v).unwrap();
        assert_eq!(s.has(&i), false);
        s.add(i);
    }
    assert_eq!(s.size(), a.len());
}

#[test]
fn keys() {
    let a = [
        ("key: 0".to_string(), "value: 0".to_string()),
        ("key: 1".to_string(), "value: 1".to_string()),
        ("key: 2".to_string(), "value: 2".to_string()),
    ];
    let t = BinarySearchTree::from(a.clone());
    let mut s = HashSet::new();
    for i_k in t.keys() {
        let i = a.iter().position(|(k, _)| k == i_k).unwrap();
        assert_eq!(s.has(&i), false);
        s.add(i);
    }
    assert_eq!(s.size(), a.len());
}

#[test]
fn values() {
    let a = [
        ("key: 0".to_string(), "value: 0".to_string()),
        ("key: 1".to_string(), "value: 1".to_string()),
        ("key: 2".to_string(), "value: 2".to_string()),
    ];
    let t = BinarySearchTree::from(a.clone());
    let mut s = HashSet::new();
    for i_v in t.values() {
        let i = a.iter().position(|(_, v)| v == i_v).unwrap();
        assert_eq!(s.has(&i), false);
        s.add(i);
    }
    assert_eq!(s.size(), a.len());
}

#[test]
fn min() {
    let t = build_example();
    assert_eq!(t.min(), &"key: 0".to_string());
}

#[test]
fn max() {
    let t = build_example();
    assert_eq!(t.max(), &"key: 99999".to_string());
}

#[test]
fn remove() {
    let mut t = build_example();
    assert!(t.size() > 0);
    let n = t.size();
    for i in 0..n {
        let k = format!("key: {}", i);
        let v = format!("value: {}", i);
        assert_eq!(t.remove(&k), Some(v));
        assert_eq!(t.has(&k), false);
        assert_eq!(t.size(), n - i - 1);
    }
    assert_eq!(t.size(), 0);
}

#[test]
fn clear() {
    let mut t = build_example();
    assert!(t.size() > 0);
    t.clear();
    assert_eq!(t.size(), 0);
}

fn build_example() -> BinarySearchTree<String, String> {
    let mut t = BinarySearchTree::<String, String>::new();
    for i in 0..50000 {
        t.set(format!("key: {}", i), format!("value: {}", i));
    }
    for i in (50000..100000).rev() {
        t.set(format!("key: {}", i), format!("value: {}", i));
    }
    return t;
}
