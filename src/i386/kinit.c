// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "basic_drivers/timer/mod.h"
#include "cap/mod.h"
#include "interrupt/init.h"
#include "mm/mod.h"
#include "stdint.h"
#include "task/scheduler/mod.h"

extern void get_bootloader_protocol(void);
void kinit(void) {
    interrupt_init();
    get_bootloader_protocol();
    mm_init();
    timer_init();
    //scheduler_init();
    cap_init();
#ifdef ENABLE_TESTS
    extern void ktest(void);
    ktest();
#else
    extern void kmain(void);
    kmain();
#endif
}
