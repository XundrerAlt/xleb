// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use crate::logger::verbose;

pub const MAX_CAPS: usize = 64;

pub const CAP_READ: u32 = 1 << 0;
pub const CAP_WRITE: u32 = 1 << 1;
pub const CAP_EXEC: u32 = 1 << 2;
pub const CAP_GRANT: u32 = 1 << 3;
pub const CAP_REVOKE: u32 = 1 << 4;
pub const CAP_ALL: u32 = CAP_READ | CAP_WRITE | CAP_EXEC | CAP_GRANT | CAP_REVOKE;

const WILDCARD: u32 = u32::MAX; // TODO: delete it

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapType {
    None,
    CNode,
    Untyped,
}

#[derive(Debug)]
pub struct Cap {
    pub type_: CapType,
    pub object: u32,
    pub rights: u32,
}

impl Cap {
    pub const NONE: Cap = Cap {
        type_: CapType::None,
        object: 0,
        rights: 0,
    };

    pub const fn new(type_: CapType, object: u32, rights: u32) -> Self {
        Self {
            type_,
            object,
            rights,
        }
    }

    pub fn is_none(&self) -> bool {
        self.type_ == CapType::None
    }

    pub fn has(&self, rights: u32) -> bool {
        (self.rights & rights) == rights
    }

    pub fn check(&self, type_: CapType, object: u32, rights: u32) -> bool {
        self.type_ == type_
            && (self.object == object || self.object == WILDCARD)
            && self.has(rights)
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

    pub fn check(&self, type_: CapType, object: u32, rights: u32) -> Option<&Cap> {
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

    pub unsafe fn grant_all(&mut self) {
        let self_addr = self as *mut CNode as u32;
        verbose!("CNode::grant_all: begin (self_addr={:#x})", self_addr);
        // TODO: remove wildcards
        self.insert(Some(0), Cap::new(CapType::Untyped, u32::MAX, CAP_ALL));
        self.insert(Some(1), Cap::new(CapType::CNode, self_addr, CAP_ALL));
        verbose!("CNode::grant_all: end");
    }
}
