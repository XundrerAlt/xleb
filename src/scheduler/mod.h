// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#pragma once
#include "thread/mod.h"

extern thread_t* thread_queue[16];
extern int thread_count;
extern thread_t* current_thread;

void scheduler_add_thread(void (*entry)(), uint8_t is_user);
void scheduler_init(void);
void schedule(void);
