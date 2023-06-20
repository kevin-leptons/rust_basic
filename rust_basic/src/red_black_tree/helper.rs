use super::node::{Color, Node};

#[derive(PartialEq, Clone)]
pub(super) enum Direction {
    Left,
    Right,
}

/// Input for creating a pair key-value.
///
/// # Example
///
/// ```
/// use rust_basic::red_black_tree::RawPair;
///
/// let raw: RawPair<&str, &str> = ("key", "value");
/// let (key, value) = raw;
pub type RawPair<K, V> = (K, V);

pub(super) struct RemoveNodeResult<K, V> {
    pub color: Color,
    pub current: *mut Node<K, V>,
    pub parent: *mut Node<K, V>,
    pub direction: Option<Direction>,
}
