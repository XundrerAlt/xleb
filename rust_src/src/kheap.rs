use core::cell::UnsafeCell;

const HDR: u32 = core::mem::size_of::<KheapBlock>() as u32;

extern "C" {
    fn untyped_alloc(size: u32) -> u32;
}

#[inline]
fn phys_to_virt(phys: u32) -> *mut u8 {
    (phys + 0xC0000000) as *mut u8
}

#[repr(C)]
struct KheapBlock {
    size: u32,
    free: u8,
    _pad: [u8; 3],
    next: *mut KheapBlock,
}

struct KHeap {
    head: *mut KheapBlock,
    total: u32,
    used: u32,
}

impl KHeap {
    const fn new() -> Self {
        KHeap {
            head: core::ptr::null_mut(),
            total: 0,
            used: 0,
        }
    }
}

struct KHeapCell(UnsafeCell<KHeap>);

unsafe impl Sync for KHeapCell {}

static KHEAP: KHeapCell = KHeapCell(UnsafeCell::new(KHeap::new()));

#[inline]
fn kheap() -> &'static mut KHeap {
    unsafe { &mut *KHEAP.0.get() }
}

#[inline]
fn align8(size: u32) -> u32 {
    (size + 7) & !7
}

#[no_mangle]
pub unsafe extern "C" fn kheap_init() {
    const KHEAP_SIZE: u32 = 4 * 1024 * 1024;
    let region = untyped_alloc(KHEAP_SIZE);
    if region == 0 {
        loop {}
    }
    let base = phys_to_virt(region);
    let head = base as *mut KheapBlock;

    (*head).size = KHEAP_SIZE - HDR;
    (*head).free = 1;
    (*head).next = core::ptr::null_mut();

    let heap = kheap();
    heap.head = head;
    heap.total = KHEAP_SIZE;
    heap.used = 0;
}

#[no_mangle]
pub unsafe extern "C" fn kmalloc(size: u32) -> *mut u8 {
    if size == 0 {
        return core::ptr::null_mut();
    }

    let size = align8(size);
    let heap = kheap();

    let mut b = heap.head;
    while !b.is_null() {
        if (*b).free == 1 && (*b).size >= size {
            if (*b).size > size + HDR + 16 {
                let nb = (b as *mut u8).add((HDR + size) as usize) as *mut KheapBlock;
                (*nb).size = (*b).size - size - HDR;
                (*nb).free = 1;
                (*nb).next = (*b).next;
                (*b).size = size;
                (*b).next = nb;
            }
            (*b).free = 0;
            heap.used += HDR + (*b).size;
            return (b as *mut u8).add(HDR as usize);
        }
        b = (*b).next;
    }
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn kfree(p: *mut u8) {
    if p.is_null() {
        return;
    }

    let heap = kheap();
    let mut b = p.sub(HDR as usize) as *mut KheapBlock;

    if (*b).free == 1 {
        return;
    }

    (*b).free = 1;
    heap.used -= HDR + (*b).size;

    b = heap.head;
    while !b.is_null() && !(*b).next.is_null() {
        if (*b).free == 1 && (*(*b).next).free == 1 {
            (*b).size += HDR + (*(*b).next).size;
            (*b).next = (*(*b).next).next;
        } else {
            b = (*b).next;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn kheap_stats(total: *mut u32, used: *mut u32) {
    let heap = kheap();
    if !total.is_null() {
        *total = heap.total;
    }
    if !used.is_null() {
        *used = heap.used;
    }
}
