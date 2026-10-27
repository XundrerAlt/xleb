// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use crate::cap::current_cspace;
use crate::logger::verbose;
use crate::sched::CURRENT_THREAD;
use core::sync::atomic::Ordering;
use xleb_api::CapType;
use xleb_api::SyscallErrno::*;

use super::SyscallFrame;

fn parse_cap_type(raw: u32) -> Result<CapType, i32> {
    match CapType::try_from(raw) {
        Ok(t) => Ok(t),
        Err(_) => {
            verbose!("sys_cap_find: invalid type_={}", raw);
            Err(EINVAL.into())
        }
    }
}

pub(crate) fn sys(frame: &mut SyscallFrame) -> i32 {
    sys_impl(frame).unwrap_or_else(|e| e)
}

fn sys_impl(frame: &mut SyscallFrame) -> Result<i32, i32> {
    let cap_type = parse_cap_type(frame.ebx)?;
    let object = frame.ecx as usize;
    let rights = frame.edx;
    let size = frame.esi as usize;

    let cspace = current_cspace()?;

    match cspace.find(cap_type, object, size, rights) {
        Some(idx) => Ok(idx as i32),
        None => {
            verbose!("sys_cap_find: no cap found");
            Err(ENOENT.into())
        }
    }
}
