//! Red Black Tree - a data structure and related algorithms.
//!
//! This document - module level document does not contain other descriptions or
//! examples. To learn more about how to use, refer to specific Entry APIs which
//! is labeled `entry` in the following sections.
//!
//! If there is the first time, [Package Document](crate) is a good way to learn
//! more about how to explore APIs and access to [Entry APIs
//! List](crate#structs).

mod helper;
mod iter;
mod node;

use helper::Direction;
pub use helper::RawPair;
use iter::TravelNodePostIter;
pub use iter::{Iter, KeyIter, ValueIter};
use node::{Color, Node};
use std::{cmp::Ordering, ptr};

use self::helper::RemoveNodeResult;

/// `entry` A container for pairs key-value.
///
/// The APIs is similar as [HashMap](crate::HashMap) but it guarantees time
/// complexity of O(log(n)) for almost all operations.
///
/// # Internal model
///
/// ```txt
///                    +--------------------- key
///                    |    +---------------- value
///                    |    |   +------------ black color
///                    |    |   |        +--- red color
///                    v    v   v        |
///                   (13, "a", B)       |
///                   /          \       |
///                  /            \      |
///                 /              \     |
///                v                v    v
///           (8, R)                (17, R)
///           /    \                 /     \
///         /       \               /       \
///        v         v             v         v
///    (1, B)        (11, B)  (15, B)       (25, B)
///         \                                /   \
///          v                              v     v
///          (6, R)                    (22, R)    (27, R)
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
/// use rust_basic::RedBlackTree;
///
/// let mut tree = RedBlackTree::from([
///     (1, 7),
///     (3, 5),
///     (9, 2),
/// ]);
/// assert_eq!(tree.get(&3), Some(&5));
/// assert_eq!(tree.min(), &1);
/// assert_eq!(tree.max(), &9);
#[derive(Debug)]
pub struct RedBlackTree<K, V>
where
    K: Ord,
{
    root: *mut Node<K, V>,
    size: usize,
}

