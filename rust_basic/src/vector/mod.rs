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
mod merge_sort;
mod quick_sort;

pub use iter::IntoIter;
use std::alloc::{self, handle_alloc_error, Layout};
use std::cmp::{min, Ordering};
use std::mem;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::ptr::{self, NonNull};

/// `entry` A container for items that is indexed by unsigned integer.
///
/// # Model
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
/// # Panic
///
/// * Call [insert](Self::insert), [push_back](Self::push_back),
///   [push_front](Self::push_front) to a vector that already has size
///   [usize::MAX].
/// * Call [insert](Self::insert) with index that is greater than
///   [size](Self::size).
/// * Call [index](Self::index), [index_mut](Self::index_mut),
///   [remove](Self::remove) or [swap](Self::swap) with index that is greater than
///   or equal to [size](Self::size).
/// * Call [pop_back](Self::pop_back) or [pop_front](Self::pop_front) to an
///   empty vector.
/// * The vector is going to use more than [isize::MAX] bytes.
///
/// # Example
///
/// ```
/// use rust_basic::Vector;
///
/// let mut vector = Vector::from([1, 2, 3]);
/// vector.push_front(4);
/// vector.push_back(5);
/// assert_eq!(vector[0], 4);
/// assert_eq!(vector[1], 1);
/// assert_eq!(vector[2], 2);
/// assert_eq!(vector[3], 3);
/// assert_eq!(vector[4], 5);
#[derive(Debug)]
pub struct Vector<T> {
    slots: *mut T,
    size: usize,
    capacity: usize,
}

// With Zero Size Types, [Vector::slots] is set to a fake address and does not
// change during vector lifetime. The address is also aligned to type T. There
// are no actual alloc/delloc/read/write to that address. These rules guarantees
// that operations on the address work correctly. Since these types require no
// additional memory for new instances, [Vector::capacity] is set to
// [usize::MAX] and does not change during vector lifetime. This rule guarantees
// operations that relies on [Vector::capacity] works correctly.
impl<'a, T: 'a> Vector<T> {
    /// Create a new empty instance.
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
            size: 0,
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
    pub fn insert(&mut self, index: usize, item: T) {
        assert!(index <= self.size, "expect: valid index");
        assert!(self.size < usize::MAX, "expect: not full vector");
        unsafe {
            self.expand();
            for i in (index..self.size).rev() {
                ptr::copy_nonoverlapping(
                    self.slots.add(i),
                    self.slots.add(i + 1),
                    1,
                );
            }
            ptr::write(self.slots.add(index), item);
            self.size += 1;
        }
    }

