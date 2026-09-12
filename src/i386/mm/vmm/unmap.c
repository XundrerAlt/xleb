// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "mm/ppage/mod.h"
#include "mm/vmm/mod.h"
#include "mm/virtconv.h"
#include "stdint.h"

void vmm_unmap(uint32_t *page_directory, uint32_t virt_addr) {
    uint32_t pde_index = virt_addr >> 22;
    uint32_t pte_index = (virt_addr >> 12) & 0x3FF;
    if (!(page_directory[pde_index] & PAGE_PRESENT)) {
        return;
    }
    uint32_t table_phys = page_directory[pde_index] & ~0xFFF;
    uint32_t *page_table = (uint32_t*)PHYS_TO_VIRT(table_phys);
    if (!(page_table[pte_index] & PAGE_PRESENT)) {
        return;
    }
    page_table[pte_index] = 0;
    __asm__ volatile("invlpg (%0)" : : "r"(virt_addr));
}