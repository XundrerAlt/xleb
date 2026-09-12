// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "interrupt/irq/mod.h"
#include "interrupt/pic/mod.h"

void irq_register_handler(uint8_t irq, irq_handler_t handler) {
    if (irq >= 16) {
        ERROR("IRQ: invalid IRQ number %d", irq);
        return;
    }
    if (irq_table[irq] != 0) {
        WARN("IRQ%d: overriding existing handler", irq);
    }
    irq_table[irq] = handler;
    pic_unmask_irq(irq);
    DEBUG("IRQ%d: handler registered", irq);
}