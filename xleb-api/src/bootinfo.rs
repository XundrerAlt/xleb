// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 XundrerAlt

pub const BOOTINFO_MAGIC: u32 = 0x4245_4C58;
pub const BOOTINFO_VERSION: u32 = 1;
pub const BOOTINFO_ADDR: u32 = 0x3000_0000;
pub const MAX_MEMORY_REGIONS: usize = 64;
pub const MAX_MODULES: usize = 8;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BootInfo {
    pub magic: u32,
    pub version: u32,
    pub memory_map: MemoryMapInfo,
    pub modules: ModuleInfo,
    pub kernel: KernelInfo,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MemoryMapInfo {
    pub regions: u32,
    pub count: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ModuleInfo {
    pub modules: u32,
    pub count: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ModuleEntry {
    pub start: u32,
    pub end: u32,
    pub cmdline: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KernelInfo {
    pub start: u32,
    pub end: u32,
}
