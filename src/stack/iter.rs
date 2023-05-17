use super::Stack;

/// An iterator over the items of a stack, from bottom to top.
pub struct Iter<'a, T> {
    stack: &'a Stack<T>,
    index: usize,
}

impl<'a, T> Iter<'a, T> {
    pub(super) fn new(stack: &'a Stack<T>) -> Self {
        return Self { stack, index: 0 };
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.stack.top {
            return None;
        }
        let r = Some(self.stack.get(self.index));
        self.index += 1;
        return r;
    }
}
