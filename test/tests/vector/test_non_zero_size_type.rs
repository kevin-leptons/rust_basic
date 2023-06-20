use crate::sample;
use rust_basic::Vector;
use testkit::NonZeroSize;

#[test]
fn new() {
    let vector = Vector::<NonZeroSize>::new();
    assert_eq!(vector.size(), 0);
}

#[test]
fn insert() {
    let mut vector = Vector::new();
    vector.insert(0, NonZeroSize::new(3));
    vector.insert(0, NonZeroSize::new(1));
    vector.insert(0, NonZeroSize::new(0));
    vector.insert(3, NonZeroSize::new(5));
    vector.insert(3, NonZeroSize::new(4));
    vector.insert(2, NonZeroSize::new(2));
    let size = vector.size();
    assert_eq!(size, 6);
    for i in 0..size {
        assert_eq!(vector[i], NonZeroSize::new(i));
    }
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn insert_panic() {
    let mut vector = Vector::new();
    vector.insert(0, NonZeroSize::new(0));
    vector.insert(0, NonZeroSize::new(1));
    vector.insert(3, NonZeroSize::new(2));
}

#[test]
fn from_iter() {
    let array = [
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
    ];
    let vector = Vector::from_iter(array.clone());
    assert_eq!(vector.size(), array.len());
    for i in 0..array.len() {
        assert_eq!(vector[i], array[i]);
    }
}

#[test]
fn from_array() {
    let array = [
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
    ];
    let vector = Vector::from(array.clone());
    assert_eq!(vector.size(), array.len());
    for i in 0..array.len() {
        assert_eq!(vector[i], array[i]);
    }
}

#[test]
#[should_panic(expected = "valid index")]
fn index_panic() {
    let vector = Vector::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
    ]);
    let _ = &vector[vector.size()];
}

#[test]
fn index_mut() {
    let mut vector = Vector::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
    ]);
    let size = vector.size();
    for i in 0..size {
        let item = &mut vector[i];
        assert_eq!(item, &NonZeroSize::new(i));
        vector[i].value = i + 100;
    }
    assert_eq!(vector.size(), size);
}

#[test]
fn index_mut_set() {
    let mut vector = Vector::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
    ]);
    let size = vector.size();
    for i in 0..size {
        assert_eq!(&mut vector[i], &NonZeroSize::new(i));
        let new_item = NonZeroSize::new(i + 100);
        vector[i] = new_item.clone();
        assert_eq!(vector[i], new_item);
    }
    assert_eq!(vector.size(), size);
}

#[test]
#[should_panic(expected = "valid index")]
fn index_mut_panic() {
    let mut vector = sample::non_zero_size_type();
    let size = vector.size();
    let _ = &mut vector[size];
}

#[test]
fn equal_true() {
    let vector0 = sample::non_zero_size_type();
    let vector1 = sample::non_zero_size_type();
    assert_eq!(vector0, vector1);
}

#[test]
fn equal_false_item() {
    let vector0 = Vector::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
        NonZeroSize::new(3),
    ]);
    let vector1 = Vector::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
        NonZeroSize::new(99),
    ]);
    assert_ne!(vector0, vector1);
}

#[test]
fn equal_false_size() {
    let vector0 = Vector::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
        NonZeroSize::new(3),
    ]);
    let vector1 = Vector::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
    ]);
    assert_ne!(vector0, vector1);
}

#[test]
fn remove_front() {
    let mut vector = Vector::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
        NonZeroSize::new(3),
    ]);
    assert_eq!(vector.remove(0), NonZeroSize::new(0));
    assert_eq!(
        vector,
        Vector::from([
            NonZeroSize::new(1),
            NonZeroSize::new(2),
            NonZeroSize::new(3),
        ])
    );
}

#[test]
fn remove_back() {
    let mut vector = Vector::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
        NonZeroSize::new(3),
    ]);
    assert_eq!(vector.remove(3), NonZeroSize::new(3));
    assert_eq!(
        vector,
        Vector::from([
            NonZeroSize::new(0),
            NonZeroSize::new(1),
            NonZeroSize::new(2),
        ])
    );
}

