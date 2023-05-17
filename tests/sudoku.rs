use rust_basic::sudoku::Sudoku;

#[test]
fn from() {
    let g = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];
    let s = Sudoku::from(g);
    for x in 0..9 {
        for y in 0..9 {
            let value = s.get(x, y);
            assert_eq!(value, g[y][x]);
        }
    }
}

#[test]
#[should_panic(expected = "expect: value is in [0, 9]")]
fn from_panic() {
    let g = [
        [10, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];
    let _ = Sudoku::from(g);
}

#[test]
#[should_panic(expected = "expect: value is in [0, 9]")]
fn set_panic_value_out_range() {
    let mut s = Sudoku::new();
    s.set(0, 0, 10);
}

#[test]
fn valid_true() {
    let s = Sudoku::from([
        [5, 3, 4, 6, 7, 8, 9, 1, 2],
        [6, 7, 2, 1, 9, 5, 3, 4, 8],
        [1, 9, 8, 3, 4, 2, 5, 6, 7],
        [8, 5, 9, 7, 6, 1, 4, 2, 3],
        [4, 2, 6, 8, 5, 3, 7, 9, 1],
        [7, 1, 3, 9, 2, 4, 8, 5, 6],
        [9, 6, 1, 5, 3, 7, 2, 8, 4],
        [2, 8, 7, 4, 1, 9, 6, 3, 5],
        [3, 4, 5, 2, 8, 6, 1, 7, 9],
    ]);
    assert_eq!(s.validate(), true);
}

#[test]
fn valid_false_row() {
    let s = Sudoku::from([
        [5, 3, 4, 6, 7, 8, 9, 1, 2],
        [6, 7, 7, 1, 9, 5, 3, 4, 8],
        [1, 9, 8, 3, 4, 2, 5, 6, 7],
        [8, 5, 9, 7, 6, 1, 4, 2, 3],
        [4, 2, 6, 8, 5, 3, 7, 9, 1],
        [7, 1, 3, 9, 2, 4, 8, 5, 6],
        [9, 6, 1, 5, 3, 7, 2, 8, 4],
        [2, 8, 7, 4, 1, 9, 6, 3, 5],
        [3, 4, 5, 2, 8, 6, 1, 7, 9],
    ]);
    assert_eq!(s.validate(), false);
}

#[test]
fn valid_false_column() {
    let s = Sudoku::from([
        [5, 3, 4, 6, 7, 8, 9, 1, 2],
        [5, 7, 2, 1, 9, 5, 3, 4, 8],
        [1, 7, 8, 3, 4, 2, 5, 6, 7],
        [8, 5, 9, 7, 6, 1, 4, 2, 3],
        [4, 2, 6, 8, 5, 3, 7, 9, 1],
        [7, 1, 3, 9, 2, 4, 8, 5, 6],
        [9, 6, 1, 5, 3, 7, 2, 8, 4],
        [2, 8, 7, 4, 1, 9, 6, 3, 5],
        [3, 4, 5, 2, 8, 6, 1, 7, 9],
    ]);
    assert_eq!(s.validate(), false);
}

#[test]
fn valid_false_block() {
    let s = Sudoku::from([
        [5, 3, 4, 6, 7, 8, 9, 1, 2],
        [5, 7, 2, 1, 9, 5, 3, 4, 8],
        [1, 7, 8, 3, 1, 2, 5, 6, 7],
        [8, 5, 9, 7, 6, 1, 4, 2, 3],
        [4, 2, 6, 8, 5, 3, 7, 9, 1],
        [7, 1, 3, 9, 2, 4, 8, 5, 6],
        [9, 6, 1, 5, 3, 7, 2, 8, 4],
        [2, 8, 7, 4, 1, 9, 6, 3, 5],
        [3, 4, 5, 2, 8, 6, 1, 7, 9],
    ]);
    assert_eq!(s.validate(), false);
}

#[test]
fn solve_once() {
    let g = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];
    let expected = [
        [5, 3, 4, 6, 7, 8, 9, 1, 2],
        [6, 7, 2, 1, 9, 5, 3, 4, 8],
        [1, 9, 8, 3, 4, 2, 5, 6, 7],
        [8, 5, 9, 7, 6, 1, 4, 2, 3],
        [4, 2, 6, 8, 5, 3, 7, 9, 1],
        [7, 1, 3, 9, 2, 4, 8, 5, 6],
        [9, 6, 1, 5, 3, 7, 2, 8, 4],
        [2, 8, 7, 4, 1, 9, 6, 3, 5],
        [3, 4, 5, 2, 8, 6, 1, 7, 9],
    ];
    let mut s = Sudoku::from(g);
    assert_eq!(s.solve_once(), true);
    for y in 0..9 {
        for x in 0..9 {
            assert_eq!(s.get(x, y), expected[y][x]);
        }
    }
}

