use super::{HashMap, Hashable, State};

/// For iteration over immutable pairs key-value in a hash map.
pub struct Iter<'a, K, V>
where
    K: Hashable + Eq,
{
    map: &'a HashMap<K, V>,
    index: usize,
}

impl<'a, K, V> Iter<'a, K, V>
where
    K: Hashable + Eq,
{
    pub(super) fn new(map: &'a HashMap<K, V>) -> Self {
        return Self { map, index: 0 };
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V>
where
    K: Hashable + Eq,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index == self.map.capacity {
                return None;
            }
            let slot = unsafe { &*self.map.slots.add(self.index) };
            self.index += 1;
            if slot.state == State::Filled {
                return Some((&slot.key, &slot.value));
            }
        }
    }
}

/// For iteration over immutable keys in a hash map.
pub struct KeyIter<'a, K, V>
where
    K: Hashable + Eq,
{
    map: &'a HashMap<K, V>,
    index: usize,
}

impl<'a, K, V> KeyIter<'a, K, V>
where
    K: Hashable + Eq,
{
    pub(super) fn new(map: &'a HashMap<K, V>) -> Self {
        return Self { map: map, index: 0 };
    }
}

impl<'a, K, V> Iterator for KeyIter<'a, K, V>
where
    K: Hashable + Eq,
{
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index >= self.map.capacity {
                return None;
            }
            let slot = unsafe { &*self.map.slots.add(self.index) };
            self.index += 1;
            if slot.state == State::Filled {
                return Some(&slot.key);
            }
        }
    }
}

/// For iteration over immutable values in a hash map.
pub struct ValueIter<'a, K, V>
where
    K: Hashable + Eq,
{
    map: &'a HashMap<K, V>,
    index: usize,
}

impl<'a, K, V> ValueIter<'a, K, V>
where
    K: Hashable + Eq,
{
    pub(super) fn new(map: &'a HashMap<K, V>) -> Self {
        return Self { map, index: 0 };
    }
}

impl<'a, K, V> Iterator for ValueIter<'a, K, V>
where
    K: Hashable + Eq,
{
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index >= self.map.capacity {
                return None;
            }
            let slot = unsafe { &*self.map.slots.add(self.index) };
            self.index += 1;
            if slot.state == State::Filled {
                return Some(&slot.value);
            }
        }
    }
}

/// For iteration over mutable values in a hash map.
pub struct ValueIterMut<'a, K, V>
where
    K: Hashable + Eq,
{
    map: &'a mut HashMap<K, V>,
    index: usize,
}

impl<'a, K, V> ValueIterMut<'a, K, V>
where
    K: Hashable + Eq,
{
    pub(super) fn new(map: &'a mut HashMap<K, V>) -> Self {
        return Self { map, index: 0 };
    }
}

impl<'a, K, V> Iterator for ValueIterMut<'a, K, V>
where
    K: Hashable + Eq,
{
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index >= self.map.capacity {
                return None;
            }
            let slot = unsafe { &mut *self.map.slots.add(self.index) };
            self.index += 1;
            if slot.state == State::Filled {
                return Some(&mut slot.value);
            }
        }
    }
}