#[test]
fn remove_middle() {
    let mut vector = Vector::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
        NonZeroSize::new(3),
    ]);
    assert_eq!(vector.remove(1), NonZeroSize::new(1));
    assert_eq!(
        vector,
        Vector::from([
            NonZeroSize::new(0),
            NonZeroSize::new(2),
            NonZeroSize::new(3),
        ])
    );
}

#[test]
fn remove_front_all() {
    let mut vector = Vector::new();
    let mut size = 0;
    let round = 100;
    for i in 0..round {
        let push_size = 10 * i;
        for k in 0..push_size {
            let item = NonZeroSize::new((i, k));
            vector.insert(0, item.clone());
            assert_eq!(vector[0], item);
            assert_eq!(vector.size(), size + 1);
            size += 1;
        }
        for k in 0..push_size {
            let front = NonZeroSize::new((i, push_size - k - 1));
            assert_eq!(vector[0], front);
            assert_eq!(vector.remove(0), front);
            assert_eq!(vector.size(), size - 1);
            size -= 1;
        }
    }
    assert_eq!(size, 0);
    assert_eq!(vector.size(), 0);
}

#[test]
fn remove_front_half() {
    let mut vector = Vector::new();
    let mut size = 0;
    let round = 32;
    for i in 0..round {
        let push_size = 10 * i;
        for k in 0..push_size {
            let item = NonZeroSize::new((i, k));
            vector.insert(0, item.clone());
            assert_eq!(vector[0], item);
            assert_eq!(vector.size(), size + 1);
            size += 1;
        }
        for k in 0..(push_size / 2) {
            let front = NonZeroSize::new((i, push_size - k - 1));
            assert_eq!(vector[0], front);
            assert_eq!(vector.remove(0), front);
            assert_eq!(vector.size(), size - 1);
            size -= 1;
        }
    }
    assert!(size > 0);
    assert_eq!(vector.size(), size);
    for _ in 0..size {
        vector.remove(0);
        size -= 1;
        assert_eq!(vector.size(), size);
    }
    assert_eq!(vector.size(), 0);
}

#[test]
fn remove_back_all() {
    let mut vector = Vector::new();
    let mut size = 0;
    let round = 100;
    for i in 0..round {
        let push_size = 10 * i;
        for k in 0..push_size {
            let item = NonZeroSize::new((i, k));
            vector.insert(vector.size(), item.clone());
            assert_eq!(vector[vector.size() - 1], item);
            assert_eq!(vector.size(), size + 1);
            size += 1;
        }
        for k in 0..push_size {
            let back = NonZeroSize::new((i, push_size - k - 1));
            assert_eq!(vector[vector.size() - 1], back);
            assert_eq!(vector.remove(vector.size() - 1), back);
            assert_eq!(vector.size(), size - 1);
            size -= 1;
        }
    }
    assert_eq!(size, 0);
    assert_eq!(vector.size(), 0);
}

#[test]
fn remove_back_half() {
    let mut vector = Vector::new();
    let mut size = 0;
    let round = 32;
    for i in 0..round {
        let push_size = 10 * i;
        for k in 0..push_size {
            let item = NonZeroSize::new((i, k));
            vector.insert(vector.size(), item.clone());
            assert_eq!(vector[vector.size() - 1], item);
            assert_eq!(vector.size(), size + 1);
            size += 1;
        }
        for k in 0..(push_size / 2) {
            let back = NonZeroSize::new((i, push_size - k - 1));
            assert_eq!(vector[vector.size() - 1], back);
            assert_eq!(vector.remove(vector.size() - 1), back);
            assert_eq!(vector.size(), size - 1);
            size -= 1;
        }
    }
    assert!(size > 0);
    assert_eq!(size, vector.size());
    for _ in 0..size {
        vector.remove(vector.size() - 1);
        size -= 1;
        assert_eq!(vector.size(), size);
    }
    assert_eq!(vector.size(), 0);
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn remove_panic() {
    let mut vector = sample::non_zero_size_type();
    vector.remove(vector.size());
}

#[test]
fn swap_front_middle() {
    let mut vector = Vector::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
        NonZeroSize::new(3),
    ]);
    vector.swap(0, 2);
    assert_eq!(
        vector,
        Vector::from([
            NonZeroSize::new(2),
            NonZeroSize::new(1),
            NonZeroSize::new(0),
            NonZeroSize::new(3),
        ])
    );
}

