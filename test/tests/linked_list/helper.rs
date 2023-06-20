use rust_basic::LinkedList;
use testkit::NonZeroSize;

pub(super) fn sample_small() -> LinkedList<NonZeroSize> {
    let mut list = LinkedList::new();
    for i in 0..10000 {
        list.insert(list.size(), NonZeroSize::new(i));
    }
    return list;
}

pub(super) fn sample_tiny() -> LinkedList<NonZeroSize> {
    let mut list = LinkedList::new();
    for i in 0..100 {
        list.insert(list.size(), NonZeroSize::new(i));
    }
    return list;
}
