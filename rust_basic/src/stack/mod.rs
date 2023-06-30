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

pub use iter::Iter;
use std::alloc::{self, handle_alloc_error, Layout};
use std::mem;
use std::ops::Index;
use std::ptr::{self, NonNull};

/// `entry` A container for first in - last out items.
///
/// # Model
///
/// ```txt
///                +---+
///          +-> 2 | 3 |<--- top
///          |     |---|
/// index ---|-> 1 | 2 |
///          |     |---|
///          +-> 0 | 1 |<--- bottom
///                +---+
/// ```
///
/// # Panic
///
/// * Call [top](Self::top) or [pop](Self::pop) to an empty stack.
/// * Call [push](Self::push) to a stack that already has size [usize::MAX].
/// * Call [index](Self::index) with an index that is greater than or equal to
///   [size](Self::size).
/// * The stack is going to use more than [isize::MAX] bytes.
///
/// # Examples
///
/// ```
/// use rust_basic::Stack;
///
/// let mut stack = Stack::from([1, 2, 3]);
/// stack.push(4);
/// assert_eq!(stack.top(), &4);
/// assert_eq!(stack.pop(), 4);
/// assert_eq!(stack.pop(), 3);
/// assert_eq!(stack.pop(), 2);
/// assert_eq!(stack.pop(), 1);
#[derive(Debug)]
pub struct Stack<T> {
    slots: *mut T,
    capacity: usize,
    top: usize,
}

// With Zero Size Types, [Stack::slots] is set to a fake address and does not
// change during stack lifetime. The address is also aligned to type T. There
// are no actual alloc/delloc/read/write to that address. These rules guarantees
// that operations on the address work correctly. Since these types require no
// additional memory for new instances, [Stack::capacity] is set to [usize::MAX]
// and does not change during stack lifetime. This rule guarantees operations
// that relies on [Stack::capacity] works correctly.
impl<T> Stack<T> {
    /// Create an empty container.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn new() -> Self {
        let (slots, capacity) = match mem::size_of::<T>() {
            0 => (NonNull::dangling().as_ptr(), usize::MAX),
            _ => (ptr::null_mut(), 0),
        };
        return Self {
            slots,
            capacity,
            top: 0,
        };
    }

    /// Quantity of items in the container.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn size(&self) -> usize {
        return self.top;
    }

    /// Borrow the top item.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn top(&self) -> &T {
        assert!(self.top > 0, "expect: not empty stack");
        unsafe { return &*self.slots.add(self.top - 1) }
    }

    /// Put a new item on the top.
    ///
    /// Time complexity: O(1) or O(n).
    ///
    /// Space complexity: O(1) or O(n).
    pub fn push(&mut self, item: T) {
        assert!(self.top < usize::MAX, "expect: not full stack");
        unsafe {
            self.expand();
            ptr::write(self.slots.add(self.top), item);
            self.top += 1;
        }
    }

    /// Remove the top item and return it.
    ///
    /// Time complexity: O(1) or O(n).
    ///
    /// Space complexity: O(1) or O(n).
    pub fn pop(&mut self) -> T {
        assert!(self.top > 0, "expect: not empty stack");
        unsafe {
            let item = ptr::read(self.slots.add(self.top - 1));
            self.top -= 1;
            self.narrow();
            return item;
        }
    }

    /// For iteration over immutable items, from the bottom to the top.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn iter(&self) -> Iter<T> {
        return Iter::new(self);
    }

    /// Remove all items, drop them, and give back memory to allocator.
    ///
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    pub fn clear(&mut self) {
        if mem::size_of::<T>() == 0 {
            self.top = 0;
            return;
        }
        unsafe {
            if self.top > 0 {
                ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                    self.slots, self.top,
                ));
                self.top = 0;
            }
            if self.capacity > 0 {
                let layout = Layout::array::<T>(self.capacity).unwrap();
                alloc::dealloc(self.slots as *mut u8, layout);
                self.slots = ptr::null_mut();
                self.capacity = 0;
            }
        }
    }

    unsafe fn expand(&mut self) {
        if mem::size_of::<T>() == 0 {
            return;
        }
        if (self.top + 1) < self.capacity {
            return;
        }
        let new_capacity = match self.capacity {
            0 => 1,
            _ => 2 * self.capacity,
        };
        let new_layout = Layout::array::<T>(new_capacity).unwrap();
        assert!(
            new_layout.size() <= isize::MAX as usize,
            "expect: smaller memory block"
        );
        let new_slot = match self.capacity {
            0 => alloc::alloc(new_layout),
            _ => {
                let old_layout = Layout::array::<T>(self.capacity).unwrap();
                alloc::realloc(
                    self.slots as *mut u8,
                    old_layout,
                    new_layout.size(),
                )
            }
        };
        if new_slot.is_null() {
            handle_alloc_error(new_layout)
        }
        self.slots = new_slot as *mut T;
        self.capacity = new_capacity;
    }

    unsafe fn narrow(&mut self) {
        if mem::size_of::<T>() == 0 {
            return;
        }
        if self.top == 0 && self.capacity > 0 {
            let old_layout = Layout::array::<T>(self.capacity).unwrap();
            alloc::dealloc(self.slots as *mut u8, old_layout);
            self.capacity = 0;
            self.slots = ptr::null_mut();
            return;
        }
        let new_capacity = self.capacity / 2;
        if (self.top + 1) >= new_capacity {
            return;
        }
        let new_layout = Layout::array::<T>(new_capacity).unwrap();
        assert!(
            new_layout.size() <= isize::MAX as usize,
            "expect: smaller memory block"
        );
        let old_layout = Layout::array::<T>(self.capacity).unwrap();
        let new_slots = alloc::realloc(
            self.slots as *mut u8,
            old_layout,
            new_layout.size(),
        );
        if new_slots.is_null() {
            handle_alloc_error(new_layout);
        }
        self.slots = new_slots as *mut T;
        self.capacity = new_capacity;
    }
}

impl<T> FromIterator<T> for Stack<T> {
    /// Create a new instance from an iterator. The first item in the iterator
    /// become the bottom item in the container.
    ///
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut stack = Stack::<T>::new();
        for item in iter {
            stack.push(item);
        }
        return stack;
    }
}

impl<T, const N: usize> From<[T; N]> for Stack<T> {
    /// Create a new instance from an array. The first item in the array become
    /// the bottom item in the container.
    ///
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn from(items: [T; N]) -> Self {
        return Self::from_iter(items.into_iter());
    }
}

impl<T> Index<usize> for Stack<T> {
    type Output = T;

    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.top, "expect: valid index");
        return unsafe { &*self.slots.add(index) };
    }
}

impl<T> Clone for Stack<T>
where
    T: Clone,
{
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn clone(&self) -> Self {
        return Self::from_iter(self.iter().map(|item| item.clone()));
    }
}

impl<T> Drop for Stack<T> {
    /// Equivalent to [Self::clear].
    fn drop(&mut self) {
        self.clear();
    }
}
