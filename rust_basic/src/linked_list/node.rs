#[derive(Debug)]
pub(super) struct Node<T> {
    pub next: *mut Node<T>,
    pub prev: *mut Node<T>,
    pub item: T,
}

pub(super) struct Cursor<T> {
    pub prev: *mut Node<T>,
    pub current: *mut Node<T>,
}
