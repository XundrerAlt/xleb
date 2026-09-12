// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "interrupt/irq/mod.h"
#include "basic_drivers/timer/mod.h"
#include "iob.h"

#define TIMER_HZ 100
extern void timer_handler(regs_t *r);

void timer_init() {
    uint32_t divisor = 1193180 / TIMER_HZ;
    outb(0x43, 0x36);
    outb(0x40, divisor & 0xFF);
    outb(0x40, (divisor >> 8) & 0xFF);
    irq_register_handler(0, timer_handler);
    INFO("PIT timer: hello world");
}