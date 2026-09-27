// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
mod idt;
mod isr;
mod irq;
mod pic;
mod stub;

use idt::{IdtEntry,IDT_ENTRIES};
use core::arch::asm;
use crate::arch::cpu::{enable_interrupts,DescriptorTablePointer};
use crate::logger::{debug,verbose};
#[cfg(xleb_tests)]
use crate::xleb_test;

static mut IDT: [IdtEntry; IDT_ENTRIES] = [IdtEntry::new(0, 0, 0); IDT_ENTRIES];

#[repr(C)]
pub(crate) struct InterruptFrame {
    pub gs: u32, pub fs: u32, pub es: u32, pub ds: u32,
    pub edi: u32, pub esi: u32, pub ebp: u32, pub esp: u32, pub ebx: u32, pub edx: u32, pub ecx: u32, pub eax: u32,
    pub int_no: u32, pub err_code: u32,
    pub eip: u32, pub cs: u32, pub eflags: u32, pub useresp: u32, pub ss: u32,
}
#[inline(always)]
unsafe fn lidt(ptr: &DescriptorTablePointer) {
    unsafe { asm!("lidt [{}]", in(reg) ptr, options(readonly, nostack, preserves_flags)) };
}

pub(crate) unsafe fn init() {
    unsafe {
        verbose!("filling entries");
        for i in 0..IDT_ENTRIES {
            IDT[i] = IdtEntry::new(0, 0x08, 0x8E);
        }
        for i in 0..32 {
            IDT[i] = IdtEntry::new(ISR_TABLE[i], 0x08, 0x8E);
        }
        let ptr = DescriptorTablePointer {
            limit: (core::mem::size_of::<[IdtEntry; IDT_ENTRIES]>() - 1) as u16,
            base: &raw const IDT as u32,
        };
        pic::remap(0x20, 0x28);
        pic::mask_all();
        lidt(&ptr);
        verbose!("idt loaded");
        enable_interrupts();
        verbose!("interrupts enabled");
    }
    debug!("interrupt: hello world");
}

unsafe extern "C" {
    #[link_name = "isr_table"]
    static ISR_TABLE: [u32; 32];
    #[link_name = "irq_table"]
    static IRQ_TABLE: [u32; 16];
}

#[cfg(xleb_tests)]
fn isr_test() {
    unsafe {
        asm!("int $0x3");
    }
}

#[cfg(xleb_tests)]
xleb_test!(isr_test);
