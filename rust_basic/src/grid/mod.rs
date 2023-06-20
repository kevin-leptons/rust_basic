//! Grid - a data structure and related algorithms.
//!
//! This document - module level document does not contain other descriptions or
//! examples. To learn more about how to use, refer to specific Entry APIs which
//! is labeled `entry` in the following sections.
//!
//! If there is the first time, [Package Document](crate) is a good way to learn
//! more about how to explore APIs and access to [Entry APIs
//! List](crate#structs).

use core::slice;
use std::alloc::{self, handle_alloc_error, Layout};
use std::cmp::min;
use std::mem;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::ptr::{self, NonNull};

/// `entry` A container for items that is indexed in 2-dimensional coordinate.
///
/// # Model
///
/// ```txt
/// +---------------------- y-axis
/// |       +-------------- x-axis
/// |       |   |   |   |
/// |       v   v   v   v
/// |       0   1   2   3
/// |     +---+---+---+---+
/// |-> 0 | 1 | 2 | 3 | 4 |
/// |     +---+---+---+---+
/// |-> 1 | 5 | 6 | 7 | 8 |
/// |     +---+---+---+---+
/// +-> 2 | 9 | 8 | 7 | 6 |
///       +---+---+---+---+
/// ```
///
/// # Panic
///
/// * Call [get](Self::set), [index](Self::index) or
///   [index_mut](Self::index_mut) with `x`, `y` that is greater than or equal
///   to [size_x](Self::size_x) and [size_y](Self::size_y) respectively.
/// * Call to [resize](Self::resize) and make the grid uses more than
///   [isize::MAX] bytes.
///
/// # Example
///
/// ```
/// use rust_basic::Grid;
///
/// let mut grid = Grid::from([
///     [1, 2, 3, 4],
///     [5, 6, 7, 8],
///     [9, 8, 7, 6],
/// ]);
/// assert_eq!(grid.size_x(), 4);
/// assert_eq!(grid.size_y(), 3);
/// assert_eq!(grid[(3, 0)], 4);
/// grid[(3, 0)] = 40;
/// assert_eq!(grid[(3, 0)], 40);
#[derive(Debug)]
pub struct Grid<T> {
    cells: *mut T,
    size_x: usize,
    size_y: usize,
}

