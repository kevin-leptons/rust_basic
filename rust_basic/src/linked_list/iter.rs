use super::node::Node;
use std::marker::PhantomData;

/// For iteration over immutable item in a linked list.
pub struct Iter<'a, T> {
    current: *mut Node<T>,
    _marker: PhantomData<&'a Node<T>>,
}

impl<'a, T> Iter<'a, T> {
    pub(super) fn new(node: *mut Node<T>) -> Self {
        return Self {
            current: node,
            _marker: PhantomData,
        };
    }
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Clone,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            return None;
        }
        unsafe {
            let current = self.current;
            self.current = (*current).next;
            return Some(&(*current).item);
        }
    }
}

/// For iteration over mutable items in a linked list.
pub struct IterMut<'a, T> {
    current: *mut Node<T>,
    _marker: PhantomData<&'a Node<T>>,
}

impl<'a, T> IterMut<'a, T> {
    pub(super) fn new(node: *mut Node<T>) -> Self {
        return Self {
            current: node,
            _marker: PhantomData,
        };
    }
}

impl<'a, T> Iterator for IterMut<'a, T>
where
    T: Clone,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            return None;
        }
        unsafe {
            let current = self.current;
            self.current = (*current).next;
            return Some(&mut (*current).item);
        }
    }
}
