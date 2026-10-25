// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use crate::logger::debug;

#[repr(C)]
pub struct SyscallFrame {
    pub gs: u32,
    pub fs: u32,
    pub es: u32,
    pub ds: u32,

    pub edi: u32,
    pub esi: u32,
    pub ebp: u32,
    pub esp_saved: u32,
    pub ebx: u32,
    pub edx: u32,
    pub ecx: u32,
    pub eax: u32,

    pub ss: u32,
    pub esp: u32,
    pub eflags: u32,
    pub cs: u32,
    pub eip: u32,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_handler(frame: *mut SyscallFrame) {
    if frame.is_null() {
        return;
    }
    let f = unsafe { &mut *frame };

    let nr = f.eax;
    let arg0 = f.ebx;
    let arg1 = f.ecx;
    let arg2 = f.edx;
    let arg3 = f.esi;
    let arg4 = f.edi;

    let ret: i32 = match nr {
        0 => {
            debug!("SYSCALL!");
            0
        }
        _ => -38,
    };
    f.eax = ret as u32;
}
