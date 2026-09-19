// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "kheap/mod.h"
#include "mod.h"
#include "ofile.h"
#include "string.h"
#include "process/mod.h"

int vfs_open(process_t *process, const char *path, int flags) {
    if (!process) {
        DEBUG("open: no process");
        return -1;
    }
    if (!path) {
        DEBUG("open: no path");
        return -1;
    }
    vfs_inode_t *inode = vfs_lookup_ns(process->ns, path);
    if (!inode) {
        DEBUG("open: '%s' not found", path);
        return -1;
    }
    vfs_ofile_t *f = vfs_ofile_create(inode, flags);
    if (!f) {
        return -1;
    }
    for (int i = 0; i < MAX_FDS; i++) {
        if (!process->fds[i]) {
            process->fds[i] = f;
            return i;
        }
    }
    vfs_ofile_unref(f);
    return -1;
}

int vfs_close(process_t *process, int fd) {
    if (!process || fd < 0 || fd >= MAX_FDS) {
        DEBUG("close: invalid fd or process");
        return -1;
    }
    vfs_ofile_t *f = process->fds[fd];
    if (!f) {
        DEBUG("close: fd %d not open", fd);
        return -1;
    }
    process->fds[fd] = 0;
    vfs_ofile_unref(f);
    return 0;
}
