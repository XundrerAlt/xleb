// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#pragma once
#include "stdint.h"
#define VFS_NAME_MAX 256

typedef enum {
    VFS_FILE,
    VFS_DIR,
    VFS_CHARDEV,
} vfs_type_t;

typedef struct vfs_inode vfs_inode_t;
struct chardev_ops;
typedef struct space space_t;

typedef struct vfs_inode {
    uint32_t id;
    vfs_type_t type;
    uint32_t size;
    uint32_t refcount;
    uint8_t *data;
    void *fs_data;
    char name[VFS_NAME_MAX];
    struct chardev_ops *ops;
    struct vfs_inode *parent;
    struct vfs_inode *children;
    struct vfs_inode *next;
} vfs_inode_t;

struct chardev_ops {
    int (*read)(vfs_inode_t *inode, uint32_t offset, void *buf, uint32_t size);
    int (*write)(vfs_inode_t *inode, uint32_t offset, const void *buf, uint32_t size);
};

typedef struct vfs_namespace {
    struct vfs_inode *root;
} vfs_namespace_t;

vfs_inode_t *vfs_inode_alloc(vfs_type_t type, const char *name);
void vfs_inode_free(vfs_inode_t *inode);
vfs_namespace_t *vfs_ns_create(void);

void vfs_inode_ref(vfs_inode_t *inode);
void vfs_inode_unref(vfs_inode_t *inode);
int vfs_inode_add_child(vfs_inode_t *dir, vfs_inode_t *child);
vfs_inode_t *vfs_inode_find_child(vfs_inode_t *dir, const char *name);
vfs_inode_t *vfs_lookup(vfs_inode_t *root, const char *path);
int vfs_read(space_t *space, int fd, void *buf, uint32_t size);
int vfs_write(space_t *space, int fd, const void *buf, uint32_t size);

void vfs_init(void);
