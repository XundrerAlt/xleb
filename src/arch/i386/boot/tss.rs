// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt

use core::mem::size_of;
use crate::logger::debug;

#[repr(C, packed)]
pub struct TssEntry {
    pub prev_tss: u32,
    pub esp0: u32,
    pub ss0: u32,
    pub esp1: u32,
    pub ss1: u32,
    pub esp2: u32,
    pub ss2: u32,
    pub cr3: u32,
    pub eip: u32,
    pub eflags: u32,
    pub eax: u32,
    pub ecx: u32,
    pub edx: u32,
    pub ebx: u32,
    pub esp: u32,
    pub ebp: u32,
    pub esi: u32,
    pub edi: u32,
    pub es: u32,
    pub cs: u32,
    pub ss: u32,
    pub ds: u32,
    pub fs: u32,
    pub gs: u32,
    pub ldt: u32,
    pub trap: u16,
    pub iomap_base: u16,
}

const _: () = assert!(size_of::<TssEntry>() == 104);

pub static mut TSS: TssEntry = TssEntry {
    prev_tss: 0,
    esp0: 0,
    ss0: 0,
    esp1: 0,
    ss1: 0,
    esp2: 0,
    ss2: 0,
    cr3: 0,
    eip: 0,
    eflags: 0,
    eax: 0,
    ecx: 0,
    edx: 0,
    ebx: 0,
    esp: 0,
    ebp: 0,
    esi: 0,
    edi: 0,
    es: 0,
    cs: 0,
    ss: 0,
    ds: 0,
    fs: 0,
    gs: 0,
    ldt: 0,
    trap: 0,
    iomap_base: 0,
};

const KERNEL_DS: u32 = 0x10;

unsafe extern "C" {
    static stack_top: u32;
    static gdt: u8;
}

#[inline]
pub unsafe fn tss_mut() -> &'static mut TssEntry {
    unsafe { &mut *core::ptr::addr_of_mut!(TSS) }
}

pub fn init() {
    let tss = unsafe { tss_mut() };
    unsafe {
        core::ptr::write_bytes(
            tss as *mut TssEntry as *mut u8,
            0,
            size_of::<TssEntry>(),
        );
    }
    tss.ss0 = KERNEL_DS;
    tss.esp0 = core::ptr::addr_of!(stack_top) as u32;
    tss.iomap_base = size_of::<TssEntry>() as u16;

    unsafe {
        install_gdt_descriptor(core::ptr::addr_of!(gdt) as u32, 0x28);
        load_tr(0x28);
    }
    debug!("TSS: hello world");
}

unsafe fn install_gdt_descriptor(gdt_base: u32, selector: u16) {
    let base = core::ptr::addr_of!(TSS) as u32;
    let limit = (size_of::<TssEntry>() - 1) as u32;
    let desc = (gdt_base + (selector as u32 & !7)) as *mut u8;
    unsafe {
        core::ptr::write_unaligned(desc.add(0) as *mut u16, limit as u16);
        core::ptr::write_unaligned(desc.add(2) as *mut u16, base as u16);
        *desc.add(4) = (base >> 16) as u8;
        *desc.add(5) = 0x89;
        *desc.add(6) = 0x00;
        *desc.add(7) = (base >> 24) as u8;
    }
}

#[inline]
unsafe fn load_tr(selector: u16) {
    unsafe {
        core::arch::asm!(
            "ltr {0:x}",
            in(reg) selector,
            options(nostack, preserves_flags),
        );
    }
}

pub fn set_stack(esp0: u32) {
    let tss = unsafe { tss_mut() };
    tss.esp0 = esp0;
}
