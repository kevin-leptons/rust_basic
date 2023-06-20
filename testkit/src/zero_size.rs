use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct ZeroSize {
    // Althrough this field is not used at all, it prevents external code
    // creates a new instance by `ZeroSize {}` and force them to use
    // [Self::new].
    _zero_size_array: [u8; 0],
}

impl ZeroSize {
    pub fn new() -> Self {
        return Self {
            _zero_size_array: [],
        };
    }
}

impl Eq for ZeroSize {}

impl PartialEq for ZeroSize {
    fn eq(&self, _: &Self) -> bool {
        return true;
    }
}

impl Ord for ZeroSize {
    fn cmp(&self, _: &Self) -> Ordering {
        return Ordering::Equal;
    }
}

impl PartialOrd for ZeroSize {
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        return Some(Ordering::Equal);
    }
}

impl Default for ZeroSize {
    fn default() -> Self {
        return Self::new();
    }
}
