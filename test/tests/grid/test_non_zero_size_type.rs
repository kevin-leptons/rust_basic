use crate::helper;
use rust_basic::Grid;
use testkit::NonZeroSize;

#[test]
fn new() {
    let grid = Grid::<NonZeroSize>::new();
    assert_eq!(grid.size_x(), 0);
    assert_eq!(grid.size_y(), 0);
}

#[test]
fn resize_expand_expand_narrow() {
    let mut grid = Grid::new();
    helper::assert_resize_non_zero_size(&mut grid, 9, 7);
    helper::assert_resize_non_zero_size(&mut grid, 88, 73);
    helper::assert_resize_non_zero_size(&mut grid, 45, 37);
}

#[test]
fn resize_expand_narrow_expand() {
    let mut grid = Grid::new();
    helper::assert_resize_non_zero_size(&mut grid, 75, 91);
    helper::assert_resize_non_zero_size(&mut grid, 13, 41);
    helper::assert_resize_non_zero_size(&mut grid, 115, 121);
}

#[test]
fn resize_expand_x_narrow_y() {
    let mut grid = Grid::new();
    helper::assert_resize_non_zero_size(&mut grid, 75, 91);
    helper::assert_resize_non_zero_size(&mut grid, 99, 19);
}

#[test]
fn resize_expand_y_narrow_x() {
    let mut grid = Grid::new();
    helper::assert_resize_non_zero_size(&mut grid, 75, 91);
    helper::assert_resize_non_zero_size(&mut grid, 57, 131);
}

#[test]
fn resize_expand_narrow_zero() {
    let mut grid = Grid::new();
    helper::assert_resize_non_zero_size(&mut grid, 55, 61);
    helper::assert_resize_non_zero_size(&mut grid, 0, 0);
}

#[test]
fn resize_expand_narrow_x_zero() {
    let mut grid = Grid::new();
    helper::assert_resize_non_zero_size(&mut grid, 55, 61);
    helper::assert_resize_non_zero_size(&mut grid, 0, 16);
}

#[test]
fn resize_expand_narrow_y_zero() {
    let mut grid = Grid::new();
    helper::assert_resize_non_zero_size(&mut grid, 55, 61);
    helper::assert_resize_non_zero_size(&mut grid, 99, 0);
}

