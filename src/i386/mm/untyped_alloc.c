// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "cap/mod.h"
#include "debug.h"
#include "mod.h"
#include "stdint.h"

uint32_t untyped_alloc(uint32_t size) {
    struct cap *cap = cnode_find(root_cnode, 0);
    if (!cap || cap->type != OBJ_UNTYPED) {
        ERROR("untyped_alloc: invalid cap");
        return 0;
    }
    uint32_t align = (size < 8) ? 8 :
    ((size & (size - 1)) == 0) ? size : 8;
    uint32_t aligned = (cap->data.untyped.watermark + align - 1) & ~(align - 1);
    if (aligned + size > cap->data.untyped.size) {
        ERROR("untyped_alloc: out of memory");
        return 0;
    }
    uint32_t addr = cap->data.untyped.base + aligned;
    cap->data.untyped.watermark = aligned + size;
    return addr;
}
