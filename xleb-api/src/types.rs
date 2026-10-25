// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 XundrerAlt
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadState {
    Ready = 0,
    Running = 1,
    Dead = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ThreadInfo {
    pub stack_top: u32,
    pub stack_size: u32,
    pub entry: u32,
}

const _: () = {
    assert!(core::mem::size_of::<ThreadInfo>() == 12);
    assert!(core::mem::align_of::<ThreadInfo>() == 4);
    assert!(core::mem::offset_of!(ThreadInfo, stack_top) == 0);
    assert!(core::mem::offset_of!(ThreadInfo, stack_size) == 4);
    assert!(core::mem::offset_of!(ThreadInfo, entry) == 8);
    assert!(core::mem::size_of::<ThreadState>() == 4);
    assert!(core::mem::align_of::<ThreadState>() == 4);
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapType {
    None = 0,
    CNode = 1,
    Untyped = 2,
    Page = 3,
}

impl TryFrom<u32> for CapType {
    type Error = ();
    fn try_from(v: u32) -> Result<CapType, ()> {
        match v {
            0 => Ok(CapType::None),
            1 => Ok(CapType::CNode),
            2 => Ok(CapType::Untyped),
            3 => Ok(CapType::Page),
            _ => Err(()),
        }
    }
}
