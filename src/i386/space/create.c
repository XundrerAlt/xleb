// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "mod.h"
#include "vfs/kheap/mod.h"
#include "vfs/mod.h"

static int next_space_id = 0;

space_t *space_create(void) {
    space_t *s = kmalloc(sizeof(space_t));
    if (!s) return 0;
    s->id = next_space_id++;
    s->pd_addr = create_page_directory();
    s->ns = vfs_ns_create();
    if (!s->pd_addr) { kfree(s); return 0; }
    return s;
}
