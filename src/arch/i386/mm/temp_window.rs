// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use crate::arch::cpu;
use crate::arch::mm::{PAGE_PRESENT, PAGE_WRITE, PageTable, map_page_in};
use crate::arch::thread::PAGE_SPACE_ADDR;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicUsize, Ordering};

static mut TEMP_PT: PageTable = PageTable([0; 1024]);
pub const TEMP_WINDOW_BASE: u32 = 0xE0000000;
pub const TEMP_WINDOW_SLOTS: usize = 16;
pub const TEMP_WINDOW_SIZE: usize = 4096;
static TEMP_SLOTS: AtomicUsize = AtomicUsize::new(0);

pub struct TempWindow {
    virt: *mut u8,
    size: usize,
}

impl TempWindow {
    pub unsafe fn map(phys: u32, size: usize) -> Self {
        let virt = unsafe { map(phys) };
        Self {
            virt: virt as *mut u8,
            size,
        }
    }

    pub fn as_ptr(&self) -> *mut u8 {
        self.virt
    }
    pub fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.virt, self.size) }
    }
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.virt, self.size) }
    }
}

impl Drop for TempWindow {
    fn drop(&mut self) {
        unsafe { unmap(self.virt as u32) };
    }
}

impl Deref for TempWindow {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl DerefMut for TempWindow {
    fn deref_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

pub unsafe fn map(phys: u32) -> u32 {
    let slots = TEMP_SLOTS.load(Ordering::Acquire);
    let slot = match (0..TEMP_WINDOW_SLOTS).find(|&i| slots & (1 << i) == 0) {
        Some(i) => i,
        None => panic!("temp_map: no free slot"),
    };
    TEMP_SLOTS.fetch_or(1 << slot, Ordering::AcqRel);

    let virt = TEMP_WINDOW_BASE + (slot as u32) * TEMP_WINDOW_SIZE as u32;

    let pd_virt = PAGE_SPACE_ADDR as *mut u32;
    let temp_pt = unsafe { core::ptr::addr_of_mut!(TEMP_PT.0) as *mut u32 };

    unsafe {
        map_page_in(pd_virt, temp_pt, virt, phys, PAGE_PRESENT | PAGE_WRITE);
        cpu::invlpg(virt);
    }

    virt
}

pub unsafe fn unmap(virt: u32) {
    let slot = ((virt - TEMP_WINDOW_BASE) / TEMP_WINDOW_SIZE as u32) as usize;
    if slot >= TEMP_WINDOW_SLOTS {
        return;
    }

    let temp_pt = unsafe { core::ptr::addr_of_mut!(TEMP_PT.0) as *mut u32 };
    let pt_index = ((virt >> 12) & 0x3FF) as usize;
    unsafe {
        *temp_pt.add(pt_index) = 0;
        cpu::invlpg(virt);
    }

    TEMP_SLOTS.fetch_and(!(1 << slot), Ordering::AcqRel);
}
