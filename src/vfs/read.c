// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "vfs/mod.h"
#include "vfs/kheap/mod.h"
#include "string.h"
#include "debug.h"

int vfs_read(vfs_inode_t *inode, uint32_t offset, void *buf, uint32_t size) {
    if (!inode || !buf) return -1;
    if (inode->type == VFS_CHARDEV && inode->ops && inode->ops->read) {
        return inode->ops->read(inode, offset, buf, size);
    }
    if (inode->type != VFS_FILE) {
        ERROR("read: '%s' is not a file", inode->name);
        return -1;
    }
    if (offset >= inode->size) return 0;
    uint32_t avail = inode->size - offset;
    uint32_t n = (size < avail) ? size : avail;
    memcpy(buf, inode->data + offset, n);
    return n;
}
