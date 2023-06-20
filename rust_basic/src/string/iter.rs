use super::kmp_table::KmpTable;
use super::String;

/// For iteration over matched sub strings. It's item is index of the first
/// matched character.
pub struct FindIter<'a> {
    target: &'a String,
    table: KmpTable<'a>,
    index: usize,
}

impl<'a> FindIter<'a> {
    pub(super) fn new(target: &'a String, pattern: &'a String) -> Self {
        return Self {
            target,
            table: KmpTable::new(pattern),
            index: 0,
        };
    }
}

impl Iterator for FindIter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.table.find(self.target, self.index) {
            None => {
                self.index = self.target.size;
                return None;
            }
            Some(index) => {
                self.index = index + 1;
                return Some(index);
            }
        }
    }
}
