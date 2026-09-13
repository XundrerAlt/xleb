// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "interrupt/idt/mod.h"
#include "mod.h"

void syscall_handler(regs_t *regs) {
    uint32_t num = regs->eax;
    uint32_t arg0 = regs->ebx;
    uint32_t arg1 = regs->ecx;
    uint32_t arg2 = regs->edx;
    switch (num) {
        case SYS_PRINT:
            DEBUG("SYSCALL!");
            regs->eax = 0;
            break;
        default:
            regs->eax = -1;
            break;
    }
}
