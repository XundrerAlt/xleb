// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "mod.h"
#include "mm/mod.h"

struct cnode *root_cnode = 0;

void cap_init(void) {
    static struct cnode root_cnode_storage;
    root_cnode = &root_cnode_storage;
    memset(root_cnode, 0, sizeof(struct cnode));
    root_cnode->size = CNODE_SLOTS;
    for (int i = 0; i < memory_map.region_count; i++) {
        if (memory_map.regions[i].type != 1) continue;
        struct cap cap;
        cap.type = OBJ_UNTYPED;
        cap.data.untyped.base = memory_map.regions[i].start;
        cap.data.untyped.size = memory_map.regions[i].size;
        cap.data.untyped.watermark = 0;
        cap.data.untyped.free = 1;
        int index = cnode_add(root_cnode, &cap);
        if (index < 0) {
            WARN("cap: cnode full!");
            break;
        }
        DEBUG("cap: untyped[%d] 0x%x - 0x%x (%d KB)", index,
             cap.data.untyped.base, cap.data.untyped.base + cap.data.untyped.size,
             cap.data.untyped.size / 1024);
    }
    INFO("Capabilities: hello world");
}
