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
    subl $0x80000000, %edi
    movl %edi, %cr3

    xorl %eax, %eax
    movl $1024, %ecx
    rep stosl

    movl $kernel_page_directory, %edi
    subl $0x80000000, %edi
    xorl %eax, %eax
    movl $0x00000083, %edx
    movl $512, %ecx
1:  movl %edx, (%edi)
    addl $0x400000, %eax
    addl $0x400000, %edx
    addl $4, %edi
    loop 1b

    xorl %eax, %eax
    movl $0x00000083, %edx
    movl $512, %ecx
2:  movl %edx, (%edi)
    addl $0x400000, %eax
    addl $0x400000, %edx
    addl $4, %edi
    loop 2b
    movl %cr4, %eax
    orl $(1 << 4), %eax
    movl %eax, %cr4

    movl %cr0, %eax
    orl $0x80000000, %eax
    movl %eax, %cr0

    lea higher_half, %ecx
    jmp *%ecx

higher_half:
    movl %ebp, (boot_magic)
    movl %esi, (info_ptr)
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

.align 16
.globl stack_top
stack_limit:
    .skip 16384
stack_top:
