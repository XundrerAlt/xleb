// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "halt.h"
#include "mod.h"
#include "thread/mod.h"

static int current_idx = 0;

void schedule(void) {
    if (thread_count == 0) return;
    thread_t *prev = current_thread;
    thread_t *next = NULL;
    for (int i = 0; i < thread_count; i++) {
        current_idx = (current_idx + 1) % thread_count;
        thread_t *t = thread_queue[current_idx];
        if (t && t != prev && t->state != THREAD_DEAD) {
            next = t;
            break;
        }
    }
    if (!next) return;
    if (prev->state == THREAD_RUNNING)
        prev->state = THREAD_READY;
    next->state = THREAD_RUNNING;
    current_thread = next;
    archspec_schedule(next);
    switch_to(prev, next);
}
