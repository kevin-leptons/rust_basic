use crate::Vector;

/// A variant of quick sort: Hoare partition scheme.
pub(super) fn sort<T>(vector: &mut Vector<T>)
where
    T: Ord,
{
    if vector.size <= 1 {
        return;
    }
    sort_slice(vector, 0, vector.size - 1);
}

pub(super) fn sort_slice<T>(vector: &mut Vector<T>, begin: usize, end: usize)
where
    T: Ord,
{
    if begin >= end {
        return;
    }
    let p = partition(vector, begin, end);
    sort_slice(vector, begin, p);
    sort_slice(vector, p + 1, end);
}

/// The returned value called `p` that determine:
/// * Slice `[begin, p]`, where value at `p` is greatest.
/// * Slice `[p + 1, end]`, where value at `p + 1` is smallest if it's existed.
/// * `p` is in `[begin, end]`.
fn partition<T>(vector: &mut Vector<T>, begin: usize, end: usize) -> usize
where
    T: Ord,
{
    let mut p = begin + (end - begin) / 2;
    let mut left = begin;
    let mut right = end;
    loop {
        loop {
            if (left > end) || (vector[left] >= vector[p]) {
                break;
            }
            left += 1;
        }
        loop {
            if (right < begin) || (vector[right] <= vector[p]) {
                break;
            }
            right -= 1;
        }
        if left >= right {
            return right;
        }
        vector.swap(left, right);
        if p == left {
            p = right;
        } else if p == right {
            p = left;
        }
        left += 1;
        right -= 1;
    }
}
