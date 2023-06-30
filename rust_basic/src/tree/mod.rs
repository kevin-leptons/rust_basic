//! Tree - a data structure and related algorithms.
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
pub use iter::{ChildIter, TravelLevelIter, TravelPostIter, TravelPreIter};
use std::ptr::addr_of;

/// `entry` A container for a tree.
///
/// A tree is a structure includes `value` and `children`, where`children` is a
/// list of other trees. A tree has only one parent. And trees does not make any
/// cycles.
///
/// # Panic
///
/// * Call [add_child](Self::add_child) or [add_children](Self::add_children) to
///   a tree that is already has [usize::MAX] children;
/// * Call [remove_child](Self::remove_child) with index that is greater than or
///   equal to [children_size](Self::children_size).
///
/// # Example
///
/// ```
/// /// Build from bottom to top, then travel over the tree like this:
/// ///
/// ///            0
/// ///          /   \
/// ///         /     \
/// ///        /       \
/// ///       1         2
/// ///    /  |  \
/// ///   3   4   5
///
/// use rust_basic::{Tree, Vector};
///
/// let mut tree0 = Tree::new(0);
/// let mut tree1 = Tree::new(1);
/// let mut tree2 = Tree::new(2);
/// let mut tree3 = Tree::new(3);
/// let tree4 = Tree::new(4);
/// let tree5 = Tree::new(5);
/// tree1.add_children([tree3, tree4, tree5]);
/// tree0.add_children([tree1, tree2]);
/// let values: Vector<u32> = tree0.travel_post_order()
///     .map(|t| t.value)
///     .collect();
/// assert_eq!(values, Vector::from([3, 4, 5, 1, 2, 0]));
#[derive(Debug)]
pub struct Tree<T> {
    /// Data of this node.
    pub value: T,

    children: Vector<Tree<T>>,
}

impl<T> Tree<T> {
    /// * Create a new instance, no children, no memory allocation.
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn new(value: T) -> Self {
        return Self {
            value,
            children: Vector::new(),
        };
    }

    /// * Time complexity: O(1) or O(n).
    /// * Space complexity: O(1).
    pub fn add_child(&mut self, child: Tree<T>) {
        self.children.push_back(child);
    }

    /// * Time complexity: O(1) or O(n).
    /// * Space complexity: O(1).
    pub fn add_children<const N: usize>(&mut self, children: [Tree<T>; N]) {
        for child in children {
            self.children.push_back(child);
        }
    }

    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn get_child(&self, index: usize) -> &Tree<T> {
        return &self.children[index];
    }

    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn get_child_mut(&mut self, index: usize) -> &mut Tree<T> {
        return &mut self.children[index];
    }

    /// * Time complexity: O(1) or O(n).
    /// * Space complexity: O(1).
    pub fn remove_child(&mut self, index: usize) -> Tree<T> {
        return self.children.remove(index);
    }

    /// * For iteration over children in the tree.
    pub fn children(&self) -> ChildIter<T> {
        return ChildIter::new(&self.children);
    }

    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn children_size(&self) -> usize {
        return self.children.size();
    }

    /// * Travel over the tree by order: current tree, then children from index
    ///   `0`.
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    pub fn travel_pre_order(&self) -> TravelPreIter<T> {
        return TravelPreIter::new(self);
    }

    /// * Travel over the tree by order: children from index `0`, then current
    ///   tree.
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    pub fn travel_post_order(&self) -> TravelPostIter<T> {
        return TravelPostIter::new(self);
    }

    /// * Travel over the tree by order: tree level is increasing.
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    pub fn travel_level_order(&self) -> TravelLevelIter<T> {
        return TravelLevelIter::new(self);
    }
}

impl<T> PartialEq for Tree<T> {
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    fn eq(&self, other: &Self) -> bool {
        return addr_of!(*self) == addr_of!(*other);
    }
}
