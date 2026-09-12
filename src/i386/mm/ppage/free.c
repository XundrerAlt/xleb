// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "mm/virtconv.h"
#include "mm/ppage/mod.h"
#include "stdint.h"

static void add_to_free_list(uint32_t start_page, uint32_t order) {
    list_add(&page_array[start_page].list, &free_areas[order].free_list);
    free_areas[order].nr_free++;
}
static int is_block_used(uint32_t start_page, uint32_t num_pages) {
    for (uint32_t i = 0; i < num_pages; i++) {
        if (page_array[start_page + i].flags != PAGE_USED) {
            return 0;
        }
    }
    return 1;
}
static uint32_t try_merge_with_buddy(uint32_t *start_page, uint32_t *order) {
    uint32_t block_size = 1 << *order;
    uint32_t buddy_page = *start_page ^ block_size;
    if (buddy_page + block_size > total_pages) {
        return 0;
    }
    struct page *buddy = &page_array[buddy_page];
    if (buddy->flags != PAGE_FREE || buddy->order != *order) {
        return 0;
    }
    list_del(&buddy->list);
    free_areas[*order].nr_free--;
    if (buddy_page < *start_page) {
        *start_page = buddy_page;
    }
    (*order)++;
    return 1;
}
static uint32_t merge_blocks(uint32_t *start_page, uint32_t order) {
    uint32_t current_order = order;
    uint32_t current_page = *start_page;
    while (current_order < MAX_ORDER) {
        if (!try_merge_with_buddy(&current_page, &current_order)) {
            break;
        }
    }
    *start_page = current_page;
    return current_order;
}
static void mark_block_free(uint32_t start_page, uint32_t block_size, uint32_t order) {
    for (uint32_t i = 0; i < block_size; i++) {
        page_array[start_page + i].flags = PAGE_FREE;
        page_array[start_page + i].order = order;
        INIT_LIST_HEAD(&page_array[start_page + i].list);
    }
}

void ppage_free(void *ptr, uint32_t order) {
    if (ptr == NULL || order > MAX_ORDER) {
        ERROR("Invalid free: ptr=0x%x, order=%u", (uint32_t)ptr, order);
        return;
    }
    uint32_t block_paddr = (uint32_t)ptr;
    uint32_t start_page = block_paddr / PAGE_SIZE;
    uint32_t num_pages = 1 << order;
    if (!is_block_used(start_page, num_pages)) {
        ERROR("Double free or invalid free at phys=0x%x", block_paddr);
        return;
    }
    uint32_t merged_page = start_page;
    uint32_t final_order = merge_blocks(&merged_page, order);
    uint32_t final_size = 1 << final_order;
    mark_block_free(merged_page, final_size, final_order);
    add_to_free_list(merged_page, final_order);
}