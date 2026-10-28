// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use crate::arch::boot::bootinfo;
use crate::arch::boot::mmap::{MEMORY_MAP, MemoryType};
use crate::arch::cpu;
use crate::arch::mm::{
    PAGE_PRESENT, PAGE_USER, PAGE_WRITE, PageDir, PageTable, map_page_in, map_region,
    PAGE_MASK, PAGE_SIZE, PD_SHIFT, PT_SHIFT, PT_MASK
};
use crate::cap::{CAP_ALL, CNode, Cap};
use crate::logger::{info, verbose};
use crate::{PHYS_TO_VIRT, VIRT_TO_PHYS};
use core::ptr;
use xleb_api::{BOOTINFO_ADDR, CapType, ThreadInfo, ThreadState};

pub(crate) mod switch;

pub const USER_STACK_ADDR: u32 = 0x4000_0000;
pub const KERNEL_STACK_ADDR: u32 = 0x8008_0000;
pub const USER_CODE_ADDR: u32 = 0x4100_0000;
pub const STACK_SIZE: usize = 0x8000;

const USER_CS: u32 = 0x1B;
const USER_DS: u32 = 0x23;

#[repr(C)]
#[derive(Debug)]
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
#[derive(Debug)]
pub struct Thread {
    pub info: ThreadInfo,
    pub tid: u32,
    pub state: ThreadState,
    pub pagespace: u32,
    pub esp: *mut u32,
    pub context: *mut ThreadContext,
    pub stack_limit: *mut u8,
    pub next: *mut Thread,
    pub prev: *mut Thread,
    pub cspace: CNode,
}

static mut ROOT_PD: PageDir = PageDir([0; 1024]);
static mut ROOT_CODE_PT: PageTable = PageTable([0; 1024]);
static mut ROOT_STACK_PT: PageTable = PageTable([0; 1024]);
static mut ROOT_BOOTINFO_PT: PageTable = PageTable([0; 1024]);

#[repr(C, align(16))]
struct RootKStack([u8; STACK_SIZE]);
static mut ROOT_KSTACK: RootKStack = RootKStack([0; STACK_SIZE]);

#[repr(C, align(4096))]
struct RootUStack([u8; STACK_SIZE]);
static mut ROOT_USTACK: RootUStack = RootUStack([0; STACK_SIZE]);

static mut ROOT_THREAD: Thread = Thread {
    info: ThreadInfo { stack_top: 0, stack_size: 0, entry: 0 },
    tid: 0,
    state: ThreadState::Ready,
    pagespace: 0,
    esp: core::ptr::null_mut(),
    context: core::ptr::null_mut(),
    stack_limit: core::ptr::null_mut(),
    next: core::ptr::null_mut(),
    prev: core::ptr::null_mut(),
    cspace: CNode::new(),
};

static mut ROOT_CTX: ThreadContext = ThreadContext {
    edi: 0,
    esi: 0,
    ebx: 0,
    ebp: 0,
    eflags: 0x202,
    eip: 0,
};

unsafe fn build_user_iret_frame(kstack_top: u32, user_eip: u32, user_esp: u32) -> u32 {
    unsafe {
        let mut sp = kstack_top as *mut u32;
        sp = sp.sub(1);
        *sp = USER_DS;
        sp = sp.sub(1);
        *sp = user_esp;
        sp = sp.sub(1);
        *sp = 0x3202; // SECURITY TODO: any threads can break any ports. fix it
        sp = sp.sub(1);
        *sp = USER_CS;
        sp = sp.sub(1);
        *sp = user_eip;
        sp as u32
    }
}

