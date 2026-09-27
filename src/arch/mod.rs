// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#[cfg(target_arch = "x86")]
mod i386;
#[cfg(target_arch = "x86")]
pub(crate) use i386::*;
