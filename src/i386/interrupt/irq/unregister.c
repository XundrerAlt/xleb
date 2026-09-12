// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "interrupt/irq/mod.h"
#include "interrupt/pic/mod.h"

void irq_unregister_handler(uint8_t irq) {
    if (irq >= 16) {
        ERROR("IRQ: invalid IRQ number %d", irq);
        return;
    }
    irq_table[irq] = 0;
    pic_mask_irq(irq);
    DEBUG("IRQ%d: handler unregistered", irq);
}