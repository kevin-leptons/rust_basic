use std::{fmt::Display, ptr::NonNull};

pub(super) struct Node<K, V> {
    pub(super) key: K,
    pub(super) value: V,
    pub(super) parrent: Option<*mut Node<K, V>>,
    pub(super) left: Option<*mut Node<K, V>>,
    pub(super) right: Option<*mut Node<K, V>>,
    pub(super) color: Color,
}

impl<K, V> PartialEq for Node<K, V>
where
    K: Ord,
{
    fn eq(&self, other: &Self) -> bool {
        return self.key == other.key;
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub(super) enum Color {
    Red,
    Black,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
