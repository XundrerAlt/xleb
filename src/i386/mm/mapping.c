// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "halt.h"
#include "mm/mod.h"
#include "mm/virtconv.h"
#include "cap/mod.h"
#include "string.h"

extern uint32_t _kernel_end;
static uint8_t temp_pt[PAGE_SIZE] __attribute__((aligned(4096)));

static uint32_t create_page_table(void) {
    uint32_t table_phys = untyped_alloc(PAGE_SIZE);
    if (!table_phys) {
        ERROR("create_page_table: out of memory");
        return 0;
    }
    uint32_t *table = (uint32_t*)temp_map(table_phys);
    if (!table) {
        ERROR("create_page_table: temp_map failed");
        return 0;
    }
    memset(table, 0, PAGE_SIZE);
    temp_unmap();
    return table_phys;
}

static inline uint32_t *get_current_pd(void) {
    uint32_t pd_phys;
    __asm__ volatile("mov %%cr3, %0" : "=r" (pd_phys));
    return (uint32_t*)PHYS_TO_VIRT(pd_phys);
}

uint32_t create_page_directory(void) {
    uint32_t pd_phys = retype(OBJ_VNODE, PAGE_SIZE);
    if (!pd_phys) {
        ERROR("create_page_directory: out of memory");
        return 0;
    }
    uint32_t *pd = (uint32_t*)temp_map(pd_phys);
    if (!pd) {
        ERROR("create_page_directory: temp_map failed");
        return 0;
    }
    memset(pd, 0, PAGE_SIZE);
    extern uint32_t kernel_page_directory[1024];
    for (int i = 768; i < 1024; i++) {
        pd[i] = kernel_page_directory[i];
    }
    temp_unmap();
    return pd_phys;
}

void map_page(uint32_t virt, uint32_t phys, uint32_t flags) {
    uint32_t *pd = get_current_pd();
    uint32_t pd_index = virt >> 22;
    uint32_t pt_index = (virt >> 12) & 0x3FF;

    if (!(pd[pd_index] & PAGE_PRESENT)) {
        uint32_t pt_phys = create_page_table();
        if (!pt_phys) return;
        pd[pd_index] = pt_phys | PAGE_PRESENT | PAGE_WRITE;
    }

    uint32_t pt_phys = pd[pd_index] & ~0xFFF;
    uint32_t *pt = (uint32_t*)PHYS_TO_VIRT(pt_phys);
    pt[pt_index] = (phys & ~0xFFF) | (flags & 0xFFF);

    __asm__ volatile("invlpg (%0)" : : "r" (virt) : "memory");
}

void unmap_page(uint32_t virt) {
    uint32_t *pd = get_current_pd();
    uint32_t pd_index = virt >> 22;
    uint32_t pt_index = (virt >> 12) & 0x3FF;

    if (!(pd[pd_index] & PAGE_PRESENT)) return;

    uint32_t pt_phys = pd[pd_index] & ~0xFFF;
    uint32_t *pt = (uint32_t*)PHYS_TO_VIRT(pt_phys);
    pt[pt_index] = 0;

    __asm__ volatile("invlpg (%0)" : : "r" (virt) : "memory");
}

void temp_map_init(void) {
    memset(temp_pt, 0, PAGE_SIZE);
    uint32_t *pd = get_current_pd();
    uint32_t pd_index = TEMP_WINDOW >> 22;
    pd[pd_index] = VIRT_TO_PHYS((uint32_t)temp_pt) | PAGE_PRESENT | PAGE_WRITE;
    INFO("temp map: hello world");
}

void *temp_map(uint32_t phys) {
    uint32_t *pd = get_current_pd();
    uint32_t pd_index = TEMP_WINDOW >> 22;
    if (!(pd[pd_index] & PAGE_PRESENT)) {
        ERROR("temp map: PT not initialized");
        return 0;
    }
    uint32_t pt_phys = pd[pd_index] & ~0xFFF;
    uint32_t *pt = (uint32_t*)PHYS_TO_VIRT(pt_phys);
    uint32_t pt_index = (TEMP_WINDOW >> 12) & 0x3FF;
    pt[pt_index] = (phys & ~0xFFF) | PAGE_PRESENT | PAGE_WRITE;
    __asm__ volatile("invlpg (%0)" : : "r" (TEMP_WINDOW) : "memory");
    return (void*)TEMP_WINDOW;
}

void temp_unmap(void) {
    unmap_page(TEMP_WINDOW);
}
