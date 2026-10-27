// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt

use crate::arch::cpu;
use crate::arch::mm::temp_window::TempWindow;
use crate::arch::mm::{PAGE_PRESENT, PAGE_USER, PAGE_WRITE, PAGE_MASK};
use crate::arch::thread::PAGE_SPACE_ADDR;
use crate::cap::{CAP_READ, CAP_WRITE, CNode, Cap, current_cspace, get_cap};
use crate::logger::verbose;
use crate::sched::CURRENT_THREAD;
use core::sync::atomic::Ordering;
use xleb_api::CapType;
use xleb_api::SyscallErrno::*;

use super::{MAX_FRAME_SIZE, SyscallFrame, PAGE_SIZE};
const PT_ENTRIES: usize = 1024;

fn validate_frame_map(pt: &Cap, frame: &Cap, vaddr: u32) -> Result<(usize, usize, usize), i32> {
    if frame.size > MAX_FRAME_SIZE {
        verbose!("sys_frame_map: frame size={} > MAX_FRAME_SIZE", frame.size);
        return Err(EINVAL.into());
    }
    if frame.size == 0 || frame.size % PAGE_SIZE != 0 {
        verbose!("sys_frame_map: frame size={} invalid", frame.size);
        return Err(EINVAL.into());
    }
    if vaddr & PAGE_MASK as u32 != 0 {
        verbose!("sys_frame_map: vaddr={:#x} not page-aligned", vaddr);
        return Err(EINVAL.into());
    }
    if pt.object & PAGE_MASK != 0 {
        verbose!("sys_frame_map: pt_obj={:#x} not page-aligned", pt.object);
        return Err(EINVAL.into());
    }
    if frame.object & PAGE_MASK != 0 {
        verbose!("sys_frame_map: frame={:#x} not page-aligned", frame.object);
        return Err(EINVAL.into());
    }

    let pd_index = (vaddr >> 22) as usize;
    let pt_index = ((vaddr >> 12) & 0x3FF) as usize;
    let pages = frame.size / PAGE_SIZE;

    if pt_index + pages > PT_ENTRIES {
        verbose!(
            "sys_frame_map: {} pages at pt_index={} cross PT boundary",
            pages,
            pt_index
        );
        return Err(ENOMEM.into());
    }

    let end_vaddr = vaddr as u64 + (pages as u64) * (PAGE_SIZE as u64);
    if (end_vaddr - 1) >> 22 != (vaddr >> 22) as u64 {
        verbose!("sys_frame_map: frame crosses PD boundary");
        return Err(ENOMEM.into());
    }

    Ok((pd_index, pt_index, pages))
}

fn frame_flags(cap: &Cap) -> u32 {
    PAGE_PRESENT | PAGE_USER | if cap.has(CAP_WRITE) { PAGE_WRITE } else { 0 }
}

unsafe fn write_pt(pt_phys: u32, pt_index: usize, frame_phys: u32, pages: usize, flags: u32) {
    unsafe {
        let mut win = TempWindow::map(pt_phys, PAGE_SIZE);
        let pt = win.as_mut_ptr() as *mut u32;
        for i in 0..pages {
            *pt.add(pt_index + i) = (frame_phys + (i as u32) * PAGE_SIZE as u32) | flags;
        }
    }
}

unsafe fn write_pd(pd_index: usize, pt_phys: u32) {
    unsafe {
        let pd = PAGE_SPACE_ADDR as *mut u32;
        *pd.add(pd_index) = pt_phys | PAGE_PRESENT | PAGE_WRITE | PAGE_USER;
    }
}

unsafe fn flush_tlb(vaddr: u32, pages: usize) {
    unsafe {
        for i in 0..pages {
            cpu::invlpg(vaddr + (i as u32) * PAGE_SIZE as u32);
        }
    }
}

pub(crate) fn sys(frame: &mut SyscallFrame) -> i32 {
    sys_impl(frame).unwrap_or_else(|e| e)
}

fn sys_impl(frame: &mut SyscallFrame) -> Result<i32, i32> {
    let pt_idx = frame.ebx as usize;
    let frame_idx = frame.ecx as usize;
    let vaddr = frame.edx;

    let cspace = current_cspace()?;

    let pt_cap = get_cap(
        cspace,
        pt_idx,
        CapType::PageTable,
        CAP_WRITE,
        "sys_frame_map",
    )?;
    let frame_cap = get_cap(cspace, frame_idx, CapType::Frame, CAP_READ, "sys_frame_map")?;

    let (pd_index, pt_index, pages) = validate_frame_map(&pt_cap, &frame_cap, vaddr)?;

    let pt_phys = pt_cap.object as u32;
    let frame_phys = frame_cap.object as u32;
    let flags = frame_flags(&frame_cap);

    unsafe {
        write_pt(pt_phys, pt_index, frame_phys, pages, flags);
        write_pd(pd_index, pt_phys);
        flush_tlb(vaddr, pages);
    }

    verbose!(
        "sys_frame_map: frame={:#x} size={} pages={} -> pt_obj={:#x} vaddr={:#x}",
        frame_phys,
        frame_cap.size,
        pages,
        pt_phys,
        vaddr
    );
    Ok(vaddr as i32)
}
