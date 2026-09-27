// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use crate::arch::cpu::{inb, outb};
use crate::logger::{debug, verbose};

const PIC1: u16 = 0x20;
const PIC2: u16 = 0xA0;
const PIC1_CMD: u16 = PIC1;
const PIC1_DATA: u16 = PIC1 + 1;
const PIC2_CMD: u16 = PIC2;
const PIC2_DATA: u16 = PIC2 + 1;

const ICW1_INIT: u8 = 0x11;
const ICW4_8086: u8 = 0x01;
const PIC_EOI: u8 = 0x20;

#[inline(always)]
unsafe fn io_wait() {
    unsafe { outb(0x80, 0) };
}

pub(crate) unsafe fn remap(offset1: u8, offset2: u8) {
    unsafe {
        let _mask1 = inb(PIC1_DATA);
        let _mask2 = inb(PIC2_DATA);

        outb(PIC1_CMD, ICW1_INIT); io_wait();
        outb(PIC2_CMD, ICW1_INIT); io_wait();

        outb(PIC1_DATA, offset1); io_wait();
        outb(PIC2_DATA, offset2); io_wait();

        outb(PIC1_DATA, 0x04); io_wait();
        outb(PIC2_DATA, 0x02); io_wait();

        outb(PIC1_DATA, ICW4_8086); io_wait();
        outb(PIC2_DATA, ICW4_8086); io_wait();

        outb(PIC1_DATA, 0xFF);
        outb(PIC2_DATA, 0xFF);
    }
    debug!("pic: remapped, master={:#x} slave={:#x}", offset1, offset2);
}

pub(crate) unsafe fn mask_all() {
    unsafe {
        outb(PIC1_DATA, 0xFF);
        outb(PIC2_DATA, 0xFF);
    }
    verbose!("all irq masked");
}

pub(crate) unsafe fn unmask(irq: u8) {
    unsafe {
        let (port, line) = if irq < 8 {
            (PIC1_DATA, irq)
        } else {
            (PIC2_DATA, irq - 8)
        };
        let mask = inb(port);
        outb(port, mask & !(1 << line));
    }
    verbose!("irq {} unmasked", irq);
}

pub(crate) unsafe fn mask(irq: u8) {
    unsafe {
        let (port, line) = if irq < 8 {
            (PIC1_DATA, irq)
        } else {
            (PIC2_DATA, irq - 8)
        };
        let mask = inb(port);
        outb(port, mask | (1 << line));
    }
    verbose!("irq {} masked", irq);
}

pub(crate) unsafe fn send_eoi(irq: u8) {
    unsafe {
        if irq >= 8 {
            outb(PIC2_CMD, PIC_EOI);
        }
        outb(PIC1_CMD, PIC_EOI);
    }
}
