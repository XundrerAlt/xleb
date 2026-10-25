// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use crate::arch::cpu;
use crate::arch::mm::temp_window::TempWindow;
use crate::arch::mm::{PAGE_PRESENT, PAGE_USER, PAGE_WRITE};
use crate::arch::thread::{Thread,VSPACE_ADDR};
use crate::cap::{CAP_ALL, CAP_READ, CAP_WRITE, Cap};
use crate::logger::{verbose, warning};
use crate::sched::CURRENT_THREAD;
use core::sync::atomic::Ordering;
use xleb_api::CapType;
use xleb_api::SyscallErrno::{self, *};
use xleb_api::SyscallNum::{self, *};

pub(crate) const MAX_FRAME_SIZE: usize = 1024 * 4096;

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

fn align_up(x: usize, a: usize) -> usize {
    (x + a - 1) & !(a - 1)
}

fn next_power_of_two(mut x: usize) -> usize {
    if x <= 1 { return 1; }
    x -= 1;
    x |= x >> 1; x |= x >> 2; x |= x >> 4; x |= x >> 8; x |= x >> 16;
    #[cfg(target_pointer_width = "64")]
    { x |= x >> 32; }
    x + 1
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
        CapType::Frame => {
            let s = frame.edx as usize;
            if s == 0 || s % 4096 != 0 {
                verbose!("sys_cap_retype: Frame size={} not multiple of 4096", s);
                return EINVAL.into();
            }
            if s > MAX_FRAME_SIZE {
                verbose!("sys_cap_retype: Frame size={} > MAX_FRAME_SIZE={}", s, MAX_FRAME_SIZE);
                return EINVAL.into();
            }
            s
        }
        CapType::VNode => 4096,
        CapType::CNode | CapType::Untyped => frame.edx as usize,
        CapType::Thread => core::mem::size_of::<Thread>(),
        CapType::None => {
            verbose!("sys_cap_retype: None is invalid");
            return EINVAL.into();
        }
    };

    let align = match cap_type {
        CapType::Frame => 4096,
        CapType::VNode => 4096,
        CapType::Thread => next_power_of_two(core::mem::align_of::<Thread>()),
        CapType::CNode | CapType::Untyped => size,
        CapType::None => unreachable!(),
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

    let obj = align_up(untyped.object, align);
    let consumed = obj + size - untyped.object;
    if consumed > untyped.size {
        verbose!(
            "sys_cap_retype: no room (need {} bytes after align, have {})",
            consumed,
            untyped.size
        );
        return ENOMEM.into();
    }

    let new_cap = Cap::new(cap_type, obj, size, untyped.rights);
    let new_idx = match cspace.insert(None, new_cap) {
        Some(idx) => idx,
        None => {
            warning!("sys_cap_retype: CNode full (no free slot)");
            return ENOMEM.into();
        }
    };

    if let Some(untyped_mut) = cspace.get_mut(untyped_idx) {
        untyped_mut.object += consumed;
        untyped_mut.size -= consumed;
    }

    if matches!(cap_type, CapType::Frame | CapType::VNode) {
        unsafe {
            let mut win = TempWindow::map(obj as u32, 4096);
            core::ptr::write_bytes(win.as_mut_ptr(), 0, 4096);
        }
    }

    verbose!(
        "sys_cap_retype: untyped[{}] -> {:?} idx={} obj={:#x} size={} (consumed={})",
        untyped_idx,
        cap_type,
        new_idx,
        obj,
        size,
        consumed
    );

    new_idx as i32
}

fn sys_frame_map(frame: &mut SyscallFrame) -> i32 {
    let vnode_idx = frame.ebx as usize;
    let frame_idx = frame.ecx as usize;
    let vaddr = frame.edx;

    let current = CURRENT_THREAD.load(Ordering::Acquire);
    if current.is_null() {
        return EPERM.into();
    }
    let cspace = unsafe { &mut (*current).cspace };

    let vnode_cap = match cspace.get(vnode_idx) {
        Some(cap) if cap.type_ == CapType::VNode && cap.has(CAP_WRITE) => *cap,
        _ => {
            verbose!("sys_frame_map: vnode_idx={} isn't PT/CAP_WRITE", vnode_idx);
            return EINVAL.into();
        }
    };
    let frame_cap = match cspace.get(frame_idx) {
        Some(cap) if cap.type_ == CapType::Frame && cap.has(CAP_READ) => *cap,
        _ => {
            verbose!("sys_frame_map: frame_idx={} isn't Frame/CAP_READ", frame_idx);
            return EINVAL.into();
        }
    };

    if frame_cap.size > MAX_FRAME_SIZE {
        verbose!("sys_frame_map: frame size={} > MAX_FRAME_SIZE", frame_cap.size);
        return EINVAL.into();
    }

    if vaddr & 0xFFF != 0 {
        verbose!("sys_frame_map: vaddr={:#x} not page-aligned", vaddr);
        return EINVAL.into();
    }

    let vnode_phys = vnode_cap.object as u32;
    let frame_phys = frame_cap.object as u32;

    if frame_phys & 0xFFF != 0 {
        verbose!("sys_frame_map: frame={:#x} not page-aligned", frame_phys);
        return EINVAL.into();
    }
    if vnode_phys & 0xFFF != 0 {
        verbose!("sys_frame_map: vnode={:#x} not page-aligned", vnode_phys);
        return EINVAL.into();
    }

    let pd_index = (vaddr >> 22) as usize;
    let pt_index = ((vaddr >> 12) & 0x3FF) as usize;

    let pages = frame_cap.size / 4096;
    if pages == 0 {
        verbose!("sys_frame_map: frame size={} < 4096", frame_cap.size);
        return EINVAL.into();
    }

    if pt_index + pages > 1024 {
        verbose!(
            "sys_frame_map: {} pages at pt_index={} cross PT boundary",
            pages,
            pt_index
        );
        return ENOMEM.into();
    }

    let end_vaddr = vaddr as u64 + (pages as u64) * 4096;
    if (end_vaddr - 1) >> 22 != (vaddr >> 22) as u64 {
        verbose!("sys_frame_map: frame crosses PD boundary");
        return ENOMEM.into();
    }

    let flags = PAGE_PRESENT
        | PAGE_USER
        | (if frame_cap.has(CAP_WRITE) {
            PAGE_WRITE
        } else {
            0
        });

    unsafe {
        let mut win = TempWindow::map(vnode_phys, 4096);
        let pt = win.as_mut_ptr() as *mut u32;
        for i in 0..pages {
            *pt.add(pt_index + i) = (frame_phys + (i as u32) * 4096) | flags;
        }

        let pd = VSPACE_ADDR as *mut u32;
        *pd.add(pd_index) = vnode_phys | PAGE_PRESENT | PAGE_WRITE | PAGE_USER;

        for i in 0..pages {
            cpu::invlpg(vaddr + (i as u32) * 4096);
        }
    }

    verbose!(
        "sys_frame_map: frame={:#x} size={} pages={} -> vnode={:#x} vaddr={:#x}",
        frame_phys,
        frame_cap.size,
        pages,
        vnode_phys,
        vaddr
    );
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
        Ok(FrameMap) => sys_frame_map(f),
        _ => {
            verbose!("Unknown syscall");
            ENOSYS.into()
        }
    };

    f.eax = ret as u32;
}
