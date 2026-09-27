// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
const COM1: u16 = 0x3F8;
use crate::arch::cpu::{inb,outb};

#[unsafe(no_mangle)]
pub extern "C" fn uart_putc(c: u8) {
    unsafe {
        while (inb(COM1 + 5) & (1 << 5)) == 0 {
            core::hint::spin_loop();
        }
        if c == b'\n' {
            uart_putc(b'\r');
        }
        outb(COM1, c);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn uart_puts(s: *const u8) {
    if s.is_null() {
        return;
    }
    let mut p = s;
    unsafe {
        while *p != 0 {
            uart_putc(*p);
            p = p.add(1);
        }
    }
}
