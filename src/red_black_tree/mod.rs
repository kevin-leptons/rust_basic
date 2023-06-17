//! Red Black Tree - a data structure and related algorithms.
//!
//! This document - module level document does not contain other descriptions or
//! examples. To learn more about how to use, refer to specific Entry APIs which
//! is labeled `entry` in the following sections.
//!
//! If there is the first time, [Package Document](crate) is a good way to learn
//! more about how to explore APIs and access to [Entry APIs
//! List](crate#structs).

mod etc;
mod iter;
mod node;

use etc::Direction;
pub use etc::RawPair;
use iter::TravelNodePostIter;
pub use iter::{Iter, KeyIter, ValueIter};
use node::{Color, Node};

/// `entry` A container for pairs key-value.
///
/// The APIs is similar as [create::HashMap] but it guarantees time complexity
/// of O(log(n)) for almost all operations.
///
/// # Example
///
/// ```
/// use rust_basic::RedBlackTree;
///
/// let mut t = RedBlackTree::from([
///     (1, 7),
///     (3, 5),
///     (9, 2),
/// ]);
/// assert_eq!(t.get(&3), &5);
/// assert_eq!(t.min(), &1);
/// assert_eq!(t.max(), &9);
pub struct RedBlackTree<K, V>
where
    K: Ord,
{
    root: Option<*mut Node<K, V>>,
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

    /// Put a new pair key-value.
    ///
    /// Time complexity: O(log(n)).
    ///
    /// Space complexity: O(n).
    pub fn set(&mut self, key: K, value: V) -> Option<V> {
        unsafe {
            let (old, new) = self.insert_binary_tree(key, value);
            if (*new).parrent.is_none() {
                (*new).color = Color::Black;
            } else {
                self.fix_insertion(new);
            }
            return old;
        }
    }

    /// Borrow an immutable value .
    ///
    /// Time complexity: O(log(n)).
    ///
    /// Space complexity: O(n).
    pub fn get(&self, key: &K) -> &V {
        unsafe {
            match lookup(&self.root, key) {
                None => panic!("expect: an existing key"),
                Some(v) => return &(*v).value,
            };
        }
    }

    /// Borrow a mutable value.
    ///
    /// Time complexity: O(log(n)).
    ///
    /// Space complexity: O(n).
    pub fn get_mut(&mut self, key: &K) -> &mut V {
        unsafe {
            match lookup(&self.root, key) {
                None => panic!("expect: an existing key"),
                Some(v) => return &mut (*v).value,
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
            return lookup(&self.root, key).is_some();
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
            let mut c = match self.root {
                None => panic!("expect: non empty tree"),
                Some(v) => v,
            };
            loop {
                match (*c).left {
                    None => return &(*c).key,
                    Some(v) => c = v,
                };
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
            let mut c = match self.root {
                None => panic!("expect: non empty tree"),
                Some(v) => v,
            };
            loop {
                match (*c).right {
                    None => return &(*c).key,
                    Some(v) => c = v,
                };
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
            let z = match lookup(&self.root, key) {
                None => return None,
                Some(v) => v,
            };
            let mut x: Option<*mut Node<K, V>> = None;
            let mut xp: Option<*mut Node<K, V>> = None;
            let mut xd: Option<Direction> = None;
            let mut origin_color = (*z).color;
            if (*z).left.is_none() {
                x = (*z).right;
                xp = (*z).parrent;
                if xp.is_some() {
                    xd = Some(get_direction(z));
                }
                self.transplant(&x, z);
            } else if (*z).right.is_none() {
                x = (*z).left;
                xp = (*z).parrent;
                if xp.is_some() {
                    xd = Some(get_direction(z));
                }
                self.transplant(&x, z);
            } else {
                let y = min_node((*z).right.unwrap());
                origin_color = (*y).color;
                if (*y).parrent == Some(z) {
                    xp = Some(y);
                } else {
                    xp = (*y).parrent;
                }
                xd = Some(get_direction(y));
                x = (*y).right;
                self.transplant(&x, y);
                self.relocate(y, z);
            }
            if origin_color == Color::Black {
                if xp.is_some() {
                    self.fix_removal(xp.unwrap(), xd.unwrap());
                } else {
                    set_color(&x, Color::Black);
                }
            }
            (*z).left = None;
            (*z).right = None;
            (*z).parrent = None;
            let value = (*Box::from_raw(z)).value;
            self.size -= 1;
            return Some(value);
        }
    }

    /// Remove all pairs, drop them and give back memory to allocator.
    ///
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(1).
    pub fn clear(&mut self) {
        if self.root.is_none() {
            return;
        }
        unsafe {
            for node in TravelNodePostIter::new(self.root.unwrap()) {
                drop(Box::from_raw(node));
            }
        }
        self.root = None;
        self.size = 0;
    }

    /// Return old value.
    unsafe fn insert_binary_tree_recursive(
        &mut self,
        current: *mut Node<K, V>,
        new_node: *mut Node<K, V>,
    ) -> Option<V>
    where
        K: Ord,
    {
        let mut c = current;
        if (*new_node).key < (*c).key {
            match (*c).left {
                None => {
                    (*c).left = Some(new_node);
                    (*new_node).parrent = Some(current);
                    return None;
                }
                Some(v) => {
                    return self.insert_binary_tree_recursive(v, new_node);
                }
            };
        } else if (*new_node).key > (*c).key {
            match (*c).right {
                None => {
                    (*c).right = Some(new_node);
                    (*new_node).parrent = Some(current);
                    return None;
                }
                Some(v) => {
                    return self.insert_binary_tree_recursive(v, new_node);
                }
            };
        } else {
            self.replace(new_node, c);
            let b = Box::from_raw(c);
            return Some(b.value);
        }
    }

    unsafe fn replace(
        &mut self,
        source: *mut Node<K, V>,
        target: *mut Node<K, V>,
    ) {
        match (*target).parrent {
            None => self.root = Some(source),
            Some(v) => {
                if (*v).left == Some(target) {
                    (*v).left = Some(source);
                } else if (*v).right == Some(target) {
                    (*v).right = Some(source);
                } else {
                    panic!("unexpected: bad link");
                }
            }
        }
        (*source).parrent = (*target).parrent;
        (*source).left = (*target).left;
        match (*target).left {
            None => {}
            Some(v) => (*v).parrent = Some(source),
        };
        (*source).right = (*target).right;
        match (*target).right {
            None => {}
            Some(v) => (*v).parrent = Some(source),
        };
    }

    unsafe fn fix_insertion(&mut self, mut c: *mut Node<K, V>) {
        loop {
            let mut p = match (*c).parrent {
                None => break,
                Some(v) => v,
            };
            if (*p).color == Color::Black {
                break;
            }
            let mut g = match (*p).parrent {
                None => break,
                Some(v) => v,
            };
            match get_direction(p) {
                Direction::Right => {
                    let u = (*g).left;
                    if get_color(&u) == Color::Red {
                        set_color(&u, Color::Black);
                        (*p).color = Color::Black;
                        (*g).color = Color::Red;
                        c = g;
                    } else {
                        if get_direction(c) == Direction::Left {
                            c = p;
                            self.rotate_right(c);
                            p = (*c).parrent.clone().unwrap();
                            g = (*p).parrent.clone().unwrap();
                        }
                        (*p).color = Color::Black;
                        (*g).color = Color::Red;
                        self.rotate_left(g);
                    }
                }
                Direction::Left => {
                    let u = (*g).right;
                    if get_color(&u) == Color::Red {
                        set_color(&u, Color::Black);
                        (*p).color = Color::Black;
                        (*g).color = Color::Red;
                        c = g;
                    } else {
                        if get_direction(c) == Direction::Right {
                            c = p;
                            self.rotate_left(c);
                            p = (*c).parrent.clone().unwrap();
                            g = (*p).parrent.clone().unwrap();
                        }
                        (*p).color = Color::Black;
                        (*g).color = Color::Red;
                        self.rotate_right(g);
                    }
                }
            }
        }
        set_color(&self.root, Color::Black);
    }

    /// Link `target`'s parrent to `source` instead of `target`.
    ///
    /// Keep children unchange for both `source` and `target`.
    unsafe fn transplant(
        &mut self,
        source: &Option<*mut Node<K, V>>,
        target: *mut Node<K, V>,
    ) {
        let parrent = (*target).parrent;
        if parrent.is_none() {
            self.root = source.clone();
        } else {
            match get_direction(target) {
                Direction::Left => {
                    (*parrent.clone().unwrap()).left = source.clone();
                }
                Direction::Right => {
                    (*parrent.clone().unwrap()).right = source.clone();
                }
            };
        }
        if source.is_some() {
            (*source.unwrap()).parrent = parrent;
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
        match (*target).parrent {
            None => {
                self.root = Some(source);
            }
            Some(parrent) => {
                match get_direction(target) {
                    Direction::Left => (*parrent).left = Some(source),
                    Direction::Right => (*parrent).right = Some(source),
                };
            }
        };
        let mut s = source;
        (*s).parrent = (*target).parrent;
        (*s).left = (*target).left;
        if (*s).left.is_some() {
            (*(*s).left.unwrap()).parrent = Some(source);
        }
        (*s).right = (*target).right;
        if (*s).right.is_some() {
            (*(*s).right.unwrap()).parrent = Some(source);
        }
        (*s).color = (*target).color;
    }

    unsafe fn fix_removal(&mut self, xp: *mut Node<K, V>, xd: Direction) {
        let mut p = Some(xp);
        let mut d = xd;
        let mut x = None;
        loop {
            let mut xp = match p {
                None => break,
                Some(v) => v,
            };
            x = get_child(p.unwrap(), d.clone());
            if get_color(&x) == Color::Red {
                break;
            }
            if d == Direction::Left {
                let mut s = (*xp).right.unwrap();
                if (*s).color == Color::Red {
                    (*s).color = Color::Black;
                    (*xp).color = Color::Red;
                    self.rotate_left(xp);
                    s = (*xp).right.unwrap();
                }
                if get_color(&(*s).left) == Color::Black
                    && get_color(&(*s).right) == Color::Black
                {
                    (*s).color = Color::Red;
                    if (*xp).parrent.is_some() {
                        d = get_direction(xp);
                    }
                    p = (*xp).parrent;
                } else {
                    if get_color(&(*s).right) == Color::Black {
                        set_color(&(*s).left, Color::Black);
                        (*s).color = Color::Red;
                        self.rotate_right(s);
                        s = (*xp).right.unwrap();
                    }
                    (*s).color = (*xp).color;
                    (*xp).color = Color::Black;
                    set_color(&(*s).right, Color::Black);
                    self.rotate_left(xp);
                    p = None;
                }
            } else {
                let mut s = (*xp).left.unwrap();
                if (*s).color == Color::Red {
                    (*s).color = Color::Black;
                    (*xp).color = Color::Red;
                    self.rotate_right(xp);
                    s = (*xp).left.unwrap();
                }
                if get_color(&(*s).right) == Color::Black
                    && get_color(&(*s).left) == Color::Black
                {
                    (*s).color = Color::Red;
                    if (*xp).parrent.is_some() {
                        d = get_direction(xp);
                    }
                    p = (*xp).parrent;
                } else {
                    if get_color(&(*s).left) == Color::Black {
                        set_color(&(*s).right, Color::Black);
                        (*s).color = Color::Red;
                        self.rotate_left(s);
                        s = (*xp).left.unwrap();
                    }
                    (*s).color = (*xp).color;
                    (*xp).color = Color::Black;
                    set_color(&(*s).left, Color::Black);
                    self.rotate_right(xp);
                    p = None;
                }
            }
        }
        if p.is_none() {
            set_color(&self.root, Color::Black);
        } else {
            let x = get_child(p.unwrap(), d);
            set_color(&x, Color::Black);
        }
    }

    /// Return (old, new)
    unsafe fn insert_binary_tree(
        &mut self,
        key: K,
        value: V,
    ) -> (Option<V>, *mut Node<K, V>) {
        let b = Box::new(Node {
            key: key,
            value: value,
            color: Color::Red,
            right: None,
            left: None,
            parrent: None,
        });
        let node = Box::leak(b);
        if self.root.is_none() {
            self.root = Some(node);
            self.size += 1;
            return (None, node);
        } else {
            let old = self
                .insert_binary_tree_recursive(self.root.clone().unwrap(), node);
            if old.is_none() {
                self.size += 1;
            }
            return (old, node);
        }
    }

    unsafe fn rotate_left(&mut self, x: *mut Node<K, V>) {
        let y = match (*x).right {
            None => panic!("expect: right child"),
            Some(v) => v,
        };
        (*x).right = (*y).left;
        if (*y).left.is_some() {
            (*(*y).left.clone().unwrap()).parrent = Some(x);
        }
        (*y).parrent = (*x).parrent;
        if (*x).parrent.is_none() {
            self.root = Some(y);
        } else {
            match get_direction(x) {
                Direction::Left => {
                    (*(*x).parrent.clone().unwrap()).left = Some(y);
                }
                Direction::Right => {
                    (*(*x).parrent.clone().unwrap()).right = Some(y);
                }
            };
        }
        (*y).left = Some(x);
        (*x).parrent = Some(y);
    }

    unsafe fn rotate_right(&mut self, x: *mut Node<K, V>) {
        let y = match (*x).left {
            None => panic!("expect: left child"),
            Some(v) => v,
        };
        (*x).left = (*y).right;
        if (*y).right.is_some() {
            (*(*y).right.clone().unwrap()).parrent = Some(x);
        }
        (*y).parrent = (*x).parrent;
        if (*x).parrent.is_none() {
            self.root = Some(y);
        } else {
            match get_direction(x) {
                Direction::Left => {
                    (*(*x).parrent.clone().unwrap()).left = Some(y);
                }
                Direction::Right => {
                    (*(*x).parrent.clone().unwrap()).right = Some(y);
                }
            };
        }
        (*y).right = Some(x);
        (*x).parrent = Some(y);
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
        let mut t = RedBlackTree::new();
        for (key, value) in iter {
            t.set(key, value);
        }
        return t;
    }
}

impl<K, V> Drop for RedBlackTree<K, V>
where
    K: Ord,
{
    /// Time complexity: O(n)).
    ///
    /// Space complexity: O(1).
    fn drop(&mut self) {
        self.clear();
    }
}

/// Retrieve which way of `node` from it's parrent.
unsafe fn get_direction<K, V>(node: *mut Node<K, V>) -> Direction {
    let p = match (*node).parrent {
        None => panic!("expect: node must have a parrent"),
        Some(v) => v.clone(),
    };
    if (*p).left == Some(node) {
        return Direction::Left;
    } else if (*p).right == Some(node) {
        return Direction::Right;
    } else {
        panic!("expect: parrent points to node");
    }
}

unsafe fn lookup<K, V>(
    current: &Option<*mut Node<K, V>>,
    key: &K,
) -> Option<*mut Node<K, V>>
where
    K: Ord,
{
    let c = match current {
        None => return None,
        Some(v) => *v,
    };
    if (*c).key == *key {
        return Some(c);
    } else if (*c).key < *key {
        return lookup(&(*c).right, key);
    } else {
        return lookup(&(*c).left, key);
    }
}

unsafe fn min_node<K, V>(top: *mut Node<K, V>) -> *mut Node<K, V>
where
    K: Ord,
{
    match (*top).left {
        None => return top,
        Some(v) => return min_node(v),
    }
}

unsafe fn get_color<K, V>(node: &Option<*mut Node<K, V>>) -> Color {
    match node {
        None => return Color::Black,
        Some(v) => return (**v).color,
    };
}

unsafe fn set_color<K, V>(node: &Option<*mut Node<K, V>>, color: Color) {
    match node {
        None => match color {
            Color::Black => return,
            Color::Red => panic!("expect: a node to set red color"),
        },
        Some(v) => {
            (**v).color = color;
        }
    }
}

unsafe fn get_child<K, V>(
    parrent: *mut Node<K, V>,
    direction: Direction,
) -> Option<*mut Node<K, V>> {
    match direction {
        Direction::Left => return (*parrent).left,
        Direction::Right => return (*parrent).right,
    }
}
