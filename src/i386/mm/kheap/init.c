// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "halt.h"
#include "mm/kheap/mod.h"
#include "mm/ppage/mod.h"
#include "mm/virtconv.h"
#include "mm/vmm/lmem.h"
#include "mm/vmm/mod.h"
#include "stddef.h"
#include "stdint.h"
#include "string.h"

extern uint32_t kernel_page_directory[1024];
#define KHEAP_START 0xC0400000 + LMEM_MAX_SIZE
#define KHEAP_LIMIT (256 * 1024 * 1024)
static uint32_t heap_brk = KHEAP_START;
heap_block_t *heap_start = NULL;
heap_stats_t heap_stats = {0};

int expand_heap(int is_initial) {
    if (heap_brk + PAGE_SIZE > KHEAP_START + KHEAP_LIMIT) {
        ERROR("kheap: out of memory");
        return -1;
    }
    uint32_t flags = is_initial ? PPAGE_LOWLEVEL_FLAG : 0;
    if (!vpage_alloc(kernel_page_directory, heap_brk, 0, flags)) {
        return -1;
    }
    heap_block_t *block = (heap_block_t*)heap_brk;
    block->magic = 0xC0C0D0AF;
    block->size = PAGE_SIZE - sizeof(heap_block_t);
    block->used = 0;
    block->next = NULL;
    block->prev = NULL;
    if (heap_start) {
        heap_block_t *last = heap_start;
        while (last->next) last = last->next;
        last->next = block;
        block->prev = last;
    } else {
        heap_start = block;
    }
    heap_stats.total_size += PAGE_SIZE;
    heap_stats.free_size += block->size;
    heap_stats.blocks_count++;
    heap_stats.free_blocks++;
    heap_brk += PAGE_SIZE;
    return 0;
}

void kheap_init() {
    memset(&heap_stats, 0, sizeof(heap_stats_t));
    heap_brk = KHEAP_START;
    for (int i = 0; i < KHEAP_INITIAL_PAGES; i++) {
        if (expand_heap(1)) {
            ERROR("kheap: init failed at page %d", i);
            break;
        }
    }
    INFO("kheap: %u KB at 0x%x", KHEAP_INITIAL_PAGES * 4, KHEAP_START);
}