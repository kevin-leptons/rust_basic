//! Grid - a data structure and related algorithms.
//!
//! This document - module level document does not contain other descriptions or
//! examples. To learn more about how to use, refer to specific Entry APIs which
//! is labeled `entry` in the following sections.
//!
//! If there is the first time, [Package Document](crate) is a good way to learn
//! more about how to explore APIs and access to [Entry APIs
//! List](crate#structs).

mod iter;

use std::alloc::{self, handle_alloc_error, Layout};
use std::cmp::min;
use std::ptr::{self, NonNull};

pub use self::iter::CellIterMut;

/// `entry` A container for items that is indexed in 2-dimensional coordinate.
///
/// # Example
///
/// ```
/// use rust_basic::Grid;
///
/// let mut g = Grid::<u32>::from([
///     [0, 1, 2],
///     [3, 4, 5],
///     [6, 7, 8],
///     [9, 0, 0],
/// ]);
/// assert_eq!(g.size_x(), 3);
/// assert_eq!(g.size_y(), 4);
/// assert_eq!(g.get(1, 0), &1);
/// g.set(1, 0, 100);
/// assert_eq!(g.get(1, 0), &100);
#[derive(Debug)]
pub struct Grid<T> {
    cells: NonNull<T>,
    size_x: usize,
    size_y: usize,
}

impl<T> Grid<T> {
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn new() -> Self {
        return Self {
            cells: NonNull::dangling(),
            size_x: 0,
            size_y: 0,
        };
    }

    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn size_x(&self) -> usize {
        return self.size_x;
    }

    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn size_y(&self) -> usize {
        return self.size_y;
    }

    /// * Put a new item at coordinate `(x, y)`. Old item will be drop.
    /// * Time complexity: O(1).
    /// * Space complexity: O(n).
    pub fn set(&mut self, x: usize, y: usize, value: T) {
        let index = self.get_index(x, y);
        unsafe {
            let pointer = self.cells.as_ptr().add(index);
            ptr::drop_in_place(pointer);
            ptr::write(pointer, value);
        }
    }

    /// * Time complexity: O(1).
    /// * Space complexity: O(n).
    pub fn get(&self, x: usize, y: usize) -> &T {
        let index = self.get_index(x, y);
        return unsafe { &*self.cells.as_ptr().add(index) };
    }

    /// * Time complexity: O(1).
    /// * Space complexity: O(n).
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        let index = self.get_index(x, y);
        return unsafe { &mut *self.cells.as_ptr().add(index) };
    }

    /// * Time complexity: O(1).
    /// * Space complexity: O(n).
    pub fn cells_mut(&mut self) -> CellIterMut<T> {
        return CellIterMut::new(self);
    }

    /// * Arrange a new size for the container. If the new one is smaller then
    ///   old values will be drop. If the new one is greater then new values is
    ///   set to default.
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    pub fn resize(&mut self, x: usize, y: usize)
    where
        T: Default,
    {
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
        let memory = unsafe { alloc::alloc(new_layout) };
        let new_cells = match NonNull::new(memory as *mut T) {
            None => handle_alloc_error(new_layout),
            Some(v) => v,
        };
        self.move_to(&new_cells, x, y);
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
        let size = self.size_x * self.size_y;
        if size == 0 {
            return;
        }
        unsafe {
            ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                self.cells.as_ptr(),
                size,
            ));
            let layout = Layout::array::<T>(size).unwrap();
            alloc::dealloc(self.cells.as_ptr() as *mut u8, layout);
        }
        self.size_x = 0;
        self.size_y = 0;
        self.cells = NonNull::dangling();
    }

    fn move_to(&mut self, cells: &NonNull<T>, size_x: usize, size_y: usize)
    where
        T: Default,
    {
        let min_x = min(self.size_x, size_x);
        let min_y = min(self.size_y, size_y);
        for y in 0..min_y {
            let old_index = y * self.size_x;
            let new_index = y * size_x;
            let source = unsafe { self.cells.as_ptr().add(old_index) };
            let target = unsafe { cells.as_ptr().add(new_index) };
            unsafe { ptr::copy_nonoverlapping(source, target, min_x) }
            if size_x < self.size_x {
                let delta = self.size_x - size_x;
                unsafe {
                    ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                        source.add(min_x),
                        delta,
                    ));
                }
            }
            if size_x > self.size_x {
                for i in self.size_x..size_x {
                    unsafe { ptr::write(target.add(i), T::default()) }
                }
            }
        }
        if size_y < self.size_y {
            for y in min_y..self.size_y {
                let index = y * self.size_x;
                unsafe {
                    ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                        self.cells.as_ptr().add(index),
                        self.size_x,
                    ))
                }
            }
        }
        if size_y > self.size_y {
            for y in min_y..size_y {
                for x in 0..size_x {
                    let index = y * size_x + x;
                    unsafe {
                        ptr::write(cells.as_ptr().add(index), T::default())
                    }
                }
            }
        }
        let old_size = self.size_x * self.size_y;
        if old_size > 0 {
            let old_layout = Layout::array::<T>(old_size).unwrap();
            unsafe {
                alloc::dealloc(self.cells.as_ptr() as *mut u8, old_layout)
            }
        }
        self.cells = NonNull::dangling();
        self.size_x = 0;
        self.size_y = 0;
    }
}

impl<T, const X: usize, const Y: usize> From<[[T; X]; Y]> for Grid<T>
where
    T: Clone + Default,
{
    /// * Create an instance from a 2-dimensional array where `value[i]` is a
    ///   row and `value[i][k]` is a cell in the row.
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
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
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    fn clone(&self) -> Self {
        let mut g = Grid::<T>::new();
        g.resize(self.size_x, self.size_y);
        for x in 0..self.size_x {
            for y in 0..self.size_y {
                g.set(x, y, self.get(x, y).clone());
            }
        }
        return g;
    }
}

impl<T> PartialEq for Grid<T>
where
    T: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.size_x != other.size_x || self.size_y != other.size_y {
            return false;
        }
        for x in 0..self.size_x {
            for y in 0..self.size_y {
                if self.get(x, y) != other.get(x, y) {
                    return false;
                }
            }
        }
        return true;
    }
}

impl<T> Drop for Grid<T> {
    fn drop(&mut self) {
        self.resize_zero();
    }
}