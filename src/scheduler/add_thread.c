// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "halt.h"
#include "mod.h"
#include "thread/mod.h"

void scheduler_add_thread(void (*entry)()) {
    thread_t* thread = thread_create(entry);
    if (thread) {
        thread_queue[thread_count++] = thread;
    }
}
