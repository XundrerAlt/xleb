#![no_std]
#![no_main]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod arch;
mod logger;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    crate::fatal!("TODO");
    loop {}
}

#[cfg(test)]
pub(crate) fn test_runner(tests: &[&dyn Fn()]) {
    crate::info!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
