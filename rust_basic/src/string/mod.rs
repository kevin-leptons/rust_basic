//! String - a data structure and related algorithms.
//!
//! This document - module level document does not contain other descriptions or
//! examples. To learn more about how to use, refer to specific Entry APIs which
//! is labeled `entry` in the following sections.
//!
//! If there is the first time, [Package Document](crate) is a good way to learn
//! more about how to explore APIs and access to [Entry APIs
//! List](crate#structs).

mod iter;
mod kmp_table;

use crate::queue::Queue;
pub use iter::FindIter;
use std::alloc::{self, handle_alloc_error, Layout};
use std::cmp::{min, Ordering};
use std::fmt::Display;
use std::ptr;

/// `entry` A container for ASCII string.
///
/// # Model
///
/// ```txt
///  +---+---+---+
///  | a | b | c |
///  +---+---+---+
///    0   1   2
///    ^   ^   ^
///    |   |   |
///    +------------- index
/// ```
/// # Panic
/// * Call [insert](Self::insert) with `to` that is greater than
///   [size](Self::size).
/// * Call [insert](Self::insert) or [append](Self::append) to a string that
///   already has size [usize::MAX].
/// * Call [get](Self::get) or [get_code](Self::get_code) with `index` that is
///   greater than or equal to [size](Self::size).
/// * Call [slice](Self::slice) with `from` that is greater than `to`; or `to`
///   is greater than [size](Self::size).
/// * Call [replace](Self::replace) and the result is longer than [usize::MAX].
/// * Call [replace](Self::replace) and there are more matches than
///   [usize::MAX].
/// * The string is going to use more than [isize::MAX] bytes.
///
/// # Example
///
/// ```
/// use rust_basic::String;
///
/// let mut string = String::from("aa bb aa");
/// let pattern = String::from("aa");
/// let value = String::from("zzz");
/// string.replace(&pattern, &value);
/// assert_eq!(string, String::from("zzz bb zzz"));
#[derive(Debug)]
pub struct String {
    head: *mut u8,
    size: usize,
    capacity: usize,
}

