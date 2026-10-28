// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt

use crate::cap::Cap;
use crate::logger::verbose;
use crate::utils::align_up;
use xleb_api::SyscallErrno::*;

pub(crate) const PAGE_PRESENT: u32 = 1 << 0;
pub(crate) const PAGE_WRITE: u32 = 1 << 1;
pub(crate) const PAGE_USER: u32 = 1 << 2;

pub(crate) const PAGE_SIZE: usize = 4096;
pub(crate) const PAGE_MASK: usize = 0xFFF;
pub(crate) const PT_ENTRIES: usize = 1024;
pub(crate) const PD_SHIFT: u32 = 22;
pub(crate) const PT_SHIFT: u32 = 12;
pub(crate) const PT_MASK: u32 = 0x3FF;

#[repr(C, align(4096))]
pub(crate) struct PageTable(pub(crate) [u32; 1024]);
#[repr(C, align(4096))]
pub(crate) struct PageDir(pub(crate) [u32; 1024]);

// TODO: direct map covers [0, 2 GiB) (kernel-half starts at 0x80000000).
// RAM above 2 GiB is unsupported until we implement highmem/PAE or move the
// kernel-half boundary.
#[macro_export]
macro_rules! VIRT_TO_PHYS {
    ($addr:expr) => {{
        let a = ($addr) as u32;
        if a >= 0x8000_0000 { a - 0x8000_0000 } else { a }
    }};
}

#[macro_export]
macro_rules! PHYS_TO_VIRT {
    ($addr:expr) => {{
        let a = ($addr) as u32;
        if a < 0x8000_0000 { a + 0x8000_0000 } else { a }
    }};
}

pub(crate) unsafe fn map_page_in(pd: *mut u32, pt: *mut u32, va: u32, phys: u32, flags: u32) {
    let pd_idx = (va >> PD_SHIFT) as usize;
    let pt_idx = ((va >> PT_SHIFT) & PT_MASK) as usize;
    unsafe {
        let pt_phys = VIRT_TO_PHYS!(pt as u32);
        *pd.add(pd_idx) = pt_phys | PAGE_PRESENT | PAGE_WRITE | PAGE_USER;
        *pt.add(pt_idx) = (phys & !PAGE_MASK as u32) | flags;
    }
}

pub(crate) unsafe fn map_region(
    pd: *mut u32,
    pt: *mut u32,
    dst_va: u32,
    src_phys: u32,
    len: usize,
    flags: u32,
) {
    let pages = (len + PAGE_SIZE - 1) / PAGE_SIZE;
    for i in 0..pages {
        let phys = src_phys + (i as u32) * PAGE_SIZE as u32;
        let va = dst_va + (i as u32) * PAGE_SIZE as u32;
        unsafe { map_page_in(pd, pt, va, phys, flags) };
    }
}

pub(crate) unsafe fn alloc_pt_from_untyped(untyped: &mut Cap, ctx: &str) -> Result<u32, i32> {
    let align = PAGE_SIZE;
    let obj = align_up(untyped.object, align);
    let consumed = obj + PAGE_SIZE - untyped.object;
    if consumed > untyped.size {
        verbose!(
            "{}: untyped too small for PT (need {}, have {})",
            ctx,
            consumed,
            untyped.size
        );
        return Err(ENOMEM.into());
    }
    let pt_phys = obj as u32;
    untyped.object += consumed;
    untyped.size -= consumed;
    unsafe {
        let pt_va = PHYS_TO_VIRT!(pt_phys) as *mut u8;
        core::ptr::write_bytes(pt_va, 0, PAGE_SIZE);
    }
    verbose!(
        "{}: allocated PT at {:#x} (consumed={})",
        ctx,
        pt_phys,
        consumed
    );
    Ok(pt_phys)
}

pub(crate) unsafe fn map_range_in_pagespace(
    ps_phys: u32,
    untyped: &mut Cap,
    vaddr: u32,
    paddr: u32,
    size: usize,
    flags: u32,
    ctx: &str,
) -> Result<(), i32> {
    if vaddr & PAGE_MASK as u32 != 0 || paddr & PAGE_MASK as u32 != 0 {
        verbose!(
            "{}: vaddr={:#x} or paddr={:#x} not page-aligned",
            ctx,
            vaddr,
            paddr
        );
        return Err(EINVAL.into());
    }
    let pages = size / PAGE_SIZE;
    if pages == 0 {
        return Err(EINVAL.into());
    }

    unsafe {
        let pd = PHYS_TO_VIRT!(ps_phys) as *mut u32;
        let pd_index = (vaddr >> PD_SHIFT) as usize;
        if pd_index >= PT_ENTRIES {
            verbose!("{}: pd_index={} out of range", ctx, pd_index);
            return Err(EINVAL.into());
        }

        let pt_phys = {
            let pde = *pd.add(pd_index);
            if pde & PAGE_PRESENT == 0 {
                let pt = unsafe { alloc_pt_from_untyped(untyped, ctx)? };
                *pd.add(pd_index) = pt | PAGE_PRESENT | PAGE_WRITE | PAGE_USER;
                pt
            } else {
                pde & !PAGE_MASK as u32
            }
        };

        let pt = PHYS_TO_VIRT!(pt_phys) as *mut u32;
        let pt_index = ((vaddr >> PT_SHIFT) & PT_MASK) as usize;
        if pt_index + pages > PT_ENTRIES {
            verbose!(
                "{}: {} pages at pt_index={} cross PT boundary",
                ctx,
                pages,
                pt_index
            );
            return Err(ENOMEM.into());
        }
        for i in 0..pages {
            let pa = paddr + (i as u32) * PAGE_SIZE as u32;
            let pte = *pt.add(pt_index + i);
            if pte & PAGE_PRESENT != 0 {
                let cur_phys = pte & !PAGE_MASK as u32;
                if cur_phys != pa {
                    verbose!(
                        "{}: page {:#x} already mapped to {:#x}, expected {:#x}",
                        ctx,
                        vaddr + (i as u32) * PAGE_SIZE as u32,
                        cur_phys,
                        pa
                    );
                    return Err(EINVAL.into());
                }
            } else {
                *pt.add(pt_index + i) = (pa & !PAGE_MASK as u32) | flags;
            }
        }
    }
    Ok(())
}