#[test]
fn swap_front_back() {
    let mut vector = Vector::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
        NonZeroSize::new(3),
    ]);
    vector.swap(0, 3);
    assert_eq!(
        vector,
        Vector::from([
            NonZeroSize::new(3),
            NonZeroSize::new(1),
            NonZeroSize::new(2),
            NonZeroSize::new(0),
        ])
    );
}

#[test]
fn swap_back_middle() {
    let mut vector = Vector::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
        NonZeroSize::new(3),
    ]);
    vector.swap(1, 3);
    assert_eq!(
        vector,
        Vector::from([
            NonZeroSize::new(0),
            NonZeroSize::new(3),
            NonZeroSize::new(2),
            NonZeroSize::new(1),
        ])
    );
}

#[test]
fn swap_middle_middle() {
    let mut vector = Vector::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
        NonZeroSize::new(3),
    ]);
    vector.swap(1, 2);
    assert_eq!(
        vector,
        Vector::from([
            NonZeroSize::new(0),
            NonZeroSize::new(2),
            NonZeroSize::new(1),
            NonZeroSize::new(3),
        ])
    );
}

#[test]
fn swap_front_itself() {
    let mut vector = Vector::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
        NonZeroSize::new(3),
    ]);
    vector.swap(0, 0);
    assert_eq!(
        vector,
        Vector::from([
            NonZeroSize::new(0),
            NonZeroSize::new(1),
            NonZeroSize::new(2),
            NonZeroSize::new(3),
        ])
    );
}

#[test]
fn swap_middle_itself() {
    let mut vector = Vector::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
        NonZeroSize::new(3),
    ]);
    vector.swap(1, 1);
    assert_eq!(
        vector,
        Vector::from([
            NonZeroSize::new(0),
            NonZeroSize::new(1),
            NonZeroSize::new(2),
            NonZeroSize::new(3),
        ])
    );
}

#[test]
fn swap_back_itself() {
    let mut vector = Vector::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
        NonZeroSize::new(3),
    ]);
    vector.swap(3, 3);
    assert_eq!(
        vector,
        Vector::from([
            NonZeroSize::new(0),
            NonZeroSize::new(1),
            NonZeroSize::new(2),
            NonZeroSize::new(3),
        ])
    );
}

#[test]
#[should_panic(expected = "expect: valid indexes")]
fn swap_panic_first_argument() {
    let mut vector = Vector::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
    ]);
    vector.swap(3, 1);
}

#[test]
#[should_panic(expected = "expect: valid indexes")]
fn swap_panic_second_argument() {
    let mut vector = Vector::from([
        NonZeroSize::new(0),
        NonZeroSize::new(1),
        NonZeroSize::new(2),
    ]);
    vector.swap(1, 3);
}

#[test]
fn iter() {
    let vector = sample::non_zero_size_type();
    let size = vector.size();
    let mut count = 0;
    for item in vector.iter() {
        assert_eq!(item, &NonZeroSize::new(count));
        count += 1;
    }
    assert_eq!(count, size);
    assert_eq!(vector.size(), size);
}

#[test]
fn iter_next_none() {
    let vector = Vector::<NonZeroSize>::new();
    assert_eq!(vector.iter().next(), None);
}

#[test]
fn iter_mut() {
    let mut vector = sample::non_zero_size_type();
    let size = vector.size();
    let mut count = 0;
    for item in vector.iter_mut() {
        assert_eq!(item, &NonZeroSize::new(count));
        item.value = size + 100 + count;
        count += 1;
    }
    assert_eq!(count, size);
    assert_eq!(vector.size(), size);
    for i in 0..size {
        assert_eq!(vector[i], NonZeroSize::new(size + 100 + i));
    }
}

#[test]
fn iter_mut_next_none() {
    let mut vector = Vector::<NonZeroSize>::new();
    assert_eq!(vector.iter_mut().next(), None);
}

#[test]
fn iter_back() {
    let vector = sample::non_zero_size_type();
    let size = vector.size();
    let mut count = 0;
    for item in vector.iter().rev() {
        assert_eq!(item, &NonZeroSize::new(size - count - 1));
        count += 1;
    }
    assert_eq!(count, size);
    assert_eq!(vector.size(), size);
}

