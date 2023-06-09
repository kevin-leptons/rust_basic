use super::PriorityQueue;

/// An iterator over immutable items in a priority queue. It does not guarantee
/// items will arrive in ordered priority.
pub struct Iter<'a, T>
where
    T: Ord,
{
    queue: &'a PriorityQueue<T>,
    index: usize,
}

impl<'a, T> Iter<'a, T>
where
    T: Ord,
{
    pub(super) fn new(queue: &'a PriorityQueue<T>) -> Self {
        return Self { queue, index: 0 };
    }
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Ord,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.queue.size() {
            return None;
        }
        self.index += 1;
        return Some(&self.queue.slots[self.index - 1]);
    }
}
