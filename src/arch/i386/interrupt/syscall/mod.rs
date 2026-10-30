// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use crate::arch::mm::PAGE_SIZE;
use crate::logger::verbose;
use xleb_api::SyscallErrno::*;
use xleb_api::SyscallNum::{self, *};

mod cap_find;
mod cap_grant;
mod cap_retype;
mod frame_map;
mod thread_conf;
mod thread_start;
mod thread_suspend;
mod x86_derive_portio;
mod x86_portio;

pub(crate) const MAX_FRAME_SIZE: usize = 1024 * PAGE_SIZE;

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
pub unsafe extern "C" fn syscall_handler(frame: &mut SyscallFrame) {
    let f = unsafe { &mut *frame };

    let nr = f.eax;
    let ret: i32 = match SyscallNum::try_from(nr) {
        Ok(ThreadSuspend) => thread_suspend::sys(f),
        Ok(ThreadConf) => thread_conf::sys(f),
        Ok(ThreadStart) => thread_start::sys(f),
        Ok(CapFind) => cap_find::sys(f),
        Ok(CapRetype) => cap_retype::sys(f),
        Ok(CapGrant) => cap_grant::sys(f),
        Ok(FrameMap) => frame_map::sys(f),
        Ok(x86_PortIo) => x86_portio::sys(f),
        Ok(x86_Derive_PortIo) => x86_derive_portio::sys(f),
        _ => {
            verbose!("Unknown syscall");
            ENOSYS.into()
        }
    };

    f.eax = ret as u32;
}
