//! Sudoku - a game solver.
//!
//! This document - module level document does not contain other descriptions or
//! examples. To learn more about how to use, refer to specific Entry APIs which
//! is labeled `entry` in the following sections.
//!
//! If there is the first time, [Package Document](crate) is a good way to learn
//! more about how to explore APIs and access to [Entry APIs
//! List](crate#structs).

mod cell;

use crate::grid::Grid;
use crate::Vector;
use cell::Cell;
use std::fmt::Display;
use std::ops::Index;

/// `entry` A container for the game Sudoku and the solver as well.
///
/// Value zero is a placeholder for solving.A Sudoku is solved if and only if
/// all rows, columns and blocks have digits 1-9 and a digit is not duplicated.
///
/// The solver is designed to break a few anti-brute-force techniques. Try it.
///
/// # Model
///
/// ```txt
/// +-------------------------------------- y-axis
/// |      -------------------------------- x-axis
/// |      | | |      | | |      | | |
/// |      0 1 2      3 4 5      6 7 8
/// |   +-------+  +-------+  +-------+
/// |-0 | 0 5 3 |  | 0 7 0 |  | 0 0 0 |
/// |-1 | 6 0 0 |  | 1 9 5 |  | 0 0 0 |
/// |-2 | 0 9 8 |  | 0 0 0 |  | 0 6 0 |
/// |   +-------+  +-------+  +-------+
/// |
/// |   +-------+  +-------+  +-------+
/// |-3 | 8 0 0 |  | 0 6 0 |  | 0 0 3 |
/// |-4 | 4 0 0 |  | 8 0 3 |  | 0 0 1 |
/// |-5 | 7 0 0 |  | 0 2 0 |  | 0 0 6 |
/// |   +-------+  +-------+  +-------+
/// |
/// |   +-------+  +-------+  +-------+
/// |-6 | 0 6 0 |  | 0 0 0 |  | 2 8 0 |
/// |-7 | 0 0 0 |  | 4 1 9 |  | 0 0 5 |
/// |-8 | 0 0 0 |  | 0 8 0 |  | 0 7 9 |
///     +-------+  +-------+  +-------+
/// ```
///
/// # Panic
///
/// * Call [index](Self::index) or [set](Self::set) with `x`, `y` that is
///   greater than or equal [GRID_SIZE](Self::GRID_SIZE).
///
/// # Example
///
/// ```
/// use rust_basic::Sudoku;
///
/// let mut sudoku = Sudoku::from([
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
/// assert_eq!(sudoku.solve(), true);
/// assert_eq!(sudoku, Sudoku::from([
///     [5, 3, 4, 6, 7, 8, 9, 1, 2],
///     [6, 7, 2, 1, 9, 5, 3, 4, 8],
///     [1, 9, 8, 3, 4, 2, 5, 6, 7],
///     [8, 5, 9, 7, 6, 1, 4, 2, 3],
///     [4, 2, 6, 8, 5, 3, 7, 9, 1],
///     [7, 1, 3, 9, 2, 4, 8, 5, 6],
///     [9, 6, 1, 5, 3, 7, 2, 8, 4],
///     [2, 8, 7, 4, 1, 9, 6, 3, 5],
///     [3, 4, 5, 2, 8, 6, 1, 7, 9],
/// ]));
#[derive(Debug)]
pub struct Sudoku {
    grid: Grid<Cell>,
}

impl Sudoku {
    /// Quantity of values in the grid, both x-axis and y-axis.
    pub const GRID_SIZE: usize = 9;

    /// Quantity of values in a block, both x-axis and y-axis.
    const BLOCK_SIZE: usize = 3;

    /// Create a new instance, all cells are set to zero.
    ///
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    pub fn new() -> Self {
        let mut grid = Grid::<Cell>::new();
        grid.resize(Self::GRID_SIZE, Self::GRID_SIZE);
        return Self { grid };
    }

