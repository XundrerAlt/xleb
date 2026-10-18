// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use xleb_api::{ThreadInfo, ThreadState};
use crate::logger::debug;
use crate::arch::cpu;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ThreadContext {
    pub edi: u32,
    pub esi: u32,
    pub ebx: u32,
    pub ebp: u32,
    pub eip: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Thread {
    pub info: ThreadInfo,
    pub pd_addr: u32,
    pub esp: *mut u32,
    pub context: *mut ThreadContext,
    pub stack_limit: *mut u8,
    pub next: *mut Thread,
    pub prev: *mut Thread,
}

static mut ROOT_THREAD: Thread = Thread {
    info: ThreadInfo {
        tid: 0,
        state: ThreadState::Ready,
        stack_top: 0,
        stack_size: 0,
        entry: 0,
    },
    pd_addr: 0,
    esp: core::ptr::null_mut(),
    context: core::ptr::null_mut(),
    stack_limit: core::ptr::null_mut(),
    next: core::ptr::null_mut(),
    prev: core::ptr::null_mut(),
};

const ROOT_STACK_SIZE: usize = 16 * 1024;

#[repr(C, align(16))]
struct RootStack([u8; ROOT_STACK_SIZE]);

static mut ROOT_STACK: RootStack = RootStack([0; ROOT_STACK_SIZE]);

static mut ROOT_CTX: ThreadContext = ThreadContext {
    edi: 0,
    esi: 0,
    ebx: 0,
    ebp: 0,
    eip: 0,
};

pub(crate) unsafe fn create_root(entry: extern "C" fn()) -> &'static mut Thread {
    let th = unsafe { &mut *core::ptr::addr_of_mut!(ROOT_THREAD) };
    let stack = unsafe { &mut *core::ptr::addr_of_mut!(ROOT_STACK) };
    let ctx = unsafe { &mut *core::ptr::addr_of_mut!(ROOT_CTX) };

    let stack_base = stack.0.as_mut_ptr() as u32;
    let stack_top = stack_base + ROOT_STACK_SIZE as u32;

    ctx.eip = entry as u32;
    let saved_esp = stack_top;

    th.info = ThreadInfo {
        tid: 0,
        state: ThreadState::Ready,
        stack_top,
        stack_size: ROOT_STACK_SIZE as u32,
        entry: entry as u32,
    };

    th.pd_addr = unsafe { cpu::read_cr3() };
    th.esp = saved_esp as *mut u32;
    th.context = ctx as *mut ThreadContext;
    th.stack_limit = stack_base as *mut u8;
    debug!("Root thread initialized");

    th
}
