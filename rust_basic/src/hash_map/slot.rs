#[derive(PartialEq, Eq)]
pub(super) enum State {
    Empty,
    Filled,
    Deleted,
}

pub(super) struct Slot<K, V> {
    pub key: K,
    pub value: V,
    pub hash: u64,
    pub state: State,
}
