// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "cap/mod.h"
#include "debug.h"
#include "mod.h"
#include "stdint.h"

uint32_t untyped_alloc(uint32_t size) {
    struct cap *cap = cnode_find(root_cnode, 0); // TODO: find first untyped cap
    if (!cap || cap->type != CAP_UNTYPED) {
        ERROR("untyped_alloc: invalid cap");
        return 0;
    }
    if (cap->data.untyped.watermark + size > cap->data.untyped.size) {
        ERROR("untyped_alloc: out of memory");
        return 0;
    }
    uint32_t addr = cap->data.untyped.base + cap->data.untyped.watermark;
    cap->data.untyped.watermark += size;
    return addr;
}
