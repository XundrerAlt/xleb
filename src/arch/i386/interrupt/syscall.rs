// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use crate::logger::verbose;
use crate::sched::CURRENT_THREAD;
use core::sync::atomic::Ordering;
use xleb_api::CapType;
use xleb_api::SyscallErrno::*;
use xleb_api::SyscallNum;
use xleb_api::SyscallNum::*;

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

fn sys_cap_find(frame: &mut SyscallFrame) -> i32 {
    let type_ = frame.ebx;
    let object = frame.ecx as usize;
    let rights = frame.edx;

    let current = CURRENT_THREAD.load(Ordering::Acquire);
    if current.is_null() {
        return -1;
    }

    let cap_type = match CapType::try_from(type_) {
        Ok(t) => t,
        Err(_) => return EINVAL.into(),
    };

    let cspace = unsafe { &(*current).cspace };
    match cspace.find(cap_type, object, rights) {
        Some(idx) => idx as i32,
        None => ENOENT.into(),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_handler(frame: &mut SyscallFrame) {
    let f = unsafe { &mut *frame };

    let nr = f.eax;
    let ret: i32 = match SyscallNum::try_from(nr) {
        Ok(Test) => {
            verbose!("SYSCALL!");
            0
        }
        Ok(CapFind) => sys_cap_find(f),
        _ => {
            verbose!("Unknown syscall");
            ENOSYS.into()
        }
    };

    f.eax = ret as u32;
}
