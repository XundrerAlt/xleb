// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#![cfg(xleb_tests)]
use crate::halt;

#[cfg(xleb_tests)]
#[repr(C)]
pub(crate) struct TestEntry {
    pub name: &'static str,
    pub func: fn(),
}

#[cfg(xleb_tests)]
#[macro_export]
macro_rules! xleb_test {
    ($name:ident) => {
        #[cfg(xleb_tests)]
        mod $name {
            #[used]
            #[unsafe(link_section = ".tests")]
            static ENTRY: $crate::test::TestEntry = $crate::test::TestEntry {
                name: stringify!($name),
                func: {
                    fn wrapper() { super::$name() }
                    wrapper
                },
            };
        }
    };
}

#[cfg(xleb_tests)]
pub(crate) fn run_tests() -> ! {
    #[allow(improper_ctypes)]
    unsafe extern "C" {
        static __tests_start: TestEntry;
        static __tests_end: TestEntry;
    }
    let start = unsafe { &__tests_start as *const TestEntry };
    let end = unsafe { &__tests_end as *const TestEntry };
    let count = (end as usize - start as usize) / core::mem::size_of::<TestEntry>();
    crate::info!("Running {} tests", count);
    for i in 0..count {
        let entry = unsafe { &*start.add(i) };
        crate::info!("test {} ...", entry.name);
        (entry.func)();
    }
    crate::info!("All tests are ok. Halting...");
    unsafe { halt(); }
}
