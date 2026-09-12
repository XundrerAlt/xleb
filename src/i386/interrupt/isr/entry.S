// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
.section .text

.macro isr_no_err n
isr\n:
    push $0
    push $\n
    jmp isr_common
.endm

.macro isr_err n
isr\n:
    push $\n
    jmp isr_common
.endm

isr_no_err 0 # division by zero
isr_no_err 1 # debug
isr_no_err 2 # non-maskable interrupt
isr_no_err 3 # breakpoint
isr_no_err 4 # overflow
isr_no_err 5 # bound range exceeded
isr_no_err 6 # invalid opcode
isr_no_err 7 # device not available
isr_err 8 # double fault
isr_no_err 9 # coprocessor segment overrun
isr_err 10 # invalid TSS
isr_err 11 # segment not present
isr_err 12 # stack segment fault
isr_err 13 # general protection fault
isr_err 14 # page fault
isr_no_err 15 # reserved
isr_no_err 16 # x87 FPU error
isr_err 17 # alignment check
isr_no_err 18 # machine check
isr_no_err 19 # SIMD exception
isr_no_err 20 # virtualization exception
isr_err 21 # control protection exception
isr_no_err 22 # reserved
isr_no_err 23 # reserved
isr_no_err 24 # reserved
isr_no_err 25 # reserved
isr_no_err 26 # reserved
isr_no_err 27 # reserved
isr_no_err 28 # hypervisor injection exception
isr_err 29 # vmm communication exception
isr_err 30 # security exception
isr_no_err 31 # reserved

isr_common:
    pusha
    push %ds
    push %es
    push %fs
    push %gs

    mov $0x10, %ax
    mov %ax, %ds
    mov %ax, %es
    mov %ax, %fs
    mov %ax, %gs

    mov %esp, %eax
    push %eax
    call isr_handler
    add $4, %esp

    pop %gs
    pop %fs
    pop %es
    pop %ds
    popa

    add $8, %esp
    iret

.section .rodata
.globl isr_entry_table
isr_entry_table:
    .long isr0, isr1, isr2, isr3, isr4, isr5, isr6, isr7
    .long isr8, isr9, isr10, isr11, isr12, isr13, isr14, isr15
    .long isr16, isr17, isr18, isr19, isr20, isr21, isr22, isr23
    .long isr24, isr25, isr26, isr27, isr28, isr29, isr30, isr31