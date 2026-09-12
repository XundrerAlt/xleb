// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "mm/kheap/mod.h"
#include "mm/ppage/mod.h"
#include "mm/virtconv.h"
#include "mm/vmm/mod.h"
#include "stddef.h"
#include "task/thread/mod.h"

#define STACK_SIZE 4096
extern uint32_t kernel_page_directory[1024];

thread_t* thread_create(void (*entry)()) {
    thread_t* thread = (thread_t*)kmalloc(sizeof(thread_t));
    if (!thread) return NULL;
    void* stack = vmalloc(kernel_page_directory, STACK_SIZE, 0);
    if (!stack) {
        kfree(thread);
        return NULL;
    }
    thread->stack_limit = stack;
    thread->state = THREAD_READY;
    uint32_t* esp = (uint32_t*)((uint8_t*)stack + STACK_SIZE);
    *(--esp) = (uint32_t)entry;
    *(--esp) = 0;
    *(--esp) = 0;
    *(--esp) = 0;
    *(--esp) = 0;
    *(--esp) = 0x202;
    thread->esp = esp;
    return thread;
}