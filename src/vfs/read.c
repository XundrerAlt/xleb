// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "vfs/mod.h"
#include "vfs/kheap/mod.h"
#include "string.h"
#include "process/mod.h"
#include "debug.h"
#include "process/mod.h"

static int vfs_read_inode(vfs_inode_t *inode, uint32_t offset, void *buf, uint32_t size) {
    if (!inode || !buf) return -1;
    if (inode->type == VFS_CHARDEV) {
        if (!inode->ops) {
            WARN("chardev '%s' has no ops", inode->name);
            return -1;
        }
        if (!inode->ops->read) {
            WARN("chardev '%s' has no read op", inode->name);
            return -1;
        }
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

int vfs_read(process_t *process, int fd, void *buf, uint32_t size) {
    if (!process || fd < 0 || fd >= MAX_FDS) return -1;
    vfs_ofile_t *f = process->fds[fd];
    if (!f || !f->inode) return -1;
    int n = vfs_read_inode(f->inode, f->offset, buf, size);
    if (n > 0) f->offset += n;
    return n;
}
