// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt

use crate::logger::debug;
use crate::arch::thread;

mod queue;

use queue::ThreadQueue;
use spin::Mutex;

pub static RUN_QUEUE: Mutex<ThreadQueue> = Mutex::new(ThreadQueue::new());

extern "C" fn idle() {
    loop {
        unsafe {
            core::arch::asm!("hlt", options(nomem, nostack, preserves_flags));
        }
    }
}

pub(crate) unsafe fn init() {
    let th = unsafe { thread::create_root(idle) };
    RUN_QUEUE.lock().push(th);
    debug!("scheduler: hello world");
}
