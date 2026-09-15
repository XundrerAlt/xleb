// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "mm/mod.h"
#include "mm/virtconv.h"
#include "stddef.h"
#include "mod.h"
#include "string.h"
#include "space/mod.h"

#define STACK_SIZE 4096
#define USER_STACK_ADDR 0x40000000
#define USER_CODE_ADDR 0x41000000

thread_t* thread_create(void (*entry)(), uint8_t is_user, uint32_t prog_size, space_t *space) {
    uint32_t thread_addr = retype(OBJ_THREAD, 0);
    if (!thread_addr) {
        ERROR("thread_create: retype TCB failed");
        return 0;
    }
    thread_t *thread = (thread_t*)PHYS_TO_VIRT(thread_addr);
    memset(thread, 0, sizeof(thread_t));
    if (space) {
        thread->space = space;
    } else {
        thread->space = space_create();
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
        map_page(thread->space->pd_addr, USER_STACK_ADDR, ustack_phys, PAGE_PRESENT | PAGE_WRITE | PAGE_USER);

        uint32_t blob_phys = (uint32_t)entry;
        uint32_t pages = (prog_size + 0xFFF) / 0x1000;
        for (uint32_t i = 0; i < pages; i++) {
            uint32_t phys = untyped_alloc(0x1000);
            void *tmp = temp_map(phys);
            void *src = temp_map(blob_phys + i * 0x1000);
            uint32_t off = i * 0x1000;
            uint32_t copy = (prog_size - off > 0x1000) ? 0x1000 : (prog_size - off);
            memcpy(tmp, src, copy);
            memset((uint8_t*)tmp + copy, 0, 0x1000 - copy);
            temp_unmap(src);
            temp_unmap(tmp);
            map_page(thread->space->pd_addr, USER_CODE_ADDR + off, phys, PAGE_PRESENT | PAGE_USER);
        }

        uint32_t *ustack_tmp = (uint32_t*)temp_map(ustack_phys);
        uint32_t *sp = ustack_tmp + (STACK_SIZE / 4);
        *(--sp) = 0x23;
        *(--sp) = USER_STACK_ADDR + STACK_SIZE;
        *(--sp) = 0x202;
        *(--sp) = 0x1B;
        *(--sp) = USER_CODE_ADDR;
        temp_unmap(ustack_tmp);

        thread->ctx.esp = USER_STACK_ADDR + STACK_SIZE - 20;
        thread->ctx.eip = USER_CODE_ADDR;
        thread->ctx.eflags = 0x202;
        thread->ctx.ss = 0x23;
        thread->is_user = 1;
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
