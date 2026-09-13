// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "stdint.h"
#include "mod.h"

int cnode_add(struct cnode *cnode, struct cap *cap) {
    if (!cnode || !cap) return -1;

    for (uint32_t i = 0; i < cnode->size; i++) {
        if (cnode->slots[i].type == CAP_NONE) {
            cnode->slots[i] = *cap;
            return i;
        }
    }
    return -1;
}

struct cap *cnode_find(struct cnode *cnode, uint32_t index) {
    if (!cnode) return 0;
    if (index >= cnode->size) return 0;
    if (cnode->slots[index].type == CAP_NONE) return 0;
    return &cnode->slots[index];
}

void cnode_remove(struct cnode *cnode, uint32_t index) {
    if (!cnode) return;
    if (index >= cnode->size) return;

    cnode->slots[index].type = CAP_NONE;
}

uint32_t cnode_count(struct cnode *cnode) {
    if (!cnode) return 0;

    uint32_t count = 0;
    for (uint32_t i = 0; i < cnode->size; i++) {
        if (cnode->slots[i].type != CAP_NONE) {
            count++;
        }
    }
    return count;
}
