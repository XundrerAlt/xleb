// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
mod header;
mod section;

use crate::logger::{debug,verbose};
use crate::PHYS_TO_VIRT;

unsafe extern "C" {
    static boot_magic: u32;
    static info_ptr: u32;
}

pub(crate) unsafe fn init() {
    let magic = unsafe { core::ptr::read_volatile(&raw const boot_magic) };
    let info = unsafe { PHYS_TO_VIRT!(core::ptr::read_volatile(&raw const info_ptr)) };
    if magic != header::MULTIBOOT_BOOTLOADER_MAGIC {
        panic!("multiboot1: boot magic (0x{:x}) not equals to 0x{:x}",
               magic, header::MULTIBOOT_BOOTLOADER_MAGIC);
    }
    if info == 0 {
        panic!("multiboot1: no info_ptr");
    } else {
        verbose!("multiboot1: ptr info = 0x{:x}", info);
    }
    debug!("multiboot1: hello world");
    let mbi = unsafe {
        &*(info as *const header::multiboot_info_t)
    };
    if mbi.flags & header::MULTIBOOT_INFO_BOOTDEV != 0 {
        debug!("- boot device: 0x{:x}", mbi.boot_device);
    }
    debug!("multiboot1 info parsing finished");
}