impl<'a> String {
    /// Create a new empty instance.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn new() -> Self {
        return Self {
            head: ptr::null_mut(),
            capacity: 0,
            size: 0,
        };
    }

    /// Quantity of characters.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn size(&self) -> usize {
        return self.size;
    }

    /// ASCII character at `index`.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn get(&self, index: usize) -> char {
        assert!(index < self.size, "expect: valid index");
        unsafe {
            return *self.head.add(index) as char;
        }
    }

    /// ASCII code at `index`.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn get_code(&self, index: usize) -> u8 {
        assert!(index < self.size, "expect: valid index");
        return unsafe { *self.head.add(index) };
    }

    /// Borrow as a `str`. todo:
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(1).
    pub fn as_str(&self) -> &str {
        unsafe {
            return std::mem::transmute(ptr::slice_from_raw_parts_mut(
                self.head, self.size,
            ));
        };
    }

    /// Get a copy of slice from the string.
    ///
    /// Time complexity: O(m).
    ///
    /// Space complexity: O(m).
    pub fn slice(&self, from: usize, to: usize) -> String {
        assert!(from <= to, "expect: valid index");
        assert!(to <= self.size, "expect: valid index");
        let raw = &self.as_str()[from..to];
        return String::from(raw);
    }

    /// Put a copy of content from an other string at `to`. Characters at `[to,
    /// end]` will be shift to `i + value.size()` where `i` is old index.
    ///
    /// Time complexity: O(n + m).
    ///
    /// Space complexity: O(n + m).
    ///
    /// # Example
    ///
    /// ```
    /// use rust_basic::String;
    ///
    /// let mut s = String::from("aa bb cc");
    /// s.insert(3, &String::from("zzz "));
    /// assert_eq!(s, String::from("aa zzz bb cc"));
    pub fn insert(&mut self, to: usize, value: &String) {
        assert!(to <= self.size, "expect: valid index");
        let new_size = self.size + value.size;
        assert!(new_size < usize::MAX, "expect: shorter value");
        let new_head = Self::alloc(new_size);
        unsafe {
            ptr::copy_nonoverlapping(self.head, new_head, to);
            ptr::copy_nonoverlapping(
                self.head.add(to),
                new_head.add(to + value.size),
                self.size - to,
            );
            ptr::copy_nonoverlapping(value.head, new_head.add(to), value.size);
            self.clear();
        }
        self.head = new_head;
        self.size = new_size;
        self.capacity = new_size;
    }

    /// Equivalent to [insert(size, value)](Self::insert).
    ///
    /// Time complexity: O(n + m).
    ///
    /// Space complexity: O(n + m).
    pub fn append(&mut self, value: &String) {
        self.insert(self.size(), value);
    }

    /// Remove whitespaces from head and tail.
    ///
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    pub fn trim(&mut self) {
        unsafe {
            let mut begin = 0;
            while begin < self.size {
                let code = *self.head.add(begin);
                if !Self::is_whitespace(code) {
                    break;
                }
                begin += 1;
            }
            let mut end = self.size;
            while end > begin {
                let code = *self.head.add(end - 1);
                if !Self::is_whitespace(code) {
                    break;
                }
                end -= 1;
            }
            self.size = end - begin;
            if begin > 0 {
                ptr::copy(self.head.add(begin), self.head, self.size);
            }
            self.narrow();
        }
    }

    /// Convert all alphabets to uppercase.
    ///
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    pub fn upper(&mut self) {
        unsafe {
            for i in 0..self.size {
                let code = *self.head.add(i);
                if code < 0x61 || code > 0x7A {
                    continue;
                }
                *(&mut *self.head.add(i)) = code - 0x20;
            }
        }
    }

    /// Convert all alphabets to lowercase.
    ///
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    pub fn lower(&mut self) {
        unsafe {
            for i in 0..self.size {
                let code = *self.head.add(i);
                if code < 0x41 || code > 0x5A {
                    continue;
                }
                *(&mut *self.head.add(i)) = code + 0x20;
            }
        }
    }

    /// For iteration over matched sub strings.
    ///
    /// Algorithm: Knuth–Morris–Pratt.
    ///
    /// Time complexity: O(n + m).
    ///
    /// Space complexity: O(n + m).
    pub fn find(&'a self, pattern: &'a String) -> FindIter {
        assert!(pattern.size > 0, "expect: not empty pattern");
        return FindIter::new(self, pattern);
    }

    /// Search for matched sub strings, remove them from matched indexes and put
    /// a copy of an other string to.
    ///
    /// Time complexity: O(n + m + k) where `m, k` is size of `pattern, value`
    /// respectively.
    ///
    /// Space complexity: O(n).
    pub fn replace(&mut self, pattern: &String, value: &String) {
        assert!(pattern.size > 0, "expect: not empty pattern");
        unsafe {
            if value.size > pattern.size {
                self.replace_expand(pattern, value);
            } else {
                self.replace_inplace(pattern, value);
            }
        }
    }

    /// Release memory and give back to allocator.
    ///
    ///  Time complexity: O(1).
    ///
    ///  Space complexity: O(1).
    pub fn clear(&mut self) {
        unsafe {
            self.size = 0;
            self.narrow();
        }
    }

    fn is_whitespace(code: u8) -> bool {
        return (code == 0x20) || (code >= 0x09 && code <= 0x0D);
    }

    unsafe fn replace_inplace(&mut self, pattern: &String, value: &String) {
        let mut founds = self.find(pattern).collect::<Queue<_>>();
        if founds.size() == 0 {
            return;
        }
        let new_size = self.size - (pattern.size - value.size) * founds.size();
        let mut read = 0;
        let mut write = 0;
        while founds.size() > 0 {
            let found = founds.pop();
            if read > 0 {
                ptr::copy(
                    self.head.add(read),
                    self.head.add(write),
                    found - read,
                )
            }
            write += found - read;
            read = found + pattern.size;
            ptr::copy_nonoverlapping(
                value.head,
                self.head.add(write),
                value.size,
            );
            write += value.size;
        }
        if read < self.size {
            ptr::copy(
                self.head.add(read),
                self.head.add(write),
                self.size - read,
            )
        }
        self.size = new_size;
        self.narrow();
    }

    unsafe fn replace_expand(&mut self, pattern: &String, value: &String) {
        let mut founds = self.find(pattern).collect::<Queue<_>>();
        if founds.size() == 0 {
            return;
        }
        let new_size = self.size + (value.size - pattern.size) * founds.size();
        let new_head = Self::alloc(new_size);
        let mut read = 0;
        let mut write = 0;
        while founds.size() > 0 {
            let found = founds.pop();
            let old_slice_size = found - read;
            ptr::copy_nonoverlapping(
                self.head.add(read),
                new_head.add(write),
                old_slice_size,
            );
            read = found + pattern.size;
            write += old_slice_size;
            ptr::copy_nonoverlapping(
                value.head,
                new_head.add(write),
                value.size,
            );
            write += value.size;
        }
        ptr::copy_nonoverlapping(
            self.head.add(read),
            new_head.add(write),
            self.size - read,
        );
        self.clear();
        self.head = new_head;
        self.capacity = new_size;
        self.size = new_size;
    }

    fn alloc(capacity: usize) -> *mut u8 {
        let layout = Layout::array::<u8>(capacity).unwrap();
        assert!(
            layout.size() <= isize::MAX as usize,
            "expect: smaller memory block"
        );
        unsafe {
            let memory = alloc::alloc(layout);
            if memory.is_null() {
                handle_alloc_error(layout);
            }
            return memory;
        }
    }

    unsafe fn narrow(&mut self) {
        if self.size == 0 && self.capacity > 0 {
            let old_layout = Layout::array::<u8>(self.capacity).unwrap();
            alloc::dealloc(self.head, old_layout);
            self.head = ptr::null_mut();
            self.capacity = 0;
            return;
        }
        let new_capacity = self.capacity / 2;
        if self.size >= new_capacity {
            return;
        }
        let old_layout = Layout::array::<u8>(self.capacity).unwrap();
        let new_layout = Layout::array::<u8>(new_capacity).unwrap();
        let new_head = alloc::realloc(self.head, old_layout, new_layout.size());
        if new_head.is_null() {
            handle_alloc_error(new_layout);
        }
        self.head = new_head;
        self.capacity = new_capacity;
    }
}

