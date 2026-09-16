// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "mod.h"
#include "vfs/kheap/mod.h"
#include "vfs/mod.h"
#include "vfs/stdout.h"

static int next_space_id = 0;

space_t *space_create(void) {
    space_t *s = kmalloc(sizeof(space_t));
    if (!s) return 0;
    s->id = next_space_id++;
    s->pd_addr = create_page_directory();
    s->ns = vfs_ns_create();
    vfs_inode_t *stdout = vfs_inode_alloc(VFS_CHARDEV, "stdout");
    stdout->ops = &stdout_ops;
    vfs_inode_add_child(s->ns->root, stdout);
    s->fds[1] = vfs_ofile_create(stdout, 1);
    if (!s->pd_addr) { kfree(s); return 0; }
    return s;
}
