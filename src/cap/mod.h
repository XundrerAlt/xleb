// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#pragma once
#include "stdint.h"
#define CNODE_SLOTS 256

enum cap_type {
    CAP_NONE,
    CAP_UNTYPED,
};

struct cap_frame {
    uint32_t addr;
    uint32_t size;
};

struct cap_untyped {
    uint32_t base;
    uint32_t size;
    uint32_t watermark;
    uint32_t free;
};

union cap_data {
    struct cap_frame frame;
    struct cap_untyped untyped;
};

struct cap {
    uint32_t type;
    union cap_data data;
};

struct cnode {
    struct cap slots[CNODE_SLOTS];
    uint32_t size;
};

extern struct cnode *root_cnode;
void cap_init(void);

int cnode_add(struct cnode *cnode, struct cap *cap);
struct cap *cnode_find(struct cnode *cnode, uint32_t index);
void cnode_remove(struct cnode *cnode, uint32_t index);
uint32_t cnode_count(struct cnode *cnode);
