// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "interrupt/irq/mod.h"
#include "basic_drivers/timer/mod.h"
#include "iob.h"
#include "scheduler/mod.h"

static volatile uint32_t timer_ticks = 0;

void timer_handler(regs_t *r) {
    timer_ticks++;
    schedule();
}

uint32_t timer_get_ticks(void) {
    return timer_ticks;
}
