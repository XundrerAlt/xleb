// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "basic_drivers/timer/mod.h"
#include "halt.h"
#include "interrupt/init.h"
#include "mm/mod.h"
#include "stdint.h"
#include "scheduler/mod.h"
#include "thread/mod.h"
#include "vfs/mod.h"

extern void get_bootloader_protocol(void);
extern uint32_t kernel_page_directory[1024];

void kinit(void) {
    interrupt_init();
    get_bootloader_protocol();
    for (int i = 768; i < 896; i++) {
        kernel_page_directory[i] = (i - 768) * 0x400000 | 0x83;
    }
    __asm__ volatile ("mov %%cr3, %%eax; mov %%eax, %%cr3" ::: "eax");
    mm_init();
    timer_init();
    scheduler_init();
#ifdef ENABLE_TESTS
    extern void ktest(void);
    thread_t *ktest_th = thread_create(ktest, 0, 0, 0);
    scheduler_add_thread(ktest_th);
#else
    extern void run_init(void);
    run_init();
#endif
    halt();
}
