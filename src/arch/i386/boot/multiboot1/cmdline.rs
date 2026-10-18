// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use super::header::{MULTIBOOT_INFO_CMDLINE, multiboot_info_t};
use crate::PHYS_TO_VIRT;
use crate::logger::debug;
use core::ffi::CStr;

pub(crate) fn parse(mbi: &multiboot_info_t) {
    if mbi.flags & MULTIBOOT_INFO_CMDLINE == 0 {
        return;
    }
    let addr = PHYS_TO_VIRT!(mbi.cmdline);
    if addr == 0 {
        return;
    }
    let cstr = unsafe { CStr::from_ptr(addr as *const core::ffi::c_char) };
    match cstr.to_str() {
        Ok(s) => debug!("- cmdline: {}", s),
        Err(_) => debug!("- cmdline: <invalid utf-8> {:?}", cstr.to_bytes()),
    }
}
