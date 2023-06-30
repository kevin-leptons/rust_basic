use std::marker::PhantomData;

use super::Node;
use crate::Stack;

/// For iteration over pairs in a Binary Search Tree. It does not guarantee that
/// items will arrive in a specific order.
pub struct Iter<'a, K, V> {
    stack: Stack<*mut Node<K, V>>,
    _marker_key: PhantomData<&'a K>,
    _marker_value: PhantomData<&'a V>,
}

impl<'a, K, V> Iter<'a, K, V> {
    pub(super) fn new(root: *mut Node<K, V>) -> Self {
        let stack = match root.is_null() {
            true => Stack::new(),
            false => Stack::from([root]),
        };
        return Self {
            stack,
            _marker_key: PhantomData,
            _marker_value: PhantomData,
        };
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.size() == 0 {
            return None;
        }
        unsafe {
            let top = self.stack.pop();
            if !(*top).left.is_null() {
                self.stack.push((*top).left);
            }
            if !(*top).right.is_null() {
                self.stack.push((*top).right);
            }
            return Some((&(*top).key, &(*top).value));
        }
    }
}

/// For iteration over keys in the red black tree.
pub struct KeyIter<'a, K, V> {
    iter: Iter<'a, K, V>,
}

impl<'a, K, V> KeyIter<'a, K, V> {
    pub(super) fn new(root: *mut Node<K, V>) -> Self {
        return Self {
            iter: Iter::new(root),
        };
    }
}

impl<'a, K, V> Iterator for KeyIter<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        return match self.iter.next() {
            None => None,
            Some((key, _)) => Some(key),
        };
    }
}

/// For iteration over values in the red black tree.
pub struct ValueIter<'a, K, V> {
    iter: Iter<'a, K, V>,
}

impl<'a, K, V> ValueIter<'a, K, V> {
    pub(super) fn new(root: *mut Node<K, V>) -> Self {
        return Self {
            iter: Iter::new(root),
        };
    }
}

impl<'a, K, V> Iterator for ValueIter<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        return match self.iter.next() {
            None => None,
            Some((_, value)) => Some(value),
        };
    }
}

/// For iteration over nodes in a tree by post order.
pub(super) struct TravelPostOrderIter<K, V> {
    main_stack: Stack<*mut Node<K, V>>,
    branch_stack: Stack<*mut Node<K, V>>,
}

impl<K, V> TravelPostOrderIter<K, V> {
    pub fn new(root: *mut Node<K, V>) -> Self {
        let main_stack = match root.is_null() {
            true => Stack::new(),
            false => Stack::from([root]),
        };
        return Self {
            main_stack,
            branch_stack: Stack::new(),
        };
    }
}

impl<K, V> Iterator for TravelPostOrderIter<K, V>
where
    K: Eq,
{
    type Item = *mut Node<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            while self.main_stack.size() > 0 {
                let main_top = *self.main_stack.top();
                if (*main_top).left.is_null() && (*main_top).right.is_null() {
                    self.main_stack.pop();
                    return Some(main_top);
                }
                if self.branch_stack.size() > 0 {
                    let branch_top = *self.branch_stack.top();
                    if branch_top == main_top {
                        self.main_stack.pop();
                        self.branch_stack.pop();
                        return Some(main_top);
                    }
                }
                self.branch_stack.push(main_top);
                if !(*main_top).left.is_null() {
                    self.main_stack.push((*main_top).left);
                }
                if !(*main_top).right.is_null() {
                    self.main_stack.push((*main_top).right);
                }
            }
            return None;
        }
    }
}
