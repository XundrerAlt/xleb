# SPDX-License-Identifier: GPL-3.0-or-later
# SPDX-FileCopyrightText: 2026 XundrerAlt
    .equ THREAD_PD_ADDR,    20
    .equ THREAD_ESP,        24
    .equ THREAD_CONTEXT,    28

    .equ CTX_EDI,         0
    .equ CTX_ESI,         4
    .equ CTX_EBX,         8
    .equ CTX_EBP,         12
    .equ CTX_EFLAGS,        16
    .equ CTX_EIP,         20

    .equ KERNEL_CS,     0x08
    .equ KERNEL_DS,     0x10

    .section .text
    .global switch_to_first
    .type switch_to_first, @function
switch_to_first:
    movl 4(%esp), %eax
    movl 8(%esp), %ecx
    movl 12(%esp), %edx

    movl %eax, %cr3

    movw $KERNEL_DS, %ax
    movw %ax, %ds
    movw %ax, %es
    movw %ax, %fs
    movw %ax, %gs
    movw %ax, %ss

    movl CTX_EDI(%ecx), %edi
    movl CTX_ESI(%ecx), %esi
    movl CTX_EBX(%ecx), %ebx
    movl CTX_EBP(%ecx), %ebp

    movl %edx, %esp

    pushl CTX_EFLAGS(%ecx)
    pushl $KERNEL_CS
    pushl CTX_EIP(%ecx)

    iretl
    .size switch_to_first, . - switch_to_first
