// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "basic_drivers/mod.h"
#include "debug.h"
#include "vfs/mod.h"

extern const raw_device_t __raw_devices_start[];
extern const raw_device_t __raw_devices_end[];

void add_raw_devices(vfs_namespace_t *ns) {
    if (!ns || !ns->root) return;
    vfs_inode_t *dev = vfs_lookup(ns->root, "/dev");
    if (!dev) {
        dev = vfs_inode_alloc(VFS_DIR, "dev");
        vfs_inode_add_child(ns->root, dev);
    }
    vfs_inode_t *raw = vfs_inode_find_child(dev, "raw");
    if (!raw) {
        raw = vfs_inode_alloc(VFS_DIR, "raw");
        vfs_inode_add_child(dev, raw);
    }
    const raw_device_t *p = __raw_devices_start;
    while (p < __raw_devices_end) {
        vfs_inode_t *inode = vfs_inode_alloc(VFS_CHARDEV, p->name);
        if (inode) {
            inode->fs_data = (void*)p;
            inode->ops = (struct chardev_ops*)&p->ops;
            vfs_inode_add_child(raw, inode);
            DEBUG("raw device: /dev/raw/%s", p->name);
        }
        p++;
    }
}
