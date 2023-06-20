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
mod slot;

use crate::Hashable;
pub use iter::{Iter, KeyIter, ValueIter, ValueIterMut};
use slot::{Slot, State};
use std::alloc::{self, handle_alloc_error, Layout};
use std::ptr;

/// `entry` A container for pairs key-value.
///
/// # Model
///
/// ```txt
///   +---------------- key
///   |
///   v
/// +-------------+
/// | 1 | "one"   |<--- value
/// |-------------|
/// | 2 | "two"   |
/// |-------------|
/// | 3 | "three" |
/// +-------------+
///   ^      ^
///   |      |
///   +--------------- pair
/// ```
///
/// # Panic
///
/// * Call [set](Self::set) to a hash map that is already has size [usize::MAX].
/// * Call [set](Self::set) and make the hash map uses more than [isize::MAX]
///   bytes.
///
/// # Example
///
/// ```
/// use rust_basic::HashMap;
///
/// let mut map = HashMap::<u32, &str>::from([
///     (1, "one"),
///     (2, "two"),
///     (3, "there"),
/// ]);
/// map.set(4, "four");
/// assert_eq!(map.has(&4), true);
/// assert_eq!(map.remove(&1), Some("one"));
/// assert_eq!(map.get(&1), None);
#[derive(Debug)]
pub struct HashMap<K, V>
where
    K: Hashable + Eq,
{
    slots: *mut Slot<K, V>,
    size: usize,
    capacity: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Hashable + Eq,
{
    /// A ratio of `size / capacity` in percentage. If the value is greater than
    /// the threshold then the table will be expand. If the value is less than
    /// or equal the threshold then the table will be narrow.
    const LOAD_THRESHOLD: usize = 80;

    /// Create a new empty instance.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn new() -> Self {
        return Self {
            slots: ptr::null_mut(),
            size: 0,
            capacity: 0,
        };
    }

    /// Quantity of pairs.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn size(&self) -> usize {
        return self.size;
    }

    /// Put a new pair key-value. If the key is already existed then remove the
    /// old pair and return the old value.
    ///
    /// Time complexity: O(1) or O(n).
    ///
    /// Space complexity: O(n).
    pub fn set(&mut self, key: K, value: V) -> Option<V> {
        assert!(self.size < usize::MAX, "expect: not full hash map");
        unsafe {
            self.expand();
            let (slot, hash) =
                match Self::lookup_set(&key, self.slots, self.capacity) {
                    None => panic!("unexpected: lookup fails"),
                    Some(v) => v,
                };
            let old_value = match (*slot).state {
                State::Filled => Some(ptr::read(slot).value),
                _ => None,
            };
            let data = Slot {
                state: State::Filled,
                key,
                hash,
                value,
            };
            ptr::write(slot, data);
            if old_value.is_none() {
                self.size += 1
            }
            return old_value;
        }
    }

    /// Borrow immutable value.
    ///
    /// Time complexity: O(1) or O(n).
    ///
    /// Space complexity: O(n).
    pub fn get<'a>(&'a self, key: &K) -> Option<&'a V> {
        unsafe {
            let slot = match Self::lookup_get(key, self.slots, self.capacity) {
                None => return None,
                Some(v) => v,
            };
            return Some(&(*slot).value);
        }
    }

    /// Borrow mutable value.
    ///
    /// Time complexity: O(1) or O(n).
    ///
    /// Space complexity: O(n).
    pub fn get_mut<'a>(&'a mut self, key: &K) -> Option<&'a mut V> {
        unsafe {
            let slot = match Self::lookup_get(key, self.slots, self.capacity) {
                None => return None,
                Some(v) => v,
            };
            return Some(&mut (*slot).value);
        }
    }

    /// If the key does exist then return `true`.
    ///
    /// Time complexity: O(1) or O(n).
    ///
    /// Space complexity: O(n).
    pub fn has(&self, key: &K) -> bool {
        unsafe {
            return Self::lookup_get(key, self.slots, self.capacity).is_some();
        }
    }

    /// Remove a pair and return the old value.
    ///
    /// Time complexity: O(1) or O(n).
    ///
    /// Space complexity: O(n).
    pub fn remove(&mut self, key: &K) -> Option<V> {
        unsafe {
            let slot = match Self::lookup_get(key, self.slots, self.capacity) {
                None => return None,
                Some(v) => v,
            };
            (*slot).state = State::Deleted;
            let value = ptr::read(slot).value;
            self.size -= 1;
            self.narrow();
            return Some(value);
        }
    }

    /// For iteration over immutable pairs key-value.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn iter(&self) -> Iter<K, V> {
        return Iter::new(self);
    }

    /// For iteration over immutable keys.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn keys(&self) -> KeyIter<K, V> {
        return KeyIter::new(self);
    }

    /// For iteration over immutable values.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn values(&self) -> ValueIter<K, V> {
        return ValueIter::new(self);
    }

    /// For iteration over mutable values.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn values_mut(&mut self) -> ValueIterMut<K, V> {
        return ValueIterMut::new(self);
    }

    /// Remove all pairs, drop values and give back memory to allocator.
    ///
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    pub fn clear(&mut self) {
        unsafe {
            for i in 0..self.capacity {
                let slot = self.slots.add(i);
                if (*slot).state == State::Filled {
                    ptr::drop_in_place(slot);
                }
                self.size = 0;
            }
            if self.capacity > 0 {
                Self::delloc(self.slots, self.capacity);
                self.slots = ptr::null_mut();
                self.capacity = 0;
            }
        }
    }

    unsafe fn lookup_get(
        key: &K,
        slots: *mut Slot<K, V>,
        capacity: usize,
    ) -> Option<*mut Slot<K, V>> {
        let (from, hash) = match Self::lookup_prepare(key, capacity) {
            None => return None,
            Some(v) => v,
        };
        unsafe {
            match Self::lookup_get_range(from, capacity, key, hash, slots) {
                Some(v) => return Some(v),
                None => {}
            };
            match Self::lookup_get_range(0, from, key, hash, slots) {
                Some(v) => return Some(v),
                None => return None,
            };
        }
    }

    /// Return `(slot, hash)`.
    unsafe fn lookup_set(
        key: &K,
        slots: *mut Slot<K, V>,
        capacity: usize,
    ) -> Option<(*mut Slot<K, V>, u64)> {
        let (from, hash) = match Self::lookup_prepare(key, capacity) {
            None => return None,
            Some(v) => v,
        };
        unsafe {
            match Self::lookup_set_range(from, capacity, key, hash, slots) {
                Some(v) => return Some((v, hash)),
                None => {}
            };
            match Self::lookup_set_range(0, from, key, hash, slots) {
                Some(v) => return Some((v, hash)),
                None => return None,
            }
        }
    }

    unsafe fn lookup_get_range(
        from: usize,
        to: usize,
        key: &K,
        hash: u64,
        slots: *mut Slot<K, V>,
    ) -> Option<*mut Slot<K, V>> {
        for index in from..to {
            let slot = &mut *slots.add(index);
            match slot.state {
                State::Empty => return None,
                State::Deleted => continue,
                State::Filled => {
                    if slot.hash == hash && slot.key == *key {
                        return Some(slot);
                    }
                }
            };
        }
        return None;
    }

    unsafe fn lookup_set_range(
        from: usize,
        to: usize,
        key: &K,
        hash: u64,
        slots: *mut Slot<K, V>,
    ) -> Option<*mut Slot<K, V>> {
        for index in from..to {
            let slot = &mut *slots.add(index);
            match slot.state {
                State::Empty | State::Deleted => return Some(slot),
                State::Filled => {
                    if slot.hash == hash && slot.key == *key {
                        return Some(slot);
                    }
                }
            };
        }
        return None;
    }

    /// Return `(index, hash)`.
    fn lookup_prepare(key: &K, capacity: usize) -> Option<(usize, u64)> {
        if capacity == 0 {
            return None;
        }
        let hash = key.hash();
        let index = hash as usize % capacity;
        return Some((index, hash));
    }

    unsafe fn expand(&mut self) {
        if self.capacity > 0 && !Self::must_expand(self.size, self.capacity) {
            return;
        }
        let old_slots = self.slots;
        let old_capacity = self.capacity;
        self.capacity = match self.capacity {
            0 => 2,
            _ => 2 * self.capacity,
        };
        self.slots = Self::alloc(self.capacity);
        self.move_from_slots(old_slots, old_capacity);
        Self::delloc(old_slots, old_capacity);
    }

    unsafe fn narrow(&mut self) {
        if self.capacity == 0 {
            return;
        }
        let new_capacity = self.capacity / 2;
        if Self::must_expand(self.size, new_capacity) {
            return;
        }
        let old_slots = self.slots;
        let old_capacity = self.capacity;
        self.capacity = new_capacity;
        self.slots = Self::alloc(new_capacity);
        self.move_from_slots(old_slots, old_capacity);
        Self::delloc(old_slots, old_capacity);
    }

    unsafe fn move_from_slots(
        &mut self,
        slots: *mut Slot<K, V>,
        capacity: usize,
    ) {
        for i in 0..capacity {
            let slot = slots.add(i);
            match (*slot).state {
                State::Filled => self.move_from_slot(slot),
                _ => continue,
            }
        }
    }

    unsafe fn move_from_slot(&self, source: *mut Slot<K, V>) {
        let (target, _) =
            match Self::lookup_set(&(*source).key, self.slots, self.capacity) {
                None => panic!("unexpected: lookup fails"),
                Some(v) => v,
            };
        ptr::copy_nonoverlapping(source, target, 1);
    }

    fn must_expand(size: usize, capacity: usize) -> bool {
        let load_integer = 100 * size / capacity;
        let load_remain = size % capacity;
        if load_integer > Self::LOAD_THRESHOLD {
            return true;
        }
        if load_integer == Self::LOAD_THRESHOLD && load_remain > 0 {
            return true;
        }
        return false;
    }

    unsafe fn alloc(capacity: usize) -> *mut Slot<K, V> {
        if capacity == 0 {
            return ptr::null_mut();
        }
        let layout = Layout::array::<Slot<K, V>>(capacity).unwrap();
        assert!(
            layout.size() <= isize::MAX as usize,
            "expect: smaller memory block"
        );
        let slots = alloc::alloc(layout) as *mut Slot<K, V>;
        if slots.is_null() {
            handle_alloc_error(layout);
        }
        for i in 0..capacity {
            (*slots.add(i)).state = State::Empty;
        }
        return slots;
    }

    unsafe fn delloc(slots: *mut Slot<K, V>, capacity: usize) {
        if capacity == 0 {
            return;
        }
        let layout = Layout::array::<Slot<K, V>>(capacity).unwrap();
        alloc::dealloc(slots as *mut u8, layout);
    }
}

