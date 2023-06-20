use crate::Vector;

#[derive(Clone, Default, Debug)]
pub(super) struct Cell {
    pub value: u8,
    pub locked: bool,
    pub fixed: bool,
    pub candidates: Vector<u8>,
}

impl Eq for Cell {}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        return self.value == other.value;
    }
}
