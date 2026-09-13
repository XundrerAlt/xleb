// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "mm/mod.h"
#include "mm/virtconv.h"
#include "stddef.h"
#include "mod.h"

#define STACK_SIZE 4096
extern uint32_t kernel_page_directory[1024];

thread_t* thread_create(void (*entry)()) {
    uint32_t thread_addr = retype(OBJ_THREAD, 0);
    if (!thread_addr) {
        ERROR("thread_create: retype TCB failed");
        return 0;
    }
    thread_t *thread = (thread_t*)PHYS_TO_VIRT(thread_addr);
    memset(thread, 0, sizeof(thread_t));
    thread->pd_addr = create_page_directory();
    if (!thread->pd_addr) {
        ERROR("thread_create: create PD failed");
        return 0;
    }
    DEBUG("thread->pd_addr = 0x%x", thread->pd_addr);
    uint32_t stack_phys = untyped_alloc(STACK_SIZE);
    if (!stack_phys) {
        ERROR("thread_create: alloc stack failed");
        return 0;
    }
    uint32_t stack_virt = PHYS_TO_VIRT(stack_phys);
    thread->stack_limit = (void*)stack_virt;
    thread->state = THREAD_READY;
    uint32_t* esp = (uint32_t*)((uint8_t*)stack_virt + STACK_SIZE);
    *(--esp) = (uint32_t)entry;
    *(--esp) = 0x202;
    *(--esp) = 0;
    *(--esp) = 0;
    *(--esp) = 0;
    *(--esp) = 0;
    thread->esp = esp;
    return thread;
}
