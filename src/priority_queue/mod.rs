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
/// # Example
///
/// ```
/// use rust_basic::PriorityQueue;
///
/// let mut q = PriorityQueue::from(["item: 2", "item: 0", "item: 1"]);
/// q.push("item: 9");
/// assert_eq!(q.pop().unwrap(), "item: 9");
/// assert_eq!(q.pop().unwrap(), "item: 2");
/// assert_eq!(q.size(), 2);
/// assert_eq!(q.top().unwrap(), &"item: 1");
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
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn new() -> Self {
        return Self {
            slots: Vector::<T>::new(),
        };
    }

    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn size(&self) -> usize {
        return self.slots.size();
    }

    /// * Time complexity: O(1), O(log(n)) or O(n).
    /// * Space complexity: O(n).
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

    /// * Remove highest priority item from the container.
    /// * Time complexity: O(1), O(log(n)) or O(n).
    /// * Space complexity: O(n).
    pub fn pop(&mut self) -> Option<T> {
        if self.slots.size() == 0 {
            return None;
        }
        if self.slots.size() == 1 {
            return Some(self.slots.remove(0));
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
        return Some(top);
    }

    /// * Retrieve highest priority item without remove it from the container.
    /// * Time complexity: O(1).
    /// * Space complexity: O(n).
    pub fn top(&self) -> Option<&T> {
        if self.size() == 0 {
            return None;
        }
        return Some(self.slots.get(0));
    }

    /// * For iteration over items in the queue. It does not guarantee items
    ///   come in ordered of priority.
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn iter(&self) -> Iter<T> {
        return Iter::new(self);
    }

    /// * Remove all items from the queue, drop them and give back memory to
    ///   allocator.
    /// * Time complexity: O(n).
    /// * Space complexity: O(1).
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
    /// * Time complexity: O(n.log(n)).
    /// * Space complexity: O(n).
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
    /// * Time complexity: O(n.log(n)).
    /// * Space complexity: O(n).
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut q = PriorityQueue::<T>::new();
        for i in iter {
            q.push(i);
        }
        return q;
    }
}
