// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt

use crate::arch::thread::{self, Thread};
use crate::logger::{debug, verbose};
use core::sync::atomic::{AtomicPtr, Ordering};
use xleb_api::ThreadState;

mod queue;
use queue::ThreadQueue;
use spin::Mutex;

pub static RUN_QUEUE: Mutex<ThreadQueue> = Mutex::new(ThreadQueue::new());
pub static CURRENT_THREAD: AtomicPtr<Thread> = AtomicPtr::new(core::ptr::null_mut());

pub fn current() -> *mut Thread {
    CURRENT_THREAD.load(Ordering::Acquire)
}

pub(crate) unsafe fn set_current(t: *mut Thread) {
    CURRENT_THREAD.store(t, Ordering::Release);
}

#[repr(align(4096))]
struct IdleBin([u8; 16]);
static IDLE_BIN: IdleBin = IdleBin([0xEB, 0xFE, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

pub(crate) unsafe fn init() {
    let th = unsafe { thread::create_root(IDLE_BIN.0.as_ptr(), IDLE_BIN.0.len()) };
    RUN_QUEUE.lock().push(th);
    unsafe { set_current(th) };
    debug!("scheduler: hello world");
    unsafe { thread::switch::do_(th) };
}

pub(crate) unsafe fn yield_now() {
    let current = current();
    crate::logger::verbose!("sched: yield current={:p}", current);

    let next = {
        let mut queue = RUN_QUEUE.lock();

        if queue.is_empty() {
            if !current.is_null() {
                unsafe { (*current).info.state = ThreadState::Running };
            }
            verbose!("sched: empty, no switch");
            return;
        }

        if !current.is_null() {
            unsafe { (*current).info.state = ThreadState::Ready };
            queue.push(current);
            verbose!("sched: push current tid={}", unsafe { (*current).info.tid });
        }

        let n = queue.pop().unwrap_or(current);
        verbose!(
            "sched: pop {:p} tid={}",
            n,
            if n.is_null() {
                0
            } else {
                unsafe { (*n).info.tid }
            }
        );
        n
    };

    if next != current {
        if !current.is_null() {
            unsafe { (*current).info.state = ThreadState::Ready };
        }
        unsafe { (*next).info.state = ThreadState::Running };
        crate::logger::verbose!(
            "sched: switch {:p} (tid={}) -> {:p} (tid={})",
            current,
            if current.is_null() {
                0
            } else {
                unsafe { (*current).info.tid }
            },
            next,
            unsafe { (*next).info.tid },
        );
        unsafe {
            set_current(next);
            thread::switch::do_(next);
        }
    } else if !current.is_null() {
        unsafe { (*current).info.state = ThreadState::Running };
        crate::logger::verbose!("sched: next == current, no switch");
    }
}