// With Zero Size Types, [Grid::cells] is set to a fake address and does not
// change during stack lifetime. The address is also aligned to type T. There
// are no actual alloc/delloc/read/write to that address. These rules guarantees
// that operations on the address work correctly.
impl<T> Grid<T> {
    /// Create a new empty container.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn new() -> Self {
        let cells = match mem::size_of::<T>() {
            0 => NonNull::dangling().as_ptr(),
            _ => ptr::null_mut(),
        };
        return Self {
            cells,
            size_x: 0,
            size_y: 0,
        };
    }

    /// Quantity of items on x-axis.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn size_x(&self) -> usize {
        return self.size_x;
    }

    /// Quantity of items on y-axis.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn size_y(&self) -> usize {
        return self.size_y;
    }

    /// Put a new item at `(x, y)`. The old item will be drop.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(n).
    pub fn set(&mut self, x: usize, y: usize, item: T) -> T {
        let index = self.get_index(x, y);
        unsafe {
            let pointer = self.cells.add(index);
            let old_item = ptr::read(pointer);
            ptr::write(pointer, item);
            return old_item;
        }
    }

    /// Arrange a new size for dimensionals. If dimensions are narrow, then old
    /// items will be dropped. If dimensions are expanded, then new items will
    /// be set as default.
    ///
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    pub fn resize(&mut self, x: usize, y: usize)
    where
        T: Default,
    {
        if mem::size_of::<T>() == 0 {
            self.size_x = x;
            self.size_y = y;
            return;
        }
        if x == self.size_x && y == self.size_y {
            return;
        }
        let new_size = x * y;
        if new_size == 0 {
            self.resize_zero();
            self.size_x = x;
            self.size_y = y;
            return;
        }
        let new_layout = Layout::array::<T>(new_size).unwrap();
        assert!(
            new_layout.size() <= isize::MAX as usize,
            "expect: smaller memory block"
        );
        let new_cells = unsafe { alloc::alloc(new_layout) as *mut T };
        if new_cells.is_null() {
            handle_alloc_error(new_layout);
        }
        unsafe { self.move_to(new_cells, x, y) }
        self.cells = new_cells;
        self.size_x = x;
        self.size_y = y;
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        assert!(
            (x < self.size_x) && (y < self.size_y),
            "expect: valid index"
        );
        return y * self.size_x + x;
    }

    fn resize_zero(&mut self) {
        if mem::size_of::<T>() == 0 {
            self.size_x = 0;
            self.size_y = 0;
            return;
        }
        let size = self.size_x * self.size_y;
        if size == 0 {
            return;
        }
        unsafe {
            ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.cells, size));
            let layout = Layout::array::<T>(size).unwrap();
            alloc::dealloc(self.cells as *mut u8, layout);
        }
        self.size_x = 0;
        self.size_y = 0;
        self.cells = ptr::null_mut();
    }

    unsafe fn move_to(&mut self, cells: *mut T, size_x: usize, size_y: usize)
    where
        T: Default,
    {
        let min_x = min(self.size_x, size_x);
        let min_y = min(self.size_y, size_y);
        for y in 0..min_y {
            let old_index = y * self.size_x;
            let new_index = y * size_x;
            let source = self.cells.add(old_index);
            let target = cells.add(new_index);
            ptr::copy_nonoverlapping(source, target, min_x);
            if size_x < self.size_x {
                let delta = self.size_x - size_x;
                ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                    source.add(min_x),
                    delta,
                ));
            }
            if size_x > self.size_x {
                for i in self.size_x..size_x {
                    ptr::write(target.add(i), T::default());
                }
            }
        }
        if size_y < self.size_y {
            for y in min_y..self.size_y {
                let index = y * self.size_x;
                ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                    self.cells.add(index),
                    self.size_x,
                ));
            }
        }
        if size_y > self.size_y {
            for y in min_y..size_y {
                for x in 0..size_x {
                    let index = y * size_x + x;
                    ptr::write(cells.add(index), T::default());
                }
            }
        }
        let old_size = self.size_x * self.size_y;
        if old_size > 0 {
            let old_layout = Layout::array::<T>(old_size).unwrap();
            alloc::dealloc(self.cells as *mut u8, old_layout);
        }
        self.cells = ptr::null_mut();
        self.size_x = 0;
        self.size_y = 0;
    }
}

impl<T, const X: usize, const Y: usize> From<[[T; X]; Y]> for Grid<T>
where
    T: Clone + Default,
{
    /// Create a new instance from a 2-dimensional array where `value[i]` is a
    /// row and `value[i][k]` is an item in the row.
    ///
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn from(value: [[T; X]; Y]) -> Self {
        let mut g = Grid::<T>::new();
        g.resize(X, Y);
        for y in 0..Y {
            for x in 0..X {
                g.set(x, y, value[y][x].clone());
            }
        }
        return g;
    }
}

impl<T> Clone for Grid<T>
where
    T: Clone + Default,
{
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn clone(&self) -> Self {
        let mut g = Grid::<T>::new();
        g.resize(self.size_x, self.size_y);
        for x in 0..self.size_x {
            for y in 0..self.size_y {
                g[(x, y)] = self[(x, y)].clone();
            }
        }
        return g;
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    /// Time complexity: O(1).
    ///
    /// Space complexity: O(n).
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        let index_1d = self.get_index(x, y);
        return unsafe { &*self.cells.add(index_1d) };
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(n).
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;
        let index_id = self.get_index(x, y);
        return unsafe { &mut *self.cells.add(index_id) };
    }
}

impl<T> Deref for Grid<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe {
            return slice::from_raw_parts(
                self.cells,
                self.size_x * self.size_y,
            );
        }
    }
}

impl<T> DerefMut for Grid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            return slice::from_raw_parts_mut(
                self.cells,
                self.size_x * self.size_y,
            );
        }
    }
}

impl<T> Eq for Grid<T> where T: Eq {}

impl<T> PartialEq for Grid<T>
where
    T: Eq,
{
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn eq(&self, other: &Self) -> bool {
        if self.size_x != other.size_x || self.size_y != other.size_y {
            return false;
        }
        for x in 0..self.size_x {
            for y in 0..self.size_y {
                if self[(x, y)] != other[(x, y)] {
                    return false;
                }
            }
        }
        return true;
    }
}

impl<T> Drop for Grid<T> {
    /// Equivalent to [Self::resize(0, 0)](Self::resize).
    fn drop(&mut self) {
        self.resize_zero();
    }
}
