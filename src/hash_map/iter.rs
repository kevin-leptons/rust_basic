use super::{HashMap, State};
use crate::HashKey;

/// For iteration over pairs key-value in a hash map.
pub struct Iter<'a, K, V>
where
    K: HashKey,
{
    map: &'a HashMap<K, V>,
    index: usize,
}

impl<'a, K, V> Iter<'a, K, V>
where
    K: HashKey,
{
    pub(super) fn new(map: &'a HashMap<K, V>) -> Self {
        return Self { map: map, index: 0 };
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V>
where
    K: HashKey,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index >= self.map.capacity {
                return None;
            }
            let slot = unsafe {
                self.map.slots.as_ptr().add(self.index).as_ref().unwrap()
            };
            self.index += 1;
            match slot.state {
                State::Filled => return Some((&slot.key, &slot.value)),
                _ => {}
            };
        }
    }
}

/// For iteration over keys in a hash map.
pub struct KeyIter<'a, K, V>
where
    K: HashKey,
{
    map: &'a HashMap<K, V>,
    index: usize,
}

impl<'a, K, V> KeyIter<'a, K, V>
where
    K: HashKey,
{
    pub(super) fn new(map: &'a HashMap<K, V>) -> Self {
        return Self { map: map, index: 0 };
    }
}

impl<'a, K, V> Iterator for KeyIter<'a, K, V>
where
    K: HashKey,
{
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index >= self.map.capacity {
                return None;
            }
            let slot = unsafe {
                self.map.slots.as_ptr().add(self.index).as_ref().unwrap()
            };
            self.index += 1;
            match slot.state {
                State::Filled => return Some(&slot.key),
                _ => {}
            };
        }
    }
}

/// For iteration over immutable values in a hash map.
pub struct ValueIter<'a, K, V>
where
    K: HashKey,
{
    map: &'a HashMap<K, V>,
    index: usize,
}

impl<'a, K, V> ValueIter<'a, K, V>
where
    K: HashKey,
{
    pub(super) fn new(map: &'a HashMap<K, V>) -> Self {
        return Self { map: map, index: 0 };
    }
}

impl<'a, K, V> Iterator for ValueIter<'a, K, V>
where
    K: HashKey,
{
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index >= self.map.capacity {
                return None;
            }
            let slot = unsafe {
                self.map.slots.as_ptr().add(self.index).as_ref().unwrap()
            };
            self.index = self.index + 1;
            match slot.state {
                State::Filled => return Some(&slot.value),
                _ => {}
            }
        }
    }
}

/// For iteration over mutable values in a hash map.
pub struct ValueIterMut<'a, K, V>
where
    K: HashKey,
{
    map: &'a mut HashMap<K, V>,
    index: usize,
}

impl<'a, K, V> ValueIterMut<'a, K, V>
where
    K: HashKey,
{
    pub(super) fn new(map: &'a mut HashMap<K, V>) -> Self {
        return Self { map: map, index: 0 };
    }
}

impl<'a, K, V> Iterator for ValueIterMut<'a, K, V>
where
    K: HashKey,
{
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index >= self.map.capacity {
                return None;
            }
            let slot = unsafe {
                self.map.slots.as_ptr().add(self.index).as_mut().unwrap()
            };
            self.index = self.index + 1;
            match slot.state {
                State::Filled => return Some(&mut slot.value),
                _ => {}
            }
        }
    }
}
