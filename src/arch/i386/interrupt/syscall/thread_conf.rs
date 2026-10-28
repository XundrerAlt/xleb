// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use crate::PHYS_TO_VIRT;
use crate::arch::cpu;
use crate::arch::mm::{
    PAGE_MASK, PAGE_PRESENT, PAGE_SIZE, PAGE_USER, PAGE_WRITE, PD_SHIFT, PT_ENTRIES, PT_MASK,
    PT_SHIFT, map_range_in_pagespace,
};
use crate::arch::thread::STACK_SIZE;
use crate::arch::thread::{KERNEL_STACK_ADDR, Thread, USER_STACK_ADDR, init_thread};
use crate::cap::{CAP_GRANT, CAP_WRITE, CNode, Cap, commit_untyped, current_cspace, get_cap};
use crate::logger::verbose;
use crate::utils::align_up;
use xleb_api::CapType;
use xleb_api::SyscallErrno::*;

use super::SyscallFrame;

fn validate_vaddr(vaddr: u32, name: &str) -> Result<(), i32> {
    if vaddr & PAGE_MASK as u32 != 0 {
        verbose!("sys_thread_conf: {}={:#x} not page-aligned", name, vaddr);
        return Err(EINVAL.into());
    }
    Ok(())
}

unsafe fn translate_user_vaddr(vaddr: u32) -> Result<u32, i32> {
    unsafe {
        let cr3 = cpu::read_cr3();
        let pd = PHYS_TO_VIRT!(cr3) as *const u32;
        let pd_index = (vaddr >> PD_SHIFT) as usize;
        let pde = *pd.add(pd_index);
        if pde & PAGE_PRESENT == 0 {
            return Err(EINVAL.into());
        }
        if pde & 0x80 != 0 {
            verbose!("sys_thread_conf: Large page isn't supported");
            return Err(EINVAL.into());
        }
        let pt = PHYS_TO_VIRT!(pde & !PAGE_MASK as u32) as *const u32;
        let pt_index = ((vaddr >> PT_SHIFT) & PT_MASK) as usize;
        let pte = *pt.add(pt_index);
        if pte & PAGE_PRESENT == 0 {
            return Err(EINVAL.into());
        }
        Ok(pte & !PAGE_MASK as u32)
    }
}

pub(crate) fn sys(frame: &mut SyscallFrame) -> i32 {
    sys_impl(frame).unwrap_or_else(|e| e)
}

fn sys_impl(frame: &mut SyscallFrame) -> Result<i32, i32> {
    let th_idx = frame.ebx as usize;
    let pagespace_idx = frame.ecx as usize;
    let code_vaddr = frame.edx;
    let code_size = frame.esi;
    let untyped_idx = frame.edi as usize;

    let cspace = current_cspace()?;

    let th = get_cap(
        cspace,
        th_idx,
        CapType::Thread,
        CAP_WRITE,
        "sys_thread_conf",
    )?;
    let ps = get_cap(
        cspace,
        pagespace_idx,
        CapType::PageSpace,
        CAP_WRITE,
        "sys_thread_conf",
    )?;
    let mut untyped = get_cap(
        cspace,
        untyped_idx,
        CapType::Untyped,
        CAP_WRITE | CAP_GRANT,
        "sys_thread_conf",
    )?;

    if untyped.object == 0 || untyped.size == 0 {
        verbose!("sys_thread_conf: untyped empty");
        return Err(EINVAL.into());
    }

    validate_vaddr(code_vaddr, "code")?;
    if code_size == 0 || code_size & PAGE_MASK as u32 != 0 {
        verbose!("sys_thread_conf: code_size={:#x} invalid", code_size);
        return Err(EINVAL.into());
    }

    let kstack_size = align_up(STACK_SIZE, PAGE_SIZE);
    let ustack_size = align_up(STACK_SIZE, PAGE_SIZE);

    let original_object = untyped.object;
    let aligned = align_up(untyped.object, PAGE_SIZE);
    let consumed_align = aligned - untyped.object;
    let need = consumed_align + kstack_size + ustack_size;
    if untyped.size < need {
        verbose!(
            "sys_thread_conf: untyped too small (need {}, have {})",
            need,
            untyped.size
        );
        return Err(ENOMEM.into());
    }
    untyped.object = aligned;
    untyped.size -= consumed_align;

    let kstack_phys = untyped.object as u32;
    let kstack_vaddr = PHYS_TO_VIRT!(kstack_phys);
    let ustack_phys = (untyped.object + kstack_size) as u32;

    let tid: u32 = 1;
    let ustack_vaddr: u32 = USER_STACK_ADDR;
    let kstack_vaddr: u32 = tid * STACK_SIZE as u32 + KERNEL_STACK_ADDR;

    let mut untyped_work = untyped;
    untyped_work.object += kstack_size + ustack_size;
    untyped_work.size -= kstack_size + ustack_size;

    unsafe {
        map_range_in_pagespace(
            ps.object as u32,
            &mut untyped_work,
            ustack_vaddr,
            ustack_phys,
            ustack_size,
            PAGE_PRESENT | PAGE_USER | PAGE_WRITE,
            "sys_thread_conf: ustack",
        )?;
        let pages = code_size as usize / PAGE_SIZE;
        let pt_index = ((code_vaddr >> PT_SHIFT) & PT_MASK) as usize;
        if pt_index + pages > PT_ENTRIES {
            verbose!("sys_thread_conf: code crosses PT boundary");
            return Err(EINVAL.into());
        }
        for i in 0..pages {
            let va = code_vaddr + (i as u32) * PAGE_SIZE as u32;
            let phys = translate_user_vaddr(va).map_err(|_| {
                verbose!(
                    "sys_thread_conf: code_vaddr={:#x} not mapped in current PD",
                    va
                );
                EINVAL
            })?;
            map_range_in_pagespace(
                ps.object as u32,
                &mut untyped_work,
                va,
                phys,
                PAGE_SIZE,
                PAGE_PRESENT | PAGE_USER,
                "sys_thread_conf: code",
            )?;
        }
    }

    let consumed = untyped_work.object - original_object;
    unsafe { commit_untyped(cspace, untyped_idx, consumed) };

    let user_esp = ustack_vaddr + ustack_size as u32;
    unsafe {
        init_thread(
            th.object as u32,
            tid,
            code_vaddr,
            user_esp,
            ps.object as u32,
            kstack_vaddr,
        );
    }

    verbose!(
        "sys_thread_conf: th={:#x} ps={:#x} entry={:#x} code_size={:#x} ustack={:#x} kstack={:#x} tid={} untyped[{}] consumed={}",
        th.object,
        ps.object,
        code_vaddr,
        code_size,
        ustack_vaddr,
        kstack_vaddr,
        tid,
        untyped_idx,
        consumed
    );
    Ok(SUCCESS.into())
}
