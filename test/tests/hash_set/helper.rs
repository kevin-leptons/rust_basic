use rust_basic::HashSet;
use testkit::NonZeroSize;

pub(super) fn non_zero_size_type() -> HashSet<NonZeroSize> {
    let mut set = HashSet::new();
    for i in 0..10000 {
        set.add(NonZeroSize::new(i));
    }
    return set;
}
