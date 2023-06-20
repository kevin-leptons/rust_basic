mod helper;

use rust_basic::{HashMap, Vector};
use testkit::NonZeroSize;

#[test]
fn new() {
    let map = HashMap::<NonZeroSize, NonZeroSize>::new();
    assert_eq!(map.size(), 0);
}

#[test]
fn set() {
    let mut map = HashMap::new();
    let size = 10000;
    for i in 0..size {
        let key = NonZeroSize::new(i);
        let value = NonZeroSize::new(("value", i));
        assert_eq!(map.set(key.clone(), value.clone()), None);
        assert_eq!(map.get(&key), Some(&value));
        assert_eq!(map.has(&key), true);
        assert_eq!(map.size(), i + 1);
    }
    for i in 0..size {
        let key = NonZeroSize::new(i);
        let value = NonZeroSize::new(("value", i));
        assert_eq!(map.get(&key), Some(&value));
    }
}

#[test]
fn set_return_old_value() {
    let mut map = HashMap::new();
    map.set(NonZeroSize::new(0), NonZeroSize::new(("value", 0)));
    map.set(NonZeroSize::new(1), NonZeroSize::new(("value", 1)));
    map.set(NonZeroSize::new(2), NonZeroSize::new(("value", 2)));
    assert_eq!(map.size(), 3);
    let key = NonZeroSize::new(0);
    let size = map.size();
    let times = 100;
    for i in 1..times {
        let old_value = NonZeroSize::new(("value", i - 1));
        let new_value = NonZeroSize::new(("value", i));
        assert_eq!(map.set(key.clone(), new_value), Some(old_value));
        assert_eq!(map.size(), size);
    }
}

#[test]
fn get_return_none() {
    let mut map = HashMap::new();
    map.set(NonZeroSize::new(0), NonZeroSize::new(("value", 0)));
    map.set(NonZeroSize::new(1), NonZeroSize::new(("value", 1)));
    map.set(NonZeroSize::new(2), NonZeroSize::new(("value", 2)));
    assert_eq!(map.get(&NonZeroSize::new(3)), None);
    assert_eq!(map.get(&NonZeroSize::new(4)), None);
}

#[test]
fn get_mut() {
    let array = [
        (NonZeroSize::new(0), NonZeroSize::new(("value", 0))),
        (NonZeroSize::new(1), NonZeroSize::new(("value", 1))),
        (NonZeroSize::new(2), NonZeroSize::new(("value", 2))),
    ];
    let mut map = HashMap::new();
    for (key, value) in array.iter() {
        map.set(key.clone(), value.clone());
    }
    for i in 0..array.len() {
        let key = NonZeroSize::new(i);
        let value = map.get_mut(&key).unwrap();
        assert_eq!(value, &NonZeroSize::new(("value", i)));
        value.value = ("new value", i);
    }
    for i in 0..array.len() {
        let key = NonZeroSize::new(i);
        let value = map.get_mut(&key).unwrap();
        assert_eq!(value, &NonZeroSize::new(("new value", i)));
    }
}

#[test]
fn get_mut_return_none() {
    let mut map = HashMap::new();
    map.set(NonZeroSize::new(0), NonZeroSize::new(("value", 0)));
    map.set(NonZeroSize::new(1), NonZeroSize::new(("value", 1)));
    map.set(NonZeroSize::new(2), NonZeroSize::new(("value", 2)));
    assert_eq!(map.get_mut(&NonZeroSize::new(3)), None);
    assert_eq!(map.get_mut(&NonZeroSize::new(4)), None);
}

#[test]
fn has_return_false() {
    let mut map = HashMap::new();
    map.set(NonZeroSize::new(0), NonZeroSize::new(("value", 0)));
    map.set(NonZeroSize::new(1), NonZeroSize::new(("value", 1)));
    map.set(NonZeroSize::new(2), NonZeroSize::new(("value", 2)));
    assert_eq!(map.has(&NonZeroSize::new(3)), false);
    assert_eq!(map.has(&NonZeroSize::new(4)), false);
}

#[test]
fn remove_return_some() {
    let mut map = helper::sample();
    let size = map.size();
    assert!(size > 0);
    for i in 0..size {
        let key = NonZeroSize::new(i);
        let value = NonZeroSize::new(("value", i));
        assert_eq!(map.remove(&key), Some(value));
        assert_eq!(map.has(&key), false);
        assert_eq!(map.size(), size - i - 1);
    }
}

#[test]
fn remove_return_none() {
    let mut map = HashMap::new();
    map.set(NonZeroSize::new(0), NonZeroSize::new(("value", 0)));
    map.set(NonZeroSize::new(1), NonZeroSize::new(("value", 1)));
    map.set(NonZeroSize::new(2), NonZeroSize::new(("value", 2)));
    assert_eq!(map.remove(&NonZeroSize::new(3)), None);
    assert_eq!(map.remove(&NonZeroSize::new(4)), None);
}

#[test]
fn keys() {
    let map = helper::sample();
    let mut keys = map.keys().map(|k| k.clone()).collect::<Vector<_>>();
    let mut expected = (0..map.size())
        .map(|i| NonZeroSize::new(i))
        .collect::<Vector<_>>();
    keys.sort();
    expected.sort();
    assert_eq!(keys, expected);
}

#[test]
fn keys_empty() {
    let map = HashMap::<NonZeroSize, NonZeroSize>::new();
    assert_eq!(map.keys().next(), None);
}