    /// Put a new value at `(x, y)`.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        assert!(x < Self::GRID_SIZE, "expect: valid index x-axis");
        assert!(y < Self::GRID_SIZE, "expect: valid index y-axis");
        assert!(value as usize <= Self::GRID_SIZE, "expect: valid value");
        let cell = &mut self.grid[(x, y)];
        cell.value = value;
        cell.locked = value != 0;
        cell.fixed = value != 0;
    }

    /// Find a solution for the current values.
    ///
    /// Time complexity: O(9^n).
    ///
    /// Space complexity: O(n).
    pub fn solve(&mut self) -> bool {
        self.find_candidates_for_cells();
        self.sort_candidates_for_cells();
        return self.solve_backtrack();
    }

    /// Check the current state is valid or not.
    ///
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    pub fn validate(&self) -> bool {
        return self.validate_numbers()
            && self.validate_rows()
            && self.validate_columns()
            && self.validate_blocks();
    }

    /// Set all placeholder values to zero.
    ///
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    pub fn reset(&mut self) {
        for cell in self.grid.iter_mut() {
            if cell.fixed {
                continue;
            }
            cell.value = 0;
            cell.locked = false;
        }
    }

    fn solve_backtrack(&mut self) -> bool {
        if self.validate() {
            return true;
        }
        let (x, y) = match self.potential_cell() {
            None => return false,
            Some(v) => v,
        };
        let cell = &mut self.grid[(x, y)];
        let candidates = cell.candidates.clone();
        cell.locked = true;
        drop(cell);
        for candiate in candidates.iter() {
            self.grid[(x, y)].value = *candiate;
            if self.validate_cell(x, y) && self.solve_backtrack() {
                return true;
            }
        }
        let cell = &mut self.grid[(x, y)];
        cell.locked = false;
        cell.value = 0;
        return false;
    }

    fn validate_cell(&self, x: usize, y: usize) -> bool {
        return self.validate_cell_rows(y)
            && self.validate_cell_columns(x)
            && self.validate_cell_block(x, y);
    }

    fn validate_numbers(&self) -> bool {
        for y in 0..self.grid.size_y() {
            for x in 0..self.grid.size_x() {
                if self.grid[(x, y)].value == 0 {
                    return false;
                }
            }
        }
        return true;
    }

    fn validate_rows(&self) -> bool {
        for y in 0..Self::GRID_SIZE {
            if !self.valid_row(y) {
                return false;
            }
        }
        return true;
    }

    fn valid_row(&self, y: usize) -> bool {
        let values = self.collect_row(y);
        return Self::duplicate(values, false) == false;
    }

    fn validate_cell_rows(&self, y: usize) -> bool {
        let values = self.collect_row(y);
        return Self::duplicate(values, true) == false;
    }

    fn validate_columns(&self) -> bool {
        for x in 0..Self::GRID_SIZE {
            if !self.valid_column(x) {
                return false;
            }
        }
        return true;
    }

    fn valid_column(&self, x: usize) -> bool {
        let values = self.collect_column(x);
        return Self::duplicate(values, false) == false;
    }

    fn validate_cell_columns(&self, x: usize) -> bool {
        let values = self.collect_column(x);
        return Self::duplicate(values, true) == false;
    }

    fn validate_blocks(&self) -> bool {
        for y_index in 0..Self::BLOCK_SIZE {
            for x_index in 0..Self::BLOCK_SIZE {
                let x_block = Self::BLOCK_SIZE * x_index;
                let y_block = Self::BLOCK_SIZE * y_index;
                if !self.validate_block(x_block, y_block) {
                    return false;
                }
            }
        }
        return true;
    }

    fn validate_block(&self, x_block: usize, y_block: usize) -> bool {
        let values = self.collect_block(x_block, y_block);
        return Self::duplicate(values, false) == false;
    }

    fn validate_cell_block(&self, x: usize, y: usize) -> bool {
        let values = self.collect_block(x, y);
        return Self::duplicate(values, true) == false;
    }

    fn find_candidates_for_cells(&mut self) {
        for y in 0..Self::GRID_SIZE {
            for x in 0..Self::GRID_SIZE {
                self.find_candidates_for_cell(x, y);
            }
        }
    }

    /// For each cell, put the most potential candidates at front. That
    /// increases possibility of correct values from beginning of solving and
    /// reduces tries.
    fn sort_candidates_for_cells(&mut self) {
        let frequencies = self.count_frequency_values();
        for cell in self.grid.iter_mut() {
            Self::sort_candidates_for_cell(&frequencies, &mut cell.candidates);
        }
    }

    fn count_frequency_values(&self) -> [u8; 10] {
        let mut frequencies = [0; 10];
        for cell in self.grid.iter() {
            frequencies[cell.value as usize] += 1;
        }
        return frequencies;
    }

    fn sort_candidates_for_cell(
        frequencies: &[u8; 10],
        candidates: &mut Vector<u8>,
    ) {
        let mut pairs = candidates
            .iter()
            .map(|v| (frequencies[*v as usize], *v))
            .collect::<Vector<_>>();
        pairs.sort();
        for (index, (_, candidate)) in pairs.iter().enumerate() {
            candidates[index] = *candidate;
        }
    }

    fn find_candidates_for_cell(&mut self, x: usize, y: usize) {
        if self.grid[(x, y)].locked {
            return;
        }
        let mut candidates = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        for value in self.collect_row(y) {
            candidates[value as usize] = 0;
        }
        for value in self.collect_column(x) {
            candidates[value as usize] = 0;
        }
        for value in self.collect_block(x, y) {
            candidates[value as usize] = 0;
        }
        self.grid[(x, y)].candidates = candidates
            .iter()
            .filter(|value| **value > 0)
            .map(|value| *value as u8)
            .collect();
        assert!(
            self.grid[(x, y)].candidates.size() > 0,
            "expect: a candidate for a cell"
        );
    }

    /// Find a cell that is not locked and has minimum candidates.
    fn potential_cell(&self) -> Option<(usize, usize)> {
        let mut candidates = None;
        let mut index = (0, 0);
        for y in 0..self.grid.size_y() {
            for x in 0..self.grid.size_x() {
                let cell = &self.grid[(x, y)];
                if cell.locked {
                    continue;
                }
                if candidates.is_none() {
                    candidates = Some(cell.candidates.size());
                    index = (x, y);
                } else if cell.candidates.size() < candidates.unwrap() {
                    candidates = Some(cell.candidates.size());
                    index = (x, y);
                }
            }
        }
        match candidates {
            None => return None,
            Some(_) => Some(index),
        }
    }

    /// The result is correct if and only if values are less than or equal to
    /// [Self::GRID_SIZE].
    fn duplicate(values: [u8; Self::GRID_SIZE], ignore_zero: bool) -> bool {
        let mut map = [0; Self::GRID_SIZE + 1];
        for value in values {
            if ignore_zero && value == 0 {
                continue;
            }
            map[value as usize] += 1;
        }
        for count in map {
            if count > 1 {
                return true;
            }
        }
        return false;
    }

    fn collect_row(&self, y: usize) -> [u8; Self::GRID_SIZE] {
        let mut values = [0; Self::GRID_SIZE];
        for x in 0..Self::GRID_SIZE {
            values[x] = self.grid[(x, y)].value;
        }
        return values;
    }

    fn collect_column(&self, x: usize) -> [u8; Self::GRID_SIZE] {
        let mut values = [0; Self::GRID_SIZE];
        for y in 0..Self::GRID_SIZE {
            values[y] = self.grid[(x, y)].value;
        }
        return values;
    }

    fn collect_block(&self, x: usize, y: usize) -> [u8; Self::GRID_SIZE] {
        let x_block = x - x % Self::BLOCK_SIZE;
        let y_block = y - y % Self::BLOCK_SIZE;
        let mut values = [0; Self::GRID_SIZE];
        let mut index = 0;
        for x in x_block..(x_block + Self::BLOCK_SIZE) {
            for y in y_block..(y_block + Self::BLOCK_SIZE) {
                values[index] = self.grid[(x, y)].value;
                index += 1;
            }
        }
        return values;
    }
}

impl From<[[u8; Self::GRID_SIZE]; Self::GRID_SIZE]> for Sudoku {
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn from(grid: [[u8; Self::GRID_SIZE]; Self::GRID_SIZE]) -> Self {
        let mut s = Sudoku::new();
        for x in 0..Self::GRID_SIZE {
            for y in 0..Self::GRID_SIZE {
                s.set(x, y, grid[y][x]);
            }
        }
        return s;
    }
}

impl Index<(usize, usize)> for Sudoku {
    type Output = u8;

    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        assert!(x < Self::GRID_SIZE, "expect: valid index x-axis");
        assert!(y < Self::GRID_SIZE, "expect: valid index y-axis");
        return &self.grid[(x, y)].value;
    }
}

impl Eq for Sudoku {}

impl PartialEq for Sudoku {
    fn eq(&self, other: &Self) -> bool {
        return self.grid == other.grid;
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..Self::GRID_SIZE {
            for x in 0..Self::GRID_SIZE {
                write!(f, "{} ", self[(x, y)])?;
                if (x + 1) % 3 == 0 {
                    write!(f, "  ")?;
                }
            }
            writeln!(f)?;
            if (y + 1) % 3 == 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
