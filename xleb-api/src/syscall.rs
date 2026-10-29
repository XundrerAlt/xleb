// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 XundrerAlt
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallNum {
    ThreadSuspend = 0,
    CapFind = 1,
    CapRetype = 2,
    FrameMap = 3,
    ThreadConf = 4,
    ThreadStart = 5,
    x86_PortIo = 6,
    x86_Derive_PortIo = 7,
}

impl TryFrom<u32> for SyscallNum {
    type Error = ();
    fn try_from(nr: u32) -> Result<SyscallNum, ()> {
        match nr {
            0 => Ok(SyscallNum::ThreadSuspend),
            1 => Ok(SyscallNum::CapFind),
            2 => Ok(SyscallNum::CapRetype),
            3 => Ok(SyscallNum::FrameMap),
            4 => Ok(SyscallNum::ThreadConf),
            5 => Ok(SyscallNum::ThreadStart),
            6 => Ok(SyscallNum::x86_PortIo),
            7 => Ok(SyscallNum::x86_Derive_PortIo),
            _ => Err(()),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallErrno {
    SUCCESS = 0,
    EPERM = -1,
    ENOENT = -2,
    ENOMEM = -12,
    EFAULT = -14,
    EINVAL = -22,
    ENOSYS = -38,
}

impl From<SyscallErrno> for i32 {
    fn from(e: SyscallErrno) -> i32 { e as i32 }
}