#[test]
fn values() {
    let map = helper::sample();
    let mut values = map.values().map(|v| v.clone()).collect::<Vector<_>>();
    let mut expected = (0..map.size())
        .map(|i| NonZeroSize::new(("value", i)))
        .collect::<Vector<_>>();
    values.sort();
    expected.sort();
    assert_eq!(values, expected);
}

#[test]
fn values_empty() {
    let map = HashMap::<NonZeroSize, NonZeroSize>::new();
    assert_eq!(map.values().next(), None);
}

#[test]
fn iter() {
    let map = helper::sample();
    let mut pairs = map
        .iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect::<Vector<_>>();
    let mut expected = (0..map.size())
        .map(|i| (NonZeroSize::new(i), NonZeroSize::new(("value", i))))
        .collect::<Vector<_>>();
    pairs.sort();
    expected.sort();
    assert_eq!(pairs, expected);
}

#[test]
fn iter_empty() {
    let map = HashMap::<NonZeroSize, NonZeroSize>::new();
    assert_eq!(map.iter().next(), None);
}

#[test]
fn equal_empty() {
    let map0 = HashMap::<NonZeroSize, NonZeroSize<(&str, usize)>>::new();
    let map1 = HashMap::<NonZeroSize, NonZeroSize<(&str, usize)>>::new();
    assert_eq!(map0, map1);
}

#[test]
fn equal_non_empty() {
    let map0 = helper::sample();
    let map1 = helper::sample();
    assert_eq!(map0, map1);
}

#[test]
fn not_equal_key() {
    let m0 = HashMap::from([
        (NonZeroSize::new(0), NonZeroSize::new(("value", 0))),
        (NonZeroSize::new(1), NonZeroSize::new(("value", 1))),
        (NonZeroSize::new(2), NonZeroSize::new(("value", 2))),
    ]);
    let m1 = HashMap::from([
        (NonZeroSize::new(0), NonZeroSize::new(("value", 0))),
        (NonZeroSize::new(1), NonZeroSize::new(("value", 1))),
        (NonZeroSize::new(9), NonZeroSize::new(("value", 2))),
    ]);
    assert_ne!(m0, m1);
}

#[test]
fn not_equal_value() {
    let m0 = HashMap::from([
        (NonZeroSize::new(0), NonZeroSize::new(("value", 0))),
        (NonZeroSize::new(1), NonZeroSize::new(("value", 1))),
        (NonZeroSize::new(2), NonZeroSize::new(("value", 2))),
    ]);
    let m1 = HashMap::from([
        (NonZeroSize::new(0), NonZeroSize::new(("value", 0))),
        (NonZeroSize::new(1), NonZeroSize::new(("value", 1))),
        (NonZeroSize::new(2), NonZeroSize::new(("value", 9))),
    ]);
    assert_ne!(m0, m1);
}

#[test]
fn not_equal_size() {
    let m0 = HashMap::from([
        (NonZeroSize::new(0), NonZeroSize::new(("value", 0))),
        (NonZeroSize::new(1), NonZeroSize::new(("value", 1))),
        (NonZeroSize::new(2), NonZeroSize::new(("value", 2))),
    ]);
    let m1 = HashMap::from([
        (NonZeroSize::new(0), NonZeroSize::new(("value", 0))),
        (NonZeroSize::new(1), NonZeroSize::new(("value", 1))),
    ]);
    assert_ne!(m0, m1);
}

#[test]
fn from_iter() {
    let array = [
        (NonZeroSize::new(0), NonZeroSize::new(("value", 0))),
        (NonZeroSize::new(1), NonZeroSize::new(("value", 1))),
        (NonZeroSize::new(2), NonZeroSize::new(("value", 2))),
    ];
    let map = HashMap::from_iter(array.clone());
    assert_eq!(map.size(), array.len());
    for i in 0..array.len() {
        let key = NonZeroSize::new(i);
        let value = NonZeroSize::new(("value", i));
        assert_eq!(map.get(&key), Some(&value));
    }
}

#[test]
fn from_array() {
    let array = [
        (NonZeroSize::new(0), NonZeroSize::new(("value", 0))),
        (NonZeroSize::new(1), NonZeroSize::new(("value", 1))),
        (NonZeroSize::new(2), NonZeroSize::new(("value", 2))),
    ];
    let map = HashMap::from(array.clone());
    for i in 0..array.len() {
        let key = NonZeroSize::new(i);
        let value = NonZeroSize::new(("value", i));
        assert_eq!(map.get(&key), Some(&value));
    }
}

#[test]
fn clone() {
    let array = [
        (NonZeroSize::new(0), NonZeroSize::new(("value", 0))),
        (NonZeroSize::new(1), NonZeroSize::new(("value", 1))),
        (NonZeroSize::new(2), NonZeroSize::new(("value", 2))),
    ];
    let map0 = HashMap::from(array.clone());
    let map1 = map0.clone();
    assert_eq!(map0, map1);
}

#[test]
fn clear() {
    let mut map = helper::sample();
    let size = map.size();
    map.clear();
    assert_eq!(map.size(), 0);
    for i in 0..size {
        let key = NonZeroSize::new(i);
        assert_eq!(map.get(&key), None);
        assert_eq!(map.get_mut(&key), None);
        assert_eq!(map.has(&key), false);
    }
}

// This test does nothing but creating a non empty container to trigger memory
// release process. The test can not work alone, it requries an external tool
// such as Valgrind to diagnose memory issues.
//
// Warn: The test maybe still passed even memory release process has issues.
#[test]
fn drop() {
    let _ = helper::sample();
}

#[test]
fn sample_must_not_empty() {
    let map = helper::sample();
    assert!(map.size() > 0);
}
