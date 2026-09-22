use core::alloc::{GlobalAlloc, Layout};

extern "C" {
    fn kmalloc(size: u32) -> *mut u8;
    fn kfree(ptr: *mut u8);
}

pub struct KernelAllocator;

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        kmalloc(layout.size() as u32)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        kfree(ptr)
    }
}

#[global_allocator]
static ALLOCATOR: KernelAllocator = KernelAllocator;

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    loop {}
}