    /// Remove an item from the container and return it. All items at `[index,
    /// end]` has new indexes `i - 1` where `i` is old index.
    ///
    /// Time complexity: O(1) or O(n).
    ///
    /// Space complexity: O(n).
    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.size, "expect: valid index");
        unsafe {
            let item = ptr::read(self.slots.add(index));
            for i in (index + 1)..self.size {
                ptr::copy_nonoverlapping(
                    self.slots.add(i),
                    self.slots.add(i - 1),
                    1,
                );
            }
            self.size -= 1;
            self.narrow();
            return item;
        }
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
        assert!(first < self.size, "expect: valid indexes");
        assert!(second < self.size, "expect: valid indexes");
        unsafe {
            ptr::swap(self.slots.add(first), self.slots.add(second));
        }
    }

    /// Equivalent to [Self::insert(size, item)](Self::insert).
    pub fn push_back(&mut self, item: T) {
        self.insert(self.size, item);
    }

    /// Equivalent to [Self::insert(0, item)](Self::insert).
    pub fn push_front(&mut self, item: T) {
        self.insert(0, item);
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
                if self[k + 1] >= self[k] {
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
            let mut select = i;
            for k in (i + 1)..self.size {
                if self[k] < self[select] {
                    select = k;
                }
            }
            if select != i {
                self.swap(i, select);
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
        merge_sort::sort(self);
    }

    /// Sort items by Quick Sort algorithm.
    ///
    /// Time complexity: O(n.log(n)) or O(n^2).
    ///
    /// Space complexity: O(n).
    ///
    /// Stable: No.
    pub fn sort_quick(&mut self)
    where
        T: Ord,
    {
        quick_sort::sort(self);
    }

    /// Remove all items, drop them and give back memory to allocator.
    ///
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    pub fn clear(&mut self) {
        if mem::size_of::<T>() == 0 {
            self.size = 0;
            return;
        }
        unsafe {
            if self.size > 0 {
                ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                    self.slots, self.size,
                ));
                self.size = 0;
            }
            if self.capacity > 0 {
                let layout = Layout::array::<T>(self.capacity).unwrap();
                alloc::dealloc(self.slots as *mut u8, layout);
                self.slots = ptr::null_mut();
                self.capacity = 0;
            }
        }
    }

    fn expand(&mut self) {
        if mem::size_of::<T>() == 0 {
            return;
        }
        if (self.size + 1) <= self.capacity {
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
            0 => unsafe { alloc::alloc(new_layout) },
            _ => {
                let old_layout = Layout::array::<T>(self.capacity).unwrap();
                unsafe {
                    alloc::realloc(
                        self.slots as *mut u8,
                        old_layout,
                        new_layout.size(),
                    )
                }
            }
        };
        if new_slots.is_null() {
            handle_alloc_error(new_layout);
        }
        self.slots = new_slots as *mut T;
        self.capacity = new_capacity;
    }

    fn narrow(&mut self) {
        if mem::size_of::<T>() == 0 {
            return;
        }
        if self.capacity == 0 {
            return;
        }
        let new_capacity = self.capacity / 2;
        if new_capacity <= self.size {
            return;
        }
        let new_layout = Layout::array::<T>(new_capacity).unwrap();
        let old_layout = Layout::array::<T>(self.capacity).unwrap();
        let new_slots = unsafe {
            alloc::realloc(self.slots as *mut u8, old_layout, new_layout.size())
        };
        if new_slots.is_null() {
            handle_alloc_error(new_layout);
        }
        self.slots = new_slots as *mut T;
        self.capacity = new_capacity;
    }
}

impl<T> FromIterator<T> for Vector<T> {
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut vector = Vector::<T>::new();
        for item in iter {
            vector.push_back(item);
        }
        return vector;
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T> {
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn from(items: [T; N]) -> Self {
        return Self::from_iter(items);
    }
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;

    /// Time complexity: O(1).
    ///
    /// Space complexity: O(n).
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.size, "expect: valid index");
        unsafe {
            return &*self.slots.add(index);
        }
    }
}

impl<T> IndexMut<usize> for Vector<T> {
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(n).
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.size, "expect: valid index");
        unsafe {
            return &mut *self.slots.add(index);
        }
    }
}

impl<T> Deref for Vector<T> {
    type Target = [T];

    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    fn deref(&self) -> &Self::Target {
        unsafe {
            return std::slice::from_raw_parts(self.slots, self.size);
        }
    }
}

impl<'a, T> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            return std::slice::from_raw_parts_mut(self.slots, self.len());
        }
    }
}

impl<T> IntoIterator for Vector<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    fn into_iter(self) -> Self::IntoIter {
        return IntoIter::new(self);
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
        let min_size = min(self.size, other.size);
        for i in 0..min_size {
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
    T: Eq,
{
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }
        for i in 0..self.size {
            if self[i] != other[i] {
                return false;
            }
        }
        return true;
    }
}

impl<T> Eq for Vector<T> where T: Eq {}

impl<T> Clone for Vector<T>
where
    T: Clone,
{
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn clone(&self) -> Self {
        let mut vector = Vector::<T>::new();
        for item in self.iter() {
            vector.push_back(item.clone());
        }
        return vector;
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
