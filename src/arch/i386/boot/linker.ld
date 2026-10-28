ENTRY(_entry)

SECTIONS
{
    . = 0x80100000;

    _kernel_start = .;

    .text : AT(ADDR(.text) - 0x80000000) {
        *(.multiboot)
        *(.text)
        *(.text.*)
    }

    .data : AT(ADDR(.data) - 0x80000000) {
        *(.data)
        *(.data.*)
    }

    .rodata : AT(ADDR(.rodata) - 0x80000000) {
        *(.rodata)
        . = ALIGN(4);
        __raw_devices_start = .;
        KEEP(*(.rodata.raw_devices))
        __raw_devices_end = .;
        *(.rodata.*)
    }

    .bss : AT(ADDR(.bss) - 0x80000000) {
        *(COMMON)
        *(.bss)
        *(.bss.*)
    }

    .tests : ALIGN(4) {
        __tests_start = .;
        KEEP(*(.tests))
        __tests_end = .;
    }

    . = ALIGN(4K);

    _kernel_end = .;
    ASSERT(_kernel_end - 0x80000000 <= 0x400000, "Kernel too big (>4MB)")
}
