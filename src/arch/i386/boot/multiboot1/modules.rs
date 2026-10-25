// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use super::header;
use crate::logger::{info, warning};
use xleb_api::MAX_MODULES;

pub static mut MODULES: [(u32, u32); MAX_MODULES] = [(0, 0); MAX_MODULES];
pub static mut MODULE_COUNT: usize = 0;

fn process_single(i: usize, m: &header::module_t) {
    info!(
        "  - module {}: {:#x}..{:#x} ({} bytes)",
        i,
        m.mod_start,
        m.mod_end,
        m.mod_end - m.mod_start
    );
    unsafe {
        if MODULE_COUNT < MAX_MODULES {
            MODULES[MODULE_COUNT] = (m.mod_start, m.mod_end);
            MODULE_COUNT += 1;
        } else {
            warning!("too many modules (max {})", MAX_MODULES);
        }
    }
}

pub(crate) unsafe fn process(mbi: &header::multiboot_info_t) {
    if mbi.flags & header::MULTIBOOT_INFO_MODS == 0 || mbi.mods_count == 0 {
        warning!("- no modules");
        return;
    }
    info!("- modules: {}", mbi.mods_count);
    let mods = unsafe {
        core::slice::from_raw_parts(
            crate::PHYS_TO_VIRT!(mbi.mods_addr) as *const header::module_t,
            mbi.mods_count as usize,
        )
    };
    for (i, m) in mods.iter().enumerate() {
        process_single(i, m);
    }
}

pub(crate) fn all() -> Option<&'static [(u32, u32)]> {
    unsafe {
        let count = core::ptr::addr_of!(MODULE_COUNT).read();
        if count == 0 {
            return None;
        }
        let ptr = core::ptr::addr_of!(MODULES) as *const (u32, u32);
        Some(core::slice::from_raw_parts(ptr, count))
    }
}

pub(crate) fn get(index: usize) -> Option<(u32, u32)> {
    unsafe {
        let count = core::ptr::addr_of!(MODULE_COUNT).read();
        if index >= count {
            return None;
        }
        let ptr = core::ptr::addr_of!(MODULES) as *const (u32, u32);
        Some(ptr.add(index).read())
    }
}
