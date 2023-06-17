use crate::Vector;

#[derive(Clone, Default)]
pub(super) struct Cell {
    pub value: u8,
    pub locked: bool,
    pub fixed: bool,
    pub candidates: Vector<u8>,
}