impl<K, V> RedBlackTree<K, V>
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

    /// Put a new pair key-value.
    ///
    /// Time complexity: O(log(n)).
    ///
    /// Space complexity: O(n).
    pub fn set(&mut self, key: K, value: V) -> Option<V> {
        unsafe {
            let node = Self::new_node(key, value);
            let old_value = self.set_node(node, self.root);
            match (*node).parent.is_null() {
                true => (*node).color = Color::Black,
                false => self.fix_set_node(node),
            }
            return old_value;
        }
    }

    /// Borrow an immutable value .
    ///
    /// Time complexity: O(log(n)).
    ///
    /// Space complexity: O(n).
    pub fn get(&self, key: &K) -> Option<&V> {
        unsafe {
            let node = Self::lookup(self.root, key);
            match node.is_null() {
                true => return None,
                false => return Some(&(*node).value),
            };
        }
    }

    /// Borrow a mutable value.
    ///
    /// Time complexity: O(log(n)).
    ///
    /// Space complexity: O(n).
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        unsafe {
            let node = Self::lookup(self.root, key);
            match node.is_null() {
                true => return None,
                false => return Some(&mut (*node).value),
            };
        }
    }

    /// If the key does exist then return `true`.
    ///
    /// Time complexity: O(log(n)).
    ///
    /// Space complexity: O(n).
    pub fn has(&self, key: &K) -> bool {
        unsafe {
            return !Self::lookup(self.root, key).is_null();
        }
    }

    /// For iteration over pairs.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn iter(&self) -> Iter<K, V> {
        return Iter::new(self.root);
    }

    /// For iteration over pairs.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn keys(&self) -> KeyIter<K, V> {
        return KeyIter::new(self.root);
    }

    /// For iteration over pairs.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn values(&self) -> ValueIter<K, V> {
        return ValueIter::new(self.root);
    }

    /// Borrow an immutable value that pair with minimum key.
    ///
    /// Time complexity: O(log(n)).
    ///
    /// Space complexity: O(n).
    pub fn min(&self) -> &K {
        unsafe {
            assert!(!self.root.is_null(), "expect: not empty tree");
            let mut current = self.root;
            loop {
                match (*current).left.is_null() {
                    true => return &(*current).key,
                    false => current = (*current).left,
                }
            }
        }
    }

    /// Borrow an immutable value that pair with maximum key.
    ///
    /// Time complexity: O(log(n)).
    ///
    /// Space complexity: O(n).
    pub fn max(&self) -> &K {
        unsafe {
            assert!(!self.root.is_null(), "expect: not empty tree");
            let mut current = self.root;
            loop {
                match (*current).right.is_null() {
                    true => return &(*current).key,
                    false => current = (*current).right,
                }
            }
        }
    }

    /// Remove a pair and return it's value.
    ///
    /// Time complexity: O(log(n)).
    ///
    /// Space complexity: O(n).
    pub fn remove(&mut self, key: &K) -> Option<V> {
        unsafe {
            let node = Self::lookup(self.root, key);
            if node.is_null() {
                return None;
            }
            let result = self.remove_node(node);
            if result.color == Color::Black {
                match result.parent.is_null() {
                    true => Self::set_color(result.current, Color::Black),
                    false => self.fix_remove_node(
                        result.parent,
                        result.direction.unwrap(),
                    ),
                };
            }
            self.size -= 1;
            return Some(Box::from_raw(node).value);
        }
    }

    /// Remove all pairs, drop them and give back memory to allocator.
    ///
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(1).
    pub fn clear(&mut self) {
        if self.root.is_null() {
            return;
        }
        unsafe {
            for node in TravelNodePostIter::new(self.root) {
                drop(Box::from_raw(node));
            }
        }
        self.root = ptr::null_mut();
        self.size = 0;
    }

    unsafe fn replace(
        &mut self,
        source: *mut Node<K, V>,
        target: *mut Node<K, V>,
    ) {
        assert!(!target.is_null(), "expect: not null target");
        match (*target).parent.is_null() {
            true => self.root = source,
            false => match Self::get_direction(target) {
                Direction::Left => (*(*target).parent).left = source,
                Direction::Right => (*(*target).parent).right = source,
            },
        }
        (*source).parent = (*target).parent;
        (*source).left = (*target).left;
        (*source).right = (*target).right;
        if !(*target).left.is_null() {
            (*(*target).left).parent = source;
        }
        if !(*target).right.is_null() {
            (*(*target).right).parent = source;
        }
    }

    unsafe fn fix_set_node(&mut self, mut current: *mut Node<K, V>) {
        loop {
            let mut parent = match (*current).parent.is_null() {
                true => break,
                false => (*current).parent,
            };
            if (*parent).color == Color::Black {
                break;
            }
            let mut grand = match (*parent).parent.is_null() {
                true => break,
                false => (*parent).parent,
            };
            match Self::get_direction(parent) {
                Direction::Right => self.fix_set_node_right(
                    &mut current,
                    &mut parent,
                    &mut grand,
                ),
                Direction::Left => self.fix_set_node_left(
                    &mut current,
                    &mut parent,
                    &mut grand,
                ),
            }
        }
        Self::set_color(self.root, Color::Black);
    }

    unsafe fn fix_set_node_right(
        &mut self,
        current: &mut *mut Node<K, V>,
        parent: &mut *mut Node<K, V>,
        grand: &mut *mut Node<K, V>,
    ) {
        let uncle = (**grand).left;
        match Self::get_color(uncle) {
            Color::Red => {
                Self::set_color(uncle, Color::Black);
                (**parent).color = Color::Black;
                (**grand).color = Color::Red;
                *current = *grand;
            }
            Color::Black => {
                if Self::get_direction(*current) == Direction::Left {
                    *current = *parent;
                    self.rotate_right(*current);
                    *parent = (**current).parent;
                    *grand = (**parent).parent;
                }
                (**parent).color = Color::Black;
                (**grand).color = Color::Red;
                self.rotate_left(*grand);
            }
        };
    }

    unsafe fn fix_set_node_left(
        &mut self,
        current: &mut *mut Node<K, V>,
        parent: &mut *mut Node<K, V>,
        grand: &mut *mut Node<K, V>,
    ) {
        let uncle = (**grand).right;
        match Self::get_color(uncle) {
            Color::Red => {
                Self::set_color(uncle, Color::Black);
                (**parent).color = Color::Black;
                (**grand).color = Color::Red;
                *current = *grand;
            }
            Color::Black => {
                if Self::get_direction(*current) == Direction::Right {
                    *current = *parent;
                    self.rotate_left(*current);
                    *parent = (**current).parent;
                    *grand = (**parent).parent;
                }
                (**parent).color = Color::Black;
                (**grand).color = Color::Red;
                self.rotate_right(*grand);
            }
        };
    }

    /// Link `target`'s parent to `source` instead of `target`.
    ///
    /// Keep children unchange for both `source` and `target`.
    unsafe fn transplant(
        &mut self,
        source: *mut Node<K, V>,
        target: *mut Node<K, V>,
    ) {
        let parent = (*target).parent;
        match parent.is_null() {
            true => self.root = source.clone(),
            false => match Self::get_direction(target) {
                Direction::Left => (*parent).left = source,
                Direction::Right => (*parent).right = source,
            },
        }
        if !source.is_null() {
            (*source).parent = parent;
        }
    }

    /// Transfer all links of `target` to `source`, including parent, left and right.
    ///
    /// Keep outer links unchange for `target`.
    ///
    /// Keep links to `source` unchange from nodes as before relocation.
    unsafe fn relocate(
        &mut self,
        source: *mut Node<K, V>,
        target: *mut Node<K, V>,
    ) {
        assert!(!target.is_null(), "expect: not null target");
        match (*target).parent.is_null() {
            true => self.root = source,
            false => match Self::get_direction(target) {
                Direction::Left => (*(*target).parent).left = source,
                Direction::Right => (*(*target).parent).right = source,
            },
        }
        (*source).parent = (*target).parent;
        (*source).left = (*target).left;
        (*source).right = (*target).right;
        if !(*source).left.is_null() {
            (*(*source).left).parent = source;
        }
        if !(*source).right.is_null() {
            (*(*source).right).parent = source;
        }
        (*source).color = (*target).color;
    }

    unsafe fn fix_remove_node(
        &mut self,
        mut parent: *mut Node<K, V>,
        mut direction: Direction,
    ) {
        while !parent.is_null() {
            let current = Self::get_child(parent, direction.clone());
            if Self::get_color(current) == Color::Red {
                break;
            }
            match direction {
                Direction::Left => {
                    self.fix_remove_node_left(&mut parent, &mut direction)
                }
                Direction::Right => {
                    self.fix_remove_node_right(&mut parent, &mut direction)
                }
            }
        }
        match parent.is_null() {
            true => Self::set_color(self.root, Color::Black),
            false => Self::set_color(
                Self::get_child(parent, direction),
                Color::Black,
            ),
        }
    }

    unsafe fn fix_remove_node_left(
        &mut self,
        parent: &mut *mut Node<K, V>,
        direction: &mut Direction,
    ) {
        let mut sibling = (**parent).right;
        if (*sibling).color == Color::Red {
            (*sibling).color = Color::Black;
            (**parent).color = Color::Red;
            self.rotate_left(*parent);
            sibling = (**parent).right;
        }
        if Self::get_color((*sibling).left) == Color::Black
            && Self::get_color((*sibling).right) == Color::Black
        {
            (*sibling).color = Color::Red;
            if !(**parent).parent.is_null() {
                *direction = Self::get_direction(*parent);
            }
            *parent = (**parent).parent;
        } else {
            if Self::get_color((*sibling).right) == Color::Black {
                Self::set_color((*sibling).left, Color::Black);
                (*sibling).color = Color::Red;
                self.rotate_right(sibling);
                sibling = (**parent).right;
            }
            (*sibling).color = (**parent).color;
            (**parent).color = Color::Black;
            Self::set_color((*sibling).right, Color::Black);
            self.rotate_left(*parent);
            *parent = ptr::null_mut();
        }
    }

    unsafe fn fix_remove_node_right(
        &mut self,
        parent: &mut *mut Node<K, V>,
        direction: &mut Direction,
    ) {
        let mut sibling = (**parent).left;
        if (*sibling).color == Color::Red {
            (*sibling).color = Color::Black;
            (**parent).color = Color::Red;
            self.rotate_right(*parent);
            sibling = (**parent).left;
        }
        if Self::get_color((*sibling).right) == Color::Black
            && Self::get_color((*sibling).left) == Color::Black
        {
            (*sibling).color = Color::Red;
            if !(**parent).parent.is_null() {
                *direction = Self::get_direction(*parent);
            }
            *parent = (**parent).parent;
        } else {
            if Self::get_color((*sibling).left) == Color::Black {
                Self::set_color((*sibling).right, Color::Black);
                (*sibling).color = Color::Red;
                self.rotate_left(sibling);
                sibling = (**parent).left;
            }
            (*sibling).color = (**parent).color;
            (**parent).color = Color::Black;
            Self::set_color((*sibling).left, Color::Black);
            self.rotate_right(*parent);
            *parent = ptr::null_mut();
        }
    }

    /// Return (old_value, new)
    unsafe fn set_node(
        &mut self,
        node: *mut Node<K, V>,
        mut current: *mut Node<K, V>,
    ) -> Option<V> {
        if current.is_null() {
            self.root = node;
            self.size += 1;
            return None;
        }
        loop {
            match (*current).key.cmp(&(*node).key) {
                Ordering::Equal => {
                    self.replace(node, current);
                    return Some(Box::from_raw(current).value);
                }
                Ordering::Less => match (*current).right.is_null() {
                    true => {
                        (*current).right = node;
                        (*node).parent = current;
                        self.size += 1;
                        return None;
                    }
                    false => current = (*current).right,
                },
                Ordering::Greater => match (*current).left.is_null() {
                    true => {
                        (*current).left = node;
                        (*node).parent = current;
                        self.size += 1;
                        return None;
                    }
                    false => current = (*current).left,
                },
            };
        }
    }

    unsafe fn new_node(key: K, value: V) -> *mut Node<K, V> {
        let node = Node {
            key,
            value,
            color: Color::Red,
            right: ptr::null_mut(),
            left: ptr::null_mut(),
            parent: ptr::null_mut(),
        };
        return Box::leak(Box::new(node));
    }

    unsafe fn remove_node(
        &mut self,
        node: *mut Node<K, V>,
    ) -> RemoveNodeResult<K, V> {
        if (*node).left.is_null() {
            return self.remove_node_right(node);
        } else if (*node).right.is_null() {
            return self.remove_node_left(node);
        } else {
            return self.remove_node_min_right(node);
        }
    }

    unsafe fn remove_node_left(
        &mut self,
        node: *mut Node<K, V>,
    ) -> RemoveNodeResult<K, V> {
        let direction = match (*node).parent.is_null() {
            true => None,
            false => Some(Self::get_direction(node)),
        };
        self.transplant((*node).left, node);
        return RemoveNodeResult {
            color: (*node).color,
            current: (*node).left,
            parent: (*node).parent,
            direction,
        };
    }

    unsafe fn remove_node_right(
        &mut self,
        node: *mut Node<K, V>,
    ) -> RemoveNodeResult<K, V> {
        let direction = match (*node).parent.is_null() {
            true => None,
            false => Some(Self::get_direction(node)),
        };
        self.transplant((*node).right, node);
        return RemoveNodeResult {
            color: (*node).color,
            current: (*node).right,
            parent: (*node).parent,
            direction,
        };
    }

    unsafe fn remove_node_min_right(
        &mut self,
        node: *mut Node<K, V>,
    ) -> RemoveNodeResult<K, V> {
        let min_node = Self::min_node_right((*node).right);
        let color = (*min_node).color;
        let parent = match (*min_node).parent == node {
            true => min_node,
            false => (*min_node).parent,
        };
        let direction = Some(Self::get_direction(min_node));
        let current = (*min_node).right;
        self.transplant(current, min_node);
        self.relocate(min_node, node);
        return RemoveNodeResult {
            color,
            current,
            parent,
            direction,
        };
    }

    unsafe fn rotate_left(&mut self, node: *mut Node<K, V>) {
        let successor = match (*node).right.is_null() {
            true => panic!("expect: right child"),
            false => (*node).right,
        };
        (*node).right = (*successor).left;
        if !(*successor).left.is_null() {
            (*(*successor).left).parent = node;
        }
        (*successor).parent = (*node).parent;
        match (*node).parent.is_null() {
            true => self.root = successor,
            false => match Self::get_direction(node) {
                Direction::Left => (*(*node).parent).left = successor,
                Direction::Right => (*(*node).parent).right = successor,
            },
        }
        (*successor).left = node;
        (*node).parent = successor;
    }

    unsafe fn rotate_right(&mut self, node: *mut Node<K, V>) {
        let successor = match (*node).left.is_null() {
            true => panic!("expect: left child"),
            false => (*node).left,
        };
        (*node).left = (*successor).right;
        if !(*successor).right.is_null() {
            (*(*successor).right).parent = node;
        }
        (*successor).parent = (*node).parent;
        match (*node).parent.is_null() {
            true => self.root = successor,
            false => match Self::get_direction(node) {
                Direction::Left => (*(*node).parent).left = successor,
                Direction::Right => (*(*node).parent).right = successor,
            },
        };
        (*successor).right = node;
        (*node).parent = successor;
    }

    /// Retrieve which way of `node` from it's parent.
    unsafe fn get_direction(node: *mut Node<K, V>) -> Direction {
        let parent = match (*node).parent.is_null() {
            true => panic!("expect: a parent"),
            false => (*node).parent,
        };
        if (*parent).left == node {
            return Direction::Left;
        } else if (*parent).right == node {
            return Direction::Right;
        } else {
            panic!("expect: parent points to node");
        }
    }

    unsafe fn lookup(mut current: *mut Node<K, V>, key: &K) -> *mut Node<K, V>
    where
        K: Ord,
    {
        while !current.is_null() {
            match (*current).key.cmp(key) {
                Ordering::Equal => return current,
                Ordering::Less => current = (*current).right,
                Ordering::Greater => current = (*current).left,
            };
        }
        return ptr::null_mut();
    }

    unsafe fn get_color(node: *mut Node<K, V>) -> Color {
        match node.is_null() {
            true => return Color::Black,
            false => return (*node).color,
        };
    }

    unsafe fn set_color(node: *mut Node<K, V>, color: Color) {
        match node.is_null() {
            true => match color {
                Color::Black => return,
                Color::Red => panic!("expect: a node to set red color"),
            },
            false => (*node).color = color,
        }
    }

    unsafe fn get_child(
        node: *mut Node<K, V>,
        direction: Direction,
    ) -> *mut Node<K, V> {
        match direction {
            Direction::Left => return (*node).left,
            Direction::Right => return (*node).right,
        }
    }

    unsafe fn min_node_right(mut node: *mut Node<K, V>) -> *mut Node<K, V>
    where
        K: Ord,
    {
        assert!(!node.is_null(), "expect: not null top");
        while !(*node).left.is_null() {
            node = (*node).left;
        }
        return node;
    }
}

