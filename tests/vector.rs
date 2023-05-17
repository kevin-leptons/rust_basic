use rust_basic::vector::Vector;

#[test]
fn set() {
    let mut v = Vector::<String>::new();
    let s0 = "item: 0".to_string();
    let s1 = "item: 1".to_string();
    let s2 = "item: 2".to_string();
    let s3 = "item: 3".to_string();
    let s4 = "item: 4".to_string();
    let s5 = "item: 5".to_string();
    v.set(0, s1.clone());
    assert_eq!(v.size(), 1);
    v.set(1, s2.clone());
    assert_eq!(v.size(), 2);
    v.set(2, s4.clone());
    assert_eq!(v.size(), 3);
    v.set(0, s0.clone());
    assert_eq!(v.size(), 4);
    v.set(3, s3.clone());
    assert_eq!(v.size(), 5);
    v.set(5, s5.clone());
    assert_eq!(v.size(), 6);
    assert_eq!(v.get(0), &s0);
    assert_eq!(v.get(1), &s1);
    assert_eq!(v.get(2), &s2);
    assert_eq!(v.get(3), &s3);
    assert_eq!(v.get(4), &s4);
    assert_eq!(v.get(5), &s5);
}

#[test]
#[should_panic(expected = "expect: `index` is not greater than size")]
fn set_cause_panic() {
    let mut v = Vector::<String>::new();
    v.set(0, "item: 0".to_string());
    v.set(1, "item: 1".to_string());
    v.set(3, "item: 3".to_string());
}

#[test]
#[should_panic(expected = "`index` is less than size")]
fn get_cause_panic() {
    let v = Vector::from([
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ]);
    v.get(3);
}

#[test]
#[should_panic(expected = "`index` is less than size")]
fn get_cause_panic_by_empty() {
    let v = Vector::<String>::new();
    v.get(0);
}

#[test]
fn index() {
    let v = Vector::<String>::from([
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ]);
    for i in 0..v.size() {
        assert_eq!(v[i], format!("item: {}", i));
    }
}

#[test]
fn from_array() {
    let a = [
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ];
    let v = Vector::<String>::from(a.clone());
    assert_eq!(v.size(), a.len());
    for i in 0..v.size() {
        assert_eq!(v[i], format!("item: {}", i));
    }
}

#[test]
fn from_iter() {
    let a = [
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ];
    let v = Vector::from_iter(a.clone().into_iter());
    assert_eq!(v.size(), a.len());
    for i in 0..v.size() {
        assert_eq!(v[i], format!("item: {}", i));
    }
}

#[test]
fn equal() {
    let v1 = Vector::from([
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
        "item: 3".to_string(),
    ]);
    let v2 = Vector::from([
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
        "item: 3".to_string(),
    ]);
    assert_eq!(v1, v2);
}

#[test]
fn equal_no() {
    let v1 = Vector::from([
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
        "item: 4".to_string(),
    ]);
    let v2 = Vector::from([
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
        "item: 3".to_string(),
    ]);
    assert_ne!(v1, v2);
}

#[test]
fn remove_first() {
    let s0 = "item: 0".to_string();
    let s1 = "item: 1".to_string();
    let s2 = "item: 2".to_string();
    let s3 = "item: 3".to_string();
    let mut v = Vector::from([s0.clone(), s1.clone(), s2.clone(), s3.clone()]);
    assert_eq!(v.remove(0), s0);
    assert_eq!(v, Vector::from([s1.clone(), s2.clone(), s3.clone()]));
}

#[test]
fn remove_last() {
    let s0 = "item: 0".to_string();
    let s1 = "item: 1".to_string();
    let s2 = "item: 2".to_string();
    let s3 = "item: 3".to_string();
    let mut v = Vector::from([s0.clone(), s1.clone(), s2.clone(), s3.clone()]);
    assert_eq!(v.remove(3), s3);
    assert_eq!(v, Vector::from([s0.clone(), s1.clone(), s2.clone()]));
}

