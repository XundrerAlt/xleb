// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "mod.h"
#include "halt.h"
#include "process/mod.h"
#include "scheduler/mod.h"

void thread_destroy(thread_t *target) {
    process_t *process = target->process;
    target->state = THREAD_DEAD;
    int alive = 0;
    for (int i = 0; i < thread_count; i++) {
        thread_t *t = thread_queue[i];
        if (t && t != current_thread && t->process == process
            && t->state != THREAD_DEAD) {
            alive++;
        }
    }
    if (!alive) {
        process_destroy(process);
    }
    schedule();
    halt();
}
