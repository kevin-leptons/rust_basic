use super::String;
use std::alloc::{self, handle_alloc_error, Layout};

/// Knuth–Morris–Pratt Table for finding matched sub strings.
pub(super) struct KmpTable<'a> {
    /// This is an array,represents for the table. The index is based on 1 and
    /// `index - 1` represents for potition in the `pattern`. The first and last
    /// index is existed but does not use. The value represents for next finding
    /// candidate that is index of the table too.
    head: *mut usize,

    /// The table is built from this pattern.
    pattern: &'a String,

    /// Size of `head` in quantity of usize.
    size: usize,
}

impl<'a> KmpTable<'a> {
    /// Time complexity: O(n).
    ///
    /// Space complexity: O(n).
    pub fn new(pattern: &'a String) -> Self {
        assert!(pattern.size > 0, "expect: not empty pattern");
        let size = pattern.size + 2;
        let head = Self::alloc(size);
        let mut table = Self {
            pattern,
            head,
            size,
        };
        table.set(1, 0);
        let mut i = 2usize;
        let mut c = 1usize;
        loop {
            if i == pattern.size {
                break;
            }
            if pattern.get_code(i) == pattern.get_code(c) {
                table.set(i, table.get(c));
            } else {
                table.set(i, c);
                loop {
                    if c < 1 {
                        break;
                    }
                    if pattern.get_code(i) == pattern.get_code(c) {
                        break;
                    }
                    c = table.get(c);
                }
            }
            i += 1;
            c += 1;
        }
        table.set(i, c);
        return table;
    }

    /// Get index of the first matched sub string.
    pub fn find(&self, target: &String, from: usize) -> Option<usize> {
        let mut t = from;
        let mut p = 0;
        loop {
            if t >= target.size {
                return None;
            }
            if self.pattern.get_code(p) == target.get_code(t) {
                t += 1;
                p += 1;
                if p == self.pattern.size {
                    return Some(t - p);
                }
            } else {
                match self.next_index(p) {
                    None => {
                        t += 1;
                        p = 0;
                    }
                    Some(v) => p = v,
                };
            }
        }
    }

    /// Next index for comparing from current index.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(n).
    fn next_index(&self, index: usize) -> Option<usize> {
        assert!(
            index < self.size - 1,
            "expect: `index` is less than size - 1"
        );
        unsafe {
            let value = *self.head.add(index + 1);
            if value < 1 {
                return None;
            }
            return Some(value - 1);
        }
    }

    /// Retrieve a value at `index`, based `1`.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(n).
    fn set(&mut self, index: usize, value: usize) {
        assert!(index < self.size, "expect: `index` is less than size");
        assert!(index > 0, "expect: `index` is greater than zero");
        unsafe {
            *(&mut *self.head.add(index)) = value;
        }
    }

    /// Retrieve a value at `index`, based `1`.
    ///
    /// Time complexity: O(1).
    ///
    /// Space complexity: O(n).
    fn get(&self, index: usize) -> usize {
        assert!(index < self.size, "expect: `index` is less than size");
        assert!(index > 0, "expect: `index` is greater than zero");
        unsafe {
            return *self.head.add(index);
        };
    }

    fn alloc(size: usize) -> *mut usize {
        unsafe {
            let layout = Layout::array::<usize>(size).unwrap();
            assert!(
                layout.size() <= isize::MAX as usize,
                "expect: smaller memory block"
            );
            let pointer = alloc::alloc(layout);
            if pointer.is_null() {
                handle_alloc_error(layout);
            }
            return pointer as *mut usize;
        }
    }
}

impl<'a> Drop for KmpTable<'a> {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::array::<usize>(self.size).unwrap();
            alloc::dealloc(self.head as *mut u8, layout);
        }
    }
}
