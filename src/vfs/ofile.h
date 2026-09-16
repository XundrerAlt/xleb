// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#pragma once
#include "stdint.h"
#include "mod.h"

#define MAX_FDS 64

typedef struct vfs_ofile {
    vfs_inode_t *inode;
    uint32_t offset;
    int flags;
    uint32_t refcount;
} vfs_ofile_t;

vfs_ofile_t *vfs_ofile_create(vfs_inode_t *inode, int flags);
void vfs_ofile_close(vfs_ofile_t *ofile);
void vfs_ofile_ref(vfs_ofile_t *ofile);
void vfs_ofile_unref(vfs_ofile_t *ofile);
int vfs_open(space_t *space, const char *path, int flags);
int vfs_close(space_t *space, int fd);
