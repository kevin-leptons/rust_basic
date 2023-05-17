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
use std::{
    alloc::handle_alloc_error,
    ptr::{self, NonNull},
};

/// `entry` A container for first in - first out items.
///
/// # Example
///
/// ```
/// use rust_basic::Queue;
///
/// let mut q = Queue::from(["one", "two", "three"]);
/// q.push("four");
/// assert_eq!(q.pop(), "one");
/// assert_eq!(q.pop(), "two");
/// assert_eq!(q.size(), 2);
#[derive(Debug)]
pub struct Queue<T> {
    slots: NonNull<T>,
    head: usize,
    size: usize,
    capacity: usize,
}

impl<T> Queue<T> {
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn new() -> Self {
        return Self {
            slots: NonNull::dangling(),
            head: 0,
            size: 0,
            capacity: 0,
        };
    }

    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn size(&self) -> usize {
        return self.size;
    }

    /// * Time complexity: O(1) or O(n).
    /// * Space complexity: O(1).
    pub fn push(&mut self, value: T) {
        self.expand();
        let mut new_tail = self.head + self.size;
        if new_tail >= self.capacity {
            new_tail = new_tail - self.capacity;
        }
        unsafe { ptr::write(self.slots.as_ptr().add(new_tail), value) }
        self.size = self.size + 1;
    }

    /// * Time complexity: O(1) or O(n).
    /// * Space complexity: O(1).
    pub fn pop(&mut self) -> T {
        assert!(self.size > 0, "expect: non empty queue");
        let value = unsafe { ptr::read(self.slots.as_ptr().add(self.head)) };
        if (self.head + 1) >= self.capacity {
            self.head = 0;
        } else {
            self.head = self.head + 1;
        }
        self.size = self.size - 1;
        self.narrow();
        return value;
    }

    /// * Retrieve an item in the queue by index. Index `0` points to top and
    ///   index `size - 1` points to bottom.
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn get(&self, index: usize) -> &T {
        assert!(index < self.size, "expect: `index` is less than size");
        let mut slot = self.head + index;
        if slot >= self.capacity {
            slot -= self.capacity;
        }
        return unsafe { self.slots.as_ptr().add(slot).as_ref().unwrap() };
    }

    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn top(&self) -> &T {
        assert!(self.size > 0, "expect: non empty queue");
        return self.get(0);
    }

    /// * For iteration over items of the queue, from top to bottom.
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn iter(&self) -> Iter<T> {
        return Iter::new(self);
    }

    /// * Remove all items from the queue, drop them and give back memory to
    ///   allocator.
    /// * Time complexity: O(n).
    /// * Space complexity: O(1).
    pub fn clear(&mut self) {
        if self.size > 0 {
            let tail = self.head + self.size;
            if tail <= self.capacity {
                unsafe {
                    ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                        self.slots.as_ptr().add(self.head),
                        self.size,
                    ))
                }
            } else {
                let c1 = tail - self.capacity;
                let c2 = self.capacity - self.head;
                unsafe {
                    ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                        self.slots.as_ptr(),
                        c1,
                    ));
                    ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                        self.slots.as_ptr().add(self.head),
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
        let tail = self.head + self.size;
        if tail > self.capacity {
            let n = tail - self.capacity;
            unsafe {
                ptr::copy_nonoverlapping(
                    self.slots.as_ptr(),
                    self.slots.as_ptr().add(self.head + self.size - 1),
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
            self.head = 0;
            return;
        }
        let new_capacity = self.capacity / 2;
        if self.size >= new_capacity {
            return;
        }
        let tail = self.head + self.size;
        if tail >= self.capacity {
            return;
        }
        let old_layout = Layout::array::<T>(self.capacity).unwrap();
        let new_layout = Layout::array::<T>(new_capacity).unwrap();
        let memory = unsafe {
            ptr::copy(
                self.slots.as_ptr().add(self.head),
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
        self.head = 0;
    }
}

impl<T, const N: usize> From<[T; N]> for Queue<T> {
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    fn from(value: [T; N]) -> Self {
        let mut q = Queue::<T>::new();
        for i in value {
            q.push(i);
        }
        return q;
    }
}

impl<T> FromIterator<T> for Queue<T> {
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut q = Queue::<T>::new();
        for i in iter {
            q.push(i);
        }
        return q;
    }
}

impl<T> Clone for Queue<T>
where
    T: Clone,
{
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    fn clone(&self) -> Self {
        let mut q = Queue::<T>::new();
        for i in self.iter() {
            q.push(i.clone());
        }
        return q;
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        self.clear();
    }
}
