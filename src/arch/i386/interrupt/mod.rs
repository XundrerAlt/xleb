// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
mod idt;
pub(crate) mod irq;
mod isr;
mod pic;
mod stub;
mod syscall;

use crate::arch::cpu::{DescriptorTablePointer, enable_interrupts};
use crate::logger::{debug, verbose};
#[cfg(xleb_tests)]
use crate::xleb_test;
use core::arch::asm;
use idt::{IDT_ENTRIES, IdtEntry};

static mut IDT: [IdtEntry; IDT_ENTRIES] = [IdtEntry::new(0, 0, 0); IDT_ENTRIES];

#[repr(C)]
pub(crate) struct InterruptFrame {
    pub gs: u32,
    pub fs: u32,
    pub es: u32,
    pub ds: u32,
    pub edi: u32,
    pub esi: u32,
    pub ebp: u32,
    pub esp: u32,
    pub ebx: u32,
    pub edx: u32,
    pub ecx: u32,
    pub eax: u32,
    pub int_no: u32,
    pub err_code: u32,
    pub eip: u32,
    pub cs: u32,
    pub eflags: u32,
    pub useresp: u32,
    pub ss: u32,
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
        verbose!("isr loaded");
        for i in 0..16 {
            IDT[i + 32] = IdtEntry::new(IRQ_TABLE[i], 0x08, 0x8E);
        }
        verbose!("irq loaded");
        IDT[0x80] = IdtEntry::new(syscall_entry as *const () as u32, 0x08, 0xEE);
        verbose!("syscall initializated");
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
    fn syscall_entry();
}

#[cfg(xleb_tests)]
fn isr_test() {
    unsafe {
        asm!("int $0x3");
    }
}

#[cfg(xleb_tests)]
xleb_test!(isr_test);
