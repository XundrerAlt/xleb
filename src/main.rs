// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#![no_std]
#![no_main]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod arch;
mod logger;

use arch::cpu::halt;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crate::fatal!("{}", info);
    unsafe { halt(); }
}

#[cfg(test)]
pub(crate) fn test_runner(tests: &[&dyn Fn()]) {
    crate::info!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