#[test]
fn from_array() {
    let array = [
        [
            NonZeroSize::new((0, 0)),
            NonZeroSize::new((1, 0)),
            NonZeroSize::new((2, 0)),
        ],
        [
            NonZeroSize::new((0, 1)),
            NonZeroSize::new((1, 1)),
            NonZeroSize::new((2, 1)),
        ],
        [
            NonZeroSize::new((0, 2)),
            NonZeroSize::new((1, 2)),
            NonZeroSize::new((2, 2)),
        ],
        [
            NonZeroSize::new((0, 3)),
            NonZeroSize::new((1, 3)),
            NonZeroSize::new((2, 3)),
        ],
    ];
    let grid = Grid::from(array.clone());
    assert_eq!(grid.size_x(), array[0].len());
    assert_eq!(grid.size_y(), array.len());
    for x in 0..array[0].len() {
        for y in 0..array.len() {
            assert_eq!(grid[(x, y)], NonZeroSize::new((x, y)));
        }
    }
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn index_panic_x() {
    let grid = helper::sample_non_zero_size_type();
    let _ = &grid[(grid.size_x(), grid.size_y() - 1)];
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn index_panic_y() {
    let grid = helper::sample_non_zero_size_type();
    let _ = &grid[(grid.size_x() - 1, grid.size_y())];
}

#[test]
fn index_mut() {
    let mut grid = helper::sample_non_zero_size_type();
    for x in 0..grid.size_x() {
        for y in 0..grid.size_y() {
            let new_item = NonZeroSize::new((x, y));
            grid[(x, y)] = new_item.clone();
            assert_eq!(grid[(x, y)], new_item);
        }
    }
    for x in 0..grid.size_x() {
        for y in 0..grid.size_y() {
            assert_eq!(grid[(x, y)], NonZeroSize::new((x, y)));
        }
    }
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn index_mut_panic_x() {
    let mut grid = helper::sample_non_zero_size_type();
    let size_x = grid.size_x();
    let size_y = grid.size_y();
    grid[(size_x, size_y - 1)] = NonZeroSize::new((0, 0));
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn index_mut_panic_y() {
    let mut grid = helper::sample_non_zero_size_type();
    let size_x = grid.size_x();
    let size_y = grid.size_y();
    grid[(size_x - 1, size_y)] = NonZeroSize::new((0, 0));
}

#[test]
fn set_return_old_value() {
    let mut grid = helper::sample_non_zero_size_type();
    for x in 0..grid.size_x() {
        for y in 0..grid.size_y() {
            assert_eq!(
                grid.set(x, y, NonZeroSize::new((usize::MAX, usize::MAX))),
                NonZeroSize::new((x, y))
            );
        }
    }
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn set_panic_x() {
    let mut grid = helper::sample_non_zero_size_type();
    grid.set(grid.size_x(), grid.size_y() - 1, NonZeroSize::new((0, 0)));
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn set_panic_y() {
    let mut grid = helper::sample_non_zero_size_type();
    grid.set(grid.size_x() - 1, grid.size_y(), NonZeroSize::new((0, 0)));
}

#[test]
fn iter() {
    let grid = helper::sample_non_zero_size_type();
    let size_x = grid.size_x();
    let size = grid.size_x() * grid.size_y();
    let mut i = 0;
    for item in grid.iter() {
        let x = i % size_x;
        let y = i / size_x;
        assert_eq!(item, &NonZeroSize::new((x, y)));
        i += 1;
    }
    assert_eq!(i, size);
}

#[test]
fn iter_mut() {
    let mut grid = helper::sample_non_zero_size_type();
    let size = grid.size_x() * grid.size_y();
    let size_x = grid.size_x();
    let size_y = grid.size_y();
    let mut i = 0;
    for item in grid.iter_mut() {
        let x = i % size_x;
        let y = i / size_x;
        assert_eq!(item, &NonZeroSize::new((x, y)));
        i += 1;
        item.value = (size_x + x + 100, size_y + y + 100);
    }
    assert_eq!(i, size);
    for x in 0..size_x {
        for y in 0..size_y {
            assert_eq!(
                grid[(x, y)],
                NonZeroSize::new((size_x + x + 100, size_y + y + 100))
            );
        }
    }
}

#[test]
fn equal_true() {
    let array = [
        [NonZeroSize::new((0, 0)), NonZeroSize::new((1, 0))],
        [NonZeroSize::new((0, 1)), NonZeroSize::new((1, 1))],
        [NonZeroSize::new((0, 2)), NonZeroSize::new((1, 2))],
    ];
    let grid0 = Grid::from(array.clone());
    let grid1 = Grid::from(array.clone());
    assert_eq!(grid0, grid1);
}

#[test]
fn equal_false_size_x() {
    let grid0 = Grid::from([
        [NonZeroSize::new((0, 0)), NonZeroSize::new((1, 0))],
        [NonZeroSize::new((0, 1)), NonZeroSize::new((1, 1))],
        [NonZeroSize::new((0, 2)), NonZeroSize::new((1, 2))],
    ]);
    let grid1 = Grid::from([
        [NonZeroSize::new((0, 0))],
        [NonZeroSize::new((0, 1))],
        [NonZeroSize::new((0, 2))],
    ]);
    assert_ne!(grid0, grid1);
}

#[test]
fn equal_false_size_y() {
    let grid0 = Grid::from([
        [NonZeroSize::new((0, 0)), NonZeroSize::new((1, 0))],
        [NonZeroSize::new((0, 1)), NonZeroSize::new((1, 1))],
        [NonZeroSize::new((0, 2)), NonZeroSize::new((1, 2))],
        [NonZeroSize::new((0, 3)), NonZeroSize::new((1, 3))],
    ]);
    let grid1 = Grid::from([
        [NonZeroSize::new((0, 0)), NonZeroSize::new((1, 0))],
        [NonZeroSize::new((0, 1)), NonZeroSize::new((1, 1))],
        [NonZeroSize::new((0, 2)), NonZeroSize::new((1, 2))],
    ]);
    assert_ne!(grid0, grid1);
}

#[test]
fn equal_false_value() {
    let grid0 = Grid::from([
        [NonZeroSize::new((0, 0)), NonZeroSize::new((1, 0))],
        [NonZeroSize::new((0, 1)), NonZeroSize::new((1, 1))],
        [NonZeroSize::new((0, 2)), NonZeroSize::new((1, 2))],
        [NonZeroSize::new((0, 3)), NonZeroSize::new((1, 3))],
    ]);
    let grid1 = Grid::from([
        [NonZeroSize::new((0, 0)), NonZeroSize::new((1, 0))],
        [NonZeroSize::new((0, 1)), NonZeroSize::new((1, 1))],
        [NonZeroSize::new((0, 2)), NonZeroSize::new((1, 2))],
        [NonZeroSize::new((0, 3)), NonZeroSize::new((9, 9))],
    ]);
    assert_ne!(grid0, grid1);
}

#[test]
fn clone() {
    let grid0 = helper::sample_non_zero_size_type();
    let grid1 = grid0.clone();
    assert_eq!(grid0, grid1);
}

// This test does nothing but creating a non empty container to trigger memory
// release process. The test can not work alone, it requries an external tool
// such as Valgrind to diagnose memory issues.
//
// Warn: The test maybe still passed even memory release process has issues.
#[test]
fn drop() {
    let _ = helper::sample_non_zero_size_type();
}

#[test]
fn sample_must_not_empty() {
    let grid = helper::sample_non_zero_size_type();
    assert!(grid.size_x() > 0);
    assert!(grid.size_y() > 0);
}
