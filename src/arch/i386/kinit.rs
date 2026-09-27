#[unsafe(no_mangle)]
pub extern "C" fn kinit() -> ! {
    crate::info!("xleb: hello world");
    loop {}
}
