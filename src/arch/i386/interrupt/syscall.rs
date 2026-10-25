// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use crate::logger::{verbose,warning};
use crate::sched::CURRENT_THREAD;
use core::sync::atomic::Ordering;
use xleb_api::CapType;
use xleb_api::SyscallErrno::{self, *};
use xleb_api::SyscallNum::{self, *};
use crate::cap::{Cap,CAP_ALL,CAP_WRITE};

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

fn sys_cap_retype(frame: &mut SyscallFrame) -> i32 {
    let untyped_idx = frame.ebx as usize;
    let type_ = frame.ecx;
    let cap_type = match CapType::try_from(type_) {
        Ok(t) => t,
        Err(_) => {
            verbose!("sys_cap_retype: invalid type_={}", type_);
            return EINVAL.into();
        }
    };

    let size = match cap_type {
        CapType::Page => 4096,
        CapType::CNode | CapType::Untyped => frame.edx as usize,
        CapType::None => {
            verbose!("sys_cap_retype: None is invalid");
            return EINVAL.into();
        }
    };

    let current = CURRENT_THREAD.load(Ordering::Acquire);
    if current.is_null() { return EPERM.into(); }
    let cspace = unsafe { &mut (*current).cspace };
    let untyped = match cspace.get(untyped_idx) {
        Some(cap) if cap.type_ == CapType::Untyped && cap.has(CAP_WRITE) => *cap,
        _ => {
            verbose!("sys_cap_retype: untyped_idx={} isn't untyped/CAP_WRITE", untyped_idx);
            return EINVAL.into();
        }
    };

    if size == 0 || size > untyped.size {
        verbose!("sys_cap_retype: size={} > untyped.size={}", size, untyped.size);
        return EINVAL.into();
    }

    let cap_type = match CapType::try_from(type_) {
        Ok(t) if t != CapType::None && t != CapType::Untyped => t,
        _ => {
            verbose!("sys_cap_retype: invalid type_={}", type_);
            return EINVAL.into();
        }
    };

    let new_cap = Cap::new(cap_type, untyped.object, size, untyped.rights);
    let new_idx = match cspace.insert(None, new_cap) {
        Some(idx) => idx,
        None => {
            warning!("sys_cap_retype: CNode full (no free slot)");
            return ENOMEM.into();
        }
    };

    if let Some(untyped_mut) = cspace.get_mut(untyped_idx) {
        untyped_mut.size -= size;
    }

    verbose!("sys_cap_retype: untyped[{}] -> {:?} idx={} size={}",
           untyped_idx, cap_type, new_idx, size);

    new_idx as i32
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
        Ok(CapRetype) => sys_cap_retype(f),
        _ => {
            verbose!("Unknown syscall");
            ENOSYS.into()
        }
    };

    f.eax = ret as u32;
}