pub(crate) unsafe fn init_thread(
    th_phys: u32,
    tid: u32,
    entry: u32,
    user_esp: u32,
    pagespace_phys: u32,
    kstack: u32,
) -> u32 {
    let kstack_top = ((kstack as usize + STACK_SIZE) & !0xF) as u32;
    verbose!(
        "init_thread: th_phys={:#x} tid={} entry={:#x} user_esp={:#x} space={:#x} kstack={:#x}..{:#x}",
        th_phys,
        tid,
        entry,
        user_esp,
        pagespace_phys,
        kstack,
        kstack_top
    );
    let frame_esp = unsafe { build_user_iret_frame(kstack_top, entry, user_esp) };
    verbose!(
        "init_thread: IRET frame at {:#x} (kstack_top={:#x}, size={})",
             frame_esp,
             kstack_top,
             kstack_top - frame_esp
    );
    unsafe {
        let th = PHYS_TO_VIRT!(th_phys) as *mut Thread;
        (*th).info = ThreadInfo { stack_top: kstack_top, stack_size: STACK_SIZE as u32, entry };
        (*th).tid = tid;
        (*th).state = ThreadState::Ready;
        (*th).pagespace = pagespace_phys;
        (*th).esp = frame_esp as *mut u32;
        (*th).context = core::ptr::null_mut();
        (*th).stack_limit = kstack as *mut u8;
        (*th).next = core::ptr::null_mut();
        (*th).prev = core::ptr::null_mut();
        (*th).cspace = CNode::new();

        verbose!(
            "init_thread: th.esp={:#x} th.stack_limit={:#x} th.state={:?}",
            (*th).esp as u32,
                 (*th).stack_limit as u32,
                 (*th).state
        );
    }
    frame_esp
}

fn validate_program(prog_phys: u32, prog_size: usize) {
    if prog_phys == 0 || prog_size == 0 {
        panic!("create_root: invalid program");
    }
}

unsafe fn zero_root_tables(
    pd: &mut PageDir,
    code_pt: &mut PageTable,
    stack_pt: &mut PageTable,
    bootinfo_pt: &mut PageTable,
) {
    unsafe {
        ptr::write_bytes(pd.0.as_mut_ptr(), 0, 1024);
        ptr::write_bytes(code_pt.0.as_mut_ptr(), 0, 1024);
        ptr::write_bytes(stack_pt.0.as_mut_ptr(), 0, 1024);
        ptr::write_bytes(bootinfo_pt.0.as_mut_ptr(), 0, 1024);
    }
}

unsafe fn copy_kernel_pd(pd_ptr: *mut u32) {
    let old_cr3 = unsafe { cpu::read_cr3() };
    let old_pd = PHYS_TO_VIRT!(old_cr3) as *const u32;
    verbose!("create_root: old_cr3={:#x}, copying PD[512..1024]", old_cr3);
    unsafe {
        for i in 512..1024 {
            *pd_ptr.add(i) = *old_pd.add(i);
        }
    }
}

unsafe fn map_user_code(pd_ptr: *mut u32, code_pt_ptr: *mut u32, prog_phys: u32, prog_size: usize) {
    verbose!(
        "create_root: mapping user code: code_phys={:#x} -> USER_CODE_ADDR={:#x}",
        prog_phys,
        USER_CODE_ADDR
    );
    unsafe {
        map_region(
            pd_ptr,
            code_pt_ptr,
            USER_CODE_ADDR,
            prog_phys,
            prog_size,
            PAGE_PRESENT | PAGE_WRITE | PAGE_USER,
        );
    }
}

unsafe fn map_user_stack(pd_ptr: *mut u32, stack_pt_ptr: *mut u32, ustack_phys: u32) {
    verbose!(
        "create_root: mapping user stack: ustack_phys={:#x} -> USER_STACK_ADDR={:#x}",
        ustack_phys,
        USER_STACK_ADDR
    );
    unsafe {
        map_region(
            pd_ptr,
            stack_pt_ptr,
            USER_STACK_ADDR,
            ustack_phys,
            STACK_SIZE,
            PAGE_PRESENT | PAGE_WRITE | PAGE_USER,
        );
    }
}

unsafe fn map_bootinfo(pd_ptr: *mut u32, bootinfo_pt_ptr: *mut u32) {
    unsafe {
        bootinfo::fill();
    }
    verbose!(
        "create_root: mapping bootinfo: phys={:#x} -> BOOTINFO_ADDR={:#x}",
        bootinfo::phys_addr(),
        BOOTINFO_ADDR
    );
    unsafe {
        map_page_in(
            pd_ptr,
            bootinfo_pt_ptr,
            BOOTINFO_ADDR,
            bootinfo::phys_addr(),
            PAGE_PRESENT | PAGE_USER,
        );
    }
}

