//! Linked List - a data structure and related algorithms.
//!
//! This document - module level document does not contain other descriptions or
//! examples. To learn more about how to use, refer to specific Entry APIs which
//! is labeled `entry` in the following sections.
//!
//! If there is the first time, [Package Document](crate) is a good way to learn
//! more about how to explore APIs and access to [Entry APIs
//! List](crate#structs).

mod iter;
mod node;

pub use iter::{Iter, IterMut};
use node::{Cursor, Node};
use std::cmp::Ordering;
use std::ops::{Index, IndexMut};
use std::ptr;

/// `entry` A container for items, indexed by unsigned integers.
///
/// # Model
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
/// # Panic
///
/// * Call [insert](Self::insert), [push_front](Self::push_front) or
///   [push_back](Self::push_back) to a list that already has size equal to
///   [usize::MAX].
/// * Call [index](Self::index), [index_mut](Self::index_mut) or
///   [remove](Self::remove) with index that is greater than or equal to
///   [size](Self::size).
/// * Call [pop_front](Self::pop_front) or [pop_back](Self::pop_back) to a list
///   that is empty.
///
/// # Example
///
/// ```
/// use rust_basic::LinkedList;
///
/// let mut list = LinkedList::from([1, 2, 3]);
/// assert_eq!(list.front(), &1);
/// assert_eq!(list.back(), &3);
/// assert_eq!(list[1], 2);
#[derive(Debug)]
pub struct LinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
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
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
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
    pub fn insert(&mut self, index: usize, item: T) {
        assert!(self.size < usize::MAX, "expected: not full list");
        unsafe {
            let cursor = self.lookup(index);
            let mut node = Self::new_node(item);
            (*node).next = cursor.current;
            (*node).prev = cursor.prev;
            match cursor.prev.is_null() {
                true => self.head = node,
                false => (*cursor.prev).next = node,
            };
            match cursor.current.is_null() {
                true => self.tail = node,
                false => (*cursor.current).prev = node,
            }
            self.size += 1;
        }
    }

    /// Borrow immutable item at index `0`.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(n).
    pub fn front(&self) -> &T {
        assert!(self.size > 0, "expect: not empty list");
        return &self[0];
    }

    /// Borrow immutable item at index `size - 1`.
    ///
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(n).
    pub fn back(&self) -> &T {
        assert!(self.size > 0, "expect: not empty list");
        return &self[self.size - 1];
    }

    /// Put a new item at index `0`.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(n).
    pub fn push_front(&mut self, value: T) {
        self.insert(0, value);
    }

    /// Put a new item at index `size`.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(n).
    pub fn push_back(&mut self, item: T) {
        self.insert(self.size, item);
    }

    /// Remove an item at index `0`
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(n).
    pub fn pop_front(&mut self) -> T {
        assert!(self.size > 0, "expect: not empty list");
        return self.remove(0);
    }

    /// Remove an item at index `size - 1`.
    ///
    /// Time complexity: O(1) or O(n).
    ///
    /// Space complexity: O(n).
    pub fn pop_back(&mut self) -> T {
        assert!(self.size > 0, "expect: not empty list");
        return self.remove(self.size - 1);
    }

    /// Remove an item at `index`.
    ///
    /// Time complexity: O(1) or O(n).
    ///
    /// Space complexity: O(n).
    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.size, "expect: valid index");
        unsafe {
            let cursor = self.lookup(index);
            let current = cursor.current;
            match cursor.prev.is_null() {
                true => self.head = (*current).next,
                false => (*cursor.prev).next = (*current).next,
            };
            match (*current).next.is_null() {
                true => self.tail = (*current).prev,
                false => (*(*current).next).prev = cursor.prev,
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
        return Iter::new(self.head);
    }

    /// For iteration over mutable items.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn iter_mut(&mut self) -> IterMut<T> {
        return IterMut::new(self.head);
    }

    /// Remove all items, drop them and give memory back to allocator.
    ///
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    pub fn clear(&mut self) {
        let mut current = self.head;
        unsafe {
            while !current.is_null() {
                let next = (*current).next;
                drop(Box::from_raw(current));
                current = next;
            }
        }
        self.head = ptr::null_mut();
        self.tail = ptr::null_mut();
        self.size = 0;
    }

    fn new_node(item: T) -> *mut Node<T> {
        let node = Node {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
            item,
        };
        return Box::leak(Box::new(node));
    }

    unsafe fn lookup(&self, index: usize) -> Cursor<T> {
        assert!(index <= self.size, "expect: valid index");
        if (index + 1) == self.size {
            return match self.tail.is_null() {
                true => Cursor {
                    current: ptr::null_mut(),
                    prev: ptr::null_mut(),
                },
                false => Cursor {
                    current: self.tail,
                    prev: (*self.tail).prev,
                },
            };
        }
        if index == self.size {
            return Cursor {
                current: ptr::null_mut(),
                prev: self.tail,
            };
        }
        let mut prev = ptr::null_mut();
        let mut current = self.head;
        for _ in 0..index {
            prev = current;
            current = (*current).next;
        }
        return Cursor { prev, current };
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
        let mut list = Self::new();
        for item in iter {
            list.push_back(item);
        }
        return list;
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

impl<T> Index<usize> for LinkedList<T> {
    type Output = T;

    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.size, "expect: valid index");
        unsafe {
            let cursor = self.lookup(index);
            return &(*cursor.current).item;
        }
    }
}

impl<T> IndexMut<usize> for LinkedList<T> {
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.size, "expect: valid index");
        unsafe {
            let cursor = self.lookup(index);
            return &mut (*cursor.current).item;
        }
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
        let mut self_node = self.head;
        let mut other_node = other.head;
        unsafe {
            loop {
                if self_node.is_null() || other_node.is_null() {
                    return self.size.cmp(&other.size);
                }
                match (*self_node).item.cmp(&(*other_node).item) {
                    Ordering::Equal => {
                        self_node = (*self_node).next;
                        other_node = (*other_node).next;
                    }
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                }
            }
        }
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

impl<T> Clone for LinkedList<T>
where
    T: Clone,
{
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn clone(&self) -> Self {
        let mut list = LinkedList::new();
        for item in self.iter() {
            list.push_back(item.clone());
        }
        return list;
    }
}

impl<T> Drop for LinkedList<T> {
    /// Equivalent [Self::clear].
    fn drop(&mut self) {
        self.clear();
    }
}
