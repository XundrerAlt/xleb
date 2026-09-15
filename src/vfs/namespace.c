// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "mod.h"
#include "kheap/mod.h"

vfs_namespace_t *vfs_ns_create(void) {
    vfs_namespace_t *ns = kmalloc(sizeof(vfs_namespace_t));
    ns->root = vfs_inode_alloc(VFS_DIR, "/");
    return ns;
}
