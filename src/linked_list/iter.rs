use std::marker::PhantomData;

use super::etc::Node;

/// For iteration over immutable item in a linked list.
pub struct Iter<'a, T> {
    current: Option<*mut Node<T>>,
    marker: PhantomData<&'a Node<T>>,
}

impl<'a, T> Iter<'a, T> {
    pub(super) fn new(node: Option<*mut Node<T>>) -> Self {
        return Self {
            current: node,
            marker: PhantomData,
        };
    }
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Clone,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_none() {
            return None;
        }
        unsafe {
            let current = self.current.clone().unwrap();
            self.current = (*current).next;
            return Some(&(*current).item);
        }
    }
}

/// For iteration over mutable items in a linked list.
pub struct IterMut<'a, T> {
    current: Option<*mut Node<T>>,
    marker: PhantomData<&'a Node<T>>,
}

impl<'a, T> IterMut<'a, T> {
    pub(super) fn new(node: Option<*mut Node<T>>) -> Self {
        return Self {
            current: node,
            marker: PhantomData,
        };
    }
}

impl<'a, T> Iterator for IterMut<'a, T>
where
    T: Clone,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_none() {
            return None;
        }
        unsafe {
            let current = self.current.unwrap();
            self.current = (*current).next;
            return Some(&mut (*current).item);
        }
    }
}
