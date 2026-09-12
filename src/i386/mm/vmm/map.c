// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "mm/ppage/mod.h"
#include "mm/vmm/mod.h"
#include "mm/virtconv.h"
#include "stdint.h"

void vmm_map(uint32_t *page_directory, uint32_t vaddr, uint32_t paddr, uint32_t flags) {
    uint32_t pde_index = vaddr >> 22;
    uint32_t pte_index = (vaddr >> 12) & 0x3FF;
    if (!(page_directory[pde_index] & PAGE_PRESENT)) {
        uint32_t table_phys = (uint32_t)ppage_alloc(0, 0);
        if (!table_phys) {
            ERROR("Failed to allocate page table");
            return;
        }
        uint32_t *page_table = (uint32_t*)PHYS_TO_VIRT(table_phys);
        for (int i = 0; i < 1024; i++) {
            page_table[i] = 0;
        }
        page_directory[pde_index] = table_phys | PAGE_PRESENT | PAGE_WRITE;
    }
    uint32_t table_phys = page_directory[pde_index] & ~0xFFF;
    uint32_t *page_table = (uint32_t*)PHYS_TO_VIRT(table_phys);
    page_table[pte_index] = paddr | flags;
    __asm__ volatile("invlpg (%0)" : : "r"(vaddr));
}