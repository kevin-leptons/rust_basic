//! Hash Map - a data structure and related algorithms.
//!
//! This document - module level document does not contain other descriptions or
//! examples. To learn more about how to use, refer to specific Entry APIs which
//! is labeled `entry` in the following sections.
//!
//! If there is the first time, [Package Document](crate) is a good way to learn
//! more about how to explore APIs and access to [Entry APIs
//! List](crate#structs).

mod iter;

use crate::hash::HashKey;
pub use iter::{Iter, KeyIter, ValueIter, ValueIterMut};
use std::alloc::{self, handle_alloc_error, Layout};
use std::ptr::{self, NonNull};

enum State {
    NotFound,
    Empty,
    Filled,
    Deleted,
}

struct Slot<K: HashKey, V> {
    state: State,
    key: K,
    value: V,
}

/// `entry` A container for pairs key-value.
///
/// # Example
///
/// ```
/// use rust_basic::HashMap;
///
/// let mut m = HashMap::<u32, &str>::from([
///     (1, "one"),
///     (2, "two"),
///     (3, "there"),
/// ]);
/// m.set(4, "four");
/// assert_eq!(m.has(&4), true);
/// assert_eq!(m.get(&1), Some(&"one"));
/// assert_eq!(m.remove(&1), Some("one"));
/// assert_eq!(m.has(&1), false);
/// assert_eq!(m.size(), 3);
#[derive(Debug)]
pub struct HashMap<K: HashKey, V> {
    slots: NonNull<Slot<K, V>>,
    size: usize,
    capacity: usize,
}

impl<K: HashKey, V> HashMap<K, V> {
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn new() -> Self {
        return Self {
            slots: NonNull::dangling(),
            size: 0,
            capacity: 0,
        };
    }

    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn size(&self) -> usize {
        return self.size;
    }

    /// * Time complexity: O(1) or O(n).
    /// * Space complexity: O(n).
    pub fn set(&mut self, key: K, value: V) -> Option<V> {
        self.expand();
        match Self::set_to(key, value, &self.slots, self.capacity) {
            None => {
                self.size = self.size + 1;
                return Option::None;
            }
            Some(v) => return Option::Some(v),
        }
    }

