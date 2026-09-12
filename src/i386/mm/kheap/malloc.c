// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "mm/kheap/mod.h"
#include "mm/virtconv.h"
#include "stddef.h"
#include "stdint.h"

void* kmalloc(uint32_t size) {
    if (!heap_start) {
        ERROR("kmalloc: heap not initialized!");
        return NULL;
    }
    if (size == 0) return NULL;
    uint32_t original_size = size;
    if (size & 7) {
        size += 8 - (size & 7);
    }
    heap_block_t *current = heap_start;
    int block_num = 0;
    while (current) {
        if (!current->used && current->size >= size) {
            if (current->size > size + sizeof(heap_block_t) + 32) {
                uint32_t remaining = current->size - size - sizeof(heap_block_t);
                heap_block_t *new_block = (heap_block_t*)((uint32_t)current + sizeof(heap_block_t) + size);
                new_block->magic = 0xC0C0D0AF;
                new_block->size = remaining;
                new_block->used = 0;
                new_block->next = current->next;
                new_block->prev = current;
                if (current->next) {
                    current->next->prev = new_block;
                }
                current->next = new_block;
                current->size = size;
                heap_stats.blocks_count++;
                heap_stats.free_blocks++;
                heap_stats.free_size += remaining;
            }
            
            current->used = 1;
            heap_stats.used_size += current->size;
            heap_stats.free_size -= current->size;
            heap_stats.free_blocks--;
            void* result = (void*)((uint32_t)current + sizeof(heap_block_t));
            return result;
        }
        current = current->next;
    }
    
    if (expand_heap(0) == 0) {
        return kmalloc(size);
    }
    ERROR("kmalloc: no free block for %u bytes", size);
    return NULL;
}