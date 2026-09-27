// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
use core::arch::global_asm;

macro_rules! stubs {
    (normal: $($name:ident : $vector:literal),* $(,)?) => {
        global_asm!(
            $(
                concat!(
                    ".global ", stringify!($name), "\n",
                    stringify!($name), ":\n",
                    "  push 0\n",
                    "  push ", $vector, "\n",
                    "  jmp isr_common\n",
                ),
            )*
        );
    };
    (errcode: $($name:ident : $vector:literal),* $(,)?) => {
        global_asm!(
            $(
                concat!(
                    ".global ", stringify!($name), "\n",
                    stringify!($name), ":\n",
                    "  push ", $vector, "\n",
                    "  jmp isr_common\n",
                ),
            )*
        );
    };
}

stubs!(normal:
    isr0: 0, isr1: 1, isr2: 2, isr3: 3,
    isr4: 4, isr5: 5, isr6: 6, isr7: 7,
    isr9: 9,
    isr15: 15, isr16: 16,
    isr18: 18, isr19: 19, isr20: 20, isr21: 21,
    isr22: 22, isr23: 23, isr24: 24, isr25: 25,
    isr26: 26, isr27: 27, isr28: 28, isr29: 29,
    isr30: 30, isr31: 31,
);

stubs!(errcode:
    isr8: 8,
    isr10: 10, isr11: 11, isr12: 12, isr13: 13, isr14: 14,
    isr17: 17,
);

stubs!(normal:
    irq0: 32, irq1: 33, irq2: 34, irq3: 35,
    irq4: 36, irq5: 37, irq6: 38, irq7: 39,
    irq8: 40, irq9: 41, irq10: 42, irq11: 43,
    irq12: 44, irq13: 45, irq14: 46, irq15: 47,
);

global_asm!(
    ".section .text",
    "isr_common:",
    "  pusha",
    "  mov ax, ds",
    "  push eax",
    "  mov ax, 0x10",
    "  mov ds, ax",
    "  mov es, ax",
    "  mov fs, ax",
    "  mov gs, ax",
    "  push esp",
    "  call isr_handler",
    "  add esp, 4",
    "  pop eax",
    "  mov ds, ax",
    "  mov es, ax",
    "  mov fs, ax",
    "  mov gs, ax",
    "  popa",
    "  add esp, 8",
    "  iret",

    "irq_common:",
    "  pusha",
    "  mov ax, ds",
    "  push eax",
    "  mov ax, 0x10",
    "  mov ds, ax",
    "  mov es, ax",
    "  mov fs, ax",
    "  mov gs, ax",
    "  push esp",
    "  call irq_handler",
    "  add esp, 4",
    "  pop eax",
    "  mov ds, ax",
    "  mov es, ax",
    "  mov fs, ax",
    "  mov gs, ax",
    "  popa",
    "  add esp, 8",
    "  iret",
);

global_asm!(
    ".section .rodata",
    ".global isr_table",
    "isr_table:",
    ".long isr0, isr1, isr2, isr3, isr4, isr5, isr6, isr7",
    ".long isr8, isr9, isr10, isr11, isr12, isr13, isr14, isr15",
    ".long isr16, isr17, isr18, isr19, isr20, isr21, isr22, isr23",
    ".long isr24, isr25, isr26, isr27, isr28, isr29, isr30, isr31",

    ".global irq_table",
    "irq_table:",
    ".long irq0,  irq1, irq2, irq3, irq4, irq5, irq6, irq7",
    ".long irq8,  irq9, irq10, irq11, irq12, irq13, irq14, irq15",
);
