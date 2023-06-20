use crate::helper;
use rust_basic::Grid;
use testkit::ZeroSize;

#[test]
fn new() {
    let grid = Grid::<ZeroSize>::new();
    assert_eq!(grid.size_x(), 0);
    assert_eq!(grid.size_y(), 0);
}

#[test]
fn resize_expand_expand_narrow() {
    let mut grid = Grid::new();
    helper::assert_resize_zero_size(&mut grid, 9, 7);
    helper::assert_resize_zero_size(&mut grid, 88, 73);
    helper::assert_resize_zero_size(&mut grid, 45, 37);
}

#[test]
fn resize_expand_narrow_expand() {
    let mut grid = Grid::new();
    helper::assert_resize_zero_size(&mut grid, 75, 91);
    helper::assert_resize_zero_size(&mut grid, 13, 41);
    helper::assert_resize_zero_size(&mut grid, 115, 121);
}

#[test]
fn resize_expand_x_narrow_y() {
    let mut grid = Grid::new();
    helper::assert_resize_zero_size(&mut grid, 75, 91);
    helper::assert_resize_zero_size(&mut grid, 99, 19);
}

#[test]
fn resize_expand_y_narrow_x() {
    let mut grid = Grid::new();
    helper::assert_resize_zero_size(&mut grid, 75, 91);
    helper::assert_resize_zero_size(&mut grid, 57, 131);
}

#[test]
fn resize_expand_narrow_zero() {
    let mut grid = Grid::new();
    helper::assert_resize_zero_size(&mut grid, 55, 61);
    helper::assert_resize_zero_size(&mut grid, 0, 0);
}

#[test]
fn resize_expand_narrow_x_zero() {
    let mut grid = Grid::new();
    helper::assert_resize_zero_size(&mut grid, 55, 61);
    helper::assert_resize_zero_size(&mut grid, 0, 16);
}

#[test]
fn resize_expand_narrow_y_zero() {
    let mut grid = Grid::new();
    helper::assert_resize_zero_size(&mut grid, 55, 61);
    helper::assert_resize_zero_size(&mut grid, 99, 0);
}

#[test]
fn from_array() {
    let array = [
        [ZeroSize::new(), ZeroSize::new(), ZeroSize::new()],
        [ZeroSize::new(), ZeroSize::new(), ZeroSize::new()],
        [ZeroSize::new(), ZeroSize::new(), ZeroSize::new()],
        [ZeroSize::new(), ZeroSize::new(), ZeroSize::new()],
    ];
    let grid = Grid::from(array.clone());
    assert_eq!(grid.size_x(), array[0].len());
    assert_eq!(grid.size_y(), array.len());
    for x in 0..array[0].len() {
        for y in 0..array.len() {
            assert_eq!(grid[(x, y)], ZeroSize::new());
        }
    }
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn index_panic_x() {
    let grid = helper::sample_zero_size_type();
    let _ = &grid[(grid.size_x(), grid.size_y() - 1)];
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn index_panic_y() {
    let grid = helper::sample_zero_size_type();
    let _ = &grid[(grid.size_x() - 1, grid.size_y())];
}

#[test]
fn index_mut() {
    let mut grid = helper::sample_zero_size_type();
    for x in 0..grid.size_x() {
        for y in 0..grid.size_y() {
            let new_item = ZeroSize::new();
            grid[(x, y)] = new_item.clone();
            assert_eq!(grid[(x, y)], new_item);
        }
    }
    for x in 0..grid.size_x() {
        for y in 0..grid.size_y() {
            assert_eq!(grid[(x, y)], ZeroSize::new());
        }
    }
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn index_mut_panic_x() {
    let mut grid = helper::sample_zero_size_type();
    let size_x = grid.size_x();
    let size_y = grid.size_y();
    grid[(size_x, size_y - 1)] = ZeroSize::new();
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn index_mut_panic_y() {
    let mut grid = helper::sample_zero_size_type();
    let size_x = grid.size_x();
    let size_y = grid.size_y();
    grid[(size_x - 1, size_y)] = ZeroSize::new();
}

#[test]
fn set_return_old_value() {
    let mut grid = helper::sample_zero_size_type();
    for x in 0..grid.size_x() {
        for y in 0..grid.size_y() {
            assert_eq!(grid.set(x, y, ZeroSize::new()), ZeroSize::new());
        }
    }
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn set_panic_x() {
    let mut grid = helper::sample_zero_size_type();
    grid.set(grid.size_x(), grid.size_y() - 1, ZeroSize::new());
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn set_panic_y() {
    let mut grid = helper::sample_zero_size_type();
    grid.set(grid.size_x() - 1, grid.size_y(), ZeroSize::new());
}

#[test]
fn iter() {
    let grid = helper::sample_zero_size_type();
    let size = grid.size_x() * grid.size_y();
    let mut i = 0;
    for item in grid.iter() {
        assert_eq!(item, &ZeroSize::new());
        i += 1;
    }
    assert_eq!(i, size);
}

#[test]
fn iter_mut() {
    let mut grid = helper::sample_zero_size_type();
    let size = grid.size_x() * grid.size_y();
    let mut i = 0;
    for item in grid.iter_mut() {
        assert_eq!(item, &ZeroSize::new());
        i += 1;
    }
    assert_eq!(i, size);
}

#[test]
fn equal_true() {
    let array = [
        [ZeroSize::new(), ZeroSize::new()],
        [ZeroSize::new(), ZeroSize::new()],
        [ZeroSize::new(), ZeroSize::new()],
    ];
    let grid0 = Grid::from(array.clone());
    let grid1 = Grid::from(array.clone());
    assert_eq!(grid0, grid1);
}

#[test]
fn equal_false_size_x() {
    let grid0 = Grid::from([
        [ZeroSize::new(), ZeroSize::new()],
        [ZeroSize::new(), ZeroSize::new()],
        [ZeroSize::new(), ZeroSize::new()],
        [ZeroSize::new(), ZeroSize::new()],
    ]);
    let grid1 = Grid::from([
        [ZeroSize::new()],
        [ZeroSize::new()],
        [ZeroSize::new()],
        [ZeroSize::new()],
    ]);
    assert_ne!(grid0, grid1);
}

#[test]
fn equal_false_size_y() {
    let grid0 = Grid::from([
        [ZeroSize::new(), ZeroSize::new()],
        [ZeroSize::new(), ZeroSize::new()],
        [ZeroSize::new(), ZeroSize::new()],
    ]);
    let grid1 = Grid::from([
        [ZeroSize::new(), ZeroSize::new()],
        [ZeroSize::new(), ZeroSize::new()],
    ]);
    assert_ne!(grid0, grid1);
}

#[test]
fn clone() {
    let grid0 = helper::sample_zero_size_type();
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
    let _ = helper::sample_zero_size_type();
}

#[test]
fn sample_must_not_empty() {
    let grid = helper::sample_zero_size_type();
    assert!(grid.size_x() > 0);
    assert!(grid.size_y() > 0);
}
