// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use super::InterruptFrame;

#[unsafe(no_mangle)]
pub(crate) extern "C" fn irq_handler(_frame: *const InterruptFrame) {
    todo!();
}