    /// * Time complexity: O(1) or O(n).
    /// * Space complexity: O(n).
    pub fn get<'a>(&'a self, key: &K) -> Option<&'a V> {
        if self.size == 0 {
            return None;
        }
        let hash = key.hash_key();
        let from = (hash as usize) % self.capacity;
        let (state, index) =
            Self::lookup(from, key, &self.slots, self.capacity);
        match state {
            State::Empty | State::Deleted | State::NotFound => {
                return Option::None
            }
            State::Filled => {
                let value = Self::read_as_ref(index, &self.slots);
                return Option::Some(value);
            }
        }
    }

    /// * Time complexity: O(1) or O(n).
    /// * Space complexity: O(n).
    pub fn get_mut<'a>(&'a self, key: &K) -> Option<&'a mut V> {
        if self.size == 0 {
            return None;
        }
        let hash = key.hash_key();
        let from = (hash as usize) % self.capacity;
        let (state, index) =
            Self::lookup(from, key, &self.slots, self.capacity);
        match state {
            State::Empty | State::Deleted | State::NotFound => {
                return Option::None
            }
            State::Filled => {
                let value = Self::read_as_mut(index, &self.slots);
                return Option::Some(value);
            }
        }
    }

    /// * Time complexity: O(1) or O(n).
    /// * Space complexity: O(n).
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if self.size == 0 {
            return None;
        }
        let hash = key.hash_key();
        let from = (hash as usize) % self.capacity;
        match Self::lookup_key(from, key, &self.slots, self.capacity) {
            None => return Option::None,
            Some(v) => {
                let value = Self::read_as_deleted(v, &self.slots);
                self.size = self.size - 1;
                self.narrow();
                return Option::Some(value);
            }
        }
    }

    /// * Time complexity: O(1) or O(n).
    /// * Space complexity: O(n).
    pub fn has(&self, key: &K) -> bool {
        if self.size == 0 {
            return false;
        }
        let hash = key.hash_key();
        let from = (hash as usize) % self.capacity;
        let (state, _) = Self::lookup(from, &key, &self.slots, self.capacity);
        match state {
            State::Filled => return true,
            _ => return false,
        }
    }

    /// * For iteration over pairs key-value in this container.
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn iter(&self) -> Iter<K, V> {
        return Iter::new(self);
    }

    /// * For iteration over keys in this container.
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn keys(&self) -> KeyIter<K, V> {
        return KeyIter::new(self);
    }

    /// * Return an iterator that be use to iterate all values in the map.
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn values(&self) -> ValueIter<K, V> {
        return ValueIter::new(self);
    }

    /// * For iteration over mutable value references in this container.
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn values_mut(&mut self) -> ValueIterMut<K, V> {
        return ValueIterMut::new(self);
    }

    /// * Remove all items from the container, drop them and give back memory to
    ///   allocator.
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    pub fn clear(&mut self) {
        if self.capacity == 0 {
            return;
        }
        for i in 0..self.capacity {
            let memory = unsafe { self.slots.as_ptr().add(i) };
            let slot = unsafe { memory.as_mut().unwrap() };
            match slot.state {
                State::Filled => unsafe { ptr::drop_in_place(memory) },
                _ => {}
            }
        }
        unsafe {
            let layout = Layout::array::<Slot<K, V>>(self.capacity).unwrap();
            alloc::dealloc(self.slots.as_ptr() as *mut u8, layout);
        }
        self.capacity = 0;
        self.size = 0;
        self.slots = NonNull::dangling();
    }

    fn lookup(
        from: usize,
        key: &K,
        slots: &NonNull<Slot<K, V>>,
        capacity: usize,
    ) -> (State, usize) {
        unsafe {
            for index in from..capacity {
                let slot = slots.as_ptr().add(index).as_ref().unwrap();
                match slot.state {
                    State::Empty => return (State::Empty, index),
                    State::Deleted => continue,
                    State::Filled => {
                        if slot.key == *key {
                            return (State::Filled, index);
                        } else {
                            continue;
                        }
                    }
                    _ => panic!("lookup fails"),
                };
            }
            for index in 0..from {
                let slot = slots.as_ptr().add(index).as_ref().unwrap();
                match slot.state {
                    State::Empty => return (State::Empty, index),
                    State::Deleted => return (State::Deleted, index),
                    State::Filled => {
                        if slot.key == *key {
                            return (State::Filled, index);
                        } else {
                            continue;
                        }
                    }
                    _ => panic!("lookup fails"),
                };
            }
            return (State::NotFound, 0);
        }
    }

    fn lookup_key(
        from: usize,
        key: &K,
        slots: &NonNull<Slot<K, V>>,
        capacity: usize,
    ) -> Option<usize> {
        unsafe {
            for index in from..capacity {
                let slot = slots.as_ptr().add(index).as_ref().unwrap();
                match slot.state {
                    State::Empty => return Option::None,
                    State::Deleted => continue,
                    State::Filled => {
                        if slot.key == *key {
                            return Option::Some(index);
                        } else {
                            continue;
                        }
                    }
                    _ => panic!("lookup fails"),
                };
            }
            for index in 0..from {
                let slot = slots.as_ptr().add(index).as_ref().unwrap();
                match slot.state {
                    State::Empty => return Option::None,
                    State::Deleted => continue,
                    State::Filled => {
                        if slot.key == *key {
                            return Option::Some(index);
                        } else {
                            continue;
                        }
                    }
                    _ => panic!("lookup fails"),
                };
            }
            return Option::None;
        }
    }

    fn set_to(
        key: K,
        value: V,
        slots: &NonNull<Slot<K, V>>,
        capacity: usize,
    ) -> Option<V> {
        let hash = key.hash_key();
        let from = (hash as usize) % capacity;
        let (state, index) = Self::lookup(from, &key, &slots, capacity);
        match state {
            State::Empty | State::Deleted => {
                Self::write(index, key, value, slots);
                return Option::None;
            }
            State::Filled => {
                let old_value = Self::set_replace(index, key, value, slots);
                return Option::Some(old_value);
            }
            _ => panic!("bad slots"),
        }
    }

    fn set_replace(
        index: usize,
        key: K,
        new_value: V,
        slots: &NonNull<Slot<K, V>>,
    ) -> V {
        let old_value = Self::read(index, slots);
        Self::write(index, key, new_value, slots);
        return old_value;
    }

    fn read(index: usize, slots: &NonNull<Slot<K, V>>) -> V {
        let slot = unsafe { ptr::read(slots.as_ptr().add(index)) };
        return slot.value;
    }

    fn read_as_deleted(index: usize, slots: &NonNull<Slot<K, V>>) -> V {
        let memory = unsafe { slots.as_ptr().add(index) };
        let slot_mut = unsafe { memory.as_mut().unwrap() };
        slot_mut.state = State::Deleted;
        let slot = unsafe { ptr::read(memory) };
        return slot.value;
    }

    fn read_as_ref<'a>(index: usize, slots: &'a NonNull<Slot<K, V>>) -> &'a V {
        let slot = unsafe { slots.as_ptr().add(index).as_mut().unwrap() };
        return &slot.value;
    }

    fn read_as_mut<'a>(
        index: usize,
        slots: &'a NonNull<Slot<K, V>>,
    ) -> &'a mut V {
        let slot = unsafe { slots.as_ptr().add(index).as_mut().unwrap() };
        return &mut slot.value;
    }

    fn write(index: usize, key: K, value: V, slots: &NonNull<Slot<K, V>>) {
        let slot = Slot::<K, V> {
            state: State::Filled,
            key: key,
            value: value,
        };
        unsafe { ptr::write(slots.as_ptr().add(index), slot) }
    }

    fn expand(&mut self) {
        if self.capacity > 0 {
            let load = 100 * (self.size + 1) / self.capacity;
            if load < 70 {
                return;
            }
        }
        let new_capacity = match self.capacity {
            0 => 2,
            _ => 2 * self.capacity,
        };
        let new_slots = Self::new_slots(new_capacity);

        if self.capacity > 0 {
            self.move_slots(&new_slots, new_capacity);
            let layout = Layout::array::<Slot<K, V>>(self.capacity).unwrap();
            unsafe { alloc::dealloc(self.slots.as_ptr() as *mut u8, layout) };
        }
        self.slots = new_slots;
        self.capacity = new_capacity;
    }

    fn new_slots(capacity: usize) -> NonNull<Slot<K, V>> {
        let new_layout = Layout::array::<Slot<K, V>>(capacity).unwrap();
        let memory = unsafe { alloc::alloc(new_layout) };
        let new_slots = match NonNull::new(memory as *mut Slot<K, V>) {
            None => handle_alloc_error(new_layout),
            Some(v) => v,
        };
        for i in 0..capacity {
            let mut slot =
                unsafe { new_slots.as_ptr().add(i).as_mut().unwrap() };
            slot.state = State::Empty;
        }
        return new_slots;
    }

    fn move_slots(
        &mut self,
        new_slots: &NonNull<Slot<K, V>>,
        new_capacity: usize,
    ) {
        for i in 0..self.capacity {
            let slot = unsafe { self.slots.as_ptr().add(i).as_ref().unwrap() };
            match slot.state {
                State::Empty | State::Deleted => continue,
                State::Filled => {
                    let key: K = unsafe { ptr::read(&slot.key) };
                    let value: V = unsafe { ptr::read(&slot.value) };
                    Self::set_to(key, value, &new_slots, new_capacity);
                }
                _ => panic!("bad slots"),
            }
        }
    }

    fn narrow(&mut self) {
        if self.capacity == 0 {
            return;
        }
        let new_capacity = self.capacity / 2;
        let new_load = 100 * self.size / new_capacity;
        if new_load >= 50 {
            return;
        }
        let new_slots = Self::new_slots(new_capacity);
        self.move_slots(&new_slots, new_capacity);
        let layout = Layout::array::<Slot<K, V>>(self.capacity).unwrap();
        unsafe { alloc::dealloc(self.slots.as_ptr() as *mut u8, layout) };
        self.slots = new_slots;
        self.capacity = new_capacity;
    }
}

impl<K, V, const N: usize> From<[(K, V); N]> for HashMap<K, V>
where
    K: HashKey,
{
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    fn from(value: [(K, V); N]) -> Self {
        let mut m = HashMap::<K, V>::new();
        for (key, value) in value {
            m.set(key, value);
        }
        return m;
    }
}

impl<K, V> FromIterator<(K, V)> for HashMap<K, V>
where
    K: HashKey,
{
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut m = HashMap::<K, V>::new();
        for (key, value) in iter {
            m.set(key, value);
        }
        return m;
    }
}

impl<K, V> PartialEq for HashMap<K, V>
where
    K: HashKey,
    V: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }
        for (k, v) in self.iter() {
            let v_other = match other.get(k) {
                None => return false,
                Some(v) => v,
            };
            if v != v_other {
                return false;
            }
        }
        return true;
    }
}

impl<K: HashKey, V> Drop for HashMap<K, V> {
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    fn drop(&mut self) {
        self.clear();
    }
}
