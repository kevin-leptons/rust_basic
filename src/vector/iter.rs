use crate::Vector;

/// For iteration over immutable items in a vector, begin from index `0`.
pub struct Iter<'a, T> {
    vector: &'a Vector<T>,
    index: usize,
}

impl<'a, T> Iter<'a, T> {
    pub(super) fn new(v: &'a Vector<T>) -> Self {
        return Self {
            vector: v,
            index: 0,
        };
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.vector.size {
            return None;
        }
        let value = self.vector.get(self.index);
        self.index += 1;
        return Some(value);
    }
}

/// For iteration over mutable items in a vector, begin from index `0`.
pub struct IterMut<'a, T> {
    vector: &'a mut Vector<T>,
    index: usize,
}

impl<'a, T> IterMut<'a, T> {
    pub(super) fn new(v: &'a mut Vector<T>) -> Self {
        return Self {
            vector: v,
            index: 0,
        };
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next<'b>(&mut self) -> Option<Self::Item> {
        if self.index >= self.vector.size {
            return None;
        }
        let value = unsafe { &mut *self.vector.slots.as_ptr().add(self.index) };
        self.index += 1;
        return Some(value);
    }
}
