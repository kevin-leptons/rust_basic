//! Sudoku - a game solver.
//!
//! This document - module level document does not contain other descriptions or
//! examples. To learn more about how to use, refer to specific Entry APIs which
//! is labeled `entry` in the following sections.
//!
//! If there is the first time, [Package Document](crate) is a good way to learn
//! more about how to explore APIs and access to [Entry APIs
//! List](crate#structs).

mod iter;
mod solver;

use crate::grid::Grid;
use crate::hash_set::HashSet;
use crate::sudoku::iter::{BlockIter, ColumnIter};
use crate::vector::Vector;
use iter::RowIter;
use solver::solve_backtrack;

const GRID_SIZE: usize = 9;

#[derive(Clone, Default)]
struct Cell {
    value: u8,
    locked: bool,
    fixed: bool,
    candidates: Vector<u8>,
}

/// `entry` A container for the game Sudoku and the solver as well.
///
/// # Example
///
/// ```
/// use rust_basic::sudoku::Sudoku;
///
/// let mut s = Sudoku::from([
///     [5, 3, 0, 0, 7, 0, 0, 0, 0],
///     [6, 0, 0, 1, 9, 5, 0, 0, 0],
///     [0, 9, 8, 0, 0, 0, 0, 6, 0],
///     [8, 0, 0, 0, 6, 0, 0, 0, 3],
///     [4, 0, 0, 8, 0, 3, 0, 0, 1],
///     [7, 0, 0, 0, 2, 0, 0, 0, 6],
///     [0, 6, 0, 0, 0, 0, 2, 8, 0],
///     [0, 0, 0, 4, 1, 9, 0, 0, 5],
///     [0, 0, 0, 0, 8, 0, 0, 7, 9],
/// ]);
/// let expected = [
///     [5, 3, 4, 6, 7, 8, 9, 1, 2],
///     [6, 7, 2, 1, 9, 5, 3, 4, 8],
///     [1, 9, 8, 3, 4, 2, 5, 6, 7],
///     [8, 5, 9, 7, 6, 1, 4, 2, 3],
///     [4, 2, 6, 8, 5, 3, 7, 9, 1],
///     [7, 1, 3, 9, 2, 4, 8, 5, 6],
///     [9, 6, 1, 5, 3, 7, 2, 8, 4],
///     [2, 8, 7, 4, 1, 9, 6, 3, 5],
///     [3, 4, 5, 2, 8, 6, 1, 7, 9],
/// ];
/// assert_eq!(s.solve_once(), true);
/// for x in 0..9 {
///     for y in 0..9 {
///         assert_eq!(s.get(x, y), expected[y][x]);
///     }
/// }
pub struct Sudoku {
    grid: Grid<Cell>,
}

impl Sudoku {
    /// * Create a new instance, zero value for each cell.
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    pub fn new() -> Self {
        let mut g = Grid::<Cell>::new();
        g.resize(GRID_SIZE, GRID_SIZE);
        return Self { grid: g };
    }

    /// * Retrieve a value at cell `(x, y)`.
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn get(&self, x: usize, y: usize) -> u8 {
        let s = self.grid.get(x, y);
        return s.value;
    }

