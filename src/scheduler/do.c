// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "halt.h"
#include "mod.h"
#include "thread/mod.h"

static int current_idx = 0;
extern void switch_to(uint32_t** old_esp, uint32_t* new_esp);

void schedule(void) {
    if (thread_count == 0) return;
    thread_t* old_thread = current_thread;
    current_idx = (current_idx + 1) % thread_count;
    thread_t* next_thread = thread_queue[current_idx];
    current_thread = next_thread;
    next_thread->state = THREAD_RUNNING;
    switch_to(&old_thread->esp, next_thread->esp);
}
