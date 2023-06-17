use rust_basic::linked_list::LinkedList;

#[test]
fn set() {
    let mut l = LinkedList::new();
    l.set(0, "item: 0".to_string());
    l.set(1, "item: 2".to_string());
    l.set(1, "item: 1".to_string());
    assert_eq!(l.size(), 3);
    for i in 0..l.size() {
        assert_eq!(l.get(i), &format!("item: {}", i));
    }
}

#[test]
#[should_panic(expected = "expect: `index` is not greater than size")]
fn set_panic_empty() {
    let mut l = LinkedList::<String>::new();
    l.set(1, "value: 0".to_string());
}

#[test]
#[should_panic(expected = "expect: `index` is not greater than size")]
fn set_panic_non_empty() {
    let mut l = LinkedList::<String>::new();
    l.set(0, "value: 0".to_string());
    l.set(2, "value: 2".to_string());
}

#[test]
fn get_mut() {
    let mut l = sample_small();
    for k in 0..l.size() {
        l.get_mut(k)
            .replace_range(.., format!("new item: {}", k).as_str());
    }
    for k in 0..l.size() {
        assert_eq!(l.get(k), &format!("new item: {}", k));
    }
}

#[test]
#[should_panic(expected = "expect: `index` is less than size")]
fn get_panic_empty() {
    let l = LinkedList::<String>::new();
    l.get(0);
}

#[test]
#[should_panic(expected = "expect: `index` is less than size")]
fn get_panic_non_empty() {
    let mut l = LinkedList::<String>::new();
    l.set(0, "value: 0".to_string());
    l.get(1);
}

#[test]
fn index_front() {
    let l = sample_tiny();
    assert_eq!(l[0], "item: 0".to_string());
}

#[test]
fn index_middle() {
    let l = sample_tiny();
    assert_eq!(l[49], "item: 49".to_string());
}

#[test]
fn index_back() {
    let l = sample_tiny();
    assert_eq!(l[99], "item: 99".to_string());
}

#[test]
#[should_panic(expected = "expect: `index` is less than size")]
fn index_panic_empty() {
    let l = LinkedList::<String>::new();
    let _ = l[0].len();
}

#[test]
#[should_panic(expected = "expect: `index` is less than size")]
fn index_panic_non_empty() {
    let l = sample_tiny();
    assert!(l.size() > 0);
    let _ = l[l.size()].len();
}

#[test]
fn push_front() {
    let mut l = sample();
    let n = l.size();
    let v = "new value".to_string();
    l.push_front(v.clone());
    assert_eq!(l.get(0), &v);
    assert_eq!(l.get(n), &format!("item: {}", n - 1));
    assert_eq!(l.size(), n + 1);
}

#[test]
fn push_back() {
    let mut l = sample();
    let n = l.size();
    let v = "new value".to_string();
    l.push_back(v.clone());
    assert_eq!(l.get(n), &v);
    assert_eq!(l.size(), n + 1);
}

#[test]
fn from_array() {
    let a = [
        "value: 0".to_string(),
        "value: 1".to_string(),
        "value: 2".to_string(),
        "value: 3".to_string(),
        "value: 4".to_string(),
    ];
    let l = LinkedList::from(a.clone());
    assert_eq!(l.size(), a.len());
    for i in 0..l.size() {
        assert_eq!(l.get(i), &a[i]);
    }
}

#[test]
fn from_iter() {
    let a = [
        "value: 0".to_string(),
        "value: 1".to_string(),
        "value: 2".to_string(),
        "value: 3".to_string(),
        "value: 4".to_string(),
    ];
    let l = LinkedList::from_iter(a.clone());
    assert_eq!(l.size(), a.len());
    for i in 0..l.size() {
        assert_eq!(l.get(i), &a[i]);
    }
}

#[test]
fn iter() {
    let l = sample();
    let mut k = 0;
    for i in l.iter() {
        assert_eq!(i, &format!("item: {}", k));
        k += 1;
    }
    assert_eq!(k, l.size());
}

#[test]
fn iter_mut() {
    let mut l = sample();
    let mut k = 0;
    for i in l.iter_mut() {
        i.replace_range(.., format!("new item: {}", k).as_str());
        k += 1;
    }
    k = 0;
    for i in l.iter() {
        assert_eq!(i, &format!("new item: {}", k));
        k += 1;
    }
}

#[test]
fn front() {
    let l = sample();
    assert_eq!(l.front(), &"item: 0");
}

#[test]
fn back() {
    let l = sample();
    assert_eq!(l.back(), &format!("item: {}", l.size() - 1));
}

#[test]
fn remove() {
    let a = [
        "value: 0".to_string(),
        "value: 1".to_string(),
        "value: 2".to_string(),
        "value: 3".to_string(),
        "value: 4".to_string(),
    ];
    let mut l = LinkedList::<String>::from(a.clone());

    assert_eq!(l.remove(0), a[0]);
    assert_eq!(l.size(), 4);
    assert_eq!(l.remove(3), a[4]);
    assert_eq!(l.size(), 3);
    assert_eq!(l.remove(1), a[2]);
    assert_eq!(l.size(), 2);
    assert_eq!(l.remove(0), a[1]);
    assert_eq!(l.size(), 1);
    assert_eq!(l.remove(0), a[3]);
    assert_eq!(l.size(), 0);
}

#[test]
#[should_panic(expected = "expect: `index` is less than size")]
fn remove_panic_empty() {
    let mut l = LinkedList::<String>::new();
    l.set(0, "value: 0".to_string());
    l.remove(1);
}

#[test]
#[should_panic(expected = "expect: `index` is less than size")]
fn remove_panic_non_empty() {
    let mut l = LinkedList::<String>::new();

    l.remove(0);
}

#[test]
fn pop_front() {
    let mut l = sample();
    let n = l.size();
    assert_eq!(l.pop_front(), "item: 0".to_string());
    assert_eq!(l.get(n - 2), &format!("item: {}", n - 1));
    assert_eq!(l.size(), n - 1);
}

#[test]
fn pop_back() {
    let mut l = sample();
    let n = l.size();
    assert_eq!(l.pop_back(), format!("item: {}", n - 1));
    assert_eq!(l.size(), n - 1);
}

#[test]
fn equal() {
    let l0 = LinkedList::from([
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ]);
    let l1 = LinkedList::from([
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ]);
    assert_eq!(l0, l1);
}

#[test]
fn equal_no() {
    let l0 = LinkedList::from([
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ]);
    let l1 = LinkedList::from([
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 3".to_string(),
    ]);
    assert_ne!(l0, l1);
}

#[test]
fn clear() {
    let mut l = LinkedList::from([
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ]);
    assert_eq!(l.size(), 3);
    l.clear();
    assert_eq!(l.size(), 0);
}

fn sample() -> LinkedList<String> {
    let mut l = LinkedList::<String>::new();
    for i in 0..100000 {
        l.set(l.size(), format!("item: {}", i));
    }
    return l;
}

fn sample_small() -> LinkedList<String> {
    let mut l = LinkedList::<String>::new();
    for i in 0..10000 {
        l.set(l.size(), format!("item: {}", i));
    }
    return l;
}

fn sample_tiny() -> LinkedList<String> {
    let mut l = LinkedList::<String>::new();
    for i in 0..100 {
        l.set(l.size(), format!("item: {}", i));
    }
    return l;
}
