// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use crate::logger::verbose;
use xleb_api::CapType;

pub const MAX_CAPS: usize = 64;

pub const CAP_READ: u32 = 1 << 0;
pub const CAP_WRITE: u32 = 1 << 1;
pub const CAP_EXEC: u32 = 1 << 2;
pub const CAP_GRANT: u32 = 1 << 3;
pub const CAP_REVOKE: u32 = 1 << 4;
pub const CAP_ALL: u32 = CAP_READ | CAP_WRITE | CAP_EXEC | CAP_GRANT | CAP_REVOKE;

#[derive(Debug, Clone, Copy)]
pub struct Cap {
    pub type_: CapType,
    pub object: usize,
    pub size: usize,
    pub rights: u32,
}

impl Cap {
    pub const NONE: Cap = Cap {
        type_: CapType::None,
        object: 0,
        size: 0,
        rights: 0,
    };

    pub const fn new(type_: CapType, object: usize, size: usize, rights: u32) -> Self {
        Self {
            type_,
            object,
            size,
            rights,
        }
    }

    pub fn is_none(&self) -> bool {
        self.type_ == CapType::None
    }
    pub fn has(&self, rights: u32) -> bool {
        (self.rights & rights) == rights
    }

    pub fn check(&self, type_: CapType, object: usize, rights: u32) -> bool {
        if self.type_ != type_ || !self.has(rights) {
            return false;
        }
        if self.object == object {
            return true;
        }
        if self.type_ == CapType::Untyped
            && object >= self.object
            && object < self.object + self.size
        {
            return true;
        }
        false
    }
}

#[derive(Debug)]
pub struct CNode {
    pub slots: [Cap; MAX_CAPS],
    pub parent: *mut CNode,
}

impl CNode {
    pub const fn new() -> Self {
        Self {
            slots: [Cap::NONE; MAX_CAPS],
            parent: core::ptr::null_mut(),
        }
    }

    pub fn find_free(&self) -> Option<usize> {
        (0..MAX_CAPS).find(|&i| self.slots[i].is_none())
    }

    pub fn get(&self, idx: usize) -> Option<&Cap> {
        if idx < MAX_CAPS && !self.slots[idx].is_none() {
            Some(&self.slots[idx])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut Cap> {
        if idx < MAX_CAPS && !self.slots[idx].is_none() {
            Some(&mut self.slots[idx])
        } else {
            None
        }
    }

    pub fn check(&self, type_: CapType, object: usize, rights: u32) -> Option<&Cap> {
        self.slots
            .iter()
            .find(|cap| cap.check(type_, object, rights))
    }

    pub fn insert(&mut self, idx: Option<usize>, cap: Cap) -> Option<usize> {
        let i = match idx {
            Some(i) if i < MAX_CAPS => i,
            _ => self.find_free()?,
        };
        self.slots[i] = cap;
        Some(i)
    }

    pub fn find(&self, type_: CapType, object: usize, rights: u32) -> Option<usize> {
        self.slots
            .iter()
            .position(|cap| cap.check(type_, object, rights))
    }
}
