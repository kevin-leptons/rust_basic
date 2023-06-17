use rust_basic::stack::Stack;

#[test]
fn push_then_pop_primitive_type() {
    let mut s = Stack::<u32>::new();
    for i in 1..100 {
        s.push(i);
    }
    for i in 1..100 {
        let r = s.pop();
        assert_eq!(r, 100 - i);
    }
}
#[test]
fn push_then_pop_non_primitive_type() {
    let mut s = Stack::<String>::new();
    for i in 1..100 {
        s.push(i.to_string());
    }
    for i in 1..100 {
        let r = s.pop();
        assert_eq!(r, (100 - i).to_string());
    }
}

#[test]
fn index() {
    let a = sample();
    let s = Stack::from(a);
    for i in 0..s.size() {
        assert_eq!(s[i], format!("item: {}", i));
    }
}

#[test]
#[should_panic(expected = "expect: `index` is less than size")]
fn index_panic_empty() {
    let s = Stack::<String>::new();
    let _ = s[0].len();
}

#[test]
#[should_panic(expected = "expect: `index` is less than size")]
fn index_panic_non_empty() {
    let s = sample();
    let _ = s[s.size()].len();
}

#[test]
fn pop() {
    let mut s = sample();
    for i in (0..s.size()).rev() {
        assert_eq!(s.pop(), format!("item: {}", i));
    }
}

#[test]
#[should_panic(expected = "expect: non empty stack")]
fn pop_panic() {
    let mut s = Stack::<u32>::new();
    s.pop();
}

#[test]
fn top() {
    let s = Stack::from(["one", "two", "three"]);
    assert_eq!(s.top(), &"three");
}

#[test]
#[should_panic(expected = "expect: non empty stack")]
fn top_panic() {
    let s = Stack::<u32>::new();
    s.top();
}

#[test]
fn iter() {
    let mut s = Stack::<String>::new();
    for i in 0..100000 {
        s.push(format!("value: {}", i));
    }
    let mut i = 0;
    for v in s.iter() {
        assert_eq!(v, &format!("value: {}", i));
        i += 1;
    }
}

#[test]
fn from_array() {
    let mut s = Stack::from([
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ]);
    assert_eq!(s.top(), &"item: 2".to_string());
    for i in (0..3).rev() {
        assert_eq!(s.pop(), format!("item: {}", i));
    }
    assert_eq!(s.size(), 0);
}

#[test]
fn from_iter() {
    let iter = [
        "item: 0".to_string(),
        "item: 1".to_string(),
        "item: 2".to_string(),
    ]
    .into_iter();
    let mut s = Stack::from_iter(iter);
    assert_eq!(s.top(), &"item: 2".to_string());
    for i in (0..3).rev() {
        assert_eq!(s.pop(), format!("item: {}", i));
    }
    assert_eq!(s.size(), 0);
}

#[test]
fn clone() {
    let s0 = sample();
    let s1 = s0.clone();
    assert_eq!(s0.size(), s1.size());
    assert_eq!(s1.top(), s0.top());
    for i in 0..s0.size() {
        assert_eq!(s0.get(i), s1.get(i));
    }
}

#[test]
fn clear() {
    let mut s = sample();
    s.clear();
}

fn sample() -> Stack<String> {
    let mut s = Stack::new();
    for i in 0..100000 {
        let item = format!("item: {}", i);
        s.push(item);
    }
    return s;
}
