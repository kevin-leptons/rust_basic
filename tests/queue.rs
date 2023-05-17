use rust_basic::queue::Queue;

#[test]
fn push_then_pop() {
    let mut q = Queue::<String>::new();
    for k in 0..500 {
        for i in 0..200 {
            q.push(format!("item: {}", i * k));
        }
        for i in 0..200 {
            assert_eq!(q.pop(), format!("item: {}", i * k));
        }
        q.push(String::from("item: last"));
        assert_eq!(q.pop(), String::from("item: last"));
    }
}

#[test]
fn from_array() {
    let a = [
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ];
    let q = Queue::<String>::from(a.clone());
    assert_eq!(q.size(), a.len());
    for i in 0..a.len() {
        assert_eq!(q.get(i), &format!("item: {}", i));
    }
}

#[test]
fn from_iterator() {
    let a = [
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ];
    let q = Queue::<String>::from_iter(a.clone().into_iter());
    assert_eq!(q.size(), a.len());
    for i in 0..a.len() {
        assert_eq!(q.get(i), &a[i]);
    }
}

#[test]
fn get() {
    let a = [
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ];
    let q = Queue::<String>::from(a.clone());
    for i in 0..a.len() {
        assert_eq!(q.get(i), &format!("item: {}", i));
    }
}

#[test]
#[should_panic(expected = "expect: `index` is less than size")]
fn get_panic() {
    let a = [
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ];
    let q = Queue::<String>::from(a.clone());
    q.get(a.len());
}

#[test]
fn iter() {
    let a = [
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ];
    let q = Queue::<String>::from(a.clone());
    let mut i = q.iter();
    for k in 0..q.size() {
        assert_eq!(i.next(), Some(&a[k]));
    }
    assert_eq!(i.next(), None);
}

#[test]
fn top() {
    let q = Queue::<String>::from([
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ]);
    assert_eq!(q.top(), &"item: 0".to_string());
}

#[test]
#[should_panic(expected = "expect: non empty queue")]
fn top_panic() {
    let q = Queue::<String>::from([]);
    q.top();
}

#[test]
fn clone() {
    let q1 = Queue::<String>::from([
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ]);
    let q2 = q1.clone();
    assert_eq!(q1.size(), q2.size());
    for i in 0..q1.size() - 1 {
        assert_eq!(q1.get(i), q2.get(i));
    }
}

#[test]
fn clear() {
    let mut q = Queue::<String>::from([
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ]);
    q.clear();
    assert_eq!(q.size(), 0);
}