#[test]
fn remove_middle() {
    let s0 = "item: 0".to_string();
    let s1 = "item: 1".to_string();
    let s2 = "item: 2".to_string();
    let s3 = "item: 3".to_string();
    let mut v = Vector::from([s0.clone(), s1.clone(), s2.clone(), s3.clone()]);
    assert_eq!(v.remove(1), s1);
    assert_eq!(v, Vector::from([s0.clone(), s2.clone(), s3.clone()]));
}

#[test]
#[should_panic(expected = "expect: `index` is less than size")]
fn remove_cause_panic() {
    let mut v = Vector::from([
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ]);
    v.remove(3);
}

#[test]
#[should_panic(expected = "expect: `index` is less than size")]
fn remove_cause_panic_by_empty() {
    let mut v = Vector::<String>::new();
    v.remove(0);
}

#[test]
fn swap() {
    let s0 = "item: 0".to_string();
    let s1 = "item: 1".to_string();
    let s2 = "item: 2".to_string();
    let s3 = "item: 3".to_string();
    let mut v = Vector::from([s0.clone(), s1.clone(), s2.clone(), s3.clone()]);
    v.swap(1, 2);
    assert_eq!(
        v,
        Vector::from([s0.clone(), s2.clone(), s1.clone(), s3.clone()])
    );
}

#[test]
#[should_panic(expected = "expect: `first` is less than size")]
fn swap_cause_panic_by_first() {
    let mut v = Vector::from([
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ]);
    v.swap(3, 1);
}

#[test]
#[should_panic(expected = "expect: `second` is less than size")]
fn swap_cause_panic_by_second() {
    let mut v = Vector::from([
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ]);
    v.swap(1, 3);
}

#[test]
fn clear() {
    let mut v = Vector::from([
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
        "item: 3".to_string(),
    ]);
    assert_eq!(v.size(), 4);
    v.clear();
    assert_eq!(v.size(), 0);
}

#[test]
fn iter() {
    let v = Vector::from([
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
        "item: 3".to_string(),
    ]);
    let mut k = 0;
    for i in v.iter() {
        assert_eq!(i, &format!("item: {}", k));
        k += 1;
    }
}

#[test]
fn iter_mut() {
    let mut v = Vector::from([
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
        "item: 3".to_string(),
    ]);
    for i in v.iter_mut() {
        i.replace_range(0..4, "new item");
    }
    for i in 0..v.size() {
        assert_eq!(v.get(i), &format!("new item: {}", i));
    }
}

#[test]
fn sort_insertion() {
    let s1 = "item: 1".to_string();
    let s2 = "item: 2".to_string();
    let s3 = "item: 3".to_string();
    let s4 = "item: 4".to_string();
    let s5 = "item: 5".to_string();
    let s6 = "item: 6".to_string();
    let s7 = "item: 7".to_string();
    let s8 = "item: 8".to_string();
    let mut v = Vector::from([
        s3.clone(),
        s2.clone(),
        s6.clone(),
        s3.clone(),
        s1.clone(),
        s5.clone(),
        s8.clone(),
        s2.clone(),
        s4.clone(),
        s1.clone(),
        s7.clone(),
        s4.clone(),
    ]);
    v.sort_insertion();
    assert_eq!(
        v,
        Vector::from([
            s1.clone(),
            s1.clone(),
            s2.clone(),
            s2.clone(),
            s3.clone(),
            s3.clone(),
            s4.clone(),
            s4.clone(),
            s5.clone(),
            s6.clone(),
            s7.clone(),
            s8.clone()
        ])
    );
}

