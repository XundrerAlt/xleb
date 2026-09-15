// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "mem/mod.h"
#include "debug.h"
#include "mod.h"
#include "stdint.h"
#include "thread/mod.h"

uint32_t obj_size(obj_type_t type, uint32_t size) {
    switch (type) {
        case OBJ_FRAME: return 1 << size;
        case OBJ_THREAD: return sizeof(thread_t);
        case OBJ_VNODE: return PAGE_SIZE;
        default: return 0;
    }
}

uint32_t retype(obj_type_t type, uint32_t size) {
    uint32_t needed = obj_size(type, size);
    if (needed == 0) {
        ERROR("retype: invalid type %d", type);
        return 0;
    }
    uint32_t addr = untyped_alloc(needed);
    void *virt = temp_map(addr);
    if (!virt) {
        ERROR("retype: temp_map failed");
        return 0;
    }
    memset(virt, 0, needed);
    temp_unmap(virt);
    struct mem new_mem;
    new_mem.type = type;
    switch (type) {
        case OBJ_FRAME:
            new_mem.data.frame.addr = addr;
            new_mem.data.frame.size = 1 << size;
            break;
        default:
            break;
    }
    int index = mspace_add(mspace, &new_mem);
    if (index < 0) {
        ERROR("retype: mspace full");
        return 0;
    }
    return addr;
}
