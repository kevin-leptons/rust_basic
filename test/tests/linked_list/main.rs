mod helper;

use rust_basic::{linked_list::LinkedList, Vector};
use testkit::NonZeroSize;

#[test]
fn new() {
    let list = LinkedList::<NonZeroSize>::new();
    assert_eq!(list.size(), 0);
}

#[test]
fn insert() {
    let mut list = LinkedList::new();
    list.insert(0, NonZeroSize::new(1));
    list.insert(1, NonZeroSize::new(3));
    list.insert(2, NonZeroSize::new(4));
    list.insert(0, NonZeroSize::new(0));
    list.insert(2, NonZeroSize::new(2));
    list.insert(5, NonZeroSize::new(5));
    assert_eq!(list.size(), 6);
    for i in 0..list.size() {
        assert_eq!(list[i], NonZeroSize::new(i));
    }
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn insert_panic_empty() {
    let mut list = LinkedList::<NonZeroSize>::new();
    list.insert(1, NonZeroSize::new(1));
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn insert_panic() {
    let mut list = LinkedList::<NonZeroSize>::new();
    list.insert(0, NonZeroSize::new(0));
    list.insert(2, NonZeroSize::new(2));
}

#[test]
fn index() {
    let list = helper::sample_tiny();
    for i in 0..list.size() {
        assert_eq!(list[i], NonZeroSize::new(i));
    }
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn index_panic() {
    let list = helper::sample_tiny();
    let _ = &list[list.size()];
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn index_panic_empty() {
    let list = LinkedList::<NonZeroSize>::new();
    let _ = &list[0];
}

#[test]
fn index_mut() {
    let mut list = helper::sample_tiny();
    let size = list.size();
    for i in 0..size {
        list[i].value = size + i;
        assert_eq!(list[i].value, size + i);
    }
    for i in 0..size {
        assert_eq!(list[i].value, size + i);
    }
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn index_mut_panic() {
    let mut list = helper::sample_tiny();
    let index = list.size();
    let _ = &mut list[index];
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn index_mut_panic_empty() {
    let mut list = LinkedList::<NonZeroSize>::new();
    let _ = &mut list[0];
}

#[test]
fn push_front() {
    let mut list = helper::sample_small();
    let size = list.size();
    let push_size = 10;
    let back_item = NonZeroSize::new(size - 1);
    for i in 0..push_size {
        let item = NonZeroSize::new(size + i);
        list.push_front(item.clone());
        assert_eq!(list[0], item);
        assert_eq!(list[size + i], back_item);
        assert_eq!(list.size(), size + i + 1);
    }
}

#[test]
fn push_back() {
    let mut list = helper::sample_small();
    let size = list.size();
    let push_size = 10;
    let front_item = NonZeroSize::new(0);
    for i in 0..push_size {
        let item = NonZeroSize::new(size + i);
        list.push_back(item.clone());
        assert_eq!(list[size + i], item);
        assert_eq!(list[0], front_item);
        assert_eq!(list.size(), size + i + 1);
    }
}

#[test]
fn from_iter() {
    let array = [
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
        NonZeroSize::new(3),
        NonZeroSize::new(4),
    ];
    let list = LinkedList::from_iter(array.clone());
    assert_eq!(list.size(), array.len());
    for i in 0..array.len() {
        assert_eq!(list[i], array[i]);
    }
}

#[test]
fn from_array() {
    let array = [
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
        NonZeroSize::new(3),
        NonZeroSize::new(4),
    ];
    let list = LinkedList::from(array.clone());
    assert_eq!(list.size(), array.len());
    for i in 0..array.len() {
        assert_eq!(list[i], array[i]);
    }
}

#[test]
fn iter() {
    let list = helper::sample_small();
    let items = list.iter().map(|item| item.clone()).collect::<Vector<_>>();
    let expected = (0..list.size())
        .map(|i| NonZeroSize::new(i))
        .collect::<Vector<_>>();
    assert_eq!(items, expected);
}

#[test]
fn iter_mut() {
    let mut list = helper::sample_tiny();
    let size = list.size();
    let mut i = 0;
    for item in list.iter_mut() {
        item.value = size + i;
        i += 1;
    }
    assert_eq!(list.size(), size);
    for i in 0..size {
        assert_eq!(list[i], NonZeroSize::new(size + i));
    }
}

#[test]
fn front() {
    let list = helper::sample_small();
    assert_eq!(list.front(), &NonZeroSize::new(0));
}

#[test]
#[should_panic(expected = "expect: not empty list")]
fn front_panic() {
    let list = LinkedList::<NonZeroSize>::new();
    list.front();
}

#[test]
fn back() {
    let list = helper::sample_small();
    assert_eq!(list.back(), &NonZeroSize::new(list.size() - 1));
}

#[test]
#[should_panic(expected = "expect: not empty list")]
fn back_panic() {
    let list = LinkedList::<NonZeroSize>::new();
    list.back();
}

#[test]
fn remove() {
    let array = [
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(0),
        NonZeroSize::new(3),
        NonZeroSize::new(4),
    ];
    let mut list = LinkedList::from(array.clone());
    assert_eq!(list.remove(0), array[0]);
    assert_eq!(list.size(), 4);
    assert_eq!(list.remove(3), array[4]);
    assert_eq!(list.size(), 3);
    assert_eq!(list.remove(1), array[2]);
    assert_eq!(list.size(), 2);
    assert_eq!(list.remove(0), array[1]);
    assert_eq!(list.size(), 1);
    assert_eq!(list.remove(0), array[3]);
    assert_eq!(list.size(), 0);
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn remove_panic() {
    let mut list = helper::sample_small();
    list.remove(list.size());
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn remove_panic_empty() {
    let mut list = LinkedList::<NonZeroSize>::new();
    list.remove(0);
}

#[test]
fn pop_front() {
    let mut list = helper::sample_small();
    let size = list.size();
    for i in 0..size {
        assert_eq!(list.size(), size - i);
        assert_eq!(list.pop_front(), NonZeroSize::new(i));
    }
    assert_eq!(list.size(), 0);
}

#[test]
#[should_panic(expected = "expect: not empty list")]
fn pop_front_panic() {
    let mut list = LinkedList::<NonZeroSize>::new();
    list.pop_front();
}

#[test]
fn pop_back() {
    let mut list = helper::sample_small();
    let size = list.size();
    for i in 0..size {
        assert_eq!(list.size(), size - i);
        assert_eq!(list.pop_back(), NonZeroSize::new(size - i - 1));
    }
    assert_eq!(list.size(), 0);
}

#[test]
#[should_panic(expected = "expect: not empty list")]
fn pop_back_panic() {
    let mut list = LinkedList::<NonZeroSize>::new();
    list.pop_back();
}

#[test]
fn equal_true() {
    let list0 = helper::sample_small();
    let list1 = helper::sample_small();
    assert_eq!(list0, list1);
}

#[test]
fn not_false_equal_item() {
    let list0 = LinkedList::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
    ]);
    let list1 = LinkedList::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(9),
    ]);
    assert_ne!(list0, list1);
}

#[test]
fn not_false_equal_size() {
    let list0 = LinkedList::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
        NonZeroSize::new(3),
    ]);
    let list1 = LinkedList::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
    ]);
    assert_ne!(list0, list1);
}

#[test]
fn clone() {
    let list0 = helper::sample_small();
    let list1 = list0.clone();
    assert_eq!(list0, list1);
}

#[test]
fn clear() {
    let mut list = helper::sample_small();
    list.clear();
    assert_eq!(list.size(), 0);
    list.clear();
}

#[test]
fn clear_empty() {
    let mut list = LinkedList::<NonZeroSize>::new();
    list.clear();
    assert_eq!(list.size(), 0);
}

// This test does nothing but creating a non empty container to trigger memory
// release process. The test can not work alone, it requries an external tool
// such as Valgrind to diagnose memory issues.
//
// Warn: The test maybe still passed even memory release process has issues.
#[test]
fn drop() {
    let _ = helper::sample_small();
}

#[test]
fn drop_empty() {
    let _ = LinkedList::<NonZeroSize>::new();
}

#[test]
fn sample_must_not_empty() {
    assert!(helper::sample_small().size() > 0);
    assert!(helper::sample_tiny().size() > 0);
}
