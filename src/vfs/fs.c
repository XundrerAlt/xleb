// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "vfs/mod.h"
#include "string.h"

static vfs_fs_t *fs_list = 0;

void vfs_register_fs(vfs_fs_t *fs) {
    if (!fs) return;
    fs->next = fs_list;
    fs_list = fs;
    DEBUG("vfs: registered fs '%s'", fs->name);
}

vfs_fs_t *vfs_find_fs(const char *name) {
    if (!name) return 0;
    vfs_fs_t *fs = fs_list;
    while (fs) {
        if (fs->name && strcmp(fs->name, name) == 0) return fs;
        fs = fs->next;
    }
    return 0;
}
