// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt

use crate::arch::boot::tss;
use crate::logger::debug;
use core::arch::{asm, global_asm};
use core::mem::offset_of;

use super::{Thread, ThreadContext};

const ESP_OFF: usize = core::mem::offset_of!(Thread, esp);
const PD_OFF: usize = core::mem::offset_of!(Thread, pd_addr);
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
    fn switch_to(pd: u32, ctx: *mut ThreadContext, esp: u32) -> !;
}

const KERNEL_DS: u32 = 0x10;

pub unsafe fn do_(new: *mut Thread) -> ! {
    let pd = unsafe { (*new).pd_addr };
    let ctx = unsafe { (*new).context };
    let esp = unsafe { (*new).esp } as u32;
    tss::set_stack(unsafe { (*new).info.stack_top });
    unsafe { switch_to(pd, ctx, esp) }
}
