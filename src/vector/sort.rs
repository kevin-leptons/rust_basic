use crate::Vector;

pub(super) fn sort_merge<T>(v: &mut Vector<T>)
where
    T: Ord,
{
    if v.size <= 1 {
        return;
    }
    let mut v1 = Vector::<T>::new();
    let mut v2 = Vector::<T>::new();
    let border = v.size / 2;
    for _ in 0..border {
        v1.set(v1.size, v.remove(v.size - 1));
    }
    loop {
        if v.size == 0 {
            break;
        }
        v2.set(v2.size, v.remove(v.size - 1));
    }
    sort_merge(&mut v1);
    sort_merge(&mut v2);
    sort_merge_join(&mut v1, &mut v2, v);
}

fn sort_merge_join<T>(
    source1: &mut Vector<T>,
    source2: &mut Vector<T>,
    target: &mut Vector<T>,
) where
    T: Ord,
{
    loop {
        if source1.size() == 0 || source2.size() == 0 {
            break;
        }
        if source1.get(0) <= source2.get(0) {
            target.set(target.size, source1.remove(0));
        } else {
            target.set(target.size, source2.remove(0));
        }
    }
    for _ in 0..source1.size() {
        target.set(target.size, source1.remove(0));
    }
    for _ in 0..source2.size() {
        target.set(target.size, source2.remove(0));
    }
}

pub(super) fn sort_quick<T>(v: &mut Vector<T>, begin: usize, end: usize)
where
    T: Ord,
{
    assert!(begin <= end, "expect: begin <= end");
    if v.size() == 1 {
        return;
    }
    assert!(end < v.size(), "expectd: end is in range");
    if begin == end {
        return;
    }
    let p = sort_quick_partition(v, begin, end);
    sort_quick(v, begin, p);
    if p + 1 <= end {
        sort_quick(v, p + 1, end);
    }
}

/// The returned value called `p` that determine:
/// * Slice `[begin, p]`, where value at `p` is greatest.
/// * Slice `[p + 1, end]`, where value at `p + 1` is smallest if it's existed.
/// * `p` is in `[begin, end]`.
fn sort_quick_partition<T>(v: &mut Vector<T>, begin: usize, end: usize) -> usize
where
    T: Ord,
{
    assert!(begin <= end, "expect: begin <= end");
    assert!(v.size() > 0, "expect: vector has items");
    if v.size() == 1 {
        return 0;
    }
    assert!(end < v.size(), "expectd: end is in range");
    if begin == end {
        return begin;
    }
    let mut p = begin + (end - begin) / 2;
    let mut left = begin;
    let mut right = end;
    loop {
        for i in left..(end + 1) {
            if v.get(left) >= v.get(p) {
                break;
            }
            left = i;
        }
        for i in (begin..right + 1).rev() {
            if v.get(right) <= v.get(p) {
                break;
            }
            right = i;
        }
        if left >= right {
            return right;
        }
        v.swap(left, right);
        if p == left {
            p = right;
        } else if p == right {
            p = left;
        }
        left = left + 1;
        right = right - 1;
    }
}
