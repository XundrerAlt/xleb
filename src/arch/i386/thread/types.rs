// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use crate::cap::CNode;
use core::mem::offset_of;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadState {
    Ready = 0,
    Running = 1,
    Dead = 2,
}

const _: () = {
    assert!(core::mem::size_of::<ThreadState>() == 4);
    assert!(core::mem::align_of::<ThreadState>() == 4);
};

#[repr(C)]
#[derive(Debug)]
pub struct ThreadContext {
    pub edi: u32,
    pub esi: u32,
    pub ebx: u32,
    pub ebp: u32,
    pub eflags: u32,
    pub eip: u32,
}

const _: () = {
    assert!(core::mem::size_of::<ThreadContext>() == 24);
    assert!(core::mem::offset_of!(ThreadContext, edi) == 0);
    assert!(core::mem::offset_of!(ThreadContext, esi) == 4);
    assert!(core::mem::offset_of!(ThreadContext, ebx) == 8);
    assert!(core::mem::offset_of!(ThreadContext, ebp) == 12);
    assert!(core::mem::offset_of!(ThreadContext, eflags) == 16);
    assert!(core::mem::offset_of!(ThreadContext, eip) == 20);
};

#[repr(C)]
#[derive(Debug)]
pub struct Thread {
    // what switch_to used are primary
    pub context: ThreadContext, // all regs
    pub pagespace: u32, // page directory
    pub esp: *mut u32, // stack

    // everything else are secondary
    pub tid: u32,
    pub state: ThreadState,
    pub kstack_top: u32,
    pub stack_limit: *mut u8,
    pub next: *mut Thread,
    pub prev: *mut Thread,
    pub cspace: CNode,
}

const _: () = {
    assert!(offset_of!(Thread, context) == 0);
    assert!(offset_of!(Thread, pagespace) == 24);
    assert!(offset_of!(Thread, esp) == 28);
    assert!(offset_of!(Thread, kstack_top) == 40);
};
