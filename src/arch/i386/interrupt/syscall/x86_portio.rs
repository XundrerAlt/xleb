// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt

use crate::arch::cpu::{inb, inl, inw, outb, outl, outw, read_cr3};
use crate::arch::mm::{
    KERNEL_SPACE_START, PAGE_MASK, PAGE_PRESENT, PAGE_SIZE, PAGE_USER, PAGE_WRITE, PD_SHIFT,
    PT_MASK, PT_SHIFT,
};
use crate::cap::{CAP_READ, CAP_WRITE, current_cspace, get_cap};
use crate::logger::verbose;
use xleb_api::CapType;
use xleb_api::SyscallErrno::*;

use super::SyscallFrame;

pub(crate) fn sys(frame: &mut SyscallFrame) -> i32 {
    sys_impl(frame).unwrap_or_else(|e| e)
}

fn validate_user_buffer(ptr: u32, size: usize, need_write: bool) -> Result<(), i32> {
    if size == 0 {
        return Ok(());
    }
    let end = (ptr as usize).checked_add(size).ok_or(EFAULT)? as u32;
    if ptr >= KERNEL_SPACE_START || end > KERNEL_SPACE_START || end <= ptr {
        verbose!(
            "sys_x86_portio: buf {:#x}..{:#x} outside user space",
            ptr,
            end
        );
        return Err(EFAULT.into());
    }

    let cr3 = unsafe { read_cr3() };
    let pd = crate::PHYS_TO_VIRT!(cr3) as *const u32;

    let mut page = ptr & !(PAGE_SIZE as u32 - 1);
    let last = (end - 1) & !(PAGE_SIZE as u32 - 1);
    while page <= last {
        let pd_idx = (page >> PD_SHIFT) as usize;
        let pde = unsafe { *pd.add(pd_idx) };
        if pde & PAGE_PRESENT == 0 {
            verbose!("sys_x86_portio: pde {:#x} not present", page);
            return Err(EFAULT.into());
        }
        if pde & 0x80 != 0 {
            if pde & PAGE_USER == 0 || (need_write && pde & PAGE_WRITE == 0) {
                verbose!("sys_x86_portio: large page {:#x} bad flags", page);
                return Err(EFAULT.into());
            }
        } else {
            let pt = crate::PHYS_TO_VIRT!(pde & !PAGE_MASK as u32) as *const u32;
            let pt_idx = ((page >> PT_SHIFT) & PT_MASK) as usize;
            let pte = unsafe { *pt.add(pt_idx) };
            if pte & PAGE_PRESENT == 0 || pte & PAGE_USER == 0 {
                verbose!("sys_x86_portio: pte {:#x} not present/user", page);
                return Err(EFAULT.into());
            }
            if need_write && pte & PAGE_WRITE == 0 {
                verbose!("sys_x86_portio: pte {:#x} not writable", page);
                return Err(EFAULT.into());
            }
        }
        if page == last {
            break;
        }
        page = page.wrapping_add(PAGE_SIZE as u32);
    }
    Ok(())
}

fn sys_impl(frame: &mut SyscallFrame) -> Result<i32, i32> {
    let port = frame.ebx as u16;
    let buf_ptr = frame.ecx;
    let len = frame.edx as usize;
    let width = frame.esi;
    let dir = frame.edi;
    let cap_idx = frame.ebp as usize;

    if !matches!(width, 1 | 2 | 4) {
        verbose!("sys_x86_portio: invalid width={}", width);
        return Err(EINVAL.into());
    }
    if len == 0 {
        verbose!("sys_x86_portio: len=0");
        return Err(EINVAL.into());
    }
    if dir > 1 {
        verbose!("sys_x86_portio: invalid direction={}", dir);
        return Err(EINVAL.into());
    }

    let cspace = current_cspace()?;
    let rights = if dir == 1 { CAP_WRITE } else { CAP_READ };
    let cap = get_cap(
        cspace,
        cap_idx,
        CapType::x86_PortIo_Cap,
        rights,
        "sys_x86_portio",
    )?;

    let port_end = port as usize + 1;
    if (port as usize) < cap.object || port_end > cap.object + cap.size {
        verbose!(
            "sys_x86_portio: port={:#x} not in cap range {:#x}..{:#x}",
            port,
            cap.object,
            cap.object + cap.size
        );
        return Err(EPERM.into());
    }

    let total_bytes = len.checked_mul(width as usize).ok_or(EINVAL)?;
    let need_write = dir == 0;
    validate_user_buffer(buf_ptr, total_bytes, need_write)?;

    unsafe {
        match (dir, width) {
            (1, 1) => {
                let src = buf_ptr as *const u8;
                for i in 0..len {
                    let v = core::ptr::read_volatile(src.add(i));
                    outb(port, v);
                }
            }
            (1, 2) => {
                let src = buf_ptr as *const u16;
                for i in 0..len {
                    let v = core::ptr::read_volatile(src.add(i));
                    outw(port, v);
                }
            }
            (1, 4) => {
                let src = buf_ptr as *const u32;
                for i in 0..len {
                    let v = core::ptr::read_volatile(src.add(i));
                    outl(port, v);
                }
            }
            (0, 1) => {
                let dst = buf_ptr as *mut u8;
                for i in 0..len {
                    let v = inb(port);
                    core::ptr::write_volatile(dst.add(i), v);
                }
            }
            (0, 2) => {
                let dst = buf_ptr as *mut u16;
                for i in 0..len {
                    let v = inw(port);
                    core::ptr::write_volatile(dst.add(i), v);
                }
            }
            (0, 4) => {
                let dst = buf_ptr as *mut u32;
                for i in 0..len {
                    let v = inl(port);
                    core::ptr::write_volatile(dst.add(i), v);
                }
            }
            _ => return Err(EINVAL.into()),
        }
    }

    verbose!(
        "sys_x86_portio: port={:#x} buf={:#x} len={} width={} dir={}",
        port,
        buf_ptr,
        len,
        width,
        dir
    );
    Ok(len as i32)
}
