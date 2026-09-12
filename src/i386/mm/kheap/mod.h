#pragma once
#include "stdint.h"
#include "list.h"
#define KHEAP_INITIAL_PAGES 4

typedef struct heap_block {
    uint32_t magic;
    uint32_t size;
    uint8_t used;
    struct heap_block *next;
    struct heap_block *prev;
} heap_block_t;

typedef struct heap_stats {
    uint32_t total_size;
    uint32_t used_size;
    uint32_t free_size;
    uint32_t blocks_count;
    uint32_t free_blocks;
} heap_stats_t;

extern heap_block_t *heap_start;
extern heap_stats_t heap_stats;

int expand_heap(int is_initial);
void kfree(void *ptr);
void kheap_init(void);
void* kmalloc(uint32_t size);