impl<K, V> FromIterator<(K, V)> for HashMap<K, V>
where
    K: Hashable + Eq,
{
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut m = HashMap::<K, V>::new();
        for (key, value) in iter {
            m.set(key, value);
        }
        return m;
    }
}

impl<K, V, const N: usize> From<[(K, V); N]> for HashMap<K, V>
where
    K: Hashable + Eq,
{
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn from(items: [(K, V); N]) -> Self {
        return Self::from_iter(items);
    }
}

impl<K, V> PartialEq for HashMap<K, V>
where
    K: Hashable + Eq,
    V: Eq,
{
    /// Time complexity: O(n) or O(n.m).
    ///
    /// Space complexity: O(n + m).
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }
        for (key, value) in self.iter() {
            let v_other = match other.get(key) {
                None => return false,
                Some(v) => v,
            };
            if value != v_other {
                return false;
            }
        }
        return true;
    }
}

impl<K, V> Eq for HashMap<K, V>
where
    K: Hashable + Eq,
    V: Eq,
{
}

impl<K, V> Clone for HashMap<K, V>
where
    K: Hashable + Eq + Clone,
    V: Clone,
{
    /// Time complexity: O(n) or O(n^2).
    ///
    /// Space complexity: O(n).
    fn clone(&self) -> Self {
        let mut map = HashMap::new();
        for (key, value) in self.iter() {
            map.set(key.clone(), value.clone());
        }
        return map;
    }
}

impl<K, V> Drop for HashMap<K, V>
where
    K: Hashable + Eq,
{
    /// Equivalent to [Self::clear].
    fn drop(&mut self) {
        self.clear();
    }
}