#[test]
fn iter_mut_back() {
    let mut vector = sample::non_zero_size_type();
    let size = vector.size();
    let mut count = 0;
    for item in vector.iter_mut().rev() {
        assert_eq!(item, &NonZeroSize::new(size - count - 1));
        item.value = size + 100 + (size - count - 1);
        count += 1;
    }
    assert_eq!(count, size);
    assert_eq!(vector.size(), size);
    for i in 0..size {
        assert_eq!(vector[i], NonZeroSize::new(size + 100 + i));
    }
}

#[test]
fn into_iter() {
    let vector = sample::non_zero_size_type();
    let size = vector.size();
    let mut count = 0;
    for item in vector.into_iter() {
        assert_eq!(item, NonZeroSize::new(count));
        count += 1;
    }
    assert_eq!(count, size);
}

#[test]
fn sort_insertion_empty() {
    let mut vector = Vector::<NonZeroSize>::new();
    vector.sort_insertion();
    assert_eq!(vector, Vector::new());
}

#[test]
fn sort_insertion_one() {
    let mut vector = Vector::from([NonZeroSize::new(1)]);
    vector.sort_insertion();
    assert_eq!(vector, Vector::from([NonZeroSize::new(1)]));
}

#[test]
fn sort_insertion() {
    let mut vector = Vector::from([
        NonZeroSize::new(2),
        NonZeroSize::new(7),
        NonZeroSize::new(9),
        NonZeroSize::new(0),
        NonZeroSize::new(8),
        NonZeroSize::new(1),
        NonZeroSize::new(3),
        NonZeroSize::new(5),
        NonZeroSize::new(4),
        NonZeroSize::new(6),
        NonZeroSize::new(10),
    ]);
    vector.sort_insertion();
    assert_eq!(
        vector,
        Vector::from([
            NonZeroSize::new(0),
            NonZeroSize::new(1),
            NonZeroSize::new(2),
            NonZeroSize::new(3),
            NonZeroSize::new(4),
            NonZeroSize::new(5),
            NonZeroSize::new(6),
            NonZeroSize::new(7),
            NonZeroSize::new(8),
            NonZeroSize::new(9),
            NonZeroSize::new(10),
        ])
    )
}

#[test]
fn sort_selection_empty() {
    let mut vector = Vector::<NonZeroSize>::new();
    vector.sort_selection();
    assert_eq!(vector, Vector::new());
}

#[test]
fn sort_selection_one() {
    let mut vector = Vector::from([NonZeroSize::new(1)]);
    vector.sort_selection();
    assert_eq!(vector, Vector::from([NonZeroSize::new(1)]));
}

#[test]
fn sort_selection() {
    let mut vector = Vector::from([
        NonZeroSize::new(2),
        NonZeroSize::new(7),
        NonZeroSize::new(9),
        NonZeroSize::new(0),
        NonZeroSize::new(8),
        NonZeroSize::new(1),
        NonZeroSize::new(3),
        NonZeroSize::new(5),
        NonZeroSize::new(4),
        NonZeroSize::new(6),
        NonZeroSize::new(10),
    ]);
    vector.sort_selection();
    assert_eq!(
        vector,
        Vector::from([
            NonZeroSize::new(0),
            NonZeroSize::new(1),
            NonZeroSize::new(2),
            NonZeroSize::new(3),
            NonZeroSize::new(4),
            NonZeroSize::new(5),
            NonZeroSize::new(6),
            NonZeroSize::new(7),
            NonZeroSize::new(8),
            NonZeroSize::new(9),
            NonZeroSize::new(10),
        ])
    )
}

#[test]
fn sort_merge() {
    let mut vector = Vector::from([
        NonZeroSize::new(2),
        NonZeroSize::new(7),
        NonZeroSize::new(9),
        NonZeroSize::new(0),
        NonZeroSize::new(8),
        NonZeroSize::new(1),
        NonZeroSize::new(3),
        NonZeroSize::new(5),
        NonZeroSize::new(4),
        NonZeroSize::new(6),
        NonZeroSize::new(10),
    ]);
    vector.sort_merge();
    assert_eq!(
        vector,
        Vector::from([
            NonZeroSize::new(0),
            NonZeroSize::new(1),
            NonZeroSize::new(2),
            NonZeroSize::new(3),
            NonZeroSize::new(4),
            NonZeroSize::new(5),
            NonZeroSize::new(6),
            NonZeroSize::new(7),
            NonZeroSize::new(8),
            NonZeroSize::new(9),
            NonZeroSize::new(10),
        ])
    )
}

