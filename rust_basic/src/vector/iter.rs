use crate::Vector;
use std::alloc;
use std::alloc::Layout;
use std::mem;
use std::ptr;

/// For take owner and iteration over items in a vector.
pub struct IntoIter<T> {
    slots: *mut T,
    size: usize,
    capacity: usize,
    front: usize,
    back: usize,
}

/// [IntoIter::back] does not point to last item, it points to next of last
/// item.
impl<T> IntoIter<T> {
    pub(super) fn new(vector: Vector<T>) -> Self {
        let iter = Self {
            slots: vector.slots,
            size: vector.size,
            capacity: vector.capacity,
            front: 0,
            back: vector.size,
        };
        mem::forget(vector);
        return iter;
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size == 0 {
            return None;
        }
        unsafe {
            let item = ptr::read(self.slots.add(self.front));
            self.size -= 1;
            self.front += 1;
            return Some(item);
        }
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.size == 0 {
            return None;
        }
        unsafe {
            let item = ptr::read(self.slots.add(self.back - 1));
            self.size -= 1;
            self.back -= 1;
            return Some(item);
        }
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        if mem::size_of::<T>() == 0 {
            return;
        }
        unsafe {
            if self.size > 0 {
                ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                    self.slots.add(self.front),
                    self.size,
                ));
            }
            if self.capacity > 0 {
                let layout = Layout::array::<T>(self.capacity).unwrap();
                alloc::dealloc(self.slots as *mut u8, layout);
            }
        }
    }
}
