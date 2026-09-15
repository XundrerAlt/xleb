// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "halt.h"
#include "mm/mod.h"
#include "mm/virtconv.h"
#include "cap/mod.h"
#include "string.h"

#define TEMP_WINDOWS 16
#define TEMP_WINDOW_BASE 0xE0000000
#define TEMP_WINDOW_SIZE 0x1000
extern uint32_t _kernel_end;
extern uint32_t kernel_page_directory[1024];
static uint8_t temp_pt[PAGE_SIZE] __attribute__((aligned(4096)));
static int temp_used[TEMP_WINDOWS] = {0};

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
    temp_unmap(table);
    return table_phys;
}

static inline uint32_t get_cr3(void) {
    uint32_t cr3;
    __asm__ volatile("mov %%cr3, %0" : "=r"(cr3));
    return cr3;
}

static inline uint32_t *get_current_pd(void) {
    return (uint32_t*)PHYS_TO_VIRT(get_cr3());
}

static int temp_window_index(uint32_t addr) {
    if (addr < TEMP_WINDOW_BASE) return -1;
    uint32_t off = addr - TEMP_WINDOW_BASE;
    if (off % TEMP_WINDOW_SIZE != 0) return -1;
    uint32_t i = off / TEMP_WINDOW_SIZE;
    if (i >= TEMP_WINDOWS) return -1;
    return (int)i;
}

static inline uint32_t temp_window_addr(int i) {
    return TEMP_WINDOW_BASE + (uint32_t)i * TEMP_WINDOW_SIZE;
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
    for (int i = 768; i < 1024; i++) {
        pd[i] = kernel_page_directory[i];
    }
    temp_unmap(pd);
    return pd_phys;
}

int map_page(uint32_t pd_phys, uint32_t virt, uint32_t phys, uint32_t flags) {
    uint32_t current_pd_phys = get_cr3();
    int is_current = (pd_phys == 0 || pd_phys == current_pd_phys);
    if (pd_phys == 0) pd_phys = current_pd_phys;

    uint32_t *pd = (uint32_t*)temp_map(pd_phys);
    if (!pd) return -1;

    uint32_t pd_index = virt >> 22;
    uint32_t pt_index = (virt >> 12) & 0x3FF;

    uint32_t pt_phys;
    if (!(pd[pd_index] & PAGE_PRESENT)) {
        pt_phys = create_page_table();
        if (!pt_phys) { temp_unmap(pd); return -1; }
        uint32_t pde_flags = PAGE_PRESENT | PAGE_WRITE;
        if (flags & PAGE_USER) pde_flags |= PAGE_USER;
        pd[pd_index] = pt_phys | pde_flags;
    } else {
        pt_phys = pd[pd_index] & ~0xFFF;
        if (flags & PAGE_USER) {
            pd[pd_index] |= PAGE_USER;
        }
    }
    temp_unmap(pd);

    uint32_t *pt = (uint32_t*)temp_map(pt_phys);
    if (!pt) return -1;
    pt[pt_index] = (phys & ~0xFFF) | (flags & 0xFFF);
    temp_unmap(pt);

    if (is_current) {
        __asm__ volatile("invlpg (%0)" : : "r" (virt) : "memory");
    }
    return 0;
}

void unmap_page(uint32_t pd_phys, uint32_t virt) {
    uint32_t current_pd_phys = get_cr3();
    int is_current = (pd_phys == 0 || pd_phys == current_pd_phys);
    if (pd_phys == 0) pd_phys = current_pd_phys;

    uint32_t *pd = (uint32_t*)temp_map(pd_phys);
    if (!pd) return;
    uint32_t pd_index = virt >> 22;
    uint32_t pt_index = (virt >> 12) & 0x3FF;

    if (!(pd[pd_index] & PAGE_PRESENT)) { temp_unmap(pd); return; }
    uint32_t pt_phys = pd[pd_index] & ~0xFFF;
    temp_unmap(pd);

    uint32_t *pt = (uint32_t*)temp_map(pt_phys);
    if (!pt) return;
    pt[pt_index] = 0;
    temp_unmap(pt);

    if (is_current) {
        __asm__ volatile("invlpg (%0)" : : "r" (virt) : "memory");
    }
}

void temp_map_init(void) {
    memset(temp_pt, 0, PAGE_SIZE);
    uint32_t *pd = get_current_pd();
    uint32_t pd_index = TEMP_WINDOW_BASE >> 22;
    pd[pd_index] = VIRT_TO_PHYS((uint32_t)temp_pt) | PAGE_PRESENT | PAGE_WRITE;
    __asm__ volatile("invlpg (%0)" : : "r" (TEMP_WINDOW_BASE) : "memory");
    INFO("temp map: initialized %d windows at 0x%x", TEMP_WINDOWS, TEMP_WINDOW_BASE);
}

void *temp_map(uint32_t phys) {
    uint32_t *pd = get_current_pd();
    uint32_t pd_index = TEMP_WINDOW_BASE >> 22;
    if (!(pd[pd_index] & PAGE_PRESENT)) {
        ERROR("temp_map: PT not initialized");
        return 0;
    }
    int i;
    for (i = 0; i < TEMP_WINDOWS; i++) {
        if (!temp_used[i]) break;
    }
    if (i == TEMP_WINDOWS) {
        ERROR("temp_map: no free window");
        return 0;
    }
    temp_used[i] = 1;
    uint32_t pt_phys = pd[pd_index] & ~0xFFF;
    uint32_t *pt = (uint32_t*)PHYS_TO_VIRT(pt_phys);
    uint32_t addr = temp_window_addr(i);
    uint32_t pt_index = (addr >> 12) & 0x3FF;
    pt[pt_index] = (phys & ~0xFFF) | PAGE_PRESENT | PAGE_WRITE;
    __asm__ volatile("invlpg (%0)" : : "r" (addr) : "memory");
    return (void*)addr;
}

void temp_unmap(void *addr) {
    if (!addr) return;
    int i = temp_window_index((uint32_t)addr);
    if (i < 0 || !temp_used[i]) return;
    uint32_t *pd = get_current_pd();
    uint32_t pd_index = TEMP_WINDOW_BASE >> 22;
    if (!(pd[pd_index] & PAGE_PRESENT)) return;
    uint32_t pt_phys = pd[pd_index] & ~0xFFF;
    uint32_t *pt = (uint32_t*)PHYS_TO_VIRT(pt_phys);
    uint32_t pt_index = ((uint32_t)addr >> 12) & 0x3FF;
    pt[pt_index] = 0;
    __asm__ volatile("invlpg (%0)" : : "r" (addr) : "memory");
    temp_used[i] = 0;
}
