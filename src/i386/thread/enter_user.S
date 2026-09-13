// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
.globl enter_user_mode
enter_user_mode:
    cli

    mov $0x23, %ax
    mov %ax, %ds
    mov %ax, %es
    mov %ax, %fs
    mov %ax, %gs

    push $0x23
    push 8(%esp)
    push $0x202
    push $0x1B
    push 4(%esp)

    iret
