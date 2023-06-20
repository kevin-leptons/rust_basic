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

pub use iter::Iter;
use iter::TravelNodePostIter;
use node::Node;

pub use self::iter::{KeyIter, ValueIter};

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
    root: Option<*mut Node<K, V>>,
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
            root: None,
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
        assert!(self.size < usize::MAX, "unexpected: size is overflow");
        let new = Self::new_node(key, value);
        let mut current = match self.root {
            None => {
                self.root = Some(new);
                self.size = 1;
                return None;
            }
            Some(v) => v,
        };
        unsafe {
            loop {
                if (*current).key > (*new).key {
                    if (*current).left.is_some() {
                        current = (*current).left.unwrap();
                    } else {
                        (*current).left = Some(new);
                        (*new).parent = Some(current);
                        break;
                    }
                } else if (*current).key < (*new).key {
                    if (*current).right.is_some() {
                        current = (*current).right.unwrap();
                    } else {
                        (*current).right = Some(new);
                        (*new).parent = Some(current);
                        break;
                    }
                } else {
                    self.replace(new, current);
                    return Some(Box::from_raw(current).value);
                }
            }
        }
        self.size += 1;
        return None;
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
        return self.lookup(key).is_some();
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
        let mut current = match self.root {
            None => panic!("expect: not empty tree"),
            Some(v) => v,
        };
        unsafe {
            loop {
                match (*current).left {
                    None => return &(*current).key,
                    Some(v) => current = v,
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
        let mut current = match self.root {
            None => panic!("expect: not empty tree"),
            Some(v) => v,
        };
        unsafe {
            loop {
                match (*current).right {
                    None => return &(*current).key,
                    Some(v) => current = v,
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
        let matched = match self.lookup(key) {
            None => return None,
            Some(v) => v,
        };
        unsafe {
            if (*matched).left.is_none() {
                self.transplant(&(*matched).right, matched);
            } else if (*matched).right.is_none() {
                self.transplant(&(*matched).left, matched);
            } else {
                let min_right = Self::min_node_right((*matched).right.unwrap());
                self.transplant(&(*min_right).right, min_right);
                self.relocate(min_right, matched);
            }
            self.size -= 1;
            return Some(Box::from_raw(matched).value);
        }
    }

    /// Remove all items, drop them and give back memory to allocator.
    ///
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    pub fn clear(&mut self) {
        unsafe {
            for node in TravelNodePostIter::new(self.root) {
                drop(Box::from_raw(node));
            }
        }
        self.root = None;
        self.size = 0;
    }

    fn lookup(&self, key: &K) -> Option<*mut Node<K, V>> {
        let mut current_w = self.root;
        unsafe {
            loop {
                let current = match current_w {
                    None => return None,
                    Some(v) => v,
                };
                if (*current).key == *key {
                    return Some(current);
                } else if (*current).key < *key {
                    current_w = (*current).right;
                } else {
                    current_w = (*current).left;
                }
            }
        }
    }

    unsafe fn transplant(
        &mut self,
        source: &Option<*mut Node<K, V>>,
        target: *mut Node<K, V>,
    ) {
        let parent_w = match (*target).parent {
            None => {
                self.root = source.clone();
                None
            }
            Some(parent) => {
                if (*parent).left == Some(target) {
                    (*parent).left = source.clone();
                } else if (*parent).right == Some(target) {
                    (*parent).right = source.clone();
                } else {
                    panic!("expect: parent points to target")
                }
                Some(parent)
            }
        };
        match *source {
            Some(v) => (*v).parent = parent_w,
            None => {}
        }
    }

    unsafe fn relocate(
        &mut self,
        source: *mut Node<K, V>,
        target: *mut Node<K, V>,
    ) {
        let parent_w = match (*target).parent {
            None => {
                self.root = Some(source.clone());
                None
            }
            Some(parent) => {
                if (*parent).left == Some(target) {
                    (*parent).left = Some(source.clone());
                } else if (*parent).right == Some(target) {
                    (*parent).right = Some(source.clone());
                } else {
                    panic!("expect: parent points to target");
                }
                Some(parent)
            }
        };
        (*source).parent = parent_w;
        (*source).left = (*target).left;
        if (*source).left.is_some() {
            (*(*source).left.unwrap()).parent = Some(source);
        }
        (*source).right = (*target).right;
        if (*source).right.is_some() {
            (*(*source).right.unwrap()).parent = Some(source);
        }
    }

    fn min_node_right(from: *mut Node<K, V>) -> *mut Node<K, V> {
        let mut current = from;
        unsafe {
            loop {
                match (*current).left {
                    None => return current,
                    Some(v) => current = v,
                };
            }
        }
    }

    unsafe fn replace(
        &mut self,
        source: *mut Node<K, V>,
        target: *mut Node<K, V>,
    ) {
        match (*target).parent {
            None => self.root = Some(source),
            Some(parent) => {
                if (*parent).left == Some(target) {
                    (*parent).left = Some(source);
                } else if (*parent).right == Some(target) {
                    (*parent).right = Some(source);
                } else {
                    panic!("expect: parent points to target")
                }
            }
        }
        (*source).left = (*target).left;
        (*source).right = (*target).right;
        match (*target).left {
            None => {}
            Some(v) => (*v).parent = Some(source),
        };
        match (*target).right {
            None => {}
            Some(v) => (*v).parent = Some(source),
        }
    }

    fn new_node(key: K, value: V) -> *mut Node<K, V> {
        let new_node = Node {
            key,
            value,
            parent: None,
            left: None,
            right: None,
        };
        let new_box = Box::new(new_node);
        return Box::leak(new_box);
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
        let mut t = BinarySearchTree::<K, V>::new();
        for (k, v) in iter {
            t.set(k, v);
        }
        return t;
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
                Some(v) => {
                    if v != value {
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
