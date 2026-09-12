// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "mm/ppage/mod.h"
#include "mm/vmm/mod.h"
#include "mm/virtconv.h"
#include "stdint.h"

#define VMALLOC_START 0xC2000000
#define VMALLOC_END   0xFFFFFFFF

static uint32_t vmalloc_next = VMALLOC_START;

void* vmalloc(uint32_t *page_directory, uint32_t size, uint32_t flags) {
    if (!page_directory) {
        ERROR("vmalloc: page_directory is NULL");
        return NULL;
    }
    if (size == 0) {
        ERROR("vmalloc: size is 0");
        return NULL;
    }
    uint32_t aligned_size = (size + PAGE_SIZE - 1) & ~(PAGE_SIZE - 1);
    uint32_t pages = aligned_size / PAGE_SIZE;
    uint32_t order = 0;
    while ((1 << order) < pages) order++;
    if (vmalloc_next + aligned_size > VMALLOC_END) {
        ERROR("vmalloc: out of virtual address space");
        return NULL;
    }
    uint32_t vaddr = vmalloc_next;
    vmalloc_next += aligned_size;
    for (uint32_t i = 0; i < pages; i++) {
        uint32_t paddr = (uint32_t)ppage_alloc(0, flags);
        if (!paddr) {
            ERROR("vmalloc: failed to allocate physical page %u", i);
            for (uint32_t j = 0; j < i; j++) {
                vpage_free(page_directory, vaddr + j * PAGE_SIZE, 0);
            }
            return NULL;
        }
        vmm_map(page_directory, vaddr + i * PAGE_SIZE, paddr, PAGE_PRESENT | PAGE_WRITE);
    }
    return (void*)vaddr;
}

void vfree(uint32_t *page_directory, void* ptr, uint32_t size) {
    if (!page_directory || !ptr || size == 0) {
        ERROR("vfree: invalid parameters");
        return;
    }
    uint32_t vaddr = (uint32_t)ptr;
    uint32_t aligned_size = (size + PAGE_SIZE - 1) & ~(PAGE_SIZE - 1);
    uint32_t pages = aligned_size / PAGE_SIZE;
    for (uint32_t i = 0; i < pages; i++) {
        vpage_free(page_directory, vaddr + i * PAGE_SIZE, 0);
    }
}