// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt

use core::arch::asm;

// interrupt manager

#[inline(always)]
pub(crate) unsafe fn enable_interrupts() {
    unsafe {
        asm!("sti", options(nomem, nostack));
    }
}

#[inline(always)]
pub(crate) unsafe fn disable_interrupts() {
    unsafe {
        asm!("cli", options(nomem, nostack));
    }
}

// ports

#[inline(always)]
pub(crate) unsafe fn outb(port: u16, value: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack));
    }
}

#[inline(always)]
pub(crate) unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe {
        asm!("in al, dx", in("dx") port, out("al") value, options(nomem, nostack));
    }
    value
}

// stop CPU

#[inline(always)]
pub(crate) unsafe fn halt() -> ! {
    loop {
        unsafe {
            asm!("hlt", options(nomem, nostack, preserves_flags));
        }
    }
}

// paging

#[inline(always)]
pub unsafe fn read_cr3() -> u32 {
    let cr3: u32;
    unsafe {
        asm!(
            "mov {}, cr3",
            out(reg) cr3,
            options(nomem, nostack, preserves_flags),
        );
    }
    cr3
}

#[inline(always)]
pub unsafe fn write_cr3(val: u32) {
    unsafe { asm!("mov cr3, {}", in(reg) val, options(nostack, preserves_flags)); }
}

#[inline]
pub unsafe fn read_cr2() -> u32 {
    let cr2: u32;
    unsafe {
        core::arch::asm!(
            "mov {}, cr2",
            out(reg) cr2,
            options(nomem, nostack, preserves_flags),
        );
    }
    cr2
}

#[inline]
pub unsafe fn invlpg(virt: u32) {
    unsafe {
        core::arch::asm!(
            "invlpg [{0}]",
             in(reg) virt,
             options(nostack, preserves_flags),
        );
    }
}

// tables

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub(crate) struct DescriptorTablePointer {
    pub limit: u16,
    pub base: u32,
}
