// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#![no_std]
#![no_main]
#![cfg_attr(xleb_tests, feature(custom_test_frameworks))]
#![cfg_attr(xleb_tests, test_runner(crate::test_runner))]
#![cfg_attr(xleb_tests, reexport_test_harness_main = "test_main")]

mod arch;
mod logger;
mod sched;

#[allow(unused)]
mod tests;

use crate::arch::cpu::halt;
use crate::logger::fatal;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    fatal!("{}", info);
    unsafe {
        halt();
    }
}
