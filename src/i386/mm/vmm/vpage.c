// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "mm/ppage/mod.h"
#include "mm/vmm/mod.h"
#include "mm/virtconv.h"
#include "stdint.h"

void* vpage_alloc(uint32_t *page_directory, uint32_t vaddr, uint32_t order, uint32_t flags) {
    if (!page_directory) {
        ERROR("vpage_alloc: page_directory is NULL");
        return NULL;
    }
    uint32_t paddr = (uint32_t)ppage_alloc(order, flags);
    if (!paddr) {
        ERROR("vpage_alloc: ppage_alloc failed for vaddr=0x%x", vaddr);
        return NULL;
    }
    vmm_map(page_directory, vaddr, paddr, PAGE_PRESENT | PAGE_WRITE);
    return (void*)vaddr;
}

void vpage_free(uint32_t *page_directory, uint32_t vaddr, uint32_t order) {
    if (!page_directory || !vaddr) {
        ERROR("vpage_free: invalid parameters");
        return;
    }
    uint32_t num_pages = 1 << order;
    for (uint32_t i = 0; i < num_pages; i++) {
        vmm_unmap(page_directory, vaddr + i * PAGE_SIZE);
    }
}