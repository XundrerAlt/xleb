// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#[unsafe(link_section = ".multiboot")]
#[unsafe(no_mangle)]
#[used]
pub static MULTIBOOT1_HEADER: [u32; 3] = [
    0x1BADB002,
0x00000003,
-(0x1BADB002u32.wrapping_add(0x00000003) as i32) as u32,
];
