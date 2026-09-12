// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "interrupt/idt/mod.h"
#include "interrupt/irq/mod.h"
#include "interrupt/pic/mod.h"

irq_handler_t irq_table[16] = {0};

void irq_handler(regs_t *regs) {
    uint8_t irq = regs->int_no - 32;
    pic_send_eoi(irq);
    if (irq >= 16) {
        ERROR("IRQ: invalid IRQ number %d (int_no=%d)", irq, regs->int_no);
        return;
    }
    if (irq_table[irq]) {
        irq_table[irq](regs);
    } else {
        DEBUG("IRQ%d: no handler", irq);
    }
}