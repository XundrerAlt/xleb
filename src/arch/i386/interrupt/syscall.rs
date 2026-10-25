// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use crate::arch::mm::temp_window::TempWindow;
use crate::arch::mm::{PAGE_PRESENT, PAGE_USER, PAGE_WRITE};
use crate::arch::thread::VSPACE_ADDR;
use crate::cap::{CAP_ALL, CAP_READ, CAP_WRITE, Cap};
use crate::logger::{verbose, warning};
use crate::sched::CURRENT_THREAD;
use core::sync::atomic::Ordering;
use xleb_api::CapType;
use xleb_api::SyscallErrno::{self, *};
use xleb_api::SyscallNum::{self, *};

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
        CapType::VNode => 4096,
        CapType::CNode | CapType::Untyped => frame.edx as usize,
        CapType::None => {
            verbose!("sys_cap_retype: None is invalid");
            return EINVAL.into();
        }
    };

    let current = CURRENT_THREAD.load(Ordering::Acquire);
    if current.is_null() {
        return EPERM.into();
    }
    let cspace = unsafe { &mut (*current).cspace };
    let untyped = match cspace.get(untyped_idx) {
        Some(cap) if cap.type_ == CapType::Untyped && cap.has(CAP_WRITE) => *cap,
        _ => {
            verbose!(
                "sys_cap_retype: untyped_idx={} isn't untyped/CAP_WRITE",
                untyped_idx
            );
            return EINVAL.into();
        }
    };

    if size == 0 || size > untyped.size {
        verbose!(
            "sys_cap_retype: size={} > untyped.size={}",
            size,
            untyped.size
        );
        return EINVAL.into();
    }

    let cap_type = match CapType::try_from(type_) {
        Ok(t) if t != CapType::None => t,
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
        untyped_mut.object += size;
    }

    verbose!(
        "sys_cap_retype: untyped[{}] -> {:?} idx={} size={}",
        untyped_idx,
        cap_type,
        new_idx,
        size
    );

    new_idx as i32
}

fn sys_cap_map(frame: &mut SyscallFrame) -> i32 {
    let vnode_idx = frame.ebx as usize;
    let frame_idx = frame.ecx as usize;
    let vaddr = frame.edx;

    let current = CURRENT_THREAD.load(Ordering::Acquire);
    if current.is_null() { return EPERM.into(); }
    let cspace = unsafe { &mut (*current).cspace };

    let vnode_cap = match cspace.get(vnode_idx) {
        Some(cap) if cap.type_ == CapType::VNode && cap.has(CAP_WRITE) => *cap,
        _ => { verbose!("sys_cap_map: vnode_idx={} isn't PT/CAP_WRITE", vnode_idx); return EINVAL.into(); }
    };
    let frame_cap = match cspace.get(frame_idx) {
        Some(cap) if cap.type_ == CapType::Page && cap.has(CAP_READ) => *cap,
        _ => { verbose!("sys_cap_map: frame_idx={} isn't Page/CAP_READ", frame_idx); return EINVAL.into(); }
    };

    if vaddr & 0xFFF != 0 { verbose!("sys_cap_map: vaddr={:#x} not page-aligned", vaddr); return EINVAL.into(); }

    let vnode_phys = vnode_cap.object as u32;
    let frame_phys = frame_cap.object as u32;
    let pd_index = (vaddr >> 22) as usize;
    let pt_index = ((vaddr >> 12) & 0x3FF) as usize;

    let flags = PAGE_PRESENT | PAGE_USER | (if frame_cap.has(CAP_WRITE) { PAGE_WRITE } else { 0 });

    unsafe {
        let mut win = TempWindow::map(vnode_phys, 4096);
        let pt = win.as_mut_ptr() as *mut u32;
        *pt.add(pt_index) = frame_phys | flags;

        let pd = VSPACE_ADDR as *mut u32;
        *pd.add(pd_index) = vnode_phys | PAGE_PRESENT | PAGE_WRITE | PAGE_USER;
    }

    verbose!("sys_cap_map: frame={:#x} -> vnode={:#x} vaddr={:#x}", frame_phys, vnode_phys, vaddr);
    vaddr as i32
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
        Ok(CapMap) => sys_cap_map(f),
        _ => {
            verbose!("Unknown syscall");
            ENOSYS.into()
        }
    };

    f.eax = ret as u32;
}
