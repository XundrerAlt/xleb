// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "halt.h"
#include "mm/mod.h"
#include "mm/ppage/mod.h"
#include "mm/virtconv.h"
#include "mm/vmm/lmem.h"
#include "mm/vmm/mod.h"
#include "stdint.h"

static uint32_t lmem_page_tables[128][1024] __attribute__((aligned(4096)));
extern uint32_t kernel_page_directory[1024];

void map_lmem(void) {
    uint32_t pstart = LMEM_START_PADDR;
    uint32_t vstart = PHYS_TO_VIRT(pstart);
    uint32_t start_pde = vstart >> 22;
    uint32_t lmem_size = LMEM_MAX_SIZE;
    if (lmem_size >= memory_map.total_memory) {
        lmem_size = memory_map.total_memory;
    }
    if (lmem_size <= pstart) {
        ERROR("No memory to map: lmem_size=0x%x <= pstart=0x%x", lmem_size, pstart);
        halt();
    }
    uint32_t pde_count = lmem_size / LMEM_PDE_SIZE;
    for (uint32_t pde_idx = start_pde; pde_idx < start_pde + pde_count; pde_idx++) {
        uint32_t table_idx = pde_idx - start_pde;
        uint32_t table_phys = VIRT_TO_PHYS((uint32_t)&lmem_page_tables[table_idx]);
        kernel_page_directory[pde_idx] = table_phys | 0x003;
        for (uint32_t pte_idx = 0; pte_idx < 1024; pte_idx++) {
            uint32_t current_phys = pstart + (table_idx * 1024 + pte_idx) * PAGE_SIZE;
            lmem_page_tables[table_idx][pte_idx] = current_phys | 0x003;
        }
    }
    __asm__ volatile("mov %%cr3, %%eax; mov %%eax, %%cr3" : : : "eax");
}