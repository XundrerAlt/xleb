// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "mm/virtconv.h"
#include "mm/ppage/mod.h"
#include "stdint.h"

static void add_block_to_free_list(uint32_t start_page, uint32_t order) {
    list_add(&page_array[start_page].list, &free_areas[order].free_list);
    free_areas[order].nr_free++;
}
static uint32_t get_max_order(uint32_t page_idx, uint32_t max_pages) {
    uint32_t order = 0;
    uint32_t block_size = 1;
    while (order < MAX_ORDER) {
        uint32_t next_size = block_size << 1;
        if (next_size > max_pages) break;
        if ((page_idx & (next_size - 1)) != 0) break;
        order++;
        block_size = next_size;
    }
    return order;
}
static void init_block_pages(uint32_t start_page, uint32_t block_size, uint32_t order) {
    for (uint32_t i = 0; i < block_size; i++) {
        uint32_t page_idx = start_page + i;
        page_array[page_idx].flags = PAGE_FREE;
        page_array[page_idx].order = order;
        INIT_LIST_HEAD(&page_array[page_idx].list);
    }
}

void ppage_add_region(uint32_t start_paddr, uint32_t page_num) {
    uint32_t start_page = start_paddr / PAGE_SIZE;
    uint32_t remaining = page_num;
    uint32_t current_page = start_page;
    while (remaining > 0) {
        uint32_t order = get_max_order(current_page, remaining);
        uint32_t block_size = 1 << order;
        init_block_pages(current_page, block_size, order);
        add_block_to_free_list(current_page, order);
        current_page += block_size;
        remaining -= block_size;
    }
}