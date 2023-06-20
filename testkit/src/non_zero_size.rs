use rust_basic::Hashable;
use std::alloc::{self, handle_alloc_error, Layout};
use std::cmp::Ordering;
use std::mem;
use std::ptr;

#[derive(Debug)]
pub struct NonZeroSize<T = usize> {
    pub value: T,
    size: usize,
    data: *mut u8,
}

impl<T> NonZeroSize<T> {
    const MIN_SIZE: usize = 2;

    pub fn new(value: T) -> Self {
        let size = mem::size_of::<T>() + Self::MIN_SIZE;
        let layout = Layout::array::<u8>(size).unwrap();
        assert!(
            layout.size() <= isize::MAX as usize,
            "unexpected: request too large memory"
        );
        let data = unsafe { alloc::alloc(layout) };
        if data.is_null() {
            handle_alloc_error(layout);
        }
        unsafe { ptr::write_bytes(data, 1, size) };
        return Self { value, size, data };
    }
}

impl<T> Clone for NonZeroSize<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        return Self::new(self.value.clone());
    }
}

impl<T> Eq for NonZeroSize<T> where T: Eq {}

impl<T> PartialEq for NonZeroSize<T>
where
    T: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        return self.value == other.value;
    }
}

impl<T> Ord for NonZeroSize<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        return self.value.cmp(&other.value);
    }
}

impl<T> PartialOrd for NonZeroSize<T>
where
    T: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl<T> Default for NonZeroSize<T>
where
    T: Default,
{
    fn default() -> Self {
        return Self::new(T::default());
    }
}

impl Hashable for NonZeroSize {
    fn hash(&self) -> u64 {
        return self.value.hash();
    }
}

impl<T> Drop for NonZeroSize<T> {
    fn drop(&mut self) {
        let layout = Layout::array::<u8>(self.size).unwrap();
        unsafe { alloc::dealloc(self.data, layout) };
    }
}
