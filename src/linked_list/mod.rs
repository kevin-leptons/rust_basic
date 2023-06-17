//! Linked List - a data structure and related algorithms.
//!
//! This document - module level document does not contain other descriptions or
//! examples. To learn more about how to use, refer to specific Entry APIs which
//! is labeled `entry` in the following sections.
//!
//! If there is the first time, [Package Document](crate) is a good way to learn
//! more about how to explore APIs and access to [Entry APIs
//! List](crate#structs).

mod etc;
mod iter;

use etc::{Cursor, Node};
pub use iter::{Iter, IterMut};
use std::{cmp::Ordering, ops::Index};

/// `entry` A container for items, indexed by unsigned integers.
///
/// # Overview
///
/// ```txt
///   +----------------------- front
///   |                 +----- back
///   |                 |
///   v                 v
/// +---+    +---+    +---+
/// |   |--->|   |--->|   |
/// | 1 |    | 2 |    | 3 |
/// |   |<---|   |<---|   |
/// +---+    +---+  ^ +---+
///   0        1        2
///   ^        ^        ^
///   |        |        |
///   +----------------------- index
/// ```
///
/// # Example
///
/// ```
/// use rust_basic::LinkedList;
///
/// let mut l = LinkedList::from([1, 2, 3]);
/// assert_eq!(l.front(), &1);
/// assert_eq!(l.back(), &3);
/// assert_eq!(l[1], 2);
#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<*mut Node<T>>,
    tail: Option<*mut Node<T>>,
    size: usize,
}

impl<T> LinkedList<T> {
    /// Create a new empty instance.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn new() -> Self {
        return Self {
            head: None,
            tail: None,
            size: 0,
        };
    }

    /// Quantity of items.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn size(&self) -> usize {
        return self.size;
    }

    /// Put a new item at `index`.
    ///
    /// Time complexity: O(1) or O(n).
    ///
    /// Space complexity: O(n).
    pub fn set(&mut self, index: usize, item: T) {
        let c = self.lookup(index);
        let b = Box::new(Node {
            next: c.current,
            prev: c.prev,
            item: item,
        });
        let n = Box::leak(b);
        match c.prev {
            None => self.head = Some(n),
            Some(v) => unsafe { (*v).next = Some(n) },
        };
        match c.current {
            None => self.tail = Some(n),
            Some(v) => unsafe { (*v).prev = Some(n) },
        }
        self.size += 1;
    }

    /// Borrow immutable item at `index`.
    ///
    /// Time complexity: O(1) or O(n).
    ///
    /// Space complexity: O(n).
    pub fn get(&self, index: usize) -> &T {
        assert!(index < self.size, "expect: `index` is less than size");
        let c = self.lookup(index);
        return unsafe { &(*c.current.unwrap()).item };
    }

    /// Borrow mutable item at `index`.
    ///
    /// Time complexity: O(1) or O(n).
    ///
    /// Space complexity: O(n).
    pub fn get_mut(&mut self, index: usize) -> &mut T {
        assert!(index < self.size, "expect: `index` is less than size");
        let c = self.lookup(index);
        return unsafe { &mut (*c.current.unwrap()).item };
    }

    /// Borrow immutable item at index `0`.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(n).
    pub fn front(&self) -> &T {
        assert!(self.size > 0, "expect: non empty list");
        return self.get(0);
    }

    /// Borrow immutable item at index `size - 1`.
    ///
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(n).
    pub fn back(&self) -> &T {
        assert!(self.size > 0, "expect: non empty list");
        return self.get(self.size - 1);
    }

    /// Put a new item at index `0`.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(n).
    pub fn push_front(&mut self, value: T) {
        self.set(0, value);
    }

    /// Put a new item at index `size`.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(n).
    pub fn push_back(&mut self, item: T) {
        self.set(self.size, item);
    }

    /// Remove an item at index `0`
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(n).
    pub fn pop_front(&mut self) -> T {
        assert!(self.size > 0, "expect: non empty list");
        return self.remove(0);
    }

    /// Remove an item at index `size - 1`.
    ///
    /// Time complexity: O(1) or O(n).
    ///
    /// Space complexity: O(n).
    pub fn pop_back(&mut self) -> T {
        assert!(self.size > 0, "expect: non empty list");
        return self.remove(self.size - 1);
    }

    /// Remove an item at `index`.
    ///
    /// Time complexity: O(1) or O(n).
    ///
    /// Space complexity: O(n).
    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.size, "expect: `index` is less than size");
        let c = self.lookup(index);
        let current = c.current.unwrap();
        unsafe {
            match c.prev {
                None => self.head = (*current).next,
                Some(v) => (*v).next = (*current).next,
            };
            match (*current).next {
                None => self.tail = (*current).prev,
                Some(v) => (*v).prev = c.prev,
            };
            self.size -= 1;
            return Box::from_raw(current).item;
        }
    }

    /// For iteration over immutable items.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn iter(&self) -> Iter<T> {
        return Iter::new(self.head.clone());
    }

    /// For iteration over mutable items.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn iter_mut(&mut self) -> IterMut<T> {
        return IterMut::new(self.head.clone());
    }

    /// Remove all items, drop them and give memory back to allocator.
    ///
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    pub fn clear(&mut self) {
        let mut current = self.head;
        loop {
            match current {
                None => break,
                Some(v) => {
                    let next = unsafe { (*v).next };
                    unsafe { drop(Box::from_raw(v)) };
                    current = next;
                }
            };
        }
        self.head = None;
        self.tail = None;
        self.size = 0;
    }

    fn lookup(&self, index: usize) -> Cursor<T> {
        if index > self.size {
            panic!("expect: `index` is not greater than size");
        }
        if (index + 1) == self.size {
            match self.tail {
                None => {
                    return Cursor {
                        current: None,
                        prev: None,
                    }
                }
                Some(v) => unsafe {
                    return Cursor {
                        current: self.tail,
                        prev: (*v).prev,
                    };
                },
            };
        }
        if index == self.size {
            return Cursor {
                current: None,
                prev: self.tail,
            };
        }
        let mut prev = None;
        let mut n = self.head;
        let mut i = 0;
        loop {
            if i == index {
                return Cursor {
                    prev: prev,
                    current: n,
                };
            }
            if n.is_none() {
                panic!("unexpected: bad links");
            }
            prev = n;
            n = unsafe { (*n.unwrap()).next };
            i += 1;
        }
    }
}

