//! Vector - a data structure and related algorithms.
//!
//! This document - module level document does not contain other descriptions or
//! examples. To learn more about how to use, refer to specific Entry APIs which
//! is labeled `entry` in the following sections.
//!
//! If there is the first time, [Package Document](crate) is a good way to learn
//! more about how to explore APIs and access to [Entry APIs
//! List](crate#structs).

mod iter;
mod sort;

pub use iter::{Iter, IterMut};
use sort::{sort_merge, sort_quick};
use std::alloc::{self, handle_alloc_error, Layout};
use std::cmp::{min, Ordering};
use std::ops::Index;
use std::ptr::{self, NonNull};

/// `entry` A container for items that is indexed by unsigned integer.
///
/// # Overview
///
/// ```txt
///
///   +------------- front
///   |       +----- back
///   |       |
///   v       v
/// +---+---+---+
/// | 1 | 2 | 3 |
/// +---+---+---+
///   0   1   2
///   ^   ^   ^
///   |   |   |
///   +------------- index
/// ```
///
/// # Example
///
/// ```
/// use rust_basic::Vector;
///
/// let mut v = Vector::from([1, 2]);
/// v.push_front(3);
/// v.push_back(4);
/// assert_eq!(v[0], 3);
/// assert_eq!(v[1], 1);
/// assert_eq!(v[2], 2);
/// assert_eq!(v[3], 4);
#[derive(Debug)]
pub struct Vector<T> {
    slots: NonNull<T>,
    size: usize,
    capacity: usize,
}

