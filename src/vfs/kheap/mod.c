// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "halt.h"
#include "mm/mod.h"
#include "mm/virtconv.h"
#include "mod.h"
#define HDR sizeof(struct kheap_block)

static struct kheap_block *kheap_head = 0;
static uint32_t kheap_total = 0;
static uint32_t kheap_used_bytes = 0;

void kheap_init(void) {
    uint32_t region = untyped_alloc(KHEAP_SIZE);
    if (!region) {
        ERROR("kheap_init: untyped_alloc failed");
        halt();
    }
    void *base = (void*)PHYS_TO_VIRT(region);
    kheap_head = (struct kheap_block*)base;
    kheap_head->size = KHEAP_SIZE - HDR;
    kheap_head->free = 1;
    kheap_head->next = 0;
    kheap_total = KHEAP_SIZE;
    kheap_used_bytes = 0;
    INFO("kheap: %u KB at 0x%x", KHEAP_SIZE / 1024, region);
}

void *kmalloc(uint32_t size) {
    if (size == 0) return 0;
    size = ALIGN8(size);
    struct kheap_block *b = kheap_head;
    while (b) {
        if (b->free && b->size >= size) {
            if (b->size > size + HDR + 16) {
                struct kheap_block *nb =
                (struct kheap_block*)((uint8_t*)b + HDR + size);
                nb->size = b->size - size - HDR;
                nb->free = 1;
                nb->next = b->next;
                b->size = size;
                b->next = nb;
            }
            b->free = 0;
            kheap_used_bytes += HDR + b->size;
            return (uint8_t*)b + HDR;
        }
        b = b->next;
    }
    return 0;
}

void kfree(void *p) {
    if (!p) return;
    struct kheap_block *b = (struct kheap_block*)((uint8_t*)p - HDR);
    if (b->free) return;
    b->free = 1;
    kheap_used_bytes -= HDR + b->size;

    b = kheap_head;
    while (b && b->next) {
        if (b->free && b->next->free) {
            b->size += HDR + b->next->size;
            b->next = b->next->next;
        } else {
            b = b->next;
        }
    }
}