// #[test]
// fn solve_once_anti_brute_force() {
//     let g = [
//         [0, 0, 0, 0, 0, 0, 0, 0, 0],
//         [0, 0, 0, 0, 0, 3, 0, 8, 5],
//         [0, 0, 1, 0, 2, 0, 0, 0, 0],
//         [0, 0, 0, 5, 0, 7, 0, 0, 0],
//         [0, 0, 4, 0, 0, 0, 1, 0, 0],
//         [0, 9, 0, 0, 0, 0, 0, 0, 0],
//         [5, 0, 0, 0, 0, 0, 0, 7, 3],
//         [0, 0, 2, 0, 1, 0, 0, 0, 0],
//         [0, 0, 0, 0, 4, 0, 0, 0, 9],
//     ];
//     let expected = [
//         [9, 8, 7, 6, 5, 4, 3, 2, 1],
//         [2, 4, 6, 1, 7, 3, 9, 8, 5],
//         [3, 5, 1, 9, 2, 8, 7, 4, 6],
//         [1, 2, 8, 5, 3, 7, 6, 9, 4],
//         [6, 3, 4, 8, 9, 2, 1, 5, 7],
//         [7, 9, 5, 4, 6, 1, 8, 3, 2],
//         [5, 1, 9, 2, 8, 6, 4, 7, 3],
//         [4, 7, 2, 3, 1, 9, 5, 6, 8],
//         [8, 6, 3, 7, 4, 5, 2, 1, 9],
//     ];
//     let mut s = Sudoku::from(g);
//     assert_eq!(s.solve_once(), true);
//     for y in 0..9 {
//         for x in 0..9 {
//             assert_eq!(s.get(x, y), expected[y][x]);
//         }
//     }
// }

// /// The eveil level from https://sudoku.com/evil/.
// #[test]
// fn solve_once_evil() {
//     let g = [
//         [0, 0, 0, 0, 0, 1, 0, 7, 2],
//         [3, 2, 0, 0, 7, 0, 0, 0, 4],
//         [6, 0, 0, 0, 0, 0, 0, 0, 0],
//         [7, 4, 0, 0, 1, 0, 0, 0, 3],
//         [0, 0, 8, 0, 0, 0, 0, 0, 0],
//         [0, 0, 0, 5, 0, 0, 9, 0, 0],
//         [0, 0, 6, 0, 2, 0, 0, 0, 0],
//         [2, 8, 0, 0, 0, 9, 3, 0, 0],
//         [0, 0, 1, 0, 0, 0, 0, 0, 8],
//     ];
//     let expected = [
//         [8, 5, 4, 3, 9, 1, 6, 7, 2],
//         [3, 2, 9, 6, 7, 5, 1, 8, 4],
//         [6, 1, 7, 8, 4, 2, 5, 3, 9],
//         [7, 4, 2, 9, 1, 6, 8, 5, 3],
//         [5, 9, 8, 2, 3, 7, 4, 1, 6],
//         [1, 6, 3, 5, 8, 4, 9, 2, 7],
//         [4, 3, 6, 1, 2, 8, 7, 9, 5],
//         [2, 8, 5, 7, 6, 9, 3, 4, 1],
//         [9, 7, 1, 4, 5, 3, 2, 6, 8],
//     ];
//     let mut s = Sudoku::from(g);
//     assert_eq!(s.solve_once(), true);
//     for y in 0..9 {
//         for x in 0..9 {
//             assert_eq!(s.get(x, y), expected[y][x]);
//         }
//     }
// }

// #[test]
// fn reset() {
//     let g = [
//         [5, 3, 0, 0, 7, 0, 0, 0, 0],
//         [6, 0, 0, 1, 9, 5, 0, 0, 0],
//         [0, 9, 8, 0, 0, 0, 0, 6, 0],
//         [8, 0, 0, 0, 6, 0, 0, 0, 3],
//         [4, 0, 0, 8, 0, 3, 0, 0, 1],
//         [7, 0, 0, 0, 2, 0, 0, 0, 6],
//         [0, 6, 0, 0, 0, 0, 2, 8, 0],
//         [0, 0, 0, 4, 1, 9, 0, 0, 5],
//         [0, 0, 0, 0, 8, 0, 0, 7, 9],
//     ];
//     let mut s = Sudoku::from(g);
//     s.solve_once();
//     s.reset();
//     for y in 0..9 {
//         for x in 0..9 {
//             assert_eq!(s.get(x, y), g[y][x]);
//         }
//     }
// }