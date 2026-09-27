ENTRY(_entry)

SECTIONS
{
    . = 0xC0100000;

    _kernel_start = .;

    .text : AT(ADDR(.text) - 0xC0000000) {
        *(.multiboot)
        *(.text)
        *(.text.*)
    }

    .data : AT(ADDR(.data) - 0xC0000000) {
        *(.data)
        *(.data.*)
    }

    .rodata : AT(ADDR(.rodata) - 0xC0000000) {
        *(.rodata)
        . = ALIGN(4);
        __raw_devices_start = .;
        KEEP(*(.rodata.raw_devices))
        __raw_devices_end = .;
        *(.rodata.*)
    }

    .bss : AT(ADDR(.bss) - 0xC0000000) {
        *(COMMON)
        *(.bss)
        *(.bss.*)
    }

    . = ALIGN(4K);

    _kernel_end = .;
    ASSERT(_kernel_end - 0xC0000000 <= 0x400000, "Kernel too big (>4MB)")
}
