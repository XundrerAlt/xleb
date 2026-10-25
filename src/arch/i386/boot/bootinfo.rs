// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt

use crate::arch::boot::mmap::MEMORY_MAP;
use crate::arch::boot::modules;
use crate::arch::boot::multiboot1::mmap;
use crate::logger::verbose;
use xleb_api::{MAX_MEMORY_REGIONS,MAX_MODULES,BOOTINFO_MAGIC,BootInfo,MemoryMapInfo,ModuleInfo,ModuleEntry,KernelInfo};

unsafe extern "C" {
    static _kernel_start: u8;
    static _kernel_end: u8;
}

#[repr(C, align(4096))]
pub struct Buf(pub [u8; 4096]);

pub static mut BUF: Buf = Buf([0; 4096]);

pub unsafe fn fill() -> u32 {
    verbose!("bootinfo: fill begin");

    let buf = unsafe { &mut *core::ptr::addr_of_mut!(BUF) };
    let base = buf.0.as_mut_ptr() as u32;
    let base_phys = crate::VIRT_TO_PHYS!(base);
    let kstart_phys = crate::VIRT_TO_PHYS!(&raw const _kernel_start as usize) as u32;
    let kend_phys = crate::VIRT_TO_PHYS!(&raw const _kernel_end as usize) as u32;

    let mmap = unsafe { &*core::ptr::addr_of!(MEMORY_MAP) };

    verbose!("bootinfo: buf virt={:#x} phys={:#x}", base, base_phys,);
    verbose!(
        "bootinfo: kernel phys {:#x}..{:#x} ({} KB)",
        kstart_phys,
        kend_phys,
        (kend_phys - kstart_phys) / 1024,
    );
    verbose!(
        "bootinfo: memory_map regions={} modules={}",
        mmap.region_count,
        modules::all().map(|m| m.len()).unwrap_or(0),
    );

    let bi = base as *mut BootInfo;
    unsafe {
        (*bi).magic = BOOTINFO_MAGIC;
        (*bi).version = 1;
        (*bi).kernel = KernelInfo {
            start: kstart_phys,
            end: kend_phys,
        };

        let regions_ptr =
            (base + core::mem::size_of::<BootInfo>() as u32) as *mut mmap::MemoryRegion;
        let count = MEMORY_MAP.region_count.min(MAX_MEMORY_REGIONS);
        for i in 0..count {
            *regions_ptr.add(i) = MEMORY_MAP.regions[i];
        }
        (*bi).memory_map = MemoryMapInfo {
            regions: regions_ptr as u32,
            count: count as u32,
        };

        let mods = modules::all().unwrap_or(&[]);
        let mods_count = mods.len().min(MAX_MODULES);
        let mods_ptr = regions_ptr.add(count) as *mut ModuleEntry;
        for (i, &(start, end)) in mods.iter().take(mods_count).enumerate() {
            *mods_ptr.add(i) = ModuleEntry {
                start,
                end,
                cmdline: 0,
            };
        }
        (*bi).modules = ModuleInfo {
            modules: mods_ptr as u32,
            count: mods_count as u32,
        };

        verbose!(
            "bootinfo: header={:#x} magic={:#x} version={}",
            base,
            (*bi).magic,
            (*bi).version,
        );
        verbose!(
            "bootinfo: memory_map ptr={:#x} count={}",
            (*bi).memory_map.regions,
            (*bi).memory_map.count,
        );
        verbose!(
            "bootinfo: modules ptr={:#x} count={}",
            (*bi).modules.modules,
            (*bi).modules.count,
        );
    }

    verbose!("bootinfo: fill end (phys={:#x})", base_phys);
    base_phys
}

pub fn virt_addr() -> u32 {
    core::ptr::addr_of!(BUF) as u32
}

pub fn phys_addr() -> u32 {
    let base = core::ptr::addr_of!(BUF) as u32;
    crate::VIRT_TO_PHYS!(base)
}
