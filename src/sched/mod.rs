// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt

use crate::arch::boot::modules;
use crate::arch::thread::{self, Thread};
use crate::logger::{debug,info,verbose};
use core::sync::atomic::{AtomicPtr, Ordering};
use xleb_api::ThreadState;
use crate::VIRT_TO_PHYS;

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
    let (prog_ptr, prog_size) = match modules::get(0) {
        Some((start, end)) => {
            debug!("root program: module 0 phys {:#x}..{:#x}", start, end);
            (start, (end - start) as usize)
        }
        None => {
            debug!("root program: idle (no modules)");
            let virt = IDLE_BIN.0.as_ptr() as u32;
            (crate::VIRT_TO_PHYS!(virt), IDLE_BIN.0.len())
        }
    };
    let th = unsafe { thread::create_root(prog_ptr, prog_size) };
    RUN_QUEUE.lock().push(th);
    unsafe { set_current(th) };
    info!("scheduler: hello world");
    unsafe { thread::switch::do_(th) };
}

pub(crate) unsafe fn yield_now() {
    let current = current();
    let next = {
        let mut queue = RUN_QUEUE.lock();
        if !current.is_null() {
            unsafe { (*current).state = ThreadState::Ready };
            queue.push(current);
        }
        let n = queue.pop().unwrap_or(current);
        if n.is_null() {
            if !current.is_null() {
                unsafe { (*current).state = ThreadState::Running };
            }
            return;
        }
        n
    };
    if next != current {
        unsafe { set_current(next); thread::switch::do_(next); }
    } else if !current.is_null() {
        unsafe { (*current).state = ThreadState::Running };
    }
}
