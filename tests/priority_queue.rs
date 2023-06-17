use std::ops::Index;

use rust_basic::priority_queue::PriorityQueue;

#[test]
fn push() {
    let mut q = PriorityQueue::<String>::new();
    let v1 = String::from("value: 1");
    let v2 = String::from("value: 2");
    let v3 = String::from("value: 3");
    let v4 = String::from("value: 4");
    q.push(v3);
    assert_eq!(q.size(), 1);
    q.push(v1);
    assert_eq!(q.size(), 2);
    q.push(v4);
    assert_eq!(q.size(), 3);
    q.push(v2);
    assert_eq!(q.size(), 4);
}

#[test]
fn pop() {
    let mut q = PriorityQueue::<String>::new();
    let v1 = String::from("value: 1");
    let v2 = String::from("value: 2");
    let v3 = String::from("value: 3");
    let v4 = String::from("value: 4");
    q.push(v3.clone());
    q.push(v1.clone());
    q.push(v4.clone());
    q.push(v2.clone());
    q.push(v1.clone());
    q.push(v3.clone());
    assert_eq!(q.pop(), v4);
    assert_eq!(q.size(), 5);
    assert_eq!(q.pop(), v3);
    assert_eq!(q.size(), 4);
    assert_eq!(q.pop(), v3);
    assert_eq!(q.size(), 3);
    assert_eq!(q.pop(), v2);
    assert_eq!(q.size(), 2);
    assert_eq!(q.pop(), v1);
    assert_eq!(q.size(), 1);
    assert_eq!(q.pop(), v1);
    assert_eq!(q.size(), 0);
}

#[test]
fn top() {
    let mut q = PriorityQueue::<String>::new();
    let v1 = String::from("value: 1");
    let v2 = String::from("value: 2");
    let v3 = String::from("value: 3");
    let v4 = String::from("value: 4");
    q.push(v3.clone());
    q.push(v1.clone());
    q.push(v4.clone());
    q.push(v2.clone());
    q.push(v4.clone());
    assert_eq!(q.top(), &v4);
}

#[test]
fn from_array() {
    let a = [
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ];
    let mut q = PriorityQueue::from(a.clone());
    assert_eq!(q.size(), a.len());
    for i in (0..q.size()).rev() {
        assert_eq!(q.pop(), format!("item: {}", i));
    }
}

#[test]
fn from_iter() {
    let a = [
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ];
    let mut q = PriorityQueue::from_iter(a.clone().into_iter());
    assert_eq!(q.size(), a.len());
    for i in (0..q.size()).rev() {
        assert_eq!(q.pop(), format!("item: {}", i));
    }
}

#[test]
fn iter() {
    let a = [
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ];
    let q = PriorityQueue::from(a.clone());
    let mut x = [false, false, false];
    for i in q.iter() {
        let p = a.iter().position(|v| v == i).unwrap();
        assert_eq!(x[p], false);
        x[p] = true;
    }
    for i in x {
        assert_eq!(i, true);
    }
}

#[test]
fn clear() {
    let mut q = PriorityQueue::from([
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ]);
    q.clear();
    assert_eq!(q.size(), 0);
}
