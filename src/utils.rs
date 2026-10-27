// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt

pub(crate) fn align_up(x: usize, a: usize) -> usize {
    (x + a - 1) & !(a - 1)
}
