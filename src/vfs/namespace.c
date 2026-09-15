// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "mod.h"
#include "kheap/mod.h"

static int stdout_write(vfs_inode_t *inode, uint32_t offset, const void *buf, uint32_t size) {
    (void)inode;
    (void)offset;
    const char *s = (const char*)buf;
    for (uint32_t i = 0; i < size; i++) {
        uart_putc(s[i]);
    }
    return size;
}

struct chardev_ops stdout_ops = {
    .read = 0,
    .write = stdout_write,
};

vfs_namespace_t *vfs_ns_create(void) {
    vfs_namespace_t *ns = kmalloc(sizeof(vfs_namespace_t));
    ns->root = vfs_inode_alloc(VFS_DIR, "/");
    return ns;
}
