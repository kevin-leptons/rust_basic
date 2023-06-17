//! Queue - a data structure and related algorithms.
//!
//! This document - module level document does not contain other descriptions or
//! examples. To learn more about how to use, refer to specific Entry APIs which
//! is labeled `entry` in the following sections.
//!
//! If there is the first time, [Package Document](crate) is a good way to learn
//! more about how to explore APIs and access to [Entry APIs
//! List](crate#structs).

mod iter;

pub use iter::Iter;
use std::alloc::{self, Layout};
use std::ops::Index;
use std::{
    alloc::handle_alloc_error,
    ptr::{self, NonNull},
};

/// `entry` A container for first in - first out items.
///
/// # Overview
///
/// ```txt
///                +---+
///          +-> 0 | 3 |<--- top
///          |     |---|
/// index ---|-> 1 | 2 |
///          |     |---|
///          +-> 2 | 1 |<--- bottom
///                +---+
/// ```
///
/// # Example
///
/// ```
/// use rust_basic::Queue;
///
/// let mut q = Queue::from([1, 2]);
/// q.push(3);
/// assert_eq!(q.top(), &1);
/// assert_eq!(q.pop(), 1);
/// assert_eq!(q.pop(), 2);
/// assert_eq!(q.pop(), 3);
#[derive(Debug)]
pub struct Queue<T> {
    slots: NonNull<T>,
    top: usize,
    size: usize,
    capacity: usize,
}

impl<T> Queue<T> {
    /// Create a new empty container.
    ///
    ///  Time complexity: O(1).
    ///
    ///  Space complexity: O(1).
    pub fn new() -> Self {
        return Self {
            slots: NonNull::dangling(),
            top: 0,
            size: 0,
            capacity: 0,
        };
    }

    /// Quantity of items in the container.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn size(&self) -> usize {
        return self.size;
    }

    /// Borrow the top item.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn top(&self) -> &T {
        assert!(self.size > 0, "expect: non empty queue");
        return self.get(0);
    }

    /// Put a new item in the bottom.
    ///
    /// Time complexity: O(1) or O(n).
    ///
    /// Space complexity: O(1).
    pub fn push(&mut self, value: T) {
        self.expand();
        let mut new_bottom = self.top + self.size;
        if new_bottom >= self.capacity {
            new_bottom = new_bottom - self.capacity;
        }
        unsafe { ptr::write(self.slots.as_ptr().add(new_bottom), value) }
        self.size = self.size + 1;
    }

    /// Remove the top item and return it.
    ///
    /// Time complexity: O(1) or O(n).
    ///
    /// Space complexity: O(1).
    pub fn pop(&mut self) -> T {
        assert!(self.size > 0, "expect: non empty queue");
        let value = unsafe { ptr::read(self.slots.as_ptr().add(self.top)) };
        if (self.top + 1) >= self.capacity {
            self.top = 0;
        } else {
            self.top = self.top + 1;
        }
        self.size = self.size - 1;
        self.narrow();
        return value;
    }

    /// Borrow an item by index. The index begin from zero and point to the top.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn get(&self, index: usize) -> &T {
        assert!(index < self.size, "expect: `index` is less than size");
        let mut slot = self.top + index;
        if slot >= self.capacity {
            slot -= self.capacity;
        }
        return unsafe { self.slots.as_ptr().add(slot).as_ref().unwrap() };
    }

    ///  For iteration over items, from top to bottom.
    ///
    ///  Time complexity: O(1).
    ///
    ///  Space complexity: O(1).
    pub fn iter(&self) -> Iter<T> {
        return Iter::new(self);
    }

    /// Remove all items, drop them and give back memory to allocator.
    ///
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(1).
    pub fn clear(&mut self) {
        if self.size > 0 {
            let bottom = self.top + self.size;
            if bottom <= self.capacity {
                unsafe {
                    ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                        self.slots.as_ptr().add(self.top),
                        self.size,
                    ))
                }
            } else {
                let c1 = bottom - self.capacity;
                let c2 = self.capacity - self.top;
                unsafe {
                    ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                        self.slots.as_ptr(),
                        c1,
                    ));
                    ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                        self.slots.as_ptr().add(self.top),
                        c2,
                    ));
                }
            }
        }
        if self.capacity > 0 {
            let layout = Layout::array::<T>(self.capacity).unwrap();
            unsafe { alloc::dealloc(self.slots.as_ptr() as *mut u8, layout) }
        }
        self.size = 0;
        self.capacity = 0;
        self.slots = NonNull::dangling();
    }

    fn expand(&mut self) {
        if self.size < self.capacity {
            return;
        }
        let new_capacity = match self.capacity {
            0 => 1,
            _ => 2 * self.capacity,
        };
        let new_layout = Layout::array::<T>(new_capacity).unwrap();
        let memory = match self.capacity {
            0 => unsafe { alloc::alloc(new_layout) },
            _ => unsafe {
                let old_layout = Layout::array::<T>(self.capacity).unwrap();
                alloc::realloc(
                    self.slots.as_ptr() as *mut u8,
                    old_layout,
                    new_layout.size(),
                )
            },
        };
        self.slots = match NonNull::new(memory as *mut T) {
            None => handle_alloc_error(new_layout),
            Some(v) => v,
        };
        let bottom = self.top + self.size;
        if bottom > self.capacity {
            let n = bottom - self.capacity;
            unsafe {
                ptr::copy_nonoverlapping(
                    self.slots.as_ptr(),
                    self.slots.as_ptr().add(self.top + self.size - 1),
                    n,
                );
            }
        }
        self.capacity = new_capacity;
    }

    fn narrow(&mut self) {
        if self.size == 0 && self.capacity > 0 {
            let layout = Layout::array::<T>(self.capacity).unwrap();
            unsafe { alloc::dealloc(self.slots.as_ptr() as *mut u8, layout) }
            self.slots = NonNull::dangling();
            self.capacity = 0;
            self.top = 0;
            return;
        }
        let new_capacity = self.capacity / 2;
        if self.size >= new_capacity {
            return;
        }
        let bottom = self.top + self.size;
        if bottom >= self.capacity {
            return;
        }
        let old_layout = Layout::array::<T>(self.capacity).unwrap();
        let new_layout = Layout::array::<T>(new_capacity).unwrap();
        let memory = unsafe {
            ptr::copy(
                self.slots.as_ptr().add(self.top),
                self.slots.as_ptr(),
                self.size,
            );
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
        self.top = 0;
    }
}

impl<T, const N: usize> From<[T; N]> for Queue<T> {
    /// Create a new instance from an array. The first item of the array become
    /// the top item in the container.
    ///
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn from(value: [T; N]) -> Self {
        let mut q = Queue::<T>::new();
        for i in value {
            q.push(i);
        }
        return q;
    }
}

impl<T> FromIterator<T> for Queue<T> {
    /// Create a new instance from an array. The first item of the iterator
    /// become the top item in the container.
    ///
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut q = Queue::<T>::new();
        for i in iter {
            q.push(i);
        }
        return q;
    }
}

impl<T> Index<usize> for Queue<T> {
    type Output = T;

    /// Equivalent to [Self::get].
    fn index(&self, index: usize) -> &Self::Output {
        return self.get(index);
    }
}

impl<T> Clone for Queue<T>
where
    T: Clone,
{
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn clone(&self) -> Self {
        let mut q = Queue::<T>::new();
        for i in self.iter() {
            q.push(i.clone());
        }
        return q;
    }
}

impl<T> Drop for Queue<T> {
    /// Equivalent to [Self::clear].
    fn drop(&mut self) {
        self.clear();
    }
}
