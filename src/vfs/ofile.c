// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "kheap/mod.h"
#include "mod.h"
#include "ofile.h"
#include "string.h"

vfs_ofile_t *vfs_ofile_create(vfs_inode_t *inode, int flags) {
    if (!inode) return 0;
    vfs_ofile_t *f = kmalloc(sizeof(vfs_ofile_t));
    if (!f) {
        ERROR("vfs_ofile_create: out of memory");
        return 0;
    }
    memset(f, 0, sizeof(vfs_ofile_t));
    f->inode = inode;
    f->offset = 0;
    f->flags = flags;
    f->refcount = 1;
    vfs_inode_ref(inode);
    return f;
}

void vfs_ofile_ref(vfs_ofile_t *f) {
    if (f) f->refcount++;
}

void vfs_ofile_unref(vfs_ofile_t *f) {
    if (!f) return;
    if (f->refcount == 0) return;
    f->refcount--;
    if (f->refcount == 0) {
        if (f->inode) vfs_inode_unref(f->inode);
        kfree(f);
    }
}

void vfs_ofile_close(vfs_ofile_t *f) {
    vfs_ofile_unref(f);
}
