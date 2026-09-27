// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#[cfg(xleb_tests)]
use crate::test::run_tests;
#[cfg(xleb_tests)]
use crate::xleb_test;

#[unsafe(no_mangle)]
pub extern "C" fn kinit() -> ! {
    crate::info!("xleb: hello world");
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
    crate::info!("test of test");
}

#[cfg(xleb_tests)]
xleb_test!(test_test);
