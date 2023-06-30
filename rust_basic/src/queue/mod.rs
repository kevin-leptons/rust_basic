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
use std::alloc::{self, handle_alloc_error, Layout};
use std::mem;
use std::ops::Index;
use std::ptr::{self, NonNull};

/// `entry` A container for first in - first out items.
///
/// # Model
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
/// # Panic
///
/// * Call [top](Self::top) or [pop](Self::pop) to an empty queue.
/// * Call [index](Self::index) with an index that is greater than or equal to
///   [size](Self::size).
/// * Call [push](Self::push) to a queue that already has size [usize::MAX].
/// * The queue is going to use more than [isize::MAX] bytes.
///
/// # Example
///
/// ```
/// use rust_basic::Queue;
///
/// let mut queue = Queue::from([1, 2, 3]);
/// queue.push(4);
/// assert_eq!(queue.top(), &1);
/// assert_eq!(queue.pop(), 1);
/// assert_eq!(queue.pop(), 2);
/// assert_eq!(queue.pop(), 3);
/// assert_eq!(queue.pop(), 4);
#[derive(Debug)]
pub struct Queue<T>
where
    T: Sized,
{
    slots: *mut T,
    top: usize,
    size: usize,
    capacity: usize,
}

// With Zero Size Types, [Queue::slots] is set to a fake address and does not
// change during queue lifetime. The address is also aligned to type T. There
// are no actual alloc/delloc/read/write to that address. These rules guarantees
// that operations on the address work correctly. Since these types require no
// additional memory for new instances, [Queue::capacity] is set to [usize::MAX]
// and does not change during queue lifetime. This rule guarantees operations
// that relies on [Queue::capacity] works correctly.
impl<T> Queue<T> {
    /// Create a new empty container.
    ///
    ///  Time complexity: O(1).
    ///
    ///  Space complexity: O(1).
    pub fn new() -> Self {
        let (slots, capacity) = match mem::size_of::<T>() {
            0 => (NonNull::dangling().as_ptr(), usize::MAX),
            _ => (ptr::null_mut(), 0),
        };
        return Self {
            capacity,
            slots,
            top: 0,
            size: 0,
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
        assert!(self.size > 0, "expect: not empty queue");
        return &self[0];
    }

    /// Put a new item to the bottom.
    ///
    /// Time complexity: O(1) or O(n).
    ///
    /// Space complexity: O(1).
    pub fn push(&mut self, item: T) {
        assert!(self.size < usize::MAX, "expect: not full queue");
        unsafe {
            self.expand();
            let mut new_bottom = self.top + self.size;
            if new_bottom >= self.capacity {
                new_bottom -= self.capacity;
            }
            ptr::write(self.slots.add(new_bottom), item);
            self.size += 1;
        }
    }

    /// Remove the top item and return it.
    ///
    /// Time complexity: O(1) or O(n).
    ///
    /// Space complexity: O(1).
    pub fn pop(&mut self) -> T {
        assert!(self.size > 0, "expect: not empty queue");
        unsafe {
            let item = ptr::read(self.slots.add(self.top));
            if (self.top + 1) >= self.capacity {
                self.top = 0;
            } else {
                self.top += 1;
            }
            self.size -= 1;
            self.narrow();
            return item;
        }
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
        if mem::size_of::<T>() == 0 {
            self.top = 0;
            self.size = 0;
            return;
        }
        unsafe {
            if self.size > 0 {
                let bottom = self.top + self.size;
                if bottom <= self.capacity {
                    ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                        self.slots.add(self.top),
                        self.size,
                    ));
                } else {
                    ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                        self.slots,
                        bottom - self.capacity,
                    ));
                    ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                        self.slots.add(self.top),
                        self.capacity - self.top,
                    ));
                }
                self.size = 0;
            }
            if self.capacity > 0 {
                let layout = Layout::array::<T>(self.capacity).unwrap();
                alloc::dealloc(self.slots as *mut u8, layout);
                self.capacity = 0;
                self.slots = ptr::null_mut();
            }
        }
    }

    unsafe fn expand(&mut self) {
        if mem::size_of::<T>() == 0 {
            return;
        }
        if self.size < self.capacity {
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
        let new_slots = match self.capacity {
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
        if new_slots.is_null() {
            handle_alloc_error(new_layout);
        }
        self.slots = new_slots as *mut T;
        let bottom = self.top + self.size;
        if bottom > self.capacity {
            ptr::copy_nonoverlapping(
                self.slots,
                self.slots.add(self.capacity),
                bottom - self.capacity,
            );
        }
        self.capacity = new_capacity;
    }

    unsafe fn narrow(&mut self) {
        if mem::size_of::<T>() == 0 {
            return;
        }
        if self.size == 0 && self.capacity > 0 {
            let layout = Layout::array::<T>(self.capacity).unwrap();
            alloc::dealloc(self.slots as *mut u8, layout);
            self.slots = ptr::null_mut();
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
        ptr::copy(self.slots.add(self.top), self.slots, self.size);
        let old_layout = Layout::array::<T>(self.capacity).unwrap();
        let new_layout = Layout::array::<T>(new_capacity).unwrap();
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
        self.top = 0;
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
        let mut queue = Queue::<T>::new();
        for item in iter {
            queue.push(item);
        }
        return queue;
    }
}

impl<T, const N: usize> From<[T; N]> for Queue<T> {
    /// Create a new instance from an array. The first item of the array become
    /// the top item in the container.
    ///
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn from(items: [T; N]) -> Self {
        return Self::from_iter(items);
    }
}

impl<T> Index<usize> for Queue<T> {
    type Output = T;

    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.size, "expect: valid index");
        let mut actual_index = self.top + index;
        if actual_index >= self.capacity {
            actual_index -= self.capacity;
        }
        return unsafe { &*self.slots.add(actual_index) };
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
        return Self::from_iter(self.iter().map(|item| item.clone()));
    }
}

impl<T> Drop for Queue<T> {
    /// Equivalent to [Self::clear].
    fn drop(&mut self) {
        self.clear();
    }
}
