use std::{
    alloc::{self, handle_alloc_error, Layout},
    ptr::NonNull,
};

use super::String;

/// For iteration over found sub strings by index of first matched character.
pub struct FindIter<'a> {
    target: &'a String,
    pattern: &'a String,
    table: KmpTable,
    index: usize,
}

impl<'a> FindIter<'a> {
    pub(super) fn new(target: &'a String, pattern: &'a String) -> Self {
        return Self {
            target: target,
            pattern: pattern,
            table: KmpTable::from(pattern),
            index: 0,
        };
    }
}

impl Iterator for FindIter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.target.size {
            return None;
        }
        match kmp_find(self.target, self.pattern, &self.table, self.index) {
            None => {
                self.index = self.target.size;
                return None;
            }
            Some(v) => {
                self.index = v + 1;
                return Some(v);
            }
        }
    }
}

fn kmp_find(
    string: &String,
    pattern: &String,
    table: &KmpTable,
    from: usize,
) -> Option<usize> {
    let mut j = from;
    let mut k = 0;
    loop {
        if j >= string.size {
            return None;
        }
        if pattern.code_at(k) == string.code_at(j) {
            j += 1;
            k += 1;
            if k == pattern.size {
                return Some(j - k);
            }
        } else {
            match table.get(k) {
                0 => {
                    j += 1;
                    k = 0;
                }
                v => k = v,
            };
        }
    }
}

/// Knuth–Morris–Pratt table.
struct KmpTable {
    head: NonNull<usize>,
    size: usize,
}

impl KmpTable {
    /// * Create a new instance, no memory allocation.
    /// * Time complexity: O(1).
    /// * Space complexity: O(1).
    pub fn new() -> Self {
        return Self {
            head: NonNull::dangling(),
            size: 0,
        };
    }

    /// * Build the table from string pattern.
    /// * Time complexity: O(n).
    /// * Space complexity: O(n).
    pub fn from(pattern: &String) -> Self {
        let mut table = Self::new();
        table.alloc(pattern.size + 2);
        table.set_at(1, 0);
        let mut p = 2;
        let mut c = 1;
        loop {
            if p as usize == pattern.size {
                break;
            }
            if pattern.code_at(p as usize) == pattern.code_at(c as usize) {
                table.set_at(p as usize, table.get_at(c as usize));
            } else {
                table.set_at(p as usize, c);
                loop {
                    if (c < 1)
                        || (pattern.code_at(p as usize)
                            == pattern.code_at(c as usize))
                    {
                        break;
                    }
                    c = table.get_at(c as usize);
                }
            }
            p += 1;
            c += 1;
        }
        table.set_at(p as usize, c);
        return table;
    }

    /// * Retrieve next candidate as an index by current index.
    /// * Time complexity: O(1).
    /// * Space complexity: O(n).
    pub fn get(&self, index: usize) -> usize {
        assert!(
            index < self.size - 1,
            "expect: `index` is less than size - 1"
        );
        let v = unsafe { *self.head.as_ptr().add(index + 1) };
        if v < 1 {
            return 0;
        }
        return v - 1;
    }

    /// * Retrieve a value at `index` that based `1`.
    /// * Time complexity: O(1).
    /// * Space complexity: O(n).
    fn set_at(&mut self, index: usize, value: usize) {
        assert!(index < self.size, "expect: `index` is less than size");
        assert!(index > 0, "expect: `index` is greater than zero");
        unsafe { *(&mut *self.head.as_ptr().add(index)) = value };
    }

    /// * Retrieve a value at `index` that based `1`.
    /// * Time complexity: O(1).
    /// * Space complexity: O(n).
    fn get_at(&self, index: usize) -> usize {
        assert!(index < self.size, "expect: `index` is less than size");
        assert!(index > 0, "expect: `index` is greater than zero");
        return unsafe { *self.head.as_ptr().add(index) };
    }

    fn alloc(&mut self, size: usize) {
        let layout = Layout::array::<usize>(size).unwrap();
        let memory = unsafe { alloc::alloc(layout) };
        match NonNull::<usize>::new(memory as *mut usize) {
            None => handle_alloc_error(layout),
            Some(v) => {
                self.head = v;
                self.size = size;
            }
        };
    }
}

impl Drop for KmpTable {
    fn drop(&mut self) {
        if self.size > 0 {
            let layout = Layout::array::<i64>(self.size).unwrap();
            unsafe { alloc::dealloc(self.head.as_ptr() as *mut u8, layout) };
        }
    }
}