#[test]
fn sort_selection() {
    let s1 = "item: 1".to_string();
    let s2 = "item: 2".to_string();
    let s3 = "item: 3".to_string();
    let s4 = "item: 4".to_string();
    let s5 = "item: 5".to_string();
    let s6 = "item: 6".to_string();
    let s7 = "item: 7".to_string();
    let s8 = "item: 8".to_string();
    let mut v = Vector::from([
        s3.clone(),
        s2.clone(),
        s6.clone(),
        s3.clone(),
        s1.clone(),
        s5.clone(),
        s8.clone(),
        s2.clone(),
        s4.clone(),
        s1.clone(),
        s7.clone(),
        s4.clone(),
    ]);
    v.sort_selection();
    assert_eq!(
        v,
        Vector::from([
            s1.clone(),
            s1.clone(),
            s2.clone(),
            s2.clone(),
            s3.clone(),
            s3.clone(),
            s4.clone(),
            s4.clone(),
            s5.clone(),
            s6.clone(),
            s7.clone(),
            s8.clone()
        ])
    );
}

#[test]
fn sort_merge() {
    let s1 = "item: 1".to_string();
    let s2 = "item: 2".to_string();
    let s3 = "item: 3".to_string();
    let s4 = "item: 4".to_string();
    let s5 = "item: 5".to_string();
    let s6 = "item: 6".to_string();
    let s7 = "item: 7".to_string();
    let s8 = "item: 8".to_string();
    let mut v = Vector::from([
        s3.clone(),
        s2.clone(),
        s6.clone(),
        s3.clone(),
        s1.clone(),
        s5.clone(),
        s8.clone(),
        s2.clone(),
        s4.clone(),
        s1.clone(),
        s7.clone(),
        s4.clone(),
    ]);
    v.sort_merge();
    assert_eq!(
        v,
        Vector::from([
            s1.clone(),
            s1.clone(),
            s2.clone(),
            s2.clone(),
            s3.clone(),
            s3.clone(),
            s4.clone(),
            s4.clone(),
            s5.clone(),
            s6.clone(),
            s7.clone(),
            s8.clone()
        ])
    );
}

#[test]
fn sort_quick() {
    let s1 = "item: 1".to_string();
    let s2 = "item: 2".to_string();
    let s3 = "item: 3".to_string();
    let s4 = "item: 4".to_string();
    let s5 = "item: 5".to_string();
    let s6 = "item: 6".to_string();
    let s7 = "item: 7".to_string();
    let s8 = "item: 8".to_string();
    let mut v = Vector::from([
        s3.clone(),
        s2.clone(),
        s6.clone(),
        s3.clone(),
        s1.clone(),
        s5.clone(),
        s8.clone(),
        s2.clone(),
        s4.clone(),
        s1.clone(),
        s7.clone(),
        s4.clone(),
    ]);
    v.sort_quick();
    assert_eq!(
        v,
        Vector::from([
            s1.clone(),
            s1.clone(),
            s2.clone(),
            s2.clone(),
            s3.clone(),
            s3.clone(),
            s4.clone(),
            s4.clone(),
            s5.clone(),
            s6.clone(),
            s7.clone(),
            s8.clone()
        ])
    );
}

#[test]
fn sort() {
    let s1 = "item: 1".to_string();
    let s2 = "item: 2".to_string();
    let s3 = "item: 3".to_string();
    let s4 = "item: 4".to_string();
    let s5 = "item: 5".to_string();
    let s6 = "item: 6".to_string();
    let s7 = "item: 7".to_string();
    let s8 = "item: 8".to_string();
    let mut v = Vector::from([
        s3.clone(),
        s2.clone(),
        s6.clone(),
        s3.clone(),
        s1.clone(),
        s5.clone(),
        s8.clone(),
        s2.clone(),
        s4.clone(),
        s1.clone(),
        s7.clone(),
        s4.clone(),
    ]);
    v.sort();
    assert_eq!(
        v,
        Vector::from([
            s1.clone(),
            s1.clone(),
            s2.clone(),
            s2.clone(),
            s3.clone(),
            s3.clone(),
            s4.clone(),
            s4.clone(),
            s5.clone(),
            s6.clone(),
            s7.clone(),
            s8.clone()
        ])
    );
}
