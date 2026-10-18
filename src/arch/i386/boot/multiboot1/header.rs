// multiboot.h — the header for Multiboot
// Copyright (C) 1999, 2001  Free Software Foundation, Inc.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
//
// Translated from multiboot.h (GRUB manual).

#![allow(dead_code)]

// ---------------------------------------------------------------------------
// Macros
// ---------------------------------------------------------------------------

/// How many bytes from the start of the file we search for the header.
pub const MULTIBOOT_SEARCH: usize = 8192;

/// Alignment of the multiboot header.
pub const MULTIBOOT_HEADER_ALIGN: usize = 4;

/// The magic field should contain this.
pub const MULTIBOOT_HEADER_MAGIC: u32 = 0x1BADB002;

/// This should be in %eax.
pub const MULTIBOOT_BOOTLOADER_MAGIC: u32 = 0x2BADB002;

/// Alignment of multiboot modules.
pub const MULTIBOOT_MOD_ALIGN: u32 = 0x0000_1000;

/// Alignment of the multiboot info structure.
pub const MULTIBOOT_INFO_ALIGN: u32 = 0x0000_0004;

/// Align all boot modules on i386 page (4KB) boundaries.
pub const MULTIBOOT_PAGE_ALIGN: u32 = 0x0000_0001;

/// Must pass memory information to OS.
pub const MULTIBOOT_MEMORY_INFO: u32 = 0x0000_0002;

/// Must pass video information to OS.
pub const MULTIBOOT_VIDEO_MODE: u32 = 0x0000_0004;

/// This flag indicates the use of the address fields in the header.
pub const MULTIBOOT_AOUT_KLUDGE: u32 = 0x0001_0000;

/// Is there basic lower/upper memory information?
pub const MULTIBOOT_INFO_MEMORY: u32 = 0x0000_0001;

/// Is there a boot device set?
pub const MULTIBOOT_INFO_BOOTDEV: u32 = 0x0000_0002;

/// Is the command-line defined?
pub const MULTIBOOT_INFO_CMDLINE: u32 = 0x0000_0004;

/// Are there modules to do something with?
pub const MULTIBOOT_INFO_MODS: u32 = 0x0000_0008;

// These next two are mutually exclusive.

/// Is there a symbol table loaded?
pub const MULTIBOOT_INFO_AOUT_SYMS: u32 = 0x0000_0010;

/// Is there an ELF section header table?
pub const MULTIBOOT_INFO_ELF_SHDR: u32 = 0x0000_0020;

/// Is there a full memory map?
pub const MULTIBOOT_INFO_MEM_MAP: u32 = 0x0000_0040;

/// Is there drive info?
pub const MULTIBOOT_INFO_DRIVE_INFO: u32 = 0x0000_0080;

/// Is there a config table?
pub const MULTIBOOT_INFO_CONFIG_TABLE: u32 = 0x0000_0100;

/// Is there a boot loader name?
pub const MULTIBOOT_INFO_BOOT_LOADER_NAME: u32 = 0x0000_0200;

/// Is there an APM table?
pub const MULTIBOOT_INFO_APM_TABLE: u32 = 0x0000_0400;

/// Is there video information?
pub const MULTIBOOT_INFO_VBE_INFO: u32 = 0x0000_0800;
pub const MULTIBOOT_INFO_FRAMEBUFFER_INFO: u32 = 0x0000_1000;

/// Memory available to the OS.
pub const MULTIBOOT_MEMORY_AVAILABLE: u32 = 1;

/// Reserved memory (not usable).
pub const MULTIBOOT_MEMORY_RESERVED: u32 = 2;

/// ACPI reclaimable memory.
pub const MULTIBOOT_MEMORY_ACPI_RECLAIMABLE: u32 = 3;

/// ACPI NVS memory.
pub const MULTIBOOT_MEMORY_NVS: u32 = 4;

/// Bad RAM (marked as unusable by firmware).
pub const MULTIBOOT_MEMORY_BADRAM: u32 = 5;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// The Multiboot header.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct multiboot_header_t {
    pub magic: u32,
    pub flags: u32,
    pub checksum: u32,
    pub header_addr: u32,
    pub load_addr: u32,
    pub load_end_addr: u32,
    pub bss_end_addr: u32,
    pub entry_addr: u32,
}

/// The symbol table for a.out.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct aout_symbol_table_t {
    pub tabsize: u32,
    pub strsize: u32,
    pub addr: u32,
    pub reserved: u32,
}

/// The section header table for ELF.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct elf_section_header_table_t {
    pub num: u32,
    pub size: u32,
    pub addr: u32,
    pub shndx: u32,
}

/* The Multiboot information (union part). */
#[repr(C)]
#[derive(Clone, Copy)]
pub union multiboot_info_u {
    pub aout_sym: aout_symbol_table_t,
    pub elf_sec: elf_section_header_table_t,
}

/// The Multiboot information.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct multiboot_info_t {
    pub flags: u32,
    pub mem_lower: u32,
    pub mem_upper: u32,
    pub boot_device: u32,
    pub cmdline: u32,
    pub mods_count: u32,
    pub mods_addr: u32,
    pub u: multiboot_info_u,
    pub mmap_length: u32,
    pub mmap_addr: u32,
}

/// The module structure.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct module_t {
    pub mod_start: u32,
    pub mod_end: u32,
    pub string: u32,
    pub reserved: u32,
}

/// The memory map.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct memory_map_t {
    pub size: u32,
    pub addr: u64,
    pub len: u64,
    pub type_: u32,
}
