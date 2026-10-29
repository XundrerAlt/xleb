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
            _ => Err(()),
        }
    }
}