impl<T, const N: usize> From<[T; N]> for LinkedList<T>
where
    T: Clone,
{
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn from(value: [T; N]) -> Self {
        return Self::from_iter(value.into_iter());
    }
}

impl<T> FromIterator<T> for LinkedList<T>
where
    T: Clone,
{
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut l = Self::new();
        for v in iter {
            l.set(l.size, v);
        }
        return l;
    }
}

impl<T> Index<usize> for LinkedList<T> {
    type Output = T;

    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn index(&self, index: usize) -> &Self::Output {
        return self.get(index);
    }
}

impl<T> Ord for LinkedList<T>
where
    T: Ord,
{
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn cmp(&self, other: &Self) -> Ordering {
        let mut n_self = self.head.clone();
        let mut n_other = other.head.clone();
        unsafe {
            loop {
                if n_self.is_none() || n_other.is_none() {
                    break;
                }
                let i_self = n_self.unwrap();
                let i_other = n_other.unwrap();
                if (*i_self).item > (*i_other).item {
                    return Ordering::Greater;
                } else if (*i_self).item < (*i_other).item {
                    return Ordering::Less;
                }
                n_self = (*i_self).next;
                n_other = (*i_other).next;
            }
        }
        return self.size.cmp(&other.size);
    }
}

impl<T> PartialOrd for LinkedList<T>
where
    T: Ord,
{
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl<T> Eq for LinkedList<T> where T: Ord {}

impl<T> PartialEq for LinkedList<T>
where
    T: Ord,
{
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn eq(&self, other: &Self) -> bool {
        return self.cmp(other) == Ordering::Equal;
    }
}

impl<T> Drop for LinkedList<T> {
    /// Equivalent [Self::clear].
    fn drop(&mut self) {
        self.clear();
    }
}
