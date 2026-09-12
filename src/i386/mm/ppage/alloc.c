// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "mm/virtconv.h"
#include "mm/ppage/mod.h"
#include "mm/vmm/lmem.h"
#include "stdint.h"

static struct page* get_free_block(uint32_t order) {
    if (list_empty(&free_areas[order].free_list)) return NULL;
    struct list_head *first = free_areas[order].free_list.next;
    struct page *page = list_entry(first, struct page, list);
    list_del(&page->list);
    free_areas[order].nr_free--;
    return page;
}
static void mark_pages_used(uint32_t start_page, uint32_t num_pages) {
    for (uint32_t i = 0; i < num_pages; i++) {
        page_array[start_page + i].flags = PAGE_USED;
    }
}
static void split_block(uint32_t start_page, uint32_t start_order, uint32_t target_order) {
    uint32_t current_order = start_order;
    while (current_order > target_order) {
        current_order--;
        uint32_t buddy_page = start_page + (1 << current_order);
        struct page *buddy = &page_array[buddy_page];
        buddy->flags = PAGE_FREE;
        buddy->order = current_order;
        INIT_LIST_HEAD(&buddy->list);
        list_add(&buddy->list, &free_areas[current_order].free_list);
        free_areas[current_order].nr_free++;
    }
    page_array[start_page].order = target_order;
}

void* ppage_alloc(uint32_t order, uint32_t flags) {
    for (uint32_t cur_order = order; cur_order <= MAX_ORDER; cur_order++) {
        struct page *page = get_free_block(cur_order);
        if (!page) {
            continue;
        }
        uint32_t start_page = page - page_array;
        uint32_t block_paddr = start_page * PAGE_SIZE;
        if ((flags & PPAGE_LOWLEVEL_FLAG) && block_paddr >= LMEM_MAX_SIZE) {
            list_add(&page->list, &free_areas[cur_order].free_list);
            free_areas[cur_order].nr_free++;
            continue;
        }
        split_block(start_page, cur_order, order);
        mark_pages_used(start_page, 1 << order);
        return (void*)block_paddr;
    }
    return NULL;
}