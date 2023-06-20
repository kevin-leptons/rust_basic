use crate::{hash_map::KeyIter, Hashable};

/// For iteration over immutable items in a hash set.
pub struct Iter<'a, T>
where
    T: Hashable + Eq,
{
    iter: KeyIter<'a, T, ()>,
}

impl<'a, T> Iter<'a, T>
where
    T: Hashable + Eq,
{
    pub(super) fn new(iter: KeyIter<'a, T, ()>) -> Self {
        return Self { iter };
    }
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Hashable + Eq,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => return None,
            Some(v) => return Some(v),
        };
    }
}
