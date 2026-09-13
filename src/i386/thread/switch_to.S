// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
.globl switch_to
switch_to:
    pushl %ebp
    pushl %ebx
    pushl %esi
    pushl %edi
    pushfl

    movl 24(%esp), %eax
    movl %esp, (%eax)

    movl 28(%esp), %eax
    movl (%eax), %ebx
    movl %ebx, %cr3
    movl 4(%eax), %esp

    popfl
    popl %edi
    popl %esi
    popl %ebx
    popl %ebp

    sti
    ret
