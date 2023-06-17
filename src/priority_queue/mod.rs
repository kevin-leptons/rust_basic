//! Priority Queue - a data structure and related algorithms.
//!
//! This document - module level document does not contain other descriptions or
//! examples. To learn more about how to use, refer to specific Entry APIs which
//! is labeled `entry` in the following sections.
//!
//! If there is the first time, [Package Document](crate) is a good way to learn
//! more about how to explore APIs and access to [Entry APIs
//! List](crate#structs).

mod iter;

use crate::vector::Vector;
pub use iter::Iter;

/// `entry` A container for prioritied items.
///
/// The priority is based on comparison between two items `a` and `b`. If `a >
/// b` then `a` has higher priority than `b`. The implementation based on Heap.
///
/// # Overview
///
/// ```txt
/// +---+
/// | 3 |<--- top
/// |---|
/// | 2 |
/// |---|
/// | 1 |<--- bottom
/// +---+
/// ```
///
/// # Example
///
/// ```
/// use rust_basic::PriorityQueue;
///
/// let mut q = PriorityQueue::from([2, 3, 1]);
/// assert_eq!(q.top(), &3);
/// assert_eq!(q.pop(), 3);
/// assert_eq!(q.pop(), 2);
/// assert_eq!(q.pop(), 1);
pub struct PriorityQueue<T>
where
    T: Ord,
{
    slots: Vector<T>,
}

impl<T> PriorityQueue<T>
where
    T: Ord,
{
    /// Create a new empty instance.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn new() -> Self {
        return Self {
            slots: Vector::<T>::new(),
        };
    }

    /// Quantity of items.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn size(&self) -> usize {
        return self.slots.size();
    }

    /// Borrow immutable highest priority item.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(n).
    pub fn top(&self) -> &T {
        assert!(self.slots.size() > 0, "expect: non empty queue");
        return self.slots.get(0);
    }

    /// Put a new item into the container.
    ///
    /// Time complexity: O(1), O(log(n)) or O(n).
    ///
    /// Space complexity: O(n).
    pub fn push(&mut self, value: T) {
        self.slots.set(self.slots.size(), value);
        let mut child_index = self.slots.size() - 1;
        loop {
            let parent_index = match self.index_of_parent(child_index) {
                None => break,
                Some(v) => v,
            };
            if self.slots.get(child_index) > self.slots.get(parent_index) {
                self.slots.swap(child_index, parent_index);
                child_index = parent_index;
            } else {
                break;
            }
        }
    }

    /// Remove the highest priority item.
    ///
    /// Time complexity: O(1), O(log(n)) or O(n).
    ///
    /// Space complexity: O(n).
    pub fn pop(&mut self) -> T {
        assert!(self.slots.size() > 0, "expect: non empty queue");
        if self.slots.size() == 1 {
            return self.slots.remove(0);
        }
        self.slots.swap(0, self.slots.size() - 1);
        let top = self.slots.remove(self.slots.size() - 1);
        let mut parent_index = 0;
        loop {
            let child_index = match self.index_of_greatest_child(parent_index) {
                None => break,
                Some(v) => v,
            };
            if self.slots.get(child_index) > self.slots.get(parent_index) {
                self.slots.swap(child_index, parent_index);
                parent_index = child_index;
            } else {
                break;
            }
        }
        return top;
    }

    /// For iteration over items. It does not guarantee items will arrive in
    /// ordered of priority.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn iter(&self) -> Iter<T> {
        return Iter::new(self);
    }

    /// Remove all items, drop them and give back memory to allocator.
    ///
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(1).
    pub fn clear(&mut self) {
        self.slots.clear();
    }

    fn index_of_parent(&self, index: usize) -> Option<usize> {
        if index == 0 {
            return Option::None;
        }
        return Option::Some((index - 1) / 2);
    }

    fn index_of_greatest_child(&self, index: usize) -> Option<usize> {
        let left_index = 2 * index + 1;
        let right_index = 2 * index + 2;
        if left_index >= self.size() {
            return None;
        }
        if right_index >= self.size() {
            return Some(left_index);
        }
        if self.slots.get(left_index) > self.slots.get(right_index) {
            return Some(left_index);
        } else {
            return Some(right_index);
        }
    }
}

impl<T, const N: usize> From<[T; N]> for PriorityQueue<T>
where
    T: Ord,
{
    /// Time complexity: O(n.log(n)).
    ///
    /// Space complexity: O(n).
    fn from(value: [T; N]) -> Self {
        let mut q = PriorityQueue::<T>::new();
        for i in value {
            q.push(i);
        }
        return q;
    }
}

impl<T> FromIterator<T> for PriorityQueue<T>
where
    T: Ord,
{
    /// Time complexity: O(n.log(n)).
    ///
    /// Space complexity: O(n).
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut q = PriorityQueue::<T>::new();
        for i in iter {
            q.push(i);
        }
        return q;
    }
}
