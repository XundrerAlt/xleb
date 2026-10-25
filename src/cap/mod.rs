// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use xleb_api::Cap;

pub const MAX_CAPS: usize = 64;

pub struct CNode {
    pub slots: [Cap; MAX_CAPS],
    pub parent: *mut CNode,
}
