// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "mod.h"
#include "mm/mod.h"
#include "vfs/kheap/mod.h"

struct mspace *mspace = 0;
memory_map_t memory_map;

void mm_init(void) {
    static struct mspace mspace_storage;
    mspace = &mspace_storage;
    memset(mspace, 0, sizeof(struct mspace));
    mspace->size = MEM_SLOTS;
    for (int i = 0; i < memory_map.region_count; i++) {
        if (memory_map.regions[i].type != 1) continue;
        struct mem mem;
        mem.type = OBJ_UNTYPED;
        mem.data.untyped.base = memory_map.regions[i].start;
        mem.data.untyped.size = memory_map.regions[i].size;
        mem.data.untyped.watermark = 0;
        mem.data.untyped.free = 1;
        int index = mspace_add(mspace, &mem);
        if (index < 0) {
            WARN("mem: mspace full");
            break;
        }
        DEBUG("mem: untyped[%d] 0x%x - 0x%x (%d KB)", index,
             mem.data.untyped.base, mem.data.untyped.base + mem.data.untyped.size,
             mem.data.untyped.size / 1024);
    }
    temp_map_init();
    kheap_init();
    INFO("memory manager: hello world");
}
