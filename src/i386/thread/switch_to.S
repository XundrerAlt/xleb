// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
.set CTX_EDI, 0
.set CTX_ESI, 4
.set CTX_EBX, 8
.set CTX_EBP, 12
.set CTX_EIP, 16
.set CTX_ESP, 20
.set CTX_EFLAGS, 24
.set CTX_SS, 28
.set PD_ADDR, 32
.set KERNEL_CS,  0x08

.globl switch_to
switch_to:
    movl 4(%esp), %eax
    movl 8(%esp), %edx

    movl %edi, CTX_EDI(%eax)
    movl %esi, CTX_ESI(%eax)
    movl %ebx, CTX_EBX(%eax)
    movl %ebp, CTX_EBP(%eax)

    pushfl
    popl %ecx
    movl $0x08, 4(%esp)
    movl %ecx, 8(%esp)
    movl %esp, CTX_ESP(%eax)

    movl PD_ADDR(%edx), %ecx
    movl %ecx, %cr3

    movl CTX_EDI(%edx), %edi
    movl CTX_ESI(%edx), %esi
    movl CTX_EBX(%edx), %ebx
    movl CTX_EBP(%edx), %ebp
    movl CTX_ESP(%edx), %esp
    iret

.globl switch_to_first
switch_to_first:
    movl 4(%esp), %edx
    movl PD_ADDR(%edx), %ecx
    movl %ecx, %cr3
    movl CTX_EDI(%edx), %edi
    movl CTX_ESI(%edx), %esi
    movl CTX_EBX(%edx), %ebx
    movl CTX_EBP(%edx), %ebp
    movl CTX_ESP(%edx), %esp
    iret
