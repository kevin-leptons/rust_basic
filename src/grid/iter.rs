use super::Grid;

/// For iteration over immutable items in a grid. See [super::Grid::iter].
pub struct Iter<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
}

impl<'a, T> Iter<'a, T> {
    pub(super) fn new(grid: &'a Grid<T>) -> Self {
        return Self {
            grid: grid,
            x: 0,
            y: 0,
        };
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x == self.grid.size_x() {
            self.x = 0;
            self.y += 1;
        }
        if self.y == self.grid.size_y() {
            return None;
        }
        let i = self.grid.get_index(self.x, self.y);
        let v = unsafe { &*self.grid.cells.as_ptr().add(i) };
        self.x += 1;
        return Some(v);
    }
}

/// For iteration over mutable itesm in a grid. See [super::Grid::iter_mut].
pub struct IterMut<'a, T> {
    grid: &'a mut Grid<T>,
    x: usize,
    y: usize,
}

impl<'a, T> IterMut<'a, T> {
    pub(super) fn new(grid: &'a mut Grid<T>) -> Self {
        return Self {
            grid: grid,
            x: 0,
            y: 0,
        };
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x == self.grid.size_x() {
            self.x = 0;
            self.y += 1;
        }
        if self.y == self.grid.size_y() {
            return None;
        }
        let i = self.grid.get_index(self.x, self.y);
        let v = unsafe { &mut *self.grid.cells.as_ptr().add(i) };
        self.x += 1;
        return Some(v);
    }
}
