// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
pub(crate) mod bootinfo;
mod multiboot1;
pub(crate) mod tss;
use core::arch::global_asm;
pub(crate) use multiboot1::mmap;
pub(crate) use multiboot1::modules;

global_asm!(include_str!("entry.S"), options(att_syntax),); // asm because it sets stack ptr

global_asm!(include_str!("gdt.S"), options(att_syntax),); // TODO: rewrite to Rust

pub(crate) unsafe fn init() {
    multiboot1::init();
    tss::init();
}
