// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "halt.h"
#include "task/scheduler/mod.h"
#include "task/thread/mod.h"

thread_t* thread_queue[16];
int thread_count = 0;
thread_t* current_thread = NULL;

void idle() {
    DEBUG("idle thread: hello world");
    while (1) {
        halt();
    }
}

void scheduler_init() {
    INFO("scheduler: hello world");
    current_thread = thread_create(idle);
    current_thread->state = THREAD_RUNNING;
    thread_queue[thread_count++] = current_thread;
}