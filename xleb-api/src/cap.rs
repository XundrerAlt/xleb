// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 XundrerAlt

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapType {
    None = 0,
    CNode = 1,
    Untyped = 2,
    Frame = 3,
    PageTable = 4,
    Thread = 5,
    PageSpace = 6,
    x86_PortIo_Cap = 7,
    x86_PortIoControl_Cap = 8,
}

impl TryFrom<u32> for CapType {
    type Error = ();
    fn try_from(v: u32) -> Result<CapType, ()> {
        match v {
            0 => Ok(CapType::None),
            1 => Ok(CapType::CNode),
            2 => Ok(CapType::Untyped),
            3 => Ok(CapType::Frame),
            4 => Ok(CapType::PageTable),
            5 => Ok(CapType::Thread),
            6 => Ok(CapType::PageSpace),
            7 => Ok(CapType::x86_PortIo_Cap),
            8 => Ok(CapType::x86_PortIoControl_Cap),
            _ => Err(()),
        }
    }
}

pub const CAP_READ: u32 = 1 << 0;
pub const CAP_WRITE: u32 = 1 << 1;
pub const CAP_EXEC: u32 = 1 << 2;
pub const CAP_GRANT: u32 = 1 << 3;
pub const CAP_REVOKE: u32 = 1 << 4;
pub const CAP_ALL: u32 = CAP_READ | CAP_WRITE | CAP_EXEC | CAP_GRANT | CAP_REVOKE;