unsafe fn install_pagespace_cap(th: &mut Thread, pd_phys: u32) {
    unsafe {
        th.cspace.insert(
            None,
            Cap::new(CapType::PageSpace, pd_phys as usize, 0, CAP_ALL),
        );
    }
}

unsafe fn install_untyped_caps(th: &mut Thread) {
    let mmap = unsafe { &*core::ptr::addr_of!(MEMORY_MAP) };
    for i in 0..mmap.region_count {
        let r = &mmap.regions[i];
        if r.type_ == MemoryType::Available {
            let base = r.start;
            let size = r.end - r.start + 1;
            unsafe {
                th.cspace
                    .insert(None, Cap::new(CapType::Untyped, base, size, CAP_ALL));
            }
            verbose!(
                "create_root: untyped {:#x}..{:#x} ({} KB)",
                base,
                r.end,
                size / 1024
            );
        }
    }
}

pub(crate) unsafe fn create_root(prog_phys: u32, prog_size: usize) -> &'static mut Thread {
    validate_program(prog_phys, prog_size);

    verbose!(
        "create_root: prog_phys={:#x} prog_size={:#x}",
        prog_phys,
        prog_size
    );

    let th = unsafe { &mut *ptr::addr_of_mut!(ROOT_THREAD) };
    let ustack = unsafe { &mut *ptr::addr_of_mut!(ROOT_USTACK) };
    let kstack = unsafe { &mut *ptr::addr_of_mut!(ROOT_KSTACK) };
    let ctx = unsafe { &mut *ptr::addr_of_mut!(ROOT_CTX) };
    let pd = unsafe { &mut *ptr::addr_of_mut!(ROOT_PD) };
    let code_pt = unsafe { &mut *ptr::addr_of_mut!(ROOT_CODE_PT) };
    let stack_pt = unsafe { &mut *ptr::addr_of_mut!(ROOT_STACK_PT) };
    let bootinfo_pt = unsafe { &mut *ptr::addr_of_mut!(ROOT_BOOTINFO_PT) };

    let pd_ptr = pd.0.as_mut_ptr();
    let code_pt_ptr = code_pt.0.as_mut_ptr();
    let stack_pt_ptr = stack_pt.0.as_mut_ptr();
    let bootinfo_pt_ptr = bootinfo_pt.0.as_mut_ptr();

    verbose!(
        "create_root: ROOT_PD={:#x} ROOT_CODE_PT={:#x}",
        pd_ptr as u32,
        code_pt_ptr as u32
    );

    unsafe {
        zero_root_tables(pd, code_pt, stack_pt, bootinfo_pt);
        copy_kernel_pd(pd_ptr);
        map_user_code(pd_ptr, code_pt_ptr, prog_phys, prog_size);
    }

    let ustack_phys = VIRT_TO_PHYS!(ustack.0.as_mut_ptr() as u32);
    unsafe {
        map_user_stack(pd_ptr, stack_pt_ptr, ustack_phys);
        map_bootinfo(pd_ptr, bootinfo_pt_ptr);
    }

    let pagespace_phys = VIRT_TO_PHYS!(pd_ptr as u32);
    let user_esp = USER_STACK_ADDR + STACK_SIZE as u32;
    unsafe {
        init_thread(
            VIRT_TO_PHYS!(th as *mut Thread as usize) as u32,
            0,
            USER_CODE_ADDR,
            user_esp,
            pagespace_phys,
            kstack.0.as_mut_ptr() as u32,
        );
    }
    ctx.eip = USER_CODE_ADDR;
    ctx.eflags = 0x202;
    th.context = ctx as *mut ThreadContext;

    unsafe {
        install_pagespace_cap(th, pagespace_phys);
        install_untyped_caps(th);
    }

    info!("Root thread initialized");
    th
}
