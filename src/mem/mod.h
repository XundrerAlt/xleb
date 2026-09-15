// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#pragma once
#include "stdint.h"
#define MEM_SLOTS 256

struct mem_frame {
    uint32_t addr;
    uint32_t size;
};

struct mem_untyped {
    uint32_t base;
    uint32_t size;
    uint32_t watermark;
    uint32_t free;
};

union mem_data {
    struct mem_frame frame;
    struct mem_untyped untyped;
};

struct mem {
    uint32_t type;
    union mem_data data;
};

struct mspace {
    struct mem slots[MEM_SLOTS];
    uint32_t size;
};

extern struct mspace *mspace;

int mspace_add(struct mspace *mspace, struct mem *mem);
struct mem *mspace_find(struct mspace *mspace, uint32_t index);
void mspace_remove(struct mspace *mspace, uint32_t index);
uint32_t mspace_count(struct mspace *mspace);
