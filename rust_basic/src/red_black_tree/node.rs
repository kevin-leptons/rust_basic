pub(super) struct Node<K, V> {
    pub(super) key: K,
    pub(super) value: V,
    pub(super) parent: *mut Node<K, V>,
    pub(super) left: *mut Node<K, V>,
    pub(super) right: *mut Node<K, V>,
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
