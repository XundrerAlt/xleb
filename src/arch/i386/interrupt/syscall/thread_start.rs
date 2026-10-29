// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt

use crate::PHYS_TO_VIRT;
use crate::arch::thread::{Thread, ThreadState};
use crate::cap::{CAP_WRITE, current_cspace, get_cap};
use crate::logger::verbose;
use crate::sched::RUN_QUEUE;
use xleb_api::CapType;
use xleb_api::SyscallErrno::*;

use super::SyscallFrame;

pub(crate) fn sys(frame: &mut SyscallFrame) -> i32 {
    sys_impl(frame).unwrap_or_else(|e| e)
}

fn sys_impl(frame: &mut SyscallFrame) -> Result<i32, i32> {
    let th_idx = frame.ebx as usize;
    verbose!("sys_thread_start: enter");

    let cspace = current_cspace()?;
    let th = get_cap(
        cspace,
        th_idx,
        CapType::Thread,
        CAP_WRITE,
        "sys_thread_start",
    )?;
    if th.object == 0 {
        verbose!("sys_thread_start: th_idx={} invalid", th_idx);
        return Err(EINVAL.into());
    }

    let th_phys = th.object as u32;
    let state = unsafe {
        let th_va = PHYS_TO_VIRT!(th_phys) as *mut Thread;
        (*th_va).state
    };

    if state != ThreadState::Ready {
        verbose!(
            "sys_thread_start: th_idx={} state={:?} (expected Ready)",
            th_idx,
            state
        );
        return Err(EINVAL.into());
    }

    let th_va = PHYS_TO_VIRT!(th.object as u32) as *mut Thread;
    RUN_QUEUE.lock().push(th_va);

    verbose!(
        "sys_thread_start: th_idx={} th={:#x} -> RUN_QUEUE (len={})",
        th_idx,
        th.object,
        RUN_QUEUE.lock().len()
    );

    Ok(SUCCESS.into())
}
