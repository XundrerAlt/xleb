// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use super::header;
use crate::logger::debug;
use crate::{PHYS_TO_VIRT, VIRT_TO_PHYS};

pub const MAX_MEMORY_REGIONS: usize = 64;

unsafe extern "C" {
    static _kernel_end: u8;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum MemoryType {
    Available = 1,
    Reserved = 2,
    AcpiReclaimable = 3,
    AcpiNvs = 4,
    BadRam = 5,
}

#[derive(Debug, Clone, Copy)]
pub struct MemoryRegion {
    pub start: u32,
    pub end: u32,
    pub size: u32,
    pub type_: MemoryType,
}

pub struct MemoryMap {
    pub regions: [MemoryRegion; MAX_MEMORY_REGIONS],
    pub region_count: usize,
    pub total_memory: u32,
    pub available_memory: u32,
}

impl MemoryMap {
    pub const fn new() -> Self {
        const ZERO: MemoryRegion = MemoryRegion {
            start: 0,
            end: 0,
            size: 0,
            type_: MemoryType::Reserved,
        };
        Self {
            regions: [ZERO; MAX_MEMORY_REGIONS],
            region_count: 0,
            total_memory: 0,
            available_memory: 0,
        }
    }
}

pub static mut MEMORY_MAP: MemoryMap = MemoryMap::new();

pub(crate) unsafe fn process(mbi: &header::multiboot_info_t) {
    if mbi.flags & header::MULTIBOOT_INFO_MEM_MAP == 0 {
        return;
    }
    debug!("- memory map ({} bytes):", mbi.mmap_length);

    let mmap_virt = PHYS_TO_VIRT!(mbi.mmap_addr);
    let mmap_end = mmap_virt + mbi.mmap_length as u32;

    let map = unsafe { &mut *core::ptr::addr_of_mut!(MEMORY_MAP) };
    let kend_phys = VIRT_TO_PHYS!(&raw const _kernel_end as usize) as u32;

    let mut p = mmap_virt;
    while p < mmap_end && map.region_count < MAX_MEMORY_REGIONS {
        let entry = unsafe { &*(p as *const header::memory_map_t) };
        let a = entry.addr;
        let mut b = entry.addr.wrapping_add(entry.len);
        if a >= 0x1_0000_0000 {
            p += entry.size + 4;
            continue;
        }
        if b > 0x1_0000_0000 {
            b = 0x1_0000_0000;
        }
        let mut start = a as u32;
        let end = (b - 1) as u32;
        let size = (b - a) as u32;
        if size == 0 {
            p += entry.size as u32 + 4;
            continue;
        }
        let mut region = MemoryRegion {
            start,
            end,
            size,
            type_: MemoryType::Reserved,
        };
        match entry.type_ {
            header::MULTIBOOT_MEMORY_AVAILABLE => {
                if end <= kend_phys {
                    region.type_ = MemoryType::Reserved;
                } else {
                    if start < kend_phys {
                        start = kend_phys;
                        region.start = start;
                        region.size = end - start + 1;
                    }
                    region.type_ = MemoryType::Available;
                    map.available_memory += region.size;
                }
            }
            header::MULTIBOOT_MEMORY_RESERVED => {
                region.type_ = MemoryType::Reserved;
            }
            header::MULTIBOOT_MEMORY_ACPI_RECLAIMABLE => {
                region.type_ = MemoryType::AcpiReclaimable;
            }
            header::MULTIBOOT_MEMORY_NVS => {
                region.type_ = MemoryType::AcpiNvs;
            }
            header::MULTIBOOT_MEMORY_BADRAM => {
                region.type_ = MemoryType::BadRam;
            }
            _ => {
                region.type_ = MemoryType::Reserved;
            }
        }
        debug!(
            "  - [0x{:x} - 0x{:x}] {:?} ({} KB)",
            region.start,
            region.end,
            region.type_,
            region.size / 1024
        );
        map.regions[map.region_count] = region;
        map.region_count += 1;
        map.total_memory += region.size;

        p += entry.size + 4;
    }
    if map.region_count == 0 {
        panic!("memory_map is empty");
    }
    debug!(
        "- total available memory: {} MB",
        map.available_memory / (1024 * 1024)
    );
}
