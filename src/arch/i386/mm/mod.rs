// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
pub(crate) mod temp_window;
pub(crate) const PAGE_PRESENT: u32 = 1 << 0;
pub(crate) const PAGE_WRITE: u32 = 1 << 1;
pub(crate) const PAGE_USER: u32 = 1 << 2;

#[repr(C, align(4096))]
pub(crate) struct PageTable(pub(crate) [u32; 1024]);
#[repr(C, align(4096))]
pub(crate) struct PageDir(pub(crate) [u32; 1024]);

#[macro_export]
macro_rules! VIRT_TO_PHYS {
    ($addr:expr) => {{
        let a = ($addr) as u32;
        if a >= 0xC000_0000 { a - 0xC000_0000 } else { a }
    }};
}

#[macro_export]
macro_rules! PHYS_TO_VIRT {
    ($addr:expr) => {{
        let a = ($addr) as u32;
        if a < 0xC000_0000 { a + 0xC000_0000 } else { a }
    }};
}

pub unsafe fn map_page_in(pd: *mut u32, pt: *mut u32, va: u32, phys: u32, flags: u32) {
    let pd_idx = (va >> 22) as usize;
    let pt_idx = ((va >> 12) & 0x3FF) as usize;
    unsafe {
        let pt_phys = VIRT_TO_PHYS!(pt as u32);
        *pd.add(pd_idx) = pt_phys | PAGE_PRESENT | PAGE_WRITE | PAGE_USER;
        *pt.add(pt_idx) = (phys & !0xFFF) | flags;
    }
}
