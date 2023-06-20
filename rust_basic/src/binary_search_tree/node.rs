pub(super) struct Node<K, V> {
    pub(super) key: K,
    pub(super) value: V,
    pub(super) parent: Option<*mut Node<K, V>>,
    pub(super) left: Option<*mut Node<K, V>>,
    pub(super) right: Option<*mut Node<K, V>>,
}

impl<K, V> PartialEq for Node<K, V>
where
    K: Ord,
{
    fn eq(&self, other: &Self) -> bool {
        return self.key == other.key;
    }
}
