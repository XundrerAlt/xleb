// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "halt.h"
#include "mod.h"
#include "thread/mod.h"

thread_t* thread_queue[16];
int thread_count = 0;
thread_t* current_thread = NULL;

void idle(void) {
    DEBUG("idle thread: hello world");
    while (1) {
        halt();
    }
}

void scheduler_init(void) {
    INFO("scheduler: hello world");
    thread_t* idle_thread = thread_create(idle, 0);
    idle_thread->state = THREAD_RUNNING;
    thread_queue[thread_count++] = current_thread;
}
