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
    .global switch_to
    .type switch_to, @function
switch_to:
    movl 4(%esp), %eax
    movl 8(%esp), %ecx
    movl 12(%esp), %edx

    movl %eax, %cr3

    movw $KERNEL_DS, %ax
    movw %ax, %ds
    movw %ax, %es
    movw %ax, %fs
    movw %ax, %gs

    movl %edx, %esp
    iretl
    .size switch_to_first, . - switch_to_first
