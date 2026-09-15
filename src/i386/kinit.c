// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "basic_drivers/timer/mod.h"
#include "cap/mod.h"
#include "interrupt/init.h"
#include "mm/mod.h"
#include "stdint.h"
#include "scheduler/mod.h"

extern void get_bootloader_protocol(void);
void kinit(void) {
    interrupt_init();
    get_bootloader_protocol();
    cap_init();
    mm_init();
    timer_init();
    scheduler_init();
#ifdef ENABLE_TESTS
    extern void ktest(void);
    scheduler_add_thread(ktest, 0);
#else
    extern void kmain(void);
    scheduler_add_thread(kmain, 0);
#endif
}
