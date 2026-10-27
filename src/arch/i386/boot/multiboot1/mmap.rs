// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use super::header;
use super::modules;
use crate::logger::{debug, verbose, warning};
use crate::{PHYS_TO_VIRT, VIRT_TO_PHYS};
use xleb_api::MAX_MEMORY_REGIONS;

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
    pub start: usize,
    pub end: usize,
    pub size: usize,
    pub type_: MemoryType,
}

pub struct MemoryMap {
    pub regions: [MemoryRegion; MAX_MEMORY_REGIONS],
    pub region_count: usize,
    pub total_memory: usize,
    pub available_memory: usize,
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

fn push_region(map: &mut MemoryMap, r: MemoryRegion) {
    if map.region_count >= MAX_MEMORY_REGIONS {
        warning!("memory map: too many regions (max {})", MAX_MEMORY_REGIONS);
        return;
    }
    debug!(
        "  - [0x{:x} - 0x{:x}] {:?} ({} KB)",
        r.start,
        r.end,
        r.type_,
        r.size / 1024
    );
    map.regions[map.region_count] = r;
    map.region_count += 1;
    map.total_memory += r.size;
}

pub static mut MEMORY_MAP: MemoryMap = MemoryMap::new();

pub(crate) unsafe fn process(mbi: &header::multiboot_info_t) {
    if mbi.flags & header::MULTIBOOT_INFO_MEM_MAP == 0 {
        return;
    }
    debug!("- memory map ({} bytes):", mbi.mmap_length);

    let mmap_virt = PHYS_TO_VIRT!(mbi.mmap_addr) as usize;
    let mmap_end = mmap_virt + mbi.mmap_length as usize;

    let map = unsafe { &mut *core::ptr::addr_of_mut!(MEMORY_MAP) };
    let kend_phys = VIRT_TO_PHYS!(&raw const _kernel_end as usize) as usize;

    let mut p = mmap_virt;
    while p < mmap_end && map.region_count < MAX_MEMORY_REGIONS {
        let entry = unsafe { &*(p as *const header::memory_map_t) };
        let a = entry.addr;
        let mut b = entry.addr.wrapping_add(entry.len);
        if a >= 0x1_0000_0000 {
            p += entry.size as usize + 4;
            continue;
        }
        if b > 0x1_0000_0000 {
            b = 0x1_0000_0000;
        }
        let mut start = a as usize;
        let end = (b - 1) as usize;
        let size = (b - a) as usize;
        if size == 0 {
            p += entry.size as usize + 4;
            continue;
        }
        let _region = MemoryRegion {
            start,
            end,
            size,
            type_: MemoryType::Reserved,
        };
        match entry.type_ {
            header::MULTIBOOT_MEMORY_AVAILABLE => {
                verbose!("kend_phys = {:#x}", kend_phys);
                verbose!(
                    "region: start={:#x} end={:#x} type={}",
                    start,
                    end,
                    entry.type_
                );
                if end <= kend_phys {
                    verbose!("  -> all reserved (end <= kend_phys)");
                    push_region(
                        map,
                        MemoryRegion {
                            start,
                            end,
                            size: end - start + 1,
                            type_: MemoryType::Reserved,
                        },
                    );
                } else {
                    verbose!("  -> splitting around modules");
                    if start < kend_phys {
                        start = kend_phys;
                    }
                    let mut cur = start;
                    for &(m_start, m_end) in modules::all().unwrap_or(&[]) {
                        let m_start = m_start as usize;
                        let m_end = m_end as usize;
                        if m_end <= cur || m_start > end {
                            continue;
                        }
                        if m_start > cur {
                            let a_end = core::cmp::min(m_start - 1, end);
                            if a_end >= cur {
                                let sz = a_end - cur + 1;
                                push_region(
                                    map,
                                    MemoryRegion {
                                        start: cur,
                                        end: a_end,
                                        size: sz,
                                        type_: MemoryType::Available,
                                    },
                                );
                                map.available_memory += sz;
                            }
                        }
                        let r_start = core::cmp::max(m_start, cur);
                        let r_end = core::cmp::min(m_end - 1, end);
                        if r_end >= r_start {
                            let sz = r_end - r_start + 1;
                            push_region(
                                map,
                                MemoryRegion {
                                    start: r_start,
                                    end: r_end,
                                    size: sz,
                                    type_: MemoryType::Reserved,
                                },
                            );
                        }
                        cur = core::cmp::max(cur, m_end);
                        if cur > end {
                            break;
                        }
                    }
                    if cur <= end {
                        let sz = end - cur + 1;
                        push_region(
                            map,
                            MemoryRegion {
                                start: cur,
                                end,
                                size: sz,
                                type_: MemoryType::Available,
                            },
                        );
                        map.available_memory += sz;
                    }
                }
            }
            header::MULTIBOOT_MEMORY_RESERVED => {
                push_region(
                    map,
                    MemoryRegion {
                        start,
                        end,
                        size,
                        type_: MemoryType::Reserved,
                    },
                );
            }
            header::MULTIBOOT_MEMORY_ACPI_RECLAIMABLE => {
                push_region(
                    map,
                    MemoryRegion {
                        start,
                        end,
                        size,
                        type_: MemoryType::AcpiReclaimable,
                    },
                );
            }
            header::MULTIBOOT_MEMORY_NVS => {
                push_region(
                    map,
                    MemoryRegion {
                        start,
                        end,
                        size,
                        type_: MemoryType::AcpiNvs,
                    },
                );
            }
            header::MULTIBOOT_MEMORY_BADRAM => {
                push_region(
                    map,
                    MemoryRegion {
                        start,
                        end,
                        size,
                        type_: MemoryType::BadRam,
                    },
                );
            }
            _ => {
                push_region(
                    map,
                    MemoryRegion {
                        start,
                        end,
                        size,
                        type_: MemoryType::Reserved,
                    },
                );
            }
        }
        p += entry.size as usize + 4;
    }
    if map.region_count == 0 {
        panic!("memory_map is empty");
    }
    debug!(
        "- total available memory: {} MB",
        map.available_memory / (1024 * 1024)
    );
}