impl From<&str> for String {
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn from(raw: &str) -> Self {
        assert!(raw.is_ascii(), "expect: ASCII string");
        if raw.len() == 0 {
            return String::new();
        }
        unsafe {
            let head = Self::alloc(raw.len());
            ptr::copy_nonoverlapping(raw.as_ptr(), head, raw.len());
            return Self {
                head,
                capacity: raw.len(),
                size: raw.len(),
            };
        }
    }
}

impl Eq for String {}

impl PartialEq for String {
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn eq(&self, other: &Self) -> bool {
        return self.cmp(other) == Ordering::Equal;
    }
}

impl Ord for String {
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn cmp(&self, other: &Self) -> Ordering {
        let n = min(self.size, other.size);
        for i in 0..n {
            let c0 = self.get_code(i);
            let c1 = other.get_code(i);
            if c0 > c1 {
                return Ordering::Greater;
            } else if c0 < c1 {
                return Ordering::Less;
            }
        }
        return self.size.cmp(&other.size);
    }
}

impl PartialOrd for String {
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Clone for String {
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    fn clone(&self) -> Self {
        return String::from(self.as_str());
    }
}

impl Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.write_str(self.as_str());
    }
}

impl Drop for String {
    /// Equivalent to [Self::clear].
    fn drop(&mut self) {
        self.clear();
    }
}
