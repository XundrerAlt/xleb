// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
pub(crate) const IDT_ENTRIES: usize = 256;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub(crate) struct IdtEntry {
    offset_low: u16,
    selector: u16,
    zero: u8,
    flags: u8,
    offset_high: u16,
}

impl IdtEntry {
    pub(crate) const fn new(handler: u32, selector: u16, flags: u8) -> Self {
        Self {
            offset_low: (handler & 0xFFFF) as u16,
            selector,
            zero: 0,
            flags,
            offset_high: ((handler >> 16) & 0xFFFF) as u16,
        }
    }
}
