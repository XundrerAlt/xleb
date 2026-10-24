// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt

use crate::arch::thread::Thread;
use crate::logger::verbose;

pub struct ThreadQueue {
    head: *mut Thread,
    tail: *mut Thread,
    len: usize,
}

impl ThreadQueue {
    pub const fn new() -> Self {
        Self {
            head: core::ptr::null_mut(),
            tail: core::ptr::null_mut(),
            len: 0,
        }
    }

    pub fn push(&mut self, t: *mut Thread) -> bool {
        unsafe {
            (*t).next = core::ptr::null_mut();
            (*t).prev = self.tail;
            if self.tail.is_null() {
                self.head = t;
            } else {
                (*self.tail).next = t;
            }
            self.tail = t;
        }
        self.len += 1;
        true
    }

    pub fn pop(&mut self) -> Option<*mut Thread> {
        if self.head.is_null() {
            verbose!("queue: pop empty");
            return None;
        }
        let t = self.head;
        unsafe {
            self.head = (*t).next;
            if self.head.is_null() {
                self.tail = core::ptr::null_mut();
            } else {
                (*self.head).prev = core::ptr::null_mut();
            }
            (*t).next = core::ptr::null_mut();
            (*t).prev = core::ptr::null_mut();
        }
        self.len -= 1;
        Some(t)
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }
    pub fn len(&self) -> usize {
        self.len
    }
}

// SAFETY: ThreadQueue is only accessed through a Mutex (or &mut self).
// Raw pointers inside are to static TCBs, never to stack data, and are
// never dereferenced without the Mutex held.
unsafe impl Send for ThreadQueue {}
unsafe impl Sync for ThreadQueue {}
