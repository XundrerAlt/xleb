// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt

use super::SyscallFrame;
use crate::PHYS_TO_VIRT;
use crate::arch::thread::{Thread, ThreadState};
use crate::cap::{CAP_WRITE, CapType, current_cspace, get_cap};
use crate::logger::verbose;
use crate::sched::{current, yield_now};
use xleb_api::SyscallErrno::*;

pub(crate) fn sys(frame: &mut SyscallFrame) -> i32 {
    sys_impl(frame).unwrap_or_else(|e| e)
}

fn sys_impl(frame: &mut SyscallFrame) -> Result<i32, i32> {
    let th_idx = frame.ebx as usize;

    let cspace = current_cspace()?;
    let th = get_cap(
        cspace,
        th_idx,
        CapType::Thread,
        CAP_WRITE,
        "sys_thread_suspend",
    )?;

    if th.object == 0 {
        verbose!("sys_thread_suspend: th_idx={} invalid object", th_idx);
        return Err(EINVAL.into());
    }

    let th_phys = th.object as u32;
    let th_va = PHYS_TO_VIRT!(th_phys) as *mut Thread;
    if th_va.is_null() {
        verbose!("sys_thread_suspend: th_idx={} null", th_idx);
        return Err(EINVAL.into());
    }

    if th_va == current() {
        verbose!(
            "sys_thread_suspend: self-suspend th_idx={} tid={} -> Inactive, yield",
            th_idx,
            unsafe { (*th_va).tid }
        );
        unsafe {
            (*th_va).state = ThreadState::Inactive;
        }
        unsafe {
            yield_now();
        }
        unreachable!("scheduler returned to Inactive thread")
    }

    let tid = unsafe { (*th_va).tid };
    unsafe {
        (*th_va).state = ThreadState::Inactive;
    }

    verbose!(
        "sys_thread_suspend: th_idx={} tid={} -> Inactive (ok)",
        th_idx,
        tid
    );
    Ok(SUCCESS.into())
}
