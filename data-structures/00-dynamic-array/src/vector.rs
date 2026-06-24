#![feature(layout_for_ptr)]

use std::{
    alloc::{Layout, alloc, dealloc, handle_alloc_error, realloc},
    ops::Index,
    ptr,
};

pub struct Vector<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        Self {
            ptr: ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }

    pub fn push(&mut self, val: T) {
        if self.len == self.cap {
            self.grow();
        }

        unsafe {
            self.ptr.add(self.len).write(val);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;

        unsafe { Some(self.ptr.add(self.len).read()) }
    }

    pub fn insert(&mut self, idx: usize, val: T) {
        if idx > self.len {
            panic!(
                "insert index out of bounds: the len is {} but the index is {}",
                self.len, idx
            );
        }

        if self.len == self.cap {
            self.grow();
        }

        unsafe {
            let src = self.ptr.add(idx);
            let dst = self.ptr.add(idx + 1);
            let count = self.len - idx;

            ptr::copy(src, dst, count);
            self.ptr.add(idx).write(val);
        }

        self.len += 1;
    }

    pub fn erase(&mut self, idx: usize) -> Option<T> {
        if idx >= self.len {
            return None;
        }

        unsafe {
            let value = self.ptr.add(idx).read();

            let src = self.ptr.add(idx + 1);
            let dst = self.ptr.add(idx);
            let count = self.len - idx - 1;

            ptr::copy(src, dst, count);
            self.len -= 1;

            Some(value)
        }
    }

    pub fn reverse(&mut self) {
        unsafe {
            let mut i = 0;
            let mut j = self.len;

            while i < j {
                j -= 1;
                ptr::swap(self.ptr.add(i), self.ptr.add(j));
                i += 1;
            }
        }
    }

    pub unsafe fn resize(&mut self, new_len: usize)
    where
        T: Default,
    {
        unsafe {
            while self.len > new_len {
                self.len -= 1;
                ptr::drop_in_place(self.ptr.add(self.len));
            }

            while self.len < new_len {
                self.push(T::default());
            }
        }
    }

    fn grow(&mut self) {
        assert!(
            std::mem::size_of::<T>() != 0,
            "ZST is not supported in this Vector yet"
        );

        let new_cap = if self.cap == 0 {
            4
        } else {
            self.cap.checked_mul(2).expect("capacity overflow")
        };

        let new_layout = Layout::array::<T>(new_cap).unwrap();

        let new_ptr = unsafe {
            if self.cap == 0 {
                alloc(new_layout)
            } else {
                let old_layout = Layout::array::<T>(self.cap).unwrap();
                realloc(self.ptr as *mut u8, old_layout, new_layout.size())
            }
        } as *mut T;

        if new_ptr.is_null() {
            handle_alloc_error(new_layout);
        }

        self.ptr = new_ptr;
        self.cap = new_cap;
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, key: usize) -> &Self::Output {
        if key >= self.len {
            panic!(
                "index out of bounds: the len is {} but the index is {}",
                self.len, key
            );
        }

        unsafe { &*self.ptr.add(key) }
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.len {
                ptr::drop_in_place(self.ptr.add(i));
            }

            if self.cap != 0 && std::mem::size_of::<T>() != 0 {
                let layout = Layout::array::<T>(self.cap).unwrap();
                dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}
