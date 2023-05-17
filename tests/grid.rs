use rust_basic::grid::Grid;

#[test]
fn resize_expand_expand_narrow() {
    let mut g = Grid::<String>::new();
    assert_resize(&mut g, 9, 7);
    assert_resize(&mut g, 88, 73);
    assert_resize(&mut g, 45, 37);
}

#[test]
fn resize_expand_narrow_expand() {
    let mut g = Grid::<String>::new();
    assert_resize(&mut g, 75, 91);
    assert_resize(&mut g, 13, 41);
    assert_resize(&mut g, 115, 121);
}

#[test]
fn resize_expand_x_narrow_y() {
    let mut g = Grid::<String>::new();
    assert_resize(&mut g, 75, 91);
    assert_resize(&mut g, 99, 19);
}

#[test]
fn resize_expand_y_narrow_x() {
    let mut g = Grid::<String>::new();
    assert_resize(&mut g, 75, 91);
    assert_resize(&mut g, 57, 131);
}

#[test]
fn resize_zero() {
    let mut g = Grid::from([
        ["0:0".to_string(), "0:1".to_string(), "0:2".to_string()],
        ["1:0".to_string(), "1:1".to_string(), "1:2".to_string()],
        ["2:0".to_string(), "2:1".to_string(), "2:2".to_string()],
        ["3:0".to_string(), "3:1".to_string(), "3:2".to_string()],
    ]);
    g.resize(0, 0);
    assert_eq!(g.size_x(), 0);
    assert_eq!(g.size_y(), 0);
}

#[test]
fn resize_zero_x() {
    let mut g = Grid::from([
        ["0:0".to_string(), "0:1".to_string(), "0:2".to_string()],
        ["1:0".to_string(), "1:1".to_string(), "1:2".to_string()],
        ["2:0".to_string(), "2:1".to_string(), "2:2".to_string()],
        ["3:0".to_string(), "3:1".to_string(), "3:2".to_string()],
    ]);
    g.resize(0, 8);
    assert_eq!(g.size_x(), 0);
    assert_eq!(g.size_y(), 8);
}

#[test]
fn resize_zero_y() {
    let mut g = Grid::from([
        ["0:0".to_string(), "0:1".to_string(), "0:2".to_string()],
        ["1:0".to_string(), "1:1".to_string(), "1:2".to_string()],
        ["2:0".to_string(), "2:1".to_string(), "2:2".to_string()],
        ["3:0".to_string(), "3:1".to_string(), "3:2".to_string()],
    ]);
    g.resize(8, 0);
    assert_eq!(g.size_x(), 8);
    assert_eq!(g.size_y(), 0);
}

#[test]
fn from_array() {
    let a = [
        ["0:0".to_string(), "0:1".to_string(), "0:2".to_string()],
        ["1:0".to_string(), "1:1".to_string(), "1:2".to_string()],
        ["2:0".to_string(), "2:1".to_string(), "2:2".to_string()],
        ["3:0".to_string(), "3:1".to_string(), "3:2".to_string()],
    ];
    let g = Grid::from(a.clone());
    assert_eq!(g.size_x(), 3);
    assert_eq!(g.size_y(), 4);
    for y in 0..4 {
        for x in 0..3 {
            assert_eq!(g.get(x, y), &format!("{}:{}", y, x));
        }
    }
}

#[test]
fn get() {
    let g = build_sample();
    for x in 0..g.size_x() {
        for y in 0..g.size_y() {
            let expected = &format!("value: {}:{}", x, y);
            assert_eq!(g.get(x, y), expected);
        }
    }
}

