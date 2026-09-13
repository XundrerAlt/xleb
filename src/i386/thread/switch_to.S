// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
.global switch_to
switch_to:
    pushl %ebp
    pushl %ebx
    pushl %esi
    pushl %edi
    pushfl

    movl 24(%esp), %eax
    movl %esp, (%eax)

    movl 28(%esp), %esp

    popfl
    popl %edi
    popl %esi
    popl %ebx
    popl %ebp

    sti
    ret
