// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt

#[macro_export]
macro_rules! VIRT_TO_PHYS {
    ($addr:expr) => {{
        let a = ($addr) as u32;
        if a >= 0xC000_0000 { a - 0xC000_0000 } else { a }
    }};
}

#[macro_export]
macro_rules! PHYS_TO_VIRT {
    ($addr:expr) => {{
        let a = ($addr) as u32;
        if a < 0xC000_0000 { a + 0xC000_0000 } else { a }
    }};
}
