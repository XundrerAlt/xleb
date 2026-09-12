// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
.section .text
.global _entry
_entry:
    cli
    movl $stack_top, %esp

    movl %eax, %ebp
    movl %ebx, %esi

    movl $kernel_page_directory, %edi
    subl $0xC0000000, %edi
    movl %edi, %cr3

    xorl %eax, %eax
    movl $1024, %ecx
    rep stosl

    movl $boot_page_table, %edi
    subl $0xC0000000, %edi
    movl $0x00000003, %eax
    movl $1024, %ecx
1:  movl %eax, (%edi)
    addl $0x1000, %eax
    addl $4, %edi
    loop 1b

    movl %cr3, %edi
    movl $boot_page_table, %eax
    subl $0xC0000000, %eax
    orl $0x00000003, %eax
    movl %eax, 0(%edi)
    movl %eax, (768 * 4)(%edi)

    movl %cr0, %eax
    orl $0x80000000, %eax
    movl %eax, %cr0

    lea higher_half, %ecx
    jmp *%ecx

higher_half:
    movl %ebp, (boot_magic)
    movl %esi, (info_ptr)
    movl %cr3, %eax
    addl $0xC0000000, %eax
    movl $0, (%eax)
    invlpg (%eax)
    call gdt_load
    call kinit
    hlt

.section .data

.global boot_magic
boot_magic:
    .long 0

.global info_ptr
info_ptr:
    .long 0

.section .bss
.global kernel_page_directory
.align 4096
kernel_page_directory:
    .skip 4096

boot_page_table:
    .skip 4096

.align 16
stack_limit:
    .skip 16384
stack_top:
