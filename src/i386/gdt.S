// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
.section .data
.align 8
gdt:
    # null descriptor (0x00)
    .long 0x00000000
    .long 0x00000000
    # kernel code segment (0x08)
    .long 0x0000FFFF
    .long 0x00CF9A00
    # kernel data segment (0x10)
    .long 0x0000FFFF
    .long 0x00CF9200
    # user code segment (0x18)
    .long 0x0000FFFF
    .long 0x00CFFA00
    # user data segment (0x20)
    .long 0x0000FFFF
    .long 0x00CFF200
    # TSS segment (0x28)
    .space 8
gdt_end:

gdt_ptr:
    .word gdt_end - gdt - 1
    .long gdt

.section .text
.global gdt_load
gdt_load:
    lgdt (gdt_ptr)
    ljmp $0x08, $reload_cs

reload_cs:
    movw $0x10, %ax
    movw %ax, %ds
    movw %ax, %es
    movw %ax, %fs
    movw %ax, %gs
    movw %ax, %ss
    ret
