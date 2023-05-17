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

/// `entry` A container for unique items.
///
/// The APIs are similar as a set, with extra ones such as [min()](Self::min)
/// and [max()](Self::max). The implementation is based on a Binary Search Tree.
///
/// # Example
///
/// ```
/// use rust_basic::BinarySearchTree;
///
/// let mut t = BinarySearchTree::from([
///     (1, 7),
///     (3, 5),
///     (9, 2),
/// ]);
/// assert_eq!(t.get(&3), Some(&5));
/// assert_eq!(t.min(), &1);
/// assert_eq!(t.max(), &9);
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
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn new() -> Self {
        return Self {
            root: None,
            size: 0,
        };
    }

    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn size(&self) -> usize {
        return self.size;
    }

    /// * Put an item into the container.
    /// * Time complexity: O(log(n)) or O(n).
    /// * Space complexity: O(n).
    pub fn set(&mut self, key: K, value: V) {
        let new_node = Node {
            key: key,
            value: value,
            parent: None,
            left: None,
            right: None,
        };
        let b = Box::new(new_node);
        let x = Box::leak(b);
        let mut n = match self.root {
            None => {
                self.root = Some(x);
                self.size = 1;
                return;
            }
            Some(v) => v,
        };
        unsafe {
            loop {
                if (*x).key < (*n).key {
                    if (*n).left.is_some() {
                        n = (*n).left.unwrap();
                    } else {
                        (*n).left = Some(x);
                        (*x).parent = Some(n);
                        break;
                    }
                } else if (*x).key > (*n).key {
                    if (*n).right.is_some() {
                        n = (*n).right.unwrap();
                    } else {
                        (*n).right = Some(x);
                        (*x).parent = Some(n);
                        break;
                    }
                } else {
                    panic!("expect: no duplicated key");
                }
            }
        }
        self.size += 1;
    }

    /// * Time complexity: O(log(n)) or O(n).
    /// * Space complexity: O(n).
    pub fn get(&self, key: &K) -> Option<&V> {
        unsafe {
            match self.lookup(key) {
                None => return None,
                Some(v) => Some(&(*v).value),
            }
        }
    }

    /// * Time complexity: O(log(n)) or O(n).
    /// * Space complexity: O(n).
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        unsafe {
            match self.lookup(key) {
                None => return None,
                Some(v) => Some(&mut (*v).value),
            }
        }
    }

    /// * Time complexity: O(log(n)) or O(n).
    /// * Space complexity: O(n).
    pub fn has(&self, key: &K) -> bool {
        return self.lookup(key).is_some();
    }

    /// * For iteration over pairs in the container. It does not guarantee that
    ///   items will arrive in a specific order.
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn iter(&self) -> Iter<K, V> {
        return Iter::new(self.root);
    }

    /// * For iteration over keys in the container. It does not guarantee that
    ///   items will arrive in a specific order.
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn keys(&self) -> KeyIter<K, V> {
        return KeyIter::new(self.root);
    }

    /// * For iteration over values in the container. It does not guarantee that
    ///   items will arrive in a specific order.
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn values(&self) -> ValueIter<K, V> {
        return ValueIter::new(self.root);
    }

    /// * Time complexity: O(log(n)) or O(n).
    /// * Space complexity: O(n).
    pub fn min(&self) -> &K {
        let mut n = match self.root {
            None => panic!("expect: non empty tree"),
            Some(v) => v,
        };
        unsafe {
            loop {
                match (*n).left {
                    None => return &(*n).key,
                    Some(v) => n = v,
                };
            }
        }
    }

    /// * Time complexity: O(log(n)) or O(n).
    /// * Space complexity: O(n).
    pub fn max(&self) -> &K {
        let mut n = match self.root {
            None => panic!("expect: non empty tree"),
            Some(v) => v,
        };
        unsafe {
            loop {
                match (*n).right {
                    None => return &(*n).key,
                    Some(v) => n = v,
                };
            }
        }
    }

    /// * Time complexity: O(log(n)) or O(n).
    /// * Space complexity: O(n).
    pub fn remove(&mut self, value: &K) -> Option<V> {
        let z = match self.lookup(value) {
            None => return None,
            Some(v) => v,
        };
        unsafe {
            if (*z).left.is_none() {
                self.transplant(&(*z).right, z);
            } else if (*z).right.is_none() {
                self.transplant(&(*z).left, z);
            } else {
                let y = Self::min_node((*z).right.unwrap());
                self.transplant(&(*y).right, y);
                self.relocate(y, z);
            }
            self.size -= 1;
            let b = Box::from_raw(z);
            return Some(b.value);
        }
    }

    /// * Remove all items from the container, drop them and give back memory to
    ///   allocator.
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
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
        let mut n = self.root.clone();
        unsafe {
            loop {
                let x = match n {
                    None => return None,
                    Some(v) => v,
                };
                if *key == (*x).key {
                    return Some(x);
                } else if *key > (*x).key {
                    n = (*x).right.clone();
                } else {
                    n = (*x).left.clone();
                }
            }
        }
    }

    unsafe fn transplant(
        &mut self,
        source: &Option<*mut Node<K, V>>,
        target: *mut Node<K, V>,
    ) {
        let parent = match (*target).parent {
            None => {
                self.root = source.clone();
                None
            }
            Some(v) => {
                if (*v).left == Some(target) {
                    (*v).left = source.clone();
                } else if (*v).right == Some(target) {
                    (*v).right = source.clone();
                } else {
                    panic!("expect: parent points to target")
                }
                Some(v.clone())
            }
        };
        if source.is_some() {
            (*source.unwrap()).parent = parent;
        }
    }

    unsafe fn relocate(
        &mut self,
        source: *mut Node<K, V>,
        target: *mut Node<K, V>,
    ) {
        let parent = match (*target).parent {
            None => {
                self.root = Some(source.clone());
                None
            }
            Some(v) => {
                if (*v).left == Some(target) {
                    (*v).left = Some(source.clone());
                } else if (*v).right == Some(target) {
                    (*v).right = Some(source.clone());
                } else {
                    panic!("expect: parent points to target");
                }
                Some(v.clone())
            }
        };
        (*source).parent = parent;
        (*source).left = (*target).left;
        if (*source).left.is_some() {
            (*(*source).left.unwrap()).parent = Some(source);
        }
        (*source).right = (*target).right;
        if (*source).right.is_some() {
            (*(*source).right.unwrap()).parent = Some(source);
        }
    }

    fn min_node(from: *mut Node<K, V>) -> *mut Node<K, V> {
        let mut n = from;
        unsafe {
            loop {
                match (*n).left {
                    None => return n,
                    Some(v) => n = v,
                };
            }
        }
    }
}

impl<K, V, const N: usize> From<[(K, V); N]> for BinarySearchTree<K, V>
where
    K: Ord,
{
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    fn from(value: [(K, V); N]) -> Self {
        return Self::from_iter(value);
    }
}

impl<K, V> FromIterator<(K, V)> for BinarySearchTree<K, V>
where
    K: Ord,
{
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut t = BinarySearchTree::<K, V>::new();
        for (k, v) in iter {
            t.set(k, v);
        }
        return t;
    }
}

impl<K, V> Drop for BinarySearchTree<K, V>
where
    K: Ord,
{
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    fn drop(&mut self) {
        self.clear();
    }
}
