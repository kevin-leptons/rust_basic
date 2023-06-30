//! Binary Search Tree - a data structure and related algorithms.
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

use iter::TravelPostOrderIter;
pub use iter::{Iter, KeyIter, ValueIter};
use node::Node;
use std::{cmp::Ordering, ptr};

/// `entry` A container for pairs key-value.
///
/// The APIs are similar as a [HashMap](crate::HashMap), with extra ones such as
/// [min()](Self::min) and [max()](Self::max).
///
/// # Internal model
///
/// ```txt
///                   +----------------------------- key
///                   |   +------------------------- value
///                   |   |
///                   v   v
///                  (6, "a")
///                 /        \
///         (2, "b")          (8, "c")
///        /        \                 \
///      (1)        (4)               (9)
///                /   \                \
///              (3)   (5)              (10)
///                                        \
///                                        (11)
///                                           \
///                                           (12)
/// ```
///
/// # Panic
///
/// * Call [set](Self::set) to a tree that already has size [usize::MAX].
/// * Call [min](Self::min) or [max](Self::max) to an empty tree.
///
/// # Example
///
/// ```
/// use rust_basic::BinarySearchTree;
///
/// let mut tree = BinarySearchTree::from([
///     (1, 7),
///     (3, 5),
///     (9, 2),
/// ]);
/// assert_eq!(tree.get(&3), Some(&5));
/// assert_eq!(tree.min(), &1);
/// assert_eq!(tree.max(), &9);
#[derive(Debug)]
pub struct BinarySearchTree<K, V>
where
    K: Ord,
{
    root: *mut Node<K, V>,
    size: usize,
}

