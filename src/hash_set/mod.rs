//! Hash Set - a data structure and related algorithms.
//!
//! This document - module level document does not contain other descriptions or
//! examples. To learn more about how to use, refer to specific Entry APIs which
//! is labeled `entry` in the following sections.
//!
//! If there is the first time, [Package Document](crate) is a good way to learn
//! more about how to explore APIs and access to [Entry APIs
//! List](crate#structs).

use crate::hash::HashKey;
use crate::hash_map::{HashMap, KeyIter};

/// `entry` A container for unique values.
///
/// # Example
///
/// ```
/// use rust_basic::HashSet;
///
/// let mut s = HashSet::<u32>::from([1, 7, 3, 5]);
/// s.add(9);
/// assert_eq!(s.has(&9), true);
/// assert_eq!(s.remove(&3), true);
/// assert_eq!(s.has(&3), false);
/// assert_eq!(s.size(), 4);
#[derive(Debug)]
pub struct HashSet<T: HashKey> {
    map: HashMap<T, ()>,
}

impl<T: HashKey> HashSet<T> {
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn new() -> Self {
        return Self {
            map: HashMap::new(),
        };
    }

    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn size(&self) -> usize {
        return self.map.size();
    }

    /// * Time complexity: O(1) or O(n).
    /// * Space complexity: O(1).
    pub fn add(&mut self, value: T) -> bool {
        return match self.map.set(value, ()) {
            None => false,
            Some(_) => true,
        };
    }

    /// * Time complexity: O(1) or O(n).
    /// * Space complexity: O(1).
    pub fn has(&self, value: &T) -> bool {
        return self.map.has(value);
    }

    /// * Time complexity: O(1) or O(n).
    /// * Space complexity: O(1).
    pub fn remove(&mut self, value: &T) -> bool {
        return match self.map.remove(&value) {
            None => false,
            Some(_) => true,
        };
    }

    /// * For iteration over items in this container.
    pub fn iter(&self) -> KeyIter<T, ()> {
        return self.map.keys();
    }

    /// * Remove all items from the container, drop them and give back memory to
    ///   allocator.
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    pub fn clear(&mut self) {
        self.map.clear();
    }
}

impl<T, const N: usize> From<[T; N]> for HashSet<T>
where
    T: HashKey,
{
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    fn from(value: [T; N]) -> Self {
        let mut s = HashSet::new();
        for v in value {
            s.add(v);
        }
        return s;
    }
}

impl<T> FromIterator<T> for HashSet<T>
where
    T: HashKey,
{
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut s = HashSet::new();
        for v in iter {
            s.add(v);
        }
        return s;
    }
}

impl<T> Clone for HashSet<T>
where
    T: HashKey + Clone,
{
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    fn clone(&self) -> Self {
        let mut s = HashSet::<T>::new();
        for k in self.iter() {
            s.add(k.clone());
        }
        return s;
    }
}

impl<T> Eq for HashSet<T> where T: HashKey {}

impl<T> PartialEq for HashSet<T>
where
    T: HashKey,
{
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    fn eq(&self, other: &Self) -> bool {
        return self.map == other.map;
    }
}