impl<K, V, const N: usize> From<[RawPair<K, V>; N]> for RedBlackTree<K, V>
where
    K: Ord,
{
    /// Time complexity: O(n.log(n)).
    ///
    /// Space complexity: O(1).
    fn from(pairs: [RawPair<K, V>; N]) -> Self {
        return Self::from_iter(pairs.into_iter());
    }
}

impl<K, V> FromIterator<RawPair<K, V>> for RedBlackTree<K, V>
where
    K: Ord,
{
    /// Time complexity: O(n.log(n)).
    ///
    /// Space complexity: O(1).
    fn from_iter<I: IntoIterator<Item = RawPair<K, V>>>(iter: I) -> Self {
        let mut tree = RedBlackTree::new();
        for (key, value) in iter {
            tree.set(key, value);
        }
        return tree;
    }
}

impl<K, V> Eq for RedBlackTree<K, V>
where
    K: Ord,
    V: Eq,
{
}

impl<K, V> PartialEq for RedBlackTree<K, V>
where
    K: Ord,
    V: Eq,
{
    /// Time complexity: O(n.log(n)).
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

impl<K, V> Clone for RedBlackTree<K, V>
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

impl<K, V> Drop for RedBlackTree<K, V>
where
    K: Ord,
{
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(1).
    fn drop(&mut self) {
        self.clear();
    }
}
