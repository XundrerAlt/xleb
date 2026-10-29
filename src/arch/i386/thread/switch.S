# SPDX-License-Identifier: GPL-3.0-or-later
# SPDX-FileCopyrightText: 2026 XundrerAlt

    .equ THREAD_CONTEXT,  0
    .equ THREAD_PD,       24    # pagespace
    .equ THREAD_ESP,      28    # esp

    .equ CTX_EDI,    0
    .equ CTX_ESI,    4
    .equ CTX_EBX,    8
    .equ CTX_EBP,    12
    .equ CTX_EFLAGS, 16
    .equ CTX_EIP,    20

    .section .text

    .global switch_to
    .type switch_to, @function
switch_to:
    movl 4(%esp), %eax
    movl 8(%esp), %edx

    testl %eax, %eax
    jz 1f

    movl %edi, CTX_EDI(%eax)
    movl %esi, CTX_ESI(%eax)
    movl %ebx, CTX_EBX(%eax)
    movl %ebp, CTX_EBP(%eax)

    pushfl
    popl %ebx
    movl %ebx, CTX_EFLAGS(%eax)

    movl %esp, THREAD_ESP(%eax)
1:
    movl THREAD_PD(%edx), %eax
    movl %eax, %cr3

    movl THREAD_ESP(%edx), %esp

    movl CTX_EDI(%edx), %edi
    movl CTX_ESI(%edx), %esi
    movl CTX_EBX(%edx), %ebx
    movl CTX_EBP(%edx), %ebp

    movl CTX_EFLAGS(%edx), %eax
    pushl %eax
    popfl

    ret
    .size switch_to, . - switch_to

    .global enter_to_user
    .type enter_to_user, @function
enter_to_user:
    movl 4(%esp), %esp
    iret
    .size enter_to_user, . - enter_to_user
