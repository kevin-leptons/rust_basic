use crate::Grid;

use super::Cell;

pub(super) struct RowIter<'a> {
    grid: &'a Grid<Cell>,
    x: usize,
    y: usize,
}

impl<'a> RowIter<'a> {
    pub fn new(g: &'a Grid<Cell>, y: usize) -> Self {
        return Self {
            grid: g,
            x: 0,
            y: y,
        };
    }
}

impl Iterator for RowIter<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x == self.grid.size_x() {
            return None;
        }
        let v = self.grid.get(self.x, self.y).value;
        self.x += 1;
        return Some(v);
    }
}

pub(super) struct ColumnIter<'a> {
    grid: &'a Grid<Cell>,
    x: usize,
    y: usize,
}

impl<'a> ColumnIter<'a> {
    pub fn new(g: &'a Grid<Cell>, x: usize) -> Self {
        return Self {
            grid: g,
            x: x,
            y: 0,
        };
    }
}

impl Iterator for ColumnIter<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y == self.grid.size_y() {
            return None;
        }
        let v = self.grid.get(self.x, self.y).value;
        self.y += 1;
        return Some(v);
    }
}

pub(super) struct BlockIter<'a> {
    grid: &'a Grid<Cell>,
    xb: usize,
    yb: usize,
    x: usize,
    y: usize,
}

impl<'a> BlockIter<'a> {
    pub fn new(g: &'a Grid<Cell>, x: usize, y: usize) -> Self {
        let xb = x - x % 3;
        let yb = y - y % 3;
        return Self {
            grid: g,
            xb: xb,
            yb: yb,
            x: xb,
            y: yb,
        };
    }
}

impl Iterator for BlockIter<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y == (self.yb + 3) {
            return None;
        }
        let v = self.grid.get(self.x, self.y).value;
        self.x += 1;
        if self.x == (self.xb + 3) {
            self.y += 1;
            self.x = self.xb;
        }
        return Some(v);
    }
}

// pub(super) struct CellIter<'a> {
//     grid: &'a mut Grid<Cell>,
//     x: usize,
//     y: usize,
// }

// impl<'a> CellIter<'a> {
//     pub(super) fn new(grid: &'a mut Grid<Cell>) -> Self {
//         return Self {
//             grid: grid,
//             x: 0,
//             y: 0,
//         };
//     }
// }

// impl<'a> Iterator for CellIter<'a> {
//     type Item = &'a mut Cell;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.x == self.grid.size_x() {
//             self.x = 0;
//             self.y += 1;
//         }
//         if self.y == self.grid.size_y() {
//             return None;
//         }
//         let x = self.x;
//         let y = self.y;
//         self.x += 1;
//         return Some(self.grid.get_mut(x, y));
//     }
// }
