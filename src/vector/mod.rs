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
use std::ops::Index;
use std::ptr::{self, NonNull};

/// `entry` A container for items that is indexed by unsigned integer.
///
/// Reading or removing from index which is `index >= size` causes panic.
/// Writing to index which is `index > size` causes panic. All sorting
/// algorithms are incremental.
///
/// # Example
///
/// ```
/// use rust_basic::Vector;
///
/// let mut v = Vector::from([2, 1, 7, 3]);
/// v.push_back(5);
/// assert_eq!(v.get(0), &2);
/// assert_eq!(v[2], 7);
/// assert_eq!(v.remove(1), 1);
/// v.sort();
/// assert_eq!(v, Vector::from([2, 3, 5, 7]));
#[derive(Debug)]
pub struct Vector<T> {
    slots: NonNull<T>,
    size: usize,
    capacity: usize,
}

impl<'a, T: 'a> Vector<T> {
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn new() -> Self {
        return Self {
            slots: NonNull::dangling(),
            size: 0,
            capacity: 0,
        };
    }

    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn size(&self) -> usize {
        return self.size;
    }

    /// * Old items in `[index, end]` has new indexes `i + 1` where `i` is old
    ///   index.
    /// * The index `index` refers to new item `item`.
    /// * Time complexity: O(1) or O(n).
    /// * Space complexity: O(n).
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

    /// * Time complexity: O(1).
    /// * Space complexity: O(n).
    pub fn get(&self, index: usize) -> &T {
        assert!(index < self.size, "expect: `index` is less than size");
        return unsafe { &*self.slots.as_ptr().add(index) };
    }

    /// * Time complexity: O(1).
    /// * Space complexity: O(n).
    pub fn get_mut(&mut self, index: usize) -> &mut T {
        assert!(index < self.size, "expect: `index` is less than size");
        return unsafe { &mut *self.slots.as_ptr().add(index) };
    }

    /// * For iteration over immutable items in the vector.
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn iter(&self) -> Iter<T> {
        return Iter::new(self);
    }

    /// * For iteration over mutable items in the vector.
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn iter_mut(&mut self) -> IterMut<T> {
        return IterMut::new(self);
    }

    /// * All items in `[index, end]` has new indexes `i - 1` where `i` is old
    ///   index.
    /// * Time complexity: O(1) or O(n).
    /// * Space complexity: O(n).
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

    /// * The index `first` refers to item at index `second` and the index
    ///   `second` refers to item at index `first`.
    /// * Time complexity: O(1).
    /// * Space complexity: O(n).
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

    /// * Equivalent of `set(size, item)`.
    pub fn push_back(&mut self, item: T) {
        self.set(self.size, item);
    }

    /// * Equivalent of `set(0, item)`.
    pub fn push_front(&mut self, item: T) {
        self.set(0, item);
    }

    /// * Equivalent of `remove(size - 1)`.
    pub fn pop_back(&mut self) -> T {
        return self.remove(self.size - 1);
    }

    /// * Equivalent of `remove(0)`.
    pub fn pop_front(&mut self) -> T {
        return self.remove(0);
    }

    /// * Sorting without specifying an algorithm.
    /// * Equivalent as [Self::sort_quick], may be change in the future.
    pub fn sort(&mut self)
    where
        T: Ord,
    {
        return self.sort_quick();
    }

    /// * Time complexity: O(n) or O(n^2).
    /// * Space complexity: O(n).
    /// * Stable: Yes.
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

    /// * Time complexity: O(n^2).
    /// * Space complexity: O(n).
    /// * Stable: No.
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

    /// * Time complexity: O(n.log(n)).
    /// * Space complexity: O(n).
    /// * Stable: Yes.
    pub fn sort_merge(&mut self)
    where
        T: Ord,
    {
        sort_merge(self);
    }

    /// * Time complexity: O(n.log(n)) or O(log(n^2)).
    /// * Space complexity: O(n).
    /// * Stable: No.
    pub fn sort_quick(&mut self)
    where
        T: Ord,
    {
        if self.size == 1 {
            return;
        }
        sort_quick(self, 0, self.size - 1);
    }

    /// * Remove all items form the container, drop them and give back memory
    ///   to allocator.
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
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
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    fn from(value: [T; N]) -> Self {
        let mut v = Vector::<T>::new();
        for i in value {
            v.push_back(i);
        }
        return v;
    }
}

impl<T> FromIterator<T> for Vector<T> {
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
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

    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    fn index(&self, index: usize) -> &Self::Output {
        return self.get(index);
    }
}

impl<T> Eq for Vector<T> where T: Eq {}

impl<T> PartialEq for Vector<T>
where
    T: Eq,
{
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }
        for i in 0..self.size {
            if self.get(i) != other.get(i) {
                return false;
            }
        }
        return true;
    }
}

impl<T> Clone for Vector<T>
where
    T: Clone,
{
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    fn clone(&self) -> Self {
        let mut r = Vector::<T>::new();
        for v in self.iter() {
            r.push_back(v.clone());
        }
        return r;
    }
}

impl<T> Default for Vector<T> {
    fn default() -> Self {
        return Self::new();
    }
}

impl<T> Drop for Vector<T> {
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    fn drop(&mut self) {
        self.clear();
    }
}
