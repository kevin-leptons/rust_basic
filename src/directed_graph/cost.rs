use std::cmp::Ordering;

use super::Vertex;

pub(super) struct Cost<'a> {
    pub(super) vertex: &'a Vertex,
    pub(super) value: u64,
}

impl<'a> Cost<'a> {
    pub(super) fn new(vertex: &'a Vertex, value: u64) -> Self {
        return Self { vertex, value };
    }
}

impl<'a> Ord for Cost<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value < other.value {
            return Ordering::Greater;
        }
        if self.value > other.value {
            return Ordering::Less;
        }
        return other.vertex.identity.cmp(&self.vertex.identity);
    }
}

impl<'a> PartialOrd for Cost<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl<'a> Eq for Cost<'a> {}

impl<'a> PartialEq for Cost<'a> {
    fn eq(&self, other: &Self) -> bool {
        return self.value == other.value;
    }
}
