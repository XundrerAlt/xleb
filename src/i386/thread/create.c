// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "mm/mod.h"
#include "mm/virtconv.h"
#include "stddef.h"
#include "mod.h"

#define STACK_SIZE 4096
#define USER_STACK_ADDR 0x40000000

thread_t* thread_create(void (*entry)(), uint8_t is_user) {
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
    uint32_t kstack_phys = untyped_alloc(STACK_SIZE);
    if (!kstack_phys) {
        ERROR("thread_create: alloc kernel stack failed");
        return 0;
    }
    uint32_t kstack_virt = PHYS_TO_VIRT(kstack_phys);
    thread->stack_limit = (void*)kstack_virt;
    thread->kernel_stack_top = (void*)((kstack_virt + STACK_SIZE) & ~0xF);
    if (is_user) {
        uint32_t ustack_phys = untyped_alloc(STACK_SIZE);
        if (!ustack_phys) {
            ERROR("thread_create: alloc user stack failed");
            return 0;
        }
        if (map_page(thread->pd_addr, USER_STACK_ADDR, ustack_phys, PAGE_PRESENT | PAGE_WRITE | PAGE_USER) != 0) {
            ERROR("thread_create: map user stack failed");
            return 0;
        }
        uint32_t entry_addr = (uint32_t)entry;
        uint32_t page = entry_addr & ~0xFFF;
        uint32_t offset = entry_addr & 0xFFF;
        uint32_t phys = VIRT_TO_PHYS(page);
        uint32_t uaddr = 0x41000000;
        if (map_page(thread->pd_addr, uaddr, phys, PAGE_PRESENT | PAGE_USER) != 0) {
            ERROR("thread_create: map user address failed");
            return 0;
        }
        uint32_t ustack_top = (USER_STACK_ADDR + STACK_SIZE) & ~0xF;
        uint32_t *tmp = (uint32_t*)temp_map(ustack_phys);
        if (!tmp) { ERROR("Failed to temp map user stack"); return 0; }
        uint32_t *sp = tmp + (STACK_SIZE / 4);
        *(--sp) = 0x23;
        *(--sp) = ustack_top;
        *(--sp) = 0x202;
        *(--sp) = 0x1B;
        *(--sp) = uaddr + offset;
        thread->ctx.esp = USER_STACK_ADDR + STACK_SIZE - 20;
        thread->ctx.eip = uaddr + offset;
        thread->ctx.eflags = 0x202;
        thread->ctx.ss = 0x23;
        thread->user_stack_top = (void*)ustack_top;
        thread->is_user = 1;
        temp_unmap(tmp);
    } else {
        uint32_t stack_top = (kstack_virt + STACK_SIZE) & ~0xF;
        uint32_t *sp = (uint32_t*)stack_top;
        *(--sp) = 0x202;
        *(--sp) = 0x08;
        *(--sp) = (uint32_t)entry;
        thread->ctx.esp = (uint32_t)sp;
        thread->ctx.eip = (uint32_t)entry;
        thread->ctx.eflags = 0x202;
        thread->ctx.ss = 0x10;
        thread->user_stack_top = 0;
        thread->is_user = 0;
    }
    thread->state = THREAD_READY;
    return thread;
}
