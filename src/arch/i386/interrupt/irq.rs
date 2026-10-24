// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use super::InterruptFrame;
use super::pic;
use crate::logger::verbose;
use core::sync::atomic::{AtomicPtr, Ordering};

pub type IrqHandler = fn(&InterruptFrame);

static IRQ_HANDLERS: [AtomicPtr<()>; 16] = [const { AtomicPtr::new(core::ptr::null_mut()) }; 16];

pub fn register(irq: u8, h: IrqHandler) {
    assert!((irq as usize) < 16);
    IRQ_HANDLERS[irq as usize].store(h as *mut (), Ordering::Release);
    unsafe {
        pic::unmask(irq);
    }
    verbose!("IRQ {} registered (to 0x{:x})", irq, h as usize);
}

fn dispatch(irq: u8, frame: &InterruptFrame) {
    let p = IRQ_HANDLERS[irq as usize].load(Ordering::Acquire);
    if !p.is_null() {
        let h: IrqHandler = unsafe { core::mem::transmute(p) };
        h(frame);
    }
}

#[unsafe(no_mangle)]
pub(crate) extern "C" fn irq_handler(frame: *const InterruptFrame) {
    let frame = unsafe { &*frame };
    if frame.int_no < 0x20 || frame.int_no >= 0x30 {
        return;
    }
    let irq = (frame.int_no - 0x20) as u8;
    dispatch(irq, frame);
    unsafe {
        pic::send_eoi(irq);
    }
}
