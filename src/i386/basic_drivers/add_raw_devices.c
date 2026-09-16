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
        vfs_inode_t *device_dir = vfs_inode_alloc(VFS_DIR, p->name);
        vfs_inode_add_child(raw, device_dir);
        for (uint32_t i = 0; i < p->file_count; i++) {
            vfs_inode_t *file = vfs_inode_alloc(VFS_CHARDEV, p->files[i].name);
            file->fs_data = (void*)p;
            file->ops = &p->files[i].ops;
            vfs_inode_add_child(device_dir, file);
            DEBUG("added /dev/raw/%s/%s", p->name, p->files[i].name);
        }
        p++;
    }
}
