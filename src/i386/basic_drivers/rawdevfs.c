// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "mod.h"
#include "vfs/mod.h"
#include "debug.h"
#include "string.h"

extern const raw_device_t __raw_devices_start[];
extern const raw_device_t __raw_devices_end[];

static vfs_inode_t *rawdevfs_root = 0;

vfs_inode_t *rawdevfs_get_root(void) {
    if (rawdevfs_root) return rawdevfs_root;
    rawdevfs_root = vfs_inode_alloc(VFS_DIR, "raw");
    if (!rawdevfs_root) {
        ERROR("rawdevfs: failed to alloc root");
        return 0;
    }
    const raw_device_t *p = __raw_devices_start;
    while (p < __raw_devices_end) {
        DEBUG("rawdevfs: adding device '%s'", p->name);
        vfs_inode_t *dev_dir = vfs_inode_alloc(VFS_DIR, p->name);
        if (!dev_dir) { p++; continue; }
        vfs_inode_add_child(rawdevfs_root, dev_dir);
        for (uint32_t i = 0; i < p->file_count; i++) {
            vfs_inode_t *file = vfs_inode_alloc(VFS_CHARDEV, p->files[i].name);
            if (!file) continue;
            file->fs_data = (void*)p;
            file->ops = &p->files[i].ops;
            vfs_inode_add_child(dev_dir, file);
            DEBUG("rawdevfs: /dev/raw/%s/%s", p->name, p->files[i].name);
        }
        p++;
    }

    return rawdevfs_root;
}

static vfs_fs_t rawdevfs = {
    .name = "rawdevfs",
    .get_root = rawdevfs_get_root,
    .next = 0,
};

void rawdevfs_init(void) {
    vfs_register_fs(&rawdevfs);
}
