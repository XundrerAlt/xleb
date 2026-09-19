// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "mod.h"
#include "vfs/kheap/mod.h"
#include "vfs/mod.h"
#include "vfs/stdout.h"
#include "string.h"

#define MAX_SPACES 64
process_t *process_table[MAX_SPACES];
uint32_t process_count = 0;

process_t *process_create(void) {
    if (process_count >= MAX_SPACES - 1) {
        ERROR("process_create: table full");
        return 0;
    }
    process_t *s = kmalloc(sizeof(process_t));
    if (!s) return 0;
    memset(s, 0, sizeof(process_t));
    s->id = process_count++;
    s->pd_addr = create_page_directory();
    s->ns = vfs_ns_create();
    if (!s->pd_addr) { kfree(s); return 0; }
    process_table[process_count] = s;
    return s;
}
