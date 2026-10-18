// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#[cfg(xleb_tests)]
use crate::test::run_tests;
#[cfg(xleb_tests)]
use crate::xleb_test;

use crate::arch::boot;
use crate::arch::interrupt;
use crate::arch::timer;
use crate::sched;
use crate::logger::info;

#[unsafe(no_mangle)]
pub extern "C" fn kinit() -> ! {
    info!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"),);
    unsafe {
        boot::init();
        interrupt::init();
        timer::init();
        sched::init();
    }
    #[cfg(xleb_tests)]
    {
        run_tests();
    }
    #[cfg(not(xleb_tests))]
    {
        todo!();
    }
}

#[cfg(xleb_tests)]
fn test_test() {
    info!("test of test");
}

#[cfg(xleb_tests)]
xleb_test!(test_test);
