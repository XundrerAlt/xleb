// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use crate::arch::cpu::outb;
use crate::arch::interrupt;
use crate::logger::{debug, verbose};
use crate::sched;
use core::sync::atomic::{AtomicU32, Ordering};

static TICKS: AtomicU32 = AtomicU32::new(0);
pub const TICK_HZ: u32 = 100;
const PIT_BASE_HZ: u32 = 1_193_182;

const _: () = assert!(TICK_HZ > 0);
const _: () = assert!(PIT_BASE_HZ / TICK_HZ < 65536);

pub(crate) fn ticks() -> u32 {
    TICKS.load(Ordering::Relaxed)
}

fn on_tick(_frame: &interrupt::InterruptFrame) {
    TICKS.fetch_add(1, Ordering::Relaxed);
    unsafe {
        sched::yield_now();
    }
}

pub(crate) unsafe fn init() {
    let divisor = (PIT_BASE_HZ / TICK_HZ) as u16;
    unsafe {
        outb(0x43, 0x36);
        outb(0x40, (divisor & 0xFF) as u8);
        outb(0x40, (divisor >> 8) as u8);
    }
    interrupt::irq::register(0, on_tick);
    debug!("timer: (PIT) hello world");
}