impl<'a, T: 'a> Vector<T> {
    /// Create a new empty instance.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn new() -> Self {
        return Self {
            slots: NonNull::dangling(),
            size: 0,
            capacity: 0,
        };
    }

    /// Quantity of items.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn size(&self) -> usize {
        return self.size;
    }

    /// Put a new item into the container. Old items at `[index, end]` has new
    /// indexes `i + 1` where `i` is old index. The `index` points to new
    /// `item`.
    ///
    /// Time complexity: O(1) or O(n).
    ///
    /// Space complexity: O(n).
    pub fn set(&mut self, index: usize, item: T) {
        assert!(
            index <= self.size,
            "expect: `index` is not greater than size"
        );
        self.expand();
        unsafe {
            for i in (index..self.size).rev() {
                ptr::copy(
                    self.slots.as_ptr().add(i),
                    self.slots.as_ptr().add(i + 1),
                    1,
                );
            }
            ptr::write(self.slots.as_ptr().add(index), item);
        }
        self.size = self.size + 1;
    }

    /// Borrow a immutable item by index.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(n).
    pub fn get(&self, index: usize) -> &T {
        assert!(index < self.size, "expect: `index` is less than size");
        return unsafe { &*self.slots.as_ptr().add(index) };
    }

    /// Borrow a mutable item by index.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(n).
    pub fn get_mut(&mut self, index: usize) -> &mut T {
        assert!(index < self.size, "expect: `index` is less than size");
        return unsafe { &mut *self.slots.as_ptr().add(index) };
    }

    /// For iteration over immutable items.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn iter(&self) -> Iter<T> {
        return Iter::new(self);
    }

    /// For iteration over mutable items.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn iter_mut(&mut self) -> IterMut<T> {
        return IterMut::new(self);
    }

    /// Remove an item from the container and return it. All items at `[index,
    /// end]` has new indexes `i - 1` where `i` is old index.
    ///
    /// Time complexity: O(1) or O(n).
    ///
    /// Space complexity: O(n).
    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.size, "expect: `index` is less than size");
        let item = unsafe { ptr::read(self.slots.as_ptr().add(index)) };
        unsafe {
            for i in (index + 1)..self.size {
                ptr::copy_nonoverlapping(
                    self.slots.as_ptr().add(i),
                    self.slots.as_ptr().add(i - 1),
                    1,
                );
            }
        }
        self.size = self.size - 1;
        self.narrow();
        return item;
    }

    /// Move the item at index `first` to `second` and the item at index
    /// `second` to `first`.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(n).
    ///
    /// # Example
    ///
    /// ```
    /// use rust_basic::Vector;
    ///
    /// let mut v = Vector::from([1, 2, 3, 4]);
    /// v.swap(1, 2);
    /// assert_eq!(v, Vector::from([1, 3, 2, 4]));
    pub fn swap(&mut self, first: usize, second: usize) {
        assert!(first < self.size, "expect: `first` is less than size");
        assert!(second < self.size, "expect: `second` is less than size");
        unsafe {
            ptr::swap(
                self.slots.as_ptr().add(first),
                self.slots.as_ptr().add(second),
            )
        }
    }

    /// Equivalent to [Self::set(size, item)](Self::set).
    pub fn push_back(&mut self, item: T) {
        self.set(self.size, item);
    }

    /// Equivalent to [Self::set(0, item)](Self::set).
    pub fn push_front(&mut self, item: T) {
        self.set(0, item);
    }

    /// Equivalent to [Self::remove(size - 1, item)](Self::remove).
    pub fn pop_back(&mut self) -> T {
        return self.remove(self.size - 1);
    }

    /// Equivalent to [Self::remove(0, item)](Self::remove).
    pub fn pop_front(&mut self) -> T {
        return self.remove(0);
    }

    /// Perform sorting without specifying an algorithm. For now, it is
    /// equivalent to [Self::sort_quick].
    pub fn sort(&mut self)
    where
        T: Ord,
    {
        return self.sort_quick();
    }

    /// Sort items by Insertion Sort algorithm.
    ///
    /// Time complexity: O(n) or O(n^2).
    ///
    /// Space complexity: O(n).
    ///
    /// Stable: Yes.
    ///
    /// Direction: Increment.
    pub fn sort_insertion(&mut self)
    where
        T: Ord,
    {
        for i in 1..self.size {
            for k in (0..i).rev() {
                if self.get(k + 1) >= self.get(k) {
                    continue;
                }
                self.swap(k, k + 1);
            }
        }
    }

    /// Sort items by Selection Sort algorithm.
    ///
    /// Time complexity: O(n^2).
    ///
    /// Space complexity: O(n).
    ///
    /// Stable: No.
    ///
    /// Direction: Increment.
    pub fn sort_selection(&mut self)
    where
        T: Ord,
    {
        if self.size < 1 {
            return;
        }
        for i in 0..(self.size - 1) {
            let mut s = i;
            for k in (i + 1)..self.size {
                if self.get(k) < self.get(s) {
                    s = k;
                }
            }
            if s != i {
                self.swap(i, s);
            }
        }
    }

    /// Sort items by Merge Sort algorithm.
    ///
    /// Time complexity: O(n.log(n)).
    ///
    /// Space complexity: O(n).
    ///
    /// Stable: Yes.
    ///
    /// Direction: Increment.
    pub fn sort_merge(&mut self)
    where
        T: Ord,
    {
        sort_merge(self);
    }

    /// Sort items by Quick Sort algorithm.
    ///
    /// Time complexity: O(n.log(n)) or O(log(n^2)).
    ///
    /// Space complexity: O(n).
    ///
    /// Stable: No.
    pub fn sort_quick(&mut self)
    where
        T: Ord,
    {
        if self.size == 1 {
            return;
        }
        sort_quick(self, 0, self.size - 1);
    }

    /// Remove all items, drop them and give back memory to allocator.
    ///
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    pub fn clear(&mut self) {
        unsafe {
            if self.size > 0 {
                ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                    self.slots.as_ptr(),
                    self.size,
                ));
            }
            if self.capacity > 0 {
                let layout = Layout::array::<T>(self.capacity).unwrap();
                alloc::dealloc(self.slots.as_ptr() as *mut u8, layout);
            }
        }
        self.size = 0;
        self.slots = NonNull::dangling();
        self.capacity = 0;
    }

    fn expand(&mut self) {
        if (self.size + 1) <= self.capacity {
            return;
        }
        let new_capacity = match self.capacity {
            0 => 1,
            _ => 2 * self.capacity,
        };
        let new_layout = Layout::array::<T>(new_capacity).unwrap();
        let new_memory = match self.capacity {
            0 => unsafe { alloc::alloc(new_layout) },
            _ => {
                let old_layout = Layout::array::<T>(self.capacity).unwrap();
                unsafe {
                    alloc::realloc(
                        self.slots.as_ptr() as *mut u8,
                        old_layout,
                        new_layout.size(),
                    )
                }
            }
        };
        self.slots = match NonNull::new(new_memory as *mut T) {
            None => handle_alloc_error(new_layout),
            Some(v) => v,
        };
        self.capacity = new_capacity;
    }

    fn narrow(&mut self) {
        if self.capacity == 0 {
            return;
        }
        let new_capacity = self.capacity / 2;
        if new_capacity <= self.size {
            return;
        }
        let new_layout = Layout::array::<T>(new_capacity).unwrap();
        let old_layout = Layout::array::<T>(self.capacity).unwrap();
        let memory = unsafe {
            alloc::realloc(
                self.slots.as_ptr() as *mut u8,
                old_layout,
                new_layout.size(),
            )
        };
        self.slots = match NonNull::new(memory as *mut T) {
            None => handle_alloc_error(new_layout),
            Some(v) => v,
        };
        self.capacity = new_capacity;
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T> {
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn from(value: [T; N]) -> Self {
        let mut v = Vector::<T>::new();
        for i in value {
            v.push_back(i);
        }
        return v;
    }
}

impl<T> FromIterator<T> for Vector<T> {
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut v = Vector::<T>::new();
        for i in iter {
            v.push_back(i);
        }
        return v;
    }
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;

    /// Equivalent to [Self::get].
    fn index(&self, index: usize) -> &Self::Output {
        return self.get(index);
    }
}

impl<T> Ord for Vector<T>
where
    T: Ord,
{
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn cmp(&self, other: &Self) -> Ordering {
        let n = min(self.size, other.size);
        for i in 0..n {
            if self[i] > other[i] {
                return Ordering::Greater;
            } else if self[i] < other[i] {
                return Ordering::Less;
            }
        }
        return self.size.cmp(&other.size);
    }
}

impl<T> PartialOrd for Vector<T>
where
    T: Ord,
{
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl<T> PartialEq for Vector<T>
where
    T: Ord,
{
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn eq(&self, other: &Self) -> bool {
        return self.cmp(other) == Ordering::Equal;
    }
}

impl<T> Eq for Vector<T> where T: Ord {}

impl<T> Clone for Vector<T>
where
    T: Clone,
{
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn clone(&self) -> Self {
        let mut r = Vector::<T>::new();
        for v in self.iter() {
            r.push_back(v.clone());
        }
        return r;
    }
}

impl<T> Default for Vector<T> {
    /// Equivalent to [Self::new].
    fn default() -> Self {
        return Self::new();
    }
}

impl<T> Drop for Vector<T> {
    /// Equivalent to [Self::clear];
    fn drop(&mut self) {
        self.clear();
    }
}
