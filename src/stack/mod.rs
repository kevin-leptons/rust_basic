//! Stack - a data structure and related algorithms.
//!
//! This document - module level document does not contain other descriptions or
//! examples. To learn more about how to use, refer to specific Entry APIs which
//! is labeled `entry` in the following sections.
//!
//! If there is the first time, [Package Document](crate) is a good way to learn
//! more about how to explore APIs and access to [Entry APIs
//! List](crate#structs).

mod iter;

pub use self::iter::Iter;
use std::{
    alloc::{self, handle_alloc_error, Layout},
    ptr::{self, NonNull},
};

/// `entry` A container for first in - last out items.
///
/// # Examples
///
/// ```
/// use rust_basic::Stack;
///
/// let mut s = Stack::from(["one", "two", "three"]);
/// s.push("four");
/// assert_eq!(s.pop(), "four");
/// assert_eq!(s.pop(), "three");
/// assert_eq!(s.top(), &"two");
/// assert_eq!(s.size(), 2);
#[derive(Debug)]
pub struct Stack<T> {
    slots: NonNull<T>,
    capacity: usize,
    top: usize,
}

impl<T> Stack<T> {
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn new() -> Self {
        return Self {
            slots: NonNull::dangling(),
            capacity: 0,
            top: 0,
        };
    }

    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn size(&self) -> usize {
        return self.top;
    }

    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn top(&self) -> &T {
        assert!(self.top > 0, "expect: non empty stack");
        return unsafe {
            self.slots.as_ptr().add(self.top - 1).as_ref().unwrap()
        };
    }

    /// * Time complexity: O(1) or O(n).
    /// * Space complexity: O(1) or O(n).
    pub fn push(&mut self, value: T) {
        self.expand_slots();
        unsafe { ptr::write(self.slots.as_ptr().add(self.top), value) }
        self.top = self.top + 1;
    }

    /// * Time complexity: O(1) or O(n).
    /// * Space complexity: O(1) or O(n).
    pub fn pop(&mut self) -> T {
        assert!(self.top > 0, "expect: non empty stack");
        let item = unsafe { ptr::read(self.slots.as_ptr().add(self.top - 1)) };
        self.top = self.top - 1;
        self.narrow_slots();
        return item;
    }

    /// * Index `0` points to the bottom item and index `size - 1` points to top
    ///   one.
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn get(&self, index: usize) -> &T {
        assert!(index < self.top, "expect: `index` is less than size");
        return unsafe { &*self.slots.as_ptr().add(index) };
    }

    /// * For iteration from bottom to top of the container.
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn iter(&self) -> Iter<T> {
        return Iter::new(self);
    }

    /// * Remove all items from the container, drop them, and give back memory to
    ///   allocator.
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    pub fn clear(&mut self) {
        if self.top > 0 {
            unsafe {
                ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                    self.slots.as_ptr(),
                    self.top,
                ))
            }
            self.top = 0;
        }
        if self.capacity > 0 {
            let layout = Layout::array::<T>(self.capacity).unwrap();
            unsafe { alloc::dealloc(self.slots.as_ptr() as *mut u8, layout) };
            self.slots = NonNull::dangling();
            self.capacity = 0;
        }
    }

    fn expand_slots(&mut self) {
        if (self.top + 1) < self.capacity {
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

    fn narrow_slots(&mut self) {
        if self.top == 0 && self.capacity > 0 {
            let old_layout = Layout::array::<T>(self.capacity).unwrap();
            unsafe {
                alloc::dealloc(self.slots.as_ptr() as *mut u8, old_layout)
            }
            self.capacity = 0;
            self.slots = NonNull::dangling();
            return;
        }
        let new_capacity = self.capacity / 2;
        if (self.top + 1) >= new_capacity {
            return;
        }
        let new_layout = Layout::array::<T>(new_capacity).unwrap();
        let old_layout = Layout::array::<T>(self.capacity).unwrap();
        let new_memory = unsafe {
            alloc::realloc(
                self.slots.as_ptr() as *mut u8,
                old_layout,
                new_layout.size(),
            )
        };
        self.slots = match NonNull::new(new_memory as *mut T) {
            None => handle_alloc_error(new_layout),
            Some(v) => v,
        };
        self.capacity = new_capacity;
    }
}

impl<T, const N: usize> From<[T; N]> for Stack<T>
where
    T: Clone,
{
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    fn from(value: [T; N]) -> Self {
        let mut s = Stack::<T>::new();
        for i in value.into_iter() {
            s.push(i);
        }
        return s;
    }
}

impl<T> FromIterator<T> for Stack<T>
where
    T: Clone,
{
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut s = Stack::<T>::new();
        for i in iter {
            s.push(i);
        }
        return s;
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T> Clone for Stack<T>
where
    T: Clone,
{
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    fn clone(&self) -> Self {
        let mut s = Stack::<T>::new();
        for i in self.iter() {
            s.push(i.clone());
        }
        return s;
    }
}
