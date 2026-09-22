#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

mod kheap;
mod kernel_alloc;

use core::panic::PanicInfo;
use alloc::boxed::Box;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_box_test() -> u32 {
    let b = Box::new(42u32);
    *b
}