impl<K, V> BinarySearchTree<K, V>
where
    K: Ord,
{
    /// Create a new empty instance.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn new() -> Self {
        return Self {
            root: ptr::null_mut(),
            size: 0,
        };
    }

    /// Quantity of pairs.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn size(&self) -> usize {
        return self.size;
    }

    /// Put a pair into the container. If the key is already existing then
    /// return the old value.
    ///
    /// Time complexity: O(log(n)) or O(n).
    ///
    /// Space complexity: O(n).
    pub fn set(&mut self, key: K, value: V) -> Option<V> {
        assert!(self.size < usize::MAX, "expect: not full tree");
        unsafe {
            let node = Self::new_node(key, value);
            return self.set_node(node);
        }
    }

    /// Borrow an immutable value.
    ///
    /// Time complexity: O(log(n)) or O(n).
    ///
    /// Space complexity: O(n).
    pub fn get(&self, key: &K) -> Option<&V> {
        unsafe {
            match self.lookup(key) {
                None => return None,
                Some(v) => return Some(&(*v).value),
            }
        }
    }

    /// Borrow a mutable value.
    ///
    /// Time complexity: O(log(n)) or O(n).
    ///
    /// Space complexity: O(n).
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        unsafe {
            match self.lookup(key) {
                None => return None,
                Some(v) => return Some(&mut (*v).value),
            }
        }
    }

    /// If the key does exist then return `true`.
    ///
    /// Time complexity: O(log(n)) or O(n).
    ///
    /// Space complexity: O(n).
    pub fn has(&self, key: &K) -> bool {
        unsafe {
            return self.lookup(key).is_some();
        }
    }

    /// For iteration over pairs. It does not guarantee that items will arrive
    /// in a specific order.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn iter(&self) -> Iter<K, V> {
        return Iter::new(self.root);
    }

    /// For iteration over keys. It does not guarantee that items will arrive
    /// in a specific order.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn keys(&self) -> KeyIter<K, V> {
        return KeyIter::new(self.root);
    }

    /// For iteration over values. It does not guarantee that items will
    /// arrive in a specific order.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn values(&self) -> ValueIter<K, V> {
        return ValueIter::new(self.root);
    }

    /// Borrow immutable value that has minimum key.
    ///
    /// Time complexity: O(log(n)) or O(n).
    ///
    /// Space complexity: O(n).
    pub fn min(&self) -> &K {
        let mut current = match self.root.is_null() {
            true => panic!("expect: not empty tree"),
            false => self.root,
        };
        unsafe {
            loop {
                match (*current).left.is_null() {
                    true => return &(*current).key,
                    false => current = (*current).left,
                };
            }
        }
    }

    /// Borrow immutable value that has maximum key.
    ///
    /// Time complexity: O(log(n)) or O(n).
    ///
    /// Space complexity: O(n).
    pub fn max(&self) -> &K {
        let mut current = match self.root.is_null() {
            true => panic!("expect: not empty tree"),
            false => self.root,
        };
        unsafe {
            loop {
                match (*current).right.is_null() {
                    true => return &(*current).key,
                    false => current = (*current).right,
                };
            }
        }
    }

    /// Remove a pair and return the old value.
    ///
    /// Time complexity: O(log(n)) or O(n).
    ///
    /// Space complexity: O(n).
    pub fn remove(&mut self, key: &K) -> Option<V> {
        unsafe {
            let node = match self.lookup(key) {
                None => return None,
                Some(v) => v,
            };
            if (*node).left.is_null() {
                self.transplant((*node).right, node);
            } else if (*node).right.is_null() {
                self.transplant((*node).left, node);
            } else {
                let min_right = Self::min_node_right((*node).right);
                self.transplant((*min_right).right, min_right);
                self.relocate(min_right, node);
            }
            self.size -= 1;
            return Some(Box::from_raw(node).value);
        }
    }

    /// Remove all items, drop them and give back memory to allocator.
    ///
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    pub fn clear(&mut self) {
        unsafe {
            for node in TravelPostOrderIter::new(self.root) {
                drop(Box::from_raw(node));
            }
        }
        self.root = ptr::null_mut();
        self.size = 0;
    }

    unsafe fn set_node(&mut self, node: *mut Node<K, V>) -> Option<V> {
        if self.root.is_null() {
            self.root = node;
            self.size = 1;
            return None;
        }
        let mut current = self.root;
        loop {
            match (*node).key.cmp(&(*current).key) {
                Ordering::Equal => {
                    self.replace(node, current);
                    return Some(Box::from_raw(current).value);
                }
                Ordering::Less => match (*current).left.is_null() {
                    true => {
                        (*current).left = node;
                        (*node).parent = current;
                        self.size += 1;
                        return None;
                    }
                    false => current = (*current).left,
                },
                Ordering::Greater => match (*current).right.is_null() {
                    true => {
                        (*current).right = node;
                        (*node).parent = current;
                        self.size += 1;
                        return None;
                    }
                    false => current = (*current).right,
                },
            }
        }
    }

    unsafe fn lookup(&self, key: &K) -> Option<*mut Node<K, V>> {
        let mut current = self.root;
        while !current.is_null() {
            match key.cmp(&(*current).key) {
                Ordering::Equal => return Some(current),
                Ordering::Less => current = (*current).left,
                Ordering::Greater => current = (*current).right,
            };
        }
        return None;
    }

    unsafe fn transplant(
        &mut self,
        source: *mut Node<K, V>,
        target: *mut Node<K, V>,
    ) {
        match (*target).parent.is_null() {
            true => self.root = source,
            false => {
                let parent = (*target).parent;
                if (*parent).left == target {
                    (*parent).left = source;
                } else if (*parent).right == target {
                    (*parent).right = source;
                } else {
                    panic!("expect: parent points to target")
                }
            }
        };
        if !source.is_null() {
            (*source).parent = (*target).parent;
        };
    }

    unsafe fn relocate(
        &mut self,
        source: *mut Node<K, V>,
        target: *mut Node<K, V>,
    ) {
        match (*target).parent.is_null() {
            true => self.root = source,
            false => {
                let parent = (*target).parent;
                if (*parent).left == target {
                    (*parent).left = source;
                } else if (*parent).right == target {
                    (*parent).right = source;
                } else {
                    panic!("expect: parent points to target");
                }
            }
        };
        (*source).parent = (*target).parent;
        (*source).left = (*target).left;
        if !(*source).left.is_null() {
            (*(*source).left).parent = source;
        }
        (*source).right = (*target).right;
        if !(*source).right.is_null() {
            (*(*source).right).parent = source;
        }
    }

    unsafe fn min_node_right(from: *mut Node<K, V>) -> *mut Node<K, V> {
        let mut current = from;
        loop {
            match (*current).left.is_null() {
                true => return current,
                false => current = (*current).left,
            };
        }
    }

    unsafe fn replace(
        &mut self,
        source: *mut Node<K, V>,
        target: *mut Node<K, V>,
    ) {
        match (*target).parent.is_null() {
            true => self.root = source,
            false => {
                let parent = (*target).parent;
                if (*parent).left == target {
                    (*parent).left = source;
                } else if (*parent).right == target {
                    (*parent).right = source;
                } else {
                    panic!("expect: parent points to target")
                }
            }
        }
        (*source).left = (*target).left;
        (*source).right = (*target).right;
        if !(*target).left.is_null() {
            (*(*target).left).parent = source;
        }
        if !(*target).right.is_null() {
            (*(*target).right).parent = source;
        }
    }

    fn new_node(key: K, value: V) -> *mut Node<K, V> {
        let node = Node {
            key,
            value,
            parent: ptr::null_mut(),
            left: ptr::null_mut(),
            right: ptr::null_mut(),
        };
        return Box::leak(Box::new(node));
    }
}

impl<K, V, const N: usize> From<[(K, V); N]> for BinarySearchTree<K, V>
where
    K: Ord,
{
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn from(value: [(K, V); N]) -> Self {
        return Self::from_iter(value);
    }
}

impl<K, V> FromIterator<(K, V)> for BinarySearchTree<K, V>
where
    K: Ord,
{
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut tree = BinarySearchTree::<K, V>::new();
        for (key, value) in iter {
            tree.set(key, value);
        }
        return tree;
    }
}

impl<K, V> Eq for BinarySearchTree<K, V>
where
    K: Ord,
    V: Eq,
{
}

impl<K, V> PartialEq for BinarySearchTree<K, V>
where
    K: Ord,
    V: Eq,
{
    /// Time complexity: O(n.log(n)) or O(n^2).
    ///
    /// Space complexity: O(1).
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }
        for (key, value) in self.iter() {
            match other.get(key) {
                None => return false,
                Some(other_value) => {
                    if other_value != value {
                        return false;
                    }
                }
            }
        }
        return true;
    }
}

impl<K, V> Clone for BinarySearchTree<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    /// Time complexity: O(n.log(n)).
    ///
    /// Space complexity: O(1).
    fn clone(&self) -> Self {
        return Self::from_iter(
            self.iter().map(|(key, value)| (key.clone(), value.clone())),
        );
    }
}

impl<K, V> Drop for BinarySearchTree<K, V>
where
    K: Ord,
{
    /// Equivalent to [Self::clear].
    fn drop(&mut self) {
        self.clear();
    }
}
