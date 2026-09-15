// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "vfs/mod.h"
#include "vfs/kheap/mod.h"
#include "string.h"
#include "debug.h"

int vfs_write(vfs_inode_t *inode, uint32_t offset, const void *buf, uint32_t size) {
    if (!inode || !buf) return -1;
    if (inode->type != VFS_FILE) {
        ERROR("write: '%s' is not a file", inode->name);
        return -1;
    }
    if (size == 0) return 0;
    uint32_t needed = offset + size;
    if (needed > inode->size) {
        uint8_t *new_data = kmalloc(needed);
        if (!new_data) {
            ERROR("write: out of memory (%u bytes)", needed);
            return -1;
        }
        if (inode->data) {
            memcpy(new_data, inode->data, inode->size);
            kfree(inode->data);
        }
        inode->data = new_data;
        inode->size = needed;
    }
    memcpy(inode->data + offset, buf, size);
    return size;
}
