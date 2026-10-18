// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt

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

global_asm!(include_str!("do_first.S"), options(att_syntax));
unsafe extern "C" {
    fn switch_to_first(pd: u32, ctx: *mut ThreadContext, esp: u32) -> !;
}

pub unsafe fn do_(old: *mut Thread, new: *mut Thread) {
    debug_assert!(!old.is_null());
    debug_assert!(!new.is_null());
    debug_assert_ne!(old, new);

    unsafe {
        asm!(
            "mov [{old} + {esp_off}], esp",

            "pushfd",
            "push ebp",
            "push ebx",
            "push esi",
            "push edi",

            "mov [{old} + {ctx_off}], esp",

            "mov eax, [{new} + {pd_off}]",
            "mov cr3, eax",

            "mov esp, [{new} + {ctx_off}]",

            "pop edi",
            "pop esi",
            "pop ebx",
            "pop ebp",
            "popfd",

            "mov esp, [{new} + {esp_off}]",

            "ret",

            old = in(reg) old,
            new = in(reg) new,
            esp_off = const ESP_OFF,
            pd_off  = const PD_OFF,
            ctx_off = const CTX_OFF,
            options(noreturn),
        );
    }
}

const KERNEL_DS: u32 = 0x10;

pub unsafe fn do_first(new: *mut Thread) -> ! {
    let pd = unsafe { (*new).pd_addr };
    let ctx = unsafe { (*new).context };
    let esp = unsafe { (*new).esp } as u32;

    unsafe { switch_to_first(pd, ctx, esp) }
}
