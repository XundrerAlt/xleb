// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "halt.h"
#include "list.h"
#include "mm/mod.h"
#include "mm/ppage/mod.h"
#include "mm/virtconv.h"
#include "stdint.h"

memory_map_t memory_map;
free_area_t free_areas[MAX_ORDER + 1];
struct page *page_array = NULL;
uint32_t total_pages = 0;

static uint32_t find_mem_for_page_array(uint32_t size) {
    for (int i = 0; i < memory_map.region_count; i++) {
        memory_region_t *region = &memory_map.regions[i];
        if (region->type == MEMORY_AVAILABLE) {
            if (region->size >= size) {
                uint32_t page_array_pstart = region->start;
                INFO("First %d KB is reserved for page array", size / 1024);
                region->start += size;
                region->size -= size;
                memory_map.available_memory -= size;
                return page_array_pstart;
            }
        }
    }
    ERROR("Not enough memory for page array! Need %u bytes", size);
    halt();
    return 0;
}

void ppage_init(void) {
    total_pages = memory_map.available_memory / PAGE_SIZE;
    uint32_t page_array_size = total_pages * sizeof(struct page);
    uint32_t page_array_vaddr = PHYS_TO_VIRT(find_mem_for_page_array(page_array_size));
    page_array = (struct page*)page_array_vaddr;
    memset(page_array, 0, page_array_size);
    for (int i = 0; i <= MAX_ORDER; i++) {
        INIT_LIST_HEAD(&free_areas[i].free_list);
        free_areas[i].nr_free = 0;
    }
    for (int i = 0; i < memory_map.region_count; i++) {
        memory_region_t *region = &memory_map.regions[i];
        if (region->type == MEMORY_AVAILABLE) {
            uint32_t start = region->start;
            uint32_t page_num = region->size / PAGE_SIZE;
            ppage_add_region(start, page_num);
        }
    }
}