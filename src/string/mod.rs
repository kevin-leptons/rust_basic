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

use std::alloc::{self, handle_alloc_error, Layout};
use std::cmp::{min, Ordering};
use std::ptr::{self, NonNull};

use crate::queue::Queue;
use crate::stack::Stack;
pub use iter::FindIter;

/// `entry` A container for ASCII string.
///
/// # Example
///
/// ```
/// use rust_basic::String;
///
/// let s = String::from("ab ac aa dd aa");
/// let p = String::from("aa");
/// let mut i = s.find(&p);
/// assert_eq!(i.next(), Some(6));
/// assert_eq!(i.next(), Some(12));
/// assert_eq!(i.next(), None);
#[derive(Debug)]
pub struct String {
    head: NonNull<u8>,
    size: usize,
    capacity: usize,
}

impl<'a> String {
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn new() -> Self {
        return Self {
            head: NonNull::dangling(),
            size: 0,
            capacity: 0,
        };
    }

    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn size(&self) -> usize {
        return self.size;
    }

    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn as_str(&self) -> &str {
        unsafe {
            let p =
                ptr::slice_from_raw_parts_mut(self.head.as_ptr(), self.size);
            return std::mem::transmute(p);
        };
    }

    /// * Time complexity: O(m).
    /// * Space complexity: O(m).
    pub fn slice(&self, from: usize, to: usize) -> String {
        assert!(from < to, "expect: `from` is less than `to`");
        assert!(to <= self.size, "expect: `to` is not greater than size");
        let mut s = String::new();
        s.size = to - from;
        s.recapacity_by_size(s.size);
        unsafe {
            ptr::copy_nonoverlapping(
                self.head.as_ptr().add(from),
                s.head.as_ptr(),
                s.size,
            )
        }
        return s;
    }

    /// * Time complexity: O(n + m).
    /// * Space complexity: O(n + m).
    pub fn insert(&mut self, to: usize, value: &String) {
        assert!(to <= self.size, "expect: `to` is not greater than size");
        let new_size = self.size + value.size;
        let new_head = Self::alloc(new_size);
        let old = self.head.as_ptr();
        let target = new_head.as_ptr();
        unsafe {
            ptr::copy_nonoverlapping(old, target, to);
            ptr::copy_nonoverlapping(
                old.add(to),
                target.add(to + value.size),
                self.size - to,
            );
            ptr::copy_nonoverlapping(
                value.head.as_ptr(),
                target.add(to),
                value.size,
            );
            self.clear();
        }
        self.head = new_head;
        self.size = new_size;
        self.capacity = new_size;
    }

    /// * Replace all matched pattern with `value`.
    /// * Time complexity: O(n + m + k) where `m, k` is size of `pattern, value`
    ///   respectively.
    /// * Space complexity: O(n).
    pub fn replace(&mut self, pattern: &String, value: &String) {
        assert!(pattern.size > 0, "expect: `pattern` is not empty");
        if pattern.size == value.size {
            self.replace_size_unchange(pattern, value);
        } else if pattern.size > value.size {
            self.replace_size_narrow(pattern, value);
        } else {
            self.replace_size_expand(pattern, value);
        }
    }

    /// * Equivalent as `insert(size, value)`.
    /// * Time complexity: O(n + m).
    /// * Space complexity: O(n + m).
    pub fn append(&mut self, value: &String) {
        self.insert(self.size(), value);
    }

    /// * Remove whitespaces from head and tail of the string.
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    pub fn trim(&mut self) {
        let p = self.head.as_ptr();
        let mut i = 0;
        loop {
            let code = unsafe { *p.add(i) };
            if !Self::is_whitespace(code) {
                break;
            }
            i += 1;
        }
        if i > 0 {
            unsafe {
                ptr::copy(p.add(i), p, self.size - i);
            }
            self.size -= i;
        }
        i = self.size;
        loop {
            if i == 0 {
                break;
            }
            i -= 1;
            let code = unsafe { *p.add(i) };
            if !Self::is_whitespace(code) {
                break;
            }
        }
        if (i + 1) < self.size {
            self.size = i + 1;
        }
    }

    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    pub fn upper(&mut self) {
        let p = self.head.as_ptr();
        unsafe {
            for i in 0..self.size {
                let c = *p.add(i);
                if c < 0x61 || c > 0x7A {
                    continue;
                }
                *(&mut *p.add(i)) = c - 0x20;
            }
        }
    }

    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    pub fn lower(&mut self) {
        let p = self.head.as_ptr();
        unsafe {
            for i in 0..self.size {
                let c = *p.add(i);
                if c < 0x41 || c > 0x5A {
                    continue;
                }
                *(&mut *p.add(i)) = c + 0x20;
            }
        }
    }

    /// * Retrieve ASCII code at `index`.
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn code_at(&self, index: usize) -> u8 {
        assert!(index < self.size, "expect: `index` is less than size");
        return unsafe { *self.head.as_ptr().add(index) };
    }

    /// * Retrieve ASCII character at `index`.
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn char_at(&self, index: usize) -> char {
        assert!(index < self.size, "expect: `index` is less than size");
        return unsafe { *self.head.as_ptr().add(index) as char };
    }

    /// * Find index that matches `pattern`.
    /// * Algorithm: Knuth–Morris–Pratt.
    /// * Time complexity: O(n + m).
    /// * Space complexity: O(n + m).
    pub fn find(&'a self, pattern: &'a String) -> FindIter {
        assert!(pattern.size > 0, "expect: `pattern` is not empty");
        return FindIter::new(self, pattern);
    }

    /// Release memory and set size of the string to zero.
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn clear(&mut self) {
        if self.capacity == 0 {
            return;
        }
        let layout = Layout::array::<u8>(self.capacity).unwrap();
        unsafe {
            alloc::dealloc(self.head.as_ptr(), layout);
        }
        self.capacity = 0;
        self.size = 0;
        self.head = NonNull::dangling();
    }

    fn recapacity_by_size(&mut self, size: usize) {
        if size == 0 {
            self.clear();
            return;
        }
        if size > self.capacity {
            self.recapacity(size);
            return;
        }
        let border = self.capacity / 2;
        if size < border {
            self.recapacity(border);
            return;
        }
    }

    fn recapacity(&mut self, capacity: usize) {
        assert!(capacity > 0, "expect: capacity is greater than zero");
        let new_layout = Layout::array::<u8>(capacity).unwrap();
        let memory = match self.capacity {
            0 => unsafe { alloc::alloc(new_layout) },
            _ => {
                let old_layout = Layout::array::<u8>(self.capacity).unwrap();
                unsafe {
                    alloc::realloc(
                        self.head.as_ptr(),
                        old_layout,
                        new_layout.size(),
                    )
                }
            }
        };
        self.head = match NonNull::new(memory) {
            None => handle_alloc_error(new_layout),
            Some(v) => v,
        };
        self.capacity = capacity;
    }

    fn alloc(capacity: usize) -> NonNull<u8> {
        unsafe {
            let layout = Layout::array::<u8>(capacity).unwrap();
            let memory = alloc::alloc(layout);
            match NonNull::new(memory) {
                None => handle_alloc_error(layout),
                Some(v) => return v,
            };
        }
    }

    fn is_whitespace(code: u8) -> bool {
        return (code == 0x20) || (code >= 0x09 && code <= 0x0D);
    }

    fn replace_size_unchange(&mut self, pattern: &String, value: &String) {
        let mut found = self.find_all_stack(pattern);
        loop {
            if found.size() == 0 {
                break;
            }
            let index = found.pop();
            unsafe {
                ptr::copy_nonoverlapping(
                    value.head.as_ptr(),
                    self.head.as_ptr().add(index),
                    value.size,
                );
            }
        }
    }

    fn replace_size_narrow(&mut self, pattern: &String, value: &String) {
        let mut found = self.find_all_queue(pattern);
        let new_size = self.size - (pattern.size - value.size) * found.size();
        let p = self.head.as_ptr();
        let v = value.head.as_ptr();
        let mut r = 0;
        let mut w = 0;
        loop {
            if found.size() == 0 {
                break;
            }
            let f = found.pop();
            if r > 0 {
                unsafe { ptr::copy(p.add(r), p.add(w), f - r) }
            }
            w += f - r;
            r = f + pattern.size;
            unsafe { ptr::copy_nonoverlapping(v, p.add(w), value.size) }
            w += value.size;
        }
        if r < self.size {
            unsafe { ptr::copy(p.add(r), p.add(w), self.size - r) }
        }
        self.size = new_size;
        self.narrow();
    }

    fn replace_size_expand(&mut self, pattern: &String, value: &String) {
        let mut found = self.find_all_stack(pattern);
        let new_size = self.size + (value.size - pattern.size) * found.size();
        self.expand_for_size(new_size);
        let p = self.head.as_ptr();
        let v = value.head.as_ptr();
        let mut r = self.size;
        let mut w = new_size;
        loop {
            if found.size() == 0 {
                break;
            }
            let f = found.pop();
            let r0 = f + pattern.size;
            let n = r - r0;
            let w0 = w - n;
            unsafe { ptr::copy_nonoverlapping(p.add(r0), p.add(w0), n) }
            w = w0 - value.size;
            unsafe { ptr::copy_nonoverlapping(v, p.add(w), value.size) }
            r = f;
        }
        self.size = new_size;
    }

    fn find_all_stack(&mut self, pattern: &String) -> Stack<usize> {
        let mut s = Stack::<usize>::new();
        for i in self.find(pattern) {
            s.push(i);
        }
        return s;
    }

    fn find_all_queue(&self, pattern: &'a String) -> Queue<usize> {
        let mut q = Queue::<usize>::new();
        for i in self.find(pattern) {
            q.push(i);
        }
        return q;
    }

    fn narrow(&mut self) {
        let border = self.capacity / 2;
        if self.size >= border {
            return;
        }
        let old_layout = Layout::array::<u8>(self.capacity).unwrap();
        let new_layout = Layout::array::<u8>(border).unwrap();
        let p = unsafe {
            alloc::realloc(self.head.as_ptr(), old_layout, new_layout.size())
        };
        self.head = match NonNull::<u8>::new(p) {
            None => handle_alloc_error(new_layout),
            Some(v) => v,
        };
        self.capacity = border;
    }

    fn expand_for_size(&mut self, size: usize) {
        assert!(
            size >= self.size,
            "expect: `size` is not less than current size"
        );
        if size <= self.capacity {
            return;
        }
        let new_capacity = size;
        let old_layout = Layout::array::<u8>(self.capacity).unwrap();
        let new_layout = Layout::array::<u8>(new_capacity).unwrap();
        let p = unsafe {
            alloc::realloc(self.head.as_ptr(), old_layout, new_layout.size())
        };
        self.head = match NonNull::<u8>::new(p) {
            None => handle_alloc_error(new_layout),
            Some(v) => v,
        };
        self.capacity = new_capacity;
    }
}

impl From<&str> for String {
    /// * Create a new instance from primitive string.
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    fn from(str: &str) -> Self {
        assert!(str.is_ascii(), "expect: ASCII string");
        let mut s = String::new();
        s.recapacity_by_size(str.len());
        unsafe {
            ptr::copy_nonoverlapping(str.as_ptr(), s.head.as_ptr(), str.len())
        }
        s.size = str.len();
        return s;
    }
}

impl Clone for String {
    fn clone(&self) -> Self {
        let mut s = String::new();
        s.recapacity_by_size(self.size);
        unsafe {
            ptr::copy_nonoverlapping(
                self.head.as_ptr(),
                s.head.as_ptr(),
                self.size,
            );
        }
        s.size = self.size;
        return s;
    }
}

impl Ord for String {
    fn cmp(&self, other: &Self) -> Ordering {
        let n = min(self.size, other.size);
        for i in 0..n {
            let c0 = self.code_at(i);
            let c1 = other.code_at(i);
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
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Eq for String {}

impl PartialEq for String {
    fn eq(&self, other: &Self) -> bool {
        return self.cmp(other) == Ordering::Equal;
    }
}

impl Drop for String {
    fn drop(&mut self) {
        self.clear();
    }
}
