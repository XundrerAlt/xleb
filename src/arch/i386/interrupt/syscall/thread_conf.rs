// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use crate::arch::thread::{Thread, init_thread, KERNEL_STACK_ADDR, USER_STACK_ADDR};
use crate::arch::mm::{map_range_in_pagespace, PAGE_SIZE, PAGE_MASK, PAGE_WRITE, PAGE_USER, PAGE_PRESENT};
use crate::cap::{CAP_GRANT, CAP_WRITE, CNode, Cap, current_cspace, get_cap, commit_untyped};
use crate::logger::verbose;
use xleb_api::CapType;
use xleb_api::SyscallErrno::*;
use crate::arch::thread::STACK_SIZE;
use crate::utils::align_up;

use super::SyscallFrame;

fn validate_vaddr(vaddr: u32, name: &str) -> Result<(), i32> {
    if vaddr & PAGE_MASK as u32 != 0 {
        verbose!("sys_thread_conf: {}={:#x} not page-aligned", name, vaddr);
        return Err(EINVAL.into());
    }
    Ok(())
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

    let th = get_cap(cspace, th_idx, CapType::Thread, CAP_WRITE, "sys_thread_conf")?;
    let ps = get_cap(cspace, pagespace_idx, CapType::PageSpace, CAP_WRITE, "sys_thread_conf")?;
    let mut untyped = get_cap(cspace, untyped_idx, CapType::Untyped, CAP_WRITE | CAP_GRANT, "sys_thread_conf")?;

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
        verbose!("sys_thread_conf: untyped too small (need {}, have {})", need, untyped.size);
        return Err(ENOMEM.into());
    }
    untyped.object = aligned;
    untyped.size -= consumed_align;

    let kstack_phys = untyped.object as u32;
    let ustack_phys = (untyped.object + kstack_size) as u32;

    let tid = 1;
    let ustack_vaddr = USER_STACK_ADDR;
    // FIXME: kstack_vaddr должен быть kernel-VA, а не в user-half.
    // KERNEL_STACK_ADDR = 0x8008_0000 — kernel-half, ок.
    let kstack_vaddr = KERNEL_STACK_ADDR + tid * STACK_SIZE as u32;

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
        map_range_in_pagespace(
            ps.object as u32,
            &mut untyped_work,
            kstack_vaddr,
            kstack_phys,
            kstack_size,
            PAGE_PRESENT | PAGE_WRITE,
            "sys_thread_conf: kstack",
        )?;
    }

    let consumed = untyped_work.object - original_object;
    unsafe { commit_untyped(cspace, untyped_idx, consumed) };

    let user_esp = ustack_vaddr + ustack_size as u32;
    unsafe {
        init_thread(th.object as u32, tid, code_vaddr, user_esp, ps.object as u32, kstack_vaddr);
    }

    verbose!(
        "sys_thread_conf: th={:#x} ps={:#x} entry={:#x} code_size={:#x} ustack={:#x} kstack={:#x} tid={} untyped[{}] consumed={}",
        th.object, ps.object, code_vaddr, code_size, ustack_vaddr, kstack_vaddr, tid, untyped_idx, consumed
    );
    Ok(SUCCESS.into())
}
