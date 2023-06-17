pub(super) enum State {
    NotFound,
    Empty,
    Filled,
    Deleted,
}

pub(super) struct Slot<K, V> {
    pub state: State,
    pub key: K,
    pub hash: u64,
    pub value: V,
}
