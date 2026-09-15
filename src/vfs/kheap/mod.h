// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#pragma once
#include "stdint.h"
#define KHEAP_SIZE (4 * 1024 * 1024)
#define ALIGN8(x) (((x) + 7) & ~7u)

struct kheap_block {
    uint32_t size;
    uint32_t free;
    struct kheap_block *next;
};

void kheap_init(void);
void *kmalloc(uint32_t size);
void kfree(void *p);
