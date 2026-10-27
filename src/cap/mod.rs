// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use crate::logger::verbose;
use crate::sched::CURRENT_THREAD;
use core::cell::UnsafeCell;
use core::sync::atomic::Ordering;
use xleb_api::CapType;
use xleb_api::SyscallErrno::{EINVAL, EPERM};

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

    pub fn check(&self, type_: CapType, object: usize, size: usize, rights: u32) -> bool {
        if self.type_ != type_ || !self.has(rights) {
            return false;
        }
        if object != 0
            && self.object != object
            && !(self.type_ == CapType::Untyped
                && object >= self.object
                && object < self.object + self.size)
        {
            return false;
        }
        if size != 0 && size > self.size {
            return false;
        }
        true
    }

    pub fn check_tr(&self, ty: CapType, rights: u32) -> bool {
        self.type_ == ty && self.has(rights)
    }
}

#[derive(Debug)]
pub struct CNode {
    slots: UnsafeCell<[Cap; MAX_CAPS]>,
    parent: UnsafeCell<*mut CNode>,
}

unsafe impl Sync for CNode {}
impl CNode {
    pub const fn new() -> Self {
        Self {
            slots: UnsafeCell::new([Cap::NONE; MAX_CAPS]),
            parent: UnsafeCell::new(core::ptr::null_mut()),
        }
    }

    pub fn find_free(&self) -> Option<usize> {
        let slots = unsafe { &*self.slots.get() };
        (0..MAX_CAPS).find(|&i| slots[i].is_none())
    }

    pub fn get(&self, idx: usize) -> Option<&Cap> {
        let slots = unsafe { &*self.slots.get() };
        if idx < MAX_CAPS && !slots[idx].is_none() {
            Some(&slots[idx])
        } else {
            None
        }
    }

    pub unsafe fn get_mut(&self, idx: usize) -> Option<&mut Cap> {
        let slots = unsafe { &mut *self.slots.get() };
        if idx < MAX_CAPS && !slots[idx].is_none() {
            Some(&mut slots[idx])
        } else {
            None
        }
    }

    pub fn check(&self, type_: CapType, object: usize, size: usize, rights: u32) -> Option<&Cap> {
        let slots = unsafe { &*self.slots.get() };
        slots
            .iter()
            .find(|cap| cap.check(type_, object, size, rights))
    }

    pub unsafe fn insert(&self, idx: Option<usize>, cap: Cap) -> Option<usize> {
        let slots = unsafe { &mut *self.slots.get() };
        let i = match idx {
            Some(i) if i < MAX_CAPS => i,
            _ => (0..MAX_CAPS).find(|&i| slots[i].is_none())?,
        };
        slots[i] = cap;
        Some(i)
    }

    pub fn find(&self, type_: CapType, object: usize, size: usize, rights: u32) -> Option<usize> {
        let slots = unsafe { &*self.slots.get() };
        slots
            .iter()
            .position(|cap| cap.check(type_, object, size, rights))
    }

    pub fn parent(&self) -> *mut CNode {
        unsafe { *self.parent.get() }
    }

    pub unsafe fn set_parent(&self, p: *mut CNode) {
        unsafe {
            *self.parent.get() = p;
        }
    }
}

pub(crate) fn get_cap(
    cspace: &CNode,
    idx: usize,
    ty: CapType,
    rights: u32,
    ctx: &str,
) -> Result<Cap, i32> {
    match cspace.get(idx) {
        Some(c) if c.check_tr(ty, rights) => Ok(*c),
        _ => {
            verbose!("{}: idx={} isn't {:?}/rights={:#x}", ctx, idx, ty, rights);
            Err(EINVAL.into())
        }
    }
}

pub(crate) fn current_cspace() -> Result<&'static CNode, i32> {
    let p = CURRENT_THREAD.load(Ordering::Acquire);
    if p.is_null() {
        return Err(EPERM.into());
    }
    Ok(unsafe { &(*p).cspace })
}

pub(crate) unsafe fn commit_untyped(cspace: &CNode, idx: usize, consumed: usize) {
    unsafe {
        if let Some(u) = cspace.get_mut(idx) {
            u.object += consumed;
            u.size -= consumed;
        }
    }
}
