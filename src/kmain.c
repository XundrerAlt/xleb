// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "basic_drivers/timer/mod.h"
#include "debug.h"
#include "interrupt/init.h"
#include "halt.h"
#include "mm/mod.h"
#include "task/scheduler/mod.h"
extern void get_bootloader_protocol();

void kmain() {
    interrupt_init();
    get_bootloader_protocol();
    mm_init();
    timer_init();
    scheduler_init();
    halt();
}
