// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#pragma once
#include "stdint.h"
#include "process/mod.h"

typedef enum {
    THREAD_READY,
    THREAD_RUNNING,
    THREAD_DEAD
} thread_state_t;

typedef struct {
    uint32_t edi;
    uint32_t esi;
    uint32_t ebx;
    uint32_t ebp;
    uint32_t eip;
    uint32_t esp;
    uint32_t eflags;
    uint32_t ss;
} thread_context_t;

typedef struct {
    thread_context_t ctx;
    process_t* process;
    uint32_t id;
    thread_state_t state;
    void* stack_limit;
    void* kernel_stack_top;
    void* user_stack_top;
    uint8_t is_user;
} thread_t;

extern thread_t* current_thread;
thread_t* thread_create(void (*entry)(), uint8_t is_user, uint32_t prog_size, process_t *process);
void thread_destroy(thread_t *target);
void switch_to(thread_t *prev, thread_t *next);
void switch_to_first(thread_t *next);
void archspec_schedule(thread_t *next);
