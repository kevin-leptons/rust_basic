use super::Sudoku;

pub(super) fn solve_backtrack(target: &mut Sudoku) -> bool {
    if target.validate() {
        return true;
    }
    let (x, y) = match target.candidate_cell() {
        None => return false,
        Some(v) => v,
    };
    let cell = target.grid.get_mut(x, y);
    let candidates = cell.candidates.clone();
    cell.locked = true;
    drop(cell);
    for c in candidates.iter() {
        target.grid.get_mut(x, y).value = *c;
        if target.valid_maybe(x, y) {
            if solve_backtrack(target) {
                return true;
            }
        }
    }
    let cell = target.grid.get_mut(x, y);
    cell.locked = false;
    cell.value = 0;
    return false;
}
