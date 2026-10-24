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
    pub tid: u32,
    pub state: ThreadState,
    pub stack_top: u32,
    pub stack_size: u32,
    pub entry: u32,
}

const _: () = {
    assert!(core::mem::size_of::<ThreadInfo>() == 20);
    assert!(core::mem::align_of::<ThreadInfo>() == 4);
    assert!(core::mem::offset_of!(ThreadInfo, tid) == 0);
    assert!(core::mem::offset_of!(ThreadInfo, state) == 4);
    assert!(core::mem::offset_of!(ThreadInfo, stack_top) == 8);
    assert!(core::mem::offset_of!(ThreadInfo, stack_size) == 12);
    assert!(core::mem::offset_of!(ThreadInfo, entry) == 16);
    assert!(core::mem::size_of::<ThreadState>() == 4);
    assert!(core::mem::align_of::<ThreadState>() == 4);
};