#[test]
fn get_mut() {
    let mut g = Grid::from([
        ["item: 0:0".to_string(), "item: 0:1".to_string()],
        ["item: 1:0".to_string(), "item: 1:1".to_string()],
        ["item: 2:0".to_string(), "item: 2:1".to_string()],
    ]);
    let v = "new item: 1:0";
    g.get_mut(0, 1).replace_range(.., v);
    assert_eq!(g.get(0, 1), v);
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn set_panic_x() {
    let mut g = build_sample();
    g.set(g.size_x(), g.size_y() - 1, "value".to_string());
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn set_panic_y() {
    let mut g = build_sample();
    g.set(g.size_x() - 1, g.size_y(), "value".to_string());
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn get_panic_x() {
    let g = build_sample();
    g.get(g.size_x(), g.size_y() - 1);
}

#[test]
#[should_panic(expected = "expect: valid index")]
fn get_panic_y() {
    let g = build_sample();
    g.get(g.size_x() - 1, g.size_y());
}

#[test]
fn cells_mut() {
    let mut g = Grid::from([
        ["0:0".to_string(), "0:1".to_string(), "0:2".to_string()],
        ["1:0".to_string(), "1:1".to_string(), "1:2".to_string()],
        ["2:0".to_string(), "2:1".to_string(), "2:2".to_string()],
        ["3:0".to_string(), "3:1".to_string(), "3:2".to_string()],
    ]);
    for c in g.cells_mut() {
        c.replace_range(1..2, "-");
    }
    for x in 0..g.size_x() {
        for y in 0..g.size_y() {
            assert_eq!(g.get(x, y), &format!("{}-{}", y, x));
        }
    }
}

#[test]
fn clone() {
    let g0 = Grid::from([
        ["0:0".to_string(), "0:1".to_string(), "0:2".to_string()],
        ["1:0".to_string(), "1:1".to_string(), "1:2".to_string()],
        ["2:0".to_string(), "2:1".to_string(), "2:2".to_string()],
        ["3:0".to_string(), "3:1".to_string(), "3:2".to_string()],
    ]);
    let g1 = g0.clone();
    assert_eq!(g0.size_x(), g1.size_x());
    assert_eq!(g0.size_y(), g1.size_y());
    for x in 0..g0.size_x() {
        for y in 0..g0.size_y() {
            assert_eq!(g0.get(x, y), g1.get(x, y));
        }
    }
}

#[test]
fn equal() {
    let a = [
        ["0:0".to_string(), "0:1".to_string(), "0:2".to_string()],
        ["1:0".to_string(), "1:1".to_string(), "1:2".to_string()],
        ["2:0".to_string(), "2:1".to_string(), "2:2".to_string()],
        ["3:0".to_string(), "3:1".to_string(), "3:2".to_string()],
    ];
    let g0 = Grid::from(a.clone());
    let g1 = Grid::from(a.clone());
    assert_eq!(g0, g1);
}

#[test]
fn equal_not_size_x() {
    let a = [
        ["0:0".to_string(), "0:1".to_string(), "0:2".to_string()],
        ["1:0".to_string(), "1:1".to_string(), "1:2".to_string()],
        ["2:0".to_string(), "2:1".to_string(), "2:2".to_string()],
        ["3:0".to_string(), "3:1".to_string(), "3:2".to_string()],
    ];
    let g0 = Grid::from([
        ["0:0".to_string(), "0:1".to_string()],
        ["1:0".to_string(), "1:1".to_string()],
        ["2:0".to_string(), "2:1".to_string()],
        ["3:0".to_string(), "3:1".to_string()],
    ]);
    let g1 = Grid::from(a.clone());
    assert_ne!(g0, g1);
}

#[test]
fn equal_not_size_y() {
    let a = [
        ["0:0".to_string(), "0:1".to_string(), "0:2".to_string()],
        ["1:0".to_string(), "1:1".to_string(), "1:2".to_string()],
        ["2:0".to_string(), "2:1".to_string(), "2:2".to_string()],
        ["3:0".to_string(), "3:1".to_string(), "3:2".to_string()],
    ];
    let g0 = Grid::from([
        ["0:0".to_string(), "0:1".to_string(), "0:2".to_string()],
        ["1:0".to_string(), "1:1".to_string(), "1:2".to_string()],
        ["2:0".to_string(), "2:1".to_string(), "2:2".to_string()],
    ]);
    let g1 = Grid::from(a.clone());
    assert_ne!(g0, g1);
}

#[test]
fn equal_not_size_x_y() {
    let a = [
        ["0:0".to_string(), "0:1".to_string(), "0:2".to_string()],
        ["1:0".to_string(), "1:1".to_string(), "1:2".to_string()],
        ["2:0".to_string(), "2:1".to_string(), "2:2".to_string()],
        ["3:0".to_string(), "3:1".to_string(), "3:2".to_string()],
    ];
    let g0 = Grid::from([
        ["0:0".to_string(), "0:1".to_string()],
        ["1:0".to_string(), "1:1".to_string()],
        ["2:0".to_string(), "2:1".to_string()],
    ]);
    let g1 = Grid::from(a.clone());
    assert_ne!(g0, g1);
}

#[test]
fn equal_not_size_value() {
    let a = [
        ["0:0".to_string(), "0:1".to_string(), "0:2".to_string()],
        ["1:0".to_string(), "1:1".to_string(), "1:2".to_string()],
        ["2:0".to_string(), "2:1".to_string(), "2:2".to_string()],
        ["3:0".to_string(), "3:1".to_string(), "3:2".to_string()],
    ];
    let g0 = Grid::from([
        ["0:0".to_string(), "0:1".to_string(), "0:2".to_string()],
        ["1:0".to_string(), "1:1".to_string(), "1:2".to_string()],
        ["2:0".to_string(), "2:1".to_string(), "2:2".to_string()],
        ["3:0".to_string(), "3:1".to_string(), "9:9".to_string()],
    ]);
    let g1 = Grid::from(a.clone());
    assert_ne!(g0, g1);
}

fn assert_resize(g: &mut Grid<String>, size_x: usize, size_y: usize) {
    g.resize(size_x, size_y);
    assert_eq!(g.size_x(), size_x);
    assert_eq!(g.size_y(), size_y);
    for x in 0..size_x {
        for y in 0..size_y {
            let v = format!("value: {}:{}", x, y);
            g.set(x, y, v.clone());
            assert_eq!(g.get(x, y), &v);
        }
    }
}

fn build_sample() -> Grid<String> {
    let mut g = Grid::<String>::new();
    g.resize(100, 1000);
    for x in 0..g.size_x() {
        for y in 0..g.size_y() {
            g.set(x, y, format!("value: {}:{}", x, y));
        }
    }
    return g;
}
