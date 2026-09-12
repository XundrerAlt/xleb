// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#pragma once
#include "stdint.h"
#include "list.h"
#define MAX_ORDER 16
#define PAGE_SIZE 4096
#define PPAGE_LOWLEVEL_FLAG 0x2
enum page_flag {
    PAGE_FREE,
    PAGE_USED,
    PAGE_RESERVED
};
typedef struct free_area {
    struct list_head free_list;
    uint32_t nr_free;
} free_area_t;
struct page {
    struct list_head list;
    uint32_t order;
    uint32_t flags;
};
extern free_area_t free_areas[MAX_ORDER + 1];
extern struct page *page_array;
extern uint32_t total_pages;

void ppage_add_region(uint32_t start, uint32_t page_num);
void* ppage_alloc(uint32_t order, uint32_t flags);
void ppage_free(void *ptr, uint32_t order);
void ppage_init(void);