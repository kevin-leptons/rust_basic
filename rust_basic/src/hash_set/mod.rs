//! Hash Set - a data structure and related algorithms.
//!
//! This document - module level document does not contain other descriptions or
//! examples. To learn more about how to use, refer to specific Entry APIs which
//! is labeled `entry` in the following sections.
//!
//! If there is the first time, [Package Document](crate) is a good way to learn
//! more about how to explore APIs and access to [Entry APIs
//! List](crate#structs).

mod iter;

use crate::hash::Hashable;
use crate::hash_map::HashMap;
pub use iter::Iter;

/// `entry` A container for unique items.
///
/// # Model
///
/// ```txt
/// +---+
/// | 1 |<--- item
/// |---|
/// | 2 |
/// |---|
/// | 3 |
/// +---+
/// ```
///
/// # Panic
///
/// * Call [add](Self::add) to a set that is already has size [usize::MAX].
/// * Call [add](Self::add) and make the set uses more than [isize::MAX] bytes.
///
/// # Example
///
/// ```
/// use rust_basic::HashSet;
///
/// let mut set = HashSet::from([1, 7, 3, 5]);
/// set.add(9);
/// assert_eq!(set.has(&9), true);
/// assert_eq!(set.remove(&3), true);
/// assert_eq!(set.has(&3), false);
#[derive(Debug)]
pub struct HashSet<T>
where
    T: Hashable + Eq,
{
    map: HashMap<T, ()>,
}

impl<T> HashSet<T>
where
    T: Hashable + Eq,
{
    /// Create a new empty instance.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn new() -> Self {
        return Self {
            map: HashMap::new(),
        };
    }

    /// Quantity of items.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
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

    /// If the item does exist then return `true`.
    ///
    /// Time complexity: O(1) or O(n).
    ///
    /// Space complexity: O(1).
    pub fn has(&self, value: &T) -> bool {
        return self.map.has(value);
    }

    /// Remove the item and return it.
    ///
    /// Time complexity: O(1) or O(n).
    ///
    /// Space complexity: O(1).
    pub fn remove(&mut self, value: &T) -> bool {
        return match self.map.remove(&value) {
            None => false,
            Some(_) => true,
        };
    }

    /// For iteration over items.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn iter(&self) -> Iter<T> {
        return Iter::new(self.map.keys());
    }

    /// Remove all items container, drop them and give back memory to allocator.
    ///
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    pub fn clear(&mut self) {
        self.map.clear();
    }
}

impl<T> FromIterator<T> for HashSet<T>
where
    T: Hashable + Eq,
{
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = HashSet::new();
        for item in iter {
            set.add(item);
        }
        return set;
    }
}

impl<T, const N: usize> From<[T; N]> for HashSet<T>
where
    T: Hashable + Eq,
{
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn from(items: [T; N]) -> Self {
        return Self::from_iter(items);
    }
}

impl<T> Clone for HashSet<T>
where
    T: Hashable + Clone + Eq,
{
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn clone(&self) -> Self {
        let mut s = HashSet::<T>::new();
        for k in self.iter() {
            s.add(k.clone());
        }
        return s;
    }
}

impl<T> Eq for HashSet<T> where T: Hashable + Eq {}

impl<T> PartialEq for HashSet<T>
where
    T: Hashable + Eq,
{
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn eq(&self, other: &Self) -> bool {
        return self.map == other.map;
    }
}
