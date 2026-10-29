// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 XundrerAlt
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallNum {
    // Threads
    ThreadSuspend = 0,
    ThreadConf = 1,
    ThreadStart = 2,

    // Capabilities
    CapFind = 16,
    CapRetype = 17,

    // Memory
    FrameMap = 32,

    // x86 Ports
    x86_PortIo = 48,
    x86_Derive_PortIo = 49,
}

impl TryFrom<u32> for SyscallNum {
    type Error = ();
    fn try_from(nr: u32) -> Result<SyscallNum, ()> {
        match nr {
            0 => Ok(SyscallNum::ThreadSuspend),
            1 => Ok(SyscallNum::ThreadConf),
            2 => Ok(SyscallNum::ThreadStart),
            16 => Ok(SyscallNum::CapFind),
            17 => Ok(SyscallNum::CapRetype),
            32 => Ok(SyscallNum::FrameMap),
            48 => Ok(SyscallNum::x86_PortIo),
            49 => Ok(SyscallNum::x86_Derive_PortIo),
            _  => Err(()),
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
