// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "cap/mod.h"
#include "debug.h"
#include "mod.h"
#include "stdint.h"

static int current_untyped_idx = 0;

static uint32_t pick_align(uint32_t size) {
    if (size < 8) return 8;
    if ((size & (size - 1)) == 0) return size;
    return 8;
}

uint32_t try_alloc(struct cap *cap, uint32_t size, uint32_t align) {
    uint32_t base = cap->data.untyped.base;
    uint32_t cur = base + cap->data.untyped.watermark;
    uint32_t aligned = (cur + align - 1) & ~(align - 1);
    if (aligned + size > base + cap->data.untyped.size) return 0;
    uint32_t addr = aligned;
    cap->data.untyped.watermark = (aligned - base) + size;
    return addr;
}

uint32_t untyped_alloc(uint32_t size) {
    if (size == 0) return 0;
    uint32_t align = pick_align(size);

    struct cap *cap = cnode_find(root_cnode, current_untyped_idx);
    if (cap && cap->type == OBJ_UNTYPED) {
        uint32_t addr = try_alloc(cap, size, align);
        if (addr) return addr;
    }

    for (uint32_t i = 0; i < root_cnode->size; i++) {
        if (i == (uint32_t)current_untyped_idx) continue;
        struct cap *c = cnode_find(root_cnode, i);
        if (!c || c->type != OBJ_UNTYPED) continue;
        uint32_t addr = try_alloc(c, size, align);
        if (addr) {
            current_untyped_idx = i;
            return addr;
        }
    }

    ERROR("untyped_alloc: out of memory");
    return 0;
}
