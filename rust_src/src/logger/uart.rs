const COM1: u16 = 0x3F8;

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    core::arch::asm!(
        "in al, dx",
        out("al") value,
        in("dx") port,
        options(nomem, nostack, preserves_flags)
    );
    value
}

#[inline]
unsafe fn outb(port: u16, value: u8) {
    core::arch::asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        options(nomem, nostack, preserves_flags)
    );
}

#[no_mangle]
pub extern "C" fn uart_putc(c: u8) {
    unsafe {
        while (inb(COM1 + 5) & (1 << 5)) == 0 {
            core::hint::spin_loop();
        }
        if c == b'\n' {
            uart_putc(b'\r');
        }
        outb(COM1, c);
    }
}

#[no_mangle]
pub extern "C" fn uart_puts(s: *const u8) {
    if s.is_null() {
        return;
    }
    let mut p = s;
    unsafe {
        while *p != 0 {
            uart_putc(*p);
            p = p.add(1);
        }
    }
}
