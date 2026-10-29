// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use super::SyscallFrame;
use crate::cap::{CAP_GRANT, Cap, current_cspace, get_cap};
use crate::logger::verbose;
use xleb_api::CapType;
use xleb_api::SyscallErrno::*;

pub(crate) fn sys(frame: &mut SyscallFrame) -> i32 {
    sys_impl(frame).unwrap_or_else(|e| e)
}

fn sys_impl(frame: &mut SyscallFrame) -> Result<i32, i32> {
    let control_idx = frame.ebx as usize;
    let start = frame.ecx as u16;
    let count = frame.edx as usize;
    let rights = frame.esi;

    let cspace = current_cspace()?;
    let control = get_cap(
        cspace,
        control_idx,
        CapType::x86_PortIoControl_Cap,
        CAP_GRANT,
        "sys_x86_derive_portio",
    )?;

    if count == 0 {
        return Err(EINVAL.into());
    }
    if start as usize + count > control.object + control.size {
        verbose!(
            "sys_x86_derive_portio: range {:#x}..{:#x} not in control {:#x}..{:#x}",
            start,
            start as usize + count,
            control.object,
            control.object + control.size
        );
        return Err(EINVAL.into());
    }
    if rights & !control.rights != 0 {
        verbose!(
            "sys_x86_derive_portio: rights {:#x} not in {:#x}",
            rights,
            control.rights
        );
        return Err(EPERM.into());
    }

    let new_idx = unsafe {
        cspace.insert(
            None,
            Cap::new(CapType::x86_PortIo_Cap, start as usize, count, rights),
        )
    }
    .ok_or_else(|| ENOMEM)?;

    verbose!(
        "sys_x86_derive_portio: control[{}] -> x86_PortIo_Cap idx={} {:#x}..{:#x} rights={:#x}",
        control_idx,
        new_idx,
        start,
        start as usize + count,
        rights
    );
    Ok(new_idx as i32)
}
