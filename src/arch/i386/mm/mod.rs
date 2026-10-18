// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt

#[macro_export]
macro_rules! PHYS_TO_VIRT {
    ($addr:expr) => {
        (($addr) as usize + 0xC000_0000) // 0xC000_0000 is kernel base
    };
}

#[macro_export]
macro_rules! VIRT_TO_PHYS {
    ($addr:expr) => {
        (($addr) as usize - 0xC000_0000)
    };
}
