use rust_basic::Grid;
use testkit::{NonZeroSize, ZeroSize};

pub(super) fn assert_resize_non_zero_size(
    grid: &mut Grid<NonZeroSize<(usize, usize)>>,
    size_x: usize,
    size_y: usize,
) {
    grid.resize(size_x, size_y);
    assert_eq!(grid.size_x(), size_x);
    assert_eq!(grid.size_y(), size_y);
    for x in 0..size_x {
        for y in 0..size_y {
            let item = NonZeroSize::new((x, y));
            grid.set(x, y, item.clone());
            assert_eq!(grid[(x, y)], item);
        }
    }
}

pub(super) fn assert_resize_zero_size(
    grid: &mut Grid<ZeroSize>,
    size_x: usize,
    size_y: usize,
) {
    grid.resize(size_x, size_y);
    assert_eq!(grid.size_x(), size_x);
    assert_eq!(grid.size_y(), size_y);
    for x in 0..size_x {
        for y in 0..size_y {
            assert_eq!(grid[(x, y)], ZeroSize::new());
            grid.set(x, y, ZeroSize::new());
            assert_eq!(grid[(x, y)], ZeroSize::new());
        }
    }
}

pub(super) fn sample_non_zero_size_type() -> Grid<NonZeroSize<(usize, usize)>> {
    let mut grid = Grid::new();
    grid.resize(101, 107);
    for x in 0..grid.size_x() {
        for y in 0..grid.size_y() {
            grid[(x, y)] = NonZeroSize::new((x, y));
        }
    }
    return grid;
}

pub(super) fn sample_zero_size_type() -> Grid<ZeroSize> {
    let mut grid = Grid::<ZeroSize>::new();
    grid.resize(101, 107);
    for x in 0..grid.size_x() {
        for y in 0..grid.size_y() {
            grid.set(x, y, ZeroSize::new());
        }
    }
    return grid;
}
