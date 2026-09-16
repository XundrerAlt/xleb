// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "mod.h"
#include "vfs/kheap/mod.h"
#include "vfs/mod.h"
#include "vfs/stdout.h"
#include "string.h"

#define MAX_SPACES 64
space_t *space_table[MAX_SPACES];
uint32_t space_count = 0;

space_t *space_create(void) {
    if (space_count >= MAX_SPACES - 1) {
        ERROR("space_create: table full");
        return 0;
    }
    space_t *s = kmalloc(sizeof(space_t));
    if (!s) return 0;
    memset(s, 0, sizeof(space_t));
    s->id = space_count++;
    s->pd_addr = create_page_directory();
    s->ns = vfs_ns_create();
    vfs_inode_t *dev = vfs_inode_alloc(VFS_DIR, "dev");
    vfs_inode_add_child(s->ns->root, dev);
    vfs_inode_t *stdout = vfs_inode_alloc(VFS_CHARDEV, "stdout");
    stdout->ops = &stdout_ops;
    vfs_inode_add_child(dev, stdout);
    s->fds[1] = vfs_ofile_create(stdout, 1);
    if (!s->pd_addr) { kfree(s); return 0; }
    space_table[space_count] = s;
    return s;
}
