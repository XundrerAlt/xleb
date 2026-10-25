// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use crate::arch::boot::bootinfo;
use crate::arch::cpu;
use crate::logger::{info, verbose};
use core::ptr;
use xleb_api::{ThreadInfo, ThreadState};

use crate::{PHYS_TO_VIRT, VIRT_TO_PHYS};

pub(crate) mod switch;

pub const USER_STACK_ADDR: u32 = 0x4000_0000;
pub const USER_CODE_ADDR: u32 = 0x4100_0000;
pub const BOOTINFO_ADDR: u32 = 0x4000_1000;
pub const STACK_SIZE: usize = 4096;

const USER_CS: u32 = 0x1B;
const USER_DS: u32 = 0x23;

const PAGE_PRESENT: u32 = 1 << 0;
const PAGE_WRITE: u32 = 1 << 1;
const PAGE_USER: u32 = 1 << 2;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ThreadContext {
    pub edi: u32,
    pub esi: u32,
    pub ebx: u32,
    pub ebp: u32,
    pub eflags: u32,
    pub eip: u32,
}

const _: () = {
    assert!(core::mem::size_of::<ThreadContext>() == 24);
    assert!(core::mem::offset_of!(ThreadContext, edi) == 0);
    assert!(core::mem::offset_of!(ThreadContext, esi) == 4);
    assert!(core::mem::offset_of!(ThreadContext, ebx) == 8);
    assert!(core::mem::offset_of!(ThreadContext, ebp) == 12);
    assert!(core::mem::offset_of!(ThreadContext, eflags) == 16);
    assert!(core::mem::offset_of!(ThreadContext, eip) == 20);
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Thread {
    pub info: ThreadInfo,
    pub tid: u32,
    pub state: ThreadState,
    pub pd_addr: u32,
    pub esp: *mut u32,
    pub context: *mut ThreadContext,
    pub stack_limit: *mut u8,
    pub next: *mut Thread,
    pub prev: *mut Thread,
}

#[repr(C, align(4096))]
struct PageTable([u32; 1024]);
#[repr(C, align(4096))]
struct PageDir([u32; 1024]);
static mut ROOT_PD: PageDir = PageDir([0; 1024]);
static mut ROOT_CODE_PT: PageTable = PageTable([0; 1024]);
static mut ROOT_STACK_PT: PageTable = PageTable([0; 1024]);
static mut ROOT_KERNEL_PT: PageTable = PageTable([0; 1024]);
static mut ROOT_BOOTINFO_PT: PageTable = PageTable([0; 1024]);

#[repr(C, align(16))]
struct RootKStack([u8; STACK_SIZE]);
static mut ROOT_KSTACK: RootKStack = RootKStack([0; STACK_SIZE]);

#[repr(C, align(4096))]
struct RootUStack([u8; STACK_SIZE]);
static mut ROOT_USTACK: RootUStack = RootUStack([0; STACK_SIZE]);

static mut ROOT_THREAD: Thread = Thread {
    info: ThreadInfo {
        stack_top: 0,
        stack_size: 0,
        entry: 0,
    },
    tid: 0,
    state: ThreadState::Ready,
    pd_addr: 0,
    esp: core::ptr::null_mut(),
    context: core::ptr::null_mut(),
    stack_limit: core::ptr::null_mut(),
    next: core::ptr::null_mut(),
    prev: core::ptr::null_mut(),
};

static mut ROOT_CTX: ThreadContext = ThreadContext {
    edi: 0,
    esi: 0,
    ebx: 0,
    ebp: 0,
    eflags: 0x202,
    eip: 0,
};

unsafe fn map_page_in(pd: *mut u32, pt: *mut u32, va: u32, phys: u32, flags: u32) {
    let pd_idx = (va >> 22) as usize;
    let pt_idx = ((va >> 12) & 0x3FF) as usize;
    unsafe {
        let pt_phys = VIRT_TO_PHYS!(pt as u32);
        *pd.add(pd_idx) = pt_phys | PAGE_PRESENT | PAGE_WRITE | PAGE_USER;
        *pt.add(pt_idx) = (phys & !0xFFF) | flags;
    }
}

unsafe fn map_region(
    pd: *mut u32,
    pt: *mut u32,
    dst_va: u32,
    src_phys: u32,
    len: usize,
    flags: u32,
) {
    let pages = (len + 0xFFF) / 0x1000;
    for i in 0..pages {
        let phys = src_phys + (i as u32) * 0x1000;
        let va = dst_va + (i as u32) * 0x1000;
        unsafe { map_page_in(pd, pt, va, phys, flags) };
    }
}

unsafe fn build_user_iret_frame(kstack_top: u32, user_eip: u32, user_esp: u32) -> u32 {
    unsafe {
        let mut sp = kstack_top as *mut u32;
        sp = sp.sub(1);
        *sp = USER_DS;
        sp = sp.sub(1);
        *sp = user_esp;
        sp = sp.sub(1);
        *sp = 0x202;
        sp = sp.sub(1);
        *sp = USER_CS;
        sp = sp.sub(1);
        *sp = user_eip;
        sp as u32
    }
}

pub(crate) unsafe fn create_root(prog_phys: u32, prog_size: usize) -> &'static mut Thread {
    if prog_phys == 0 || prog_size == 0 {
        panic!("create_root: invalid program");
    }

    verbose!(
        "create_root: prog_phys={:#x} prog_size={:#x}",
        prog_phys,
        prog_size
    );

    let th = unsafe { &mut *ptr::addr_of_mut!(ROOT_THREAD) };
    let kstack = unsafe { &mut *ptr::addr_of_mut!(ROOT_KSTACK) };
    let ustack = unsafe { &mut *ptr::addr_of_mut!(ROOT_USTACK) };
    let ctx = unsafe { &mut *ptr::addr_of_mut!(ROOT_CTX) };
    let pd = unsafe { &mut *ptr::addr_of_mut!(ROOT_PD) };
    let code_pt = unsafe { &mut *ptr::addr_of_mut!(ROOT_CODE_PT) };
    let stack_pt = unsafe { &mut *ptr::addr_of_mut!(ROOT_STACK_PT) };
    let kernel_pt = unsafe { &mut *ptr::addr_of_mut!(ROOT_KERNEL_PT) };
    let bootinfo_pt = unsafe { &mut *ptr::addr_of_mut!(ROOT_BOOTINFO_PT) };

    let pd_ptr = pd.0.as_mut_ptr();
    let code_pt_ptr = code_pt.0.as_mut_ptr();
    let stack_pt_ptr = stack_pt.0.as_mut_ptr();
    let bootinfo_pt_ptr = stack_pt.0.as_mut_ptr();

    verbose!(
        "create_root: ROOT_PD={:#x} ROOT_CODE_PT={:#x} ROOT_STACK_PT={:#x}",
        pd_ptr as u32,
        code_pt_ptr as u32,
        stack_pt_ptr as u32,
    );

    unsafe {
        ptr::write_bytes(pd.0.as_mut_ptr(), 0, 1024);
        ptr::write_bytes(code_pt.0.as_mut_ptr(), 0, 1024);
        ptr::write_bytes(stack_pt.0.as_mut_ptr(), 0, 1024);
        ptr::write_bytes(kernel_pt.0.as_mut_ptr(), 0, 1024);
    }
    verbose!("create_root: PD/PT zeroed");

    let old_cr3 = unsafe { cpu::read_cr3() };
    let old_pd = PHYS_TO_VIRT!(old_cr3) as *const u32;
    verbose!("create_root: old_cr3={:#x}, copying PD[768..1024]", old_cr3);
    unsafe {
        for i in 768..1024 {
            *pd_ptr.add(i) = *old_pd.add(i);
        }
    }
    verbose!(
        "create_root: PD[768]={:#x} PD[1023]={:#x} (copied)",
        unsafe { *pd_ptr.add(768) },
        unsafe { *pd_ptr.add(1023) },
    );

    let code_phys = prog_phys;
    verbose!(
        "create_root: mapping user code: code_phys={:#x} pages={} -> USER_CODE_ADDR={:#x}",
        code_phys,
        (prog_size + 0xFFF) / 0x1000,
        USER_CODE_ADDR,
    );
    unsafe {
        map_region(
            pd_ptr,
            code_pt_ptr,
            USER_CODE_ADDR,
            code_phys,
            prog_size,
            PAGE_PRESENT | PAGE_USER,
        );
    }

    let ustack_phys = VIRT_TO_PHYS!(ustack.0.as_mut_ptr() as u32);
    verbose!(
        "create_root: mapping user stack: ustack={:#x} ustack_phys={:#x} -> USER_STACK_ADDR={:#x}",
        ustack.0.as_mut_ptr() as u32,
        ustack_phys,
        USER_STACK_ADDR,
    );
    unsafe {
        map_page_in(
            pd_ptr,
            stack_pt_ptr,
            USER_STACK_ADDR,
            ustack_phys,
            PAGE_PRESENT | PAGE_WRITE | PAGE_USER,
        );
    }

    unsafe {
        bootinfo::fill();
    }
    verbose!(
        "create_root: mapping bootinfo: bootinfo={:#x} bootinfo_phys={:#x} -> BOOTINFO_ADDR={:#x}",
        bootinfo::virt_addr(),
        bootinfo::phys_addr(),
        BOOTINFO_ADDR,
    );
    unsafe {
        map_page_in(
            pd_ptr,
            bootinfo_pt_ptr,
            BOOTINFO_ADDR,
            bootinfo::phys_addr(),
            PAGE_PRESENT | PAGE_WRITE | PAGE_USER,
        );
    }

    let kstack_base = kstack.0.as_mut_ptr() as u32;
    let kstack_top = (kstack_base + STACK_SIZE as u32) & !0xF;
    let user_esp = USER_STACK_ADDR + STACK_SIZE as u32;
    let frame_esp = unsafe { build_user_iret_frame(kstack_top, USER_CODE_ADDR, user_esp) };
    verbose!(
        "create_root: kstack_base={:#x} kstack_top={:#x} user_esp={:#x} frame_esp={:#x}",
        kstack_base,
        kstack_top,
        user_esp,
        frame_esp,
    );

    ctx.eip = USER_CODE_ADDR;
    ctx.eflags = 0x202;

    th.info = ThreadInfo {
        stack_top: kstack_top,
        stack_size: STACK_SIZE as u32,
        entry: USER_CODE_ADDR,
    };
    th.tid = 0;
    th.state = ThreadState::Ready;
    th.pd_addr = VIRT_TO_PHYS!(pd_ptr as u32);
    th.esp = frame_esp as *mut u32;
    th.context = ctx as *mut ThreadContext;
    th.stack_limit = kstack_base as *mut u8;
    th.next = ptr::null_mut();
    th.prev = ptr::null_mut();

    verbose!(
        "create_root: th.pd_addr={:#x} th.esp={:#x} th.stack_limit={:#x}",
        th.pd_addr,
        th.esp as u32,
        th.stack_limit as u32,
    );
    info!("Root thread initialized");
    th
}
