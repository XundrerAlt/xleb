// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "boot/module.h"
#include "debug.h"
#include "halt.h"
#include "thread/mod.h"
#include "scheduler/mod.h"

#define USER_ENTRY 0x41000000

void run_init(void) {
    if (module_count == 0) {
        ERROR("no modules");
        halt();
    }

    module_t *m = module_get(0);
    DEBUG("loading module 0: 0x%x - 0x%x (%u bytes)",
          m->start, m->end, m->size);

    thread_t *init = thread_create((void*)m->start, 1, m->size);
    if (!init) {
        ERROR("failed to create init thread");
        halt();
    }
    DEBUG("init thread created, switching");
    scheduler_add_thread(init);
}
