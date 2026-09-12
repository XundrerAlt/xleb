// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#pragma once
#include "stdint.h"
#define PAGE_PRESENT  0x001
#define PAGE_WRITE    0x002
#define PAGE_USER     0x004
#define PAGE_WRITETHRU 0x008
#define PAGE_CACHE_DIS 0x010
#define PAGE_ACCESSED 0x020
#define PAGE_DIRTY    0x040
#define PAGE_PAT      0x080
#define PAGE_GLOBAL   0x100

void vmm_map(uint32_t *page_directory, uint32_t virt_addr, uint32_t phys_addr, uint32_t flags);
void vmm_unmap(uint32_t *page_directory, uint32_t virt_addr);
void* vpage_alloc(uint32_t *page_directory, uint32_t vaddr, uint32_t order, uint32_t flags);
void vpage_free(uint32_t *page_directory, uint32_t vaddr, uint32_t order);
void* vmalloc(uint32_t *page_directory, uint32_t order, uint32_t flags);
void vfree(uint32_t *page_directory, void* ptr, uint32_t size);