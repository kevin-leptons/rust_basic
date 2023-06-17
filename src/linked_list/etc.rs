#[derive(Debug)]
pub(super) struct Node<T> {
    pub next: Option<*mut Node<T>>,
    pub prev: Option<*mut Node<T>>,
    pub item: T,
}

pub(super) struct Cursor<T> {
    pub prev: Option<*mut Node<T>>,
    pub current: Option<*mut Node<T>>,
}
