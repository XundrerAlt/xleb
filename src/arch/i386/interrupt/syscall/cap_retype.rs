// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt

use crate::PHYS_TO_VIRT;
use crate::arch::mm::PAGE_SIZE;
use crate::arch::thread::{Thread, copy_kernel_pd};
use crate::cap::{CAP_WRITE, CNode, Cap, commit_untyped, current_cspace, get_cap};
use crate::logger::{verbose, warning};
use crate::utils::align_up;
use xleb_api::CapType;
use xleb_api::SyscallErrno::*;

use super::{MAX_FRAME_SIZE, SyscallFrame};

fn alloc_in_untyped(untyped: &Cap, size: usize, align: usize) -> Result<(usize, usize), i32> {
    let obj = align_up(untyped.object, align);
    let consumed = obj + size - untyped.object;
    if consumed > untyped.size {
        verbose!(
            "sys_cap_retype: no room (need {} bytes after align, have {})",
            consumed,
            untyped.size
        );
        return Err(ENOMEM.into());
    }
    Ok((obj, consumed))
}

fn parse_cap_type(raw: u32) -> Result<CapType, i32> {
    match CapType::try_from(raw) {
        Ok(t) => Ok(t),
        Err(_) => {
            verbose!("sys_cap_retype: invalid type_={}", raw);
            Err(EINVAL.into())
        }
    }
}

fn compute_size(cap_type: CapType, raw: usize) -> Result<usize, i32> {
    match cap_type {
        CapType::Frame => {
            if raw == 0 || raw % PAGE_SIZE != 0 {
                verbose!(
                    "sys_cap_retype: Frame size={} not multiple of {}",
                    raw,
                    PAGE_SIZE
                );
                return Err(EINVAL.into());
            }
            if raw > MAX_FRAME_SIZE {
                verbose!(
                    "sys_cap_retype: Frame size={} > MAX_FRAME_SIZE={}",
                    raw,
                    MAX_FRAME_SIZE
                );
                return Err(EINVAL.into());
            }
            Ok(raw)
        }
        CapType::PageTable | CapType::PageSpace => Ok(PAGE_SIZE),
        CapType::CNode | CapType::Untyped => Ok(raw),
        CapType::Thread => Ok(core::mem::size_of::<Thread>()),
        CapType::None => {
            verbose!("sys_cap_retype: None is invalid");
            Err(EINVAL.into())
        }
    }
}

fn insert_cap(
    cspace: &CNode,
    cap_type: CapType,
    obj: usize,
    size: usize,
    rights: u32,
) -> Result<usize, i32> {
    unsafe { cspace.insert(None, Cap::new(cap_type, obj, size, rights)) }.ok_or_else(|| {
        warning!("sys_cap_retype: CNode full (no free slot)");
        ENOMEM.into()
    })
}

unsafe fn zero_object(obj: usize, len: usize) {
    unsafe {
        let va = PHYS_TO_VIRT!(obj as u32) as *mut u8;
        core::ptr::write_bytes(va, 0, len);
    }
}

pub(crate) fn sys(frame: &mut SyscallFrame) -> i32 {
    sys_impl(frame).unwrap_or_else(|e| e)
}

fn sys_impl(frame: &mut SyscallFrame) -> Result<i32, i32> {
    let untyped_idx = frame.ebx as usize;
    let cap_type = parse_cap_type(frame.ecx)?;
    let size = compute_size(cap_type, frame.edx as usize)?;

    let cspace = current_cspace()?;
    let untyped = get_cap(
        cspace,
        untyped_idx,
        CapType::Untyped,
        CAP_WRITE,
        "sys_cap_retype",
    )?;

    let (obj, consumed) = alloc_in_untyped(&untyped, size, PAGE_SIZE)?;

    let new_idx = insert_cap(cspace, cap_type, obj, size, untyped.rights)?;

    unsafe {
        commit_untyped(cspace, untyped_idx, consumed);
    }

    if matches!(cap_type, CapType::Frame | CapType::PageTable) {
        unsafe { zero_object(obj, PAGE_SIZE) };
    }
    if cap_type == CapType::PageSpace {
        unsafe {
            zero_object(obj, PAGE_SIZE);
            copy_kernel_pd(PHYS_TO_VIRT!(obj as u32) as *mut u32);
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
    Ok(new_idx as i32)
}
