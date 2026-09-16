// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "kheap/mod.h"
#include "mod.h"
#include "ofile.h"
#include "string.h"
#include "space/mod.h"

int vfs_open(space_t *space, const char *path, int flags) {
    if (!space) {
        DEBUG("open: no space");
        return -1;
    }
    if (!path) {
        DEBUG("open: no path");
        return -1;
    }
    vfs_inode_t *inode = vfs_lookup(space->ns->root, path);
    if (!inode) {
        DEBUG("open: '%s' not found", path);
        return -1;
    }
    vfs_ofile_t *f = vfs_ofile_create(inode, flags);
    if (!f) {
        return -1;
    }
    for (int i = 0; i < MAX_FDS; i++) {
        if (!space->fds[i]) {
            space->fds[i] = f;
            return i;
        }
    }
    vfs_ofile_unref(f);
    return -1;
}

int vfs_close(space_t *space, int fd) {
    if (!space || fd < 0 || fd >= MAX_FDS) {
        DEBUG("close: invalid fd or space");
        return -1;
    }
    vfs_ofile_t *f = space->fds[fd];
    if (!f) {
        DEBUG("close: fd %d not open", fd);
        return -1;
    }
    space->fds[fd] = 0;
    vfs_ofile_unref(f);
    return 0;
}
