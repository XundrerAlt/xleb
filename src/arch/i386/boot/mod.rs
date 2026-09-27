// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
mod multiboot1;
use core::arch::global_asm;

global_asm!(
    include_str!("entry.S"),
    options(att_syntax),
); // asm because it sets stack ptr

global_asm!(
    include_str!("gdt.S"),
    options(att_syntax),
); // TODO: rewrite to Rust
