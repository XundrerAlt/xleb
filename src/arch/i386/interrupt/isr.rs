// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use super::InterruptFrame;
use crate::arch::thread::ThreadState;
use crate::logger::{debug, error, fatal, verbose};
use crate::sched::{current, yield_now};

const EXCEPTION_NAMES: [&str; 32] = [
    "Division by Zero",
    "Debug",
    "Non-Maskable Interrupt",
    "Breakpoint",
    "Overflow",
    "Bound Range Exceeded",
    "Invalid Opcode",
    "Device Not Available",
    "Double Fault",
    "Coprocessor Segment Overrun",
    "Invalid TSS",
    "Segment Not Present",
    "Stack Segment Fault",
    "General Protection Fault",
    "Page Fault",
    "Reserved",
    "x87 FPU Error",
    "Alignment Check",
    "Machine Check",
    "SIMD Exception",
    "Virtualization Exception",
    "Control Protection Exception",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Hypervisor Injection Exception",
    "VMM Communication Exception",
    "Security Exception",
    "Reserved",
];

#[inline]
fn cpl(cs: u32) -> u8 {
    (cs & 3) as u8
}

#[unsafe(no_mangle)]
pub(crate) extern "C" fn isr_handler(frame: *const InterruptFrame) {
    let frame = unsafe { &*frame };
    if frame.int_no >= 32 {
        error!("isr: vector {} out of range", frame.int_no);
        return;
    }
    let name = EXCEPTION_NAMES[frame.int_no as usize];

    let ring = cpl(frame.cs);
    if ring == 3 {
        verbose!(
            "user exception {}: {} (err={:#x})",
            frame.int_no,
            name,
            frame.err_code
        );
        if frame.int_no == 14 {
            let cr2 = unsafe { crate::arch::cpu::read_cr2() };
            verbose!("cr2={:#x}", cr2);
        }
        verbose!(
            "eax={:#x} ebx={:#x} ecx={:#x} edx={:#x}",
            frame.eax,
            frame.ebx,
            frame.ecx,
            frame.edx
        );
        verbose!(
            "eip={:#x} cs={:#x} eflags={:#x}",
            frame.eip,
            frame.cs,
            frame.eflags
        );
        verbose!("suspending current thread...");
        let th = current();
        unsafe {
            (*th).state = ThreadState::Inactive;
            yield_now();
        }
        unreachable!("scheduler returned to Inactive thread");
        return;
    }

    match frame.int_no {
        1 | 3 => {
            debug!("exception {}: {}", frame.int_no, name);
            debug!(
                "eax={:#x} ebx={:#x} ecx={:#x} edx={:#x}",
                frame.eax, frame.ebx, frame.ecx, frame.edx
            );
            debug!(
                "eip={:#x} cs={:#x} eflags={:#x}",
                frame.eip, frame.cs, frame.eflags
            );
        }
        14 => {
            let cr2 = unsafe { crate::arch::cpu::read_cr2() };
            fatal!(
                "exception {}: {} (err={:#x})",
                frame.int_no,
                name,
                frame.err_code
            );
            fatal!("cr2={:#x}", cr2);
            fatal!(
                "eax={:#x} ebx={:#x} ecx={:#x} edx={:#x}",
                frame.eax,
                frame.ebx,
                frame.ecx,
                frame.edx
            );
            fatal!(
                "eip={:#x} cs={:#x} eflags={:#x}",
                frame.eip,
                frame.cs,
                frame.eflags
            );
            panic!("this's fatal exception.");
        }
        _ => {
            fatal!(
                "exception {}: {} (err={:#x})",
                frame.int_no,
                name,
                frame.err_code
            );
            fatal!(
                "eax={:#x} ebx={:#x} ecx={:#x} edx={:#x}",
                frame.eax,
                frame.ebx,
                frame.ecx,
                frame.edx
            );
            fatal!(
                "eip={:#x} cs={:#x} eflags={:#x}",
                frame.eip,
                frame.cs,
                frame.eflags
            );
            panic!("this's fatal exception.");
        }
    }
}
