// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt

use crate::arch::boot::tss;
use crate::logger::verbose;
use core::arch::global_asm;
use core::mem::offset_of;

use super::{Thread, ThreadContext, ThreadState};

const ESP_OFF: usize = core::mem::offset_of!(Thread, esp);
const PD_OFF: usize = core::mem::offset_of!(Thread, pagespace);
const CTX_OFF: usize = core::mem::offset_of!(Thread, context);

const CTX_EDI: usize = offset_of!(ThreadContext, edi);
const CTX_ESI: usize = offset_of!(ThreadContext, esi);
const CTX_EBX: usize = offset_of!(ThreadContext, ebx);
const CTX_EBP: usize = offset_of!(ThreadContext, ebp);
const CTX_EFLAGS: usize = offset_of!(ThreadContext, eflags);
const CTX_EIP: usize = offset_of!(ThreadContext, eip);
const KERNEL_CS: u32 = 0x08;

global_asm!(include_str!("switch.S"), options(att_syntax));
unsafe extern "C" {
    fn switch_to(old: *mut Thread, new: *mut Thread);
    fn enter_to_user(esp: u32) -> !;
}

const KERNEL_DS: u32 = 0x10;

pub unsafe fn do_(old: *mut Thread, new: *mut Thread) {
    unsafe {
        let old_esp = if old.is_null() { 0 } else { (*old).esp as u32 };
        let new_esp = (*new).esp as u32;
        tss::set_stack((*new).kstack_top);
        enter_to_user((*new).esp as u32);
        switch_to(old, new);
    }
}

pub unsafe fn do_first(new: *mut Thread) -> ! {
    unsafe {
        (*new).state = ThreadState::Running;
        tss::set_stack((*new).kstack_top);
        core::arch::asm!(
            "mov cr3, {pd}",
            pd = in(reg) (*new).pagespace,
            options(nostack, preserves_flags),
        );
        enter_to_user((*new).esp as u32)
    }
}
