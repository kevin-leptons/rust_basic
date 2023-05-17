use std::marker::PhantomData;

use crate::{Queue, Stack};

use super::node::Node;

pub(super) struct TravelNodePostIter<K, V> {
    main_stack: Stack<*mut Node<K, V>>,
    branch_stack: Stack<*mut Node<K, V>>,
}

impl<K, V> TravelNodePostIter<K, V> {
    pub fn new(root: *mut Node<K, V>) -> Self {
        return Self {
            main_stack: Stack::from([root]),
            branch_stack: Stack::new(),
        };
    }
}

impl<K, V> Iterator for TravelNodePostIter<K, V>
where
    K: Eq,
{
    type Item = *mut Node<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            loop {
                if self.main_stack.size() == 0 {
                    return None;
                }
                let main_top = *self.main_stack.top();
                if (*main_top).left.is_none() && (*main_top).right.is_none() {
                    self.main_stack.pop();
                    return Some(main_top);
                }
                if self.branch_stack.size() > 0 {
                    let branch_top = *self.branch_stack.top();
                    if (*branch_top).key == (*main_top).key {
                        self.main_stack.pop();
                        self.branch_stack.pop();
                        return Some(main_top);
                    }
                }
                self.branch_stack.push(main_top);
                if (*main_top).left.is_some() {
                    self.main_stack.push((*main_top).left.unwrap());
                }
                if (*main_top).right.is_some() {
                    self.main_stack.push((*main_top).right.unwrap());
                }
            }
        }
    }
}

/// For iteration over pairs in the red black tree.
pub struct Iter<'a, K, V> {
    queue: Queue<*mut Node<K, V>>,
    marker_k: PhantomData<&'a K>,
    marker_v: PhantomData<&'a V>,
}

impl<'a, K, V> Iter<'a, K, V> {
    pub(super) fn new(root: Option<*mut Node<K, V>>) -> Self {
        let queue = match root {
            None => Queue::new(),
            Some(v) => Queue::from([v]),
        };
        return Self {
            queue: queue,
            marker_k: PhantomData,
            marker_v: PhantomData,
        };
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.size() == 0 {
            return None;
        }
        unsafe {
            let top = self.queue.pop();
            if (*top).left.is_some() {
                self.queue.push((*top).left.unwrap());
            }
            if (*top).right.is_some() {
                self.queue.push((*top).right.unwrap());
            }
            return Some((&(*top).key, &(*top).value));
        }
    }
}

/// For iteration over keys in the red black tree.
pub struct KeyIter<'a, K, V> {
    queue: Queue<*mut Node<K, V>>,
    marker_k: PhantomData<&'a K>,
}

impl<'a, K, V> KeyIter<'a, K, V> {
    pub(super) fn new(root: Option<*mut Node<K, V>>) -> Self {
        let queue = match root {
            None => Queue::new(),
            Some(v) => Queue::from([v]),
        };
        return Self {
            queue: queue,
            marker_k: PhantomData,
        };
    }
}

impl<'a, K, V> Iterator for KeyIter<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.size() == 0 {
            return None;
        }
        unsafe {
            let top = self.queue.pop();
            if (*top).left.is_some() {
                self.queue.push((*top).left.unwrap());
            }
            if (*top).right.is_some() {
                self.queue.push((*top).right.unwrap());
            }
            return Some(&(*top).key);
        }
    }
}

/// For iteration over values in the red black tree.
pub struct ValueIter<'a, K, V> {
    queue: Queue<*mut Node<K, V>>,
    marker_v: PhantomData<&'a V>,
}

impl<'a, K, V> ValueIter<'a, K, V> {
    pub(super) fn new(root: Option<*mut Node<K, V>>) -> Self {
        let queue = match root {
            None => Queue::new(),
            Some(v) => Queue::from([v]),
        };
        return Self {
            queue: queue,
            marker_v: PhantomData,
        };
    }
}

impl<'a, K, V> Iterator for ValueIter<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.size() == 0 {
            return None;
        }
        unsafe {
            let top = self.queue.pop();
            if (*top).left.is_some() {
                self.queue.push((*top).left.unwrap());
            }
            if (*top).right.is_some() {
                self.queue.push((*top).right.unwrap());
            }
            return Some(&(*top).value);
        }
    }
}