    /// * Put a value at cell `(x, y)`. Value `0` make a cell become a
    ///  placeholder for solving later.
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        assert!(value <= 9, "expect: value is in [0, 9]");
        let s = self.grid.get_mut(x, y);
        s.value = value;
        s.locked = value != 0;
        s.fixed = value != 0;
    }

    /// * Find a solution for the current state.
    /// * Time complexity: O(9^n).
    /// * Space complexity: O(n).
    pub fn solve_once(&mut self) -> bool {
        self.initialize_candidates();
        return solve_backtrack(self);
    }

    /// * Check the current state is valid or not.
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    pub fn validate(&self) -> bool {
        return self.validate_numbers()
            && self.validate_rows()
            && self.validate_columns()
            && self.validate_blocks();
    }

    /// * Set placeholder cells to value `0`.
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    pub fn reset(&mut self) {
        for c in self.grid.cells_mut() {
            if c.fixed {
                continue;
            }
            c.value = 0;
            c.locked = false;
        }
    }

    fn valid_maybe(&self, x: usize, y: usize) -> bool {
        return self.valid_row_maybe(y)
            && self.valid_column_maybe(x)
            && self.valid_block_maybe(x, y);
    }

    fn validate_numbers(&self) -> bool {
        for y in 0..self.grid.size_y() {
            for x in 0..self.grid.size_x() {
                if self.grid.get(x, y).value == 0 {
                    return false;
                }
            }
        }
        return true;
    }

    fn validate_rows(&self) -> bool {
        for y in 0..GRID_SIZE {
            if !self.valid_row(y) {
                return false;
            }
        }
        return true;
    }

    fn valid_row(&self, y: usize) -> bool {
        let mut row = [0; GRID_SIZE];
        for x in 0..GRID_SIZE {
            row[x] = self.grid.get(x, y).value;
        }
        return Self::duplicate(&row) == false;
    }

    fn valid_row_maybe(&self, y: usize) -> bool {
        let mut s = HashSet::<u8>::new();
        for x in 0..GRID_SIZE {
            let v = self.grid.get(x, y).value;
            if v == 0 {
                continue;
            }
            if s.has(&v) {
                return false;
            }
            s.add(v);
        }
        return true;
    }

    fn validate_columns(&self) -> bool {
        for x in 0..GRID_SIZE {
            if !self.valid_column(x) {
                return false;
            }
        }
        return true;
    }

    fn valid_column(&self, x: usize) -> bool {
        let mut column = [0; GRID_SIZE];
        for y in 0..GRID_SIZE {
            column[y] = self.grid.get(x, y).value;
        }
        return Self::duplicate(&column) == false;
    }

    fn valid_column_maybe(&self, x: usize) -> bool {
        let mut s = HashSet::<u8>::new();
        for y in 0..GRID_SIZE {
            let v = self.grid.get(x, y).value;
            if v == 0 {
                continue;
            }
            if s.has(&v) {
                return false;
            }
            s.add(v);
        }
        return true;
    }

    fn validate_blocks(&self) -> bool {
        for by in 0..3 {
            for bx in 0..3 {
                let mut block = [0; GRID_SIZE];
                let gx = 3 * bx;
                let gy = 3 * by;
                let mut k = 0;
                for x in gx..(gx + 3) {
                    for y in gy..(gy + 3) {
                        block[k] = self.grid.get(x, y).value;
                        k += 1;
                    }
                }
                if Self::duplicate(&block) {
                    return false;
                }
            }
        }
        return true;
    }

    fn valid_block_maybe(&self, x: usize, y: usize) -> bool {
        let xb = x - x % 3;
        let yb = y - y % 3;
        let mut s = HashSet::<u8>::new();
        for x in xb..(xb + 3) {
            for y in yb..(yb + 3) {
                let v = self.grid.get(x, y).value;
                if v == 0 {
                    continue;
                }
                if s.has(&v) {
                    return false;
                }
                s.add(v);
            }
        }
        return true;
    }

    fn duplicate(list: &[u8; 9]) -> bool {
        let mut s = HashSet::<u8>::new();
        for v in list {
            if s.has(v) {
                return true;
            }
            s.add(*v);
        }
        return false;
    }

    fn initialize_candidates(&mut self) {
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                Self::initialize_candidate(self, x, y);
            }
        }
    }

    fn initialize_candidate(&mut self, x: usize, y: usize) {
        if self.grid.get_mut(x, y).locked {
            return;
        }
        let mut s = HashSet::<u8>::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        for v in RowIter::new(&self.grid, y) {
            s.remove(&v);
        }
        for v in ColumnIter::new(&self.grid, x) {
            s.remove(&v);
        }
        for v in BlockIter::new(&self.grid, x, y) {
            s.remove(&v);
        }
        assert!(s.size() > 0, "expect: a candidate for a square");
        self.grid.get_mut(x, y).candidates =
            Vector::from_iter(s.iter().map(|i| *i));
    }

    fn candidate_cell(&self) -> Option<(usize, usize)> {
        let mut cell: Option<&Cell> = None;
        let mut coordinate = (0, 0);
        for y in 0..self.grid.size_y() {
            for x in 0..self.grid.size_x() {
                let c = self.grid.get(x, y);
                if c.locked == false {
                    if cell.is_none() {
                        cell = Some(c);
                        coordinate = (x, y);
                    } else if c.candidates.size()
                        < cell.unwrap().candidates.size()
                    {
                        cell = Some(c);
                        coordinate = (x, y);
                    }
                }
            }
        }
        match cell {
            None => return None,
            Some(_) => Some(coordinate),
        }
    }
}

impl From<[[u8; 9]; 9]> for Sudoku {
    /// * Create a new instance from 2-dimensional array. Value `0` is
    ///   placeholder for solving.
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    fn from(grid: [[u8; 9]; 9]) -> Self {
        let mut s = Sudoku::new();
        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                s.set(x, y, grid[y][x]);
            }
        }
        return s;
    }
}
