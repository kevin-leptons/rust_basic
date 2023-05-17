use crate::{
    vector::{self, Vector},
    Queue, Stack,
};

use super::Tree;

/// For iteration over children in a tree.
pub struct ChildIter<'a, T> {
    iter: vector::Iter<'a, Tree<T>>,
}

impl<'a, T> ChildIter<'a, T> {
    pub(super) fn new(children: &'a Vector<Tree<T>>) -> Self {
        return Self {
            iter: children.iter(),
        };
    }
}

impl<'a, T> Iterator for ChildIter<'a, T> {
    type Item = &'a Tree<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => return None,
            Some(v) => return Some(v),
        }
    }
}

/// For iteration over nodes of tree by pre order.
pub struct TravelPreIter<'a, T> {
    stack: Stack<&'a Tree<T>>,
}

impl<'a, T> TravelPreIter<'a, T> {
    pub(super) fn new(tree: &'a Tree<T>) -> Self {
        return Self {
            stack: Stack::from([tree]),
        };
    }
}

impl<'a, T> Iterator for TravelPreIter<'a, T> {
    type Item = &'a Tree<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.size() == 0 {
            return None;
        }
        let tree = self.stack.pop();
        for i in (0..tree.children_size()).rev() {
            self.stack.push(tree.get_child(i));
        }
        return Some(tree);
    }
}

/// For iteration over nodes of tree.
pub struct TravelPostIter<'a, T> {
    main_stack: Stack<&'a Tree<T>>,
    branch_stack: Stack<&'a Tree<T>>,
}

impl<'a, T> TravelPostIter<'a, T> {
    pub(super) fn new(tree: &'a Tree<T>) -> Self {
        return Self {
            main_stack: Stack::from([tree]),
            branch_stack: Stack::from([]),
        };
    }
}

impl<'a, T> Iterator for TravelPostIter<'a, T> {
    type Item = &'a Tree<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.main_stack.size() == 0 {
                return None;
            }
            if self.main_stack.top().children_size() == 0 {
                return Some(self.main_stack.pop());
            }
            if (self.branch_stack.size() > 0)
                && (self.main_stack.top() == self.branch_stack.top())
            {
                let next = self.main_stack.pop();
                self.branch_stack.pop();
                return Some(next);
            }
            self.branch_stack.push(self.main_stack.top());
            let children: Vector<&Tree<T>> =
                self.main_stack.top().children().collect();
            for i in (0..children.size()).rev() {
                self.main_stack.push(children.get(i));
            }
        }
    }
}

/// For iteration over nodes of tree by level order.
pub struct TravelLevelIter<'a, T> {
    queue: Queue<&'a Tree<T>>,
}

impl<'a, T> TravelLevelIter<'a, T> {
    pub(super) fn new(tree: &'a Tree<T>) -> Self {
        return Self {
            queue: Queue::from([tree]),
        };
    }
}

impl<'a, T> Iterator for TravelLevelIter<'a, T> {
    type Item = &'a Tree<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.size() == 0 {
            return None;
        }
        let tree = self.queue.pop();
        for child in tree.children() {
            self.queue.push(child);
        }
        return Some(tree);
    }
}
