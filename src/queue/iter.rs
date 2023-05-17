use super::Queue;

/// An iterator over the items of a queue, from head to tail.
pub struct Iter<'a, T> {
    queue: &'a Queue<T>,
    index: usize,
}

impl<'a, T> Iter<'a, T> {
    pub(super) fn new(queue: &'a Queue<T>) -> Self {
        return Self {
            queue: queue,
            index: 0,
        };
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.queue.size {
            return None;
        }
        let i = self.queue.get(self.index);
        self.index += 1;
        return Some(i);
    }
}