#[test]
fn sort_merge_empty() {
    let mut vector = Vector::<NonZeroSize>::new();
    vector.sort_merge();
    assert_eq!(vector, Vector::new());
}

#[test]
fn sort_merge_one() {
    let mut vector = Vector::from([NonZeroSize::new(1)]);
    vector.sort_merge();
    assert_eq!(vector, Vector::from([NonZeroSize::new(1)]));
}

#[test]
fn sort_quick_empty() {
    let mut vector = Vector::<NonZeroSize>::new();
    vector.sort_quick();
    assert_eq!(vector, Vector::new());
}

#[test]
fn sort_quick_one() {
    let mut vector = Vector::from([NonZeroSize::new(1)]);
    vector.sort_quick();
    assert_eq!(vector, Vector::from([NonZeroSize::new(1)]));
}

#[test]
fn sort_quick() {
    let mut vector = Vector::from([
        NonZeroSize::new(2),
        NonZeroSize::new(7),
        NonZeroSize::new(9),
        NonZeroSize::new(0),
        NonZeroSize::new(8),
        NonZeroSize::new(1),
        NonZeroSize::new(3),
        NonZeroSize::new(5),
        NonZeroSize::new(4),
        NonZeroSize::new(6),
        NonZeroSize::new(10),
    ]);
    vector.sort_quick();
    assert_eq!(
        vector,
        Vector::from([
            NonZeroSize::new(0),
            NonZeroSize::new(1),
            NonZeroSize::new(2),
            NonZeroSize::new(3),
            NonZeroSize::new(4),
            NonZeroSize::new(5),
            NonZeroSize::new(6),
            NonZeroSize::new(7),
            NonZeroSize::new(8),
            NonZeroSize::new(9),
            NonZeroSize::new(10),
        ])
    )
}

#[test]
fn sort_empty() {
    let mut vector = Vector::<NonZeroSize>::new();
    vector.sort();
    assert_eq!(vector, Vector::new());
}

#[test]
fn sort_one() {
    let mut vector = Vector::from([NonZeroSize::new(1)]);
    vector.sort();
    assert_eq!(vector, Vector::from([NonZeroSize::new(1)]));
}

#[test]
fn sort() {
    let mut vector = Vector::from([
        NonZeroSize::new(2),
        NonZeroSize::new(7),
        NonZeroSize::new(9),
        NonZeroSize::new(0),
        NonZeroSize::new(8),
        NonZeroSize::new(1),
        NonZeroSize::new(3),
        NonZeroSize::new(5),
        NonZeroSize::new(4),
        NonZeroSize::new(6),
        NonZeroSize::new(10),
    ]);
    vector.sort();
    assert_eq!(
        vector,
        Vector::from([
            NonZeroSize::new(0),
            NonZeroSize::new(1),
            NonZeroSize::new(2),
            NonZeroSize::new(3),
            NonZeroSize::new(4),
            NonZeroSize::new(5),
            NonZeroSize::new(6),
            NonZeroSize::new(7),
            NonZeroSize::new(8),
            NonZeroSize::new(9),
            NonZeroSize::new(10),
        ])
    )
}

#[test]
fn clone() {
    let vector0 = sample::non_zero_size_type();
    let vector1 = vector0.clone();
    assert_eq!(vector0, vector1);
}

#[test]
fn clear() {
    let mut vector = sample::non_zero_size_type();
    vector.clear();
    assert_eq!(vector.size(), 0);
}

// This test does nothing but creating a non empty container to trigger memory
// release process. The test can not work alone, it requries an external tool
// such as Valgrind to diagnose memory issues.
//
// Warn: The test maybe still passed even memory release process has issues.
#[test]
fn drop() {
    let _ = sample::non_zero_size_type();
}

#[test]
fn sample_must_not_empty() {
    let stack = sample::non_zero_size_type();
    assert!(stack.size() > 0);
}
