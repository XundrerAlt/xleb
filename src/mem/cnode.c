// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "stdint.h"
#include "mod.h"
#include "mm/mod.h"

int mspace_add(struct mspace *mspace, struct mem *mem) {
    if (!mspace || !mem) return -1;

    for (uint32_t i = 0; i < mspace->size; i++) {
        if (mspace->slots[i].type == OBJ_NONE) {
            mspace->slots[i] = *mem;
            return i;
        }
    }
    return -1;
}

struct mem *mspace_find(struct mspace *mspace, uint32_t index) {
    if (!mspace) return 0;
    if (index >= mspace->size) return 0;
    if (mspace->slots[index].type == OBJ_NONE) return 0;
    return &mspace->slots[index];
}

void mspace_remove(struct mspace *mspace, uint32_t index) {
    if (!mspace) return;
    if (index >= mspace->size) return;

    mspace->slots[index].type = OBJ_NONE;
}

uint32_t mspace_count(struct mspace *mspace) {
    if (!mspace) return 0;

    uint32_t count = 0;
    for (uint32_t i = 0; i < mspace->size; i++) {
        if (mspace->slots[i].type != OBJ_NONE) {
            count++;
        }
    }
    return count;
}
