// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
.section .text

.macro irq n
.globl irq\n
irq\n:
    push $0
    push $\n
    jmp irq_common
.endm

irq 32
irq 33
irq 34
irq 35
irq 36
irq 37
irq 38
irq 39
irq 40
irq 41
irq 42
irq 43
irq 44
irq 45
irq 46
irq 47

irq_common:
    pushal
    pushl %ds
    pushl %es
    pushl %fs
    pushl %gs

    mov $0x10, %ax
    mov %ax, %ds
    mov %ax, %es
    mov %ax, %fs
    mov %ax, %gs

    pushl %esp
    call irq_handler
    addl $4, %esp

    popl %gs
    popl %fs
    popl %es
    popl %ds
    popal

    add $8, %esp
    iret

.section .rodata
.align 4
.globl irq_entry_table
irq_entry_table:
    .long irq32, irq33, irq34, irq35, irq36, irq37, irq38, irq39
    .long irq40, irq41, irq42, irq43, irq44, irq45, irq46, irq47
