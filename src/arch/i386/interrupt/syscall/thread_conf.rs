// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use crate::arch::thread::{Thread, init_thread};
use crate::cap::{CAP_WRITE, CNode, Cap, current_cspace, get_cap};
use crate::logger::verbose;
use crate::sched::CURRENT_THREAD;
use core::sync::atomic::Ordering;
use xleb_api::CapType;
use xleb_api::SyscallErrno::*;

use super::SyscallFrame;

const TID: u32 = 1;

fn validate_vaddrs(code: u32, ustack: u32) -> Result<(), i32> {
    if code & 0xFFF != 0 || ustack & 0xFFF != 0 {
        return Err(EINVAL.into());
    }
    Ok(())
}

unsafe fn conf_thread(
    th: &Cap,
    pagespace: &Cap,
    code_vaddr: u32,
    ustack_vaddr: u32,
    kstack_vaddr: u32,
) {
    unsafe {
        init_thread(
            th.object as *mut Thread,
            TID,
            code_vaddr,
            ustack_vaddr,
            pagespace.object as u32,
            kstack_vaddr,
        );
    }
}

pub(crate) fn sys(frame: &mut SyscallFrame) -> i32 {
    sys_impl(frame).unwrap_or_else(|e| e)
}

fn sys_impl(frame: &mut SyscallFrame) -> Result<i32, i32> {
    let thread_idx = frame.ebx as usize;
    let pagespace_idx = frame.ecx as usize;
    let code_vaddr = frame.edx;
    let ustack_vaddr = frame.esi;
    let kstack_vaddr = frame.edi;

    let cspace = current_cspace()?;

    let th = get_cap(
        cspace,
        thread_idx,
        CapType::Thread,
        CAP_WRITE,
        "sys_thread_conf",
    )?;
    if th.object == 0 {
        verbose!("sys_thread_conf: thread_idx={} has object=0", thread_idx);
        return Err(EINVAL.into());
    }

    let pagespace = get_cap(
        cspace,
        pagespace_idx,
        CapType::PageSpace,
        CAP_WRITE,
        "sys_thread_conf",
    )?;
    if pagespace.object == 0 || pagespace.object == th.object {
        verbose!(
            "sys_thread_conf: pagespace_idx={} invalid object",
            pagespace_idx
        );
        return Err(EINVAL.into());
    }

    validate_vaddrs(code_vaddr, ustack_vaddr)?;

    unsafe {
        conf_thread(&th, &pagespace, code_vaddr, ustack_vaddr, kstack_vaddr);
    }

    verbose!(
        "sys_thread_conf: th={:#x} space={:#x} entry={:#x} ustack={:#x} tid={}",
        th.object,
        pagespace.object,
        code_vaddr,
        ustack_vaddr,
        TID
    );
    Ok(SUCCESS.into())
}
