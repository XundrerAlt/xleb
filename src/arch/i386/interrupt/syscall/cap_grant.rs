// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use crate::PHYS_TO_VIRT;
use crate::arch::thread::Thread;
use crate::cap::{CAP_GRANT, Cap, current_cspace, get_cap};
use crate::logger::verbose;
use core::sync::atomic::Ordering;
use xleb_api::CapType;
use xleb_api::SyscallErrno::*;

use super::SyscallFrame;

pub(crate) fn sys(frame: &mut SyscallFrame) -> i32 {
    sys_impl(frame).unwrap_or_else(|e| e)
}

fn sys_impl(frame: &mut SyscallFrame) -> Result<i32, i32> {
    let target_thread_idx = frame.ebx as usize;
    let src_cap_idx = frame.ecx as usize;
    let rights = frame.edx;

    if rights == 0 {
        verbose!("sys_cap_grant: rights=0");
        return Err(EINVAL.into());
    }

    let cspace = current_cspace()?;
    let target_cap = get_cap(
        cspace,
        target_thread_idx,
        CapType::Thread,
        CAP_GRANT,
        "sys_cap_grant",
    )?;
    if target_cap.object == 0 {
        verbose!(
            "sys_cap_grant: target_thread_idx={} invalid",
            target_thread_idx
        );
        return Err(EINVAL.into());
    }
    let target_th = PHYS_TO_VIRT!(target_cap.object as u32) as *mut Thread;

    let src = cspace.get(src_cap_idx).ok_or_else(|| {
        verbose!("sys_cap_grant: src_cap_idx={} not found", src_cap_idx);
        EINVAL
    })?;
    if src.is_none() {
        verbose!("sys_cap_grant: src_cap_idx={} is None", src_cap_idx);
        return Err(EINVAL.into());
    }

    if rights & !src.rights != 0 {
        verbose!(
            "sys_cap_grant: rights {:#x} not in src {:#x}",
            rights,
            src.rights
        );
        return Err(EPERM.into());
    }

    let new_cap = Cap::new(src.type_, src.object, src.size, rights);
    let target_cspace = unsafe { &(*target_th).cspace };
    let new_idx = unsafe { target_cspace.insert(None, new_cap) }.ok_or_else(|| {
        verbose!("sys_cap_grant: target cspace full");
        ENOMEM
    })?;

    verbose!(
        "sys_cap_grant: {:?} idx={} (src[{}]) -> target[{}] idx={} rights={:#x}",
        src.type_,
        new_idx,
        src_cap_idx,
        target_thread_idx,
        new_idx,
        rights
    );
    Ok(new_idx as i32)
}
